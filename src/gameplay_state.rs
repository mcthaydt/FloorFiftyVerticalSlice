use crate::PLAYER_SIZE;
use crate::{game_camera, game_timer, platforms, player, ui};
use crate::{Platform, Player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GameplayStatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameplayStateSubstates {
    PreGame,
    DuringGame,
    PostGame,
}

#[derive(Resource)]
pub struct Gravity(f32);

pub struct TopFloorReachedEvent;
pub struct DeathRegionReachedEvent;

impl Plugin for GameplayStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TopFloorReachedEvent>()
            .add_event::<DeathRegionReachedEvent>()
            .insert_resource(Gravity(-275.0))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(550.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(player::PlayerPlugin)
            .add_plugin(game_camera::GameCameraPlugin)
            .add_plugin(platforms::PlatformsPlugin)
            .add_plugin(ui::UIPlugin)
            .add_plugin(game_timer::GameTimerPlugin)
            .add_system_set(
                SystemSet::on_enter(GameplayStateSubstates::PreGame)
                    .with_system(initilizate_physics_system),
            )
            .add_system_set(
                SystemSet::on_update(GameplayStateSubstates::PreGame)
                    .with_system(switch_gameplay_substates_system),
            )
            .add_system_set(
                SystemSet::on_update(GameplayStateSubstates::DuringGame)
                    .with_system(game_completion_system),
            );
    }
}

// Initialize the physics system with the specified gravity value
pub fn initilizate_physics_system(
    mut rapier_config: ResMut<RapierConfiguration>,
    gravity: Res<Gravity>,
) {
    rapier_config.gravity = Vec2::new(0.0, gravity.0);
}

// Switch to the "DuringGame" gameplay substate
fn switch_gameplay_substates_system(mut gameplay_substate: ResMut<State<GameplayStateSubstates>>) {
    gameplay_substate
        .set(GameplayStateSubstates::DuringGame)
        .unwrap();
}

// Reset the game when the top floor is reached or the death region is reached
fn game_completion_system(
    mut player_query: Query<((&mut Player, &mut Transform), With<Player>)>,
    mut platform_query: Query<&mut Platform, With<Platform>>,
    mut ev_game_completed: EventReader<TopFloorReachedEvent>,
    mut ev_game_failed: EventReader<DeathRegionReachedEvent>,
) {
    let (mut player_object, mut _player_transform) = player_query.single_mut();

    // Helper function to reset the game
    let mut reset_game = || {
        player_object.1.translation = Vec3::new(0.0, -PLAYER_SIZE * 2.0, 0.0);
        player_object.0.score = 0;

        for mut platform_object in platform_query.iter_mut() {
            if platform_object.already_collided {
                platform_object.already_collided = false;
            }
        }
    };

    // Reset the game when the top floor is reached
    for _ev in ev_game_completed.iter() {
        reset_game();
    }

    // Reset the game when the death region is reached
    for _ev in ev_game_failed.iter() {
        reset_game();
    }
}
