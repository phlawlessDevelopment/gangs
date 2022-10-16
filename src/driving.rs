use crate::{camera::MainCamera, states::Views};
use bevy::prelude::*;
use rand::Rng;

const BASE_SPEED: f32 = 0.5;
const BASE_MAX_SPEED: f32 = 500.0;
const BASE_ROT_SPEED: f32 = 1.0;
pub struct DrivingPlugin;

#[derive(Component, Debug)]
struct Vehicle {
    velocity: Vec2,
    rot_speed: f32,
}

#[derive(Component)]
struct PlayerVehicle;

fn create_player_vehicle(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn()
        .insert(Name::from("Player"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                texture: asset_server.load("sprites/car_yellow_1.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..default()
            });
        })
        .insert(Vehicle {
            rot_speed: BASE_ROT_SPEED,
            velocity: Vec2::ZERO,
        })
        .insert(PlayerVehicle);
}
fn create_ai_vehicles(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        commands
            .spawn()
            .insert(Name::from("Ai"))
            .insert_bundle(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    rng.gen::<f32>() * 500.0,
                    rng.gen::<f32>() * 500.0,
                    1.0,
                ))
                .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("sprites/car_black_1.png"),
                    ..default()
                });
            })
            .insert(Vehicle {
                rot_speed: BASE_ROT_SPEED,
                velocity: Vec2::ZERO,
            });
    }
}

fn create_vehicles(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_player_vehicle(&mut commands, &asset_server);
    create_ai_vehicles(&mut commands, &asset_server);
}

fn poll_input(
    keys: &Res<Input<KeyCode>>,
    camera_t: &mut Transform,
    player_v: &mut Vehicle,
    player_t: &mut Transform,
    delta: f32,
) {
    let mut velocity_this_frame = Vec3::ZERO;
    if keys.pressed(KeyCode::Space) || keys.pressed(KeyCode::W) {
        let forward =
            Vec2::new(player_t.right().y, player_t.right().x * -1.0) * -BASE_SPEED * delta;
        player_v.velocity += forward;
        velocity_this_frame = Vec3::new(player_v.velocity.x, player_v.velocity.y, 0.0);
    } else {
        if player_v.velocity.length() > 0.1 {
            player_v.velocity = player_v
                .velocity
                .clamp_length(0.0, player_v.velocity.length() -BASE_SPEED * delta);
        } else {
            player_v.velocity = Vec2::ZERO;
        }
    }
    if keys.pressed(KeyCode::A) {
        if player_v.velocity.length() > 0.1 {
            player_t.rotate_local_z(player_v.rot_speed * delta);
        }
    }
    if keys.pressed(KeyCode::D) {
        if player_v.velocity.length() > 0.1 {
            player_t.rotate_local_z(-player_v.rot_speed * delta);
        }
    }
    player_v.velocity = player_v.velocity.clamp_length(0.0, BASE_MAX_SPEED);
    player_t.translation += velocity_this_frame;
    camera_t.translation += velocity_this_frame;
}

fn driving(
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&mut Vehicle, &mut Transform), With<PlayerVehicle>>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerVehicle>)>,
    time: Res<Time>,
) {
    let mut player = player.single_mut();
    let mut camera = camera.single_mut();
    poll_input(
        &keys,
        &mut camera,
        &mut player.0,
        &mut player.1,
        time.delta_seconds(),
    );
}

impl Plugin for DrivingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(Views::Driving).with_system(driving))
            .add_system_set(SystemSet::on_enter(Views::Driving).with_system(create_vehicles));
    }
}
