use bevy::prelude::*;

pub struct SessionPlugin;

impl Plugin for SessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Session>();
    }
}

/// Actors are stateful entities in the context of a [Session].
/// The assigned actor id is valid across clients and the network.
#[derive(Component)]
pub struct Actor {
    id: u64,
}

impl Actor {
    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Resource, Default)]
pub struct Session {
    next_actor_id: u64,
}

impl Session {
    pub fn actor(&mut self) -> Actor {
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        Actor { id }
    }
}
