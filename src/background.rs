use std::f32::consts::PI;
use bevy::prelude::*;

/// sets up a scene with textured entities
pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // load a texture and retrieve its aspect ratio
    // let texture_handle = asset_server.load("images/ui_elements/fieldwire-logo-stack.png");
    let texture_handle = asset_server.load("images/backgrounds/skybox.png");
    let aspect = 0.5;

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 2000.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, -300.0),
        ..default()
    });
}