use std::net::ToSocketAddrs;
use stp::client::{RequestResult, StpClient};
use stp::error::ConnectResult;

pub struct ChatClient {
    stp: StpClient,
}

impl ChatClient {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr)?;
        Ok(Self { stp })
    }

    pub fn fetch(&mut self, room_id: &str) -> RequestResult {
        let request = format!("fetch|||{}", room_id);
        self.stp.send_request(request)
    }

    pub fn create_room(&mut self, room_id: &str) -> RequestResult {
        let request = format!("create|||{}", room_id);
        self.stp.send_request(request)
    }

    pub fn append(&mut self, room_id: &str, msg: &str) -> RequestResult {
        let request = format!("append|||{}|||{}", room_id, msg);
        self.stp.send_request(request)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
