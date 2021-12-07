use chat_client::ChatClient;
use state::{Main, State};
use std::error::Error;
use std::fs;

mod state;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr();
    let mut client = ChatClient::new(addr)?;

    let mut state: Box<dyn State> = Box::new(Main);
    while !state.exit() {
        state = state.update(&mut client)?;
    }

    Ok(())
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
