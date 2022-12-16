use bevy::{
	prelude::*
};

pub(crate) fn setup_ui_components(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    });
}