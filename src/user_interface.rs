use bevy::prelude::*;

use bevy_easy_vec_ui::EasyVecUi;

use crate::{
    ConnectedPlayers, 
    RunTrigger, 
    SyncTriggerIndexEvent, 
};

pub fn interface(
    keys: Res<ButtonInput<KeyCode>>,
    connected_players: Res<ConnectedPlayers>,
    mut event_writer: EventWriter<SyncTriggerIndexEvent>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    if keys.pressed(KeyCode::ShiftLeft) {
        if keys.just_released(KeyCode::KeyD) {
            info!("pressed: KeyD");  
            run_trigger.trigger_add_one();
        }
        if keys.just_released(KeyCode::KeyE) {
            info!("pressed: KeyE");  
            run_trigger.trigger_sub_one();
        }
        if keys.just_released(KeyCode::KeyF) {
            info!("pressed: KeyF");  
            let players = connected_players.players.lock().unwrap();
            for player_id in players.keys() {
                event_writer.send(SyncTriggerIndexEvent{
                    player_id: *player_id, 
                    trigger_idx: run_trigger.get_trigger_idx(),
                });
                info!("post trigger:{:?}", &player_id);
            }
        }
    }
}

pub fn easy_vec_ui(
    mut easy_vec_ui_resource: ResMut<EasyVecUi>,
    connected_players: Res<ConnectedPlayers>,
    run_trigger: Res<RunTrigger>,
) {
    let mut left_data_vec: Vec<String> = Vec::new();
    
    // Lock the connected players to read player data
    let players_guard = connected_players.players.lock().unwrap();
    for (uuid, player_status) in players_guard.iter() { // Iterate over each player and create a row for each one
        left_data_vec.push(String::from(format!("Player ID: [{}] Last heartbeat: [{:?}]", uuid, player_status.last_heartbeat)));
    }
    
    left_data_vec.push(String::from(format!("( Shift + E ) <--- Client Run Trigger Index [{}] ---> ( Shift + D )", run_trigger.get_trigger_idx())));
    left_data_vec.push(String::from(format!("( Shift + F ) All Clients Run Trigger: [{}]", run_trigger.get_triggers_ref()[run_trigger.get_trigger_idx()])));

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