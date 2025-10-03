//! Rendering Setup Systems
//!
//! Shared camera and lighting setup for tactical RPG view in both game
//! and level editor applications.

use bevy::prelude::*;
use tracing::debug;

/// System to setup tactical camera
pub fn setup_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(0.0, 20.0, 0.0);
    let camera_target = Vec3::new(5.0, 0.0, 5.0);
    debug!(
        "Spawning 3D camera at position {} looking at {}",
        camera_pos, camera_target
    );
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z)
            .looking_at(camera_target, Vec3::Y),
    ));
}

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

/// Plugin for rendering setup (camera and lighting)
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_lighting));
    }
}
