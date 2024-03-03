mod message;
mod handlers;

use async_nats::ConnectError;
use message::NatsServer;


#[tokio::main]
async fn main() -> Result<(), ConnectError> {
    let client = async_nats::connect("demo.nats.io").await?;
    let _handles = NatsServer::new(client)
        .with_state(5)
        .handle("core.hello.world", handlers::hello_world)
        .handle("core/hello/world", handlers::hello_world2)
        .start().await;

    Ok(())
}
