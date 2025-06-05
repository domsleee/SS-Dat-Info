import { BoxGeometry, CylinderGeometry, DoubleSide, Group, Mesh, MeshBasicMaterial, MeshNormalMaterial, SphereGeometry } from "three"; 

export function createCharacterGroup(scale = 0.04) {
  // Default to half size (0.5)
  const characterGroup = new Group();
  
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
  const frontHeadGeometry = new SphereGeometry(
    headRadius,
    headSegments,
    headSegments,
    0,
    Math.PI,
    0,
    Math.PI
  );
  const frontHeadMaterial = new MeshBasicMaterial({
    color: 0x00ff00,
    side: DoubleSide,
  });
  const frontHeadMesh = new Mesh(frontHeadGeometry, frontHeadMaterial);
  
  // Back of head (red)
  const backHeadGeometry = new SphereGeometry(
    headRadius,
    headSegments,
    headSegments,
    Math.PI,
    Math.PI,
    0,
    Math.PI
  );
  const backHeadMaterial = new MeshBasicMaterial({
    color: 0xff0000,
    side: DoubleSide,
  });
  const backHeadMesh = new Mesh(backHeadGeometry, backHeadMaterial);
  
  // Group for the complete head
  const headGroup = new Group();
  headGroup.add(frontHeadMesh);
  headGroup.add(backHeadMesh);
  headGroup.position.y = BASE.headY * scale;
  frontHeadMesh.castShadow = true;
  backHeadMesh.castShadow = true;
  
  // Neck
  const neckGeometry = new CylinderGeometry(
    BASE.neckRadius * scale,
    BASE.neckRadius * scale,
    BASE.neckHeight * scale,
    32
  );
  const neckMaterial = new MeshNormalMaterial();
  const neckMesh = new Mesh(neckGeometry, neckMaterial);
  neckMesh.position.y = BASE.neckY * scale;
  
  // Body
  const bodyGeometry = new BoxGeometry(
    BASE.bodyWidth * scale,
    BASE.bodyHeight * scale,
    BASE.bodyDepth * scale
  );
  const bodyMaterial = new MeshNormalMaterial();
  const bodyMesh = new Mesh(bodyGeometry, bodyMaterial);
  
  // Snowboard
  const boardGeometry = new BoxGeometry(
    BASE.boardWidth * scale,
    BASE.boardHeight * scale,
    BASE.boardLength * scale
  );
  const boardMaterial = new MeshBasicMaterial({ color: 0xffff00 });
  const boardMesh = new Mesh(boardGeometry, boardMaterial);
  boardMesh.position.y = BASE.boardY * scale;
  
  // Add all meshes to the group
  characterGroup.add(bodyMesh);
  characterGroup.add(neckMesh);
  characterGroup.add(headGroup);
  characterGroup.add(boardMesh);
  
  bodyMesh.castShadow = true;
  neckMesh.castShadow = true;
  boardMesh.castShadow = true;
  
  return characterGroup;
}