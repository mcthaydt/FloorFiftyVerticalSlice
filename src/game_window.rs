use bevy::prelude::*;

pub struct GameWindowPlugin;

#[derive(Resource)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDimensions {
            width: 960.0,
            height: 540.0,
        })
        .add_startup_system(initilizate_window)
        .add_system(bevy::window::close_on_esc);
    }
}

pub fn initilizate_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
}
