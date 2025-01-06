import * as THREE from "three";
import { getDomAH, getDomVM } from "./data";
import { setupVideo, videoIds } from "./video";
import { calculateAcceleration, getSpeed } from './coordUtil';
import { Buffer } from "buffer";
import { RowData } from "analyze/src/types";
import { SnowboardTrackAnalyzer } from "./snowboardTrackAnalyzer";
import { createCameraSetup } from "./cameraSetup";
import { createCharacterGroup } from "./characterGroup";
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

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
//   const ambientLight = new THREE.AmbientLight(0x404040, 0.9); // 0.5 intensity
// scene.add(ambientLight);

  const firstRow = data[0];
  // setCameraToRow(camera, firstRow, axesHelper);
  setCameraToRowEl!.onclick = () => {
    setCameraToRow(camera, data[playerState.lastFrameRendered]);
    renderFrame(0, playerState.lastFrameRendered);
  };

  const characterGroup = createCharacterGroup();
  setMeshPosition(characterGroup, firstRow);
  scene.add(characterGroup);

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(480, 360);
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

    if (frameToRender < 0 || frameToRender > data.length - 1) return;
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
    const offsetQuat = new THREE.Quaternion();
    offsetQuat.setFromEuler(new THREE.Euler(
      Math.PI/8,            // Lean forwarding a bit
      -Math.PI/16,          // Face right a bit
      0           // and here
    ));
    const finalQuat = new THREE.Quaternion();
    finalQuat.multiplyQuaternions(offsetQuat, quaternion);
    characterGroup.setRotationFromQuaternion(finalQuat);

    setMeshPosition(characterGroup, row);
    const speed = getSpeed(data, frameToRender);
    if (frameToRender > 0) {
      cameraSystem.updateCamera(data[frameToRender], data[frameToRender-1], characterGroup, speed);
    }

    const drift = getDriftSeconds(frameToRender);
    const newHtml = `frame: ${frameToRender} / ${data.length}
x: ${characterGroup.position.x}
y: ${characterGroup.position.y}
z: ${characterGroup.position.z}
accel(y): ${calculateAcceleration(data, frameToRender).toFixed(1)}
falling?: ${calculateAcceleration(data, frameToRender) > 4.5}
speed (km/h): ${speed.toFixed(0)}
drift (s): ${
  drift
    ? (drift.actualSeconds - drift.expectedSeconds).toFixed(3)
    : "N/A"
}

left: ${row.left}
movementState: ${row.movementState}
quaternion (w,x,y,z): ${quaternion.w}, ${quaternion.x}, ${
quaternion.y
}, ${quaternion.z}
    `;

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


function setCameraToRow(camera, row: RowData) {
  camera.position.x = -row.x - 1;
  camera.position.y = -row.y + 10;
  camera.position.z = row.z - 3;

  const lookAtDistance = 8; // Distance to look ahead
  camera.lookAt(new THREE.Vector3(-row.x, -row.y, row.z + lookAtDistance));

  camera.position.y -= 4;
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

main();
