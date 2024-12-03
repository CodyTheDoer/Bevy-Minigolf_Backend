use bevy::prelude::*;

use bevy_matchbox::{matchbox_signaling::SignalingServer, prelude::*};
use std::net::{Ipv4Addr, SocketAddrV4};
use uuid::Uuid;

use rmp_serde::encode;

use crate::{
    ClientProtocol,
    MapSets, 
    PlayerInfo,
    PlayerInfoStorage, 
    RunTrigger,
};

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

pub fn start_host_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
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

pub fn receive_client_requests(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut map_sets: ResMut<MapSets>,
    mut player_info_storage: ResMut<PlayerInfoStorage>,
    mut run_trigger: ResMut<RunTrigger>,
    client_protocol: Res<State<ClientProtocol>>, 
    mut set_client_protocol: ResMut<NextState<ClientProtocol>>,
) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    let mut send_client_state_update_bool = false;

    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => {
                info!("Received message: {:?}", message.clone());
                        
                // Trim surrounding parentheses and split by the comma
                let trimmed_message: &str = message.trim_start_matches('(').trim_end_matches(')');
                let parts: Vec<&str> = trimmed_message.splitn(2, ", ").collect();
        
                // Ensure that the split resulted in the expected two parts (protocol and payload)
                if parts.len() == 2 {
                    let protocol = parts[0].trim();
                    let payload = parts[1].trim();

                    match protocol {
                        "ClientProtocol::InitPlayerConnection" => {    
                            let trimmed_message = payload.trim_start_matches('(').trim_end_matches(')');
                            let parts: Vec<&str> = trimmed_message.splitn(3, ", ").collect();
                        
                            if parts.len() == 3 {
                                let player_id = parts[0].trim();
                                let username = parts[1].trim();
                                let email = parts[2].trim();
                                
                                info!("Init ClientProtocol::InitPlayerConnection: ({:?}, {:?}, {:?},)", player_id, username, email);
                                
                                let player = PlayerInfo::from_vec_str(parts);
                                player_info_storage.add(player);
                                run_trigger.set_target("db_pipeline_player_init", true);
                                send_client_state_update_bool = true;
                            };
                        },
                        "REQUEST_FULL_MAP_SETS" => {
                            match encode::to_vec(&*map_sets) {
                                Ok(serialized_map_sets) => {
                                    socket.send(serialized_map_sets.into(), _id);
                                    info!("Sent full map sets to peer: {:?}", _id);
                                }
                                Err(ser_err) => {
                                    error!("Failed to serialize map sets for sending: {:?}", ser_err);
                                }
                            }
                        },
                        _ => {},
                    }    
                };
            }
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }

    if send_client_state_update_bool == true {
        if let Some(player) = player_info_storage.get_last_player_id_string() {
            set_client_protocol.set(ClientProtocol::InitPlayerConnection);
            send_client_state_update(player, socket, client_protocol);
            // set_client_protocol.set(ClientProtocol::Idle);
        };
    }
}

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
