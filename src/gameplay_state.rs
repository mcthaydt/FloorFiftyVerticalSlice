use crate::{game_camera, platforms, player, ui};
use bevy::prelude::*;

pub struct GameplayStatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameplayStateSubstates {
    PreGame,
    DuringGame,
    PostGame,
}

impl Plugin for GameplayStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameplayStateSubstates::PreGame)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(game_camera::GameCameraPlugin)
            .add_plugin(platforms::PlatformsPlugin)
            .add_plugin(ui::UIPlugin);
    }
}
