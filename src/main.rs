mod handlers;
mod message;
use sqlx::{postgres, Pool, Postgres};

use async_nats::ConnectError;
use message::NatsServer;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
// use tokio::sync::Mutex;

pub struct AppState {
    database: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool: Pool<Postgres> = postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/counter")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS counts (
        id SERIAL PRIMARY KEY,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        integer_value INTEGER
    );",
    )
    .execute(&pool)
    .await?;

    let client = async_nats::connect("nats://localhost:4222").await?;
    let state = Arc::new(AppState { database: pool });

    let _handles = NatsServer::new(client, state)
        .handle("count.value", handlers::count)
        .handle("count.increment", handlers::increment)
        .handle("count.decrement", handlers::decrement)
        .start()
        .await;

    let mut sig = signal(SignalKind::hangup()).unwrap();
    loop {
        sig.recv().await;
    }

    // Ok(())
}
