use bevy::prelude::*;

use bevy_easy_vec_ui::BevyEasyVecUiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyEasyVecUiPlugin::init(
            "fonts/MatrixtypeDisplay-KVELZ.ttf",
            -1,
            42.0,
            12.0,
        ))             
        .run();
}