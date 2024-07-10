use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_grow: 1.,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            ..Default::default()
        })
        .with_children(|commands| {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(300.),
                        height: Val::Px(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            commands.spawn(
                                TextBundle::from_section(
                                    "Lorem ipsum dolor sit amet",
                                    TextStyle {
                                        font_size: 25.,
                                        ..Default::default()
                                    },
                                )
                                .with_no_wrap()
                            );
                        });
                });
        });
}

fn update(time: Res<Time>, mut query: Query<&mut Style, With<Text>>) {
    for mut style in query.iter_mut() {
        style.margin.left = Val::Px(time.elapsed_seconds().cos() * 100.);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
