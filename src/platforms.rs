use crate::{GameplayStateSubstates, WindowDimensions};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct PlatformsPlugin;

const PLATFORM_COLOR_BASE: &str = "ffffff";
// const PLATFORM_COLOR_BASE: &str = "656c72";
// const PLATFORM_COLOR_BASE_COLLIDED: &str = "c1bdc6";

// const PLATFORM_COLOR_MOVING: &str = "6a725c";
// const PLATFORM_COLOR_MOVING_COLLIDED: &str = "c3c6bd";

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
    platform_moving_speed: f32,
}

#[derive(Component)]
struct PlatformCollisionIndiactor;

#[derive(Resource)]
struct SpawnCount(i8);

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnCount(19))
            .add_state(GameplayStateSubstates::PreGame)
            .add_system_set(
                SystemSet::on_enter(GameplayStateSubstates::PreGame)
                    .with_system(spawn_initial_platform_system)
                    .with_system(spawn_platform_batch),
            )
            .add_system_set(
                SystemSet::on_update(GameplayStateSubstates::DuringGame)
                    .with_system(platform_properties_system),
            );
    }
}

fn spawn_initial_platform_system(
    mut commands: Commands,
    window: Res<WindowDimensions>,
    asset_server: Res<AssetServer>,
) {
    let platform_texture = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(64.0, 32.0),
                    }),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, -window.height / 4.0, 0.0),
                texture: asset_server.load("PlatformTexture2.png"),
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(PLATFORM_WIDTH / 2.0, PLATFORM_HEIGHT / 2.0),
            Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Undefined,
                platform_moving_speed: 0.0,
            },
        ))
        .id();

    let platform_collision_indicator = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::hex(PLATFORM_COLOR_BASE).unwrap(),
                    custom_size: Some(Vec2::new(PLATFORM_WIDTH * 1.1, PLATFORM_HEIGHT / 6.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, -PLATFORM_HEIGHT / 1.5, 0.0),
                ..Default::default()
            },
            PlatformCollisionIndiactor,
        ))
        .id();

    commands
        .entity(platform_texture)
        .push_children(&[platform_collision_indicator]);
}

fn spawn_platform_batch(
    mut commands: Commands,
    window: Res<WindowDimensions>,
    spawn_count: Res<SpawnCount>,
    asset_server: Res<AssetServer>,
) {
    let left_bound: f32 = -(window.width / 2.0 - PLATFORM_WIDTH);
    let right_bound: f32 = window.width / 2.0 - PLATFORM_WIDTH;
    let spacing: f32 = window.height / 4.2;

    let mut rng = rand::thread_rng();

    for index in 1..(spawn_count.0 + 1) {
        let platform_texture = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
                        rect: Some(Rect {
                            min: Vec2::new(0.0, 0.0),
                            max: Vec2::new(64.0, 32.0),
                        }),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        rng.gen_range(left_bound..right_bound),
                        -window.height / 4.0 + (spacing * index as f32),
                        0.0,
                    ),
                    texture: asset_server.load("PlatformTexture2.png"),
                    ..Default::default()
                },
                RigidBody::Fixed,
                Collider::cuboid(PLATFORM_WIDTH / 2.0, PLATFORM_HEIGHT / 2.0),
            ))
            .id();

        let platform_collision_indicator = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::hex(PLATFORM_COLOR_BASE).unwrap(),
                        custom_size: Some(Vec2::new(PLATFORM_WIDTH * 1.1, PLATFORM_HEIGHT / 6.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, -PLATFORM_HEIGHT / 1.5, 0.0),
                    ..Default::default()
                },
                PlatformCollisionIndiactor,
            ))
            .id();

        let _platform = commands
            .entity(platform_texture)
            .push_children(&[platform_collision_indicator])
            .id();

        let plat_type_rng_value = rng.gen_range(0..200);
        let plat_speed_rng_value = rng.gen_range(100.0..200.0);
        if plat_type_rng_value % 2 == 0 {
            commands.entity(platform_texture).insert(Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Moving,
                platform_moving_speed: plat_speed_rng_value,
            });
        } else if plat_type_rng_value % 2 != 0 {
            commands.entity(platform_texture).insert(Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Stationary,
                platform_moving_speed: 0.0,
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
            platform_transform.translation.x += platform_object.platform_moving_speed
                * time.delta_seconds()
                * platform_object.direction;
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
