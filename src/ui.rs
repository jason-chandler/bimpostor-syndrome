use bevy::{
	prelude::*
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// root node
	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::SpaceBetween,
				..default()
			},
			..default()
		})
		.with_children(|parent_lv1| {

			parent_lv1
				.spawn(NodeBundle {
					style: Style {
						size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
						position_type: PositionType::Absolute,
						justify_content: JustifyContent::FlexEnd,
						align_items: AlignItems::FlexStart,
						..default()
					},
					..default()
				})
				.with_children(|parent| {
					parent
            .spawn(ImageBundle {
              style: Style {
                size: Size::new(Val::Px(100.0), Val::Auto),
                ..default()
              },
              image: asset_server.load("images/ui_elements/fieldwire.png").into(),
              ..default()
            });
        });

      parent_lv1
        .spawn(ButtonBundle {
          style: Style {
              size: Size::new(Val::Px(150.0), Val::Px(65.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings 11",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
      
      parent_lv1
        .spawn(ButtonBundle {
          style: Style {
              size: Size::new(Val::Px(150.0), Val::Px(65.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings 12",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

        parent_lv1
          .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
          })
          .with_children(|parent| {
              parent.spawn(TextBundle::from_section(
                  "Settings 13",
                  TextStyle {
                      font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                      font_size: 40.0,
                      color: Color::rgb(0.9, 0.9, 0.9),
                  },
              ));
          });

      // Crosshair
      parent_lv1
        .spawn(NodeBundle {
          style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
          },
          ..default()
        })
        .with_children(|parent| {
          // bevy logo (image)
          parent.spawn(ImageBundle {
            style: Style {
              size: Size::new(Val::Px(50.0), Val::Auto),
              ..default()
            },
            image: asset_server.load("images/crosshairs/crosshair2.png").into(),
            ..default()
          });
        });


      parent_lv1
        .spawn(ButtonBundle {
          style: Style {
              size: Size::new(Val::Px(150.0), Val::Px(65.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings 1",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });

      parent_lv1
        .spawn((
          // Create a TextBundle that has a Text with a single section.
          TextBundle::from_section(
              // Accepts a `String` or any type that converts into a `String`, such as `&str`
              "hello\nbevy!",
              TextStyle {
                  font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                  font_size: 100.0,
                  color: Color::WHITE,
              },
          ) // Set the alignment of the Text
          .with_text_alignment(TextAlignment::TOP_CENTER)
          // Set the style of the TextBundle itself.
          .with_style(Style {
              position_type: PositionType::Absolute,
              position: UiRect {
                  bottom: Val::Px(5.0),
                  right: Val::Px(15.0),
                  ..default()
              },
              ..default()
          }),
          ColorText,
      ));


      parent_lv1
        .spawn(ButtonBundle {
          style: Style {
              size: Size::new(Val::Px(150.0), Val::Px(65.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings 2",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
      
      parent_lv1
        .spawn(ButtonBundle {
          style: Style {
              size: Size::new(Val::Px(150.0), Val::Px(65.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Settings 3",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
    });
}


// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub struct ColorText;

pub(crate) fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
  for mut text in &mut query {
    let seconds = time.elapsed_seconds();

    // Update the color of the first and only section.
    text.sections[0].style.color = Color::Rgba {
      red: (1.25 * seconds).sin() / 2.0 + 0.5,
      green: (0.75 * seconds).sin() / 2.0 + 0.5,
      blue: (0.50 * seconds).sin() / 2.0 + 0.5,
      alpha: 1.0,
    };
  }
}

pub(crate) fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
        Interaction::Clicked => {
            text.sections[0].value = "Press".to_string();
            *color = PRESSED_BUTTON.into();
        }
        Interaction::Hovered => {
            text.sections[0].value = "Hover".to_string();
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    }
  }
}