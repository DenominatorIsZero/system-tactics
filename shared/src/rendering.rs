//! Rendering System
//!
//! Camera controls, lighting setup, and UI rendering for tactical RPG view
//! in both game and level editor applications.

use bevy::prelude::*;
use tracing::debug;

use crate::rendering::camera::{setup_camera, camera_rotation_animation_system, cache_level_diagonal_system, position_camera_for_level_system, update_camera_limits_system, CameraLimits, CameraRotationState};
use crate::rendering::ui::{
    spawn_fps_counter, spawn_level_name_ui, update_fps_display, update_level_name_display,
};

pub mod camera;
pub mod ui;


/// System to setup tactical lighting
pub fn setup_lighting(mut commands: Commands) {
    // Add directional light
    let light_pos = Vec3::new(4.0, 8.0, 4.0);
    let light_target = Vec3::ZERO;
    let light_illuminance = 10000.0;
    debug!(
        "Adding directional light at position {} with illuminance {}",
        light_pos, light_illuminance
    );
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: light_illuminance,
            ..default()
        },
        Transform::from_xyz(light_pos.x, light_pos.y, light_pos.z)
            .looking_at(light_target, Vec3::Y),
    ));

    // Add ambient lighting
    let ambient_brightness = 300.0;
    debug!(
        "Setting up ambient lighting with brightness {}",
        ambient_brightness
    );
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: ambient_brightness,
        affects_lightmapped_meshes: true,
    });
}


/// Plugin for rendering setup (lighting and UI)
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraRotationState>()
            .init_resource::<CameraLimits>()
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_systems(
                Startup,
                (
                    setup_camera,
                    setup_lighting,
                    spawn_fps_counter,
                    spawn_level_name_ui,
                ),
            )
            .add_systems(
                Update,
                (
                    camera_rotation_animation_system,
                    cache_level_diagonal_system,
                    update_camera_limits_system
                        .after(cache_level_diagonal_system)
                        .after(camera_rotation_animation_system),
                    update_fps_display,
                    update_level_name_display,
                    position_camera_for_level_system
                        .after(update_camera_limits_system),
                ),
            );
    }
}
