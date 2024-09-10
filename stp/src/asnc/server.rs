use crate::error::{ConnectError, RequestError};
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// STP сервер.
pub struct StpServer {
    tcp: TcpListener,
}

impl StpServer {
    /// Закрепляем сервер на сокете.
    pub async fn bind<Addrs>(addrs: Addrs) -> io::Result<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    /// Принимаем входящее соединение и производим handshake.
    pub async fn accept(&self) -> Result<StpConnection, ConnectError> {
        let (stream, _) = self.tcp.accept().await?;
        Self::try_handshake(stream).await
    }

    /// Проводим handshake, чтобы убедиться, что клиент поддерживает STP:
    /// 1) ожидаем байты "clnt",
    /// 1) отправляем байты "serv" в ответ.
    async fn try_handshake(mut stream: TcpStream) -> Result<StpConnection, ConnectError> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"clnt" {
            return Err(ConnectError::BadHandshake);
        }
        stream.write_all(b"serv").await?;
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
    pub async fn process_request<F>(&mut self, handler: F) -> Result<(), RequestError>
    where
        F: FnOnce(String) -> String,
    {
        let request = super::recv_string(&mut self.stream).await?;
        let response = handler(request);
        super::send_string(&response, &mut self.stream).await?;
        Ok(())
    }

    pub async fn process_request_async<F, Fut>(&mut self, handler: F) -> Result<(), RequestError>
    where
        Fut: Future<Output = String>,
        F: FnOnce(String) -> Fut,
    {
        let request = super::recv_string(&mut self.stream).await?;
        let response = handler(request).await;
        super::send_string(&response, &mut self.stream).await?;
        Ok(())
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
