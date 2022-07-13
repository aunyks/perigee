use bevy::log::Level;
use bevy::log::LogSettings;
use bevy::prelude::*;

use bevy_fly_camera::FlyCameraPlugin;
use perigee_single_player::level_0::Sim;
use plugins::levels::*;
use plugins::FirstPersonControlPlugin;
use states::{FirstPersonControlSettings, GameLevel};

mod components;
mod plugins;
mod states;
mod systems;

fn main() {
    App::new()
        // Configure log plugin (added by DefaultPlugins)
        .insert_resource(LogSettings {
            level: Level::ERROR,
            filter: String::from("bevy_interace=trace"),
        })
        .insert_resource(WindowDescriptor {
            title: String::from("Bevy Simulation Debugging"),
            width: 800.,
            height: 700.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        // Enable First Person controls
        .add_state(FirstPersonControlSettings::Enabled)
        .add_plugin(FirstPersonControlPlugin)
        .insert_resource(Sim::new())
        .add_state(GameLevel::Zero)
        .add_plugin(GameLevel0)
        .add_plugin(PauseMenuLevel)
        .run();
}
