use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;

use game_window::{GameWindowPlugin, WindowDimensions};
use gameplay_state::{GameplayStatePlugin, GameplayStateSubstates};
use platforms::Platform;
use player::Player;
use ui::UIPlugin;

mod game_camera;
mod game_window;
mod gameplay_state;
mod platforms;
mod player;
mod ui;

const WINDOW_TITLE: &str = "FLOOR FIFTY";
const WINDOW_WIDTH: i16 = 960;
const WINDOW_HEIGHT: i16 = 540;

#[derive(Resource)]
pub struct Gravity(f32);

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(Gravity(-220.0))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: WINDOW_TITLE.to_string(),
                        width: WINDOW_WIDTH as f32,
                        height: WINDOW_HEIGHT as f32,
                        present_mode: PresentMode::Fifo,
                        monitor: MonitorSelection::Index(1),
                        position: WindowPosition::Centered,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(450.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(initilizate_physics_system)
        .add_plugin(GameWindowPlugin)
        .add_plugin(GameplayStatePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}

pub fn initilizate_physics_system(
    mut rapier_config: ResMut<RapierConfiguration>,
    gravity: Res<Gravity>,
) {
    rapier_config.gravity = Vec2::new(0.0, gravity.0);
}
