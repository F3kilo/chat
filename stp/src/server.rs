use crate::error::{ConnectError, RequestError};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

/// STP сервер.
pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    /// Закрепляем сервер на сокете.
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }

    /// Принимаем входящее соединение и производим handshake.
    pub fn accept(&self) -> Result<StpConnection, ConnectError> {
        let (stream, _) = self.tcp.accept()?;
        Self::try_handshake(stream)
    }

    /// Проводим handshake, чтобы убедиться, что клиент поддерживает STP:
    /// 1) ожидаем байты "clnt",
    /// 1) отправляем байты "serv" в ответ.
    fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectError> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            return Err(ConnectError::BadHandshake);
        }
        stream.write_all(b"serv")?;
        Ok(StpConnection { stream })
    }
}

/// Соединение с клиентом.
/// Позволяет обрабатывать запросы.
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    /// Обрабатываем запрос и возвращаем ответ используя логику
    /// предоставленную вызывающей стороной.
    pub fn process_request<F>(&mut self, handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(String) -> String,
    {
        let request = super::recv_string(&mut self.stream)?;
        let response = handler(request);
        super::send_string(&response, &mut self.stream)?;
        Ok(())
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
