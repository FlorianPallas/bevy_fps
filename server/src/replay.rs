use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::session::Actor;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub struct ReplayPlugin {
    pub path: PathBuf,
}

impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();
        app.insert_resource(Replay { file });
        app.add_systems(FixedPostUpdate, write_snapshot);
    }
}

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

#[derive(Resource)]
pub struct Replay {
    pub file: File,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub actors: Vec<(u64, Transform)>,
}

fn write_snapshot(mut replay: ResMut<Replay>, q_actors: Query<(&Actor, &Transform)>) {
    let actors = q_actors
        .iter()
        .map(|(actor, transform)| (actor.id(), *transform))
        .collect::<Vec<_>>();

    let frame = Frame { actors };

    bincode::serde::encode_into_std_write(frame, &mut replay.file, BINCODE_CONFIG).unwrap();
}
