use bevy::prelude::*;

use crate::{GameplayStateSubstates, Player};

pub struct UIPlugin;

#[derive(Component)]
struct ScoreUI;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayStateSubstates::PreGame)
                .with_system(initilizate_black_bars_system)
                .with_system(initilizate_score_system),
        )
        .add_system_set(
            SystemSet::on_update(GameplayStateSubstates::DuringGame)
                .with_system(update_score_system),
        );
    }
}

fn initilizate_black_bars_system(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .id();

    let top_bar = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .id();

    let bottom_bar = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(90.0),
                    ..default()
                },
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .id();

    commands.entity(root).push_children(&[top_bar]);
    commands.entity(root).push_children(&[bottom_bar]);
}

fn initilizate_score_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("papercut.ttf");
    let score_ui = commands
        .spawn((
            TextBundle::from_section(
                "0.0".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 130.0,
                    color: Color::hex("FFFFFF7F").unwrap(),
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(6.5),
                    left: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            }),
            ScoreUI,
        ))
        .id();

    let score_ui_bg = commands
        .spawn((
            TextBundle::from_section(
                "0.0".to_string(),
                TextStyle {
                    font: font,
                    font_size: 150.0,
                    color: Color::hex("FFFFFF7F").unwrap(),
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(7.5),
                    left: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            }),
            ScoreUI,
        ))
        .id();

    commands.entity(score_ui_bg).push_children(&[score_ui]);
}

fn update_score_system(
    mut text_query: Query<&mut Text, With<ScoreUI>>,
    player_query: Query<&Player, With<Player>>,
) {
    let player = player_query.single();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = player.score.to_string();
    }
}
