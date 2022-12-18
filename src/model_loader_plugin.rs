use bevy::{prelude::*};

use crate::AppState;

struct LoadModelEvent {
    pub path: String,
}

#[derive(Default)]
struct UnLoadModelEvent;

pub struct ModelLoaderPlugin;

#[derive(Component)]
pub struct Model;

#[derive(Resource)]
struct LoaderState(bool);

impl Plugin for ModelLoaderPlugin {
    fn build(&self, app: &mut App){
        app //.add_system_set(SystemSet::on_enter(AppState::Display).with_system(spawn_gltf))
        .add_event::<LoadModelEvent>()
        .add_event::<UnLoadModelEvent>()
        .insert_resource(LoaderState(false))
        .add_system_set(
            SystemSet::on_enter(AppState::Display)
            .with_system(init_loader)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Display)
                .with_system(keyboard_input_system)
                .with_system(load_event_listener)
                .with_system(unload_event_listener)
        );
    }
}

fn init_loader(
    mut commands: Commands
){

}
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut load_events: EventWriter<LoadModelEvent>,
    mut unload_events: EventWriter<UnLoadModelEvent>
) {

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
        load_events.send(LoadModelEvent {
            path: "LL_house.gltf#Scene0".to_string(),
        });
    }

    if keyboard_input.just_released(KeyCode::U) {
        info!("'U' just released");
        unload_events.send_default();
    }
}

fn load_event_listener(
    mut events: EventReader<LoadModelEvent>,
    mut commands: Commands,
    mut state: ResMut<LoaderState>,
    ass: Res<AssetServer>,
) {
    for load_event in events.iter() {
        info!("{}", load_event.path);
        info!("Loader state is: {}",state.0);

        if !state.0 {
            let my_gltf = ass.load(load_event.path.to_string());

            // to position our 3d model, simply use the Transform
            // in the SceneBundle
            commands.spawn(SceneBundle {
                scene: my_gltf,
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            })
            .insert(Model);
            state.0 = true;
        }
    }
}

fn unload_event_listener(
    mut events: EventReader<UnLoadModelEvent>,
    model_root: Query<Entity, With<Model>>,
    mut state: ResMut<LoaderState>,
    mut commands: Commands,
){
    for _load_event in events.iter() {
        info!("Loader state is: {}",state.0);
        if state.0 {
            let root_entity = model_root.single();
            commands.entity(root_entity).despawn_recursive();
            info!("Event!");
            state.0 = false;
        }
    }
}