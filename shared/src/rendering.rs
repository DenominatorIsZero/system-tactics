//! Rendering Setup Systems
//!
//! Shared camera and lighting setup for tactical RPG view in both game
//! and level editor applications.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use tracing::debug;

/// Component to mark the tactical camera for movement controls
#[derive(Component)]
pub struct TacticalCamera;

/// System to setup tactical camera
pub fn setup_camera(mut commands: Commands) {
    // True isometric camera setup with orthographic projection
    // Position camera above the grid center
    let camera_pos = Vec3::new(4.5, 20.0, -4.5); // Above grid center

    // Standard isometric rotation: 45° around Y-axis, then -30° around X-axis
    let rotation = Quat::from_rotation_y(-30.0_f32.to_radians())
        * Quat::from_rotation_x(-30.0_f32.to_radians());

    debug!(
        "Spawning isometric camera at position {} with rotation {:?}",
        camera_pos, rotation
    );

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(camera_pos).with_rotation(rotation),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.1, // Zoom out to see the full grid
            ..OrthographicProjection::default_3d()
        }),
        TacticalCamera,
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

/// System for WASD camera movement
pub fn camera_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<TacticalCamera>>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        let movement_speed = 10.0; // Units per second
        let delta_time = time.delta_secs();

        // Calculate movement vectors relative to camera orientation
        // For orthographic camera, we need to move parallel to ground plane
        let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize();
        let right = transform.right();

        // Movement aligned with camera view but parallel to ground
        if keyboard_input.pressed(KeyCode::KeyW) {
            // Move forward relative to camera (but only in XZ plane)
            transform.translation += forward * movement_speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            // Move backward relative to camera (but only in XZ plane)
            transform.translation -= forward * movement_speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            // Move left relative to camera
            transform.translation -= right * movement_speed * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            // Move right relative to camera
            transform.translation += right * movement_speed * delta_time;
        }
    }
}

/// System for mouse wheel and trackpad zoom
pub fn camera_zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Projection, With<TacticalCamera>>,
) {
    if let Ok(mut projection) = camera_query.single_mut() {
        for event in mouse_wheel_events.read() {
            let zoom_speed = 0.0001; // Adjust orthographic scale

            // Adjust orthographic scale for zoom (smaller scale = more zoomed in)
            if let Projection::Orthographic(ortho) = projection.as_mut() {
                ortho.scale = (ortho.scale - event.y * zoom_speed).max(0.005).min(0.05);
            }
        }
    }
}

/// Plugin for rendering setup (camera and lighting)
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_lighting))
            .add_systems(Update, (camera_movement_system, camera_zoom_system));
    }
}
