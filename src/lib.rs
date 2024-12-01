use bevy::prelude::*;
use sqlx::MySqlPool;

pub mod handlers;

#[derive(Resource)]
pub struct DatabasePool(pub MySqlPool);

#[derive(Debug, Resource)]
pub struct RunTrigger{
    network_get_client_state_game: bool,
}