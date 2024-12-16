use bevy::prelude::*;

use bevy_easy_vec_ui::{BevyEasyVecUiPlugin, EasyVecUi};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyEasyVecUiPlugin::init("fonts/MatrixtypeDisplay-KVELZ.ttf")
            .camera_layer(-1)
            .title("Easy Vec to Ui Interface")
            .title_font_size(42.0)
            .data_font_size(12.0)
            .build()
        )
        .add_systems(Update, easy_vec_ui)
        .run();
}

fn easy_vec_ui(mut easy_vec_ui_resource: ResMut<EasyVecUi>) {
    let left_data_vec = vec![
            String::from("Left"),
            String::from("Vec"),
            String::from("Ui"),
            String::from("DATA"),
            String::from("Points"),
        ];
    let right_data_vec = vec![
            String::from("Right"),
            String::from("Vec"),
            String::from("Ui"),
            String::from("DATA"),
            String::from("Points"),
        ];
    easy_vec_ui_resource.inject_vec_left(left_data_vec);
    easy_vec_ui_resource.inject_vec_right(right_data_vec);
}