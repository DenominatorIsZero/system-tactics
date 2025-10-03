//! SystemTactics Level Editor
//!
//! A Bevy-based level editor for creating hex maps, designing tactical scenarios,
//! and building encounters for SystemTactics.

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use shared::{LevelPlugin, RenderingPlugin};
use tracing::info;

fn main() {
    info!("Starting SystemTactics Level Editor application");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "SystemTactics Level Editor".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../assets".to_string(),
                    ..default()
                }),
        )
        .add_plugins(RenderingPlugin)
        .add_plugins(LevelPlugin)
        .add_systems(Update, placeholder_editor_system)
        .run();

    info!("SystemTactics Level Editor application shutting down");
}

fn placeholder_editor_system() {
    // Placeholder system for the level editor
    // TODO: Implement hex grid visualization and editing tools
}
