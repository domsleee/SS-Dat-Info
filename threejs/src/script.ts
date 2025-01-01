import * as THREE from "three";
import { getDomAH } from './data';
import { setupVideo } from "./video";


const offsetSeconds = 0-0.3;//0.3; // smaller number = threejs plays earlier
const videoTarget = setupVideo();
const playerState = {
  isPlaying: true,
  resumedFrame: 0,
  lastFrameRendered: -5000,
  lastFrameChecked: -5000,
};

function main() {
  const playPauseButton = document.getElementById("playPauseButton")!;
  const backButton = document.getElementById("backButton")!;
  const forwardButton = document.getElementById("forwardButton")!;
  const playerRange = document.getElementById("playerRange")! as HTMLInputElement;
  const frameInfo = document.getElementById("frameInfo")! as HTMLDivElement;
  const setCameraToRowEl = document.getElementById('setCameraToFrame');

  playPauseButton.onclick = () => {
    if (playerState.isPlaying) {
      playerState.isPlaying = false;
      videoTarget.videoTarget?.pauseVideo();
    } else {
      playerState.resumedFrame = performance.now() - 10 * playerState.lastFrameRendered;
      playerState.isPlaying = true;
      videoTarget.videoTarget?.playVideo();
    }
    
    
    playPauseButton.innerHTML = `<i data-lucide="${playerState.isPlaying ? 'pause' : 'play'}"></i>`;
    lucide.createIcons();
  };

  forwardButton.onclick = () => {
    renderFrame(0, (playerState.lastFrameRendered + 1) % data.length);
  };

  backButton.onclick = () => {
    renderFrame(0, (playerState.lastFrameRendered - 1 + data.length) % data.length);
  };
  
  const preText = document.getElementById("preText")!;
  const data = getDomAH();
  
  playerRange.min = "0";
  playerRange.max = data.length.toString();

  const camera = new THREE.PerspectiveCamera(
    80,
    window.innerWidth / window.innerHeight,
    0.01,
    1000
  );
  
  const scene = new THREE.Scene();
  const axesHelper = new THREE.AxesHelper(1000);
  scene.add(axesHelper);
  
  const firstRow = data[0];
  setCameraToRow(camera, firstRow, axesHelper);
  setCameraToRowEl!.onclick = () => {
    setCameraToRow(camera, data[playerState.lastFrameRendered], axesHelper);
    renderFrame(0, playerState.lastFrameRendered);
  }

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
  playerRange.addEventListener('click', (e) => {
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

    const frameToRender = frame ?? (Math.floor((time - playerState.resumedFrame) / 10) % data.length);
    const row = data[frameToRender];
    //
    const [x, w, y, z] = [row[3], -row[4], -row[5], row[6]];
    // const [w, x, y, z] = [1, 0, 0, 1];
    const quaternion = new THREE.Quaternion(x, y, -z, -w);
    characterGroup.setRotationFromQuaternion(quaternion);

    setMeshPosition(characterGroup, row);

    const drift = getDriftSeconds(frameToRender);
    const newHtml = `
        frame: ${frameToRender} / ${data.length}
        x: ${characterGroup.position.x}
        y: ${characterGroup.position.y}
        z: ${characterGroup.position.z}
        drift: ${drift ? drift.actualSeconds - drift.expectedSeconds : 'N/A'}

        quaternion: ${quaternion.x}, ${quaternion.y}, ${quaternion.z}, ${quaternion.w}
    `;
    playerRange.value = frameToRender.toString();
    frameInfo.innerHTML = `Frame: ${frameToRender} / ${data.length}`;

    if (newHtml !== preText.innerHTML) {
      preText.innerHTML = newHtml;
    }

    
    if (videoTarget.videoTarget !== undefined && Math.abs(playerState.lastFrameChecked - frameToRender) > 100) {
      tryResync(frameToRender);
      playerState.lastFrameChecked = frameToRender;
    }
    
    playerState.lastFrameRendered = frameToRender;
    renderer.render(scene, camera);
  }
}

function createCharacterGroup() {
  const characterGroup = new THREE.Group();

  // Create front and back of head using half spheres
  const headRadius = 6;
  const headSegments = 32;
  
  // Front of head (green)
  const frontHeadGeometry = new THREE.SphereGeometry(
    headRadius, headSegments, headSegments, 
    -Math.PI/2, Math.PI  // Half circle from front
  );
  const frontHeadMaterial = new THREE.MeshBasicMaterial({ 
    color: 0x00FF00,
    side: THREE.DoubleSide  // Make sure both sides render
  });
  const frontHeadMesh = new THREE.Mesh(frontHeadGeometry, frontHeadMaterial);
  
  // Back of head (red)
  const backHeadGeometry = new THREE.SphereGeometry(
    headRadius, headSegments, headSegments, 
    Math.PI/2, Math.PI  // Half circle from back
  );
  const backHeadMaterial = new THREE.MeshBasicMaterial({ 
    color: 0xFF0000,
    side: THREE.DoubleSide  // Make sure both sides render
  });
  const backHeadMesh = new THREE.Mesh(backHeadGeometry, backHeadMaterial);

  // Group for the complete head
  const headGroup = new THREE.Group();
  headGroup.add(frontHeadMesh);
  headGroup.add(backHeadMesh);
  headGroup.position.y = -50/8 - 3;

  // Neck
  const neckGeometry = new THREE.CylinderGeometry(2, 2, 6, 32);
  const neckMaterial = new THREE.MeshNormalMaterial();
  const neckMesh = new THREE.Mesh(neckGeometry, neckMaterial);
  neckMesh.position.y = -50/8 + 1;

  // Body
  const bodyGeometry = new THREE.BoxGeometry(20/4, 50/4, 30/4);
  const bodyMaterial = new THREE.MeshNormalMaterial();
  const bodyMesh = new THREE.Mesh(bodyGeometry, bodyMaterial);

  // Snowboard
  const boardGeometry = new THREE.BoxGeometry(25/4, 2/4, 80/4);
  const boardMaterial = new THREE.MeshBasicMaterial({ color: 0xFFFF00 });
  const boardMesh = new THREE.Mesh(boardGeometry, boardMaterial);
  boardMesh.position.y = 50/8 + 1;

  // Add all meshes to the group
  characterGroup.add(bodyMesh);
  characterGroup.add(neckMesh);
  characterGroup.add(headGroup);
  characterGroup.add(boardMesh);

  return characterGroup;
}

function setCameraToRow(camera, row, axesHelper) {
  camera.position.x = -row[0];
  camera.position.y = -row[1];
  camera.position.z = row[2] - 25;

  camera.lookAt(
    new THREE.Vector3(
      -row[0],
      -row[1],
      row[2] + 20
    )
  );

  axesHelper.position.x = -row[0];
  axesHelper.position.y = -row[1]-25;
  axesHelper.position.z = row[2];

  camera.position.y -= 20;
}

function tryResync(frameToRender) {
  if (videoTarget.videoTarget === undefined) {
    return;
  }

  const driftInfo = getDriftSeconds(frameToRender);
  if (driftInfo && Math.abs(driftInfo.actualSeconds - driftInfo.expectedSeconds) > 0.1) {
      videoTarget.videoTarget.seekTo(driftInfo.expectedSeconds+0.18, true);
  }
}

function getDriftSeconds(frameToRender): { expectedSeconds: number, actualSeconds: number } | undefined {
  const expectedSeconds = offsetSeconds + frameToRender / 100;
  if (!videoTarget.videoTarget) {
    return undefined
  }
  const actualSeconds = videoTarget.videoTarget.getCurrentTime();

  return { expectedSeconds, actualSeconds }

}

function setMeshPosition(mesh, row) {
  mesh.position.x = -row[0];
  mesh.position.y = -row[1];
  mesh.position.z = row[2];
}

main();
