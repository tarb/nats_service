use super::handler::Handler;
use async_nats::SubscribeError;
use futures::{Future, StreamExt};
use std::pin::Pin;
use tokio::task::{JoinHandle, JoinSet};

pub struct NatsServer<S =()> {
    client: async_nats::Client,
    state: S,
    handles: Vec<
        Pin<
            Box<dyn Send + Future<Output = Result<(&'static str, JoinHandle<()>), SubscribeError>>>,
        >,
    >,
}

impl<S: Default> NatsServer<S> {
    pub fn new(client: async_nats::Client) -> Self {
        Self {
            client: client,
            handles: Vec::new(),
            state: Default::default(),
        }
    }

    pub fn with_state(self, state: S) -> Self {
        Self { client: self.client, state: state, handles: self.handles }
    }

    pub fn handle<P, H>(mut self, subject: &'static str, h: H) -> Self
    where
        H: 'static + Send + Copy + Handler<P>,
    {

        let client = self.client.clone();
        let message_task = Box::pin(async move {
            let subscription = client.subscribe(subject).await;

            match subscription {
                Ok(mut sub) => {
                    let handle = tokio::spawn(async move {
                        while let Some(message) = sub.next().await {
                            let client = client.clone();
                            tokio::task::spawn(async move {
                                let bs = h.call(&message).await;
                                if let Some(reply) = &message.reply {
                                    async move {
                                        let _ = client.publish(reply.clone(), bs).await;
                                    }
                                    .await;
                                }
                            });
                        }
                    });
                    Ok((subject, handle))
                }
                Err(e) => Err(e),
            }
        });
        self.handles.push(message_task);

        Self { client: self.client, state: self.state, handles: self.handles }
    }

    pub async fn start(self) -> Result<SubjectHandles, SubscribeError> {
        let num_tasks = self.handles.len();

        let mut set = JoinSet::new();
        for task in self.handles {
            set.spawn(task);
        }

        let mut handles = Vec::with_capacity(num_tasks);
        while let Some(res) = set.join_next().await {
            match res {
                Ok(res) => handles.push(res?),
                _ => {} // dont really know what to do on a JoinError
            }
        }

        Ok(SubjectHandles(handles))
    }
}

pub struct SubjectHandles(pub Vec<(&'static str, JoinHandle<()>)>);
