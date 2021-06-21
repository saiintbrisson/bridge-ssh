use std::{fmt::Debug, io};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct SshId {
    buf: Vec<u8>,
}

impl SshId {
    pub fn new(software: &str) -> Self {
        Self {
            buf: format!("SSH-2.0-{}\r\n", software).into_bytes(),
        }
    }

    /// Get a reference to the ssh id's buf escaped.
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }

    /// Get a reference to the ssh id's buf.
    pub fn buf_trimmed(&self) -> &[u8] {
        &self.buf[..self.buf.len() - 2]
    }
}

impl SshId {
    pub async fn from_reader<R>(src: &mut R) -> io::Result<Self>
    where
        R: AsyncRead + Unpin,
    {
        let mut buf = [0u8; 255];
        let end = src.read(&mut buf).await?;
        let buf = buf[..end].to_vec();

        debug!("received ssh id {:?}", String::from_utf8_lossy(&buf));

        if &buf[..8] != b"SSH-2.0-" || &buf[buf.len() - 2..] != b"\r\n" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid id string",
            ));
        }

        Ok(Self { buf })
    }

    pub async fn to_writer<W>(&self, dst: &mut W) -> io::Result<usize>
    where
        W: AsyncWrite + Unpin,
    {
        dst.write(self.buf()).await
    }

    pub async fn handle_id<R, W>(&self, src: &mut R, dst: &mut W) -> io::Result<Self>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        dst.write(self.buf()).await?;
        Self::from_reader(src).await
    }
}

impl Debug for SshId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SshId ({:?})",
            String::from_utf8_lossy(self.buf_trimmed())
        )
    }
}
