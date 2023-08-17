//! Loads and renders a glTF file as a scene.

use bevy::{
    core_pipeline::upscaling::UpscalingNode,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{
        camera::CameraRenderGraph,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        view::NoFrustumCulling,
        RenderApp,
    },
    window::PresentMode,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            NoRenderPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        camera_render_graph: CameraRenderGraph::new("NOTHING"),
        transform: Transform::from_xyz(3.4, 1.0, 1.5).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    },));
    //let mesh = meshes.add(Mesh::from(shape::UVSphere {
    //    radius: 0.5,
    //    sectors: 1024,
    //    stacks: 1024,
    //}));
    //let material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    let scene: Handle<Scene> = asset_server.load("kitchen_gltf_no_window_cover.gltf#Scene0");

    for x in -20..=20 {
        for z in -20..=20 {
            //commands.spawn(PbrBundle {
            //    mesh: mesh.clone(),
            //    material: material.clone(),
            //    transform: Transform::from_xyz(x as f32 * 2.0, y as f32 * 2.0, z as f32 * 2.0),
            //    ..default()
            //});
            commands.spawn((
                SceneBundle {
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 60.0,
                        0.0,
                        z as f32 * 60.0,
                    )),
                    scene: scene.clone(),
                    //visibility: Visibility::Hidden,
                    ..default()
                },
                //NoFrustumCulling,
            ));
        }
    }
}

struct NoRenderPlugin;
impl Plugin for NoRenderPlugin {
    fn build(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_sub_graph("NOTHING")
            .add_render_graph_node::<ViewNodeRunner<UpscalingNode>>(
                "NOTHING",
                bevy::core_pipeline::core_3d::graph::node::UPSCALING,
            );
    }
}
