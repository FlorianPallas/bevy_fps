use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{
    ConnectionEvent, ConnectionLostEvent, QuinnetServer, QuinnetServerPlugin,
    ServerEndpointConfiguration,
};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use shared::consts::GAME_PORT;

pub struct NetPlugin;

#[derive(Component, Deref)]
#[component(immutable)]
struct Client {
    pub id: u64,
}

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetServerPlugin::default());
        app.add_systems(Startup, start_listening);
        app.add_systems(Update, handle_server_events);
    }
}

fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(
            ServerEndpointConfiguration::from_string(format!("[::]:{GAME_PORT}").as_str()).unwrap(),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "::1".to_string(),
            },
            ChannelsConfiguration::default(),
        )
        .unwrap();
}

fn handle_server_events(
    mut connection_events: EventReader<ConnectionEvent>,
    mut connection_lost_events: EventReader<ConnectionLostEvent>,
    mut commands: Commands,
    clients: Query<(Entity, &Client)>,
) {
    for &ConnectionEvent { id } in connection_events.read() {
        info!("Client {id} connected");
        commands.spawn(Client { id });
    }

    for &ConnectionLostEvent { id } in connection_lost_events.read() {
        info!("Client {id} disconnected");
        let Some((entity, _)) = clients.into_iter().find(|&(_, c)| c.id == id) else {
            warn!("Could not find entity for client {id}");
            return;
        };
        commands.entity(entity).despawn();
    }
}
