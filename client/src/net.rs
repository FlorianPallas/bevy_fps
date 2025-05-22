use crate::command::{Args, CommandAppExt, CommandEvent};
use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::{
    ClientEndpointConfiguration, ConnectionEvent, ConnectionFailedEvent, ConnectionLostEvent,
};
use bevy_quinnet::client::{QuinnetClient, QuinnetClientPlugin};
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use shared::consts::GAME_PORT;
use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6, ToSocketAddrs};

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(QuinnetClientPlugin::default());
        app.add_systems(Update, handle_client_events);

        // Connect to localhost at startup
        app.add_systems(Startup, |mut evw: EventWriter<CommandEvent>| {
            evw.write(CommandEvent::new(
                "connect",
                vec![format!("localhost:{GAME_PORT}").as_str()],
            ));
        });

        app.add_command(
            "connect",
            |In(args): Args, mut client: ResMut<QuinnetClient>| {
                let server_addr = args[1].as_str();

                let server_addr = server_addr
                    .to_socket_addrs()
                    .expect("Could not resolve server address")
                    .next()
                    .expect("Server address not valid");

                client
                    .open_connection(
                        ClientEndpointConfiguration::from_addrs(
                            server_addr,
                            SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
                        ),
                        CertificateVerificationMode::SkipVerification,
                        ChannelsConfiguration::default(),
                    )
                    .expect("Could not open connection");
            },
        );

        app.add_command(
            "disconnect",
            |_args: Args, mut client: ResMut<QuinnetClient>| {
                client.close_all_connections();
            },
        )
    }
}

fn handle_client_events(
    mut connection_events: EventReader<ConnectionEvent>,
    mut connection_failed_events: EventReader<ConnectionFailedEvent>,
    mut connection_lost_events: EventReader<ConnectionLostEvent>,
) {
    for ev in connection_events.read() {
        info!("Connected to server as {}", ev.client_id.unwrap());
    }

    for ev in connection_failed_events.read() {
        info!("Connection failed: {:?}", ev.err);
    }

    for _ in connection_lost_events.read() {
        info!("Connection lost");
    }
}
