//! Input Handling Systems
//!
//! Input handling for camera controls, level cycling, and debug commands
//! for the tactical RPG.

use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::*;
use tracing::info;

use crate::level::LevelsResource;
use crate::rendering::camera::{calculate_camera_focus_point, CameraRotationState, RotationMode, TacticalCamera};

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

/// System to log current camera position and settings when 'C' key is pressed
pub fn debug_camera_logging_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Transform, &Projection), With<TacticalCamera>>,
    levels_resource: Res<LevelsResource>,
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

/// Resource to track mouse panning state
#[derive(Resource, Default)]
pub struct MousePanState {
    pub is_panning: bool,
}

/// System for mouse panning input (right mouse button + drag)
pub fn camera_mouse_pan_system(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut pan_state: ResMut<MousePanState>,
    rotation_state: Res<CameraRotationState>,
    mut camera_query: Query<&mut Transform, With<TacticalCamera>>,
) {
    // Block panning during camera rotation to maintain consistent behavior
    if !matches!(rotation_state.rotation_mode, RotationMode::Stable) {
        return;
    }

    // Handle mouse button events for starting/stopping panning
    for event in mouse_button_events.read() {
        if event.button == MouseButton::Right {
            match event.state {
                ButtonState::Pressed => {
                    pan_state.is_panning = true;
                }
                ButtonState::Released => {
                    pan_state.is_panning = false;
                }
            }
        }
    }

    // Handle mouse motion for actual panning
    if pan_state.is_panning {
        if let Ok(mut transform) = camera_query.single_mut() {
            for event in mouse_motion_events.read() {
                let pan_sensitivity = 0.01; // Adjust sensitivity for comfortable panning

                // Convert mouse delta to world-space movement relative to camera orientation
                // Similar to WASD movement but based on mouse motion
                let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize();
                let right = transform.right();

                // Apply movement in camera-relative coordinates
                // Invert Y to match typical mouse panning expectations
                let movement = right * (-event.delta.x * pan_sensitivity)
                             + forward * (event.delta.y * pan_sensitivity);

                transform.translation += movement;
            }
        }
    }
}

/// Plugin for input handling (camera controls, level cycling, debug commands)
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePanState>()
            .add_systems(
                Update,
                (
                    level_cycling_input_system,
                    camera_movement_system,
                    camera_zoom_system,
                    camera_rotation_input_system,
                    camera_mouse_pan_system,
                    debug_camera_logging_system,
                ),
            );
    }
}
