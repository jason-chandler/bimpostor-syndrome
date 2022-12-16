use bevy::{
	prelude::*,
  core_pipeline::clear_color::ClearColorConfig
};

pub(crate) fn setup_ui_components(mut commands: Commands, asset_server: Res<AssetServer>) {

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
            image: asset_server.load("images/crosshairs/crosshair.png").into(),
            ..default()
          });
        });

        parent_lv1
          .spawn(NodeBundle {
            style: Style {
              size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
              position_type: PositionType::Absolute,
              position: UiRect {
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
              },
              justify_content: JustifyContent::FlexEnd,
              align_items: AlignItems::FlexStart,
              ..default()
            },
            ..default()
          })
          .with_children(|parent| {
            // bevy logo (image)
            parent.spawn(ImageBundle {
              style: Style {
                size: Size::new(Val::Px(100.0), Val::Auto),
                ..default()
              },
              image: asset_server.load("images/ui_elements/fieldwire-logo-stack.png").into(),
              ..default()
            });
          });

        parent_lv1
          .spawn(NodeBundle {
            style: Style {
              size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
              position_type: PositionType::Absolute,
              position: UiRect {
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
              },
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
                size: Size::new(Val::Px(300.0), Val::Auto),
                ..default()
              },
              image: asset_server.load("images/ui_elements/fieldwire-logo-stack.png").into(),
              ..default()
            });
          });

    });
}