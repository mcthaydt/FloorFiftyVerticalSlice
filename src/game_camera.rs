use crate::{window_manager::WindowDimensions, GameplayStateSubstates, Player};
use bevy::time::Stopwatch;
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_rapier2d::prelude::*;
use lerp::Lerp;

pub struct GameCameraPlugin;

#[derive(Component)]
struct PlayerCamera {
    follow_speed: f32,
    zoom_speed: f32,
    stationary_zoom_amt: f32,
    motion_zoom_amt: f32,
}

#[derive(Component)]
struct Background;

#[derive(Resource)]
struct CameraStopwatch(Stopwatch);

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraStopwatch(Stopwatch::new()))
            .add_system_set(
                SystemSet::on_enter(GameplayStateSubstates::PreGame)
                    .with_system(spawn_camera_system)
                    .with_system(spawn_background_system),
            )
            .add_system_set(
                SystemSet::on_update(GameplayStateSubstates::DuringGame)
                    .with_system(follow_player_system)
                    .with_system(camera_zoom_system),
            );
    }
}

fn spawn_camera_system(mut commands: Commands) {
    // This code spawns a 2D camera entity with a bloom effect and a `PlayerCamera` component.
    // The camera has an orthographic projection and is positioned at (0, 0, 1).
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                scale: 0.8,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        BloomSettings {
            threshold: 0.68,
            intensity: 3.0,
            ..default()
        },
        PlayerCamera {
            follow_speed: 5.0,
            zoom_speed: 1.5,
            stationary_zoom_amt: 0.8,
            motion_zoom_amt: 1.1,
        },
    ));
}

fn spawn_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<WindowDimensions>,
) {
    // This code spawns a sprite entity with a `Background` component and a texture from the `asset_server`.
    // The sprite is positioned at (0, 0, -1) and has a custom size that is slightly larger than the window size.
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
    mut background_query: Query<
        &mut Transform,
        (With<Background>, Without<PlayerCamera>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    time: Res<Time>,
) {
    // This code updates the position of a camera entity and a background sprite entity.
    // The camera follows the player's vertical position, and the background sprite is positioned at the same vertical position as the camera.
    
    let (mut camera, _camera_object) = camera_query.single_mut();
    let mut background = background_query.single_mut();
    let player = player_query.single();

    let follow_pos: Vec3 = Vec3::new(0.0, player.translation.y, 1.0);
    camera.0.translation = camera
        .0
        .translation
        .lerp(follow_pos, time.delta_seconds() * camera.1.follow_speed);

    background.translation = Vec3::new(0.0, camera.0.translation.y, -1.0);
}

fn camera_zoom_system(
    mut camera_query: Query<(&mut OrthographicProjection, &PlayerCamera), With<PlayerCamera>>,
    player_query: Query<&Velocity, With<Player>>,
    mut game_stopwatch: ResMut<CameraStopwatch>,
    time: Res<Time>,
) {
    // This code updates the scale of a camera's projection (zoom) based on the player's movement and a timer.
    // If the player is stationary for a certain amount of time, the camera zooms in.
    // Otherwise, the camera zooms out.

    let (mut camera_proj, camera_obj) = camera_query.single_mut();
    let player_vel = player_query.single();

    if player_vel.linvel.x.abs() < 10.0 {
        game_stopwatch.0.tick(time.delta());
    } else {
        game_stopwatch.0.reset();
    }

    let target_scale;
    if game_stopwatch.0.elapsed_secs() > 0.45 {
        target_scale = camera_obj.stationary_zoom_amt;
    } else {
        target_scale = camera_obj.motion_zoom_amt;
    }

    camera_proj.scale = camera_proj
        .scale
        .lerp(target_scale, time.delta_seconds() * camera_obj.zoom_speed);
}
