use bevy::prelude::*;

use crate::tb_combat::{Character, Seat};

pub struct GuiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);
const HOVERED_BUTTON: Color = Color::rgb(0.55, 0.55, 0.55);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

#[derive(Debug, Default)]
struct Buttons {
    change_lane_r: Option<Entity>,
    change_lane_l: Option<Entity>,
    dash: Option<Entity>,
    shoot_passenger: Option<Entity>,
    shoot_back_r: Option<Entity>,
    shoot_back_l: Option<Entity>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut buttons: ResMut<Buttons>) {
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
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    buttons.change_lane_l = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
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
                                                    "Change Lane (L)",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        })
                                        .id()
                                        .into();
                                    buttons.change_lane_r = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
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
                                                    "Change Lane (R)",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        })
                                        .id()
                                        .into();
                                    buttons.dash = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
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
                                                    "Dash (Driver)",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/SourceCodePro.ttf"),
                                                        font_size: 24.0,
                                                        color: Color::BLACK,
                                                    },
                                                )
                                                .with_style(Style { ..default() }),
                                            );
                                        })
                                        .id()
                                        .into();
                                    buttons.shoot_passenger = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
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
                                                "Shoot (Passenger)",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::BLACK,
                                                },
                                            ));
                                        })
                                        .id()
                                        .into();
                                    buttons.shoot_back_r = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
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
                                                "Shoot (Back R)",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::BLACK,
                                                },
                                            ));
                                        })
                                        .id()
                                        .into();
                                    buttons.shoot_back_l = parent
                                        .spawn_bundle(ButtonBundle {
                                            button: Button::default(),
                                            style: Style {
                                                size: Size {
                                                    width: Val::Px(200.0),
                                                    height: Val::Percent(50.0),
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
                                                "Shoot (Back L)",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::BLACK,
                                                },
                                            ));
                                        })
                                        .id()
                                        .into();
                                });
                        });
                });
        });
}

fn button_events(
    mut buttons_q: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
    buttons: Res<Buttons>,
    characters: Query<&Character>,
) {
    match buttons.change_lane_l {
        Some(button) => match buttons_q.get_mut(button) {
            Ok((interaction, mut color)) => match interaction {
                Interaction::Clicked => {
                    if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Driver) {
                        println!("Change Left");
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            },
            Err(_) => {}
        },
        None => {}
    }
    match buttons.change_lane_r {
        Some(button) => match buttons_q.get_mut(button) {
            Ok((interaction, mut color)) => match interaction {
                Interaction::Clicked => {
                    if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Driver) {
                        println!("Change Right");
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            },
            Err(_) => {}
        },
        None => {}
    }
    match buttons.dash {
        Some(button) => match buttons_q.get_mut(button) {
            Ok((interaction, mut color)) => match interaction {
                Interaction::Clicked => {
                    if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Driver) {
                        println!("Dash");
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            },
            Err(_) => {}
        },
        None => {}
    }
    match buttons.shoot_passenger {
        Some(button) => match buttons_q.get_mut(button) {
            Ok((interaction, mut color)) => match interaction {
                Interaction::Clicked => {
                    if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Passenger) {
                        println!("Shoot Passenger");
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            },
            Err(_) => {}
        },
        None => {}
    }
    match buttons.shoot_back_l {
        Some(button) => match buttons_q.get_mut(button) {
            Ok((interaction, mut color)) => match interaction {
                Interaction::Clicked => {
                    if let Some(driver) = characters.iter().find(|c| c.seat == Seat::BackLeft) {
                        println!("Shoot Back Left");
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            },
            Err(_) => {}
        },
        None => {}
    }
    match buttons.shoot_back_r {
        Some(button) => match buttons_q.get_mut(button) {
            Ok((interaction, mut color)) => match interaction {
                Interaction::Clicked => {
                    if let Some(driver) = characters.iter().find(|c| c.seat == Seat::BackRight) {
                        println!("Shoot Back Right");
                    }
                    *color = PRESSED_BUTTON.into();
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            },
            Err(_) => {}
        },
        None => {}
    }
}
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Buttons>()
            .add_system(button_events)
            .add_startup_system(setup);
    }
}
