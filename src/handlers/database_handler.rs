use bevy::prelude::*;

use bevy_matchbox::prelude::*;
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use sqlx::{MySqlPool, Error};
use uuid::Uuid;

use crate::{
    DatabasePool,
    PlayerInfo,
    PlayerInfoStorage,
    RunTrigger,
    SyncPlayerIdEvent,
};

pub fn db_pipeline_player_init(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>, 
    player_info_storage: ResMut<PlayerInfoStorage>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    info!("db_pipeline_player_init: pre");
    info!("PlayerInfoStorage size: {:?}", player_info_storage.players_vec_len());
    if let Some(player) = player_info_storage.get_last_player_and_pop() {
        let pool = pool.0.clone();
        // Spawn the background task using bevy_tokio_tasks
        runtime.spawn_background_task(move |ctx| {
            db_pipeline_player_init_async(player, pool, ctx)
        });
    };
    run_trigger.set_target("db_pipeline_player_init", false);
}

pub async fn db_pipeline_player_init_async(
    player: PlayerInfo,
    pool: MySqlPool,
    mut ctx: TaskContext,
) {
    info!("Init: db_query_player_create_if_null_async");
    info!("Player: {:?}", player);

    // Step 1: Fetch player IDs and emails from the database
    let player_ids_emails = match fetch_player_ids_and_emails(&pool, &mut ctx).await {
        Ok(data) => data,
        Err(_) => return, // Error handling already done in helper function
    };

    // Step 2: Extract player information
    let player_id = player.get_id().to_string();
    let player_email = player.get_email();

    // Step 3: Check if player ID exists in the fetched list
    let id_match = player_ids_emails.iter().any(|(id, _)| {
        if let Some(ref id_value) = id {
            id_value.to_string() == player_id
        } else {
            false
        }
    });

    if id_match {
        // Player with this ID already exists in the database
        info!("Player exists");
        return;
    }

    // Step 4: Check if player email exists and handle accordingly
    let email_match = player_ids_emails.iter().find(|(_, email)| email == &player_email);

    match email_match {
        Some((db_player_id, _)) => {
            // Player email exists, so we need to sync the ID with the client

            if let Some(db_player_id) = db_player_id.clone() {
                // Send the correct ID (from the database) to the client
                ctx.run_on_main_thread(move |ctx| { 
                    let event_writer = ctx.world.get_resource_mut::<Events<SyncPlayerIdEvent>>();
                    if let Some(mut writer) = event_writer {
                        writer.send(SyncPlayerIdEvent {
                            player_id_host: db_player_id.to_string(), // Use the ID from the database
                            player_id_client: player_id,
                        });
                    }
                })
                .await;
            } else {
                eprintln!("Error: Player ID is empty for the matched email: {:?}", player_email);
            }
        },
        None => {
            // Step 5: If no ID or email match found, insert a new player
            if let Err(err) = insert_new_player(&pool, &player, &mut ctx).await {
                eprintln!("Failed to insert new player {:?}", err);
                eprintln!("player_id: {:?}", player.get_id());
            }
        }
    }
}

async fn fetch_player_ids_and_emails(
    pool: &MySqlPool,
    ctx: &mut TaskContext,
) -> Result<Vec<(Option<Uuid>, String)>, Error> {
    // Define the query to fetch both player_id and email from the player_table
    match sqlx::query_as::<_, (Option<Uuid>, String)>("SELECT player_id, email FROM player_table")
        .fetch_all(pool)
        .await
    {
        Ok(player_data) => Ok(player_data),
        Err(err) => {
            let err_for_ctx = err.to_string(); // Convert error to string or clone it before moving it
            eprintln!("Failed to execute query: {:?}", err_for_ctx);
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err_for_ctx);
            })
            .await;
            Err(err)
        }
    }
}

async fn _fetch_player_ids(
    pool: &MySqlPool,
    ctx: &mut TaskContext,
) -> Result<Vec<(Option<Uuid>,)>, Error> {
    match sqlx::query_as::<_, (Option<Uuid>,)>("SELECT player_id FROM player_table")
        .fetch_all(pool)
        .await
    {
        Ok(player_ids) => Ok(player_ids),
        Err(err) => {
            let err_for_ctx = err.to_string(); // Convert error to string or clone it before moving it
            eprintln!("Failed to execute query: {:?}", err_for_ctx);
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err_for_ctx);
            })
            .await;
            Err(err)
        }
    }
}

async fn _fetch_player_emails(
    pool: &MySqlPool,
    ctx: &mut TaskContext,
) -> Result<Vec<(String,)>, Error> {
    match sqlx::query_as::<_, (String,)>("SELECT email FROM player_table")
        .fetch_all(pool)
        .await
    {
        Ok(player_emails) => Ok(player_emails),
        Err(err) => {
            let err_for_ctx = err.to_string(); // Convert error to string or clone it before moving it
            eprintln!("Failed to execute query: {:?}", err_for_ctx);
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err_for_ctx);
            })
            .await;
            Err(err)
        }
    }
}

async fn _update_player_id(
    pool: &MySqlPool,
    player: &PlayerInfo,
    player_email: &str,
) -> Result<(), Error> {
    sqlx::query(
        "UPDATE player_table
         SET player_id = UUID_TO_BIN(?), updated = NOW()
         WHERE email = ?",
    )
    .bind(player.get_id().to_string()) // Bind the new player_id
    .bind(player_email)                // Bind the matched email
    .execute(pool)
    .await
    .map(|_| ())
}

async fn insert_new_player(
    pool: &MySqlPool,
    player: &PlayerInfo,
    ctx: &mut TaskContext,
) -> Result<(), Error> {
    match sqlx::query(
        "INSERT INTO player_table (player_id, username, email, created, updated)
         VALUES (UUID_TO_BIN(?), ?, ?, NOW(), NOW())",
    )
    .bind(player.get_id().to_string())
    .bind(player.get_username())
    .bind(player.get_email())
    .execute(pool)
    .await
    {
        Ok(_) => {
            println!("Inserted new player with ID: {:?}", player.get_id());
            Ok(())
        }
        Err(err) => {
            let err_for_ctx = err.to_string(); // Convert error to string or clone it before moving it
            eprintln!("Failed to insert new player: {:?}", err_for_ctx);
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to insert new player set in the task: {:?}", err_for_ctx);
            })
            .await;
            Err(err)
        }
    }
}

pub fn sync_player_id_init_system(
    mut event_reader: EventReader<SyncPlayerIdEvent>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
) {
    for event in event_reader.read() {
        let peers: Vec<_> = socket.connected_peers().collect();
        for peer in peers {
            let message = format!("({}, SyncExistingPlayerId({:?}))", event.player_id_client.clone(), event.player_id_host.clone());
            info!("Sending sync_player_id_init_system update: {message:?} to {peer}");
            socket.send(message.as_bytes().into(), peer);
        }
    }
}