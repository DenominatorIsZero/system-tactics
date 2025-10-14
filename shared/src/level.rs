//! Level Geometry System
//!
//! Shared level creation functionality for spawning hex-based level geometry
//! in both the game and level editor applications.

use anyhow::{Context, Result};
use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{info, warn};

use crate::colors::*;

/// Represents a tactical level with hex grid layout and height data
#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct Level {
    /// Human-readable name for this level
    pub name: String,
    /// Width of the hex grid (number of columns)
    pub width: i32,
    /// Height of the hex grid (number of rows)
    pub height: i32,
    /// Height data for each hex position, stored as [row][col]
    pub heights: Array2<f32>,
}

impl Level {
    /// Create a new level with the specified dimensions and a height gradient
    /// that matches the current hardcoded behavior (low front-left to high back-right)
    pub fn new(name: String, width: i32, height: i32) -> Self {
        let mut heights = Array2::zeros((height as usize, width as usize));

        // Replicate the current gradient calculation
        for r in 0..height {
            for q in 0..width {
                let q_norm = q as f32 / (width - 1) as f32;
                let r_norm = r as f32 / (height - 1) as f32;
                let height_factor = (q_norm + r_norm) / 2.0;
                let hex_height = 1.0 + height_factor * 3.0;
                heights[(r as usize, q as usize)] = hex_height;
            }
        }

        Self {
            name,
            width,
            height,
            heights,
        }
    }

    /// Get the height at a specific hex coordinate
    pub fn get_height(&self, hex: Hex) -> f32 {
        if hex.x >= 0 && hex.x < self.width && hex.y >= 0 && hex.y < self.height {
            self.heights[(hex.y as usize, hex.x as usize)]
        } else {
            0.0 // Default height for out-of-bounds coordinates
        }
    }

    /// Generate all hex coordinates for this level's grid
    pub fn get_hex_grid(&self) -> Vec<Hex> {
        let mut grid = Vec::new();
        for q in 0..self.width {
            for r in 0..self.height {
                grid.push(Hex::new(q, r));
            }
        }
        grid
    }

    /// Returns the standard hex layout configuration used throughout the tactical RPG
    ///
    /// Uses pointy orientation with 1.0 scale for consistent hex positioning across
    /// all level operations (bounds calculation, rendering, camera positioning, etc.)
    pub fn hex_layout() -> HexLayout {
        HexLayout::pointy().with_scale(Vec2::splat(1.0))
    }

    /// Calculate the world-space bounding box of this level's hex grid
    pub fn get_world_bounds(&self) -> (Vec3, Vec3) {
        let hex_layout = Self::hex_layout();

        let mut min_bounds = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_bounds = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        // Calculate bounds by checking all hex positions
        for hex in self.get_hex_grid() {
            let world_pos = hex_layout.hex_to_world_pos(hex);
            let height = self.get_height(hex);

            // Convert to 3D world position (hex uses XZ plane, height is Y)
            let hex_world_pos = Vec3::new(world_pos.x, height, world_pos.y);

            min_bounds = min_bounds.min(hex_world_pos);
            max_bounds = max_bounds.max(hex_world_pos);
        }

        (min_bounds, max_bounds)
    }

    /// Get the center hex coordinates for this level (handles all dimension cases)
    pub fn get_center_hexes(&self) -> Vec<Hex> {
        match (self.width % 2, self.height % 2) {
            (1, 1) => {
                // Odd × Odd: single center hex
                vec![Hex::new(self.width / 2, self.height / 2)]
            }
            (0, 0) => {
                // Even × Even: 4 center hexes
                let w_half = self.width / 2;
                let h_half = self.height / 2;
                vec![
                    Hex::new(w_half - 1, h_half - 1),
                    Hex::new(w_half, h_half - 1),
                    Hex::new(w_half - 1, h_half),
                    Hex::new(w_half, h_half),
                ]
            }
            (0, 1) => {
                // Even × Odd: 2 center hexes
                let w_half = self.width / 2;
                let h_center = self.height / 2;
                vec![Hex::new(w_half - 1, h_center), Hex::new(w_half, h_center)]
            }
            (1, 0) => {
                // Odd × Even: 2 center hexes
                let w_center = self.width / 2;
                let h_half = self.height / 2;
                vec![Hex::new(w_center, h_half - 1), Hex::new(w_center, h_half)]
            }
            _ => {
                // Fallback for invalid dimensions (should not happen with valid levels)
                vec![Hex::new(self.width / 2, self.height / 2)]
            }
        }
    }

    /// Get the center world position (average of center hexes)
    pub fn get_center_world_pos(&self) -> Vec3 {
        let center_hexes = self.get_center_hexes();
        let hex_layout = Self::hex_layout();

        let mut total_pos = Vec3::ZERO;
        let count = center_hexes.len() as f32;

        // Average the 3D positions of all center hexes
        for hex in center_hexes {
            let world_pos_2d = hex_layout.hex_to_world_pos(hex);
            let height = self.get_height(hex);
            total_pos += Vec3::new(world_pos_2d.x, height, world_pos_2d.y);
        }

        total_pos / count
    }

    /// Get the 3D diagonal extent of this level for isometric camera calculations
    pub fn get_level_diagonal_extent(&self) -> f32 {
        let (min_bounds, max_bounds) = self.get_world_bounds();
        let width = max_bounds.x - min_bounds.x; // X axis
        let depth = max_bounds.z - min_bounds.z; // Z axis for depth
        let height = max_bounds.y - min_bounds.y; // Y axis for vertical extent

        // For isometric view, we need the true 3D diagonal including height
        (width * width + depth * depth + height * height).sqrt()
    }

    /// Save this level to a TOML file in the assets/levels/ directory
    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        self.save_to_directory("assets/levels", filename)
    }

    /// Save this level to a TOML file in the specified directory
    pub fn save_to_directory(&self, directory: &str, filename: &str) -> Result<()> {
        // Create the directory if it doesn't exist
        std::fs::create_dir_all(directory)
            .with_context(|| format!("Failed to create directory: {directory}"))?;

        let file_path = format!("{directory}/{filename}");
        let toml_content =
            toml::to_string(self).with_context(|| "Failed to serialize level to TOML")?;

        fs::write(&file_path, toml_content)
            .with_context(|| format!("Failed to write level to file: {file_path}"))?;

        info!(
            "Saved level '{level_name}' to {file_path}",
            level_name = self.name
        );
        Ok(())
    }
}

/// Resource containing all available levels and tracking the current level
#[derive(Resource, Debug)]
pub struct LevelsResource {
    /// All available levels loaded from TOML files
    pub levels: Vec<Level>,
    /// Index of the currently active level
    pub current_level_index: usize,
}

impl LevelsResource {
    /// Create a new LevelsResource with a single default level
    pub fn with_default() -> Self {
        let default_level = Level::new("Default Level".to_string(), 10, 10);
        info!("LevelsResource: Created with fallback default level");
        Self {
            levels: vec![default_level],
            current_level_index: 0,
        }
    }

    /// Get the currently active level
    pub fn current_level(&self) -> &Level {
        &self.levels[self.current_level_index]
    }

    /// Get the total number of available levels
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }
}

/// Load all level files from the assets/levels/ directory
pub fn load_levels_from_assets() -> Result<LevelsResource> {
    load_levels_from_directory("assets/levels")
}

/// Load all level files from a specific directory
pub fn load_levels_from_directory(levels_dir: &str) -> Result<LevelsResource> {
    info!("Loading level files from directory: {levels_dir}");

    // Check if the levels directory exists
    if fs::metadata(levels_dir).is_err() {
        warn!("Levels directory '{levels_dir}' not found, using default level");
        return Ok(LevelsResource::with_default());
    }

    // Read all TOML files from the levels directory
    let entries = fs::read_dir(levels_dir)
        .with_context(|| format!("Failed to read levels directory: {levels_dir}"))?;

    let mut levels = Vec::new();

    for entry in entries {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        // Only process .toml files
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            info!("Loading level file: {file_name}");

            match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<Level>(&content) {
                    Ok(level) => {
                        info!(
                            "Successfully loaded level: '{level_name}' ({width}x{height})",
                            level_name = level.name,
                            width = level.width,
                            height = level.height
                        );
                        levels.push(level);
                    }
                    Err(err) => {
                        warn!("Failed to parse TOML in {file_name}: {err}");
                    }
                },
                Err(err) => {
                    warn!("Failed to read file {file_name}: {err}");
                }
            }
        }
    }

    // If no levels were loaded successfully, use default
    if levels.is_empty() {
        warn!("No valid level files found, using default level");
        return Ok(LevelsResource::with_default());
    }

    // Sort levels by name for consistent ordering
    levels.sort_by(|a, b| a.name.cmp(&b.name));

    info!("Successfully loaded {count} levels", count = levels.len());
    Ok(LevelsResource {
        levels,
        current_level_index: 0,
    })
}

/// Component to mark the level name display text
#[derive(Component)]
pub struct LevelNameDisplay;

/// Component to mark hex grid entities for easy despawning during level changes
#[derive(Component)]
pub struct HexGridEntity;

/// System to spawn the level name UI text in the bottom-right corner
pub fn spawn_level_name_ui(mut commands: Commands, levels_resource: Res<LevelsResource>) {
    let level = levels_resource.current_level();
    info!(
        "Spawning level name UI for level: '{level_name}'",
        level_name = level.name
    );

    // Create UI text positioned in bottom-right corner of screen
    let entity = commands
        .spawn((
            Text::new(&level.name),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                ..default()
            },
            LevelNameDisplay,
        ))
        .id();

    info!("Level name UI entity spawned: {entity:?} at bottom-right corner");
}

/// System to update the level name display when the levels resource changes
pub fn update_level_name_display(
    levels_resource: Res<LevelsResource>,
    mut text_query: Query<&mut Text, With<LevelNameDisplay>>,
) {
    if levels_resource.is_changed() {
        let level = levels_resource.current_level();
        info!(
            "Level changed, updating level name display to: '{level_name}'",
            level_name = level.name
        );
        for mut text in text_query.iter_mut() {
            **text = level.name.clone();
        }
    }
}

/// Create a hexagonal column mesh using hexx
pub fn create_hex_column_mesh(layout: &HexLayout, height: f32) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(layout, height)
        .without_bottom_face()
        .center_aligned()
        .build();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

/// System to spawn hex grid based on the LevelsResource (used for initial spawn)
pub fn spawn_hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    levels_resource: Res<LevelsResource>,
) {
    spawn_hex_grid_internal(&mut commands, &mut meshes, &mut materials, &levels_resource);
}

/// System to handle left/right arrow key input for level cycling
pub fn level_cycling_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut levels_resource: ResMut<LevelsResource>,
) {
    let level_count = levels_resource.level_count();

    // Only process input if we have multiple levels
    if level_count <= 1 {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        // Cycle to previous level (with wraparound)
        let new_index = if levels_resource.current_level_index == 0 {
            level_count - 1
        } else {
            levels_resource.current_level_index - 1
        };

        let old_level_name = levels_resource.current_level().name.clone();
        levels_resource.current_level_index = new_index;
        let new_level_name = &levels_resource.current_level().name;

        info!(
            "Level cycling: Previous (←) - switched from '{old_name}' to '{new_name}' (index {new_index})",
            old_name = old_level_name,
            new_name = new_level_name
        );
    }

    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        // Cycle to next level (with wraparound)
        let new_index = (levels_resource.current_level_index + 1) % level_count;

        let old_level_name = levels_resource.current_level().name.clone();
        levels_resource.current_level_index = new_index;
        let new_level_name = &levels_resource.current_level().name;

        info!(
            "Level cycling: Next (→) - switched from '{old_name}' to '{new_name}' (index {new_index})",
            old_name = old_level_name,
            new_name = new_level_name
        );
    }
}

/// System to handle level switching by despawning old hex grid and spawning new one
pub fn level_switching_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    levels_resource: Res<LevelsResource>,
    hex_grid_query: Query<Entity, With<HexGridEntity>>,
) {
    // Only trigger when LevelsResource has actually changed
    if !levels_resource.is_changed() {
        return;
    }

    let level = levels_resource.current_level();
    info!(
        "Level switched: Despawning old hex grid and spawning new grid for '{level_name}'",
        level_name = level.name
    );

    // Despawn all existing hex grid entities
    let despawn_count = hex_grid_query.iter().count();
    for entity in hex_grid_query.iter() {
        commands.entity(entity).despawn();
    }

    if despawn_count > 0 {
        info!(
            "Despawned {count} hex grid entities from previous level",
            count = despawn_count
        );
    }

    // Spawn new hex grid for the current level using existing logic
    spawn_hex_grid_internal(&mut commands, &mut meshes, &mut materials, &levels_resource);
}

/// Internal function to spawn hex grid (extracted from spawn_hex_grid for reuse)
fn spawn_hex_grid_internal(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    levels_resource: &Res<LevelsResource>,
) {
    let level = levels_resource.current_level();
    info!(
        "Spawning hex grid for level '{level_name}' ({width}x{height})",
        level_name = level.name,
        width = level.width,
        height = level.height
    );

    // Use centralized hex layout configuration for consistency
    let hex_layout = Level::hex_layout();

    // Create tactical gray material for hex surfaces
    let hex_material = materials.add(StandardMaterial {
        base_color: HEX_SURFACE_GRAY,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        reflectance: 0.2,
        ..default()
    });

    // Generate hex grid from Level data
    let hex_grid = level.get_hex_grid();
    info!(
        "Generated {count} hex coordinates for the grid",
        count = hex_grid.len()
    );

    for hex in hex_grid {
        let height = level.get_height(hex);
        let hex_mesh = create_hex_column_mesh(&hex_layout, height);
        let world_pos = hex_layout.hex_to_world_pos(hex);

        // Spawn hex column with both solid surface and wireframe edges
        commands.spawn((
            Mesh3d(meshes.add(hex_mesh)),
            MeshMaterial3d(hex_material.clone()),
            Transform::from_xyz(world_pos.x, height / 2.0, world_pos.y),
            Wireframe,     // Add wireframe component for green edges
            HexGridEntity, // Mark as hex grid entity for level cycling
        ));
    }

    info!("Hex grid spawning completed");
}

/// System to automatically position and zoom camera for optimal level viewing when level changes
pub fn position_camera_for_level_system(
    levels_resource: Res<LevelsResource>,
    mut camera_query: Query<
        (&mut Transform, &mut Projection),
        With<crate::rendering::TacticalCamera>,
    >,
    windows: Query<&Window>,
) {
    // Only trigger when LevelsResource has actually changed
    if !levels_resource.is_changed() {
        return;
    }

    if let Ok((mut transform, mut projection)) = camera_query.single_mut() {
        let level = levels_resource.current_level();

        // Get center world position (handles all dimension cases)
        let center_pos = level.get_center_world_pos();

        // Calculate camera height as center position height + 20 units
        let camera_height = center_pos.y + 20.0;

        // Use camera's actual forward vector for inverse raycast
        let camera_forward = transform.forward();

        // Inverse raycast: center_pos = camera_pos + t * camera_forward
        // Solve for t: we want ray to hit the center position
        let height_diff = camera_height - center_pos.y;
        let t = height_diff / (-camera_forward.y); // Negative because forward points down

        // Calculate camera XZ position using inverse raycast
        let camera_x = center_pos.x - t * camera_forward.x;
        let camera_z = center_pos.z - t * camera_forward.z;
        let camera_pos = Vec3::new(camera_x, camera_height, camera_z);

        // Viewport-aware scale calculation using level diagonal extent
        let window = match windows.iter().next() {
            Some(window) => window,
            None => {
                warn!("No window available for camera positioning");
                return; // No window available, skip camera positioning
            }
        };
        let viewport_width = window.width();
        let viewport_height = window.height();

        // Determine viewport dimension based on camera Y rotation
        // Extract Y rotation from camera transform
        let (yaw, _, _) = transform.rotation.to_euler(bevy::math::EulerRot::YXZ);
        let yaw_degrees = yaw.to_degrees();

        // Normalize angle to 0-360 range for easier comparison
        let normalized_yaw = if yaw_degrees < 0.0 {
            yaw_degrees + 360.0
        } else {
            yaw_degrees
        };

        // Check if camera is rotated to portrait orientation (±90° from default)
        // Default starts at -45° (315°), so portrait orientations are around 45° and 225°
        let is_portrait = (normalized_yaw >= 35.0 && normalized_yaw <= 55.0) ||
                         (normalized_yaw >= 215.0 && normalized_yaw <= 235.0);

        let viewport_size = if is_portrait {
            window.height()
        } else {
            window.width()
        };

        // Get diagonal extent of level in world units
        let level_diagonal = level.get_level_diagonal_extent();

        let padding = 3.0;
        let padded_diagonal = level_diagonal + padding;

        // Calculate scale: padded diagonal should fit in viewport
        let optimal_scale = padded_diagonal / viewport_size;

        // Apply the new position and zoom
        transform.translation = camera_pos;
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = optimal_scale;
        }

        info!(
            "Camera positioned for level '{level_name}': position {camera_pos:?}, scale {scale:.4} (viewport: {viewport_width}x{viewport_height}, using {viewport_dim} {viewport_size}, yaw: {yaw:.1}°, diagonal: {diagonal:.2})",
            level_name = level.name,
            scale = optimal_scale,
            viewport_dim = if is_portrait { "height" } else { "width" },
            yaw = normalized_yaw,
            diagonal = level_diagonal
        );
    }
}


/// Plugin for level geometry creation
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Load levels from assets or fallback to default
        let levels_resource = match load_levels_from_assets() {
            Ok(levels) => {
                info!(
                    "LevelPlugin: Successfully loaded {count} levels from assets",
                    count = levels.level_count()
                );
                levels
            }
            Err(err) => {
                warn!("LevelPlugin: Failed to load levels from assets: {err}");
                warn!("LevelPlugin: Using fallback default level");
                LevelsResource::with_default()
            }
        };

        app.add_plugins(WireframePlugin::default())
            .insert_resource(WireframeConfig {
                // Only show wireframes on entities with Wireframe component
                global: false,
                // Set the wireframe color to tactical green
                default_color: HEX_EDGE_GREEN,
            })
            .insert_resource(levels_resource)
            .add_systems(Startup, (spawn_hex_grid, spawn_level_name_ui))
            .add_systems(
                Update,
                (
                    level_cycling_input_system,
                    level_switching_system,
                    update_level_name_display,
                    position_camera_for_level_system.after(crate::rendering::camera_rotation_animation_system),
                ),
            );

        info!("LevelPlugin: Plugin setup completed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_level_save_and_load_roundtrip() {
        // Create a temporary directory for this test
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let temp_path = temp_dir.path().to_str().expect("Failed to get temp path");

        // Create test levels
        let default_level = Level::new("Default Level".to_string(), 10, 10);
        let small_level = Level::new("Small Test Level".to_string(), 5, 5);
        let large_level = Level::new("Large Test Level".to_string(), 15, 15);

        // Save all levels to TOML files in the temporary directory
        default_level
            .save_to_directory(temp_path, "default.toml")
            .expect("Failed to save default level");
        small_level
            .save_to_directory(temp_path, "test_small.toml")
            .expect("Failed to save small level");
        large_level
            .save_to_directory(temp_path, "test_large.toml")
            .expect("Failed to save large level");

        // Load levels back from the temporary directory
        let levels_resource = load_levels_from_directory(temp_path)
            .expect("Failed to load levels from temp directory");

        // Verify we loaded 3 levels
        assert_eq!(
            levels_resource.level_count(),
            3,
            "Should have loaded 3 levels"
        );

        // Verify each level was loaded correctly by checking names
        let level_names: Vec<&str> = levels_resource
            .levels
            .iter()
            .map(|l| l.name.as_str())
            .collect();
        assert!(
            level_names.contains(&"Default Level"),
            "Should contain Default Level"
        );
        assert!(
            level_names.contains(&"Small Test Level"),
            "Should contain Small Test Level"
        );
        assert!(
            level_names.contains(&"Large Test Level"),
            "Should contain Large Test Level"
        );

        // Test one specific level for correctness (default level)
        let loaded_default = levels_resource
            .levels
            .iter()
            .find(|l| l.name == "Default Level")
            .expect("Default level should be found");

        assert_eq!(loaded_default.width, 10, "Default level width should be 10");
        assert_eq!(
            loaded_default.height, 10,
            "Default level height should be 10"
        );
        assert_eq!(
            loaded_default.heights.shape(),
            &[10, 10],
            "Default level heights should be 10x10"
        );

        // Test that height data is preserved (check a few specific values)
        assert!(
            (loaded_default.heights[(0, 0)] - 1.0).abs() < 0.001,
            "First height should be ~1.0"
        );
        assert!(
            (loaded_default.heights[(9, 9)] - 4.0).abs() < 0.001,
            "Last height should be ~4.0"
        );

        // The temporary directory will be automatically cleaned up when temp_dir goes out of scope
    }

    #[test]
    fn test_levels_resource_current_level() {
        let level1 = Level::new("Level 1".to_string(), 5, 5);
        let level2 = Level::new("Level 2".to_string(), 7, 7);

        let levels_resource = LevelsResource {
            levels: vec![level1, level2],
            current_level_index: 0,
        };

        assert_eq!(levels_resource.current_level().name, "Level 1");
        assert_eq!(levels_resource.level_count(), 2);
    }

    #[test]
    fn test_fallback_to_default_when_no_files() {
        // Try to load from a nonexistent directory
        let nonexistent_dir = "/tmp/nonexistent_levels_dir_12345";

        let levels_resource =
            load_levels_from_directory(nonexistent_dir).expect("Should fallback to default");

        assert_eq!(levels_resource.level_count(), 1);
        assert_eq!(levels_resource.current_level().name, "Default Level");
    }
}
