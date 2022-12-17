use crate::{platform_indicators::PlatformIndicator, GameplayStateSubstates, WindowDimensions};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct PlatformsPlugin;

pub const PLATFORM_WIDTH: f32 = 64.0 * 1.875;
pub const PLATFORM_HEIGHT: f32 = 32.0 * 0.625;

#[derive(PartialEq, Eq)]
pub enum PlatformType {
    Undefined,
    Stationary,
    Moving,
}

#[derive(Component)]
pub struct Platform {
    pub already_collided: bool,
    pub platform_type: PlatformType,
    direction: f32,
    platform_moving_speed: f32,
}

#[derive(Resource)]
pub struct SpawnCount(pub i8);

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnCount(19))
            .add_plugin(PlatformIndicator)
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
    commands.spawn((
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
    ));
}

// Spawns a batch of platforms in the game
fn spawn_platform_batch(
    mut commands: Commands,
    window: Res<WindowDimensions>,
    spawn_count: Res<SpawnCount>,
    asset_server: Res<AssetServer>,
) {
    // Calculate the left and right bounds for the platforms
    let left_bound: f32 = -(window.width / 2.0 - PLATFORM_WIDTH);
    let right_bound: f32 = window.width / 2.0 - PLATFORM_WIDTH;

    // Calculate the spacing between the platforms
    let spacing: f32 = window.height / 4.2;

    // Initialize a random number generator
    let mut rng = rand::thread_rng();

    // Iterate through the number of platforms to be spawned
    for index in 1..(spawn_count.0 + 1) {
        // Spawn the platform sprite with a random position within the bounds and specified texture
        let platform = commands
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
                // Add a fixed rigid body component
                RigidBody::Fixed,
                // Add a cuboid collider component with specified dimensions
                Collider::cuboid(PLATFORM_WIDTH / 2.0, PLATFORM_HEIGHT / 2.0),
            ))
            .id();

        // Generate random values for the platform type and speed
        let plat_type_rng_value = rng.gen_range(0..200);
        let plat_speed_rng_value = rng.gen_range(100.0..200.0);

        // Insert the Platform component based on the random values
        if plat_type_rng_value % 2 == 0 {
            commands.entity(platform).insert(Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Moving,
                platform_moving_speed: plat_speed_rng_value,
            });
        } else if plat_type_rng_value % 2 != 0 {
            commands.entity(platform).insert(Platform {
                already_collided: false,
                direction: 1.0,
                platform_type: PlatformType::Stationary,
                platform_moving_speed: 0.0,
            });
        }
    }
}

// Modifies the properties of moving platforms in the game
fn platform_properties_system(
    mut platform_query: Query<(&mut Platform, &mut Transform), With<Platform>>,
    time: Res<Time>,
    window: Res<WindowDimensions>,
) {
    // Calculate the left and right bounds for the platforms
    let left_bound: f32 = -(window.width / 2.0 - PLATFORM_WIDTH);
    let right_bound: f32 = window.width / 2.0 - PLATFORM_WIDTH;

    // Iterate through the platforms in the query
    for (mut platform_object, mut platform_transform) in platform_query.iter_mut() {
        // Check if the platform is a moving platform
        if platform_object.platform_type == PlatformType::Moving {
            // Update the position of the platform based on its speed and direction
            platform_transform.translation.x += platform_object.platform_moving_speed
                * time.delta_seconds()
                * platform_object.direction;

            // Check if the platform has reached the left or right bounds
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
