use bevy::prelude::*;
use sqlx::MySqlPool;
use sqlx::FromRow;  
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

pub mod handlers;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ClientProtocol{
    #[default]
    Idle,
    InitPlayerConnection,
    SyncExistingPlayerId,
}

#[derive(Event)]
pub struct SyncPlayerIdEvent {
    pub player_id_host: String,
    pub player_id_client: String,
}

#[derive(Resource)]
pub struct DatabasePool(pub MySqlPool);

#[derive(Clone, Debug, Resource)]
pub struct PlayerInfo {
    player_id: String,
    player_email: String,
    player_username: String,
}

#[derive(Clone, Debug, Resource)]
pub struct PlayerInfoStorage {
    pub players: Arc<Mutex<Vec<Arc<Mutex<PlayerInfo>>>>>,
}

#[derive(Debug, Resource)]
pub struct RunTrigger{
    db_pipeline_player_init: bool,
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