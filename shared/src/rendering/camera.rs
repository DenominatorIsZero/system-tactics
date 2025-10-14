//! Camera Positioning Systems
//!
//! Advanced camera positioning, viewport calculations, and level-aware
//! camera management for optimal tactical RPG viewing.

use bevy::prelude::*;
use tracing::{info, warn};

use super::TacticalCamera;
use crate::level::LevelsResource;

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
