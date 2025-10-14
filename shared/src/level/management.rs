//! Level Management Systems
//!
//! Level switching, grid lifecycle management, and level state transitions
//! for dynamic tactical RPG level loading.

use bevy::prelude::*;
use tracing::info;

use super::LevelsResource;
use super::mesh::{HexGridEntity, spawn_hex_grid_internal};

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
