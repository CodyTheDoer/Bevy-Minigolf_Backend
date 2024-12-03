//! Runs both signaling with server/client topology and runs the host in the same process

use bevy::{prelude::*, 
    input::common_conditions::*,
};

use bevy_matchbox::prelude::*;
use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;
use tokio::runtime::Runtime;

use minigolf_backend_server::{
    DatabasePool,
    RunTrigger,
    MapSets,
    PlayerIdStorage,
};

use minigolf_backend_server::handlers::map_set_handler::{
    client_sync_protocol_send_existing_map_sets,
    first_time_boot_setup_map_set,
};
use minigolf_backend_server::handlers::signaling_server_handler::{
    network_get_client_state_game,
    receive_client_requests,
    start_host_socket,
    start_signaling_server,
};

async fn establish_connection() -> sqlx::Result<sqlx::Pool<sqlx::MySql>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Create a connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Set the number of maximum connections in the pool
        .connect(&database_url)
        .await?;

    Ok(pool)
}

fn main() {
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    
    // Use the runtime to block on the async function and get the pool
    let pool = runtime.block_on(establish_connection())
        .expect("Failed to create database connection pool");
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
        .insert_resource(DatabasePool(pool))
        .insert_resource(MapSets::new())
        .insert_resource(PlayerIdStorage::new())
        .insert_resource(RunTrigger::new())
        // .add_systems(Update, send_message.run_if(on_timer(Duration::from_secs(5))))
        .add_systems(Startup, (start_signaling_server, start_host_socket).chain())
        .add_systems(Update, temp_interface.run_if(input_just_released(KeyCode::ShiftLeft)))
        .add_systems(Update, receive_client_requests)
        .add_systems(Update, first_time_boot_setup_map_set.run_if(input_just_released(KeyCode::Space)))
        .add_systems(Update, client_sync_protocol_send_existing_map_sets.run_if(input_just_released(KeyCode::KeyZ)))
        .add_systems(Update, network_get_client_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.network_get_client_state_game()))
        .run();
}

fn temp_interface(
    keys: Res<ButtonInput<KeyCode>>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    if keys.pressed(KeyCode::KeyG) {
        info!("pressed: KeyG");  
        run_trigger.set_target("network_get_client_state_game", true);
    };
    let mut trigger = "";
    if keys.pressed(KeyCode::Space) {
        info!("pressed: Space");  
        trigger = "game_handler_toggle_state_game";
    };
    if keys.pressed(KeyCode::KeyB) {
        info!("pressed: KeyB");  
        trigger = "party_handler_active_player_add_bonk";
    };
    if keys.pressed(KeyCode::KeyA) { // should trigger with new turn
        info!("pressed: KeyA");  
        trigger = "party_handler_active_player_set_hole_completion_state_true";
    };
    if keys.pressed(KeyCode::KeyC) {
        info!("pressed: KeyC");  
        trigger = "game_handler_cycle_state_camera";
    };
    if keys.pressed(KeyCode::KeyM) {
        info!("pressed: KeyM");  
        trigger = "game_handler_cycle_state_map_set";
    };
    if keys.pressed(KeyCode::KeyN) {
        info!("pressed: KeyN");  
        trigger = "game_handler_state_turn_next_player_turn";
    };
    if keys.pressed(KeyCode::KeyP) {
        info!("pressed: KeyP");  
        trigger = "party_handler_cycle_active_player";
    };
    if keys.pressed(KeyCode::KeyS) {
        info!("pressed: KeyS");  
        trigger = "game_handler_start_game_local";
    };
    if keys.pressed(KeyCode::Numpad1) {
        info!("pressed: Numpad1");  
        trigger = "party_handler_remove_last_player";
    };
    if keys.pressed(KeyCode::Numpad3) {
        info!("pressed: Numpad3");  
        trigger = "party_handler_remove_ai";
    };
    if keys.pressed(KeyCode::Numpad7) {
        info!("pressed: Numpad7");  
        trigger = "party_handler_new_player_local";
    };
    if keys.pressed(KeyCode::Numpad8) {
        info!("pressed: Numpad8");  
        trigger = "party_handler_new_player_remote";
    };
    if keys.pressed(KeyCode::Numpad9) {
        info!("pressed: Numpad9");   
        trigger = "party_handler_new_player_ai";
    };
    let peers: Vec<_> = socket.connected_peers().collect();
    for peer in peers {
        info!("Sending message: {trigger:?} to {peer}");
        socket.send(trigger.as_bytes().into(), peer);
    };
}