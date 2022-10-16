use bevy::prelude::*;
pub struct TBCombatPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Character;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let positions = [(150.0, -150.0), (150.0, 150.0)];
    let car_sprites = ["sprites/car_black_1.png", "sprites/car_yellow_1.png"];
    for i in 0..positions.len() {
        commands
            .spawn()
            .insert(Name::from("Car"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    positions[i].0,
                    positions[i].1,
                    1.0,
                ))
                .with_rotation(Quat::from_rotation_z((90.0 as f32).to_radians())),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load(car_sprites[i]),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(SpriteBundle {
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
                        });
                        parent.spawn_bundle(SpriteBundle {
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
                        });
                        parent.spawn_bundle(SpriteBundle {
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
                        });
                        parent.spawn_bundle(SpriteBundle {
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
                        });
                    });
            });
    }
}

impl Plugin for TBCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
