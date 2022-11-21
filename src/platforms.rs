use crate::{initilizate_window, WindowDimensions};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct PlatformsPlugin;

const PLATFORM_COLOR: &str = "040a27";
const PLATFORM_WIDTH: f32 = 64.0 * 1.875;
const PLATFORM_HEIGHT: f32 = 32.0 * 0.625;

#[derive(PartialEq)]
enum PlatformType {
    Undefined,
    Stationary,
    Moving,
}

#[derive(Component)]
pub struct Platform {
    pub already_collided: bool,
    direction: f32,
    platform_type: PlatformType,
}

#[derive(Resource)]
struct SpawnCount(i8);

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnCount(49))
            .add_startup_system(spawn_initial_platform_system.after(initilizate_window))
            .add_startup_system(spawn_platform_batch.after(spawn_initial_platform_system));
    }
}

fn spawn_initial_platform_system(mut commands: Commands, window: Res<WindowDimensions>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::hex(PLATFORM_COLOR).unwrap(),
                custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -window.height / 4.0, 0.0),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(PLATFORM_WIDTH / 2.0, PLATFORM_HEIGHT / 2.0),
        Platform {
            already_collided: false,
            direction: 1.0,
            platform_type: PlatformType::Undefined,
        },
    ));
}

fn spawn_platform_batch(
    mut commands: Commands,
    window: Res<WindowDimensions>,
    spawn_count: Res<SpawnCount>,
) {
    let left_bound: f32 = -(window.width / 2.0 - PLATFORM_WIDTH);
    let right_bound: f32 = window.width / 2.0 - PLATFORM_WIDTH;
    let spacing: f32 = window.height / 4.2;

    let mut rng = rand::thread_rng();

    for index in 1..(spawn_count.0 - 1) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::hex(PLATFORM_COLOR).unwrap(),
                    custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    rng.gen_range(left_bound..right_bound),
                    -window.height / 4.0 + (spacing * index as f32),
                    0.0,
                ),
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(PLATFORM_WIDTH / 2.0, PLATFORM_HEIGHT / 2.0),
            Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Undefined,
            },
        ));
    }
}
