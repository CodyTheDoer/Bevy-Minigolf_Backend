use bevy::prelude::*;

use crate::RunTrigger;

impl RunTrigger {
    pub fn new() -> Self {
        let triggers = vec![
            String::from("party_handler_active_player_add_bonk"),
            String::from("party_handler_active_player_set_ball_location"),
            String::from("party_handler_active_player_set_hole_completion_state_true"),
            String::from("party_handler_cycle_active_player"),
            String::from("party_handler_new_player_ai"),
            String::from("party_handler_new_player_local"),
            String::from("party_handler_new_player_remote"),
            String::from("party_handler_remove_ai"),
            String::from("party_handler_remove_last_player"),
            String::from("network_get_client_state_all"),
            String::from("network_get_client_state_game"),
            String::from("game_handler_cycle_state_camera"),
            String::from("game_handler_cycle_state_map_set"),
            String::from("game_handler_cycle_current_level"),
            String::from("game_handler_get_active_ball_location"),
            String::from("game_handler_reset_active_ball_location"),
            String::from("game_handler_set_active_ball_location"),
            String::from("game_handler_state_turn_next_player_turn"),
            String::from("game_handler_start_game_local"),
            String::from("game_handler_toggle_state_game"),
            String::from("leader_board_log_game"),
            String::from("leader_board_review_last_game"),
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