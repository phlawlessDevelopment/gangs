use bevy::prelude::*;

pub struct GuiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);
const HOVERED_BUTTON: Color = Color::rgb(0.55, 0.55, 0.55);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(buttons).add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(17.5)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexStart,
                                justify_content: JustifyContent::SpaceBetween,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(10.0),
                                                },
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::new(
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(50.0),
                                                    Val::Px(50.0),
                                                ),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(
                                                TextBundle::from_section(
                                                    "Change Lane",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        });
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(10.0),
                                                },
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::new(
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(50.0),
                                                    Val::Px(50.0),
                                                ),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(
                                                TextBundle::from_section(
                                                    "Dash",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        });
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(10.0),
                                                },
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::new(
                                                    Val::Px(0.0),
                                                    Val::Px(0.0),
                                                    Val::Px(50.0),
                                                    Val::Px(50.0),
                                                ),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(TextBundle::from_section(
                                                "Shoot",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::BLACK,
                                                },
                                            ));
                                        });
                                });
                        })
                        .id()
                        .id();
                });
        });
}

fn buttons(
    mut button_q: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    text_q: Query<&Text>,
) {
    for (interaction, mut color, children) in button_q.iter_mut() {
        let text = text_q.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match text.sections[0].value.as_str() {
                    "Change Lane" => {
                        println!("Change Lane");
                    }
                    "Dash" => {
                        println!("Dash");
                    }
                    "Shoot" => {
                        println!("Shoot");
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
