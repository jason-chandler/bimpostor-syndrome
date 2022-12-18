mod main_menu;
mod model_loader_plugin;
mod selection;
mod camera;
mod ui;
mod background;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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
    .add_plugin(ModelLoaderPlugin)
    .add_startup_system(spawn_camera)
    .add_system_set(
        SystemSet::on_enter(AppState::Display)
        .with_system(spawn_basic_scene)
        .with_system(ui::setup_ui_components)
        .with_system(background::setup)
    )
    .add_system(camera::orbital_camera)
    .add_startup_system(selection::init_selection_material)
    .add_system(selection::add_selection)
    .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
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
