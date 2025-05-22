use bevy::prelude::*;
use std::iter;

/// Register commands as one-shot-systems, and call them through events.
pub struct CommandPlugin;

/// Shortcut for the command system argument input
pub type Args = In<Vec<String>>;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandEvent>();
    }
}

pub trait CommandAppExt {
    fn add_command<M>(&mut self, name: &str, system: impl IntoSystem<Args, (), M> + 'static);
}

#[derive(Debug, Event, Deref, DerefMut)]
pub struct CommandEvent(Vec<String>);

impl CommandEvent {
    pub fn new(name: &str, args: Vec<&str>) -> Self {
        Self(iter::once(name).chain(args).map(|a| a.to_owned()).collect())
    }
}

impl CommandAppExt for App {
    fn add_command<M>(&mut self, name: &str, system: impl IntoSystem<Args, (), M> + 'static) {
        let name = name.to_owned();
        let system_id = self.register_system(system);

        let update_system = move |mut commands: Commands, mut reader: EventReader<CommandEvent>| {
            reader
                .read()
                .filter(|e| e.first() == Some(&name))
                .for_each(|e| commands.run_system_with(system_id, (*e).clone()));
        };

        self.add_systems(Update, update_system);
    }
}
