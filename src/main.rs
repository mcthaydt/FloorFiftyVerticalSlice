use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;

use game_window::{initilizate_window, GameWindowPlugin, WindowDimensions};
use game_world::GameWorldPlugin;
use platforms::{Platform, PlatformsPlugin};
use player::{spawn_player_system, Player, PlayerPlugin};

mod game_camera;
mod game_window;
mod game_world;
mod platforms;
mod player;

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
                        monitor: MonitorSelection::Index(1),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(GameWindowPlugin)
        .add_plugin(GameWorldPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
