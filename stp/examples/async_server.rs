use std::error::Error;
use stp::asnc::server::{StpConnection, StpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = StpServer::bind("127.0.0.1:55331").await?;
    loop {
        let connection = server.accept().await?;
        process_connection(connection).await?
    }
}

async fn process_connection(conn: StpConnection) -> Result<(), Box<dyn Error>> {
    let req = conn.recv_request().await?;
    assert_eq!(req, "Hello, server");
    conn.send_response("Hello, client").await?;
    Ok(())
}
