use chrono::{DateTime, Utc};
use std::error::Error;
use std::{fmt, fs};
use stp::server::StpServer;

fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr)?;

    // Создаём новый чат.
    let mut chat = Chat::default();

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(mut connection) = server.accept() else {
            continue;
        };

        let addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        // Обрабатываем запрос.
        connection.process_request(|req| {
            // Если запрос fetch, возвращаем историю сообщений.
            if req == "fetch" {
                return chat.history();
            }

            // Если запрос append, добавляем новое сообщение.
            if let Some(msg) = req.strip_prefix("append:") {
                return chat.append(addr, msg.into());
            }

            // Если запрос неизвестен, возвращаем сообщение об ошибке.
            format!("Unknown request: {}", req)
        })?;
    }
}

/// Храним последовательность сообщений.
#[derive(Default, Clone)]
pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    /// Выводим историю сообщений.
    pub fn history(&self) -> String {
        self.messages.iter().map(|m| m.to_string()).collect()
    }

    /// Добавляем новое сообщение.
    pub fn append(&mut self, from: String, msg: String) -> String {
        let sent = Utc::now();
        let msg = Message { sent, from, msg };
        self.messages.push(msg.clone());
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
