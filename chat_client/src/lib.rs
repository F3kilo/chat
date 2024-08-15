use std::net::ToSocketAddrs;
use stp::client::StpClient;
use stp::error::{ConnectError, RequestError};

/// Клиент чата.
pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    /// Подключаемся к серверу.
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    /// Запрашиваем сообщения в чате.
    pub fn fetch(&mut self) -> Result<String, RequestError> {
        self.stp.send_request("fetch")
    }

    /// Добавляем сообщение.
    pub fn append(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("append:{}", msg);
        self.stp.send_request(request)
    }
}

