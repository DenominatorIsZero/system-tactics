//! SystemTactics - Main Game Binary
//!
//! A tactical RPG inspired by Final Fantasy Tactics with unique god evolution
//! and system apocalypse mechanics, built in Bevy and compiled to WASM.

use bevy::prelude::*;
use tracing::{info, debug};

fn main() {
    info!("Starting SystemTactics game application");

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

    info!("SystemTactics game application shutting down");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setting up 3D scene with camera, lighting, and test cube");

    // Spawn a 3D camera
    let camera_pos = Vec3::new(5.0, 5.0, 5.0);
    let camera_target = Vec3::ZERO;
    debug!("Spawning 3D camera at position {} looking at {}", camera_pos, camera_target);
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(camera_pos.x, camera_pos.y, camera_pos.z).looking_at(camera_target, Vec3::Y),
    ));

    // Add basic lighting
    let light_pos = Vec3::new(4.0, 8.0, 4.0);
    let light_target = Vec3::ZERO;
    let light_illuminance = 10000.0;
    debug!("Adding directional light at position {} with illuminance {}", light_pos, light_illuminance);
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: light_illuminance,
            ..default()
        },
        Transform::from_xyz(light_pos.x, light_pos.y, light_pos.z).looking_at(light_target, Vec3::Y),
    ));

    let ambient_brightness = 300.0;
    debug!("Setting up ambient lighting with brightness {}", ambient_brightness);
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: ambient_brightness,
        affects_lightmapped_meshes: true,
    });

    // Spawn a test cube to verify 3D rendering works
    let cube_pos = Vec3::new(0.0, 0.5, 0.0);
    let cube_size = Vec3::new(1.0, 1.0, 1.0);
    let cube_color = Color::srgb(0.8, 0.7, 0.6);
    debug!("Spawning test cube at position {} with size {} and color {:?}", cube_pos, cube_size, cube_color);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(cube_size.x, cube_size.y, cube_size.z))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: cube_color,
            ..default()
        })),
        Transform::from_xyz(cube_pos.x, cube_pos.y, cube_pos.z),
    ));

    info!("3D scene setup complete - camera, lighting, and test cube spawned");
}

fn placeholder_system() {
    // Placeholder system for the game loop
    // This runs every frame - using trace level to avoid spam
    // TODO: Implement actual game systems
}
