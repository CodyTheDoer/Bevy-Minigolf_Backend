use bevy::prelude::*;

use bevy_easy_vec_ui::BevyEasyVecUiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyEasyVecUiPlugin)             
        .run();
}