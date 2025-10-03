/**
 * HX File Format Parser V2
 * 
 * Based on actual file analysis of editor_cross.hx
 * 
 * The HX format is a serialized object dump with markers for data sections.
 * Key findings:
 * - "mesh\0" marker at offset 0x524 is followed immediately by triangle indices
 * - Indices are stored as int32 triplets (12 bytes per triangle)
 * - Vertices are stored as float32 triplets (12 bytes per vertex)
 * - The file contains memory pointers that are not useful for parsing
 */

export interface HXMeshData {
  vertices: Float32Array;
  indices: Uint32Array;
  normals?: Float32Array;
  uvs?: Float32Array;
  name?: string;
}

export interface HXParseResult {
  meshes: HXMeshData[];
  metadata?: Record<string, unknown>;
}

export class HXParser {
  private view: DataView;
  private bytes: Uint8Array;
  private fileSize: number;

  constructor(arrayBuffer: ArrayBuffer) {
    this.view = new DataView(arrayBuffer);
    this.bytes = new Uint8Array(arrayBuffer);
    this.fileSize = arrayBuffer.byteLength;
  }

  parse(): HXParseResult {
    // Validate magic
    this.validateMagic();

    // Find mesh marker
    const meshMarkerOffset = this.findString('mesh\0', 0);
    if (meshMarkerOffset === -1) {
      throw new Error('No mesh marker found in file');
    }

    console.log(`[HX Parser] Found mesh marker at offset ${meshMarkerOffset} (0x${meshMarkerOffset.toString(16)})`);

    // Parse geometry data after mesh marker
    const mesh = this.parseGeometryAfterMarker(meshMarkerOffset + 5); // +5 to skip "mesh\0"

    return {
      meshes: [mesh],
      metadata: {
        fileSize: this.fileSize,
        meshMarkerOffset,
      }
    };
  }

  private validateMagic(): void {
    const magic = String.fromCharCode(...this.bytes.slice(0, 23));
    if (!magic.startsWith('SurRender database file')) {
      throw new Error(`Invalid HX file magic: "${magic.substring(0, 20)}..."`);
    }
  }

  private findString(needle: string, startOffset: number): number {
    const needleBytes = new TextEncoder().encode(needle);
    
    for (let i = startOffset; i <= this.bytes.length - needleBytes.length; i++) {
      let match = true;
      for (let j = 0; j < needleBytes.length; j++) {
        if (this.bytes[i + j] !== needleBytes[j]) {
          match = false;
          break;
        }
      }
      if (match) return i;
    }
    
    return -1;
  }

  private parseGeometryAfterMarker(offset: number): HXMeshData {
    console.log(`[HX Parser] Parsing geometry at offset ${offset} (0x${offset.toString(16)})`);

    // Read indices first (they come right after mesh marker)
    const { indices, endOffset } = this.readIndicesHeuristic(offset);
    console.log(`[HX Parser] Found ${indices.length / 3} triangles, indices end at offset ${endOffset} (0x${endOffset.toString(16)})`);

    // Vertices come right after indices
    const vertices = this.readVerticesAfterIndices(endOffset, indices);
    console.log(`[HX Parser] Found ${vertices.length / 3} vertices`);

    // Try to read per-triangle UVs (chunk 0x05 format)
    // Format: 6 floats per triangle (3 vertices × 2 UV coords each)
    // NOTE: After investigation, the data at this location doesn't appear to be proper UVs
    // (values like 33.7494 are way outside normal UV range). Disabling for now.
    const uvs = undefined; // this.readPerTriangleUVs(uvDataOffset, triangleCount, indices);
    
    console.log(`[HX Parser] No UV data found in file - will need to generate procedurally`);

    return {
      vertices,
      indices,
      uvs,
    };
  }

  private readIndicesHeuristic(offset: number): { indices: Uint32Array, endOffset: number } {
    const indices: number[] = [];
    let currentOffset = offset;

    // Read int32 triplets until we hit something that doesn't look like an index
    while (currentOffset + 12 <= this.fileSize) {
      const i1 = this.view.getInt32(currentOffset, true);
      const i2 = this.view.getInt32(currentOffset + 4, true);
      const i3 = this.view.getInt32(currentOffset + 8, true);

      // Check if these look like valid indices
      if (this.isReasonableIndex(i1) && this.isReasonableIndex(i2) && this.isReasonableIndex(i3)) {
        indices.push(i1, i2, i3);
        currentOffset += 12;
      } else {
        break;
      }
    }

    return {
      indices: new Uint32Array(indices),
      endOffset: currentOffset
    };
  }

  private readVerticesAfterIndices(offset: number, indices: Uint32Array): Float32Array {
    // Find the maximum index to know how many vertices we need
    const maxIndex = Math.max(...indices);
    const requiredVertices = maxIndex + 1;

    console.log(`[HX Parser] Need at least ${requiredVertices} vertices (max index: ${maxIndex})`);
    console.log(`[HX Parser] Looking for vertices starting at offset ${offset} (0x${offset.toString(16)})`);

    // Try reading vertices right at this offset
    const vertices = this.tryReadVerticesAt(offset, requiredVertices);
    if (vertices) {
      return vertices;
    }

    throw new Error(`Could not find vertex data for ${requiredVertices} vertices at offset ${offset}`);
  }

  private readUVs(offset: number, vertexCount: number): Float32Array {
    const uvs = new Float32Array(vertexCount * 2);
    
    for (let i = 0; i < vertexCount; i++) {
      const uvOffset = offset + i * 8; // 2 floats per UV
      
      if (uvOffset + 8 > this.fileSize) {
        console.warn(`[HX Parser] UVs truncated at vertex ${i}`);
        break;
      }

      const u = this.view.getFloat32(uvOffset, true);
      const v = this.view.getFloat32(uvOffset + 4, true);

      // Basic validation: UVs should be reasonable values
      if (!this.isReasonableUV(u, v)) {
        console.warn(`[HX Parser] Suspicious UV at vertex ${i}: (${u}, ${v})`);
        // Continue anyway, might be valid
      }

      uvs[i * 2] = u;
      uvs[i * 2 + 1] = v;
    }

    return uvs;
  }

  private readPerTriangleUVs(offset: number, triangleCount: number): Float32Array | undefined {
    // Per-triangle UV format: 6 floats per triangle (24 bytes)
    // Each triangle has 3 UV pairs (one for each vertex)
    const expectedBytes = triangleCount * 24;
    
    if (offset + expectedBytes > this.fileSize) {
      return undefined; // Not enough data
    }

    // Read per-triangle UVs
    const perTriangleUVs: number[][] = [];
    let allReasonable = true;
    
    for (let i = 0; i < triangleCount; i++) {
      const triOffset = offset + i * 24;
      const triangleUVs: number[] = [];
      
      for (let v = 0; v < 3; v++) {
        const u = this.view.getFloat32(triOffset + v * 8, true);
        const v_coord = this.view.getFloat32(triOffset + v * 8 + 4, true);
        
        if (!this.isReasonableUV(u, v_coord)) {
          allReasonable = false;
        }
        
        triangleUVs.push(u, v_coord);
      }
      
      perTriangleUVs.push(triangleUVs);
    }

    if (!allReasonable) {
      return undefined; // Data doesn't look like UVs
    }

    // Convert per-triangle UVs to per-vertex format for Three.js
    // Since we're converting to non-indexed geometry anyway for per-face UVs,
    // we need to duplicate vertices and assign UVs per triangle vertex
    // NOTE: The source code negates V coordinates (lines 3464, 3469, 3473 in HMG_3DE.c)
    const uvArray = new Float32Array(triangleCount * 3 * 2); // 3 vertices per tri, 2 coords per UV
    
    for (let i = 0; i < triangleCount; i++) {
      const triUVs = perTriangleUVs[i];
      for (let v = 0; v < 3; v++) {
        uvArray[(i * 3 + v) * 2] = triUVs[v * 2];        // U coordinate (no change)
        uvArray[(i * 3 + v) * 2 + 1] = -triUVs[v * 2 + 1]; // V coordinate (NEGATED)
      }
    }

    return uvArray;
  }

  private isReasonableUV(u: number, v: number): boolean {
    return !isNaN(u) && !isNaN(v) &&
           isFinite(u) && isFinite(v) &&
           Math.abs(u) < 100 && Math.abs(v) < 100; // UVs can be outside [0,1] for tiling
  }

  private tryReadVerticesAt(offset: number, count: number): Float32Array | null {
    const vertices = new Float32Array(count * 3);
    
    for (let i = 0; i < count; i++) {
      const voffset = offset + i * 12;
      
      if (voffset + 12 > this.fileSize) {
        return null;
      }

      const x = this.view.getFloat32(voffset, true);
      const y = this.view.getFloat32(voffset + 4, true);
      const z = this.view.getFloat32(voffset + 8, true);

      if (!this.isReasonableVertex(x, y, z)) {
        return null;
      }

      vertices[i * 3] = x;
      vertices[i * 3 + 1] = y;
      vertices[i * 3 + 2] = z;
    }

    return vertices;
  }

  private isReasonableVertex(x: number, y: number, z: number): boolean {
    return !isNaN(x) && !isNaN(y) && !isNaN(z) &&
           isFinite(x) && isFinite(y) && isFinite(z) &&
           Math.abs(x) < 10000 && Math.abs(y) < 10000 && Math.abs(z) < 10000;
  }

  private isReasonableIndex(idx: number): boolean {
    return Number.isInteger(idx) && idx >= 0 && idx < 100000;
  }

  hexDump(offset: number, length: number): string {
    const end = Math.min(offset + length, this.fileSize);
    let output = '';
    
    for (let i = offset; i < end; i += 16) {
      const hex = Array.from(this.bytes.slice(i, Math.min(i + 16, end)))
        .map(b => b.toString(16).padStart(2, '0'))
        .join(' ');
      const ascii = Array.from(this.bytes.slice(i, Math.min(i + 16, end)))
        .map(b => (b >= 32 && b < 127) ? String.fromCharCode(b) : '.')
        .join('');
      output += `${i.toString(16).padStart(8, '0')}: ${hex.padEnd(48, ' ')} ${ascii}\n`;
    }
    
    return output;
  }
}

export function parseHXFile(arrayBuffer: ArrayBuffer): HXParseResult {
  const parser = new HXParser(arrayBuffer);
  return parser.parse();
}
