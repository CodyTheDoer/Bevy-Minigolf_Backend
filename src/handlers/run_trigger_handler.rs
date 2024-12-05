use bevy::prelude::*;

use bevy_matchbox::prelude::*;

use crate::{
    RunTrigger,
    SyncTriggerIndexEvent,
};

impl RunTrigger {
    pub fn new() -> Self {
        let triggers = vec![
            String::from("camera_handler_cycle_state_camera"),
            String::from("game_handler_game_start"),
            String::from("game_handler_game_state_change_routines"),
            String::from("game_handler_update_players_ref_ball_locations"),
            String::from("game_handler_update_players_reset_ref_ball_locations "),
            String::from("game_handler_update_players_store_current_ball_locations_to_ref"),
            String::from("leader_board_log_game"),
            String::from("leader_board_review_last_game"),
            String::from("level_handler_set_state_next_level"),
            String::from("level_handler_set_state_next_map_set"),
            String::from("network_get_client_state_game"),
            String::from("party_handler_active_player_add_bonk"),
            String::from("party_handler_active_player_set_ball_location"),
            String::from("party_handler_active_player_set_hole_completion_state_true"),
            String::from("party_handler_cycle_active_player"),
            String::from("party_handler_new_player_ai"),
            String::from("party_handler_new_player_local"),
            String::from("party_handler_new_player_remote"),
            String::from("party_handler_remove_ai"),
            String::from("party_handler_remove_last_player"),
            String::from("turn_handler_set_turn_next"),
        ];
        Self{
            trigger_idx: 0,
            triggers,
            db_pipeline_player_init: false,
            network_get_client_state_game: false,
        }
    }

    pub fn get_triggers_ref(&self) -> &Vec<String> {
        &self.triggers
    }

    pub fn get_trigger_idx(&self) -> usize {
        self.trigger_idx as usize
    }

    pub fn set_trigger_idx(&mut self, idx: usize) {
        self.trigger_idx = idx as i32;
    }

    pub fn trigger_add_one(&mut self) {
        let len_check = self.triggers.len();
        if self.trigger_idx == len_check as i32 - 1 {
            self.set_trigger_idx(0);
        } else {
            self.trigger_idx += 1;
        }
    }

    pub fn trigger_sub_one(&mut self) {
        let len_check = self.triggers.len();
        if self.trigger_idx == 0 {
            self.set_trigger_idx(len_check - 1);
        } else {
            self.trigger_idx -= 1;
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "network_get_client_state_game" => {
                self.network_get_client_state_game
            },
            "db_pipeline_player_init" => {
                self.db_pipeline_player_init
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "db_pipeline_player_init" => {
                self.db_pipeline_player_init = state;
                info!("db_pipeline_player_init: {}", self.get("db_pipeline_player_init"));  
            },
            "network_get_client_state_game" => {
                self.network_get_client_state_game = state;
                info!("response: network_get_client_state_game: {}", self.get("network_get_client_state_game"));  
            },
            _ => {},
        }
    }

    pub fn db_pipeline_player_init(&self) -> bool {
        self.db_pipeline_player_init
    }

    pub fn network_get_client_state_game(&self) -> bool {
        self.network_get_client_state_game
    }
}

pub fn client_run_trigger(
    trigger: ResMut<RunTrigger>,
    mut event_reader: EventReader<SyncTriggerIndexEvent>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
) {
    for event in event_reader.read() {
        info!("client_run_trigger:{:?}", event.player_id.clone());
        let target_idx =  trigger.get_trigger_idx();
        let triggers = trigger.get_triggers_ref();
        let trigger = triggers[target_idx].as_str();
        let peers: Vec<_> = socket.connected_peers().collect();
        for peer in peers {
            let message = format!("({}, RunTrigger({:?}))", event.player_id.clone(), trigger);
            info!("Sending sync_player_id_init_system update: {message:?} to {peer}");
            socket.send(message.as_bytes().into(), peer);
        }
    }
}