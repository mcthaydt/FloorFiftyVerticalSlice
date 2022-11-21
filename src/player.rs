use crate::Platform;
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

pub struct PlayerPlugin;

const PLAYER_COLOR: &str = "2A75BE";
const PLAYER_SIZE: f32 = 32.0 * 1.56;

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
    jump_force: f32,
    player_colliding: bool,
    player_grounded: bool,
    player_facing_right: bool,
}

#[derive(Component)]
struct PlayerGroundDetection;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system)
            .add_system(player_input_system)
            .add_system_to_stage(CoreStage::PostUpdate, player_collision_detection_system);
    }
}

fn spawn_player_system(mut commands: Commands) {
    let player = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                    color: Color::hex(PLAYER_COLOR).unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::ball(PLAYER_SIZE / 2.2),
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            Player {
                movement_speed: 300.0,
                jump_force: 200.0,
                player_colliding: false,
                player_grounded: false,
                player_facing_right: true,
            },
        ))
        .id();

    let player_ground_detection = commands
        .spawn((
            Sensor,
            Collider::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 6.0),
            ActiveEvents::COLLISION_EVENTS,
            TransformBundle {
                local: Transform::from_xyz(0.0, -PLAYER_SIZE / 2.0 - PLAYER_SIZE / 6.0, 0.0),
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
    mut player_query: Query<((&mut Player, &mut Velocity, &mut Transform), With<Player>)>,
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

    if player.0.player_colliding == true {
        player.1.linvel.y = player.0.jump_force;
    }

    let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
    if down {
        player.1.linvel.y = -player.0.jump_force * 3.0;
    }

    let respawn = keyboard_input.just_pressed(KeyCode::R);
    if respawn == true {
        player.2.translation = Vec3::splat(0.0);
    }
}

fn player_collision_detection_system(
    mut collision_events: EventReader<CollisionEvent>,
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
                platform_object.already_collided = true;
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
