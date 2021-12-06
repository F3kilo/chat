use stp::client::StpClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = StpClient::connect("127.0.0.1:55331")?;
    let response = client.send_request("Hello, server")?;
    assert_eq!(response, "Hello, client");
    Ok(())
}
