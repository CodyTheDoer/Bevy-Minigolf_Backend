use bevy::prelude::*;
use sqlx::MySqlPool;
use sqlx::FromRow;  
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

pub mod handlers;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ClientProtocol{
    InitPlayerConnection,
}

#[derive(Resource)]
pub struct DatabasePool(pub MySqlPool);

#[derive(Clone, Resource)]
pub struct PlayerIdStorage {
    player_ids: Vec<String>,
}

impl PlayerIdStorage {
    pub fn new() -> Self {
        let player_ids: Vec<String> = Vec::new();
        PlayerIdStorage {
            player_ids,
        }
    }

    pub fn add(&mut self, id: &str) {
        self.player_ids.push(String::from(id));
    }

    pub fn get_last_str(&self) -> String {
        let count = self.player_ids.clone().len();
        let last = &self.player_ids[count];
        last.to_owned()
    }

    pub fn get_last_str_and_pop(&mut self) -> String {
        let count = self.player_ids.clone().len() - 1;
        let last = &self.player_ids.clone()[count];
        self.player_ids.pop();
        last.to_owned()
    }
}

#[derive(Debug, Resource)]
pub struct RunTrigger{
    network_get_client_state_game: bool,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct MapSets{
    pub map_sets: Vec<MapSet>,
}

impl MapSets {
    pub fn new() -> Self {
        let map_sets: Vec<MapSet> = Vec::new();
        MapSets { 
            map_sets,
        }
    }
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct MapSet {
    pub map_set_id: Uuid,
    pub map_set_name: String,
    pub created: OffsetDateTime, // Use time crate's OffsetDateTime to handle timestamp values
    pub last_updated: OffsetDateTime, // Use time crate's OffsetDateTime to handle timestamp values
    pub hole_range_start: i32,
    pub hole_range_end: i32,
    pub file_path_level_1: Option<String>,
    pub file_path_level_2: Option<String>,
    pub file_path_level_3: Option<String>,
    pub file_path_level_4: Option<String>,
    pub file_path_level_5: Option<String>,
    pub file_path_level_6: Option<String>,
    pub file_path_level_7: Option<String>,
    pub file_path_level_8: Option<String>,
    pub file_path_level_9: Option<String>,
    pub file_path_level_10: Option<String>,
    pub file_path_level_11: Option<String>,
    pub file_path_level_12: Option<String>,
    pub file_path_level_13: Option<String>,
    pub file_path_level_14: Option<String>,
    pub file_path_level_15: Option<String>,
    pub file_path_level_16: Option<String>,
    pub file_path_level_17: Option<String>,
    pub file_path_level_18: Option<String>,
}