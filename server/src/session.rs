use std::net::SocketAddr;

use bridge_core::ssh_id::SshId;
use once_cell::sync::Lazy;
use tokio::net::TcpStream;

static SERVER_ID: Lazy<SshId> = Lazy::new(|| SshId::new("BridgeSSH_v0.1.0"));

pub struct Session {
    ssh_id: SshId,
}

pub async fn init_connection(stream: TcpStream, addr: SocketAddr) {
    trace!("received connection from {}", addr);

    let (mut src, mut dst) = tokio::io::split(stream);
    let ssh_id = match SERVER_ID.handle_id(&mut src, &mut dst).await {
        Ok(ssh_id) => ssh_id,
        Err(err) => {
            warn!("failed to handle ssh id string: {}", err);
            return;
        }
    };
}
