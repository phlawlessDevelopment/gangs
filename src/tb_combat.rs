use bevy::prelude::*;

pub mod gui;
pub struct TBCombatPlugin;

#[derive(Debug, PartialEq, Eq)]
enum AttackType {
    Single,
    Multi,
    Cone,
    Circle,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Team {
    Player,
    Ai,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Seat {
    Driver,
    Passenger,
    BackLeft,
    BackRight,
}
#[derive(Component)]
struct Attack {
    dmg: u32,
    atk_type: AttackType,
}
#[derive(Component)]
pub struct Character {
    pub team: Team,
    pub seat: Seat,
    pub can_atk: bool,
}

#[derive(Component)]
pub struct Vehicle {
    team: Team,
}
#[derive(Component)]
pub struct GridPosition {
    x: u32,
    y: u32,
}
#[derive(Default)]
struct CharactersCanAttack {
    passenger: bool,
    back_right: bool,
    back_left: bool,
}
#[derive(Default)]
pub struct VehicleMovement {
    pub vehicle: Option<Entity>,
    pub movement: Vec2,
}

fn create_vehicle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: (f32, f32),
    sprite_path: &str,
    x: u32,
    y: u32,
    is_player: bool,
) {
    commands
        .spawn()
        .insert(Name::from("Vehicle"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(position.0, position.1, 3.0))
                .with_rotation(Quat::from_rotation_z((90.0 as f32).to_radians())),
            ..default()
        })
        .insert(Vehicle {
            team: if is_player { Team::Player } else { Team::Ai },
        })
        .insert(GridPosition { x, y })
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(sprite_path),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load("sprites/character.png"),
                            transform: Transform::from_translation(Vec3 {
                                x: -10.0,
                                y: -25.0,
                                z: 1.0,
                            })
                            .with_scale(Vec3 {
                                x: 0.6,
                                y: 0.6,
                                z: 0.6,
                            }),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 1.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 1.0,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Attack {
                            atk_type: AttackType::Multi,
                            dmg: 5,
                        })
                        .insert(Character {
                            team: if is_player { Team::Player } else { Team::Ai },
                            seat: Seat::BackLeft,
                            can_atk: false,
                        });
                    parent
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load("sprites/character.png"),
                            transform: Transform::from_translation(Vec3 {
                                x: 10.0,
                                y: 25.0,
                                z: 1.0,
                            })
                            .with_scale(Vec3 {
                                x: 0.6,
                                y: 0.6,
                                z: 0.6,
                            }),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 0.0,
                                    green: 1.0,
                                    blue: 0.0,
                                    alpha: 1.0,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Attack {
                            dmg: 5,
                            atk_type: AttackType::Single,
                        })
                        .insert(Character {
                            team: if is_player { Team::Player } else { Team::Ai },
                            seat: Seat::Passenger,
                            can_atk: false,
                        });
                    parent
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load("sprites/character.png"),
                            transform: Transform::from_translation(Vec3 {
                                x: 10.0,
                                y: -25.0,
                                z: 1.0,
                            })
                            .with_scale(Vec3 {
                                x: 0.6,
                                y: 0.6,
                                z: 0.6,
                            }),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Attack {
                            dmg: 5,
                            atk_type: AttackType::Cone,
                        })
                        .insert(Character {
                            team: if is_player { Team::Player } else { Team::Ai },
                            seat: Seat::BackRight,
                            can_atk: false,
                        });
                    parent
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load("sprites/character.png"),
                            transform: Transform::from_translation(Vec3 {
                                x: -10.0,
                                y: 25.0,
                                z: 1.0,
                            })
                            .with_scale(Vec3 {
                                x: 0.6,
                                y: 0.6,
                                z: 0.6,
                            }),
                            sprite: Sprite {
                                color: Color::Rgba {
                                    red: 1.0,
                                    green: 0.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Attack {
                            dmg: 5,
                            atk_type: AttackType::Circle,
                        })
                        .insert(Character {
                            team: if is_player { Team::Player } else { Team::Ai },
                            seat: Seat::Driver,
                            can_atk: false,
                        });
                });
        });
}

fn create_road(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let x_offset = 128.0 * 4.0;
    // top row
    for i in 0..9 {
        commands
            .spawn()
            .insert(Name::from("Tile"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    -x_offset + i as f32 * 128.0,
                    128.0,
                    1.0,
                )),
                ..default()
            })
            .insert(GridPosition { x: i, y: 0 })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/road_horiz_top.png"),

                    ..default()
                });
            });
        // overlay
        commands
            .spawn()
            .insert(Name::from("Highlight"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    -x_offset + i as f32 * 128.0,
                    128.0,
                    2.0,
                )),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/overlay_tile.png"),
                    sprite: Sprite {
                        color: Color::Rgba {
                            red: 0.0,
                            green: 0.0,
                            blue: 0.0,
                            alpha: 0.5,
                        },
                        ..default()
                    },
                    ..default()
                });
            });
    }
    // middle row
    for i in 0..9 {
        commands
            .spawn()
            .insert(Name::from("Tile"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    -x_offset + i as f32 * 128.0,
                    0.0,
                    1.0,
                )),
                ..default()
            })
            .insert(GridPosition { x: i, y: 1 })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/road_blank.png"),
                    ..default()
                });
            });
        // overlay
        commands
            .spawn()
            .insert(Name::from("Highlight"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    -x_offset + i as f32 * 128.0,
                    0.0,
                    2.0,
                )),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/overlay_tile.png"),
                    sprite: Sprite {
                        color: Color::Rgba {
                            red: 0.0,
                            green: 0.0,
                            blue: 0.0,
                            alpha: 0.5,
                        },
                        ..default()
                    },
                    ..default()
                });
            });
    }
    // bottom row
    for i in 0..9 {
        commands
            .spawn()
            .insert(Name::from("Tile"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    -x_offset + i as f32 * 128.0,
                    -128.0,
                    1.0,
                )),
                ..default()
            })
            .insert(GridPosition { x: i, y: 2 })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/road_horiz_bot.png"),
                    ..default()
                });
            });
        // overlay
        commands
            .spawn()
            .insert(Name::from("Highlight"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    -x_offset + i as f32 * 128.0,
                    -128.0,
                    2.0,
                )),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/overlay_tile.png"),
                    sprite: Sprite {
                        color: Color::Rgba {
                            red: 0.0,
                            green: 0.0,
                            blue: 0.0,
                            alpha: 0.5,
                        },
                        ..default()
                    },
                    ..default()
                });
            });
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let positions = [(128.0, 128.0)];
    let grid_positions = [(5, 0)];
    let car_sprites = ["sprites/car_yellow_1.png"];

    //player vehicle
    create_vehicle(
        &mut commands,
        &asset_server,
        (128.0, -128.0),
        "sprites/car_black_1.png",
        5,
        3,
        true,
    );
    //ai vehicles
    for i in 0..positions.len() {
        create_vehicle(
            &mut commands,
            &asset_server,
            positions[i],
            car_sprites[i],
            grid_positions[i].0,
            grid_positions[i].1,
            false,
        );
    }

    create_road(&mut commands, &asset_server);
}

fn set_can_attack(
    mut characters: Query<&mut Character>,
    vehicles: Query<(&Vehicle, &GridPosition)>,
    mut can_attack: ResMut<CharactersCanAttack>,
) {
    let up: Vec2 = Vec2::new(0.0, 1.0);
    let down: Vec2 = Vec2::new(0.0, -1.0);
    let left: Vec2 = Vec2::new(-1.0, 0.0);
    let right: Vec2 = Vec2::new(1.0, 0.0);
    if let Some((player_vehicle, player_grid)) =
        vehicles.iter().find(|(v, _g)| v.team == Team::Player)
    {
        // currently only supports 1 of each team
        if let Some((ai_vehicle, ai_grid)) = vehicles.iter().find(|(v, _g)| v.team == Team::Ai) {
            let direction = (Vec2::new(player_grid.x as f32, player_grid.y as f32)
                - Vec2::new(ai_grid.x as f32, ai_grid.y as f32))
            .normalize();
            for mut player_character in characters.iter_mut().filter(|c| c.team == Team::Player) {
                match direction {
                    up => {
                        if player_character.seat == Seat::BackRight
                            || player_character.seat == Seat::Passenger
                        {
                            player_character.can_atk = true;
                        }
                    }
                    down => {
                        if player_character.seat == Seat::BackLeft {
                            player_character.can_atk = true;
                        }
                    }
                    left => {
                        if player_character.seat == Seat::Passenger {
                            player_character.can_atk = true;
                        }
                    }
                    right => {
                        if player_character.seat == Seat::BackRight
                            || player_character.seat == Seat::BackLeft
                        {
                            player_character.can_atk = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
fn do_move(
    mut vehicle_movement_res: ResMut<VehicleMovement>,
    mut vehicles: Query<&mut Transform, With<Vehicle>>,
) {
    match vehicle_movement_res.vehicle {
        Some(entity) => match vehicles.get_mut(entity) {
            Ok(mut transform) => {
                transform.translation += Vec3::new(
                    vehicle_movement_res.movement.x * 128.0,
                    vehicle_movement_res.movement.y * 128.0,
                    0.0
                );
                vehicle_movement_res.vehicle = None
            }
            Err(_) => {}
        },
        None => {},
    }
}

impl Plugin for TBCombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharactersCanAttack>()
            .init_resource::<VehicleMovement>()
            .add_startup_system(setup)
            .add_system(do_move)
            .add_system(set_can_attack);
    }
}
