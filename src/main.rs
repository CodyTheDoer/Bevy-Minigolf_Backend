//! Runs both signaling with server/client topology and runs the host in the same process
//!
//! Sends messages periodically to all connected clients.

use bevy::{prelude::*, 
    app::ScheduleRunnerPlugin, 
    log::LogPlugin, 
    input::common_conditions::*,
    input::InputPlugin,
    // time::common_conditions::on_timer,
    utils::Duration,
};

use bevy_matchbox::{matchbox_signaling::SignalingServer, prelude::*};
use std::net::{Ipv4Addr, SocketAddrV4};

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins)
        .insert_resource(RunTrigger::new())
        // .add_systems(Update, send_message.run_if(on_timer(Duration::from_secs(5))))
        .add_systems(Startup, (start_signaling_server, start_host_socket).chain())
        .add_systems(Update, temp_interface.run_if(input_just_released(KeyCode::ShiftLeft)))
        .add_systems(Update, receive_messages)
        .add_systems(Update, network_get_client_state_game.run_if(|run_trigger: Res<RunTrigger>|run_trigger.network_get_client_state_game()))
        .run();
}

fn start_signaling_server(mut commands: Commands) {
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

fn start_host_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/minigolf");
    commands.insert_resource(socket);
}

fn send_game_state_update(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = "StateGameConnection::Online";
        info!("Sending game_state update: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}

fn receive_messages(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    let mut update_received = false;
    for (_id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => info!("Received message: {message:?}"),
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
        update_received = true;
    }
    
    if update_received == true {
        send_game_state_update(socket);
        update_received = false;
    }
}

fn network_get_client_state_game(
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

#[derive(Debug, Resource)]
pub struct RunTrigger{
    network_get_client_state_game: bool,
}

impl RunTrigger {
    pub fn new() -> Self {
        Self{
            network_get_client_state_game: false,
        }
    }

    pub fn get(&self, target: &str) -> bool {
        match target {
            "network_get_client_state_game" => {
                self.network_get_client_state_game
            },
            _ => {false},
        }
    }

    pub fn set_target(&mut self, target: &str, state: bool) {
        match target {
            "network_get_client_state_game" => {
                self.network_get_client_state_game = state;
                info!("response: network_get_client_state_game: {}", self.get("network_get_client_state_game"));  
            },
            _ => {},
        }
    }

    pub fn network_get_client_state_game(&self) -> bool {
        self.network_get_client_state_game
    }
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