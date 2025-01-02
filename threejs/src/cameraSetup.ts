import * as THREE from 'three';

interface CameraConfig {
  distance: number;      // Distance behind target
  height: number;        // Height above target
  smoothing: number;     // Camera movement smoothing (0-1)
  minDistance: number;   // Minimum follow distance
  maxDistance: number;   // Maximum follow distance
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
  distance: 2,
  height: 1.5,
  smoothing: 0.995,
  minDistance: 2,
  maxDistance: 2
};

export function createCameraSetup(
  camera: THREE.Camera,
  scene: THREE.Scene,
  config: Partial<CameraConfig> = {}
): CameraSetup {
  // Merge with default config
  const cameraConfig: CameraConfig = {
    ...DEFAULT_CONFIG,
    ...config
  };

  // Store current camera target position for smoothing
  const currentTarget = new THREE.Vector3();
  const currentPosition = new THREE.Vector3();

  // Motion trail setup
  const trailGeometry = new THREE.BufferGeometry();
  const trailMaterial = new THREE.LineBasicMaterial({
    color: 0x00ff00,
    transparent: true,
    opacity: 0.5
  });
  const trailPositions = new Float32Array(300); // 100 points * 3 coordinates
  trailGeometry.setAttribute('position', new THREE.BufferAttribute(trailPositions, 3));
  const trail = new THREE.Line(trailGeometry, trailMaterial);
  const trailPoints: THREE.Vector3[] = [];

  function updateCamera(
    row: Position,
    prevRow?: Position,
    characterGroup?: THREE.Group,
    speed?: number
  ): void {
    // Calculate direction
    const direction = new THREE.Vector3(
      prevRow ? (-row.x - -prevRow.x) : 0,
      prevRow ? (-row.y - -prevRow.y) : 0,
      prevRow ? (row.z - prevRow.z) : 1
    ).normalize();

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

    // Calculate desired camera position
    let dynamicDistance = cameraConfig.distance;
    if (speed !== undefined) {
      // Adjust distance based on speed
      dynamicDistance = THREE.MathUtils.clamp(
        cameraConfig.distance * (1 + speed / 10),
        cameraConfig.minDistance,
        cameraConfig.maxDistance
      );
    }

    const cameraOffset = new THREE.Vector3(
      -direction.x * dynamicDistance,
      cameraConfig.height,
      -direction.z * dynamicDistance
    );

    const targetPosition = new THREE.Vector3(
      -row.x + cameraOffset.x,
      -row.y + cameraOffset.y,
      row.z + cameraOffset.z
    );

    // Smooth camera movement
    currentPosition.lerp(targetPosition, cameraConfig.smoothing);
    camera.position.copy(currentPosition);

    // Smooth look-at target
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