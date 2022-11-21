use bevy::prelude::*;

use crate::Player;

pub struct UIPlugin;

#[derive(Component)]
struct ScoreUI;

const TEXT_COLOR: &str = "0e1312";

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initilizate_score_system)
            .add_system(update_score_system);
    }
}

fn initilizate_score_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("papercut.ttf");
    commands.spawn((
        TextBundle::from_section(
            "0.0".to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 80.0,
                color: Color::hex(TEXT_COLOR).unwrap(),
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(15.0),
                left: Val::Px(30.0),
                ..default()
            },
            ..default()
        }),
        ScoreUI,
    ));
}

fn update_score_system(
    mut text_query: Query<&mut Text, With<ScoreUI>>,
    player_query: Query<&Player, With<Player>>,
) {
    let mut text = text_query.single_mut();
    let player = player_query.single();
    text.sections[0].value = player.score.to_string();
}
