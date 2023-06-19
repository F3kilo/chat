use chat_server::chat::Chat;
use chat_server::handler::{Request, RequestHandler};
use stp::asnc::server::{StpConnection, StpServer};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"));

    let server = StpServer::bind(addr).await?;
    let chat = Chat::default();

    loop {
        let connection = match server.accept().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let addr = match connection.peer_addr().await {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", addr);

        let chat = chat.clone();
        tokio::spawn(async move {
            if handle_connection(connection, chat).await.is_err() {
                println!("Client disconnected: {}", addr);
            }
        });
    }
}

async fn handle_connection(mut connection: StpConnection, chat: Chat) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(chat);
    loop {
        let req_str = connection.recv_request().await?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req)).await?;
    }
}
