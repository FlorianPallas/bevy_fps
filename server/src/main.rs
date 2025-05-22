mod net;
mod replay;

use crate::net::NetPlugin;
use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use shared::consts::TICK_RATE;
use shared::plugins::SharedPlugins;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / TICK_RATE as f64,
            ))),
        )
        .add_plugins(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(SharedPlugins)
        // .add_plugins(ReplayPlugin {
        //     path: format!("./replays/{}.bin", Utc::now().timestamp()).into(),
        // })
        .add_plugins(NetPlugin)
        .run();
}
