# SS Dat Info - AI Coding Agent Instructions

## Project Overview

This is a **game data reverse-engineering project** for Supreme Snowboarding (1999). It contains tools for:
- **Replay analysis** (`analyze/`) - Parse `.dat` replay files and score checkpoint collisions
- **3D mesh viewer** (`threejs/`) - Render HX mesh files with procedurally-generated UVs
- **Game launcher** (`Display_Config/`) - Tauri desktop app with C++ injector for QoL mods

Key insight: This project **reverse-engineers binary formats** using Cheat Engine memory dumps and Ghidra decompilation.

## Critical Project-Specific Patterns

### 1. HX Mesh File Format & UV Generation

**CRITICAL**: UV coordinates are **NOT stored in HX files** - they're generated procedurally at runtime using planar projection.

- HX files contain only: vertices (float32 triplets), triangle indices (uint32 triplets), per-triangle material IDs
- UV generation algorithm reverse-engineered from `meshViewerNotes/decompiled/sr.c` line 64537
- Game uses **Y-down left-handed** coordinate system (Y+ = down, Z+ = into screen)
- Different meshes use **different UV parameters** (not auto-detected):
  - `Editor_Cross`: scale=3.0 (3x3 tiling) for debug visualization
  - `board`: scale=1.0 (1x1 mapping) for unique textures
- V-axis is **inverted** in UV formula: `V = (-vertex.y - min_v) * scale_v + offset_v`
- See `meshViewerNotes/HX_FILE_FORMAT.md` for complete specification

**When modifying mesh rendering**:
1. Check `meshViewerNotes/SOLUTION_SUMMARY.md` for current understanding
2. Verify against game-extracted data in `threejs/src/meshViewer/gameExtracted_*.ts`
3. UV parameters must match per-mesh (not per-geometry-type)

### 2. Replay (.dat) File Format

Binary structure (analyzed in `analyze/src/analyzeReplay.ts`):
- Header: timing data (total recording time, checkpoint times, finish time)
- Player name: null-terminated UTF-8 string starting at offset 24
- Coordinate blocks: repeating 14-byte frames with position data
- Timing is in **10ms increments** (multiply by 10 for milliseconds)

**Checkpoint scoring algorithm** (`analyze/src/PlaneUtil/`):
- Plane collision detection using 3D line-segment intersection
- 90.5m radius validation (`PLANE_RADIUS` constant)
- Must cross Check_Point_1 before Check_Point_2 counts
- Track identification by comparing scores across all 12 levels
- Level checkpoint data extracted from game memory using Cheat Engine Lua scripts

### 3. Multi-Component Architecture

```
analyze/           - Bun library, exports analyzeReplay()
├─ Used by threejs/ as local package dependency
threejs/           - Vite app with mesh viewer + replay analyzer
├─ Deployed to GitHub Pages via `bun run deploy`
Display_Config/    - Tauri desktop app (Vue + Rust)
├─ Display_Config_Resources/ - C++ DLL injector (msbuild)
```

**Key dependencies**:
- All TypeScript uses **Bun** (not npm/yarn) - `bun install`, `bun run`, `bun test`
- Display_Config requires **Visual Studio C++** for injector build
- Build orchestration via `just` (Display_Config only)

## Developer Workflows

### Running Tests
```powershell
cd analyze && bun test        # Replay parsing tests
cd Display_Config/Display_Config && bun test  # Tauri app tests
cd Display_Config/Display_Config/src-tauri && cargo test  # Rust tests
```

### Linting & Type Checking
```powershell
cd analyze && bun run lint && bun run typecheck
cd threejs && bun run lint && bun run typecheck
cd Display_Config/Display_Config && bun run check:all  # Runs lint, test, typecheck, Rust clippy/fmt
```

### Development Servers
```powershell
cd threejs && bun dev                    # Vite dev server (mesh viewer + replay analyzer)
cd Display_Config/Display_Config && bun tauri dev  # Tauri desktop app hot reload
```

### Building & Deployment
```powershell
cd threejs && bun run deploy             # Build + deploy to GitHub Pages
cd Display_Config && just                # Build Display_Config.exe + Injector.exe
cd Display_Config && just display_config_helper  # Build just the injector
```

## Reverse Engineering Workflow

### Extracting Game Data with Cheat Engine

1. **Mesh UVs/vertices** (`meshViewerNotes/aiScripts/extractUVsVerified.lua`):
   - Set breakpoint at `srGFInterface::getModelByName` 
   - Load mesh in-game, capture EAX register (srMeshModel pointer)
   - Read offsets: +0x13C (UV array), +0x1DC (vertices), +0x234 (vertex count)
   - Save as TypeScript file in `threejs/src/meshViewer/`

2. **Level checkpoints** (`analyze/src/LevelData/README.md`):
   - Breakpoint at `Supreme_Game.dll+1a59b` (object creation)
   - Filter for `xxx_Point_*` and `Player_Start_Location_*` objects
   - Save as `.raw.ts` files with position coordinates

3. **Decompiled source** (`meshViewerNotes/decompiled/`):
   - Ghidra decompilation of game executables
   - Use to verify reverse-engineered algorithms
   - Function names are auto-generated (`FUN_XXXXXXXX`)
   - Cross-reference assembly (.asm) for exact offsets

## Code Style & Conventions

- **TypeScript**: Strict type checking enabled, use `tsgo` for type checking
- **Testing**: Bun test framework (`describe`, `test`, `expect`)
- **Linting**: ESLint with `@stylistic` plugin for formatting
- **Coordinate naming**: Use `PositionXYZ` interface for 3D points (from `analyze/src/types.ts`)
- **File organization**: Group related reverse-engineering docs in `meshViewerNotes/aiScripts/`

### Common Patterns

```typescript
// Replay analysis
import { analyzeReplay } from 'dat-analyze';
const result = analyzeReplay(datFileBytes, { skipCoords: false });

// HX mesh parsing
import { parseHXFile } from './parseHXFile';
const { vertices, indices } = parseHXFile(hxFileBuffer);

// Coordinate system conversion (Y-down to Y-up)
const yUpVertex = { x: vertex.x, y: -vertex.y, z: vertex.z };
```

## Project-Specific Gotchas

1. **Don't assume UV auto-detection works**: Each mesh type has explicit parameters (see `MESH_UV_PARAMS` pattern in `meshViewerNotes/SOLUTION_SUMMARY.md`)

2. **Negative V coordinates are intentional**: Game uses V ∈ [-3, 0] for RepeatWrapping with Y-down coordinates

3. **Timing precision**: All replay times are 10ms increments, not frame-based

4. **Build isolation**: Display_Config C++ components must be built on Windows (uses msbuild + VS toolchain)

5. **Documentation is ground truth**: Reverse-engineered specs in `meshViewerNotes/` supersede assumptions - always verify against decompiled source or game memory dumps

## Key Files for Context

- `meshViewerNotes/HX_FILE_FORMAT.md` - Complete HX format specification
- `meshViewerNotes/aiScripts/README.md` - Investigation history & Cheat Engine workflows
- `analyze/src/analyzeReplay.ts` - Replay binary format parsing logic
- `threejs/src/meshViewer/runMeshViewer.ts` - UV generation implementation
- `Display_Config/README.md` - Build system & development workflow

## External Dependencies

- **Supreme Snowboarding (1999)** - Game being reverse-engineered
- **Cheat Engine** - Memory inspection & Lua scripting
- **Ghidra** - Binary decompilation (see `meshViewerNotes/decompiled/`)
- **Tauri** - Desktop app framework (Rust + WebView)
- **Three.js** - 3D rendering library

## When Starting a New Feature

1. Search `meshViewerNotes/` for existing reverse-engineering documentation
2. Check if similar data has been extracted (see `gameExtracted_*.ts` files)
3. For binary formats, verify against decompiled source in `meshViewerNotes/decompiled/`
4. Add tests to the appropriate package (`analyze/`, `threejs/`, or `Display_Config/`)
5. Update relevant `.md` documentation with findings
