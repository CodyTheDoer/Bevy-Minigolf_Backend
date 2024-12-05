use bevy::prelude::*;

use std::time::{
        Duration,
        Instant,
};

use crate::{
    ConnectedPlayers,
    HeartBeatMonitorTimer,
};

pub fn heartbeat_monitor_system(
    time: Res<Time>,
    mut timer: ResMut<HeartBeatMonitorTimer>,
    connected_players: ResMut<ConnectedPlayers>,
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