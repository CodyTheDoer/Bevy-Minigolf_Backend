use bevy::prelude::*;

use crate::RunTrigger;

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            network_get_client_state_game: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "network_get_client_state_game" => {
                self.network_get_client_state_game
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "network_get_client_state_game" => {
                self.network_get_client_state_game = state;
                info!("response: network_get_client_state_game: {}", self.get("network_get_client_state_game"));  
            },
            _ => {},
        }
    }

    pub fn network_get_client_state_game(&self) -> bool {
        self.network_get_client_state_game
    }
}