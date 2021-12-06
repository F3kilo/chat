use crate::error::{ConnectError, ConnectResult, RecvResult, SendResult};
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use thiserror::Error;

pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    pub fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }

    pub fn incoming(&self) -> impl Iterator<Item = ConnectResult<StpConnection>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<StpConnection> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"serv")?;
        Ok(StpConnection { stream })
    }
}

pub type BindResult = Result<StpServer, BindError>;

#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        crate::send_string(response, &mut self.stream)
    }

    pub fn recv_request(&mut self) -> RecvResult {
        crate::recv_string(&mut self.stream)
    }
}
