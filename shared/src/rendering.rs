//! Rendering Setup Systems
//!
//! Shared camera and lighting setup for tactical RPG view in both game
//! and level editor applications.

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use tracing::{debug, info};

/// Component to mark the tactical camera for movement controls
#[derive(Component)]
pub struct TacticalCamera;

/// Component to mark the FPS counter display text
#[derive(Component)]
pub struct FpsDisplay;

/// Resource to track camera rotation state for smooth animations
#[derive(Resource, Default)]
pub struct CameraRotationState {
    pub rotation_mode: RotationMode,
    pub focus_point: Vec3, // Point to rotate around, calculated when rotation starts
}

/// Enum to represent the current rotation state of the camera
#[derive(Default)]
pub enum RotationMode {
    #[default]
    Stable,
    Clockwise(f32),        // f32 = remaining rotation in radians
    CounterClockwise(f32), // f32 = remaining rotation in radians
}

/// Calculate where the camera's forward ray intersects the XZ plane (Y=0)
///
/// LIMITATION: This uses ground plane (Y=0) intersection, which doesn't account for
/// variable hex heights. For tall hexes, the camera is actually looking at the hex
/// top surface, not the ground. This causes the rotation center to be offset from
/// the hex the player is actually viewing.
///
/// TODO (Task 8): Replace with proper hex raycasting when level data structure is
/// implemented. Should raycast against hex column bounds and return intersection
/// with the actual hex surface the camera is looking at.
fn calculate_camera_focus_point(transform: &Transform) -> Vec3 {
    let camera_pos = transform.translation;
    let forward_dir = transform.forward();

    // If camera is looking parallel to XZ plane, use fallback
    if forward_dir.y.abs() < 0.001 {
        // Camera looking horizontally, use a point directly in front
        return camera_pos + forward_dir * 10.0;
    }

    // Calculate intersection with XZ plane (Y = 0)
    // Ray equation: point = camera_pos + t * forward_dir
    // For Y = 0: 0 = camera_pos.y + t * forward_dir.y
    // So: t = -camera_pos.y / forward_dir.y
    let t = -camera_pos.y / forward_dir.y;

    // Calculate intersection point
    let intersection = camera_pos + forward_dir * t;

    // Return intersection point on XZ plane
    // NOTE: This will be inaccurate for variable height hexes - see TODO above
    Vec3::new(intersection.x, 0.0, intersection.z)
}

/// Orbit the camera around a dynamic point while preserving camera rotation
fn orbit_camera_around_point(transform: &mut Transform, pivot: Vec3, y_rotation: f32) {
    // Get current offset from pivot point
    let offset = transform.translation - pivot;

    // Apply Y rotation to the offset vector
    let rotation_quat = Quat::from_rotation_y(y_rotation);
    let rotated_offset = rotation_quat * offset;

    // Set new camera position
    transform.translation = pivot + rotated_offset;

    // Also rotate the camera's orientation by the same amount
    transform.rotation = rotation_quat * transform.rotation;
}

/// System to setup tactical camera
pub fn setup_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(4.5, 20.0, -4.5); // Above grid center

    let rotation = Quat::from_rotation_y(-45.0_f32.to_radians())
        * Quat::from_rotation_x(-45.0_f32.to_radians());

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
    rotation_state: Res<CameraRotationState>,
    mut camera_query: Query<&mut Transform, With<TacticalCamera>>,
) {
    // Block movement during camera rotation to maintain consistent focus point
    if !matches!(rotation_state.rotation_mode, RotationMode::Stable) {
        return;
    }

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
                ortho.scale = (ortho.scale - event.y * zoom_speed).clamp(0.005, 0.05);
            }
        }
    }
}

/// System for Q/E camera rotation input (starts smooth rotation)
pub fn camera_rotation_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut rotation_state: ResMut<CameraRotationState>,
    camera_query: Query<&Transform, With<TacticalCamera>>,
) {
    // Only accept input when camera is stable (not currently rotating)
    if matches!(rotation_state.rotation_mode, RotationMode::Stable) {
        if let Ok(transform) = camera_query.single() {
            // Q rotates counter-clockwise (90 degrees)
            if keyboard_input.just_pressed(KeyCode::KeyQ) {
                rotation_state.focus_point = calculate_camera_focus_point(transform);
                rotation_state.rotation_mode =
                    RotationMode::CounterClockwise(90.0_f32.to_radians());
            }
            // E rotates clockwise (90 degrees)
            if keyboard_input.just_pressed(KeyCode::KeyE) {
                rotation_state.focus_point = calculate_camera_focus_point(transform);
                rotation_state.rotation_mode = RotationMode::Clockwise(90.0_f32.to_radians());
            }
        }
    }
}

/// System for smooth camera rotation animation
pub fn camera_rotation_animation_system(
    time: Res<Time>,
    mut rotation_state: ResMut<CameraRotationState>,
    mut camera_query: Query<&mut Transform, With<TacticalCamera>>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        // Extract focus point before borrowing rotation_mode mutably
        let focus_point = rotation_state.focus_point;

        match &mut rotation_state.rotation_mode {
            RotationMode::Clockwise(remaining) => {
                let rotation_speed = 180.0_f32.to_radians(); // 180 degrees per second
                let delta_rotation = rotation_speed * time.delta_secs();
                let this_frame_rotation = delta_rotation.min(*remaining);

                // Use cached focus point calculated when rotation started
                orbit_camera_around_point(&mut transform, focus_point, -this_frame_rotation);

                *remaining -= this_frame_rotation;

                // If rotation is complete, snap to stable state
                if *remaining <= 0.0 {
                    rotation_state.rotation_mode = RotationMode::Stable;
                }
            }
            RotationMode::CounterClockwise(remaining) => {
                let rotation_speed = 180.0_f32.to_radians(); // 180 degrees per second
                let delta_rotation = rotation_speed * time.delta_secs();
                let this_frame_rotation = delta_rotation.min(*remaining);

                // Use cached focus point calculated when rotation started
                orbit_camera_around_point(&mut transform, focus_point, this_frame_rotation);

                *remaining -= this_frame_rotation;

                // If rotation is complete, snap to stable state
                if *remaining <= 0.0 {
                    rotation_state.rotation_mode = RotationMode::Stable;
                }
            }
            RotationMode::Stable => {
                // No rotation needed - no calculations performed
            }
        }
    }
}

/// System to spawn the FPS counter in the top-left corner
pub fn spawn_fps_counter(mut commands: Commands) {
    info!("Spawning FPS counter UI");

    let entity = commands
        .spawn((
            Text::new("FPS: --"),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(crate::colors::YELLOW_ACCENT),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                ..default()
            },
            FpsDisplay,
        ))
        .id();

    info!("FPS counter UI entity spawned: {entity:?} at top-left corner");
}

/// System to update the FPS counter display
pub fn update_fps_display(
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    mut text_query: Query<&mut Text, With<FpsDisplay>>,
) {
    for mut text in text_query.iter_mut() {
        if let Some(fps_diagnostic) =
            diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                **text = format!("FPS: {fps_smoothed:.1}");
            }
        }
    }
}

/// System to log current camera position and settings when 'C' key is pressed
pub fn debug_camera_logging_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Transform, &Projection), With<TacticalCamera>>,
    levels_resource: Res<crate::level::LevelsResource>,
) {
    // Only trigger on 'C' key press (not hold)
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        if let Ok((transform, projection)) = camera_query.single() {
            let level = levels_resource.current_level();

            // Get orthographic scale
            let scale = match projection {
                Projection::Orthographic(ortho) => ortho.scale,
                _ => 0.0,
            };

            // Convert rotation quaternion to readable angles (in degrees)
            // EulerRot::YXZ order: Y(yaw), X(pitch), Z(roll)
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let yaw_deg = yaw.to_degrees();
            let pitch_deg = pitch.to_degrees();
            let roll_deg = roll.to_degrees();

            // Log detailed camera debug information
            info!(
                "CAMERA_DEBUG: Level='{level_name}' ({width}x{height}) | Pos=({pos_x:.3}, {pos_y:.3}, {pos_z:.3}) | Scale={scale:.6} | Rotation=({yaw:.1}°, {pitch:.1}°, {roll:.1}°)",
                level_name = level.name,
                width = level.width,
                height = level.height,
                pos_x = transform.translation.x,
                pos_y = transform.translation.y,
                pos_z = transform.translation.z,
                pitch = pitch_deg,
                yaw = yaw_deg,
                roll = roll_deg
            );
        }
    }
}

/// Plugin for rendering setup (camera and lighting)
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraRotationState>()
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, (setup_camera, setup_lighting, spawn_fps_counter))
            .add_systems(
                Update,
                (
                    camera_movement_system,
                    camera_zoom_system,
                    camera_rotation_input_system,
                    camera_rotation_animation_system,
                    update_fps_display,
                    debug_camera_logging_system,
                ),
            );
    }
}
