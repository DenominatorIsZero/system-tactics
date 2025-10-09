//! SystemTactics - Main Game Binary
//!
//! A tactical RPG inspired by Final Fantasy Tactics with unique god evolution
//! and system apocalypse mechanics, built in Bevy and compiled to WASM.

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use shared::{LIGHT_BACKGROUND, LevelPlugin, RenderingPlugin};
use tracing::info;

fn main() {
    info!("Starting SystemTactics game application");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SystemTactics".into(),
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../assets".to_string(),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(LIGHT_BACKGROUND))
        .add_plugins(RenderingPlugin)
        .add_plugins(LevelPlugin)
        .add_systems(Update, placeholder_system)
        .run();

    info!("SystemTactics game application shutting down");
}

fn placeholder_system() {
    // Placeholder system for the game loop
    // This runs every frame - using trace level to avoid spam
    // TODO: Implement actual game systems
}
