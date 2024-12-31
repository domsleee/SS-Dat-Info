import * as THREE from "three";
import { getDomAH } from './data';
import { setupVideo } from "./video";

const offsetSeconds = 23;//0.3; // smaller number = threejs plays earlier
const videoTarget = setupVideo();
const playerState = {
  isPlaying: true,
  resumedFrame: 0,
  lastFrameRendered: -5000,
  lastFrameChecked: -5000,
};

function main() {
  const playPauseButton = document.getElementById("playPauseButton")!;

  playPauseButton.onclick = () => {
    if (playerState.isPlaying) {
      playerState.isPlaying = false;
      playPauseButton.innerHTML = playerState.isPlaying ? "Pause" : "Play";
      videoTarget.videoTarget?.pauseVideo();
    } else {
      playerState.resumedFrame = performance.now() - 10 * playerState.lastFrameRendered;
      playerState.isPlaying = true;
      videoTarget.videoTarget?.playVideo();
      playPauseButton.innerHTML = playerState.isPlaying ? "Pause" : "Play";
    }
  };

  const preText = document.getElementById("preText")!;
  const data = getDomAH().slice(2300, 30 * 100);

  const camera = new THREE.PerspectiveCamera(
    80,
    window.innerWidth / window.innerHeight,
    0.01,
    1000
  );

  const firstRow = data[0];
  camera.position.x = -data[0][0];
  camera.position.y = -(data[0][1]);
  camera.position.z = data[0][2] - 25;

  camera.lookAt(
    new THREE.Vector3(
      -firstRow[0],
      -firstRow[1],
      firstRow[2]
    )
  );

  camera.position.y -= 50;

  const scene = new THREE.Scene();

  const geometry = new THREE.BoxGeometry(20/4, 50/4, 30/4);
  const material = new THREE.MeshNormalMaterial();

  const mesh = new THREE.Mesh(geometry, material);
  setMeshPosition(mesh, firstRow);
  scene.add(mesh);

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(480, 360);
  renderer.setAnimationLoop(animation);
  document.getElementById("container")!.prepend(renderer.domElement);

  // animation
  function animation(time) {
    if (!playerState.isPlaying) {
      return;
    }

    const frameToRender = Math.floor((time - playerState.resumedFrame) / 10) % data.length;

    const row = data[frameToRender];
    //
    const [x, w, y, z] = [row[3], row[4], row[5], row[6]];
    const quaternion = new THREE.Quaternion(x, y, z, w);
    mesh.quaternion.copy(quaternion);

    setMeshPosition(mesh, row);

    const newHtml = `
        frame: ${frameToRender} / ${data.length}
        x: ${mesh.position.x}
        y: ${mesh.position.y}
        z: ${mesh.position.z}
        drift: ${JSON.stringify(getDriftSeconds(frameToRender))}

        quaternion: ${quaternion.x}, ${quaternion.y}, ${quaternion.z}, ${quaternion.w}
    `;

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

function tryResync(frameToRender) {
  if (videoTarget.videoTarget === undefined) {
    return;
  }

  const driftInfo = getDriftSeconds(frameToRender);
  if (driftInfo && Math.abs(driftInfo.actualSeconds - driftInfo.expectedSeconds) > 0.2) {
      videoTarget.videoTarget.seekTo(driftInfo.expectedSeconds);
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
