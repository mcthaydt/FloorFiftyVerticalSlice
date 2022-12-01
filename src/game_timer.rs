use crate::player::Player;
use crate::GameplayStateSubstates;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct GameTimerPlugin;

#[derive(Component)]
struct GameTimerUI;

#[derive(Resource)]
struct GameTimerUIOffset(Vec3);

#[derive(Resource)]
struct GameStopwatch(Stopwatch);

#[derive(Resource)]
struct CurrentGameTime(f32);

#[derive(Resource)]
pub struct FinalGameTime(f32);

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameStopwatch(Stopwatch::new()))
            .insert_resource(GameTimerUIOffset(Vec3::new(
                960.0 / 15.0,
                540.0 / 15.0,
                1.0,
            )))
            .insert_resource(CurrentGameTime(0.0))
            .insert_resource(FinalGameTime(0.0))
            .add_system_set(
                SystemSet::on_enter(GameplayStateSubstates::PreGame)
                    .with_system(show_timer_ui_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameplayStateSubstates::PreGame).with_system(start_timer_system),
            )
            .add_system_set(
                SystemSet::on_update(GameplayStateSubstates::DuringGame)
                    .with_system(update_timer_system),
            )
            .add_system_set(
                SystemSet::on_enter(GameplayStateSubstates::PostGame)
                    .with_system(reset_and_save_timer_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameplayStateSubstates::PostGame)
                    .with_system(hide_timer_ui_system),
            );
    }
}

fn show_timer_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_stopwatch: ResMut<GameStopwatch>,
    game_timer_ui_offset: Res<GameTimerUIOffset>,
) {
    game_stopwatch.0.pause();

    let font = asset_server.load("papercut.ttf");
    let text_style = TextStyle {
        font,
        font_size: 75.0,
        // f07927
        color: Color::Hsla {
            hue: 0.07,
            saturation: 0.87,
            lightness: 0.04,
            alpha: 0.85,
        },
    };
    let text_alignment = TextAlignment::CENTER_LEFT;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                game_stopwatch.0.elapsed_secs().to_string(),
                text_style.clone(),
            )
            .with_alignment(text_alignment),
            transform: Transform::from_xyz(
                game_timer_ui_offset.0.x,
                game_timer_ui_offset.0.y,
                game_timer_ui_offset.0.z,
            ),
            ..default()
        },
        GameTimerUI,
    ));
}

fn start_timer_system(mut game_stopwatch: ResMut<GameStopwatch>) {
    game_stopwatch.0.unpause();
}

fn update_timer_system(
    mut game_stopwatch: ResMut<GameStopwatch>,
    mut current_game_time: ResMut<CurrentGameTime>,
    time: Res<Time>,
    mut game_timer_ui_query: Query<(&mut Text, &mut Transform), With<GameTimerUI>>,
    player_query: Query<&Transform, (With<Player>, Without<GameTimerUI>)>,
    game_timer_ui_offset: Res<GameTimerUIOffset>,
) {
    game_stopwatch.0.tick(time.delta());
    current_game_time.0 = game_stopwatch.0.elapsed_secs();

    let (mut text, mut text_transform) = game_timer_ui_query.single_mut();
    let text_value = f32::trunc(current_game_time.0 * 100.0) / 100.0;
    text.sections[0].value = text_value.to_string();

    let player_transform = player_query.single();
    text_transform.translation = game_timer_ui_offset.0 + player_transform.translation;
}

fn reset_and_save_timer_system(
    mut game_stopwatch: ResMut<GameStopwatch>,
    current_game_time: Res<CurrentGameTime>,
    mut final_game_time: ResMut<FinalGameTime>,
) {
    game_stopwatch.0.pause();
    final_game_time.0 = current_game_time.0;
}

fn hide_timer_ui_system(
    mut game_timer_ui_query: Query<&mut Text, With<GameTimerUI>>,
    game_timer_ui_entity_query: Query<Entity, With<GameTimerUI>>,
    mut commands: Commands,
) {
    let mut text = game_timer_ui_query.single_mut();
    text.sections[0].value = "".to_string();

    for entities in game_timer_ui_entity_query.iter() {
        commands.entity(entities).despawn_recursive();
    }
}

// Show Timer UI
// Start Timer
// Update Timer
// Stop Timer
// Reset Timer + Save Time
// Hide Timer
