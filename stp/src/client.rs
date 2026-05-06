use crate::error::{ConnectError, RequestError};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Клиент STP.
pub struct StpClient {
    stream: TcpStream,
}

impl StpClient {
    /// Пытаемся подключится к серверу и проверяем, что он поддерживает STP.
    pub fn connect<Addrs>(addrs: Addrs) -> Result<Self, ConnectError>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handshake(stream)
    }

    /// Проводим handshake, чтобы убедиться, что сервер поддерживает STP:
    /// 1) отправляем байты "clnt",
    /// 1) ожидаем байты "serv" в ответ.
    fn try_handshake(mut stream: TcpStream) -> Result<Self, ConnectError> {
        stream.write_all(b"clnt")?;
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"serv" {
            return Err(ConnectError::BadHandshake);
        }
        Ok(Self { stream })
    }

    /// Отправка запроса на сервер и получение ответа.
    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> Result<String, RequestError> {
        crate::send_string(req, &mut self.stream)?;
        let response = crate::recv_string(&mut self.stream)?;
        Ok(response)
    }
}
