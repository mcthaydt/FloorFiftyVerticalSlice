use crate::{initilizate_window, WindowDimensions};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct PlatformsPlugin;

const PLATFORM_COLOR: &str = "656c72";
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
        app.insert_resource(SpawnCount(19))
            .add_startup_system(spawn_initial_platform_system.after(initilizate_window))
            .add_startup_system(spawn_platform_batch)
            .add_system(platform_properties_system);
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

    for index in 1..(spawn_count.0 + 1) {
        let mut plat_type_rng = rand::thread_rng();
        let platform = commands
            .spawn((
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
            ))
            .id();

        let plat_type_rng_value = plat_type_rng.gen_range(0..200);
        if plat_type_rng_value % 2 == 0 {
            commands.entity(platform).insert(Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Moving,
            });
        } else if plat_type_rng_value % 2 != 0 {
            commands.entity(platform).insert(Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Stationary,
            });
        }
    }
}

fn platform_properties_system(
    mut platform_query: Query<(&mut Platform, &mut Transform), With<Platform>>,
    time: Res<Time>,
    window: Res<WindowDimensions>,
) {
    let left_bound: f32 = -(window.width / 2.0 - PLATFORM_WIDTH);
    let right_bound: f32 = window.width / 2.0 - PLATFORM_WIDTH;

    for (mut platform_object, mut platform_transform) in platform_query.iter_mut() {
        if platform_object.platform_type == PlatformType::Moving {
            platform_transform.translation.x +=
                125.0 * time.delta_seconds() * platform_object.direction;
            if platform_transform.translation.x > right_bound {
                platform_transform.translation.x = right_bound;
                platform_object.direction = -1.0;
            }
            if platform_transform.translation.x < left_bound {
                platform_transform.translation.x = left_bound;
                platform_object.direction = 1.0;
            }
        }
    }
}
