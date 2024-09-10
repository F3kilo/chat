use chrono::{DateTime, Utc};
use std::error::Error;
use std::sync::Arc;
use std::{fmt, fs};
use stp::asnc::server::{StpConnection, StpServer};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr).await?;

    // Создаём новый чат.
    let chat = Arc::new(Chat::default());

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(connection) = server.accept().await else {
            continue;
        };

        tokio::spawn(process_connection(connection, chat.clone()));
    }
}

async fn process_connection(mut connection: StpConnection, chat: Arc<Chat>) {
    loop {
        let chat = chat.clone();
        
        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        // Обрабатываем запрос.
        let processing_result = connection
            .process_request_async(|req| async move {
                // Если запрос fetch, возвращаем историю сообщений.
                if req == "fetch" {
                    return chat.history().await;
                }

                // Если запрос append, добавляем новое сообщение.
                if let Some(msg) = req.strip_prefix("append:") {
                    return chat.append(addr, msg.into()).await;
                }

                // Если запрос неизвестен, возвращаем сообщение об ошибке.
                format!("Unknown request: {}", req)
            })
            .await;

        if let Err(e) = processing_result {
            eprintln!("Error processing request: {}", e);
            break;
        }
    }
}

/// Храним последовательность сообщений.
#[derive(Default)]
pub struct Chat {
    messages: RwLock<Vec<Message>>,
}

impl Chat {
    /// Выводим историю сообщений.
    pub async fn history(&self) -> String {
        self.messages
            .read()
            .await
            .iter()
            .map(|m| m.to_string())
            .collect()
    }

    /// Добавляем новое сообщение.
    pub async fn append(&self, from: String, msg: String) -> String {
        let sent = Utc::now();
        let msg = Message { sent, from, msg };
        self.messages.write().await.push(msg.clone());
        msg.to_string()
    }
}

// Сообщение с датой отправки и идентификатором отправителя.
#[derive(Debug, Clone)]
pub struct Message {
    sent: DateTime<Utc>,
    from: String,
    msg: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{} {}]: {}", self.sent, self.from, self.msg)
    }
}
