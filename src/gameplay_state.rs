use crate::{game_camera, platforms, player, ui};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GameplayStatePlugin;

#[derive(Resource)]
pub struct Gravity(f32);

impl Plugin for GameplayStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(-220.0))
            .add_startup_system(initilizate_gameplay_state_system)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(450.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(player::PlayerPlugin)
            .add_plugin(platforms::PlatformsPlugin)
            .add_plugin(ui::UIPlugin)
            .add_plugin(game_camera::GameCameraPlugin);
    }
}

pub fn initilizate_gameplay_state_system(
    mut rapier_config: ResMut<RapierConfiguration>,
    gravity: Res<Gravity>,
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
}
