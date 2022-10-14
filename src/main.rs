use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod camera;
mod driving;
mod states;

use crate::{camera::CameraPlugin, driving::DrivingPlugin, states::Views};

fn main() {
    App::new()
        .add_state(Views::Driving)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(DrivingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
