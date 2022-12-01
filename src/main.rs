use bevy::{prelude::*, window::PresentMode};
// use bevy_inspector_egui::WorldInspectorPlugin;

use game_window::{GameWindowPlugin, WindowDimensions};
use gameplay_state::{
    DeathRegionReachedEvent, GameplayStatePlugin, GameplayStateSubstates, TopFloorReachedEvent,
};
use platforms::Platform;
use player::{Player, PLAYER_SIZE};

mod game_camera;
mod game_timer;
mod game_window;
mod gameplay_state;
mod platform_indicators;
mod platforms;
mod player;
mod ui;

const WINDOW_TITLE: &str = "FLOOR FIFTY";
const WINDOW_WIDTH: i16 = 960;
const WINDOW_HEIGHT: i16 = 540;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: WINDOW_TITLE.to_string(),
                        width: WINDOW_WIDTH as f32,
                        height: WINDOW_HEIGHT as f32,
                        present_mode: PresentMode::Fifo,
                        position: WindowPosition::Centered,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(GameWindowPlugin)
        .add_plugin(GameplayStatePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}
