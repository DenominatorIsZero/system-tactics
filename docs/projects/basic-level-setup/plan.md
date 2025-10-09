# Basic Level Setup - Implementation Plan

## Overview

Implement the foundational 3D hex grid level system as specified in `specs/basic-level-setup.md`. This plan breaks down the implementation into discrete, sequential tasks that establish the visual foundation for tactical gameplay.

## Implementation Strategy

**Goal**: Get something visible on screen as early as possible, then incrementally improve it.

### Phase 1: Minimal Visible Prototype

Get a simple 3D hex grid rendering in the game with basic camera.

### Phase 2: Proper Materials and Positioning

Add correct colors, materials, and hex positioning.

### Phase 3: Full Feature Set

Add camera controls, level editor integration, and polish.

## Task Breakdown

### Task 1: Basic Bevy 3D Setup ✅

**Goal**: Get a simple 3D scene running to verify Bevy setup works.

- [x] Add basic 3D camera to game/src/main.rs
- [x] Add simple lighting (ambient + directional)
- [x] Spawn a single test cube or cylinder to verify 3D rendering works
- [x] Run `just run-game` to see something on screen
- [x] Add tracing logs for better debugging visibility

**Test**: `just run-game` shows a 3D scene with a simple shape and lighting. ✅

### Task 2: Single Static Hex Column ✅

**Goal**: Replace test shape with one hex column using hexx.

- [x] Study hexx 3D columns example briefly
- [x] Add hexx dependency to workspace (already available)
- [x] Create simple hex column mesh using hexx (height=1.0, fixed position)
- [x] Replace test shape with hex column
- [x] Use UV checker texture from hexx examples for better visualization
- [x] Move functionality to shared library with plugin architecture
- [x] Configure workspace-level assets to avoid duplication
- [x] Remove unnecessary config structs and simplify code

**Test**: `just run-game` shows a single 3D hex column with UV checker texture. ✅

### Task 3: Grid of Hex Columns ✅

**Goal**: Show multiple hexes arranged in a small grid.

- [x] Create simple loop to spawn 10x10 grid of hex columns
- [x] All hexes and material but different height
- [x] Use rectangular grid layout (0-9 coordinates)
- [x] Implement height gradient from front-left (low) to back-right (high)
- [x] Adjust camera and hex scale for optimal visibility

**Test**: `just run-game` shows 10x10 grid of hex columns in proper hexagonal layout. ✅

### Task 4: Basic Camera Controls ✅

**Goal**: Add simple camera movement to navigate the hex grid.

- [x] Add WASD camera movement (no bounds checking yet)
- [x] Add mouse wheel zoom (basic implementation)
- [x] Fixed isometric angle with proper orthographic projection
- [x] No fancy bounds or easing, just functional movement
- [x] Fixed hex grid alignment using pointy orientation
- [x] Camera movement aligned with viewing perspective

**Test**: Can move camera around the hex grid with WASD and zoom with mouse wheel. ✅

### Task 5: Proper Colors and Materials ✅

**Goal**: Make it look good with the specified color scheme.

- [x] Add color constants (created shared/src/colors.rs with tactical RPG palette)
- [x] Create proper materials with gray surface and green edges
- [x] Apply to all hex columns
- [x] Set white background for clean tactical view

**Test**: Hex grid looks good with gray surfaces, green edges, and proper background. ✅

### Task 6: Level Data Structure

**Goal**: Replace hardcoded grid with actual Level struct and add level management system.

#### Subtask 6.1: Basic Level Struct and Display ✅
- [x] Add toml and ndarray dependencies to shared/Cargo.toml
- [x] Create Level struct with grid data and constructor in shared/src/level.rs
  - Grid size parameters (width, height)
  - Height data using ndarray::Array2<f32>
  - Level name field
  - Level::new(name, width, height) constructor that replicates current gradient
- [x] Add UI text system to display level name in bottom-right corner
- [x] Replace hardcoded hex grid generation with Level resource
- [x] Export from shared/src/lib.rs

**Test**: Game still shows same 10x10 grid but driven by Level struct, with level name displayed. ✅

#### Subtask 6.2: Level Serialization and Asset Loading
- [ ] Add TOML serialization support with serde derives
- [ ] Create assets/levels/ directory structure
- [ ] Create sample level files (default.toml, test_small.toml, test_large.toml)
- [ ] Implement level loading system that reads all TOML files from assets/levels/
- [ ] Add LevelsResource containing Vec<Level> of all available levels
- [ ] Add current level index tracking

**Test**: Game loads levels from TOML files and can access multiple levels.

#### Subtask 6.3: Level Cycling System
- [ ] Implement left/right arrow key input handling
- [ ] Add level switching system that:
  - Despawns current hex grid entities
  - Loads new level from LevelsResource
  - Spawns new hex grid based on selected level
  - Updates level name display
- [ ] Add smooth transitions between levels

**Test**: Can cycle through multiple levels with left/right arrows, each showing different sizes and names.

### Task 7: Variable Heights

**Goal**: Show different height hexes to prove the system works.

- [ ] Update Level to support different heights per hex
- [ ] Create test level with varied heights (some 1, some 2, some 3)
- [ ] Update hex spawning to use actual heights from Level
- [ ] Test that taller hexes are visibly taller

**Test**: Hex grid shows varied heights creating interesting terrain.

### Task 8: Enhanced Camera Controls

**Goal**: Make camera controls feel polished.

- [ ] Add camera bounds to prevent moving too far from level
- [ ] Add smooth movement and zoom (acceleration/deceleration)
- [ ] Tune movement speeds for good feel
- [ ] Add zoom limits (min/max distance)
- [ ] **Update camera rotation system to use hex raycasting** (currently uses XZ plane intersection which is inaccurate for variable height hexes - needs Level data structure from Task 6)

**Test**: Camera controls feel smooth and stay appropriately bounded. Rotation centers correctly on the hex the camera is actually viewing.

### Task 9: Level Editor Integration

**Goal**: Get level editor showing the same thing.

- [ ] Copy rendering system to level-editor binary
- [ ] Test that level editor shows identical hex grid
- [ ] Both use same Level resource and systems

**Test**: `just run-level-editor` shows identical hex grid visualization.

### Task 10: WASM and Polish

**Goal**: Verify WASM works and clean up code.

- [ ] Test WASM compilation and browser functionality
- [ ] Move shared code to proper locations
- [ ] Add tests for core functionality
- [ ] Run quality checks and fix issues

**Test**: `just wasm` works and shows hex grid in browser with acceptable performance.

## Dependencies and Setup

### New Crate Dependencies

```toml
# shared/Cargo.toml additions
ndarray = { version = "0.16", features = ["serde"] }

# No additional dependencies needed for game or level-editor
# (hexx and bevy already included)
```

### Development Commands

```bash
# Test individual components
cargo test -p shared

# Run game with level rendering
just run-game

# Run level editor
just run-level-editor

# Test WASM build
just wasm

# Quality checks
just check
```

## Risk Mitigation

### Medium Risk: 3D Performance in WASM

- **Mitigation**: Start with simple mesh generation, optimize if needed
- **Fallback**: Reduce level size or simplify mesh complexity
- **Testing**: Validate performance early with `just wasm`

### Low Risk: Hexx 3D Integration

- **Mitigation**: Study hexx examples thoroughly before implementation
- **Fallback**: Custom cylinder mesh generation if hexx doesn't meet needs
- **Testing**: Test mesh generation with various heights and layouts

### Low Risk: Camera Control Feel

- **Mitigation**: Reference strategy game camera patterns
- **Fallback**: Adjust movement and zoom speeds based on testing
- **Testing**: Get user feedback on camera responsiveness

## Success Metrics

- [ ] Default 10x10 level loads and displays in both applications
- [ ] Hex columns render with correct gray surface and green edge colors
- [ ] Camera controls feel smooth and responsive
- [ ] Consistent 60fps performance in native builds
- [ ] WASM build works acceptably in browser environment
- [ ] Code passes all quality checks (`just check`)

## Post-Implementation

### Immediate Follow-ups

- Test with various level sizes (5x5, 15x15, 20x20)
- Gather user feedback on camera control feel
- Profile performance with larger levels

### Future Integration Points

- Hex selection and highlighting system
- Unit placement and movement visualization
- Level editor terrain modification tools
- Combat system integration

## References

- **Specification**: `docs/projects/basic-level-setup/specs/basic-level-setup.md`
- **Hexx 3D Example**: https://github.com/ManevilleF/hexx/blob/main/examples/3d_columns.rs
- **Obsidian Design Docs**: SystemTactics Technical Architecture Evidence notes
- **Example Project**: `/Users/eren/Projects/example_project/` for color scheme reference
