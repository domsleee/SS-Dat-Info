import * as THREE from 'three';

interface CameraConfig {
  distance: number;
  height: number;
  smoothing: number;
}

interface CameraSetup {
  trail: THREE.Line;
  trailPoints: THREE.Vector3[];
  updateCamera: (row: Position, prevRow?: Position, characterGroup?: THREE.Group, speed?: number) => void;
  updateConfig: (newConfig: Partial<CameraConfig>) => void;
  getConfig: () => CameraConfig;
}

interface Position {
  x: number;
  y: number;
  z: number;
}

const DEFAULT_CONFIG: CameraConfig = {
  distance: 1.5,
  height: 1.5,
  smoothing: 0.995,
};

export function createCameraSetup(
  camera: THREE.Camera,
  scene: THREE.Scene,
  config: Partial<CameraConfig> = {}
): CameraSetup {
  const cameraConfig: CameraConfig = {
    ...DEFAULT_CONFIG,
    ...config
  };

  // Store current camera target position for smoothing
  const currentTarget = new THREE.Vector3();
  const currentPosition = new THREE.Vector3();
  
  // Store last valid direction
  const lastDirection = new THREE.Vector3(0, 0, 1);

  // Motion trail setup
  const trailGeometry = new THREE.BufferGeometry();
  const trailMaterial = new THREE.LineBasicMaterial({
    color: 0x00ff00,
    transparent: true,
    opacity: 0.5
  });
  const trailPositions = new Float32Array(300);
  trailGeometry.setAttribute('position', new THREE.BufferAttribute(trailPositions, 3));
  const trail = new THREE.Line(trailGeometry, trailMaterial);
  const trailPoints: THREE.Vector3[] = [];

  function updateCamera(
    row: Position,
    prevRow?: Position,
    characterGroup?: THREE.Group,
    speed?: number
  ): void {
    // Calculate direction, with fallback to last valid direction
    const newDirection = new THREE.Vector3();
    if (prevRow) {
      newDirection.set(
        -row.x - -prevRow.x,
        -row.y - -prevRow.y,
        row.z - prevRow.z
      );
      
      // Only update direction if movement is significant
      if (newDirection.lengthSq() > 0.01) {
        newDirection.normalize();
        lastDirection.copy(newDirection);
      }
    }

    // Update trail
    trailPoints.unshift(new THREE.Vector3(-row.x, -row.y, row.z));
    if (trailPoints.length > 100) trailPoints.pop();
    
    const positions = trail.geometry.attributes.position.array;
    for (let i = 0; i < trailPoints.length; i++) {
      positions[i * 3] = trailPoints[i].x;
      positions[i * 3 + 1] = trailPoints[i].y;
      positions[i * 3 + 2] = trailPoints[i].z;
    }
    trail.geometry.attributes.position.needsUpdate = true;

    // Use fixed distance regardless of speed
    const cameraOffset = new THREE.Vector3(
      -lastDirection.x * cameraConfig.distance,
      cameraConfig.height,
      -lastDirection.z * cameraConfig.distance
    );

    const targetPosition = new THREE.Vector3(
      -row.x + cameraOffset.x,
      -row.y + cameraOffset.y,
      row.z + cameraOffset.z
    );

    // Apply smoothing to camera position
    currentPosition.lerp(targetPosition, cameraConfig.smoothing);
    camera.position.copy(currentPosition);

    // Update look-at target with smoothing
    const lookAtTarget = new THREE.Vector3(-row.x, -row.y, row.z);
    currentTarget.lerp(lookAtTarget, cameraConfig.smoothing);
    camera.lookAt(currentTarget);
  }

  function updateConfig(newConfig: Partial<CameraConfig>): void {
    Object.assign(cameraConfig, newConfig);
  }

  return {
    trail,
    trailPoints,
    updateCamera,
    updateConfig,
    getConfig: () => ({...cameraConfig})
  };
}