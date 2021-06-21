use std::collections::HashMap;

use bridge_core::host_key::{HostKey, HostKeyAlgorithm};

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

