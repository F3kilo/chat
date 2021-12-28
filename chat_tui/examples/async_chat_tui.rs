use chat_client::asnc::ChatClient;
use chat_tui::async_state::{Main, State};
use tokio::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = get_server_addr().await;
    let mut client = ChatClient::new(addr).await?;

    let mut state: Box<dyn State> = Box::new(Main);
    while !state.exit() {
        state = state.update(&mut client).await?;
    }

    Ok(())
}

async fn get_server_addr() -> String {
    fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
