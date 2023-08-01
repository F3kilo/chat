use crate::error::{ConnectError, ConnectResult};
use crate::{RecvResult, SendResult};
use std::io;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Represent STP server, that can accept incoming connections.
pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    /// Binds server to specified socket.
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    /// Wait for incoming connection.
    pub async fn accept(&self) -> ConnectResult<StpConnection> {
        let (connection, _) = self.tcp.accept().await?;
        Self::try_handshake(connection).await
    }

    async fn try_handshake(mut stream: TcpStream) -> ConnectResult<StpConnection> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"serv").await?;
        Ok(StpConnection { stream })
    }
}

pub type BindResult = Result<StpServer, BindError>;

/// Bind to socket error
#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

/// Represent connection from client.
///
/// Allows to receive requests and send responses.
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    /// Send response to client
    pub async fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        super::send_string_async(response, &mut self.stream).await
    }

    /// Receive requests from client
    pub async fn recv_request(&mut self) -> RecvResult {
        super::recv_string_async(&mut self.stream).await
    }

    /// Address of connected client
    pub async fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
