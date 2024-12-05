use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::Instant;
use sqlx::MySqlPool;
use sqlx::FromRow;  
use time::OffsetDateTime;
use uuid::Uuid;

pub mod handlers;
pub mod user_interface;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Asset, Component, TypePath)]
pub struct CameraUi;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum ClientProtocol{
    #[default]
    Idle,
    InitPlayerConnection,
    SyncExistingPlayerId,
    RunTrigger,
}

#[derive(Clone, Debug, Resource)]
pub struct ConnectedPlayers {
    // Using a HashMap to map player UUIDs to metadata about their connection
    pub players: Arc<Mutex<HashMap<Uuid, PlayerHeartBeatStatus>>>,
}

#[derive(Component)]
pub struct ConnectedPlayersNode;

#[derive(Resource)]
pub struct DatabasePool(pub MySqlPool);


#[derive(Resource)]
pub struct Fonts {
    pub fonts: Vec<TextStyle>,
}

impl Fonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        Fonts {
            fonts,
        }
    }
}

#[derive(Resource)]
pub struct HeartBeatMonitorTimer(pub Timer);

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct MapSets{
    pub map_sets: Vec<MapSet>,
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

impl MapSets {
    pub fn new() -> Self {
        let map_sets: Vec<MapSet> = Vec::new();
        MapSets { 
            map_sets,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PacketAllStates {
    player_id: String,
    state_game: String,
    state_cam_orbit_entity: String,
    state_game_play_style: String,
    state_level: String,
    state_map_set: String,
    state_menu: String,
    state_turn: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PacketHeartBeat {
    player_id: String,
}

#[derive(Clone, Debug)]
pub struct PlayerHeartBeatStatus {
    pub last_heartbeat: Instant,
    // Additional fields can be added here, e.g., player status, connection info, etc.
}

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

#[derive(Component)]
pub struct PlayerStatusText;

#[derive(Debug, Resource)]
pub struct RunTrigger{
    trigger_idx: i32,
    triggers: Vec<String>,
    db_pipeline_player_init: bool,
    network_get_client_state_game: bool,
}

#[derive(Event)]
pub struct SyncPlayerIdEvent {
    pub player_id_host: String,
    pub player_id_client: String,
}

#[derive(Event)]
pub struct SyncTriggerIndexEvent {
    pub player_id: Uuid,
    pub trigger_idx: usize,
}

#[derive(Component)]
pub struct TitleText;

#[derive(Resource)]
pub struct UiUpdateTimer(pub Timer);