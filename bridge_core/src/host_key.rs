use std::fmt::Debug;

use once_cell::sync::Lazy;
use ring::{
    pkcs8::Document,
    rand::SystemRandom,
    signature::{EcdsaKeyPair, Ed25519KeyPair, Signature, ECDSA_P256_SHA256_ASN1_SIGNING},
};

use crate::error::Result;

static RNG: Lazy<SystemRandom> = Lazy::new(|| SystemRandom::new());

pub enum HostKey {
    Ecdsa256(EcdsaKeyPair),
    Ed25519(Ed25519KeyPair),
    // I don't want to use any crates other than `ring`
    // to generate the RSA key pair for now,
    // Waiting on: https://github.com/briansmith/ring/issues/219
    // or maybe https://github.com/briansmith/ring/pull/733 to get reopened
    // RsaSha256(RsaKeyPair),
}

impl HostKey {
    pub fn algorithm(&self) -> HostKeyAlgorithm {
        match self {
            HostKey::Ecdsa256(_) => HostKeyAlgorithm::Ecdsa256,
            HostKey::Ed25519(_) => HostKeyAlgorithm::Ed25519,
            // HostKey::RsaSha256(_) => HostKeyAlgorithm::RsaSha256,
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Result<Signature> {
        Ok(match self {
            HostKey::Ecdsa256(key_pair) => key_pair.sign(&*RNG, msg)?,
            HostKey::Ed25519(key_pair) => key_pair.sign(msg),
        })
    }

    pub fn algorithms() -> &'static [HostKeyAlgorithm] {
        &HOST_KEYS
    }
}

impl Debug for HostKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HostKey ({:?})", self.algorithm())
    }
}

pub static HOST_KEYS: [HostKeyAlgorithm; 2] =
    [HostKeyAlgorithm::Ecdsa256, HostKeyAlgorithm::Ed25519];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HostKeyAlgorithm {
    Ecdsa256,
    Ed25519,
    // RsaSha256,
}

impl HostKeyAlgorithm {
    pub fn id(&self) -> &'static str {
        match self {
            HostKeyAlgorithm::Ecdsa256 => "ecdsa-sha2-nistp256",
            HostKeyAlgorithm::Ed25519 => "ssh-ed25519",
            // HostKeyAlgorithm::RsaSha256 => "rsa-sha2-256",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HostKeyAlgorithm::Ecdsa256 => "ecdsa_nistp256",
            HostKeyAlgorithm::Ed25519 => "ed25519",
            // HostKeyAlgorithm::RsaSha256 => "rsa_sha256",
        }
    }

    pub fn generate_key(&self) -> Result<Document> {
        Ok(match self {
            HostKeyAlgorithm::Ecdsa256 => {
                EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &*RNG)?
            }
            HostKeyAlgorithm::Ed25519 => Ed25519KeyPair::generate_pkcs8(&*RNG)?,
        })
    }
}
