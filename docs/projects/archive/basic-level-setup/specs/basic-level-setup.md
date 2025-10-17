# Basic Level Setup - Specification

## Overview

Implement a foundational level system for SystemTactics that displays a hex grid as 3D cylindrical hexes with basic camera controls. This provides the visual foundation for tactical gameplay and level editing.

## Requirements

### Functional Requirements

1. **Level Data Structure**
   - Level struct with required name, rows, columns, and 2D hex height data
   - Simple initialization system for creating default levels with uniform heights
   - Shared between game and level editor applications

2. **Level Rendering**
   - Each hex rendered as a 3D column with individual height
   - Hex surfaces using GRAY_SECONDARY color with GREEN_PRIMARY edges for visibility
   - Proper hex grid positioning using hexx coordinate system
   - Height variation per hex for terrain representation
   - Consistent visual appearance across game and level editor
   - Color constants matching example project and website theme

3. **Camera Control System**
   - WASD and arrow key movement for camera panning
   - Mouse wheel for zoom in/out
   - Fixed isometric viewing angle for tactical clarity
   - Movement restricted to reasonable bounds around the level

### Non-Functional Requirements

- **Performance**: Smooth 60fps rendering for levels up to 20x20 hexes
- **Usability**: Intuitive camera controls similar to strategy game conventions
- **Compatibility**: Works identically in both game and level editor
- **Maintainability**: Clean separation between level data, rendering, and camera systems

## Game Design Integration

### Obsidian Documentation References

- **Primary Design Doc**: References hex grid tactical combat from Evidence notes
- **Related Systems**: Foundation for future combat positioning and movement systems
- **Player Experience**: Provides the spatial environment where tactical decisions occur

### Tactical RPG Considerations

- **Hex Grid Integration**: Uses shared hex coordinate system for consistent positioning
- **Combat Impact**: Establishes visual foundation for unit placement and movement
- **Level Editor Integration**: Enables map creation workflow for tactical scenarios
- **Visual Clarity**: Edge highlighting ensures clear hex boundaries for tactical precision
- **Fixed Camera Benefits**: Consistent sprite orientation and UI positioning for tactical gameplay

## Technical Specifications

### Architecture Impact

- **Shared Library Changes**: Add level data structures and hex positioning utilities
- **Game Binary Changes**: Add level rendering and camera systems to main game
- **Level Editor Changes**: Same rendering and camera systems for consistency
- **WASM Considerations**: 3D rendering performance optimized for web deployment

### Data Structures

```rust
// Level data structure using ndarray for efficient 2D storage
use ndarray::Array2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub name: String,
    pub rows: u32,
    pub columns: u32,
    pub hex_heights: Array2<u32>, // 2D array of discrete height levels per hex
}

impl Level {
    pub fn new(name: String, rows: u32, columns: u32, default_height: u32) -> Self {
        let hex_heights = Array2::from_elem((rows as usize, columns as usize), default_height);
        Self {
            name,
            rows,
            columns,
            hex_heights,
        }
    }

    pub fn get_height(&self, row: usize, col: usize) -> u32 {
        self.hex_heights[[row, col]]
    }

    pub fn set_height(&mut self, row: usize, col: usize, height: u32) {
        self.hex_heights[[row, col]] = height;
    }

    pub fn height_as_world_units(&self, row: usize, col: usize) -> f32 {
        self.get_height(row, col) as f32
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new("default_level".to_string(), 10, 10, 1)
    }
}

// Hex positioning utilities
pub fn hex_to_world_position(hex_pos: HexPosition, height_level: u32) -> Vec3;
pub fn generate_hex_positions(rows: u32, columns: u32) -> Vec<HexPosition>;
pub fn height_level_to_world_units(height_level: u32) -> f32;
```

### Rendering System Design

```rust
// Bevy systems for level rendering using hexx mesh generation
// No HexTile component needed - generate meshes directly from Level data

// Hexx integration for 3D column generation
use hexx::*;
use bevy::render::mesh::Indices;

// Color constants matching example project and website
pub const HEX_SURFACE_COLOR: Color = Color::srgb(0.294, 0.333, 0.388); // GRAY_SECONDARY from example project
pub const HEX_EDGE_COLOR: Color = Color::srgb(0.133, 0.698, 0.298);    // GREEN_PRIMARY from example project
pub const BACKGROUND_COLOR: Color = Color::srgb(0.216, 0.255, 0.318);  // Website background

// Generate hex column mesh using hexx
pub fn create_hex_column_mesh(layout: &HexLayout, height: f32) -> Mesh;
pub fn setup_hex_materials(commands: &mut Commands, materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial>;

// Level rendering system
pub fn spawn_level_meshes(
    mut commands: Commands,
    level: Res<Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Generate 3D hex column meshes directly from level data
    for row in 0..level.rows {
        for col in 0..level.columns {
            let height = level.get_height(row as usize, col as usize);
            let hex_pos = /* calculate from row/col using hexx */;
            let world_pos = hex_to_world_position(hex_pos, height);

            commands.spawn(PbrBundle {
                mesh: /* hexx-generated column mesh */,
                material: /* material using HEX_SURFACE_COLOR and HEX_EDGE_COLOR */,
                transform: Transform::from_translation(world_pos),
                ..default()
            });
        }
    }
}

// Camera controller component
#[derive(Component)]
pub struct CameraController {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub bounds: CameraBounds,
    pub fixed_angle: Vec3, // Fixed isometric viewing angle
}

#[derive(Debug, Clone)]
pub struct CameraBounds {
    pub min_distance: f32,
    pub max_distance: f32,
    pub center: Vec3,
    pub radius: f32,
}
```

## User Interface

### Bevy 3D Components

- **Hex Columns**: Generated using hexx's 3D column mesh system with PbrBundle materials
- **Mesh Generation**: Leverage hexx's built-in hexagonal column mesh creation
- **Camera Setup**: 3D camera with fixed isometric angle positioned above level
- **Lighting**: Basic ambient and directional lighting for clear hex visibility

### Visual Design

- **Hex Surface Color**: `GRAY_SECONDARY` (`Color::srgb(0.294, 0.333, 0.388)`) - matches website content areas
- **Hex Edge Color**: `GREEN_PRIMARY` (`Color::srgb(0.133, 0.698, 0.298)`) - matches example project theme
- **Background Color**: `BACKGROUND_COLOR` (`Color::srgb(0.216, 0.255, 0.318)`) - matches website background
- **Hex Dimensions**: Radius derived from hexx layout, height configurable per level
- **Camera Position**: Fixed isometric view angled to show hex tops and sides clearly
- **Color Consistency**: All colors stored as constants matching example project/website palette

### Camera Controls

1. **Movement (WASD/Arrow Keys)**:
   - Smooth panning across the level surface
   - Movement speed scales with camera distance
   - Bounded to keep level in view

2. **Zoom (Mouse Wheel)**:
   - Zoom in/out while maintaining focus on level
   - Minimum distance prevents clipping through hexes
   - Maximum distance keeps level visible
   - Fixed isometric angle preserved during zoom

## Success Criteria

### Acceptance Criteria

- [ ] Level struct properly stores required name, rows, columns, and 2D hex heights
- [ ] Default level (10x10 hexes) loads and displays in both game and level editor
- [ ] Each hex renders using GRAY_SECONDARY surface and GREEN_PRIMARY edges
- [ ] Hex positions correctly follow hex grid coordinate system
- [ ] WASD/arrow keys smoothly pan camera across level
- [ ] Mouse wheel zooms in/out with appropriate bounds
- [ ] Fixed isometric camera angle provides clear tactical view
- [ ] Camera movement stays within reasonable bounds around level
- [ ] Consistent 60fps performance with 10x10 hex level

### Testing Requirements

- **Unit Tests**: Level creation, hex positioning math, coordinate conversion
- **Integration Tests**: Level rendering system with Bevy ECS components
- **User Testing**: Camera controls feel natural and responsive
- **Performance Tests**: Frame rate stability with maximum expected level size

## Edge Cases and Error Handling

### Known Edge Cases

1. **Empty Level**: Handle levels with 0 rows or columns gracefully
2. **Large Levels**: Performance degradation with levels >20x20 hexes
3. **Camera Bounds**: Prevent camera from moving too far from level
4. **Zoom Limits**: Prevent camera from clipping through level or zooming too far out

### Error Scenarios

- **Level Creation Failure**: Default to minimal valid level (1x1)
- **Rendering Failure**: Fall back to wireframe or basic shapes
- **Camera Control Issues**: Reset to default position and controls

## Dependencies and Constraints

### Technical Dependencies

- **Rust Crates**: bevy (3D rendering), hexx (hex coordinate system and 3D mesh generation), ndarray (2D array operations)
- **Hexx 3D Support**: Uses hexx's built-in column mesh generation capabilities
- **Reference Implementation**: Based on hexx 3D columns example (https://github.com/ManevilleF/hexx/blob/main/examples/3d_columns.rs)
- **Bevy Features**: 3D rendering, PBR materials, input handling, camera controls
- **WASM Support**: All 3D features must work in browser deployment

### Design Constraints

- **Obsidian Integration**: Level concept aligns with tactical combat design documents
- **Existing Systems**: Must integrate with current hex grid coordinate system
- **Performance Limits**: 3D rendering performance acceptable for WASM deployment
- **Visual Consistency**: Matches color scheme from example project

### Shared Library Integration

- **Hex Grid System**: Builds on existing HexPosition and coordinate math
- **Cross-Application**: Same level data and rendering in game and level editor
- **Future Expansion**: Design allows for additional hex properties (terrain, elevation, etc.)

## Implementation Notes

### Hexx 3D Integration Approach

1. **Study Reference**: Analyze hexx 3D columns example for mesh generation patterns
2. **Adapt for SystemTactics**: Modify example approach for our level grid system
3. **Material Integration**: Apply gray/green color scheme to hexx-generated meshes
4. **Performance Testing**: Validate hexx approach works well for our grid sizes
5. **Fallback Plan**: If hexx 3D doesn't meet needs, implement custom cylinder generation

### Development Phases

1. **Phase 1**: Level data structure and hexx layout integration
2. **Phase 2**: 3D hex column rendering using hexx mesh generation
3. **Phase 3**: Camera controller with movement and zoom

### Risk Assessment

- **Medium Risk**: 3D rendering performance in WASM deployment
- **Low Risk**: Hex coordinate positioning (existing system foundation)
- **Low Risk**: Fixed camera controls (simpler than orbit controls)
- **Low Risk**: Sprite compatibility (single viewing angle eliminates complexity)

### Future Integration Points

- **Tactical Combat**: Hex selection, unit placement, movement visualization
- **Level Editor**: Hex editing, terrain modification, scenario placement
- **Visual Effects**: Highlighting, animations, particle effects

## References

- **Obsidian Design Docs**: Hex grid tactical combat specifications
- **Hexx 3D Example**: https://github.com/ManevilleF/hexx/blob/main/examples/3d_columns.rs
- **Technical References**: Bevy 3D rendering documentation, hexx coordinate system and mesh generation
- **Visual References**: Example project color scheme and UI patterns
- **Similar Implementations**: Civilization VI hex grid, XCOM tactical view