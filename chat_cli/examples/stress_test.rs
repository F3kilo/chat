use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use chat_client::asnc::ChatClient;
use tokio::fs;
use tokio::time::Instant;

const CLIENTS_NUMBER: usize = 100;
const REQUESTS_NUMBER: usize = 100;

#[tokio::main]
async fn main() {
    let addr = get_server_addr().await;

    let start = Instant::now();

    let counter = Arc::new(AtomicU64::default());
    let mut join_set = tokio::task::JoinSet::new();
    for client_idx in 0..CLIENTS_NUMBER {
        let addr = addr.clone();
        let counter = counter.clone();
        let mut client = ChatClient::new(addr).await.unwrap();

        let request_future = async move {
            for request_idx in 0..REQUESTS_NUMBER {
                let idx = client_idx * CLIENTS_NUMBER + request_idx;
                if idx % 100 == 1 {
                    if client.append("s").await.is_err() {
                        continue;
                    };
                } else {
                    if client.fetch().await.is_err() {
                        continue;
                    };
                }

                counter.fetch_add(1, Ordering::Relaxed);
            }
        };

        join_set.spawn(request_future);
    }

    join_set.join_all().await;

    let finish = Instant::now();


    println!("Sent {} requests", counter.load(Ordering::Relaxed));
    println!("Elapsed: {:?}", finish - start);
}

async fn get_server_addr() -> String {
    fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
