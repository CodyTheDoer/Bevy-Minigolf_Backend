use bevy::prelude::*;

use crate::{
    EasyVecUiCamera,
    EasyVecUiFonts, 
    EasyVecUiNode,
    EasyVecUiTitleText, 
    EasyVecUiUpdateTimer,
};

pub fn interface(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ShiftLeft) {
        if keys.just_released(KeyCode::KeyD) {
            info!("pressed: KeyD");
        };
    }
}

pub fn setup_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut fonts: ResMut<EasyVecUiFonts>,
) {
    // Load and setup fonts
    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf");
    let matrix_display = TextStyle {
        font: font.clone(),
        font_size: 42.0,
        ..default()
    };
    let matrix_display_small = TextStyle {
        font: font.clone(),
        font_size: 12.0,
        ..default()
    };
    fonts.fonts.push(matrix_display);
    fonts.fonts.push(matrix_display_small);

    // Set up a 2D camera for the Ui
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            camera: Camera {
                order: -1, // Render before the 3D scene
                ..default()
            },
            ..default()
        },
        EasyVecUiCamera,
    ));

    // Title: Create a screen-sized Ui node for the centered title
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::Center, // Align the title text to the center vertically
                justify_content: JustifyContent::Center, // Center the title text horizontally
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(10.0), // Height is 10% of the screen, to occupy the top area
                top: Val::Percent(0.0),     // Position it at the very top
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Easy Vec to Ui Interface",
                            fonts.fonts[0].clone(),
                        )],
                        ..default()
                    },
                    ..default()
                },
                EasyVecUiTitleText, // Tag the title text so it can be updated later
            ));
        });

    // HUD: Create a Ui node to display connected players
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::FlexStart,     // Align items from the top of the node
                flex_direction: FlexDirection::Column,  // Stack items vertically
                justify_content: JustifyContent::FlexStart, // Align from the start (top-left)
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.0), // Position at the bottom of the screen
                left: Val::Percent(0.0),   // Align it to the left of the screen
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Tag this node so it can be dynamically updated
            parent.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column, // Stack items vertically
                        ..default()
                    },
                    ..default()
                },
                EasyVecUiNode, // Tag the node for easy updates later
            ));
        });
}

pub fn ui_update_system(
    time: Res<Time>,
    mut timer: ResMut<EasyVecUiUpdateTimer>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: ResMut<EasyVecUiFonts>,
    query: Query<Entity, With<EasyVecUiNode>>,
) {
    // Check if the timer has finished
    if timer.0.tick(time.delta()).finished() {
        let temp_vec = vec![
            String::from("Temp"),
            String::from("Vec"),
            String::from("Ui"),
            String::from("DATA"),
            String::from("Points"),
        ];
        // Call the function to update the connected players Ui
        update_ui(temp_vec , query, commands, asset_server, fonts);
    }
}

pub fn update_ui(
    connected_players: Vec<String>,
    query: Query<Entity, With<EasyVecUiNode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<EasyVecUiFonts>,
) {
    // Load and setup fonts
    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf");
    let matrix_display_small = TextStyle {
        font: font.clone(),
        font_size: 14.0,
        ..default()
    };
    fonts.fonts.push(matrix_display_small.clone());

    
    if let Ok(connected_players_node) = query.get_single() {
        commands.entity(connected_players_node).despawn_descendants();

        // Iterate over each player and create a row for each one
        for status in connected_players.iter() {
            // Spawn a new node for each player, representing a row
            commands.entity(connected_players_node).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row, // Arrange items horizontally within the row
                            align_items: AlignItems::Center,    // Center items vertically within the row
                            margin: UiRect::all(Val::Px(5.0)),  // Add some spacing between rows
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        // Player ID text
                        row.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{}", status),
                                    matrix_display_small.clone(),
                                )],
                                ..default()
                            },
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)), // Spacing between player ID and other fields
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
        }

        let info_vec = vec![
            format!("________________________________________"),
            format!("________________________________________"),
            ];
        for info in info_vec.iter() {
            commands.entity(connected_players_node).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row, // Arrange items horizontally within the row
                            align_items: AlignItems::Center,    // Center items vertically within the row
                            margin: UiRect::all(Val::Px(5.0)),  // Add some spacing between rows
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        // Player ID text
                        row.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{}", info),
                                    matrix_display_small.clone(),
                                )],
                                ..default()
                            },
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)), // Spacing between player ID and other fields
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
        }
    }
}