import './polyfills'
import * as THREE from "three";
import { getDomVM } from "./data";
import { setupVideo, videoIds } from "./video";
import { calculateAcceleration, getSpeed } from './coordUtil';
import { AnalyzeResult, RowData } from "analyze/src/types";
import { SnowboardTrackAnalyzer } from "./snowboardTrackAnalyzer";
import { createCameraSetup } from "./cameraSetup";
import { createCharacterGroup } from "./characterGroup";
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { parseLittleEndianFloat32 } from "analyze/src/analyzeReplay";

const dimensions = {
  width: 480*2,
  height: 360*2,
}

const offsetSeconds = 0 - 1.05; //0.2 for VM, 0.3 for AH; // smaller number = threejs plays earlier
const videoTarget = setupVideo(videoIds.vm109, dimensions);
const playerState = {
  isPlaying: true,
  resumedTime: 0,
  lastFrameRendered: -5000,
  lastFrameChecked: -5000,
};

async function main() {
  document.getElementById("mainContainer")!.style.width = `${dimensions.width * 2}px`;

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
  }

  playPauseButton.onclick = () => {
    if (playerState.isPlaying) {
      playerState.isPlaying = false;
      videoTarget.videoTarget?.pauseVideo();
    } else {
      playerState.resumedTime =
        performance.now() - 10 * playerState.lastFrameRendered;
      playerState.isPlaying = true;
      videoTarget.videoTarget?.playVideo();
    }

    playPauseButton.innerHTML = `<i data-lucide="${
      playerState.isPlaying ? "pause" : "play"
    }"></i>`;

    // @ts-expect-error its fine.
    lucide.createIcons();
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
  const analyzeResult = await getDomVM();
  document.getElementById("headerInfo")!.innerText = getHeaderText(analyzeResult)
  const data = analyzeResult.coords!.rows;

  playerRange.min = "0";
  playerRange.max = data.length.toString();

  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.01,
    1000
  );
  
  const scene = new THREE.Scene();
  const topOffset = 42 / 540 * dimensions.height;
  const topOffset2 = 124 / 540 * dimensions.height;
  const topOffset3 = 152 / 540 * dimensions.height;
  const leftTimeOffset = 22 / 720 * dimensions.width;
  const leftSpeedOffset = 20 / 720 * dimensions.width;
  const fontSize = 22 / 540 * dimensions.height;
  const paddingSize = 10 / 540 * dimensions.height;
  
  const nameText = createText(analyzeResult.playerName, { top: `${topOffset}px`, right: `${dimensions.width + leftTimeOffset}px`, fontSize: `${fontSize}px` });
  const timeText = createText(getHumanReadableTime(analyzeResult, 0), { top: `${topOffset + leftTimeOffset}px`, right: `${dimensions.width + leftTimeOffset}px`, fontSize: `${fontSize}px` });
  // const speedText = createText("Speed (km/h)", { top: '24px', left: '20px' });
  const speedValue = createText("000km/h", { top: `${topOffset3}px`, right: `${dimensions.width + leftTimeOffset}px`, fontSize: `${fontSize}px` });
  const speedValue2 = createText("000km/h", { top: `${topOffset2}px` , left: `${dimensions.width + leftSpeedOffset}px`, backgroundColor: 'black', letterSpacing: '1px', padding: `0 ${paddingSize}px`, fontSize: `${fontSize}px` });

  const keyControls = document.getElementById('keyControls')!;
  keyControls.style.bottom = '0';
  keyControls.style.right = `${dimensions.width}px`;

  const cameraSystem = createCameraSetup(camera, scene);

  const analyzer = new SnowboardTrackAnalyzer(data.map(c => transformPosition(c)), { accelerationThreshold: 0.05});
  const slopeMesh = analyzer.createSlopeMesh({ playerYOffset: 0.4});
  const jumpMarkers = analyzer.createJumpMarkers({ });

  scene.add(slopeMesh);
  console.log(jumpMarkers)
  jumpMarkers.forEach(marker => scene.add(marker));

  const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
  directionalLight.castShadow = true;
  scene.add(directionalLight);

  const firstRow = data[0];

  const characterGroup = createCharacterGroup();
  setMeshPosition(characterGroup, firstRow);
  scene.add(characterGroup);

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(dimensions.width, dimensions.height);
  renderer.setAnimationLoop(renderFrame);
  renderer.shadowMap.enabled = true;
  document.getElementById("container")!.prepend(renderer.domElement);

  const controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true; // Optional - adds smooth damping effect
  controls.dampingFactor = 0.05;
  controls.minDistance = 1;  // Minimum zoom distance
  controls.maxDistance = 50; // Maximum zoom distance

  // range
  playerRange.addEventListener("click", (e) => {
    const clickPosition = e.offsetX / playerRange.offsetWidth;
    const frame = Math.floor(clickPosition * data.length);
    playerState.resumedTime = performance.now() - 10 * frame;
    renderFrame(0, frame);
  });

  // animation
  function renderFrame(time, frame: number | undefined = undefined) {
    if (frame === undefined && !playerState.isPlaying) {
      return;
    }

    const frameToRender =
      frame ?? Math.floor((time - playerState.resumedTime) / 10) % data.length;

    if (frameToRender < 0 || frameToRender > data.length - 1) return;
    const row = data[frameToRender];
    updateCharacterRotation(characterGroup, row);

    setMeshPosition(characterGroup, row);
    const speed = getSpeed(data, frameToRender);
    if (frameToRender > 0) {
      cameraSystem.updateCamera(data[frameToRender], data[frameToRender-1], characterGroup, speed);
    }

    const drift = getDriftSeconds(frameToRender);
    const newHtml = `x: ${characterGroup.position.x}
y: ${characterGroup.position.y}
z: ${characterGroup.position.z}
accel(y): ${calculateAcceleration(data, frameToRender).toFixed(1)}
falling?: ${calculateAcceleration(data, frameToRender) > 4.5}
drift (s): ${
  drift
    ? (drift.actualSeconds - drift.expectedSeconds).toFixed(3)
    : "N/A"
}
rotation3x3:
[
  ${row.rotation3x3[0].map(n => n.toFixed(3)).join(', ')}
  ${row.rotation3x3[1].map(n => n.toFixed(3)).join(', ')}
  ${row.rotation3x3[2].map(n => n.toFixed(3)).join(', ')}
]
${getRawString(row.raw)}`;

    setKeyInputState(keys, row);
    playerRange.value = frameToRender.toString();
    frameInfo.innerHTML = `Frame: ${frameToRender} / ${data.length}`;
    timeText.textContent = getHumanReadableTime(analyzeResult, frameToRender);
    speedValue.textContent = `${Math.floor(getSpeed(data, frameToRender)).toFixed(0).padStart(3, '0')} km/h`;;
    speedValue2.textContent = speedValue.textContent.replace(' ', '')

    if (newHtml !== preText.innerHTML) {
      preText.innerHTML = newHtml;
    }

    if (
      videoTarget.videoTarget !== undefined &&
      Math.abs(playerState.lastFrameChecked - frameToRender) > 100
    ) {
      tryResync(frameToRender);
    }

    playerState.lastFrameRendered = frameToRender;
    renderer.render(scene, camera);
  }
}

function setKeyInputState(keys: any, row: RowData) {
  return; // todo
  if (row.left) {
    keys.left.classList.add("pressed");
  } else {
    keys.left.classList.remove("pressed");
  }
  
  if (row.right) {
    keys.right.classList.add("pressed");
  } else {
    keys.right.classList.remove("pressed");
  }

  if (row.shift) {
    keys.shift.classList.add("pressed");
  } else {
    keys.shift.classList.remove("pressed");
  }
}

function createText(text: string, style?: Partial<CSSStyleDeclaration>) {
  const textDiv = document.createElement('div');
  textDiv.style.position = 'absolute';
  textDiv.style.color = 'white';
  textDiv.style.fontSize = '22px';  
  Object.assign(textDiv.style, style);
  textDiv.textContent = text;
  document.getElementById("container")!.appendChild(textDiv);
  return textDiv;
}


function getHumanReadableTime(analyzeResult: AnalyzeResult, frame: number) {
  let actualMs = Math.max(0, 10 * frame - ((analyzeResult.lagBeforeStartMs + analyzeResult.startMs)));
  actualMs = Math.min(actualMs, analyzeResult.displayedMs);
  return msToHumanReadable(actualMs);
}

function msToHumanReadable(ms: number) {
  return new Date(ms).toISOString().slice(14, 22).replace('.', ':');
}

function getRawString(raw: string) {
  const arr = new Array<number>();
  for (let i = 0; i < raw.length; i += 8) {
    arr.push(parseLittleEndianFloat32(raw.slice(i, i + 8)));
  }

  const rowCount = 4; // Number of rows to display
  const itemsPerRow = Math.ceil(arr.length / rowCount);

  // Format numbers to fixed width
  let formattedNumbers = arr.map(n => n.toFixed(3));
  const maxWidth = Math.max(...formattedNumbers.map(n => n.length));
  formattedNumbers = formattedNumbers.map(value => value.padStart(maxWidth, ' ')).map((value, index) => index >= 11 && index <= 22 ? `<mark>${value}</mark>` : value);

  const rows = new Array<string>();
  for (let i = 0; i < arr.length; i += itemsPerRow) {
    rows.push(
      formattedNumbers
        .slice(i, i + itemsPerRow)
        .join(', ')
    );
  }

  return `${raw.length/2} bytes, ${arr.length} 4-byte floats:\n` + rows.join('\n');
}

function updateCharacterRotation(characterGroup: THREE.Group, row: RowData) {
  const rotation3x3 = row.rotation3x3;

  // Convert the 3×3 to a full 4×4 matrix
  const matrix = new THREE.Matrix4().set(
    rotation3x3[0][0], rotation3x3[0][1], rotation3x3[0][2], 0,
    rotation3x3[1][0], rotation3x3[1][1], rotation3x3[1][2], 0,
    rotation3x3[2][0], rotation3x3[2][1], rotation3x3[2][2], 0,
    0,                0,                0,                1
  );

  // Extract Euler angles from that matrix (assuming an XYZ rotation order)
  const euler = new THREE.Euler().setFromRotationMatrix(matrix, 'XYZ');

  // --- Fix pitch (looking "up" instead of "down") ---
  euler.x = -euler.x;
  // --- Fix yaw (turning "left" instead of "right") ---
  euler.y = -euler.y;

  // Recreate the matrix from our modified Euler
  const fixedMatrix = new THREE.Matrix4().makeRotationFromEuler(euler);

  // Set the group’s rotation from this fixed matrix
  characterGroup.setRotationFromMatrix(fixedMatrix);
}

function tryResync(frameToRender) {
  if (videoTarget.videoTarget === undefined) {
    return;
  }

  const driftInfo = getDriftSeconds(frameToRender);
  if (
    driftInfo &&
    Math.abs(driftInfo.actualSeconds - driftInfo.expectedSeconds) > 0.1
  ) {
    const expectedFrame = (driftInfo.actualSeconds - offsetSeconds)  * 100;
    playerState.resumedTime = performance.now() - 10 * expectedFrame;
  }
}

function getDriftSeconds(
  frameToRender: number
): { expectedSeconds: number; actualSeconds: number } | undefined {
  const expectedSeconds = offsetSeconds + frameToRender / 100;
  if (!videoTarget.videoTarget) {
    return undefined;
  }
  const actualSeconds = videoTarget.videoTarget.getCurrentTime();

  return { expectedSeconds, actualSeconds };
}

function setMeshPosition(mesh, row: RowData) {
  const transformed = transformPosition(row);
  mesh.position.x = transformed.x;
  mesh.position.y = transformed.y;
  mesh.position.z = transformed.z;
}

function transformPosition(position) {
  return {
    x: -position.x,
    y: -position.y,
    z: position.z,
  }
}

function getHeaderText(analyzeResult: AnalyzeResult) {
  return `\
Player: ${analyzeResult.playerName}
Track : ${analyzeResult.trackName}
Time  : ${msToHumanReadable(analyzeResult.displayedMs)}
CP1   : ${msToHumanReadable(analyzeResult.checkpoint1Ms)}
`;
}

main();
