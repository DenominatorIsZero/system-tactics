//! Level Geometry System
//!
//! Shared level creation functionality for spawning hex-based level geometry
//! in both the game and level editor applications.

use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use hexx::{ColumnMeshBuilder, Hex, HexLayout};

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

/// Generate a rectangular grid of hex coordinates
fn generate_rectangular_grid(width: i32, height: i32) -> Vec<Hex> {
    let mut grid = Vec::new();

    // Simple 0-based grid (0 to width-1, 0 to height-1)
    for q in 0..width {
        for r in 0..height {
            grid.push(Hex::new(q, r));
        }
    }

    grid
}

/// Calculate height for a hex column with gradient from front-left (low) to back-right (high)
fn calculate_hex_height(hex: Hex, grid_width: i32, grid_height: i32) -> f32 {
    // Normalize coordinates to 0-1 range (0-based coordinates)
    let q_norm = hex.x as f32 / (grid_width - 1) as f32;
    let r_norm = hex.y as f32 / (grid_height - 1) as f32;

    // Create gradient: higher towards right (higher x) and back (higher y)
    let height_factor = (q_norm + r_norm) / 2.0;

    1.0 + height_factor * 9.0
}

/// System to spawn a 10x10 grid of hex columns with varying heights
pub fn spawn_hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Smaller hex scale to fit more hexes in view
    let hex_layout = HexLayout {
        scale: Vec2::splat(1.0),
        ..default()
    };

    // Load shared texture and material
    let uv_texture = asset_server.load("uv_checker.png");
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(uv_texture),
        ..default()
    });

    // Generate rectangular 10x10 hex grid
    let grid_width = 10;
    let grid_height = 10;
    let hex_grid = generate_rectangular_grid(grid_width, grid_height);

    for hex in hex_grid {
        let height = calculate_hex_height(hex, grid_width, grid_height);
        let hex_mesh = create_hex_column_mesh(&hex_layout, height);
        let world_pos = hex_layout.hex_to_world_pos(hex);

        commands.spawn((
            Mesh3d(meshes.add(hex_mesh)),
            MeshMaterial3d(material_handle.clone()),
            Transform::from_xyz(world_pos.x, height / 2.0, world_pos.y),
        ));
    }
}

/// Plugin for level geometry creation
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hex_grid);
    }
}
