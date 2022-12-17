use bevy::{prelude::*};

use crate::AppState;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_system_set(
            SystemSet::on_update(AppState::Menu)
                .with_system(start_button_clicked),
        );
    }
}

fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut app_state: ResMut<State<AppState>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    for interaction in &interactions {
        match *interaction {
            Interaction::Clicked => {
                let root_entity = menu_root.single();
                commands.entity(root_entity).despawn_recursive();

                app_state.set(AppState::Display).unwrap();
                mouse_input.clear();
            }
            Interaction::Hovered => {
                //*color = Color::BLUE.into();
            }
            Interaction::None => {
                //*color = Color::RED.into();
            }
        }   
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = spawn_button(&mut commands, &asset_server, "View Bim button", Color::RED);
    commands.entity(start_button).insert(StartButton);

    commands
    .spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .insert(MenuUIRoot)
    .with_children(|commands| {
        commands.spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Percent(3.0)),
                ..default()
            },
            text: Text::from_section(
                "FieldWire BIM viewer",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 96.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        });
    })
    .add_child(start_button);
}

fn spawn_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    color: Color,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(65.0), Val::Percent(15.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 64.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .id()
}