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
    // This code plays a sound from the `asset_server` with a smooth fade-in effect and loops it.
    // The sound is played at 95% volume.
    audio
        .play(asset_server.load("Track4.wav"))
        .fade_in(AudioTween::new(
            Duration::from_secs(0),
            AudioEasing::OutPowi(0),
        ))
        .with_volume(0.95)
        .looped();
}
