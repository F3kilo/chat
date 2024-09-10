use std::error::Error;
use stp::server::{StpConnection, StpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let server = StpServer::bind("127.0.0.1:55331")?;
    let conn = server.accept()?;
    process_connection(conn)?;
    Ok(())
}

fn process_connection(mut conn: StpConnection) -> Result<(), Box<dyn Error>> {
    conn.process_request(|req| {
        assert_eq!(req, "Hello, server");
        format!("Hello, client")
    })?;

    Ok(())
}
