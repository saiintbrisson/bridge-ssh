use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    server: ServerSettings,
    #[serde(default = "default_keys_dir")]
    keys_dir: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Config::new()
            .with_merged(Environment::new().separator("_"))?
            .try_into()
    }
}

impl Settings {
    /// Get a reference to the settings' server.
    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    /// Get a reference to the settings' keys dir.
    pub fn keys_dir(&self) -> &str {
        self.keys_dir.as_str()
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    #[serde(default = "default_addr")]
    addr: String,
    #[serde(default = "default_max_clients")]
    max_clients: u16,
}

impl ServerSettings {
    /// Get a reference to the server settings's addr.
    pub fn addr(&self) -> &str {
        self.addr.as_str()
    }

    /// Get a reference to the server settings's max clients.
    pub fn max_clients(&self) -> &u16 {
        &self.max_clients
    }
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            addr: default_addr(),
            max_clients: default_max_clients(),
        }
    }
}

fn default_keys_dir() -> String {
    "./keys/".into()
}

fn default_addr() -> String {
    "0.0.0.0:22".into()
}

fn default_max_clients() -> u16 {
    u16::max_value()
}

use bridge_core::{
    error::Result as BridgeResult,
    host_key::{HostKey, HostKeyAlgorithm},
};
use ring::signature::{EcdsaKeyPair, Ed25519KeyPair, ECDSA_P256_SHA256_ASN1_SIGNING};

use std::{collections::HashMap, path::Path};

pub fn load_keys<P>(dir: P) -> BridgeResult<HashMap<HostKeyAlgorithm, HostKey>>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(&dir)?;

    let mut host_keys = HashMap::with_capacity(HostKey::algorithms().len());
    for algorithm in HostKey::algorithms() {
        host_keys.insert(*algorithm, load_or_generate(algorithm, &dir)?);
    }

    Ok(host_keys)
}

fn load_or_generate<P>(algorithm: &HostKeyAlgorithm, dir: P) -> BridgeResult<HostKey>
where
    P: AsRef<Path>,
{
    use std::fs::{read, write};

    let name = algorithm.name();
    let mut path = dir.as_ref().to_path_buf();

    path.push(format!("ssh_{}_key", name));
    let private_path = path.with_extension("pem");

    let pem::Pem { contents, .. } = match private_path.exists().then(|| read(private_path)) {
        Some(content) => pem::parse(content?)?,
        None => {
            info!("{:?} host key not present, generating...", algorithm);
            let pem = pem::Pem {
                contents: algorithm.generate_key()?.as_ref().to_vec(),
                tag: format!("{} PRIVATE KEY", algorithm.name().to_string()),
            };
            write(path.with_extension("pem"), pem::encode(&pem))?;
            pem
        }
    };

    Ok(match algorithm {
        HostKeyAlgorithm::Ecdsa256 => HostKey::Ecdsa256(EcdsaKeyPair::from_pkcs8(
            &ECDSA_P256_SHA256_ASN1_SIGNING,
            &contents[..],
        )?),
        HostKeyAlgorithm::Ed25519 => HostKey::Ed25519(Ed25519KeyPair::from_pkcs8(&contents[..])?),
    })
}
