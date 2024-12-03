use bevy::prelude::*;

use std::sync::Arc;
use std::sync::Mutex;

use crate::{
    PlayerInfo,
    PlayerInfoStorage,
};

impl PlayerInfo {
    pub fn new(player_id: String, player_email: String, player_username: String) -> Self {
        PlayerInfo {
            player_id,
            player_email,
            player_username,
        }
    }

    pub fn from_vec_str(info: Vec<&str>) -> PlayerInfo {
        let player_id: String = String::from(info[0]);
        let player_email: String = String::from(info[1]);
        let player_username: String = String::from(info[2]);
        PlayerInfo {
            player_id,
            player_email, 
            player_username,
        }
    }

    pub fn from_vec_string(info: Vec<String>) -> PlayerInfo {
        PlayerInfo {
            player_id: info[0].clone(),
            player_email: info[1].clone(),
            player_username: info[2].clone(),
        }
    }

    pub fn get_id(&self) -> String {
        self.player_id.clone()
    }

    pub fn get_email(&self) -> String {
        self.player_email.clone()
    }

    pub fn get_username(&self) -> String {
        self.player_username.clone()
    }
}

impl PlayerInfoStorage {
    pub fn new() -> Self {
        PlayerInfoStorage {
            players: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&self, player: PlayerInfo) {
        info!("PlayerInfoStorage: Add {:?}", player.clone());
        let mut players = self.players.lock().unwrap();
        players.push(Arc::new(Mutex::new(player)));
        info!("players: {:?}", players);
    }

    pub fn players_vec_len(&self) -> usize {
        info!("PlayerInfoStorage: players_vec_len");
        let players = self.players.lock().unwrap();
        players.len()
    }

    pub fn get_last_player(&self) -> Option<PlayerInfo> {
        info!("PlayerInfoStorage: get_last_player");
        let players = self.players.lock().unwrap();
        if let Some(last) = players.last() {
            let player = last.lock().unwrap();
            Some(player.clone())
        } else {
            None
        }
    }

    pub fn get_last_player_and_pop(&self) -> Option<PlayerInfo> {
        info!("PlayerInfoStorage: get_last_player_and_pop");
        let mut players = self.players.lock().unwrap();
        if let Some(last) = players.pop() {
            let player = last.lock().unwrap();
            Some(player.clone())
        } else {
            None
        }
    }

    pub fn get_last_player_id_and_pop_player(&self) -> Option<String> {
        info!("PlayerInfoStorage: get_last_player_id_and_pop_player");
        let mut players = self.players.lock().unwrap();
        if let Some(last) = players.pop() {
            let player = last.lock().unwrap();
            Some(player.player_id.clone())
        } else {
            None
        }
    }

    pub fn get_last_player_id_string(&self) -> Option<String> {
        info!("PlayerInfoStorage: get_last_player_id_string");
        let mut players = self.players.lock().unwrap().clone();
        if let Some(last) = players.last() {
            let player_info = last.lock().unwrap().clone();
            let player_id = player_info.get_id();
            Some(player_id)
        } else {
            None
        }
    }
}

