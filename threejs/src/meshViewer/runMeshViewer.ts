import * as THREE from 'three';
import { TGALoader } from 'three/examples/jsm/loaders/TGALoader.js';
import { parseHXFile } from './parseHXFile';
import { compareBoardUVs, analyzeCurrentParameters } from './compareBoardUVs';
import { gameExtracted_zeppelin } from './gameExtracted_zeppelin';

// ===== Three.js setup =====
let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let mesh: THREE.Mesh | null = null;
let wireframe: THREE.LineSegments | null = null;
let currentTexture: THREE.Texture | null = null;
let isDragging = false;
let previousMousePosition = { x: 0, y: 0 };
const currentProjectionMode: 'XY' | 'XZ' | 'YZ' | 'XY_FLIP' | 'XZ_FLIP' | 'YZ_FLIP' = 'XY_FLIP';  // Y-down coordinate system for game compatibility
let storedGeometryData: { vertices: Float32Array, indices: Uint32Array, mapping_scale_u: number, mapping_scale_v: number, sizeX: number, sizeY: number, sizeZ: number } | null = null;
const uvOffsetV = 0.0;  // Interactive offset for V axis (not used in box mapping)
const vScaleMultiplier = 1.0;  // Multiplier for V scale (not used in box mapping)

initScene();
setupKeyboardControls();
loadMeshFromHash();

// Listen for hash changes to support navigation
window.addEventListener('hashchange', () => {
  loadMeshFromHash();
});

function loadMeshFromHash() {
  const hash = window.location.hash.slice(1); // Remove '#'

  const defaults = {
    'planeAudience01': {
      hxPath: '/meshes/plane_audience_01/plane_aud_1.hx',
      tgaPath: '/meshes/plane_audience_01/plane_aud_1.tga',
    },
    'slopeHard': {
      hxPath: '/meshes/slope_hard/slope_hard.HX',
      tgaPath: '/meshes/slope_hard/hard.tga',
    },
    'editorCross': {
      hxPath: '/meshes/Editor_Cross/Editor_cross.HX',
      tgaPath: '/meshes/Editor_Cross/checker.tga',
    },
    'board': {
      hxPath: '/meshes/board/board.hx',
      tgaPath: '/meshes/board/Burton_Custom_\'10.tga',
    },
  }


  if (hash in defaults) {
    const { hxPath, tgaPath } = defaults[hash as keyof typeof defaults];
    loadTexture(hash, hxPath, tgaPath);
  } else if (hash === 'zeppelinGameExtracted') {
    loadZeppelinGameExtracted();
  } else {
    console.warn(`Unknown hash: ${hash}, defaulting to board`);
    loadTexture('board', defaults.board.hxPath, defaults.board.tgaPath);
  }
}

async function loadTexture(hash: string, hxPath: string, tgaPath: string) {
  console.log('[Hash] Loading texture');
  document.title = `Mesh Viewer - ${hash}`;

  const meshData = await fetch(hxPath);
  const loadedTgaImage = await fetch(tgaPath);
  if (meshData.ok && loadedTgaImage.ok) {
    const hxData = await meshData.arrayBuffer();
    const tgaData = await loadedTgaImage.blob();
    loadHXData(hxData);
    loadTGAData(tgaData);
  }
}

async function loadZeppelinGameExtracted() {
  console.log('[Hash] Loading game-extracted board mesh (ACTUAL GAME DATA)');
  document.title = 'Mesh Viewer - Board (Game Extracted)';
  
  // Use the extracted game data directly
  const data = gameExtracted_zeppelin;
  console.log(`✅ Loaded game-extracted mesh: ${data.vertexCount} vertices, ${data.triangleCount} triangles`);
  console.log(`   UV bounds: U=[${data.metadata.uvBounds.u.min}, ${data.metadata.uvBounds.u.max}], V=[${data.metadata.uvBounds.v.min}, ${data.metadata.uvBounds.v.max}]`);
  
  // CRITICAL: Convert from game's Y-down coordinate system to Three.js Y-up
  // Game uses Y-down (Y is negative, goes downward)
  // Three.js uses Y-up (Y is positive, goes upward)
  // We need to NEGATE the Y coordinates
  console.log('🔄 Converting from game Y-down to Three.js Y-up coordinate system...');
  console.log(`   Original Y range: ${data.metadata.bounds.y.min} to ${data.metadata.bounds.y.max}`);
  
  const positions = new Float32Array(data.positions.length);
  for (let i = 0; i < data.positions.length; i += 3) {
    positions[i] = data.positions[i];       // X - unchanged
    positions[i + 1] = -data.positions[i + 1]; // Y - NEGATE (flip Y axis)
    positions[i + 2] = data.positions[i + 2];   // Z - unchanged
  }
  
  console.log(`   Flipped Y range: ${-data.metadata.bounds.y.max} to ${-data.metadata.bounds.y.min}`);
  
  // Convert Uint16Array indices to Uint32Array to match the expected type
  const indices32 = new Uint32Array(data.indices);
  
  // Use the game's actual UVs - these should be correct as-is
  loadMesh(positions, indices32, data.uvs);
  
  // Also try to load the texture
  const boardTga = await fetch('/meshes/board/Burton_Custom_\'10.tga');
  if (boardTga.ok) {
    const tgaData = await boardTga.blob();
    loadTGAData(tgaData);
  }
}

function loadHXData(buffer: ArrayBuffer) {
  try {
    const data = parseHXFile(buffer);
    if (data.meshes.length > 0) {
      const mesh = data.meshes[0];
      loadMesh(mesh.vertices, mesh.indices, mesh.uvs);
    } else {
      throw new Error("No meshes found in file");
    }
  } catch (err) {
    setError("Error: " + (err instanceof Error ? err.message : String(err)));
    console.error(err);
  }
}

function loadTGAData(blob: Blob) {
  const url = URL.createObjectURL(blob);
  const loader = new TGALoader();
  loader.load(
    url,
    (texture) => {
      URL.revokeObjectURL(url);
      applyTexture(texture);
    },
    undefined,
    (err) => {
      URL.revokeObjectURL(url);
      setError(
        "TGA load error: " + (err instanceof Error ? err.message : String(err))
      );
      console.error(err);
    }
  );
}

function setupKeyboardControls() {
  // Keyboard controls disabled for now - box mapping uses fixed parameters
  console.log('[Controls] Box mapping active - keyboard adjustments disabled');
}

function initScene() {
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x1a1a2e);

  camera = new THREE.PerspectiveCamera(
    60,
    window.innerWidth / window.innerHeight,
    0.1,
    1000
  );
  camera.position.set(120, 90, 120);
  camera.lookAt(0, 0, 0);

  renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  scene.add(new THREE.AmbientLight(0xffffff, 0.6));
  const d1 = new THREE.DirectionalLight(0xffffff, 0.8);
  d1.position.set(100, 100, 100);
  scene.add(d1);
  const d2 = new THREE.DirectionalLight(0xffffff, 0.5);
  d2.position.set(-100, -50, -100);
  scene.add(d2);

  scene.add(new THREE.GridHelper(200, 20, 0x666666, 0x333333));
  scene.add(new THREE.AxesHelper(80));

  renderer.domElement.addEventListener("mousedown", (e: MouseEvent) => {
    isDragging = true;
    previousMousePosition = { x: e.clientX, y: e.clientY };
  });
  
  renderer.domElement.addEventListener("mousemove", (e: MouseEvent) => {
    if (isDragging && mesh) {
      const dx = e.clientX - previousMousePosition.x;
      const dy = e.clientY - previousMousePosition.y;
      mesh.rotation.y += dx * 0.01;
      if (wireframe) wireframe.rotation.y += dx * 0.01;
      mesh.rotation.x += dy * 0.01;
      if (wireframe) wireframe.rotation.x += dy * 0.01;
      previousMousePosition = { x: e.clientX, y: e.clientY };
    }
  });
  
  renderer.domElement.addEventListener("mouseup", () => {
    isDragging = false;
  });
  
  renderer.domElement.addEventListener("wheel", (e: WheelEvent) => {
    e.preventDefault();
    camera.position.multiplyScalar(1 + e.deltaY * 0.001);
  });

  window.addEventListener("resize", () => {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
  });

  animate();
}

function animate() {
  requestAnimationFrame(animate);
  if (!isDragging && mesh) {
    mesh.rotation.x += 0.006;
    if (wireframe) wireframe.rotation.x += 0.006;
  }
  renderer.render(scene, camera);
}


// ===== Per-face planar UVs =====
// Matches the SurRender engine's srModeler::planarMap algorithm
// Reference: sr.c line 64537, HX_FILE_FORMAT.md UV Generation section
function generateFaceUVs(geometry: THREE.BufferGeometry, scaleU: number, scaleV: number, sizeX: number, sizeY: number, projectionMode: 'XY' | 'XZ' | 'YZ' | 'XY_FLIP' | 'XZ_FLIP' | 'YZ_FLIP' = 'XY'): THREE.BufferGeometry {
  const isFlipped = projectionMode.includes('_FLIP');
  console.log(`\n=== generateFaceUVs (BOX MAPPING${isFlipped ? ' - Y-axis FLIPPED' : ''}) ===`);
  // Convert to non-indexed so each face gets unique verts/UVs
  if (geometry.index) geometry = geometry.toNonIndexed();

  const pos = geometry.attributes.position;
  const uvs = new Float32Array(pos.count * 2);

  // Calculate bounding box for the entire mesh (matching getAxialBounds in the engine)
  geometry.computeBoundingBox();
  const bbox = geometry.boundingBox!;
  
  console.log(`[UV Gen] BOX MAPPING - each face selects projection based on normal`);
  console.log(`[UV Gen] Bbox dimensions: X=${(bbox.max.x-bbox.min.x).toFixed(2)}, Y=${(bbox.max.y-bbox.min.y).toFixed(2)}, Z=${(bbox.max.z-bbox.min.z).toFixed(2)}`);
  
  // BOX MAPPING: Pre-calculate scale factors for ALL three axes
  // Each face will use the appropriate axes based on its normal
  const range_x = bbox.max.x - bbox.min.x;
  const range_y = bbox.max.y - bbox.min.y;
  const range_z = bbox.max.z - bbox.min.z;
  
  // ENGINE ALGORITHM (sr.c line 64585):
  // scale_u = mapping.scale_u / (bbox_max_u - bbox_min_u)
  // scale_v = mapping.scale_v / (bbox_max_v - bbox_min_v)
  // Direct division by bounding box range - NO maxDim normalization!
  const scale_x = range_x !== 0 ? scaleU / range_x : 0.0;
  const scale_y = range_y !== 0 ? scaleV / range_y : 0.0;  // FIX: Use scaleV for Y axis
  const scale_z = range_z !== 0 ? scaleU / range_z : 0.0;  // FIX: Use scaleU for Z axis
  
  console.log(`[UV Gen] Bbox ranges: X=${range_x.toFixed(2)}, Y=${range_y.toFixed(2)}, Z=${range_z.toFixed(2)}`);
  console.log(`[UV Gen] Scale factors: X=${scale_x.toFixed(6)}, Y=${scale_y.toFixed(6)}, Z=${scale_z.toFixed(6)}`);
  console.log(`[UV Gen] Input scaleU: ${scaleU}, scaleV: ${scaleV}`);
  
  // Offsets - game uses offset_u=0, offset_v=-scale to produce [0, scale] and [-scale, 0] ranges
  const offset_u = 0.0;
  const offset_v = -scaleV;
  
  console.log(`[UV Gen] Offsets: U=${offset_u.toFixed(2)}, V=${offset_v.toFixed(2)}`);
  console.log(`[UV Gen] Expected UV ranges: U=[${offset_u.toFixed(1)}, ${(scaleU + offset_u).toFixed(1)}], V=[${offset_v.toFixed(1)}, ${(scaleV + offset_v).toFixed(1)}]`);
  
  // Detect if mesh is elongated (board-like)
  const maxDim = Math.max(range_x, range_y, range_z);
  const minDim = Math.min(range_x, range_y, range_z);
  const midDim = range_x + range_y + range_z - maxDim - minDim;
  const elongationRatio = maxDim / midDim;
  const isElongatedMesh = elongationRatio > 3.0;
  
  if (isElongatedMesh) {
    console.log(`[UV Gen] Elongated mesh detected (${elongationRatio.toFixed(1)}:1) - using PER-AXIS box mapping`);
  } else {
    console.log(`[UV Gen] Cube-like mesh - using BOX MAPPING (per-triangle projection)`);
  }
  
  // Process each vertex
  // We need to handle front/back faces differently to avoid mirroring
  for (let i = 0; i < pos.count; i += 3) {
    // Get triangle vertices
    const v0_x = pos.getX(i);
    const v0_y = pos.getY(i);
    const v0_z = pos.getZ(i);
    const v1_x = pos.getX(i + 1);
    const v1_y = pos.getY(i + 1);
    const v1_z = pos.getZ(i + 1);
    const v2_x = pos.getX(i + 2);
    const v2_y = pos.getY(i + 2);
    const v2_z = pos.getZ(i + 2);
    
    // Calculate triangle normal to determine projection axis (BOX MAPPING)
    const edge1_x = v1_x - v0_x;
    const edge1_y = v1_y - v0_y;
    const edge1_z = v1_z - v0_z;
    const edge2_x = v2_x - v0_x;
    const edge2_y = v2_y - v0_y;
    const edge2_z = v2_z - v0_z;
    
    // Cross product to get normal
    const normal_x = edge1_y * edge2_z - edge1_z * edge2_y;
    const normal_y = edge1_z * edge2_x - edge1_x * edge2_z;
    const normal_z = edge1_x * edge2_y - edge1_y * edge2_x;
    
    // BOX MAPPING: Select projection axis based on dominant normal component
    // This is how the game handles 3D objects like Editor_Cross!
    // For elongated meshes (boards), use SINGLE projection plane instead
    // - If normal points mostly along Z → use XY projection
    // - If normal points mostly along Y → use XZ projection  
    // - If normal points mostly along X → use YZ projection
    
    let faceProjection: 'XY' | 'XZ' | 'YZ';
    
    if (isElongatedMesh) {
      // Elongated meshes (boards) still use box mapping, but primarily XY projection
      // The key difference is they use per-axis normalization instead of maxDim
      const abs_nx = Math.abs(normal_x);
      const abs_ny = Math.abs(normal_y);
      const abs_nz = Math.abs(normal_z);
      
      if (abs_nz >= abs_nx && abs_nz >= abs_ny) {
        faceProjection = 'XY';  // Normal points along Z → project onto XY plane
      } else if (abs_ny >= abs_nx && abs_ny >= abs_nz) {
        faceProjection = 'XZ';  // Normal points along Y → project onto XZ plane
      } else {
        faceProjection = 'YZ';  // Normal points along X → project onto YZ plane
      }
    } else {
      // Cube-like meshes use box mapping (per-triangle projection)
      const abs_nx = Math.abs(normal_x);
      const abs_ny = Math.abs(normal_y);
      const abs_nz = Math.abs(normal_z);
      
      if (abs_nz >= abs_nx && abs_nz >= abs_ny) {
        faceProjection = 'XY';  // Normal points along Z → project onto XY plane
      } else if (abs_ny >= abs_nx && abs_ny >= abs_nz) {
        faceProjection = 'XZ';  // Normal points along Y → project onto XZ plane
      } else {
        faceProjection = 'YZ';  // Normal points along X → project onto YZ plane
      }
    }
    
    // For debugging first few triangles
    if (i < 30) {
      console.log(`  Tri${i/3}: normal=(${normal_x.toFixed(1)},${normal_y.toFixed(1)},${normal_z.toFixed(1)}) → ${faceProjection} projection`);
    }
    
    // Process each vertex of the triangle
    for (let j = 0; j < 3; j++) {
      // BOX MAPPING: Use the projection axis selected for THIS face
      let vertex_u: number, vertex_v: number;
      let min_u: number, min_v: number;
      
      // For Y-down to Y-up conversion, flip Y when used
      const posY = isFlipped ? -pos.getY(i + j) : pos.getY(i + j);
      
      switch (faceProjection) {
        case 'XY':  // Normal points along Z
          vertex_u = pos.getX(i + j);
          vertex_v = posY;
          min_u = bbox.min.x;
          min_v = isFlipped ? -bbox.max.y : bbox.min.y;
          break;
        case 'XZ':  // Normal points along Y
          vertex_u = pos.getX(i + j);
          vertex_v = pos.getZ(i + j);
          min_u = bbox.min.x;
          min_v = bbox.min.z;
          break;
        case 'YZ':  // Normal points along X
          vertex_u = posY;
          vertex_v = pos.getZ(i + j);
          min_u = isFlipped ? -bbox.max.y : bbox.min.y;
          min_v = bbox.min.z;
          break;
      }
      
      // Engine algorithm (sr.c line 64585-64587):
      // U = ((vertex[axis_u] - bbox_min_u) * scale_u) + offset_u
      // V = ((-vertex[axis_v] - bbox_min_v) * scale_v) + offset_v
      // Where: scale_u = mapping.scale_u / (bbox_max_u - bbox_min_u)
      //        scale_v = mapping.scale_v / (bbox_max_v - bbox_min_v)
      // We already calculated scale_x, scale_y, scale_z using this formula above
      
      // Select the correct scale based on projection
      let scale_u_final: number, scale_v_final: number;
      switch (faceProjection) {
        case 'XY':
          scale_u_final = scale_x;
          scale_v_final = scale_y;
          break;
        case 'XZ':
          scale_u_final = scale_x;
          scale_v_final = scale_z;
          break;
        case 'YZ':
          scale_u_final = scale_y;
          scale_v_final = scale_z;
          break;
      }
      
      const u = (vertex_u - min_u) * scale_u_final + offset_u;
      const v = (-vertex_v - min_v) * scale_v_final + offset_v;
      
      // Debug first few UVs to compare with game data
      if ((i + j) < 12) {
        const px = pos.getX(i + j).toFixed(1);
        const py = pos.getY(i + j).toFixed(1);
        const pz = pos.getZ(i + j).toFixed(1);
        console.log(`  V${i+j}: pos=(${px},${py},${pz}) vertex_uv=(${vertex_u.toFixed(1)},${vertex_v.toFixed(1)}) [${faceProjection}] → UV=(${u.toFixed(3)},${v.toFixed(3)})`);
      }
      
      uvs[(i + j) * 2] = u;
      uvs[(i + j) * 2 + 1] = v;
    }
  }

  geometry.setAttribute("uv", new THREE.BufferAttribute(uvs, 2));
  return geometry;
}

// ===== Mesh/texture pipeline =====
function createDefaultTexture(): THREE.CanvasTexture {
  const c = document.createElement("canvas");
  c.width = 128;
  c.height = 128;
  const ctx = c.getContext("2d");
  if (ctx) {
    for (let y = 0; y < 8; y++) {
      for (let x = 0; x < 8; x++) {
        ctx.fillStyle = (x + y) % 2 === 0 ? "#ffffff" : "#ffffff";
        ctx.fillRect(x * 16, y * 16, 16, 16);
      }
    }
  }
  const t = new THREE.CanvasTexture(c);
  t.wrapS = THREE.RepeatWrapping;
  t.wrapT = THREE.RepeatWrapping;
  // Repeat is set by loadMesh based on UV source
  t.needsUpdate = true;
  return t;
}

function loadMesh(vertices: Float32Array, indices: Uint32Array, uvs?: Float32Array) {
  if (mesh) {
    scene.remove(mesh);
    if (wireframe) scene.remove(wireframe);
    mesh.geometry.dispose();
    if (wireframe) wireframe.geometry.dispose();
  }
  
  // Reset texture to default checker when loading a new mesh
  if (currentTexture) {
    currentTexture.dispose();
    currentTexture = null;
    console.log('[Texture] Cleared previous texture - using default checker');
  }

  let geometry = new THREE.BufferGeometry();
  let usingFileUVs = false;
  let uvSource = "";
  
  // Check if we have per-triangle UVs from the file
  if (uvs && uvs.length > 0) {
    console.log(`Using UVs from file: ${uvs.length / 2} UV coordinates (per-triangle format)`);
    
    // First create indexed geometry
    geometry.setAttribute("position", new THREE.BufferAttribute(vertices, 3));
    geometry.setIndex(new THREE.BufferAttribute(indices, 1));
    
    // Convert to non-indexed since UVs are per-triangle
    geometry = geometry.toNonIndexed();
    
    // Now set the per-triangle UVs
    geometry.setAttribute("uv", new THREE.BufferAttribute(uvs, 2));
    usingFileUVs = true;
    uvSource = "from file (per-triangle)";
  } else {
    console.log("No UVs in file, generating procedurally");
    
    // Set up indexed geometry
    geometry.setAttribute("position", new THREE.BufferAttribute(vertices, 3));
    geometry.setIndex(new THREE.BufferAttribute(indices, 1));

    // Compute bounds first to calculate UV scale
    geometry.computeBoundingBox();
    
    // Calculate UV scale based on model size
    const bbox = geometry.boundingBox!;
    const sizeX = bbox.max.x - bbox.min.x;
    const sizeY = bbox.max.y - bbox.min.y;
    const sizeZ = bbox.max.z - bbox.min.z;
    const maxSize = Math.max(sizeX, sizeY, sizeZ);
    
    // Engine algorithm: scale parameter defines the desired UV range after normalization
    // Formula: U = ((vertex.x - min.x) / range.x) * scale_u + offset_u
    // This means scale_u determines how many times the texture repeats
    // For example: scale_u = 3.0 means UVs go from 0 to 3 (texture repeats 3 times)
    
    // CRITICAL: Game uses negative V coordinates for RepeatWrapping!
    // - Y-down coordinate system in game vs Y-up in Three.js
    // - Game UVs: U=[0,3], V=[-3,0] (NEGATIVE V!)
    // - This allows RepeatWrapping to tile texture correctly
    
    // Engine's MappingInfo scale values (from memory analysis)
    // Editor_Cross uses 3x3 tiling with scale = 3.0
    // Board uses 1x1 tiling with scale = 1.0
    // Detect mesh type by aspect ratio to apply appropriate UV parameters
    
    // Calculate elongation - board has one dimension much larger than the other two
    const maxDim = Math.max(sizeX, sizeY, sizeZ);
    const minDim = Math.min(sizeX, sizeY, sizeZ);
    const midDim = sizeX + sizeY + sizeZ - maxDim - minDim;
    const elongationRatio = maxDim / midDim;  // Board: 148.5/36.7 = 4.04
    
    let mapping_scale_u: number;
    let mapping_scale_v: number;
    
    // Detect flat planes (billboards, audience sprites, etc.)
    // These have one dimension near zero (< 1.0 unit)
    const isFlatPlane = minDim < 1.0;
    
    if (isFlatPlane) {
      // Flat billboards/planes - scale UVs to preserve aspect ratio
      // For XY plane: if Y > X, then V coordinate should be scaled proportionally
      // This makes the texture map 1:1 with the mesh's actual proportions
      const aspectRatio = maxDim / midDim;  // e.g., 180.4 / 88.1 = 2.05
      if (sizeY > sizeX && sizeY > sizeZ) {
        // Y is largest - scale V coordinate
        mapping_scale_u = 1.0;
        mapping_scale_v = aspectRatio;  // V range will be larger to match height
      } else if (sizeX > sizeY && sizeX > sizeZ) {
        // X is largest - scale U coordinate
        mapping_scale_u = aspectRatio;
        mapping_scale_v = 1.0;
      } else {
        // Z is largest (shouldn't happen for flat plane, but handle it)
        mapping_scale_u = 1.0;
        mapping_scale_v = 1.0;
      }
      console.log(`[Mesh Type] Detected flat plane (thickness: ${minDim.toFixed(2)}, aspect: ${aspectRatio.toFixed(2)}:1) - UV scale U=${mapping_scale_u.toFixed(2)}, V=${mapping_scale_v.toFixed(2)}`);
    } else if (elongationRatio > 3.0) {
      // Elongated meshes (like snowboard ~4:1 ratio) - use 1x1 tiling
      mapping_scale_u = 1.0;
      mapping_scale_v = 1.0;
      console.log(`[Mesh Type] Detected elongated mesh (${elongationRatio.toFixed(1)}:1) - using 1x1 UV tiling`);
    } else {
      // Cubic meshes (like editor_cross) - use 3x3 tiling
      mapping_scale_u = 3.0;
      mapping_scale_v = 3.0;
      console.log(`[Mesh Type] Detected cube/box mesh (${elongationRatio.toFixed(1)}:1) - using 3x3 UV tiling`);
    }
    
    console.log(`Model bounds: ${sizeX.toFixed(1)} x ${sizeY.toFixed(1)} x ${sizeZ.toFixed(1)} (max: ${maxSize.toFixed(1)})`);
    console.log(`UV mapping with engine-compatible scales`);
    console.log(`UV mapping scales: U=${mapping_scale_u.toFixed(2)}, V=${mapping_scale_v.toFixed(2)}, offset_v=${uvOffsetV.toFixed(3)}, vScale=${vScaleMultiplier.toFixed(2)}x`);
    
    // Store geometry data for regeneration when switching projection modes
    storedGeometryData = { vertices, indices, mapping_scale_u, mapping_scale_v, sizeX, sizeY, sizeZ };
    
    // Generate planar UVs matching the engine's algorithm
    // The function will normalize to [0,1] then multiply by these scales
    // Use the current projection mode (can be changed with keyboard)
    geometry = generateFaceUVs(geometry, mapping_scale_u, mapping_scale_v, sizeX, sizeY, currentProjectionMode);
    
    // Debug: Show UV ranges
    const uvAttr = geometry.attributes.uv;
    if (uvAttr) {
      let minU = Infinity, maxU = -Infinity, minV = Infinity, maxV = -Infinity;
      for (let i = 0; i < uvAttr.count; i++) {
        const u = uvAttr.getX(i);
        const v = uvAttr.getY(i);
        minU = Math.min(minU, u);
        maxU = Math.max(maxU, u);
        minV = Math.min(minV, v);
        maxV = Math.max(maxV, v);
      }
      console.log('UV ranges (single XY projection):');
      console.log(`  U: ${minU.toFixed(4)} to ${maxU.toFixed(4)} (range: ${(maxU - minU).toFixed(4)})`);
      console.log(`  V: ${minV.toFixed(4)} to ${maxV.toFixed(4)} (range: ${(maxV - minV).toFixed(4)})`);
      
      console.log('First 10 UVs:');
      const posAttr = geometry.attributes.position;
      for (let i = 0; i < Math.min(10, uvAttr.count); i++) {
        const x = posAttr.getX(i);
        const y = posAttr.getY(i);
        const z = posAttr.getZ(i);
        const u = uvAttr.getX(i);
        const v = uvAttr.getY(i);
        console.log(`  V${i}: pos=(${x.toFixed(2)},${y.toFixed(2)},${z.toFixed(2)}) → UV=(${u.toFixed(4)},${v.toFixed(4)})`);
      }
      
      // Compare with extracted game data (if this is a board)
      // analyzeCurrentParameters(elongationRatio);
      // const positionsArray = new Float32Array(posAttr.array);
      // const uvsArray = new Float32Array(uvAttr.array);
      // compareBoardUVs(positionsArray, uvsArray);
    }
    
    uvSource = "generated planar (procedural)";
  }
  
  geometry.computeVertexNormals();

  const map = currentTexture || createDefaultTexture();
  
  // UVs are baked in or from file, so texture repeat is 1:1
  if (map) {
    map.repeat.set(1, 1);
    console.log("MAP")
    map.needsUpdate = true;
  }
  
  const material = new THREE.MeshStandardMaterial({
    map,
    side: THREE.DoubleSide,
    flatShading: false,
  });

  mesh = new THREE.Mesh(geometry, material);
  material.needsUpdate = true;
  if (geometry.attributes.uv) {
    geometry.attributes.uv.needsUpdate = true;
  }
  scene.add(mesh);

  const edges = new THREE.EdgesGeometry(geometry);
  const wmat = new THREE.LineBasicMaterial({ color: 0x00ff00 });
  wireframe = new THREE.LineSegments(edges, wmat);
  // scene.add(wireframe);

  setInfo(`Mesh Loaded
Vertices: ${vertices.length / 3}
Triangles: ${indices.length / 3}
UVs: ${uvSource}
Texture: ${currentTexture ? "TGA" : "Checker (default)"}
Using File UVs: ${usingFileUVs ? "Yes" : "No"}

<b>Box Mapping:</b> Enabled - 3x3 tiling`);
  setError("");
}

function applyTexture(texture: THREE.Texture | null) {
  if (!texture) {
    const errorEl = document.getElementById("error");
    if (errorEl) errorEl.textContent = "TGA load error: texture is undefined";
    return;
  }
  currentTexture = texture;
  
  // Check if this is a cube mesh that needs texture repeating
  const isCubeMesh = storedGeometryData && 
    Math.abs(storedGeometryData.sizeX - storedGeometryData.sizeY) < 0.1 && 
    Math.abs(storedGeometryData.sizeY - storedGeometryData.sizeZ) < 0.1;
  
  if (isCubeMesh) {
    // Cube meshes: allow texture repeating
    currentTexture.wrapS = THREE.RepeatWrapping;
    currentTexture.wrapT = THREE.RepeatWrapping;
    console.log('[Texture] Cube mesh detected - using RepeatWrapping');
  } else {
    // Other meshes: clamp to edge (no repeating)
    currentTexture.wrapS = THREE.ClampToEdgeWrapping;
    currentTexture.wrapT = THREE.ClampToEdgeWrapping;
    console.log('[Texture] Non-cube mesh - using ClampToEdgeWrapping');
  }
  
  // UVs have scale baked in, so texture repeat is always 1:1
  currentTexture.repeat.set(1, 1);
  currentTexture.offset.set(0, 0);  // No offset - map texture directly to UVs
  currentTexture.needsUpdate = true;

  if (mesh && mesh.material instanceof THREE.MeshStandardMaterial) {
    mesh.material.map = currentTexture;
    mesh.material.needsUpdate = true;
    
    // Debug: Log final texture settings
    console.log(`[Texture Applied] repeat=(${currentTexture.repeat.x}, ${currentTexture.repeat.y}), offset=(${currentTexture.offset.x}, ${currentTexture.offset.y}), wrapS=${currentTexture.wrapS}, wrapT=${currentTexture.wrapT}`);
  }

  const info = document.getElementById("info");
  if (!info) return;
  
  let dims = "unknown size";
  if (currentTexture.image) {
    const w = currentTexture.image.width;
    const h = currentTexture.image.height;
    if (w && h) dims = `${w}×${h}`;
    console.log("TGA texture size:", w, h);
  }
  if (info.innerHTML.includes("Mesh Loaded")) {
    if (/Texture:.*?<br>/.test(info.innerHTML)) {
      info.innerHTML = info.innerHTML.replace(
        /Texture:.*?<br>/,
        `Texture: TGA (${dims})<br>`
      );
    } else {
      info.innerHTML += `Texture: TGA (${dims})<br>`;
    }
  }
}

// ===== UI wiring =====
document.getElementById("hx-btn")?.addEventListener("click", () => {
  const input = document.getElementById("hx-input") as HTMLInputElement;
  input?.click();
});

document.getElementById("tga-btn")?.addEventListener("click", () => {
  const input = document.getElementById("tga-input") as HTMLInputElement;
  input?.click();
});

document.getElementById("hx-input")?.addEventListener("change", (e) => {
  const target = e.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = (event) => {
    const result = event.target?.result;
    if (!result || !(result instanceof ArrayBuffer)) {
      setError("Invalid file data");
      return;
    }
    loadHXData(result);
  };
  reader.readAsArrayBuffer(file);
});

document.getElementById("tga-input")?.addEventListener("change", (e) => {
  const target = e.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;
  loadTGAData(file);
});

function setInfo(html: string) {
  const info = document.getElementById("info");
  if (info) info.innerHTML = html.replace(/\n/g, "<br>");
}

function setError(text: string) {
  const error = document.getElementById("error");
  if (error) error.textContent = text;
}

