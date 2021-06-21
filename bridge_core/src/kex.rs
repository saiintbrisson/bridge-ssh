use crate::algorithm::SshAlgorithm;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KexAlgorithm {
    Curve25519,
    EcdhP256,
}

impl SshAlgorithm for KexAlgorithm {
    fn id(&self) -> &'static str {
        match self {
            KexAlgorithm::Curve25519 => "curve25519-sha256",
            KexAlgorithm::EcdhP256 => "ecdh-sha2-nistp256",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            KexAlgorithm::Curve25519 => "curve25519",
            KexAlgorithm::EcdhP256 => "ecdh_p256",
        }
    }
}
