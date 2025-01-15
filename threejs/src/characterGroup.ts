import * as THREE from "three";
import { Mesh } from "three"; 

export function createCharacterGroup(scale = 0.04) {
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
    frontHeadMesh.castShadow = true;
    backHeadMesh.castShadow = true;
  
    // Neck
    const neckGeometry = new THREE.CylinderGeometry(
      BASE.neckRadius * scale,
      BASE.neckRadius * scale,
      BASE.neckHeight * scale,
      32
    );
    const neckMaterial = new THREE.MeshNormalMaterial();
    const neckMesh = new Mesh(neckGeometry, neckMaterial);
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
  
    bodyMesh.castShadow = true;
    neckMesh.castShadow = true;
    boardMesh.castShadow = true;
  
    return characterGroup;
  }