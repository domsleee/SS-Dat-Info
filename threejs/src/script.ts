import * as THREE from "three";
import { getDomAH, getDomVM } from "./data";
import { setupVideo, videoIds } from "./video";

import { Buffer } from "buffer";
import { RowData } from "analyze/src/types";

window.Buffer = Buffer;

const offsetSeconds = 0 - 0.2; //0.3 for AH; // smaller number = threejs plays earlier
const videoTarget = setupVideo(videoIds.vm);
const playerState = {
  isPlaying: true,
  resumedFrame: 0,
  lastFrameRendered: -5000,
  lastFrameChecked: -5000,
};

async function main() {
  const playPauseButton = document.getElementById("playPauseButton")!;
  const backButton = document.getElementById("backButton")!;
  const forwardButton = document.getElementById("forwardButton")!;
  const playerRange = document.getElementById(
    "playerRange"
  )! as HTMLInputElement;
  const frameInfo = document.getElementById("frameInfo")! as HTMLDivElement;
  const setCameraToRowEl = document.getElementById("setCameraToFrame");

  playPauseButton.onclick = () => {
    if (playerState.isPlaying) {
      playerState.isPlaying = false;
      videoTarget.videoTarget?.pauseVideo();
    } else {
      playerState.resumedFrame =
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
  const data = await getDomVM();

  playerRange.min = "0";
  playerRange.max = data.length.toString();


  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.01,
    1000
  );

  const scene = new THREE.Scene();
  const axesHelper = new THREE.AxesHelper(1000);
  scene.add(axesHelper);

  const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
  directionalLight.position.set(5, 5, 5);
  directionalLight.castShadow = true;
  scene.add(directionalLight);

  const pointLight1 = new THREE.PointLight(0x2196f3, 1, 10);
  pointLight1.position.set(-2, 2, -2);
  scene.add(pointLight1);

  const pointLight2 = new THREE.PointLight(0xff9800, 1, 10);
  pointLight2.position.set(2, 2, 2);
  scene.add(pointLight2);

  const firstRow = data[0];
  setCameraToRow(camera, firstRow, axesHelper);
  setCameraToRowEl!.onclick = () => {
    setCameraToRow(camera, data[playerState.lastFrameRendered], axesHelper);
    renderFrame(0, playerState.lastFrameRendered);
  };

  const characterGroup = createCharacterGroup();
  // const geometry = new THREE.CapsuleGeometry(10/4, 15/4, 4, 8);  // radius, length, capSegments, radialSegments
  const material = new THREE.MeshNormalMaterial();

  // const mesh = new THREE.Mesh(characterGroup, material);
  setMeshPosition(characterGroup, firstRow);
  scene.add(characterGroup);

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(480, 360);
  renderer.setAnimationLoop(renderFrame);
  document.getElementById("container")!.prepend(renderer.domElement);

  // range
  playerRange.addEventListener("click", (e) => {
    const clickPosition = e.offsetX / playerRange.offsetWidth;
    const frame = Math.floor(clickPosition * data.length);
    playerState.resumedFrame = performance.now() - 10 * frame;
    renderFrame(0, frame);
  });

  // animation
  function renderFrame(time, frame: number | undefined = undefined) {
    if (frame === undefined && !playerState.isPlaying) {
      return;
    }

    const frameToRender =
      frame ?? Math.floor((time - playerState.resumedFrame) / 10) % data.length;
    const row = data[frameToRender];
    // best is w,y,x,z
    const [x, w, y, z] = [
      -Number(row.rw.toFixed(5)),
      Number(row.rx.toFixed(5)),
      -Number(row.ry.toFixed(5)),
      Number(row.rz.toFixed(5)),
    ];
    // const [w, x, y, z] = [1, 0, 0, 1];
    const quaternion = new THREE.Quaternion(x, y, z, w);
    characterGroup.setRotationFromQuaternion(quaternion);

    setMeshPosition(characterGroup, row);

    const drift = getDriftSeconds(frameToRender);
    const newHtml = `frame: ${frameToRender} / ${data.length}
x: ${characterGroup.position.x}
y: ${characterGroup.position.y}
z: ${characterGroup.position.z}
speed (km/h): ${getSpeed(data, frameToRender).toFixed(0)}
drift (s): ${
  drift
    ? (drift.actualSeconds - drift.expectedSeconds).toFixed(3)
    : "N/A"
}

quaternion (w,x,y,z): ${quaternion.w}, ${quaternion.x}, ${
quaternion.y
}, ${quaternion.z}
    `;
    playerRange.value = frameToRender.toString();
    frameInfo.innerHTML = `Frame: ${frameToRender} / ${data.length}`;

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
}

function createCharacterGroup(scale = 0.2) {
  // Default to half size (0.5)
  const characterGroup = new THREE.Group();

  // Base dimensions (at scale = 1.0)
  const BASE = {
    headRadius: 4,
    neckRadius: 2,
    neckHeight: 6,
    bodyWidth: 5,
    bodyHeight: 12.5,
    bodyDepth: 7.5,
    boardWidth: 6.25,
    boardHeight: 0.5,
    boardLength: 20,
    // Y positions
    headY: 15,
    neckY: 9,
    boardY: -7,
  };

  // Create front and back of head using half spheres
  const headRadius = BASE.headRadius * scale;
  const headSegments = 32;

  // Front of head (green)
  const frontHeadGeometry = new THREE.SphereGeometry(
    headRadius,
    headSegments,
    headSegments,
    0,
    Math.PI,
    0,
    Math.PI
  );
  const frontHeadMaterial = new THREE.MeshBasicMaterial({
    color: 0x00ff00,
    side: THREE.DoubleSide,
  });
  const frontHeadMesh = new THREE.Mesh(frontHeadGeometry, frontHeadMaterial);

  // Back of head (red)
  const backHeadGeometry = new THREE.SphereGeometry(
    headRadius,
    headSegments,
    headSegments,
    Math.PI,
    Math.PI,
    0,
    Math.PI
  );
  const backHeadMaterial = new THREE.MeshBasicMaterial({
    color: 0xff0000,
    side: THREE.DoubleSide,
  });
  const backHeadMesh = new THREE.Mesh(backHeadGeometry, backHeadMaterial);

  // Group for the complete head
  const headGroup = new THREE.Group();
  headGroup.add(frontHeadMesh);
  headGroup.add(backHeadMesh);
  headGroup.position.y = BASE.headY * scale;

  // Neck
  const neckGeometry = new THREE.CylinderGeometry(
    BASE.neckRadius * scale,
    BASE.neckRadius * scale,
    BASE.neckHeight * scale,
    32
  );
  const neckMaterial = new THREE.MeshNormalMaterial();
  const neckMesh = new THREE.Mesh(neckGeometry, neckMaterial);
  neckMesh.position.y = BASE.neckY * scale;

  // Body
  const bodyGeometry = new THREE.BoxGeometry(
    BASE.bodyWidth * scale,
    BASE.bodyHeight * scale,
    BASE.bodyDepth * scale
  );
  const bodyMaterial = new THREE.MeshNormalMaterial();
  const bodyMesh = new THREE.Mesh(bodyGeometry, bodyMaterial);

  // Snowboard
  const boardGeometry = new THREE.BoxGeometry(
    BASE.boardWidth * scale,
    BASE.boardHeight * scale,
    BASE.boardLength * scale
  );
  const boardMaterial = new THREE.MeshBasicMaterial({ color: 0xffff00 });
  const boardMesh = new THREE.Mesh(boardGeometry, boardMaterial);
  boardMesh.position.y = BASE.boardY * scale;

  // Add all meshes to the group
  characterGroup.add(bodyMesh);
  characterGroup.add(neckMesh);
  characterGroup.add(headGroup);
  characterGroup.add(boardMesh);

  return characterGroup;
}

function setCameraToRow(camera, row: RowData, axesHelper) {
  camera.position.x = -row.x - 1;
  camera.position.y = -row.y + 10;
  camera.position.z = row.z - 3;

  const lookAtDistance = 8; // Distance to look ahead
  camera.lookAt(new THREE.Vector3(-row.x, -row.y, row.z + lookAtDistance));

  axesHelper.position.x = -row.x;
  axesHelper.position.y = -row.y - 5;
  axesHelper.position.z = row.z;

  camera.position.y -= 4;
}

function getSpeed(
  data: Array<RowData>,
  frameToRender: number,
  frameRate: number = 100
) {
  if (frameToRender === 0) return 0;

  const r0 = data[frameToRender - 1];
  const r1 = data[frameToRender];

  const dx = r1.x - r0.x;
  const dy = r1.y - r0.y;
  const dz = r1.z - r0.z;
  const distance = Math.sqrt(dx * dx + dy * dy + dz * dz);

  const timeBetweenFramesHours = 1 / (frameRate * 3600);
  const speed = distance / 1000 / timeBetweenFramesHours;

  return speed;
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
    videoTarget.videoTarget.seekTo(driftInfo.expectedSeconds + 0.18, true);
  }
}

function getDriftSeconds(
  frameToRender
): { expectedSeconds: number; actualSeconds: number } | undefined {
  const expectedSeconds = offsetSeconds + frameToRender / 100;
  if (!videoTarget.videoTarget) {
    return undefined;
  }
  const actualSeconds = videoTarget.videoTarget.getCurrentTime();

  return { expectedSeconds, actualSeconds };
}

function setMeshPosition(mesh, row: RowData) {
  mesh.position.x = -row.x;
  mesh.position.y = -row.y;
  mesh.position.z = row.z;
}

main();
