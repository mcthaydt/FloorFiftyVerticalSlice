use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio};
use std::time::Duration;

pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin).add_startup_system(play_music);
    }
}

fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("Track4.wav"))
        .fade_in(AudioTween::new(
            Duration::from_secs(0),
            AudioEasing::OutPowi(2),
        ))
        .with_volume(0.95)
        .looped();
}
