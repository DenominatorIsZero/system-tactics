//! Camera Systems
//!
//! Camera setup, positioning, rotation, and management for optimal tactical RPG viewing.

use bevy::prelude::*;
use tracing::{debug, info, warn};

use crate::level::LevelsResource;

/// Component to mark the tactical camera for movement controls
#[derive(Component)]
pub struct TacticalCamera;

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
pub fn calculate_camera_focus_point(transform: &Transform) -> Vec3 {
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
pub fn orbit_camera_around_point(transform: &mut Transform, pivot: Vec3, y_rotation: f32) {
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
        "Spawning isometric camera at position {camera_pos} with rotation {rotation:?}"
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

/// System to automatically position and zoom camera for optimal level viewing when level changes
pub fn position_camera_for_level_system(
    levels_resource: Res<LevelsResource>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<TacticalCamera>>,
    windows: Query<&Window>,
) {
    // Only trigger when LevelsResource has actually changed
    if !levels_resource.is_changed() {
        return;
    }

    if let Ok((mut transform, mut projection)) = camera_query.single_mut() {
        let level = levels_resource.current_level();

        // Get center world position (handles all dimension cases)
        let center_pos = level.get_center_world_pos();

        // Calculate camera height as center position height + 20 units
        let camera_height = center_pos.y + 20.0;

        // Use camera's actual forward vector for inverse raycast
        let camera_forward = transform.forward();

        // Inverse raycast: center_pos = camera_pos + t * camera_forward
        // Solve for t: we want ray to hit the center position
        let height_diff = camera_height - center_pos.y;
        let t = height_diff / (-camera_forward.y); // Negative because forward points down

        // Calculate camera XZ position using inverse raycast
        let camera_x = center_pos.x - t * camera_forward.x;
        let camera_z = center_pos.z - t * camera_forward.z;
        let camera_pos = Vec3::new(camera_x, camera_height, camera_z);

        // Viewport-aware scale calculation using level diagonal extent
        let window = match windows.iter().next() {
            Some(window) => window,
            None => {
                warn!("No window available for camera positioning");
                return; // No window available, skip camera positioning
            }
        };
        let viewport_width = window.width();
        let viewport_height = window.height();

        // Determine viewport dimension based on camera Y rotation
        // Extract Y rotation from camera transform
        let (yaw, _, _) = transform.rotation.to_euler(bevy::math::EulerRot::YXZ);
        let yaw_degrees = yaw.to_degrees();

        // Normalize angle to 0-360 range for easier comparison
        let normalized_yaw = if yaw_degrees < 0.0 {
            yaw_degrees + 360.0
        } else {
            yaw_degrees
        };

        // Check if camera is rotated to portrait orientation (±90° from default)
        // Default starts at -45° (315°), so portrait orientations are around 45° and 225°
        let is_portrait =
            (35.0..=55.0).contains(&normalized_yaw) || (215.0..=235.0).contains(&normalized_yaw);

        let viewport_size = if is_portrait {
            window.height()
        } else {
            window.width()
        };

        // Get diagonal extent of level in world units
        let level_diagonal = level.get_level_diagonal_extent();

        let padding = 3.0;
        let padded_diagonal = level_diagonal + padding;

        // Calculate scale: padded diagonal should fit in viewport
        let optimal_scale = padded_diagonal / viewport_size;

        // Apply the new position and zoom
        transform.translation = camera_pos;
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = optimal_scale;
        }

        info!(
            "Camera positioned for level '{level_name}': position {camera_pos:?}, scale {scale:.4} (viewport: {viewport_width}x{viewport_height}, using {viewport_dim} {viewport_size}, yaw: {yaw:.1}°, diagonal: {diagonal:.2})",
            level_name = level.name,
            scale = optimal_scale,
            viewport_dim = if is_portrait { "height" } else { "width" },
            yaw = normalized_yaw,
            diagonal = level_diagonal
        );
    }
}
