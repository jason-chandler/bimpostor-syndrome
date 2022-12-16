mod camera;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "BIM Viewer".to_string(),
            resizable: false,
            ..Default::default()
        },
        ..default()
    }))
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    .add_startup_system(asset_loading)
    .add_system(camera::orbital_camera)
    .add_startup_system(spawn_gltf)
    .run();
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("LL_house.gltf#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(Name::new("Test"));
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {

}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    camera::spawn_camera(commands)
}
