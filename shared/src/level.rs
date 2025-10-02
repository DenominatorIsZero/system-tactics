//! Level Geometry System
//!
//! Shared level creation functionality for spawning hex-based level geometry
//! in both the game and level editor applications.

use bevy::prelude::*;
use bevy::render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages};
use hexx::{HexLayout, ColumnMeshBuilder};

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


/// System to spawn a single test hex column with UV checker texture
pub fn spawn_test_hex_column(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let hex_layout = HexLayout {
        scale: Vec2::splat(1.0),
        ..default()
    };

    let hex_mesh = create_hex_column_mesh(&hex_layout, 1.0);
    let uv_texture = asset_server.load("uv_checker.png");

    commands.spawn((
        Mesh3d(meshes.add(hex_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(uv_texture),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

/// Plugin for level geometry creation
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_hex_column);
    }
}