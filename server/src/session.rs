use std::net::SocketAddr;

use bridge_core::ssh_id::SshId;
use once_cell::sync::Lazy;
use tokio::net::TcpStream;

static SERVER_ID: Lazy<SshId> = Lazy::new(|| SshId::new("BridgeSSH_v0.1.0"));

pub struct Session {
    ssh_id: SshId,
}

pub async fn init_connection(stream: TcpStream, addr: SocketAddr) {
}
