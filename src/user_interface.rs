use bevy::prelude::*;

use bevy_matchbox::prelude::*;

use crate::{
    CameraUi,
    ClientProtocol,
    ConnectedPlayers,
    ConnectedPlayersNode,
    Fonts,
    RunTrigger,
    SyncTriggerIndexEvent,
    TitleText,
    UiUpdateTimer,
};

pub fn interface(
    keys: Res<ButtonInput<KeyCode>>,
    connected_players: Res<ConnectedPlayers>,
    mut event_writer: EventWriter<SyncTriggerIndexEvent>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    if keys.pressed(KeyCode::KeyG) {
        info!("pressed: KeyG");  
        run_trigger.set_target("network_get_client_state_game", true);
    };

    if keys.pressed(KeyCode::KeyE) {
        info!("pressed: KeyE");  
        info!("RunTrigger.trigger_idx += 1");  
        let current_idx = run_trigger.get_trigger_idx();
        run_trigger.set_trigger_idx(current_idx + 1);
    }
    if keys.pressed(KeyCode::KeyD) {
        info!("pressed: KeyD");  
        info!("RunTrigger.trigger_idx -= 1");  
        let current_idx = run_trigger.get_trigger_idx();
        run_trigger.set_trigger_idx(current_idx - 1);
    }
    if keys.pressed(KeyCode::KeyR) {
        info!("pressed: KeyR");  
        let players = connected_players.players.lock().unwrap();
        for player_id in players.keys() {
            event_writer.send(SyncTriggerIndexEvent{
                player_id: *player_id, 
                trigger_idx: run_trigger.get_trigger_idx(),
            });
        }
    }
}

pub fn setup_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut fonts: ResMut<Fonts>,
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
        font_size: 14.0,
        ..default()
    };
    fonts.fonts.push(matrix_display);
    fonts.fonts.push(matrix_display_small);

    // Set up a 2D camera for the UI
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            camera: Camera {
                order: -1, // Render before the 3D scene
                ..default()
            },
            ..default()
        },
        CameraUi,
    ));

    // Title: Create a screen-sized UI node for the centered title
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
                            "Minigolf Backend Server: UI",
                            fonts.fonts[0].clone(),
                        )],
                        ..default()
                    },
                    ..default()
                },
                TitleText, // Tag the title text so it can be updated later
            ));
        });

    // HUD: Create a UI node to display connected players
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
                ConnectedPlayersNode, // Tag the node for easy updates later
            ));
        });
}

pub fn ui_update_system(
    time: Res<Time>,
    mut timer: ResMut<UiUpdateTimer>,
    mut commands: Commands,
    connected_players: Res<ConnectedPlayers>,
    query: Query<Entity, With<ConnectedPlayersNode>>,
    asset_server: Res<AssetServer>,
    fonts: ResMut<Fonts>,
) {
    // Check if the timer has finished
    if timer.0.tick(time.delta()).finished() {
        // Call the function to update the connected players UI
        update_ui(commands, connected_players, query, asset_server, fonts);
    }
}

pub fn update_ui(
    mut commands: Commands,
    connected_players: Res<ConnectedPlayers>,
    query: Query<Entity, With<ConnectedPlayersNode>>,
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<Fonts>,
) {
    // Load and setup fonts
    let font = asset_server.load("fonts/MatrixtypeDisplay-KVELZ.ttf");
    let matrix_display_small = TextStyle {
        font: font.clone(),
        font_size: 14.0,
        ..default()
    };
    fonts.fonts.push(matrix_display_small.clone());

    // Find the entity representing the node that displays connected players
    if let Ok(connected_players_node) = query.get_single() {
        // Clear existing children (if any) to refresh the UI
        commands.entity(connected_players_node).despawn_descendants();

        // Lock the connected players to read player data
        let players_guard = connected_players.players.lock().unwrap();
        
        // Iterate over each player and create a row for each one
        for (uuid, player_status) in players_guard.iter() {
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
                                    format!("Player ID: {}", uuid),
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

                        // Last Heartbeat text
                        row.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("Last heartbeat: {:?}", player_status.last_heartbeat),
                                    matrix_display_small.clone(),
                                )],
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
        }
    }
}
