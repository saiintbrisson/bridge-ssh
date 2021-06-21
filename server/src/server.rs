use std::collections::HashMap;

use bridge_core::host_key::{HostKey, HostKeyAlgorithm};
use tokio::net::TcpListener;

use crate::settings::Settings;

pub struct Server {
    settings: Settings,
    keys: HashMap<HostKeyAlgorithm, HostKey>,
}

impl Server {
    pub fn new(settings: Settings, keys: HashMap<HostKeyAlgorithm, HostKey>) -> Self {
        Self { settings, keys }
    }
}

impl Server {
    /// Get a reference to the server's settings.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Get a reference to the server's keys.
    pub fn keys(&self) -> &HashMap<HostKeyAlgorithm, HostKey> {
        &self.keys
    }
}

pub async fn init_server(server: Server) -> anyhow::Result<()> {
    let listener = TcpListener::bind(server.settings.server().addr()).await?;
    info!("listening for connections on {}", listener.local_addr()?);

    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(crate::session::init_connection(stream, addr));
    }
}
