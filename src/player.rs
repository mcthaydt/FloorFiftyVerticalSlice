use crate::GameplayStateSubstates;
use crate::{DeathRegionReachedEvent, TopFloorReachedEvent};
use crate::{Platform, WindowDimensions};

use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

pub struct PlayerPlugin;

pub const PLAYER_SIZE: f32 = 32.0 * 1.56;

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
    jump_force: f32,
    pub player_colliding: bool,
    pub player_grounded: bool,
    player_facing_right: bool,
    pub score: i8,
}

#[derive(Component)]
struct PlayerGroundDetection;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TopFloorReachedEvent>()
            .add_event::<DeathRegionReachedEvent>()
            .add_system_set(
                SystemSet::on_enter(GameplayStateSubstates::PreGame)
                    .with_system(spawn_player_system),
            )
            .add_system_set(
                SystemSet::on_update(GameplayStateSubstates::DuringGame)
                    .with_system(player_input_system)
                    .with_system(player_screen_looping_system)
                    .with_system(player_animation_system)
                    .with_system(player_collision_detection_system),
            );
    }
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                    ..Default::default()
                },
                texture: asset_server.load("PlayerTexture.png"),
                transform: Transform::from_xyz(0.0, -PLAYER_SIZE * 2.0, 0.0),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::ball(PLAYER_SIZE / 1.7),
            ColliderMassProperties::Mass(3.85),
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            (ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_KINEMATIC),
            Player {
                movement_speed: 356.0,
                jump_force: 268.2,
                player_colliding: false,
                player_grounded: false,
                player_facing_right: true,
                score: 0,
            },
        ))
        .id();

    let player_ground_detection = commands
        .spawn((
            Sensor,
            Collider::cuboid(PLAYER_SIZE / 3.0, PLAYER_SIZE / 9.0),
            ActiveEvents::COLLISION_EVENTS,
            (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC),
            TransformBundle {
                local: Transform::from_xyz(0.0, -PLAYER_SIZE / 2.0 - PLAYER_SIZE / 9.0, 0.0),
                ..Default::default()
            },
            PlayerGroundDetection,
        ))
        .id();

    commands
        .entity(player)
        .push_children(&[player_ground_detection]);
}

fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        (&mut Player, &mut Velocity, &mut Transform, &GlobalTransform),
        With<Player>,
    )>,
    mut failure_event: EventWriter<DeathRegionReachedEvent>,
) {
    let (mut player, _player_velocity) = player_query.single_mut();

    let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
    let x_input = -(left as i8) + right as i8;

    if right {
        player.0.player_facing_right = true;
    }
    if left {
        player.0.player_facing_right = false;
    }

    let mut player_input_dir = Vec2::new(x_input as f32, 0.0);
    if player_input_dir != Vec2::ZERO {
        player_input_dir /= player_input_dir.length();
    }

    player.1.linvel.x = player_input_dir.x * player.0.movement_speed;

    if player.0.player_colliding {
        player.1.linvel.y = player.0.jump_force;
    }

    let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
    if down {
        player.1.linvel.y = -player.0.jump_force * 3.5;
    }

    let respawn = keyboard_input.just_pressed(KeyCode::R);
    if respawn {
        failure_event.send(DeathRegionReachedEvent);
    }

    if player.3.translation().y < -400.0 {
        failure_event.send(DeathRegionReachedEvent);
    }
}

fn player_collision_detection_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut top_floor_reached_event: EventWriter<TopFloorReachedEvent>,
    player_ground_detection_query: Query<(
        (Entity, &mut PlayerGroundDetection),
        With<PlayerGroundDetection>,
    )>,
    mut player_query: Query<((Entity, &mut Player), With<Player>)>,
    mut platform_query: Query<(Entity, &mut Platform), With<Platform>>,
) {
    let (mut player_entity, _player_object) = player_query.single_mut();
    let (player_ground_detection_entity, _player_ground_detection_object) =
        player_ground_detection_query.single();

    let mut total_count = 0;

    for _index in platform_query.iter() {
        total_count += 1;
    }

    if player_entity.1.score == total_count {
        top_floor_reached_event.send(TopFloorReachedEvent);
    }

    for collision_event in collision_events.iter() {
        for (platform_entity, mut platform_object) in platform_query.iter_mut() {
            if *collision_event
                == CollisionEvent::Started(
                    player_entity.0,
                    platform_entity,
                    CollisionEventFlags::from_bits(0).unwrap(),
                )
            {
                player_entity.1.player_colliding = true;
            } else if *collision_event
                == CollisionEvent::Stopped(
                    player_entity.0,
                    platform_entity,
                    CollisionEventFlags::from_bits(0).unwrap(),
                )
            {
                player_entity.1.player_colliding = false;
            }

            if *collision_event
                == CollisionEvent::Started(
                    player_ground_detection_entity.0,
                    platform_entity,
                    CollisionEventFlags::from_bits(1).unwrap(),
                )
            {
                player_entity.1.player_grounded = true;
                if !platform_object.already_collided {
                    player_entity.1.score += 1;
                    platform_object.already_collided = true;
                }
            } else if *collision_event
                == CollisionEvent::Stopped(
                    player_ground_detection_entity.0,
                    platform_entity,
                    CollisionEventFlags::from_bits(1).unwrap(),
                )
            {
                player_entity.1.player_grounded = false;
            }
        }
    }
}

fn player_screen_looping_system(
    mut player_query: Query<((&mut Transform, &Player), With<Player>)>,
    window_dimensions: Res<WindowDimensions>,
) {
    let (mut player_transform, _player_object) = player_query.single_mut();

    if player_transform.0.translation.x > window_dimensions.width / 2.0 + PLAYER_SIZE / 2.0_f32 {
        player_transform.0.translation.x = -(window_dimensions.width / 2.0) + PLAYER_SIZE / 2.0;
    } else if player_transform.0.translation.x < -(window_dimensions.width / 2.0) {
        player_transform.0.translation.x = window_dimensions.width / 2.0 + PLAYER_SIZE / 2.0_f32;
    }
}

fn player_animation_system(mut player_query: Query<((&mut Sprite, &Player), With<Player>)>) {
    let (mut player_sprite, _player_object) = player_query.single_mut();

    if player_sprite.1.player_facing_right {
        player_sprite.0.flip_x = false;
    } else {
        player_sprite.0.flip_x = true;
    }
}
