use stp::asnc::client::StpClient;
use stp::error::{ConnectError, RequestError};
use tokio::net::ToSocketAddrs;

/// Клиент чата.
pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    /// Подключаемся к серверу.
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> Result<Self, ConnectError> {
        let stp = StpClient::connect(addr).await?;
        Ok(Self { stp })
    }

    /// Запрашиваем сообщения в чате.
    pub async fn fetch(&mut self) -> Result<String, RequestError> {
        self.stp.send_request("fetch").await
    }

    /// Добавляем сообщение.
    pub async fn append(&mut self, msg: &str) -> Result<String, RequestError> {
        let request = format!("append:{}", msg);
        self.stp.send_request(request).await
    }
}
