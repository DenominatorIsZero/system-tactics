//! SystemTactics - Main Game Binary
//!
//! A tactical RPG inspired by Final Fantasy Tactics with unique god evolution
//! and system apocalypse mechanics, built in Bevy and compiled to WASM.

use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use shared::colors::LIGHT_BACKGROUND;
use shared::input::InputPlugin;
use shared::level::LevelPlugin;
use shared::rendering::RenderingPlugin;
use tracing::info;

#[cfg(target_arch = "wasm32")]
use bevy::asset::embedded_asset;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    info!("Starting SystemTactics game application");

    let mut app = App::new();

    // Configure plugins with conditional asset handling
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SystemTactics".into(),
                    fit_canvas_to_parent: true,
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(configure_asset_plugin()),
    );

    // Embed assets for WASM builds
    #[cfg(target_arch = "wasm32")]
    {
        info!("Embedding assets for WASM build");
        embedded_asset!(app, "assets/uv_checker.png");
        embedded_asset!(app, "assets/levels/default.toml");
        embedded_asset!(app, "assets/levels/test_small.toml");
        embedded_asset!(app, "assets/levels/test_large.toml");
    }

    app.insert_resource(ClearColor(LIGHT_BACKGROUND))
        .add_plugins(InputPlugin)
        .add_plugins(RenderingPlugin)
        .add_plugins(LevelPlugin)
        .add_systems(Update, placeholder_system)
        .run();

    info!("SystemTactics game application shutting down");
}

fn configure_asset_plugin() -> AssetPlugin {
    #[cfg(target_arch = "wasm32")]
    {
        info!("Configuring AssetPlugin for WASM (embedded assets)");
        AssetPlugin::default()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        info!("Configuring AssetPlugin for native (file system assets)");
        AssetPlugin {
            file_path: "../assets".to_string(),
            ..default()
        }
    }
}

fn placeholder_system() {
    // Placeholder system for the game loop
    // This runs every frame - using trace level to avoid spam
    // TODO: Implement actual game systems
}
