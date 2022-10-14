use bevy::prelude::*;

use crate::states::Views;

pub struct DrivingPlugin;

#[derive(Component, Debug)]
struct Vehicle {
    speed: f32,
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
        .insert(Vehicle { speed: 100.0 })
        .insert(PlayerVehicle);
}

fn create_vehicles(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_player_vehicle(&mut commands, &asset_server);
}

fn driving(player_vehicle: Query<&Vehicle, With<PlayerVehicle>>) {
    // let player = player_vehicle.single();
    // println!("{:?}", player);
}

impl Plugin for DrivingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(Views::Driving).with_system(driving))
            .add_system_set(SystemSet::on_enter(Views::Driving).with_system(create_vehicles));
    }
}


