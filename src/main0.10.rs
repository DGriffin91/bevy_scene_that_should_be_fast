//! Loads and renders a glTF file as a scene.

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3.4, 1.0, 1.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    },));

    commands.spawn(SceneBundle {
        scene: asset_server.load("kitchen_gltf_no_window_cover.gltf#Scene0"),
        ..default()
    });
}
