use chat_client::asnc::ChatClient;
use std::error::Error;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr().await;

    // Читаем аргументы командной строки.
    let mut cli_args = std::env::args().skip(1);
    let Some(action) = cli_args.next() else {
        return Err(String::from("No action provided, use 'append' or 'fetch'").into());
    };

    println!("Performing action: {action}...");

    // Соединяемся с сервером чата.
    let mut client = ChatClient::new(addr).await?;

    if action == "fetch" {
        // Выводим историю.
        let chat_history = client.fetch().await?;
        println!("Chat history:");
        println!("{}", chat_history);
        return Ok(());
    }

    if action == "append" {
        // Отправляем новое сообщение.
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        client.append(&msg).await?;
        return Ok(());
    }

    Err(String::from("Unknown action, use 'append' or 'fetch'").into())
}

async fn get_server_addr() -> String {
    fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
