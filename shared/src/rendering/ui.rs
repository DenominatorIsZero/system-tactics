//! User Interface Systems
//!
//! UI components and systems for level name display, FPS counter,
//! and other tactical RPG interface elements.

use bevy::prelude::*;
use tracing::info;

use crate::colors::YELLOW_ACCENT;
use crate::level::LevelsResource;

/// Component to mark the level name display text
#[derive(Component)]
pub struct LevelNameDisplay;

/// Component to mark the FPS counter display text
#[derive(Component)]
pub struct FpsDisplay;

/// System to spawn the level name UI text in the bottom-right corner
pub fn spawn_level_name_ui(mut commands: Commands, levels_resource: Res<LevelsResource>) {
    let level = levels_resource.current_level();
    info!(
        "Spawning level name UI for level: '{level_name}'",
        level_name = level.name
    );

    // Create UI text positioned in bottom-right corner of screen
    let entity = commands
        .spawn((
            Text::new(&level.name),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::BLACK),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                ..default()
            },
            LevelNameDisplay,
        ))
        .id();

    info!("Level name UI entity spawned: {entity:?} at bottom-right corner");
}

/// System to update the level name display when the levels resource changes
pub fn update_level_name_display(
    levels_resource: Res<LevelsResource>,
    mut text_query: Query<&mut Text, With<LevelNameDisplay>>,
) {
    if levels_resource.is_changed() {
        let level = levels_resource.current_level();
        info!(
            "Level changed, updating level name display to: '{level_name}'",
            level_name = level.name
        );
        for mut text in text_query.iter_mut() {
            **text = level.name.clone();
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
            TextColor(YELLOW_ACCENT),
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
