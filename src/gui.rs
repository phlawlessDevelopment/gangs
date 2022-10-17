use bevy::prelude::*;

use crate::tb_combat::{Character, Seat};

pub struct GuiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);
const HOVERED_BUTTON: Color = Color::rgb(0.55, 0.55, 0.55);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

#[derive(Debug, Component)]
struct ChangeLaneLeft;

#[derive(Debug, Component)]
struct ChangeLaneRight;
#[derive(Debug, Component)]
struct Dash;
#[derive(Debug, Component)]
struct ShootPassenger;

#[derive(Debug, Component)]
struct ShootBackR;

#[derive(Debug, Component)]
struct ShootBackL;

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
                                        flex_direction: FlexDirection::Row,
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
                                        .insert(ChangeLaneLeft)
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
                                        });
                                    parent
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
                                        .insert(ChangeLaneRight)
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
                                        });
                                    parent
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
                                        .insert(Dash)
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
                                        });
                                    parent
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
                                        .insert(ShootPassenger)
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
                                        });
                                    parent
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
                                        .insert(ShootBackR)
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
                                        });
                                    parent
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
                                        .insert(ShootBackL)
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
                                        });
                                });
                        });
                });
        });
}

fn button_events(
    mut buttons: ParamSet<(
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ChangeLaneLeft>)>,
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ChangeLaneRight>)>,
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Dash>)>,
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ShootPassenger>)>,
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ShootBackL>)>,
        Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<ShootBackR>)>,
    )>,
    text_q: Query<&Text>,
    characters: Query<&Character>,
) {
    // let (cll_interaction, mut cll_color) = buttons.p0().single_mut();
    // let (clr_interaction, mut clr_color) = buttons.p1().single();
    // let (dash_interaction, mut dash_color) = buttons.p2().single();
    // let (shoot_pass_interaction, mut shoot_pass_color) = buttons.p3().single();
    // let (shoot_l_interaction, mut shoot_l_color) = buttons.p4().single();
    // let (shoot_r_interaction, mut shoot_r_color) = buttons.p5().single();

    match buttons.p0().single().0 {
        Interaction::Clicked => {
            if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Driver) {
                println!("Change Left");
            }
            *buttons.p0().single_mut().1 = PRESSED_BUTTON.into();
        }
        Interaction::Hovered => {
            *buttons.p0().single_mut().1 = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *buttons.p0().single_mut().1 = NORMAL_BUTTON.into();
        }
    }
    // match *clr_interaction {
    //     Interaction::Clicked => {
    //         if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Driver) {
    //             println!("Change Right");
    //         }
    //         *clr_color = PRESSED_BUTTON.into();
    //     }
    //     Interaction::Hovered => {
    //         *clr_color = HOVERED_BUTTON.into();
    //     }
    //     Interaction::None => {
    //         *clr_color = NORMAL_BUTTON.into();
    //     }
    // }
    // match *dash_interaction {
    //     Interaction::Clicked => {
    //         if let Some(driver) = characters.iter().find(|c| c.seat == Seat::Driver) {
    //             println!("Dash");
    //         }
    //         *dash_color = PRESSED_BUTTON.into();
    //     }
    //     Interaction::Hovered => {
    //         *dash_color = HOVERED_BUTTON.into();
    //     }
    //     Interaction::None => {
    //         *dash_color = NORMAL_BUTTON.into();
    //     }
    // }
    // match *shoot_pass_interaction {
    //     Interaction::Clicked => {
    //         if let Some(passenger) = characters.iter().find(|c| c.seat == Seat::Passenger) {
    //             println!("Shoot Passenger");
    //         }
    //         *shoot_pass_color = PRESSED_BUTTON.into();
    //     }
    //     Interaction::Hovered => {
    //         *shoot_pass_color = HOVERED_BUTTON.into();
    //     }
    //     Interaction::None => {
    //         *shoot_pass_color = NORMAL_BUTTON.into();
    //     }
    // }
    // match *shoot_l_interaction {
    //     Interaction::Clicked => {
    //         if let Some(back_left) = characters.iter().find(|c| c.seat == Seat::BackLeft) {
    //             println!("Shoot Left");
    //         }
    //         *shoot_l_color = PRESSED_BUTTON.into();
    //     }
    //     Interaction::Hovered => {
    //         *shoot_l_color = HOVERED_BUTTON.into();
    //     }
    //     Interaction::None => {
    //         *shoot_l_color = NORMAL_BUTTON.into();
    //     }
    // }
    // match *shoot_r_interaction {
    //     Interaction::Clicked => {
    //         if let Some(back_right) = characters.iter().find(|c| c.seat == Seat::BackRight) {
    //             println!("Shoot Right");
    //         }
    //         *shoot_r_color = PRESSED_BUTTON.into();
    //     }
    //     Interaction::Hovered => {
    //         *shoot_r_color = HOVERED_BUTTON.into();
    //     }
    //     Interaction::None => {
    //         *shoot_r_color = NORMAL_BUTTON.into();
    //     }
    // }
}
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_events).add_startup_system(setup);
    }
}
