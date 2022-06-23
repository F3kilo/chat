use std::error::Error;
use stp::server::{StpConnection, StpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let server = StpServer::bind("127.0.0.1:55331")?;
    for connection in server.incoming() {
        process_connection(connection?)?
    }
    Ok(())
}

fn process_connection(mut conn: StpConnection) -> Result<(), Box<dyn Error>> {
    let req = conn.recv_request()?;
    assert_eq!(req, "Hello, server");
    conn.send_response("Hello, client")?;
    Ok(())
}
