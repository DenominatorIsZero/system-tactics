//! Debug Aid Systems
//!
//! Visual debugging aids for camera systems, raycasting, and game mechanics.
//! All debug aids can be toggled with F1 key.

use bevy::prelude::*;
use tracing::debug;

use crate::{
    colors::YELLOW_ACCENT,
    rendering::camera::{TacticalCamera, calculate_camera_focus_point},
};

/// Component marker for debug crosshair UI elements
#[derive(Component)]
pub struct DebugCrosshair;

/// Component marker for debug camera position text
#[derive(Component)]
pub struct DebugCameraText;

/// Component marker for debug focus point text
#[derive(Component)]
pub struct DebugFocusText;

/// Component marker for debug distance text
#[derive(Component)]
pub struct DebugDistanceText;

/// Resource to track whether debug aids should be visible
#[derive(Resource, Default)]
pub struct DebugAidVisibility {
    pub visible: bool, // Defaults to false - start with debug aids hidden
}

/// System to render intersection point sphere (world-space debug marker)
pub fn camera_intersection_debug_system(
    mut gizmos: Gizmos,
    debug_visibility: Res<DebugAidVisibility>,
    camera_query: Query<&Transform, With<TacticalCamera>>,
) {
    // Only render if debug aids are visible
    if !debug_visibility.visible {
        return;
    }

    if let Ok(transform) = camera_query.single() {
        // Calculate focus point intersection with ground plane
        let focus_point = calculate_camera_focus_point(transform);

        // Draw sphere at intersection point - this shows where the ray hits the ground
        gizmos.sphere(focus_point, 0.15, Color::srgba(1.0, 0.0, 0.0, 0.9)); // Red sphere
    }
}

/// System to spawn/despawn crosshair UI elements
pub fn debug_crosshair_system(
    mut commands: Commands,
    debug_visibility: Res<DebugAidVisibility>,
    existing_crosshair_query: Query<Entity, With<DebugCrosshair>>,
) {
    let crosshair_exists = !existing_crosshair_query.is_empty();

    if debug_visibility.visible && !crosshair_exists {
        // Spawn crosshair UI elements - horizontal and vertical lines using borders
        let line_color = Color::srgba(1.0, 1.0, 0.0, 0.8); // Yellow color
        let line_thickness = 2.0;
        let line_length = 20.0;

        // Horizontal line (centered)
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(line_length),
                height: Val::Px(line_thickness),
                border: UiRect::all(Val::Px(1.0)),
                // Center the element by offsetting by half its size
                margin: UiRect {
                    left: Val::Px(-line_length / 2.0),
                    top: Val::Px(-line_thickness / 2.0),
                    ..default()
                },
                ..default()
            },
            BorderColor(line_color),
            BackgroundColor(line_color),
            DebugCrosshair,
        ));

        // Vertical line (centered)
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(line_thickness),
                height: Val::Px(line_length),
                border: UiRect::all(Val::Px(1.0)),
                // Center the element by offsetting by half its size
                margin: UiRect {
                    left: Val::Px(-line_thickness / 2.0),
                    top: Val::Px(-line_length / 2.0),
                    ..default()
                },
                ..default()
            },
            BorderColor(line_color),
            BackgroundColor(line_color),
            DebugCrosshair,
        ));

        debug!("Spawned debug crosshair UI elements");
    } else if !debug_visibility.visible && crosshair_exists {
        // Despawn existing crosshair elements
        for entity in existing_crosshair_query.iter() {
            commands.entity(entity).despawn();
        }
        debug!("Despawned debug crosshair UI elements");
    }
}

/// System to spawn/despawn debug text UI elements underneath FPS counter
pub fn debug_text_spawn_system(
    mut commands: Commands,
    debug_visibility: Res<DebugAidVisibility>,
    existing_camera_text: Query<Entity, With<DebugCameraText>>,
    existing_focus_text: Query<Entity, With<DebugFocusText>>,
    existing_distance_text: Query<Entity, With<DebugDistanceText>>,
) {
    let texts_exist = !existing_camera_text.is_empty()
        || !existing_focus_text.is_empty()
        || !existing_distance_text.is_empty();

    if debug_visibility.visible && !texts_exist {
        // Spawn debug text lines underneath FPS counter
        let text_color = YELLOW_ACCENT;
        let font_size = 16.0;

        // Camera position text (first line below FPS)
        commands.spawn((
            Text::new("Camera: (0.0, 0.0, 0.0)"),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(50.0), // 30px below FPS counter
                left: Val::Px(20.0),
                ..default()
            },
            DebugCameraText,
        ));

        // Focus point text (second line)
        commands.spawn((
            Text::new("Focus: (0.0, 0.0, 0.0)"),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(75.0), // 25px spacing
                left: Val::Px(20.0),
                ..default()
            },
            DebugFocusText,
        ));

        // Distance text (third line)
        commands.spawn((
            Text::new("Distance: 0.0"),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(text_color),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0), // 25px spacing
                left: Val::Px(20.0),
                ..default()
            },
            DebugDistanceText,
        ));

        debug!("Spawned debug text UI elements underneath FPS counter");
    } else if !debug_visibility.visible && texts_exist {
        // Despawn existing debug text elements
        for entity in existing_camera_text
            .iter()
            .chain(existing_focus_text.iter())
            .chain(existing_distance_text.iter())
        {
            commands.entity(entity).despawn();
        }
        debug!("Despawned debug text UI elements");
    }
}

// Type alias to reduce complexity
type DistanceTextQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<DebugDistanceText>,
        Without<DebugCameraText>,
        Without<DebugFocusText>,
    ),
>;

/// System to update debug text content with current camera data
pub fn debug_text_update_system(
    camera_query: Query<&Transform, (With<TacticalCamera>, Changed<Transform>)>,
    mut camera_text_query: Query<&mut Text, With<DebugCameraText>>,
    mut focus_text_query: Query<&mut Text, (With<DebugFocusText>, Without<DebugCameraText>)>,
    mut distance_text_query: DistanceTextQuery,
) {
    // Only update when camera transform has changed
    if let Ok(transform) = camera_query.single() {
        let camera_pos = transform.translation;
        let focus_point = calculate_camera_focus_point(transform);
        let distance = camera_pos.distance(focus_point);

        // Update camera position text
        for mut text in camera_text_query.iter_mut() {
            **text = format!(
                "Camera: ({x:.1}, {y:.1}, {z:.1})",
                x = camera_pos.x,
                y = camera_pos.y,
                z = camera_pos.z
            );
        }

        // Update focus point text
        for mut text in focus_text_query.iter_mut() {
            **text = format!(
                "Focus: ({x:.1}, {y:.1}, {z:.1})",
                x = focus_point.x,
                y = focus_point.y,
                z = focus_point.z
            );
        }

        // Update distance text
        for mut text in distance_text_query.iter_mut() {
            **text = format!("Distance: {distance:.1}");
        }
    }
}
