use crate::error::{ConnectError, ConnectResult, RecvError, SendError};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

/// Represent client-side connection for STP
pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    /// Try to connect to specified address and perform handshake.
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Self::try_handshake(stream).await
    }

    /// Send request to connected STP server.
    pub async fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        super::send_string_async(req, &self.stream).await?;
        let response = super::recv_string_async(&self.stream).await?;
        Ok(response)
    }

    async fn try_handshake(s: TcpStream) -> ConnectResult<Self> {
        super::write_all_async(&s, b"clnt").await?;
        let mut buf = [0; 4];
        super::read_exact_async(&s, &mut buf).await?;
        if &buf != b"serv" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream: s })
    }
}

pub type RequestResult = Result<String, RequestError>;

/// Error for request sending. It consists from two steps: sending and receiving data.
///
/// `SendError` caused by send data error.
/// `RecvError` caused by receive data error.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),
    #[error(transparent)]
    Recv(#[from] RecvError),
}
