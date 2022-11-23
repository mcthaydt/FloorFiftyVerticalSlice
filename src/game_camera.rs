use crate::{GameplayStateSubstates, Player};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub struct GameCameraPlugin;

#[derive(Component)]
struct PlayerCamera {
    follow_speed: f32,
}

const BACKGROUND_COLOR: &str = "c0dffa";

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayStateSubstates::PreGame)
                .with_system(initilizate_camera_system),
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
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::hex(BACKGROUND_COLOR).unwrap()),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
        PlayerCamera { follow_speed: 5.0 },
    ));
}

fn follow_player_system(
    mut camera_query: Query<((&mut Transform, &PlayerCamera), With<PlayerCamera>)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    time: Res<Time>,
) {
    let (mut camera, _camera_object) = camera_query.single_mut();
    let player = player_query.single();
    let follow_pos: Vec3 = Vec3::new(0.0, player.translation.y, 1.0);
    camera.0.translation = camera
        .0
        .translation
        .lerp(follow_pos, time.delta_seconds() * camera.1.follow_speed);
}
