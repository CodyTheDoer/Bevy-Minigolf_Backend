//! Runs both signaling with server/client topology and runs the host in the same process

use bevy::prelude::*;

use std::time::Duration;

use minigolf_backend_server::{
    EasyVecUiFonts,
    EasyVecUiUpdateTimer,
};

use minigolf_backend_server::user_interface::{
    interface,
    setup_ui,
    ui_update_system,
};

pub struct EasyVecUiPlugin;

impl Plugin for EasyVecUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EasyVecUiFonts::new());
        app.insert_resource(EasyVecUiUpdateTimer(Timer::new(Duration::from_millis(250), TimerMode::Repeating)));
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, interface);
        app.add_systems(Update, ui_update_system);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EasyVecUiPlugin)             
        .run();
}