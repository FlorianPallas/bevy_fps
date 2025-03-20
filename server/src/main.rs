use bevy::prelude::*;
use shared::plugins::SharedPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SharedPlugins)
        .add_systems(Startup, shared::scenes::example::setup)
        .run();
}
