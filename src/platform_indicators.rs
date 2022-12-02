use crate::player::Player;
use crate::Platform;
use crate::{platforms::PLATFORM_HEIGHT, platforms::PLATFORM_WIDTH, GameplayStateSubstates};
use bevy::prelude::*;

pub struct PlatformIndicator;

#[derive(Component)]
struct PlatformCollisionIndicator;

impl Plugin for PlatformIndicator {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_exit(GameplayStateSubstates::PreGame)
                .with_system(spawn_platform_collision_indicators),
        )
        .add_system_set(
            SystemSet::on_update(GameplayStateSubstates::DuringGame)
                .with_system(update_indicator_position)
                .with_system(update_indicator_color),
        )
        .add_system_set(
            SystemSet::on_exit(GameplayStateSubstates::PostGame)
                .with_system(despawn_platform_collision_indicators),
        );
    }
}

fn spawn_platform_collision_indicators(
    mut commands: Commands,
    platform_query: Query<&Transform, With<Platform>>,
) {
    for platfroms in platform_query.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("ffffff").unwrap(),
                    custom_size: Some(Vec2::new(PLATFORM_WIDTH * 1.1, PLATFORM_HEIGHT / 6.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    platfroms.translation.x,
                    platfroms.translation.y + -PLATFORM_HEIGHT / 1.5,
                    1.0,
                ),
                ..Default::default()
            },
            PlatformCollisionIndicator,
        ));
    }
}

fn update_indicator_position(
    platform_query: Query<&Transform, With<Platform>>,
    mut platform_collision_query: Query<
        &mut Transform,
        (With<PlatformCollisionIndicator>, Without<Platform>),
    >,
) {
    let mut platform_positions_vector = Vec::new();

    for platform_transform in platform_query.iter() {
        platform_positions_vector.push(platform_transform.translation)
    }

    let mut index = 0;
    for mut platform_collision_transform in platform_collision_query.iter_mut() {
        index += 1;
        platform_collision_transform.translation = Vec3::new(
            platform_positions_vector[index - 1].x,
            platform_positions_vector[index - 1].y + -PLATFORM_HEIGHT / 1.5,
            1.0,
        );
    }
}

fn update_indicator_color(
    platform_query: Query<&Platform, (With<Platform>, Without<Player>)>,
    mut platform_collision_query: Query<
        &mut Sprite,
        (
            With<PlatformCollisionIndicator>,
            Without<Platform>,
            Without<Player>,
        ),
    >,
) {
    let mut platform_color_vector = Vec::new();

    for platforms in platform_query.iter() {
        platform_color_vector.push(platforms);
    }

    let mut index = 0;
    for mut platform_collision_sprites in platform_collision_query.iter_mut() {
        index += 1;
        if platform_color_vector[index - 1].already_collided  {
            platform_collision_sprites.color = Color::GREEN;
        } else {
            platform_collision_sprites.color = Color::ORANGE_RED;
        }
    }
}

fn despawn_platform_collision_indicators(
    mut commands: Commands,
    indicator_query: Query<Entity, With<PlatformCollisionIndicator>>,
) {
    for entities in indicator_query.iter() {
        commands.entity(entities).despawn_recursive();
    }
}
// spawn platform collsiion indicators
// update colision indicators
// // update platform collsiion indicator position
// // update platform collision indicator color
// delete collision indicators
