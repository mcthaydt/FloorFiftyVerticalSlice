use crate::{GameplayStateSubstates, Player};
use bevy::prelude::*;

pub struct UIPlugin;

#[derive(Component)]
struct ScoreUI;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayStateSubstates::PreGame)
                .with_system(spawn_black_bars_system)
                .with_system(spawn_score_ui_system),
        )
        .add_system_set(
            SystemSet::on_update(GameplayStateSubstates::DuringGame)
                .with_system(update_score_ui_system),
        );
    }
}

fn spawn_black_bars_system(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // This function spawns two black bars as children of a root node in the UI.
    // The bars are positioned at the top and bottom of the screen and take up 10% of the screen height each.
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

    // The root node is a container for the top and bottom bars.
    // The top and bottom bars are used to create a letterbox effect around the main content of the UI.
    commands.entity(root).push_children(&[top_bar]);
    commands.entity(root).push_children(&[bottom_bar]);
}

fn spawn_score_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // This code loads a font from the `asset_server` and then uses it to create a text entity in the UI.
    // The text has a font size of 130 and is semi-transparent white.
    // The text is positioned at the top-left corner of the screen, with a small offset from the top and left edges.
    let font = asset_server.load("papercut.ttf");

    // The text entity displays the player's score in the UI.
    // It is positioned at the top-left corner of the screen so that it is always visible as the player's score increases.
    commands.spawn((
        TextBundle::from_section(
            "".to_string(),
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
    ));
}

fn update_score_ui_system(
    mut text_query: Query<&mut Text, With<ScoreUI>>,
    player_query: Query<&Player, With<Player>>,
) {
    // This function updates the text of a text entity in the UI with the player's current score.
    // The text entity is identified using the `ScoreUI` component.
    
    let player = player_query.single();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = player.score.to_string();
    }
}
