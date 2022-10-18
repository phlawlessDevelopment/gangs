use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod camera;
mod driving;
mod states;
mod tb_combat;

use crate::{
    camera::CameraPlugin, driving::DrivingPlugin, states::Views, tb_combat::gui::GuiPlugin,
    tb_combat::TBCombatPlugin,
};

fn main() {
    App::new()
        .add_state(Views::TbCombat)
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(TBCombatPlugin)
        // .add_plugin(DrivingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
