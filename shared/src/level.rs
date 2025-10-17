//! Level System
//!
//! Core level data structures, management, and mesh generation for tactical RPG
//! hex-based level geometry in both the game and level editor applications.

use anyhow::{Context, Result};
#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use hexx::{Hex, HexLayout};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(target_arch = "wasm32")]
use toml;
use tracing::{info, warn};

#[cfg(not(target_arch = "wasm32"))]
use crate::colors::*;
use crate::level::management::level_switching_system;
use crate::level::mesh::spawn_hex_grid;

pub mod management;
pub mod mesh;

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
    ///
    /// Returns (min_bounds, max_bounds) where:
    /// - min_bounds: minimum X, Y, Z coordinates across all hexes
    /// - max_bounds: maximum X, Y, Z coordinates across all hexes
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
    ///
    /// Returns different numbers of hexes based on grid dimensions:
    /// - Odd×Odd: 1 center hex
    /// - Even×Even: 4 center hexes
    /// - Even×Odd or Odd×Even: 2 center hexes
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

    /// Create a new LevelsResource with provided levels
    pub fn new(levels: Vec<Level>) -> Self {
        Self {
            levels,
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

/// Create levels resource from embedded assets for WASM builds
#[cfg(target_arch = "wasm32")]
pub fn create_levels_from_embedded_assets() -> LevelsResource {
    let mut levels = Vec::new();

    // Manually include the embedded level data
    // These correspond to the files we embedded with embedded_asset!
    let level_data = [
        (
            "default.toml",
            include_str!("../../assets/levels/default.toml"),
        ),
        (
            "test_small.toml",
            include_str!("../../assets/levels/test_small.toml"),
        ),
        (
            "test_large.toml",
            include_str!("../../assets/levels/test_large.toml"),
        ),
    ];

    for (filename, content) in level_data {
        match toml::from_str::<Level>(content) {
            Ok(level) => {
                info!(
                    "Successfully loaded embedded level: '{}' ({}x{})",
                    level.name, level.width, level.height
                );
                levels.push(level);
            }
            Err(err) => {
                warn!("Failed to parse embedded level file '{filename}': {err}");
            }
        }
    }

    if levels.is_empty() {
        warn!("No embedded levels loaded, using default level");
        LevelsResource::with_default()
    } else {
        info!("Successfully loaded {} embedded levels", levels.len());
        LevelsResource::new(levels)
    }
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

/// Plugin for level geometry creation
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Load levels differently for native vs WASM builds
        let levels_resource = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Native: Load from file system
                match load_levels_from_assets() {
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
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                // WASM: Create levels resource with embedded asset data
                info!("LevelPlugin: Loading levels from embedded assets for WASM");
                create_levels_from_embedded_assets()
            }
        };

        // Add wireframe plugin only for native builds (WASM doesn't support POLYGON_MODE_LINE)
        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(WireframePlugin::default())
            .insert_resource(WireframeConfig {
                // Only show wireframes on entities with Wireframe component
                global: false,
                // Set the wireframe color to tactical green
                default_color: HEX_EDGE_GREEN,
            });

        app.insert_resource(levels_resource)
            .add_systems(Startup, spawn_hex_grid)
            .add_systems(Update, (level_cycling_input_system, level_switching_system));

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
