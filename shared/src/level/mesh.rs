//! Mesh Generation Systems
//!
//! Hex column mesh generation, grid spawning systems, and mesh utilities
//! for rendering tactical RPG level geometry.

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
use hexx::{ColumnMeshBuilder, HexLayout};
use tracing::info;

use super::{Level, LevelsResource};
use crate::colors::HEX_SURFACE_GRAY;

/// Component to mark entities that are part of the hex grid
#[derive(Component)]
pub struct HexGridEntity;

/// Create a hex column mesh using the hexx library
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

/// Internal function to spawn hex grid for a given level
pub fn spawn_hex_grid_internal(
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

        // Spawn hex column - with wireframes on native, without on WASM
        #[cfg(not(target_arch = "wasm32"))]
        commands.spawn((
            Mesh3d(meshes.add(hex_mesh)),
            MeshMaterial3d(hex_material.clone()),
            Transform::from_xyz(world_pos.x, 0.0, world_pos.y),
            Wireframe,     // Add tactical green wireframe edges (native only)
            HexGridEntity, // Mark for easy identification/cleanup
        ));

        #[cfg(target_arch = "wasm32")]
        commands.spawn((
            Mesh3d(meshes.add(hex_mesh)),
            MeshMaterial3d(hex_material.clone()),
            Transform::from_xyz(world_pos.x, 0.0, world_pos.y),
            HexGridEntity, // Mark for easy identification/cleanup
        ));
    }

    info!("Hex grid spawning completed");
}
