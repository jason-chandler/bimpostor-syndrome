use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::reflect::{TypeUuid, Uuid};
use bevy::render::camera::{Projection, RenderTarget};
use bevy::render::render_resource::{AsBindGroup};

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

#[derive(Component, Debug, Default, Clone)]
pub(crate) struct Selectable {
    pub selected: bool,
    pub unselected_color: Handle<ColorMaterial>,
    pub selected_color: CustomMaterial,
}

impl Selectable {
    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn add_selection(
    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mobile_input: Res<Touches>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut selectable_query: Query<(&mut Selectable, &Interaction)>,
    ui_node_query: Query<&Interaction, With<Node>>,
) {
    let mut change_selected = false;

    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            change_selected = true;
        }
    }

    if change_selected {
        for (mut selectable, interaction) in selectable_query.iter_mut() {
            if selectable.selected && *interaction != Interaction::Clicked {
                selectable.selected = false;
            } else if *interaction == Interaction::Clicked {
                selectable.selected = !selectable.selected;
            }
        }
    } else {
        // Deselect everything if the user clicks anything not selectable and it's not the ui
        let mut ignore_click = false;
        for interaction in ui_node_query.iter() {
            if *interaction == Interaction::Clicked {
                ignore_click = true;
            }
        }

        let mouse_triggered = mouse_input.just_pressed(MouseButton::Left) || mobile_input.iter_just_pressed().next().is_some();
        if mouse_triggered && !ignore_click {
            for (mut selectable, _interaction) in &mut selectable_query.iter_mut() {
                if selectable.selected {
                    selectable.selected = false;
                }
            }
        }
    }
}

pub(crate) fn init_selection_material(
    mut commands: Commands,
    entity_material_query: Query<(Entity, &Handle<ColorMaterial>)>,
    mut color_query: Query<&mut Selectable>,
) {
    for (entity, material) in entity_material_query.iter() {
        match color_query.get_mut(entity) {
            Ok(mut selectable) => selectable.unselected_color = material.to_owned(),
            _ => {
                let init_component = Selectable {
                    selected: false,
                    unselected_color: material.to_owned(),
                    selected_color: CustomMaterial {
                        color: Color::BLUE,
                        color_texture: None,
                        alpha_mode: AlphaMode::Blend,
                    },
                    ..default()
                };
                commands.entity(entity).insert(init_component);
            }
        }
    }
}
