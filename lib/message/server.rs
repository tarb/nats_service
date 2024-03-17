use super::handler::Handler;
use async_nats::SubscribeError;
use futures::{Future, StreamExt};
use std::pin::Pin;
use tokio::task::{JoinHandle, JoinSet};

type Listener =
    Pin<Box<dyn Send + Future<Output = Result<(&'static str, JoinHandle<()>), SubscribeError>>>>;

pub struct NatsServer<S> {
    client: async_nats::Client,
    state: S,
    handles: Vec<Listener>,
}

impl<S: 'static + Clone + Send + Sync> NatsServer<S> {
    pub fn new(client: async_nats::Client, state: S) -> Self {
        Self {
            client,
            handles: Vec::new(),
            state,
        }
    }

    pub fn handle<P, H>(mut self, subject: &'static str, h: H) -> Self
    where
        H: 'static + Send + Copy + Handler<P, S>,
    {
        let client = self.client.clone();
        let state = self.state.clone();
        let message_task = Box::pin(async move {
            let subscription = client.subscribe(subject).await;

            match subscription {
                Ok(mut sub) => {
                    let state = state.clone();

                    let handle = tokio::spawn(async move {
                        while let Some(message) = sub.next().await {
                            let client = client.clone();
                            let state = state.clone();

                            tokio::task::spawn(async move {
                                let bs = h.call(&message, state).await;
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

        Self {
            client: self.client,
            state: self.state,
            handles: self.handles,
        }
    }

    pub async fn start(self) -> Result<SubjectHandles, SubscribeError> {
        let num_tasks = self.handles.len();

        let mut set = JoinSet::new();
        for task in self.handles {
            set.spawn(task);
        }

        let mut handles = Vec::with_capacity(num_tasks);
        while let Some(res) = set.join_next().await {
            if let Ok(res) = res {
                handles.push(res?);
            }
        }

        Ok(SubjectHandles(handles))
    }
}

pub struct SubjectHandles(pub Vec<(&'static str, JoinHandle<()>)>);
