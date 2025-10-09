//! Level Geometry System
//!
//! Shared level creation functionality for spawning hex-based level geometry
//! in both the game and level editor applications.

use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use tracing::info;

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
}

/// Component to mark the level name display text
#[derive(Component)]
pub struct LevelNameDisplay;

/// System to spawn the level name UI text in the bottom-right corner
pub fn spawn_level_name_ui(mut commands: Commands, level: Res<Level>) {
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

/// System to update the level name display when the level changes
pub fn update_level_name_display(
    level: Res<Level>,
    mut text_query: Query<&mut Text, With<LevelNameDisplay>>,
) {
    if level.is_changed() {
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

/// System to spawn hex grid based on the Level resource
pub fn spawn_hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    level: Res<Level>,
) {
    info!(
        "Spawning hex grid for level '{level_name}' ({width}x{height})",
        level_name = level.name,
        width = level.width,
        height = level.height
    );

    // Pointy orientation hex layout for proper tactical RPG alignment
    let hex_layout = HexLayout::pointy().with_scale(Vec2::splat(1.0));

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
            Wireframe, // Add wireframe component for green edges
        ));
    }

    info!("Hex grid spawning completed");
}

/// Plugin for level geometry creation
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // Create a default 10x10 level to match the current behavior
        let default_level = Level::new("Default Level".to_string(), 10, 10);
        info!(
            "LevelPlugin: Created default level '{level_name}'",
            level_name = default_level.name
        );

        app.add_plugins(WireframePlugin::default())
            .insert_resource(WireframeConfig {
                // Only show wireframes on entities with Wireframe component
                global: false,
                // Set the wireframe color to tactical green
                default_color: HEX_EDGE_GREEN,
            })
            .insert_resource(default_level)
            .add_systems(Startup, (spawn_hex_grid, spawn_level_name_ui))
            .add_systems(Update, update_level_name_display);

        info!("LevelPlugin: Plugin setup completed");
    }
}
