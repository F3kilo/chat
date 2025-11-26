use chat_client::ChatClient;
use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr();

    // Читаем аргументы командной строки.
    let mut cli_args = std::env::args().skip(1);
    let Some(action) = cli_args.next() else {
        return Err("No action provided, use 'append' or 'fetch'".into());
    };

    println!("Performing action: {action}...");

    // Соединяемся с сервером чата.
    let mut client = ChatClient::new(addr)?;

    if action == "fetch" {
        // Выводим историю.
        let chat_history = client.fetch()?;
        println!("Chat history:");
        println!("{}", chat_history);
        return Ok(());
    }

    if action == "append" {
        // Отправляем новое сообщение.
        let Some(msg) = cli_args.next() else {
            return Err("No message provided".into());
        };
        client.append(&msg)?;
        return Ok(());
    }

    Err("Unknown action, use 'append' or 'fetch'".into())
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
