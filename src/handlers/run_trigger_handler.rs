use bevy::prelude::*;

use crate::RunTrigger;

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            db_pipeline_player_init: false,
            network_get_client_state_game: false,
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