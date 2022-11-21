use crate::{game_camera, platforms, player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GameWorldPlugin;

#[derive(Resource)]
struct Gravity(f32);

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(-220.0))
            .add_startup_system(initilizate_game_world_system)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(450.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(player::PlayerPlugin)
            .add_plugin(platforms::PlatformsPlugin)
            .add_plugin(game_camera::GameCameraPlugin);
    }
}

fn initilizate_game_world_system(
    mut _commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    gravity: Res<Gravity>,
    _asset_server: Res<AssetServer>,
) {
    // Init. World Settings
    rapier_config.gravity = Vec2::new(0.0, gravity.0);

    // // Spawn UI Text
    // let font = asset_server.load("Vogue.ttf");
    // commands
    //     .spawn_bundle(
    //         TextBundle::from_section(
    //             "0.0".to_string(),
    //             TextStyle {
    //                 font: font.clone(),
    //                 font_size: 50.0,
    //                 color: Color::hex("1b1b1b").unwrap(),
    //             },
    //         )
    //         .with_style(Style {
    //             align_self: AlignSelf::FlexEnd,
    //             position_type: PositionType::Absolute,
    //             position: UiRect {
    //                 top: Val::Px(15.0),
    //                 left: Val::Px(25.0),
    //                 ..default()
    //             },
    //             ..default()
    //         }),
    //     )
    //     .insert(ScoreUI);

    // // Spawn Initial Platform
    // commands
    //     .spawn()
    //     .insert_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::hex(PLATFORM_COLOR).unwrap(),
    //             custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
    //             ..Default::default()
    //         },
    //         transform: Transform::from_xyz(0.0, -(WINDOW_HEIGHT as f32) / 4.0, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(RigidBody::Fixed)
    //     .insert(Collider::cuboid(
    //         PLATFORM_WIDTH / 2.0,
    //         PLATFORM_HEIGHT / 2.0,
    //     ))
    //     .insert(Platform {
    //         already_collided: false,
    //         direction: 0.0,
    //         platform_type: PlatformType::Stationary,
    //     });

    // // Spawn Additional Platforms
    // let mut rng = rand::thread_rng();
    // for index in 1..50 {
    //     commands
    //         .spawn()
    //         .insert_bundle(SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::hex(PLATFORM_COLOR).unwrap(),
    //                 custom_size: Some(Vec2::new(PLATFORM_WIDTH, PLATFORM_HEIGHT)),
    //                 ..default()
    //             },
    //             // texture: asset_server.load("PlatformTexture.png"),
    //             transform: Transform::from_xyz(
    //                 rng.gen_range(
    //                     -(WINDOW_WIDTH as f32 / 2.0 - PLATFORM_WIDTH as f32)
    //                         ..(WINDOW_WIDTH as f32 / 2.0 - PLATFORM_WIDTH as f32),
    //                 ),
    //                 -(WINDOW_HEIGHT as f32 / 4.0) + (WINDOW_HEIGHT as f32) / 4.2 * index as f32,
    //                 0.0,
    //             ),
    //             ..Default::default()
    //         })
    //         .insert(RigidBody::Fixed)
    //         .insert(Collider::cuboid(
    //             PLATFORM_WIDTH / 2.0,
    //             PLATFORM_HEIGHT / 2.0,
    //         ))
    //         .insert(Platform {
    //             already_collided: false,
    //             direction: 1.0,
    //             platform_type: PlatformType::Undefined,
    //         });
    // }
}
