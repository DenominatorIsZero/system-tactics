//! SystemTactics Level Editor
//!
//! A Bevy-based level editor for creating hex maps, designing tactical scenarios,
//! and building encounters for SystemTactics.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SystemTactics Level Editor".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, placeholder_editor_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Add editor UI placeholder
    commands.spawn((
        Text2d::new("SystemTactics Level Editor\nHex Map Creation Tool"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn placeholder_editor_system() {
    // Placeholder system for the level editor
    // TODO: Implement hex grid visualization and editing tools
}
