use crate::error::{ConnectError, RequestError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

/// Клиент STP.
pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    /// Пытаемся подключится к серверу и проверяем, что он поддерживает STP.
    pub async fn connect<Addrs>(addrs: Addrs) -> Result<Self, ConnectError>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Self::try_handshake(stream).await
    }

    /// Проводим handshake, чтобы убедиться, что сервер поддерживает STP:
    /// 1) отправляем байты "clnt",
    /// 1) ожидаем байты "serv" в ответ.
    async fn try_handshake(mut stream: TcpStream) -> Result<Self, ConnectError> {
        stream.write_all(b"clnt").await?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).await?;
        if &buf != b"serv" {
            return Err(ConnectError::BadHandshake);
        }
        Ok(Self { stream })
    }

    /// Отправка запроса на сервер и получение ответа.
    pub async fn send_request<R: AsRef<str>>(&mut self, req: R) -> Result<String, RequestError> {
        super::send_string(req, &mut self.stream).await?;
        let response = super::recv_string(&mut self.stream).await?;
        Ok(response)
    }
}
