//! Camera Systems
//!
//! Camera setup, positioning, rotation, and management for optimal tactical RPG viewing.

use bevy::prelude::*;
use bevy::window::WindowResized;
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

/// Resource to track dynamic camera zoom limits and movement bounds
#[derive(Resource)]
pub struct CameraLimits {
    pub min_zoom_scale: f32, // Closest zoom (smallest scale value) - explicit constant
    pub max_zoom_scale: f32, // Furthest zoom (largest scale value) - calculated optimal scale
    pub level_diagonal: f32, // Cached diagonal extent from current level
    pub needs_recalculation: bool, // Flag to trigger limits recalculation
    pub optimal_camera_position: Vec3, // Optimal camera position for current level
    pub current_movement_radius: f32, // Current movement distance based on current zoom level
    pub rotation_processed: bool, // Flag to track if current rotation completion was processed
}

impl Default for CameraLimits {
    fn default() -> Self {
        Self {
            min_zoom_scale: 0.005,     // Same as current hardcoded minimum
            max_zoom_scale: 0.05,      // Temporary default, will be calculated
            level_diagonal: 10.0,      // Default level size
            needs_recalculation: true, // Initially needs calculation
            optimal_camera_position: Vec3::new(4.5, 20.0, -4.5), // Default camera position
            current_movement_radius: 5.0, // Default movement radius
            rotation_processed: false, // Initially no rotation to process
        }
    }
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

/// Calculate movement radius based on current zoom level
/// Returns level_diagonal/2 at closest zoom (min_zoom_scale), 0 at furthest zoom (max_zoom_scale)
pub fn calculate_movement_radius(camera_limits: &CameraLimits, current_scale: f32) -> f32 {
    // Clamp current scale to valid zoom range
    // min_zoom_scale = smallest scale value = closest zoom
    // max_zoom_scale = largest scale value = furthest zoom
    let clamped_scale =
        current_scale.clamp(camera_limits.min_zoom_scale, camera_limits.max_zoom_scale);

    // Linear interpolation: level_diagonal/2 at min_zoom_scale (closest), 0 at max_zoom_scale (furthest)
    let zoom_factor = (camera_limits.max_zoom_scale - clamped_scale)
        / (camera_limits.max_zoom_scale - camera_limits.min_zoom_scale);

    zoom_factor * (camera_limits.level_diagonal / 2.0)
}

/// Calculate optimal camera scale based on level diagonal, viewport size and camera orientation
pub fn calculate_optimal_scale(level_diagonal: f32, viewport_size: f32) -> f32 {
    let padding = 3.0;
    let padded_diagonal = level_diagonal + padding;
    padded_diagonal / viewport_size
}

/// Calculate optimal camera position for a level center using inverse raycast
pub fn calculate_optimal_camera_position(center_pos: Vec3, camera_forward: Dir3) -> Vec3 {
    // Calculate camera height as center position height + 20 units
    let camera_height = center_pos.y + 20.0;

    // Inverse raycast: center_pos = camera_pos + t * camera_forward
    // Solve for t: we want ray to hit the center position
    let height_diff = camera_height - center_pos.y;
    let t = height_diff / (-camera_forward.y); // Negative because forward points down

    // Calculate camera XZ position using inverse raycast
    let camera_x = center_pos.x - t * camera_forward.x;
    let camera_z = center_pos.z - t * camera_forward.z;
    Vec3::new(camera_x, camera_height, camera_z)
}

/// Determine viewport size based on camera orientation and window dimensions
pub fn get_viewport_size_for_orientation(transform: &Transform, window: &Window) -> f32 {
    // Determine viewport dimension based on camera Y rotation
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

    if is_portrait {
        window.height()
    } else {
        window.width()
    }
}

/// System to setup tactical camera
pub fn setup_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(4.5, 20.0, -4.5); // Above grid center

    let rotation = Quat::from_rotation_y(-45.0_f32.to_radians())
        * Quat::from_rotation_x(-45.0_f32.to_radians());

    debug!("Spawning isometric camera at position {camera_pos} with rotation {rotation:?}");

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
    mut camera_limits: ResMut<CameraLimits>,
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

                // If rotation is complete, snap to stable state and mark for processing
                if *remaining <= 0.0 {
                    rotation_state.rotation_mode = RotationMode::Stable;
                    camera_limits.rotation_processed = false; // Mark as needing processing
                }
            }
            RotationMode::CounterClockwise(remaining) => {
                let rotation_speed = 180.0_f32.to_radians(); // 180 degrees per second
                let delta_rotation = rotation_speed * time.delta_secs();
                let this_frame_rotation = delta_rotation.min(*remaining);

                // Use cached focus point calculated when rotation started
                orbit_camera_around_point(&mut transform, focus_point, this_frame_rotation);

                *remaining -= this_frame_rotation;

                // If rotation is complete, snap to stable state and mark for processing
                if *remaining <= 0.0 {
                    rotation_state.rotation_mode = RotationMode::Stable;
                    camera_limits.rotation_processed = false; // Mark as needing processing
                }
            }
            RotationMode::Stable => {
                // No rotation needed - no calculations performed
            }
        }
    }
}

/// System that handles all camera updates when level changes
/// Calculates diagonal, optimal position, updates limits, sets position + zoom, and movement radius
pub fn on_level_change_system(
    levels_resource: Res<LevelsResource>,
    mut camera_limits: ResMut<CameraLimits>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<TacticalCamera>>,
    windows: Query<&Window>,
) {
    // Only trigger when LevelsResource has actually changed
    if !levels_resource.is_changed() {
        return;
    }

    let Ok((mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };

    let Some(window) = windows.iter().next() else {
        warn!("No window available for level change camera update");
        return;
    };

    let level = levels_resource.current_level();

    // 1. Calculate and cache level diagonal
    let level_diagonal = level.get_level_diagonal_extent();
    camera_limits.level_diagonal = level_diagonal;

    // 2. Calculate and cache optimal camera position
    let center_pos = level.get_center_world_pos();
    let camera_forward = transform.forward();
    let optimal_position = calculate_optimal_camera_position(center_pos, camera_forward);
    camera_limits.optimal_camera_position = optimal_position;

    // 3. Update camera limits (max zoom scale) based on new level and current orientation
    let viewport_size = get_viewport_size_for_orientation(&transform, window);
    let optimal_scale = calculate_optimal_scale(level_diagonal, viewport_size);
    camera_limits.max_zoom_scale = optimal_scale;

    // 4. Set optimal position and zoom
    transform.translation = optimal_position;
    if let Projection::Orthographic(ortho) = projection.as_mut() {
        ortho.scale = optimal_scale;
    }

    // 5. Update movement radius based on new zoom level
    if let Projection::Orthographic(ortho) = projection.as_ref() {
        camera_limits.current_movement_radius =
            calculate_movement_radius(&camera_limits, ortho.scale);
    }

    camera_limits.needs_recalculation = false;

    info!(
        "Level change: Updated camera for '{level_name}' - position: {position:?}, scale: {scale:.4}, diagonal: {diagonal:.2}, movement_radius: {radius:.2}",
        level_name = level.name,
        position = optimal_position,
        scale = optimal_scale,
        diagonal = level_diagonal,
        radius = camera_limits.current_movement_radius
    );
}

/// System that updates movement radius when camera zoom changes
pub fn on_zoom_change_system(
    mut camera_limits: ResMut<CameraLimits>,
    camera_query: Query<&Projection, (With<TacticalCamera>, Changed<Projection>)>,
) {
    // Only trigger when camera projection has changed (zoom)
    if let Ok(Projection::Orthographic(ortho)) = camera_query.single() {
        let new_movement_radius = calculate_movement_radius(&camera_limits, ortho.scale);

        // Only update if the value has actually changed to avoid unnecessary work
        if (new_movement_radius - camera_limits.current_movement_radius).abs() > 0.001 {
            camera_limits.current_movement_radius = new_movement_radius;

            debug!(
                "Zoom change: Updated movement radius to {radius:.3} (scale={scale:.4})",
                radius = new_movement_radius,
                scale = ortho.scale
            );
        }
    }
}

/// System that updates zoom limits and movement radius when rotation completes
/// Only runs when CameraRotationState has changed AND rotation_mode is Stable AND not yet processed
pub fn on_rotation_complete_system(
    rotation_state: Res<CameraRotationState>,
    levels_resource: Res<LevelsResource>,
    mut camera_limits: ResMut<CameraLimits>,
    camera_query: Query<(&Transform, &Projection), With<TacticalCamera>>,
    windows: Query<&Window>,
) {
    // Only update when rotation has completed (now stable) and not yet processed
    if !matches!(rotation_state.rotation_mode, RotationMode::Stable) {
        return;
    }

    // Don't process if already processed
    if camera_limits.rotation_processed {
        return;
    }

    let Ok((transform, projection)) = camera_query.single() else {
        return;
    };

    let Some(window) = windows.iter().next() else {
        warn!("No window available for rotation complete camera update");
        return;
    };

    // Recalculate optimal camera position because camera forward vector changed
    let level = levels_resource.current_level();
    let center_pos = level.get_center_world_pos();
    let updated_position = calculate_optimal_camera_position(center_pos, transform.forward());
    camera_limits.optimal_camera_position = updated_position;

    // Update zoom limits because viewport orientation changed
    let viewport_size = get_viewport_size_for_orientation(transform, window);
    let optimal_scale = calculate_optimal_scale(camera_limits.level_diagonal, viewport_size);
    camera_limits.max_zoom_scale = optimal_scale;

    // Update movement radius because limits changed
    if let Projection::Orthographic(ortho) = projection {
        camera_limits.current_movement_radius =
            calculate_movement_radius(&camera_limits, ortho.scale);
    }

    camera_limits.rotation_processed = true; // Mark as processed

    info!(
        "Rotation complete: Updated camera - position: {position:?}, max_scale: {scale:.4}, movement_radius: {radius:.2}",
        position = updated_position,
        scale = optimal_scale,
        radius = camera_limits.current_movement_radius
    );
}

/// System to handle window resize events by updating camera limits
pub fn on_window_resize_system(
    mut window_resize_events: EventReader<WindowResized>,
    mut camera_limits: ResMut<CameraLimits>,
    camera_query: Query<(&Transform, &Projection), With<TacticalCamera>>,
    windows: Query<&Window>,
) {
    // Only trigger when window was actually resized
    if window_resize_events.read().count() == 0 {
        return;
    }

    let Ok((transform, projection)) = camera_query.single() else {
        return;
    };

    let Some(window) = windows.iter().next() else {
        warn!("No window available for resize camera update");
        return;
    };

    // Update camera limits based on new window size
    let viewport_size = get_viewport_size_for_orientation(transform, window);
    let optimal_scale = calculate_optimal_scale(camera_limits.level_diagonal, viewport_size);
    camera_limits.max_zoom_scale = optimal_scale;

    // Update movement radius because limits changed
    if let Projection::Orthographic(ortho) = projection {
        camera_limits.current_movement_radius =
            calculate_movement_radius(&camera_limits, ortho.scale);
    }

    info!(
        "Window resize: Updated camera limits - max_scale: {scale:.4}, movement_radius: {radius:.2}",
        scale = optimal_scale,
        radius = camera_limits.current_movement_radius
    );
}
