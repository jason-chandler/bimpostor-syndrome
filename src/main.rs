mod main_menu;
mod model_loader_plugin;
mod camera;
mod ui;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
<<<<<<< HEAD
use bevy_atmosphere::prelude::*;
=======
use std::f32::consts::PI;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
>>>>>>> main

pub use main_menu::*;
pub use model_loader_plugin::*;


pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Display,
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .insert_resource(AtmosphereModel::default())
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
    .add_state(AppState::Display)
    .add_plugin(WorldInspectorPlugin::new())
    .add_plugin(MainMenuPlugin)
<<<<<<< HEAD
    .add_plugin(AtmospherePlugin)
    .add_system(camera::set_brightness)
=======
    .add_plugin(ModelLoaderPlugin)
    .add_plugins(DefaultPickingPlugins)
>>>>>>> main
    .add_startup_system(spawn_camera)
    .add_system_set(
        SystemSet::on_enter(AppState::Display)
        .with_system(spawn_basic_scene)
        .with_system(ui::setup_ui_components)
    )
    .add_system(camera::orbital_camera)
    .add_startup_system(spawn_gltf)
    .run();
}

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut transform_query: Query<Entity>,
    mut child_query: Query<&Children>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("LL_house.gltf#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    let scene  = commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    })
    .insert(Name::new("Test"))
    .insert(PickableBundle::default()).id();


    for mut child in child_query.iter_descendants(scene) {
        commands.entity(child).insert(PickableBundle::default());
    }
}

// https://bevyengine.org/examples/3d/lighting/
fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        }, PickableBundle::default()))
        .insert(Name::new("Ground"));

        const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    camera::spawn_camera(commands)
}
