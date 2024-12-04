use bevy::prelude::*;

use bevy_matchbox::{matchbox_signaling::SignalingServer, prelude::*};
use std::{
    net::{
        Ipv4Addr, 
        SocketAddrV4
    }, 
    str::FromStr,
    time::{
        Duration,
        Instant,
    },
};
use regex::Regex;
use rmp_serde::decode;
use serde_json;
use uuid::Uuid;

use crate::{
    ClientProtocol,
    ConnectedPlayers,
    HeartBeatMonitorTimer,
    PacketAllStates,
    PacketHeartBeat,
    PlayerInfo,
    PlayerInfoStorage,
    RunTrigger,
    SyncTriggerIndexEvent,
};

pub fn client_run_trigger(
    trigger: ResMut<RunTrigger>,
    mut event_reader: EventReader<SyncTriggerIndexEvent>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
) {
    for event in event_reader.read() {
        let target_idx =  trigger.get_trigger_idx();
        let triggers = trigger.get_triggers_ref();
        let trigger = triggers[target_idx].as_str();
        let peers: Vec<_> = socket.connected_peers().collect();
        for peer in peers {
            let message = format!("({}, RunTrigger({}))", event.player_id.clone(), trigger);
            info!("Sending sync_player_id_init_system update: {message:?} to {peer}");
            socket.send(message.as_bytes().into(), peer);
        }
    }
}

pub fn start_signaling_server(mut commands: Commands) {
    info!("Starting signaling server");
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3536);
    let signaling_server = MatchboxServer::from(
        SignalingServer::client_server_builder(addr)
            .on_connection_request(|connection| {
                info!("Connecting: {connection:?}");
                Ok(true) // Allow all connections
            })
            .on_id_assignment(|(socket, id)| info!("{socket} received {id}"))
            .on_host_connected(|id| info!("Host joined: {id}"))
            .on_host_disconnected(|id| info!("Host left: {id}"))
            .on_client_connected(|id| info!("Client joined: {id}"))
            .on_client_disconnected(|id| info!("Client left: {id}"))
            .cors()
            .trace()
            .build(),
    );
    commands.insert_resource(signaling_server);
}

pub fn heartbeat_monitor_system(
    time: Res<Time>,
    mut timer: ResMut<HeartBeatMonitorTimer>,
    mut connected_players: ResMut<ConnectedPlayers>,
) {
    // Check if the timer has finished
    if timer.0.tick(time.delta()).finished() {
        info!("heartbeat_monitor_system:");
        let timeout_duration = Duration::from_secs(15);
        let mut players = connected_players.players.lock().unwrap();
        let now = Instant::now();

        // Find and remove players who have not sent a heartbeat in the last `timeout_duration`
        players.retain(|player_id, player_status| {
            let is_active = now.duration_since(player_status.last_heartbeat) < timeout_duration;
            if !is_active {
                warn!("Removing player {} due to timeout.", player_id);
            }
            is_active
        });
    }
}

pub fn start_host_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}

pub fn receive_client_requests(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut connected_players: ResMut<ConnectedPlayers>,
    mut player_info_storage: ResMut<PlayerInfoStorage>,
    mut run_trigger: ResMut<RunTrigger>,
    client_protocol: Res<State<ClientProtocol>>, 
    mut set_client_protocol: ResMut<NextState<ClientProtocol>>,
) {
    // Regex to match the command and payload format after deserialization
    let re = Regex::new(r#"^\(([^,]+),\s*\((.*)\)\)$"#).unwrap();

    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    let mut send_client_state_update_bool = false;

    for (_id, message) in socket.receive() {
        // Deserialize the entire message from MessagePack binary to String
        match decode::from_slice::<String>(&message) {
            Ok(deserialized_message) => {
                info!("Received deserialized message: {:?}", deserialized_message);

                // Apply regex to the deserialized message
                if let Some(caps) = re.captures(&deserialized_message) {
                    let command = caps.get(1).map_or("", |m| m.as_str());
                    let payload = caps.get(2).map_or("", |m| m.as_str());

                    match command {
                        "InitPlayerConnection" => {
                            let parts: Vec<&str> = payload.splitn(3, ", ").collect();

                            if parts.len() == 3 {
                                let player_id = parts[0].trim();
                                let username = parts[1].trim();
                                let email = parts[2].trim();

                                info!(
                                    "Init InitPlayerConnection: ({:?}, {:?}, {:?})",
                                    player_id, username, email
                                );

                                let player = PlayerInfo::new(player_id.to_string(), username.to_string(), email.to_string());
                                player_info_storage.add(player);
                                connected_players.add_player_string(String::from(player_id));
                                run_trigger.set_target("db_pipeline_player_init", true);
                                send_client_state_update_bool = true;
                            };
                        }
                        "PacketAllStates" => {
                            info!("Packet: AllStates Payload: {}", payload.clone());
                            
                            // Deserialize the payload using JSON
                            // If payload itself is JSON as a string, deserialize it again
                            match serde_json::from_str::<PacketAllStates>(payload) {
                                Ok(all_states) => {
                                    info!("Received PacketAllStates for peer {:?}: {:?}", _id, all_states);
                                    // Handle the deserialized PacketAllStates data as needed
                                }
                                Err(err) => {
                                    error!("Failed to deserialize PacketAllStates from JSON: {:?}", err);
                                }
                            }
                        }
                        "PacketHeartBeat" => {
                            info!("Packet: HeartBeat Payload: {}", payload.clone());
                            
                            // Deserialize the payload using JSON
                            // If payload itself is JSON as a string, deserialize it again
                            match serde_json::from_str::<PacketHeartBeat>(payload) {
                                Ok(heart_beat) => {
                                    info!("Received PacketHeartBeat for peer {:?}: {:?}", _id, heart_beat);
                                    let id = heart_beat.player_id;
                                    let uid = Uuid::from_str(&id);
                                    connected_players.update_heartbeat(&uid.unwrap());
                                    }
                                Err(err) => {
                                    error!("Failed to deserialize PacketAllStates from JSON: {:?}", err);
                                }
                            }
                        }
                        _ => {
                            info!("Unknown command received: {}", command);
                        }
                    }
                } else {
                    info!("Invalid message format: {}", deserialized_message);
                }
            }
            Err(err) => {
                error!("Failed to deserialize message from MessagePack: {:?}", err);
            }
        }
    }

    if send_client_state_update_bool {
        if let Some(player) = player_info_storage.get_last_player_id_string() {
            set_client_protocol.set(ClientProtocol::InitPlayerConnection);
            send_client_state_update(player, socket, client_protocol);
        }
    }
}

pub fn send_client_state_update(
    player_id: String,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    client_protocol: Res<State<ClientProtocol>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = format!("({}, {:?})", 
            player_id,
            client_protocol.get(),
        );
        info!("Sending game_state update: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}


                        // "REQUEST_FULL_MAP_SETS" => {
                        //     match encode::to_vec(&*map_sets) {
                        //         Ok(serialized_map_sets) => {
                        //             socket.send(serialized_map_sets.into(), _id);
                        //             info!("Sent full map sets to peer: {:?}", _id);
                        //         }
                        //         Err(ser_err) => {
                        //             error!("Failed to serialize map sets for sending: {:?}", ser_err);
                        //         }
                        //     }
                        // },

pub fn network_get_client_state_game(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut run_trigger: ResMut<RunTrigger>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let trigger = "network_get_client_state_game";
        info!("Sending message: {trigger:?} to {peer}");
        socket.send(trigger.as_bytes().into(), peer);
    }
    run_trigger.set_target("network_get_client_state_game", false);
}
