mod handlers;
mod message;

use async_nats::ConnectError;
use message::NatsServer;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    count: Mutex<u64>,
}

#[tokio::main]
async fn main() -> Result<(), ConnectError> {
    let client = async_nats::connect("demo.nats.io").await?;
    let state = Arc::new(AppState {
        count: Mutex::new(0),
    });

    let _handles = NatsServer::new(client, state)
        .handle("count.value", handlers::count)
        .handle("count.increment", handlers::increment)
        .handle("count.decrement", handlers::decrement)
        .start()
        .await;

    Ok(())
}
