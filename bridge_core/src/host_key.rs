use std::fmt::Debug;

use once_cell::sync::Lazy;
use ring::{
    pkcs8::Document,
    rand::SystemRandom,
    signature::{EcdsaKeyPair, Ed25519KeyPair, Signature, ECDSA_P256_SHA256_ASN1_SIGNING},
};

use crate::{algorithm::SshAlgorithm, error::Result};

static RNG: Lazy<SystemRandom> = Lazy::new(|| SystemRandom::new());

pub enum HostKey {
    EcdsaP256(EcdsaKeyPair),
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
            HostKey::EcdsaP256(_) => HostKeyAlgorithm::EcdsaP256,
            HostKey::Ed25519(_) => HostKeyAlgorithm::Ed25519,
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Result<Signature> {
        Ok(match self {
            HostKey::EcdsaP256(key_pair) => key_pair.sign(&*RNG, msg)?,
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
    [HostKeyAlgorithm::EcdsaP256, HostKeyAlgorithm::Ed25519];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HostKeyAlgorithm {
    EcdsaP256,
    Ed25519,
}

impl HostKeyAlgorithm {
    pub fn generate_key(&self) -> Result<Document> {
        Ok(match self {
            HostKeyAlgorithm::EcdsaP256 => {
                EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, &*RNG)?
            }
            HostKeyAlgorithm::Ed25519 => Ed25519KeyPair::generate_pkcs8(&*RNG)?,
        })
    }
}

impl SshAlgorithm for HostKeyAlgorithm {
    fn id(&self) -> &'static str {
        match self {
            HostKeyAlgorithm::EcdsaP256 => "ecdsa-sha2-nistp256",
            HostKeyAlgorithm::Ed25519 => "ssh-ed25519",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            HostKeyAlgorithm::EcdsaP256 => "ecdsa_p256",
            HostKeyAlgorithm::Ed25519 => "ed25519",
        }
    }
}
