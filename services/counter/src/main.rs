mod handlers;
mod repo;

use message::NatsServer;
use sqlx::{postgres, Pool, Postgres};
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};

const DATABASE_URL: &str = env!("DATABASE_URL");
const NATS_URL: &str = env!("NATS_URL");

// might be easier to use state here if we instead used a trait object
// then the handlers wont need to juggle the generic type
// pub struct AppState {
//     repo: Box<dyn repo::Repository>,
// }

pub struct AppState<R: repo::Repository> {
    repo: R,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let database: Pool<Postgres> = postgres::PgPoolOptions::new()
    //     .max_connections(30)
    //     .connect(DATABASE_URL)
    //     .await?;
    // sqlx::migrate!().run(&database).await?;
    // let repo = repo::Postgres::new(database);

    let repo = repo::Memory::new();

    let client = async_nats::connect(NATS_URL).await?;

    let state = Arc::new(AppState { repo });
    let _handles = NatsServer::new(client, state)
        .handle("count.value", handlers::count)
        .handle("count.increment", handlers::increment)
        .handle("count.decrement", handlers::decrement)
        .start()
        .await?;

    let mut sig = signal(SignalKind::hangup()).unwrap();
    sig.recv().await;

    Ok(())
}
