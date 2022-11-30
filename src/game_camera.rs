use crate::{game_window::WindowDimensions, GameplayStateSubstates, Player};
use bevy::{
    core_pipeline::{bloom::BloomSettings, clear_color::ClearColorConfig},
    prelude::*,
};

pub struct GameCameraPlugin;

#[derive(Component)]
struct PlayerCamera {
    follow_speed: f32,
}

#[derive(Component)]
struct Background;

const BACKGROUND_COLOR: &str = "c0dffa";

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayStateSubstates::PreGame)
                .with_system(initilizate_camera_system)
                .with_system(initilizate_background_system),
        )
        .add_system_set(
            SystemSet::on_update(GameplayStateSubstates::DuringGame)
                .with_system(follow_player_system),
        );
    }
}

fn initilizate_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::hex(BACKGROUND_COLOR).unwrap()),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
        BloomSettings {
            threshold: 0.68,
            intensity: 3.05,
            ..default()
        },
        PlayerCamera { follow_speed: 5.0 },
    ));
}

fn initilizate_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<WindowDimensions>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.width * 1.1, window.height * 1.1)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            texture: asset_server.load("BackgroundTexture2.png"),
            ..Default::default()
        },
        Background,
    ));
}

fn follow_player_system(
    mut camera_query: Query<((&mut Transform, &PlayerCamera), With<PlayerCamera>)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut background_query: Query<
        &mut Transform,
        (With<Background>, Without<PlayerCamera>, Without<Player>),
    >,
    time: Res<Time>,
) {
    let (mut camera, _camera_object) = camera_query.single_mut();
    let player = player_query.single();
    let mut background = background_query.single_mut();

    let follow_pos: Vec3 = Vec3::new(0.0, player.translation.y, 1.0);
    camera.0.translation = camera
        .0
        .translation
        .lerp(follow_pos, time.delta_seconds() * camera.1.follow_speed);
    background.translation = Vec3::new(0.0, camera.0.translation.y, -1.0);
}
