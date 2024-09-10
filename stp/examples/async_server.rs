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

async fn process_connection(mut conn: StpConnection) -> Result<(), Box<dyn Error>> {
    conn.process_request(|request| {
        assert_eq!(request, "Hello, server");
        "Hello, client".into()
    })
    .await?;

    Ok(())
}
