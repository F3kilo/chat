mod chat;
mod handler;

use chat::Chat;
use handler::{Request, RequestHandler};
use std::error::Error;
use std::{fs, thread};
use stp::server::{StpConnection, StpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr)?;
    let chat = Chat::default();

    for connection in server.incoming() {
        let connection = match connection {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Can't establish connection: {}", e);
                continue;
            }
        };

        let chat = chat.clone();
        thread::spawn(move || {
            if let Err(e) = handle_connection(connection, chat) {
                eprintln!("Request handling error: {}", e);
            }
        });
    }
    Ok(())
}

fn handle_connection(mut connection: StpConnection, chat: Chat) -> Result<(), anyhow::Error> {
    let mut handler = RequestHandler::new(chat);
    loop {
        let req_str = connection.recv_request()?;
        let req = Request::new(&req_str);
        connection.send_response(handler.handle(req))?;
    }
}
