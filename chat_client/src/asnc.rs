use tokio::net::ToSocketAddrs;
use stp::asnc::client::{RequestResult, StpClient};
use stp::error::ConnectResult;

pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr).await?;
        Ok(Self { stp })
    }

    pub async fn fetch(&mut self, room_id: &str) -> RequestResult {
        let request = format!("fetch|||{}", room_id);
        self.stp.send_request(request).await
    }

    pub async fn create_room(&mut self, room_id: &str) -> RequestResult {
        let request = format!("create|||{}", room_id);
        self.stp.send_request(request).await
    }

    pub async fn append(&mut self, room_id: &str, msg: &str) -> RequestResult {
        let request = format!("append|||{}|||{}", room_id, msg);
        self.stp.send_request(request).await
    }
}
