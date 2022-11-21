use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub struct GameCameraPlugin;

#[derive(Component)]
struct PlayerCamera {
    follow_speed: f32,
}

const BACKGROUND_COLOR: &str = "F8F0E3";

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initilizate_camera_system);
    }
}

fn initilizate_camera_system(mut commands: Commands) {
    // Spawn Camera
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
