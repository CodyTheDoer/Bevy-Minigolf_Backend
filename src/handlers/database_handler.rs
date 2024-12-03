use bevy::prelude::*;

use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use sqlx::{MySqlPool, Error};
use uuid::Uuid;

use crate::{
    DatabasePool,
    RunTrigger,
    PlayerInfo,
    PlayerInfoStorage,
};

pub fn db_pipeline_player_init(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>, 
    mut player_info_storage: ResMut<PlayerInfoStorage>,
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

    // Fetch player IDs
    let player_ids = match fetch_player_ids(&pool, &mut ctx).await {
        Ok(ids) => ids,
        Err(_) => return, // If error handling already performed in helper function, just return here
    };

    // Fetch player emails
    let player_emails = match fetch_player_emails(&pool, &mut ctx).await {
        Ok(emails) => emails,
        Err(_) => return, // If error handling already performed in helper function, just return here
    };

    // Check if the player already exists by ID or email
    let player_id = player.get_id().to_string();
    let player_email = player.get_email();

    let id_match = player_ids.iter().any(|(id,)| {
        if let Some(ref id_value) = id {
            id_value.to_string() == player_id
        } else {
            false
        }
    });
    
    if !id_match {
        let mut email_match_needs_id = false;
        
        if let Some(_) = player_emails.iter().find(|(email,)| email == &player_email) {
            email_match_needs_id = true;
        }

        if email_match_needs_id {
            if let Err(e) = update_player_id(&pool, &player, &player_email).await {
                eprintln!("Error updating player ID: {}", e);
            } else {
                println!("Player ID synced with db: {:?}", player_id);
            }
            return; // No need to insert a new player since email was found
        } else {
            if let Err(err) = insert_new_player(&pool, &player, &mut ctx).await {
                eprintln!("Failed to insert new player {:?}", err);
                eprintln!("player_id: {:?}", player.get_id());   
            }
        }
    } else {
        info!("Player exists");
    }
}

async fn fetch_player_ids(
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

async fn fetch_player_emails(
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

async fn update_player_id(
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