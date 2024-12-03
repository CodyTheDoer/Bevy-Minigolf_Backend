use bevy::prelude::*;

use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use sqlx::MySqlPool;
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
    let rows: Vec<(Uuid,)> = match sqlx::query_as::<_, (Uuid,)>("SELECT player_id FROM player_table")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Failed to execute query: {:?}", err);
            // Assuming `ctx` is an available context where you want to report the error
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err);
            })
            .await;
            return;
        }
    };

    let mut existing_match = false;
    let player_id = player.get_id();
    for uuid in rows {
        let owned_id = uuid.0.to_string();
        // Compare player_id with owned_id
        if player_id == owned_id {
            existing_match = true;
            break; // No need to continue looping once a match is found
        }
    }
    if !existing_match {
        let _new_player = {
            let insert_result = sqlx::query(
            "INSERT INTO player_table (player_id, username, email, created, updated
                ) VALUES (
                    UUID_TO_BIN(?), ?, ?, NOW(), NOW()
                )"
                )
                .bind(player.get_id().to_string())
                .bind(player.get_username())
                .bind(player.get_email())
                .execute(&pool)
                .await;
    
            match insert_result {
                Ok(_) => {
                    println!("Inserted new player set with ID: {:?}", player.get_id());
                }
                Err(err) => {
                    eprintln!("Failed to insert new player {:?}", err);
                    eprintln!("player_id: {:?}", player.get_id());
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new player set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };    
    } else {
        info!("player exists");
    }
}
