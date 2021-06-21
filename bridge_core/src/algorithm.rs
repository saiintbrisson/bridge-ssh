pub trait SshAlgorithm {
    /// Returns the SSH id.
    /// For example, `HostKeyAlgorithm::Ed25519` has the id `ssh-ed25519`.
    fn id(&self) -> &'static str;

    /// Returns the algorithm's friendly name.
    fn name(&self) -> &'static str;
}
