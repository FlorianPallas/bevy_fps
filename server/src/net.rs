use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{
    ConnectionEvent, ConnectionLostEvent, QuinnetServer, QuinnetServerPlugin,
    ServerEndpointConfiguration,
};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use shared::consts::GAME_PORT;

pub struct NetPlugin;

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
) {
    for ev in connection_events.read() {
        info!("Client {} connected", ev.id);
    }

    for ev in connection_lost_events.read() {
        info!("Client {} lost connection", ev.id);
    }
}
