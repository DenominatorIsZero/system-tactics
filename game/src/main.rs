//! SystemTactics - Main Game Binary
//!
//! A tactical RPG inspired by Final Fantasy Tactics with unique god evolution
//! and system apocalypse mechanics, built in Bevy and compiled to WASM.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SystemTactics".into(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, placeholder_system)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Add some placeholder text
    commands.spawn((
        Text2d::new("SystemTactics - Coming Soon\nTactical RPG with God Evolution"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn placeholder_system() {
    // Placeholder system for the game loop
    // TODO: Implement actual game systems
}
