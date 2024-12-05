//! Runs both signaling with server/client topology and runs the host in the same process

use bevy::{prelude::*, 
    input::common_conditions::*,
};

use dotenv::dotenv;
use std::{
        time::Duration,
        env,
};
use sqlx::mysql::MySqlPoolOptions;
use tokio::runtime::Runtime;

use minigolf_backend_server::{
    ClientProtocol,
    ConnectedPlayers,
    DatabasePool,
    Fonts,
    HeartBeatMonitorTimer,
    MapSets,
    PlayerInfoStorage,
    RunTrigger,
    SyncPlayerIdEvent,
    SyncTriggerIndexEvent,
    UiUpdateTimer,
};

use minigolf_backend_server::user_interface::{
    interface,
    setup_ui,
    ui_update_system,
};

use minigolf_backend_server::handlers::{
    database_handler::{
        db_pipeline_player_init,
        sync_player_id_init_system,
    },
    heartbeat_handler::heartbeat_monitor_system,
    map_set_handler::first_time_boot_setup_map_set,
    run_trigger_handler::client_run_trigger,
    signaling_server_handler::{
        network_get_client_state_game,
        receive_client_requests,
        start_host_socket,
        start_signaling_server,
    },
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
        
        .insert_state(ClientProtocol::Idle)
        
        .add_event::<SyncPlayerIdEvent>() 
        .add_event::<SyncTriggerIndexEvent>() 

        .insert_resource(ConnectedPlayers::new())
        .insert_resource(DatabasePool(pool))
        .insert_resource(Fonts::new())
        .insert_resource(MapSets::new())
        .insert_resource(PlayerInfoStorage::new())
        .insert_resource(RunTrigger::new())

        .insert_resource(UiUpdateTimer(Timer::new(Duration::from_millis(250), TimerMode::Repeating)))
        .insert_resource(HeartBeatMonitorTimer(Timer::new(Duration::from_secs(5), TimerMode::Repeating)))
        
        // .add_systems(Update, send_message.run_if(on_timer(Duration::from_secs(5))))
        .add_systems(Startup, (start_signaling_server, start_host_socket).chain())
        .add_systems(Startup, setup_ui)

        .add_systems(Update, interface)
        .add_systems(Update, sync_player_id_init_system)
        .add_systems(Update, receive_client_requests)
        .add_systems(Update, heartbeat_monitor_system)
        .add_systems(Update, client_run_trigger)
        .add_systems(Update, first_time_boot_setup_map_set.run_if(input_just_released(KeyCode::Space)))
        .add_systems(Update, db_pipeline_player_init.run_if(|run_trigger: Res<RunTrigger>|run_trigger.db_pipeline_player_init()))
        .add_systems(Update, network_get_client_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.network_get_client_state_game()))
        .add_systems(Update, ui_update_system)                
        // .add_systems(Update, client_sync_protocol_send_existing_map_sets.run_if(input_just_released(KeyCode::KeyZ)))

        .run();
}