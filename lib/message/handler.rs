use async_nats::Message;
use bytes::Bytes;
use std::future::Future;

pub trait FromMessage: Sized {
    type Error: IntoBytes + Send;
    fn from_message(message: &Message) -> Result<Self, Self::Error>;
}

pub trait IntoBytes: Sized {
    fn into_bytes(self) -> Bytes;
}

impl<T: IntoBytes, E: IntoBytes> IntoBytes for Result<T, E> {
    fn into_bytes(self) -> Bytes {
        match self {
            Ok(t) => t.into_bytes(),
            Err(e) => e.into_bytes(),
        }
    }
}

pub trait Handler<P, S> {
    fn call(self, message: &Message, state: S) -> impl Send + Future<Output = Bytes>;
}

impl<F, FR, RV, S> Handler<(), S> for F
where
    F: Send + Fn(S) -> FR,
    FR: Send + Future<Output = RV>,
    RV: IntoBytes,
    S: Clone + Send + Sync,
{
    async fn call(self, _message: &Message, state: S) -> Bytes {
        (self)(state).await.into_bytes()
    }
}

impl<F, P, FR, RV, S> Handler<P, S> for F
where
    F: Send + Fn(S, P) -> FR,
    FR: Send + Future<Output = RV>,
    RV: IntoBytes,
    S: Clone + Send + Sync,
    P: Send + FromMessage,
{
    async fn call(self, message: &Message, state: S) -> Bytes {
        match P::from_message(message) {
            Ok(v) => (self)(state, v).await.into_bytes(),
            Err(e) => e.into_bytes(),
        }
    }
}

impl<F, P1, P2, R, RV, S> Handler<(P1, P2), S> for F
where
    F: Send + Fn(S, P1, P2) -> R,
    R: Send + Future<Output = RV>,
    RV: IntoBytes,
    S: Clone + Send + Sync,
    P1: Send + FromMessage,
    P2: Send + FromMessage,
{
    async fn call(self, message: &Message, state: S) -> Bytes {
        let r1 = P1::from_message(message);
        let r2 = P2::from_message(message);

        match (r1, r2) {
            (Ok(v1), Ok(v2)) => (self)(state, v1, v2).await.into_bytes(),
            (Err(e), _) => e.into_bytes(),
            (_, Err(e)) => e.into_bytes(),
        }
    }
}

impl<F, P1, P2, P3, R, RV, S> Handler<(P1, P2, P3), S> for F
where
    F: 'static + Send + Sync + Copy + Fn(S, P1, P2, P3) -> R,
    R: 'static + Send + Sync + Future<Output = RV>,
    RV: IntoBytes,
    S: Clone + Send + Sync,
    P1: Send + FromMessage,
    P2: Send + FromMessage,
    P3: Send + FromMessage,
{
    async fn call(self, message: &Message, state: S) -> Bytes {
        let r1 = P1::from_message(message);
        let r2 = P2::from_message(message);
        let r3 = P3::from_message(message);

        match (r1, r2, r3) {
            (Ok(v1), Ok(v2), Ok(v3)) => (self)(state, v1, v2, v3).await.into_bytes(),
            (Err(e), _, _) => e.into_bytes(),
            (_, Err(e), _) => e.into_bytes(),
            (_, _, Err(e)) => e.into_bytes(),
        }
    }
}
