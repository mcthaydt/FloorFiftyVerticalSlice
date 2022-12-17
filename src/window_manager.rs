use bevy::prelude::*;

pub struct GameWindowPlugin;

#[derive(Resource)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        // 16:9 Ratio
        app.insert_resource(WindowDimensions {
            width: 960.0,
            height: 540.0,
        })
        .add_startup_system(setup_window_settings_system.at_start())
        .add_system(bevy::window::close_on_esc);
    }
}

pub fn setup_window_settings_system(mut windows: ResMut<Windows>) {
    // This code sets up the window settings for the primary window in the `windows` resource.
    // It makes the cursor invisible and locks it to the window.
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
}
