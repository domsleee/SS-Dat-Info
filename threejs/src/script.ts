import { getDomVM } from "./data";
import { setupVideo, videoIds } from "./video";
import { calculateAcceleration, getSpeed } from "./coordUtil";
import { AnalyzeResult, RowData, UNKNOWN_TRACK } from "dat-analyze/src/types";
import { SnowboardTrackAnalyzer } from "./snowboardTrackAnalyzer";
import { createCameraSetup } from "./cameraSetup";
import { createCharacterGroup } from "./characterGroup";
import { setupConfig, updateConfigDOM } from "./config";
import { Config, MainLoopContainer, TextFields } from "./types";
import { createPresets } from "./presets";
import { LevelScore, PlaneCollisionInfo } from "dat-analyze/src/PlaneUtil/types";
import { minBy } from "lodash-es";
import { getScoreBreakdown, MAX_SCORE, PLANE_RADIUS, NEAREST_START_POINT_DIST } from "dat-analyze/src/PlaneUtil/scoreTrack";
import { createElement, Info, Pause, Play, SkipBack, SkipForward } from 'lucide';
import { PositionXYZ } from "dat-analyze/src/generateCircle/types";
import { DirectionalLight, Euler, Group, Matrix4, PerspectiveCamera, Scene, WebGLRenderer } from "three";
import { getBlock } from "dat-analyze/src/analyzeReplay";

const dimensions = {
  width: 480,
  height: 360,
};

const config: Config = {
  offsetSeconds: -1.05, // smaller number = threejs plays earlier
  videoId: videoIds.vm109,
  syncWithVideo: true,
};
const SYNC_THRESHOLD = 0.02;

const playerState = {
  isPlaying: true,
  resumedTime: 0,
  lastFrameRendered: -5000,
  lastFrameChecked: -5000,
};

const videoTarget = setupVideo(config.videoId, dimensions, async () => {
  if (playerState.isPlaying) {
    videoTarget.videoTarget?.playVideo();
  } else {
    videoTarget.videoTarget?.pauseVideo();
  }
});

async function main() {
  setupButtons();
  document.getElementById("mainContainer")!.style.width = `${
    dimensions.width * 2
  }px`;
  const textFields = createTextFields();
  const analyzeResult = await getDomVM();
  const mainLoopContainer: MainLoopContainer = { textFields, analyzeResult };

  let last: { dispose: () => void } | undefined;  
  const processAnalyzeResult = (analyzeResult: AnalyzeResult) => {
    last?.dispose();
    mainLoopContainer.analyzeResult = analyzeResult;
    last = mainLoop(mainLoopContainer);
  };
  createPresets(config, videoTarget, processAnalyzeResult);
  setupConfig(config, videoTarget, processAnalyzeResult);
  processAnalyzeResult(analyzeResult);
}

function setupButtons() {
  const playPauseButton = document.getElementById("playPauseButton")!;
  const backButton = document.getElementById("backButton")!;
  const forwardButton = document.getElementById("forwardButton")!;

  setPlayPauseButtonIcon(playPauseButton);
  backButton.innerHTML = createElement(SkipBack).outerHTML;
  forwardButton.innerHTML = createElement(SkipForward).outerHTML;
}

function setPlayPauseButtonIcon(playPauseButton: HTMLElement) {
  playPauseButton.innerHTML = createElement(playerState.isPlaying ? Pause : Play).outerHTML;
}

function mainLoop(mainLoopContainer: MainLoopContainer) {
  const { textFields, analyzeResult } = mainLoopContainer;
  updateConfigDOM(config);

  textFields.nameText.textContent = analyzeResult.playerName;
  textFields.timeText.textContent = getHumanReadableTime(analyzeResult, 0)

  const playPauseButton = document.getElementById("playPauseButton")!;
  const backButton = document.getElementById("backButton")!;
  const forwardButton = document.getElementById("forwardButton")!;
  const playerRange = document.getElementById(
    "playerRange"
  )! as HTMLInputElement;
  const frameInfo = document.getElementById("frameInfo")! as HTMLDivElement;
  const keys = {
    left: document.getElementsByClassName("keyLeft")[0] as HTMLElement,
    right: document.getElementsByClassName("keyRight")[0] as HTMLElement,
    shift: document.getElementsByClassName("keyShift")[0] as HTMLElement,
  };

  playPauseButton.onclick = () => {
    if (playerState.isPlaying) {
      playerState.isPlaying = false;
      if (config.syncWithVideo) {
        videoTarget.videoTarget?.pauseVideo();
        const expectedObject = resync();
        if (expectedObject) {
          renderFrame(0, Math.floor(expectedObject.expectedFrame));
        }
      }
    } else {
      playerState.resumedTime =
        performance.now() - 10 * playerState.lastFrameRendered;
      playerState.isPlaying = true;

      if (config.syncWithVideo) {
        videoTarget.videoTarget?.playVideo();
      }
    }

    setPlayPauseButtonIcon(playPauseButton);
  };

  forwardButton.onclick = () => {
    renderFrame(0, (playerState.lastFrameRendered + 1) % data.length);
  };

  backButton.onclick = () => {
    renderFrame(
      0,
      (playerState.lastFrameRendered - 1 + data.length) % data.length
    );
  };

  const preText = document.getElementById("preText")!;
  const data = analyzeResult.coords!.rows.slice(0, -1);
  document.getElementById("headerInfo")!.innerHTML =
    getHeaderHtml(analyzeResult);
  document.getElementById("scoreInfo")!.innerHTML =
    getScoreHtml(analyzeResult);

  playerRange.min = "0";
  playerRange.max = data.length.toString();

  const camera = new PerspectiveCamera(
    75,
    dimensions.width / dimensions.height,
    0.01,
    1000
  );

  const scene = new Scene();

  const keyControls = document.getElementById("keyControls")!;
  keyControls.style.bottom = "0";
  keyControls.style.right = `${dimensions.width}px`;

  const cameraSystem = createCameraSetup(camera, scene);

  const analyzer = new SnowboardTrackAnalyzer(
    data.map((c) => transformPosition(c)),
    { accelerationThreshold: 0.05 }
  );
  const slopeMesh = analyzer.createSlopeMesh({ playerYOffset: 0.4 });
  const jumpMarkers = analyzer.createJumpMarkers({});

  scene.add(slopeMesh);
  jumpMarkers.forEach((marker) => scene.add(marker));

  const directionalLight = new DirectionalLight(0xffffff, 1);
  directionalLight.castShadow = true;
  scene.add(directionalLight);

  const firstRow = data[0];

  const characterGroup = createCharacterGroup();
  setMeshPosition(characterGroup, firstRow);
  scene.add(characterGroup);

  const renderer = new WebGLRenderer({ antialias: true });
  renderer.setSize(dimensions.width, dimensions.height);
  renderer.setAnimationLoop((time) => renderFrame(time));
  renderer.shadowMap.enabled = true;
  document.getElementById("container")!.prepend(renderer.domElement);

  playerRange.addEventListener("click", (e) => {
    const clickPosition = e.offsetX / playerRange.offsetWidth;
    const frame = Math.floor(clickPosition * data.length);
    playerState.resumedTime = performance.now() - 10 * frame;
    if (config.syncWithVideo) {
      videoTarget.videoTarget?.seekTo(frame / 100 + config.offsetSeconds, true);
    }
    playerState.lastFrameChecked = frame;
    renderFrame(0, frame);
  });

  // animation
  function renderFrame(time: number, frame: number | undefined = undefined) {
    if (frame === undefined && !playerState.isPlaying) {
      return;
    }

    const frameToRender = frame ?? Math.floor((time - playerState.resumedTime) / 10) % data.length;
    
    if (frameToRender < 0 || frameToRender > data.length - 1) return;
    const row = data[frameToRender];
    updateCharacterRotation(characterGroup, row);

    setMeshPosition(characterGroup, row);
    const speed = getSpeed(data, frameToRender);
    if (frameToRender > 0) {
      cameraSystem.updateCamera(
        data[frameToRender],
        data[frameToRender - 1],
        characterGroup,
        speed
      );
    }

    const drift = getDriftSeconds(frameToRender);
    const newHtml = `x: ${characterGroup.position.x}
y: ${characterGroup.position.y}
z: ${characterGroup.position.z}
accel(y): ${calculateAcceleration(data, frameToRender).toFixed(1)}
drift(s): ${
  drift ? (drift.actualSeconds - drift.expectedSeconds).toFixed(3) : "N/A"
}
rotation3x3:
[
  ${row.rotation3x3[0].map((n) => n.toFixed(3)).join(", ")}
  ${row.rotation3x3[1].map((n) => n.toFixed(3)).join(", ")}
  ${row.rotation3x3[2].map((n) => n.toFixed(3)).join(", ")}
]
${getRawString(getBlock(analyzeResult.data, frameToRender))}`;

    setKeyInputState(keys, row);
    playerRange.value = frameToRender.toString();
    frameInfo.innerHTML = `Frame: ${frameToRender} / ${data.length}`;
    textFields.timeText.textContent = getHumanReadableTime(analyzeResult, frameToRender);
    textFields.speedText1.textContent = `${Math.floor(getSpeed(data, frameToRender))
      .toFixed(0)
      .padStart(3, "0")} km/h`;
    textFields.speedText2.textContent = textFields.speedText1.textContent.replace(" ", "");

    if (newHtml !== preText.innerHTML) {
      preText.innerHTML = newHtml;
    }

    if (
      videoTarget.videoTarget !== undefined &&
      Math.abs(playerState.lastFrameChecked - frameToRender) > 100
    ) {
      tryResync(frameToRender);
      playerState.lastFrameChecked = frameToRender;
    }

    playerState.lastFrameRendered = frameToRender;
    renderer.render(scene, camera);
  }

  return {
    dispose: () => {
      console.log("DISPOSE!!!!");
      renderer.domElement.remove();
      renderer.dispose();
    },
  };
}


function createTextFields(): TextFields {
  const topOffset = (42 / 540) * dimensions.height;
  const topOffset2 = (124 / 540) * dimensions.height;
  const topOffset3 = (152 / 540) * dimensions.height;
  const leftTimeOffset = (22 / 720) * dimensions.width;
  const leftSpeedOffset = (20 / 720) * dimensions.width;
  const fontSize = (22 / 540) * dimensions.height;
  const paddingSize = (10 / 540) * dimensions.height;

  const nameText = createText("", {
    top: `${topOffset}px`,
    right: `${dimensions.width + leftTimeOffset}px`,
    fontSize: `${fontSize}px`,
  });
  const timeText = createText("", {
    top: `${topOffset + leftTimeOffset}px`,
    right: `${dimensions.width + leftTimeOffset}px`,
    fontSize: `${fontSize}px`,
  });
  // const speedText = createText("Speed (km/h)", { top: '24px', left: '20px' });
  const speedText1 = createText("000km/h", {
    top: `${topOffset3}px`,
    right: `${dimensions.width + leftTimeOffset}px`,
    fontSize: `${fontSize}px`,
  });
  const speedText2 = createText("000km/h", {
    top: `${topOffset2}px`,
    left: `${dimensions.width + leftSpeedOffset}px`,
    backgroundColor: "black",
    letterSpacing: "1px",
    padding: `0 ${paddingSize}px`,
    fontSize: `${fontSize}px`,
  });

  return { nameText, timeText, speedText1, speedText2 };
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function setKeyInputState(keys: unknown, row: RowData) {
  return; // todo
  // if (row.left) {
  //   keys.left.classList.add("pressed");
  // } else {
  //   keys.left.classList.remove("pressed");
  // }

  // if (row.right) {
  //   keys.right.classList.add("pressed");
  // } else {
  //   keys.right.classList.remove("pressed");
  // }

  // if (row.shift) {
  //   keys.shift.classList.add("pressed");
  // } else {
  //   keys.shift.classList.remove("pressed");
  // }
}

function createText(text: string, style?: Partial<CSSStyleDeclaration>) {
  const textDiv = document.createElement("div");
  textDiv.style.position = "absolute";
  textDiv.style.color = "white";
  textDiv.style.fontSize = "22px";
  Object.assign(textDiv.style, style);
  textDiv.textContent = text;
  document.getElementById("container")!.appendChild(textDiv);
  return textDiv;
}

function getHumanReadableTime(analyzeResult: AnalyzeResult, frame: number) {
  let actualMs = Math.max(
    0,
    10 * frame - (analyzeResult.lagBeforeStartMs + analyzeResult.startMs)
  );
  actualMs = Math.min(actualMs, analyzeResult.displayedMs);
  return msToHumanReadable(actualMs);
}

function msToHumanReadable(ms: number) {
  return new Date(ms).toISOString().slice(14, 22).replace(".", ":");
}

function getRawString(block: Uint8Array) {
  const arr = new Array<number>();
  const dataView = new DataView(block.buffer, 0);
  for (let i = 0; i < block.length-1; i += 4) {
    arr.push(dataView.getFloat32(i, true));
  }

  const rowCount = 7; // Number of rows to display
  const itemsPerRow = Math.ceil(arr.length / rowCount);

  // Format numbers to fixed width
  let formattedNumbers = arr.map((n) => n.toFixed(3));
  const maxWidth = Math.max(10, ...formattedNumbers.map((n) => n.length));
  formattedNumbers = formattedNumbers
    .map((value) => value.padStart(maxWidth, " "))
    .map((value, index) =>
      index >= 11 && index <= 22 ? `<mark>${value}</mark>` : value
    );

  const rows = new Array<string>();
  for (let i = 0; i < arr.length; i += itemsPerRow) {
    rows.push(formattedNumbers.slice(i, i + itemsPerRow).join(", "));
  }

  return (
    `frame: ${block.length} bytes, ${arr.length} 4-byte floats:\n` +
    rows.join("\n")
  );
}

function updateCharacterRotation(characterGroup: Group, row: RowData) {
  const rotation3x3 = row.rotation3x3;

  // Convert the 3×3 to a full 4×4 matrix
  const matrix = new Matrix4().set(
    rotation3x3[0][0],
    rotation3x3[0][1],
    rotation3x3[0][2],
    0,
    rotation3x3[1][0],
    rotation3x3[1][1],
    rotation3x3[1][2],
    0,
    rotation3x3[2][0],
    rotation3x3[2][1],
    rotation3x3[2][2],
    0,
    0,
    0,
    0,
    1
  );

  // Extract Euler angles from that matrix (assuming an XYZ rotation order)
  const euler = new Euler().setFromRotationMatrix(matrix, "XYZ");

  // --- Fix pitch (looking "up" instead of "down") ---
  euler.x = -euler.x;
  // --- Fix yaw (turning "left" instead of "right") ---
  euler.y = -euler.y;

  // Recreate the matrix from our modified Euler
  const fixedMatrix = new Matrix4().makeRotationFromEuler(euler);

  // Set the group’s rotation from this fixed matrix
  characterGroup.setRotationFromMatrix(fixedMatrix);
}

let resyncCount = 0;
function tryResync(frameToRender: number) {
  if (videoTarget.videoTarget === undefined) {
    return;
  }

  if (!config.syncWithVideo) return;

  const driftInfo = getDriftSeconds(frameToRender);
  if (
    driftInfo &&
    Math.abs(driftInfo.actualSeconds - driftInfo.expectedSeconds) >
      SYNC_THRESHOLD
  ) {
    console.log(
      `Synced video to replay, precision: ${SYNC_THRESHOLD}, times: ${++resyncCount}.`
    );
    resync(driftInfo.actualSeconds);
  }
}

function resync(actualSeconds?: number): { expectedFrame: number } | undefined {
  if (!videoTarget.videoTarget) {
    return undefined;
  }
  actualSeconds ??= videoTarget.videoTarget!.getCurrentTime();
  const expectedFrame = (actualSeconds! - config.offsetSeconds) * 100;
  playerState.resumedTime = performance.now() - 10 * expectedFrame;
  return { expectedFrame };
}

function getDriftSeconds(
  frameToRender: number
): { expectedSeconds: number; actualSeconds: number } | undefined {
  const expectedSeconds = config.offsetSeconds + frameToRender / 100;
  if (!videoTarget.videoTarget) {
    return undefined;
  }
  const actualSeconds = videoTarget.videoTarget.getCurrentTime();

  return { expectedSeconds, actualSeconds };
}

function setMeshPosition(mesh: { position: PositionXYZ }, row: RowData) {
  const transformed = transformPosition(row);
  mesh.position.x = transformed.x;
  mesh.position.y = transformed.y;
  mesh.position.z = transformed.z;
}

function transformPosition(position: PositionXYZ): PositionXYZ {
  return {
    x: -position.x,
    y: -position.y,
    z: position.z,
  };
}

function getHeaderHtml(analyzeResult: AnalyzeResult) {
  const {timingDataFromHeader } = analyzeResult;
  const didFinishRun = timingDataFromHeader.totalTimeToFinishMs !== 0;
  const cp1String = timingDataFromHeader.checkpoint1TotalMs !== 0
    ? msToHumanReadable(analyzeResult.checkpoint1Ms - 10)
    : "N/A (No CP1 data recorded)"
  const cp2 = getCheckpoint2Collision(analyzeResult);
  const cp2String = cp2
    ? msToHumanReadable(cp2.frame2 * 10 - analyzeResult.timingDataFromHeader.crossStartPlusStartDelayMs)
    : (analyzeResult.trackName === UNKNOWN_TRACK ? "N/A (Unknown Track)" : "N/A (Could not find CP2)");
  const levelScore = analyzeResult.trackName !== UNKNOWN_TRACK
    ? analyzeResult.trackScoreData?.levelScores[0]
    : undefined;

  const timeString = didFinishRun
    ? msToHumanReadable(analyzeResult.displayedMs)
    : "N/A (Did not finish)";
  const startTime = msToHumanReadable(analyzeResult.startMs);
  const totalTime = didFinishRun
    ? msToHumanReadable(analyzeResult.totalMs)
    : "N/A (Did not finish)";

  console.log(analyzeResult);
  const allCollisions = [levelScore?.scoreData.firstValidCheckPoint1Collision, cp2, levelScore?.scoreData.firstValidFinishPointCollision];

  const invalidRunReason = getInvalidRunReason(analyzeResult);

  return `\
Player: ${analyzeResult.playerName}
Track : ${analyzeResult.trackName}
CP1   : ${cp1String}${getCollisionText(allCollisions, levelScore?.scoreData.firstValidCheckPoint1Collision, `Distance to Check_Point_1, must be ≤${PLANE_RADIUS}m`)}
CP2   : ${cp2String}${getCollisionText(allCollisions, cp2, `Distance to Check_Point_2, must be ≤${PLANE_RADIUS}m`)}
Time  : ${timeString}${getCollisionText(allCollisions, levelScore?.scoreData.firstValidFinishPointCollision, `Distance to Finish_Point, must be ≤${PLANE_RADIUS}m`)}

Legitimate Run? : ${invalidRunReason ? `<span style='color:red'>No (${invalidRunReason})</span>` : "Yes"}
Start Time      : ${startTime}${getCollisionText([], levelScore?.scoreData.firstValidStartPointCollision, `Distance to Start_Point, must be ≤${PLANE_RADIUS}m`)}
Total Time      : ${totalTime}
Lag before start: ${msToHumanReadable(analyzeResult.lagBeforeStartMs)}${getNearestStartPlaneText(analyzeResult)}
Leg after finish: ${msToHumanReadable(analyzeResult.lagAfterFinishMs)}
Recording time  : ${msToHumanReadable(analyzeResult.recordingMs)}
`
}

function getInvalidRunReason(analyzer: AnalyzeResult): string | undefined {
  if (analyzer.trackName === UNKNOWN_TRACK) return "UnknownTrack";
  const score = analyzer.trackScoreData?.levelScores[0].score;
  if (score !== MAX_SCORE) {
    return `Score ${score} is not max score of ${MAX_SCORE}`;
  }
  return undefined;
}

function getCheckpoint2Collision(analyzeResult: AnalyzeResult): PlaneCollisionInfo | undefined {
  if (analyzeResult.trackName === UNKNOWN_TRACK) return undefined;

  const levelScore = analyzeResult.trackScoreData?.levelScores[0];
  if (!levelScore) return undefined;

  const { scoreData } = levelScore;
  const validCheckpoint2 = levelScore.scoreData.validCheckPoint2Collisions?.at(0);
  if (validCheckpoint2) {
    return validCheckpoint2;
  }

  const { firstValidStartPointCollision, firstValidCheckPoint1Collision, firstValidFinishPointCollision} = scoreData;
  const candidateCheckpoint2Collisions = levelScore.scoreData.allCheckPoint2Collisions
    .filter(t => !firstValidStartPointCollision || firstValidStartPointCollision.frame2 <= t.frame1)
    .filter(t => !firstValidCheckPoint1Collision || firstValidCheckPoint1Collision.frame2 <= t.frame1)
    .filter(t => !firstValidFinishPointCollision || t.frame2 <= firstValidFinishPointCollision.frame2);
  return minBy(candidateCheckpoint2Collisions, t => t.distance);
}

function getCollisionText(allCollisions: Array<PlaneCollisionInfo | undefined>, collision: PlaneCollisionInfo | undefined, caption: string) {
  if (!collision) return "";
  const maxLength = Math.max(...allCollisions.map(t => t?.distance.toFixed(2).length ?? 0));
  const text = ` (${collision.distance.toFixed(2).padStart(maxLength, ' ')}m)`;
  return getTextWithCaption(text, caption, collision.distance > PLANE_RADIUS);
}

function getNearestStartPlaneText(analyzeResult: AnalyzeResult) {
  if (analyzeResult.trackName === UNKNOWN_TRACK) return "";
  const levelScore = analyzeResult.trackScoreData?.levelScores[0];
  const text = ` (${levelScore?.scoreData.nearestStartDist.toFixed(2)}m)`;
  return getTextWithCaption(text, `Distance to nearest Start_Point, must be ≤${NEAREST_START_POINT_DIST}m`, (levelScore?.scoreData.nearestStartDist ?? 5) > 2);
}

function getTextWithCaption(text: string, caption: string, isWarning: boolean) {
  const style = isWarning
    ? "color:darkorange"
    : "color:#1a5f1a"; // dark forest green
  const iconCaptionSpan = `<span data-tooltip='${caption}'> ${createInfoIcon().outerHTML}</span>`;
  return `<span style='${style}'>${text}</span>${iconCaptionSpan}`;
}

function createInfoIcon() {
  const el = createElement(Info);
  el.style.width = "12px";
  el.style.height = "12px";
  el.style.verticalAlign = "middle";
  return el;
}

function getScoreHtml(analyzeResult: AnalyzeResult) {
  const levelScores = analyzeResult.trackScoreData?.levelScores;
  const scores = [...new Set(analyzeResult.trackScoreData?.levelScores.map(t => t.score))].sort((a,b) => b-a);

  const table1 = `
  <table>
    <thead>
      <th>Score</th>
      <th style='text-align: left'>Track</th>
    </thead>
    <tbody>
      ${scores
        .map((score) => {
          return `<tr>
            <td>${score}</td>
            <td>${levelScores?.filter(t => t.score === score).map(t => t.name).sort().join(", ")}</td>
          </tr>`;
        })
        .join("")}
    </tbody>
  </table>`;

  const table2 = getScoreBreakdownTable(analyzeResult);
  const table3 = getAllCollisionsTable(analyzeResult, levelScores![0]);
  const highTimes = getHighTimes(analyzeResult);
  return table1 + table2 + highTimes + table3;
}

function getHighTimes(analyzeResult: AnalyzeResult) {
  if (analyzeResult.trackName === UNKNOWN_TRACK) return "";
  const levelScore = analyzeResult.trackScoreData?.levelScores[0];
  if (levelScore?.score !== 5) return "";

  const cp2 = getCheckpoint2Collision(analyzeResult);
  const formatMsForHighTimes = (ms: number) => (ms/1000).toFixed(2);
  const highTimesString = [
    `"${analyzeResult.playerName}"`,
    formatMsForHighTimes(analyzeResult.checkpoint1Ms),
    formatMsForHighTimes(cp2!.frame2 * 10 - analyzeResult.timingDataFromHeader.crossStartPlusStartDelayMs),
    formatMsForHighTimes(analyzeResult.displayedMs),
    "2025/1/1",
    0
  ];
  return `
    <div style='margin-top:15px'>High Times</div>
    <pre>${highTimesString.join(",")},</pre>
`
}

function getScoreBreakdownTable(analyzeResult: AnalyzeResult) {
  if (analyzeResult.trackName === UNKNOWN_TRACK) return "";
  const levelScore = analyzeResult.trackScoreData?.levelScores[0];
  if (!levelScore) return "";

  const scoreData = levelScore.scoreData;
  const breakdown = getScoreBreakdown(scoreData);
  const table = `

  <div style='margin-top:15px'>Score breakdown for ${levelScore.name}</div>
  <table>
    <thead>
      <th>Metric</th>
      <th>Value</th>
    </thead>
    <tbody>
      ${breakdown.map((item) => `<tr><td>${item[0]}</td><td>${item[1].valid ? '✔️' : '❌'}</td></tr>`).join("")}
    </tbody>
  </table>
  <div style='font-size:10px'>Explanation of the track logic is in <a target='_blank' href='https://github.com/domsleee/SS-Dat-Info/wiki/Dat%E2%80%90Info-design-notes#determining-trackname-of-a-dat-file'>the wiki</a>.</div>`;
  return table;
}

function getAllCollisionsTable(analyzeResult: AnalyzeResult, levelScore: LevelScore) {
  if (!levelScore) return "";

  const allLevelCollisions = analyzeResult.trackScoreData!.allCollisions.find(t => t.name === levelScore.name)!.collisions!;
  const collisionCols = allLevelCollisions.map(t => {
    return [
      `${t.objectName}<br /><span style='font-size:8px'>(${t.plane.position.x.toFixed(2)},${t.plane.position.y.toFixed(2)},${t.plane.position.z.toFixed(2)})</span>`,
      t.frame1,
      t.frame2,
      t.distance.toFixed(2)
    ];
  });

  const table = `
  <div style='margin-top:15px'>Collisions for ${levelScore.name}</div>
  <table>
    <thead>
      <th>Plane</th>
      <th>Frame1</th>
      <th>Frame2</th>
      <th>Distance</th>
    </thead>
    <tbody>
      ${collisionCols.map(row => `<tr>${row.map(r => `<td>${r}</td>`).join("")}</tr>`).join("\n")}
    </tbody>
  </table`;
  return table;
}

main();
