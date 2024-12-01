use bevy::prelude::*;

use bevy_matchbox::{matchbox_signaling::SignalingServer, prelude::*};
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::RunTrigger;

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

pub fn send_game_state_update(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = "StateGameConnection::Online";
        info!("Sending game_state update: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}

pub fn receive_messages(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    let mut update_received = false;
    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => {
                info!("Received message: {message:?}");

            },
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
        update_received = true;
    }
    
    if update_received == true {
        send_game_state_update(socket);
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
