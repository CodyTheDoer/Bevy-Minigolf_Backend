use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use rmp_serde::{
    encode, 
    decode, 
    to_vec_named,
};

use sqlx::{MySqlPool,
    query_as,
};
use time::{macros::datetime,
    OffsetDateTime,
};
use uuid::Uuid;

use crate::{
    DatabasePool,
    MapSet,
    MapSets,
};

pub fn first_time_boot_setup_map_set(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>, 
) {
    let pool = pool.0.clone();

    // Spawn the background task using bevy_tokio_tasks
    runtime.spawn_background_task(move |ctx| {
        first_time_boot_setup_map_set_async(pool, ctx)
    });
}

pub async fn first_time_boot_setup_map_set_async(
    pool: MySqlPool,
    mut ctx: TaskContext, 
) {
    // Count how many players exist in the player_table
    let res: (i64,) = match sqlx::query_as("SELECT COUNT(*) FROM map_set_table")
        .fetch_one(&pool)
        .await
    {
        Ok(res) => res, 
        Err(err) => {
            eprintln!("Failed to execute query: {:?}", err);
            // Run a callback on the main thread to handle the error properly
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err);
            })
            .await;
            return;
        }
    };

    println!("Number of res: {}", res.0);
    if res.0 == 0 {
        let _map_set_whole_couse = {
            // No players exist, so let's create a new one
            let map_set_id = Uuid::now_v7(); // Use the UUID directly, not as String
            let map_set_name = String::from("Standard Maps: Whole Course");
            let created = datetime!(2024-12-01 17:34:56); // Generated in UTC
            let hole_range_start = 1;
            let hole_range_end = 18;
            let file_paths: [Option<&str>; 18] = [
                Some("glb/map/level_1.glb"), 
                Some("glb/map/level_2.glb"), 
                Some("glb/map/level_3.glb"), 
                Some("glb/map/level_4.glb"), 
                Some("glb/map/level_5.glb"), 
                Some("glb/map/level_6.glb"), 
                Some("glb/map/level_7.glb"), 
                Some("glb/map/level_8.glb"), 
                Some("glb/map/level_9.glb"), 
                Some("glb/map/level_10.glb"), 
                Some("glb/map/level_11.glb"), 
                Some("glb/map/level_12.glb"), 
                Some("glb/map/level_13.glb"), 
                Some("glb/map/level_14.glb"), 
                Some("glb/map/level_15.glb"), 
                Some("glb/map/level_16.glb"), 
                Some("glb/map/level_17.glb"), 
                Some("glb/map/level_18.glb"), 
            ];

            let insert_result = sqlx::query(
            "INSERT INTO map_set_table (map_set_id, map_set_name, created, last_updated, 
                    hole_range_start, hole_range_end, file_path_level_1, file_path_level_2, 
                    file_path_level_3, file_path_level_4, file_path_level_5, file_path_level_6, 
                    file_path_level_7, file_path_level_8, file_path_level_9, file_path_level_10, 
                    file_path_level_11, file_path_level_12, file_path_level_13, file_path_level_14, 
                    file_path_level_15, file_path_level_16, file_path_level_17, file_path_level_18
                ) VALUES (
                    UUID_TO_BIN(?), ?, ?, NOW(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                )"
                )
                .bind(map_set_id.to_string())
                .bind(map_set_name)
                .bind(created)
                .bind(hole_range_start)
                .bind(hole_range_end)
                .bind(file_paths[0])
                .bind(file_paths[1])
                .bind(file_paths[2])
                .bind(file_paths[3])
                .bind(file_paths[4])
                .bind(file_paths[5])
                .bind(file_paths[6])
                .bind(file_paths[7])
                .bind(file_paths[8])
                .bind(file_paths[9])
                .bind(file_paths[10])
                .bind(file_paths[11])
                .bind(file_paths[12])
                .bind(file_paths[13])
                .bind(file_paths[14])
                .bind(file_paths[15])
                .bind(file_paths[16])
                .bind(file_paths[17])
                .execute(&pool)
                .await;

            match insert_result {
                Ok(_) => {
                    println!("Inserted new map set with ID: {:?}", map_set_id);
                }
                Err(err) => {
                    eprintln!("Failed to insert new map set: {:?}", err);
                    eprintln!("map set: {:?}", map_set_id);
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new map set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };
        
        let _map_set_front_nine = {
            // No players exist, so let's create a new one
            let map_set_id = Uuid::now_v7(); // Use the UUID directly, not as String
            let map_set_name = String::from("Standard Maps: Front Nine");
            let created = datetime!(2024-12-01 17:34:56); // Generated in UTC
            let hole_range_start = 1;
            let hole_range_end = 9;
            let file_paths: [Option<&str>; 18] = [
                Some("glb/map/level_1.glb"), 
                Some("glb/map/level_2.glb"), 
                Some("glb/map/level_3.glb"), 
                Some("glb/map/level_4.glb"), 
                Some("glb/map/level_5.glb"), 
                Some("glb/map/level_6.glb"), 
                Some("glb/map/level_7.glb"), 
                Some("glb/map/level_8.glb"), 
                Some("glb/map/level_9.glb"), 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
            ];

            let insert_result = sqlx::query(
            "INSERT INTO map_set_table (map_set_id, map_set_name, created, last_updated, 
                    hole_range_start, hole_range_end, file_path_level_1, file_path_level_2, 
                    file_path_level_3, file_path_level_4, file_path_level_5, file_path_level_6, 
                    file_path_level_7, file_path_level_8, file_path_level_9, file_path_level_10, 
                    file_path_level_11, file_path_level_12, file_path_level_13, file_path_level_14, 
                    file_path_level_15, file_path_level_16, file_path_level_17, file_path_level_18
                ) VALUES (
                    UUID_TO_BIN(?), ?, ?, NOW(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                )"
                )
                .bind(map_set_id.to_string())
                .bind(map_set_name)
                .bind(created)
                .bind(hole_range_start)
                .bind(hole_range_end)
                .bind(file_paths[0])
                .bind(file_paths[1])
                .bind(file_paths[2])
                .bind(file_paths[3])
                .bind(file_paths[4])
                .bind(file_paths[5])
                .bind(file_paths[6])
                .bind(file_paths[7])
                .bind(file_paths[8])
                .bind(file_paths[9])
                .bind(file_paths[10])
                .bind(file_paths[11])
                .bind(file_paths[12])
                .bind(file_paths[13])
                .bind(file_paths[14])
                .bind(file_paths[15])
                .bind(file_paths[16])
                .bind(file_paths[17])
                .execute(&pool)
                .await;

            match insert_result {
                Ok(_) => {
                    println!("Inserted new map set with ID: {:?}", map_set_id);
                }
                Err(err) => {
                    eprintln!("Failed to insert new map set: {:?}", err);
                    eprintln!("map set: {:?}", map_set_id);
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new map set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };
        
        let _map_set_back_nine = {
            // No players exist, so let's create a new one
            let map_set_id = Uuid::now_v7(); // Use the UUID directly, not as String
            let map_set_name = String::from("Standard Maps: Back Nine");
            let created = datetime!(2024-12-01 17:34:56); // Generated in UTC
            let hole_range_start = 1;
            let hole_range_end = 18;
            let file_paths: [Option<&str>; 18] = [
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                None, 
                Some("glb/map/level_10.glb"), 
                Some("glb/map/level_11.glb"), 
                Some("glb/map/level_12.glb"), 
                Some("glb/map/level_13.glb"), 
                Some("glb/map/level_14.glb"), 
                Some("glb/map/level_15.glb"), 
                Some("glb/map/level_16.glb"), 
                Some("glb/map/level_17.glb"), 
                Some("glb/map/level_18.glb"), 
            ];

            let insert_result = sqlx::query(
            "INSERT INTO map_set_table (map_set_id, map_set_name, created, last_updated, 
                    hole_range_start, hole_range_end, file_path_level_1, file_path_level_2, 
                    file_path_level_3, file_path_level_4, file_path_level_5, file_path_level_6, 
                    file_path_level_7, file_path_level_8, file_path_level_9, file_path_level_10, 
                    file_path_level_11, file_path_level_12, file_path_level_13, file_path_level_14, 
                    file_path_level_15, file_path_level_16, file_path_level_17, file_path_level_18
                ) VALUES (
                    UUID_TO_BIN(?), ?, ?, NOW(), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                )"
                )
                .bind(map_set_id.to_string())
                .bind(map_set_name)
                .bind(created)
                .bind(hole_range_start)
                .bind(hole_range_end)
                .bind(file_paths[0])
                .bind(file_paths[1])
                .bind(file_paths[2])
                .bind(file_paths[3])
                .bind(file_paths[4])
                .bind(file_paths[5])
                .bind(file_paths[6])
                .bind(file_paths[7])
                .bind(file_paths[8])
                .bind(file_paths[9])
                .bind(file_paths[10])
                .bind(file_paths[11])
                .bind(file_paths[12])
                .bind(file_paths[13])
                .bind(file_paths[14])
                .bind(file_paths[15])
                .bind(file_paths[16])
                .bind(file_paths[17])
                .execute(&pool)
                .await;

            match insert_result {
                Ok(_) => {
                    println!("Inserted new map set with ID: {:?}", map_set_id);
                }
                Err(err) => {
                    eprintln!("Failed to insert new map set: {:?}", err);
                    eprintln!("map set: {:?}", map_set_id);
                    ctx.run_on_main_thread(move |_ctx| {
                        info!("Failed to insert new map set in the task: {:?}", err);
                    })
                    .await;
                }
            }
        };
    };
}

pub fn client_sync_protocol_send_existing_map_sets(
    pool: Res<DatabasePool>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let pool = pool.0.clone();

    // Spawn the background task using bevy_tokio_tasks
    runtime.spawn_background_task(move |ctx| {
        send_existing_map_sets_async(pool, ctx)
    });
}

pub async fn send_existing_map_sets_async(
    pool: MySqlPool,
    mut ctx: TaskContext
) {
    let query = "SELECT * FROM map_set_table";

    let map_sets: Vec<MapSet> = match query_as::<_, MapSet>(query).fetch_all(&pool).await {
        Ok(map_sets) => {
            let map_sets_clone = map_sets.clone();
            ctx.run_on_main_thread(move |ctx| {
                if let Some(mut map_sets_resource) = ctx.world.get_resource_mut::<MapSets>() {
                    for map_set in map_sets_clone.iter() {
                        map_sets_resource.map_sets.push(map_set.clone());
                    }
                } else {
                    info!("Failed to access map_sets_resource");
                }        
            });
            map_sets
        },
        Err(err) => {
            error!("Failed to execute query: {:?}", err);

            // Run a callback on the main thread to handle the error properly
            ctx.run_on_main_thread(move |_ctx| {
                info!("Failed to execute query in the task: {:?}", err);
            })
            .await;
            return; // Exit early since the query failed.
        },
    };

    ctx.run_on_main_thread(move |ctx| {
        if let Some(mut socket) = ctx.world.get_resource_mut::<MatchboxSocket<SingleChannel>>() {
            let peers: Vec<_> = socket.connected_peers().collect();

            // Send the summary first
            let summary: Vec<(Uuid, OffsetDateTime)> = map_sets
                .iter()
                .map(|ms| (ms.map_set_id.clone(), ms.last_updated))
                .collect();

            match encode::to_vec(&summary) {
                Ok(serialized_summary) => {
                    for peer in peers {
                        socket.send(serialized_summary.clone().into(), peer);
                    }
                    info!("Finished sending map set summary to peers.");
                }
                Err(ser_err) => {
                    error!("Failed to serialize map sets summary for sending: {:?}", ser_err);
                }
            }

            // Wait for a request from the client to send the full map sets
            // This can be handled when we receive a specific request message from a client
        } else {
            info!("Failed to access matchbox resource");
        }
    })
    .await;
}

// pub async fn send_existing_map_sets_async(
//     pool: MySqlPool,
//     mut ctx: TaskContext
// ) {
//     let query = "SELECT * FROM map_set_table";

//     let map_sets: Vec<MapSet> = match query_as::<_, MapSet>(query).fetch_all(&pool).await {
//         Ok(map_sets) => {
//             // info!("Retrieved map sets: {:?}", map_sets);
//             map_sets
//         }
//         Err(err) => {
//             error!("Failed to execute query: {:?}", err);

//             // Run a callback on the main thread to handle the error properly
//             ctx.run_on_main_thread(move |_ctx| {
//                 info!("Failed to execute query in the task: {:?}", err);
//             })
//             .await;
//             return; // Exit early since the query failed.
//         }
//     };

//     ctx.run_on_main_thread(move |ctx| {
//         if let Some(mut socket) = ctx.world.get_resource_mut::<MatchboxSocket<SingleChannel>>() {
//             let peers: Vec<_> = socket.connected_peers().collect();

//             let summary: Vec<(Uuid, OffsetDateTime)> = map_sets
//                 .iter()
//                 .map(|ms| (ms.map_set_id.clone(), ms.last_updated)) // Use timestamp or version
//                 .collect();

//             match encode::to_vec(&summary) {
//                 Ok(serialized_summary) => {
//                     // info!("Serialization successful, attempting to send to peers...");
//                     for peer in peers {
//                         // info!("Sending map_sets update to {peer}");
//                         socket.send(serialized_summary.clone().into(), peer);
//                     }
//                     info!("Finished sending updates.");
//                 }
//                 Err(ser_err) => {
//                     error!("Failed to serialize map sets summary for sending: {:?}", ser_err);
//                 }
//             }

//             let peers: Vec<_> = socket.connected_peers().collect();
//             match encode::to_vec(&map_sets) {
//                 Ok(serialized_map_sets) => {
//                     // info!("Serialization successful, attempting to send to peers...");
//                     for peer in peers {
//                         // info!("Sending map_sets update to {peer}");
//                         socket.send(serialized_map_sets.clone().into(), peer);
//                     }
//                     info!("Finished sending updates.");
//                 }
//                 Err(ser_err) => {
//                     error!("Failed to serialize map sets for sending: {:?}", ser_err);
//                 }
//             }
//         } else {
//             info!("Failed to access matchbox resource");
//         }
//     })
//     .await;
// }