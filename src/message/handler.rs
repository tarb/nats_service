use async_nats::Message;
use bytes::Bytes;
use std::future::Future;

pub trait FromMessage<T>: Sized {
    type Error: IntoBytes + Send;
    fn from_message(message: &Message) -> Result<Self, Self::Error>;
}

pub trait IntoBytes: Sized {
    fn into_bytes(&self) -> Bytes;
}

pub trait Handler<P> {
    fn call(self, message: &Message) -> impl Send + Future<Output = Bytes>;
}

impl<F, P, FR, RV> Handler<P> for F
where
    F: Send + Fn(P) -> FR,
    FR: Send + Future<Output = RV>,
    RV: IntoBytes,
    P: Send + for<'a> FromMessage<&'a Message>,
{
    async fn call(self, message: &Message) -> Bytes {
        match P::from_message(message) {
            Ok(v) => (self)(v).await.into_bytes(),
            Err(e) => e.into_bytes(),
        }
    }
}

impl<F, P1, P2, R, RV> Handler<(P1, P2)> for F
where
    F: Send + Fn(P1, P2) -> R,
    R: Send + Future<Output = RV>,
    RV: IntoBytes,
    P1: Send + for<'a> FromMessage<&'a Message>,
    P2: Send + for<'a> FromMessage<&'a Message>,
{
    async fn call(self, message: &Message) -> Bytes {
        let r1 = P1::from_message(message);
        let r2 = P2::from_message(message);

        match (r1, r2) {
            (Ok(v1), Ok(v2)) => (self)(v1, v2).await.into_bytes(),
            (Err(e), _) => e.into_bytes(),
            (_, Err(e)) => e.into_bytes(),
        }
    }
}

impl<F, P1, P2, P3, R, RV> Handler<(P1, P2, P3)> for F
where
    F: 'static + Send + Sync + Copy + Fn(P1, P2, P3) -> R,
    R: 'static + Send + Sync + Future<Output = RV>,
    RV: IntoBytes,
    P1: Send + for<'a> FromMessage<&'a Message>,
    P2: Send + for<'a> FromMessage<&'a Message>,
    P3: Send + for<'a> FromMessage<&'a Message>,
{
    async fn call(self, message: &Message) -> Bytes {
        let r1 = P1::from_message(message);
        let r2 = P2::from_message(message);
        let r3 = P3::from_message(message);

        match (r1, r2, r3) {
            (Ok(v1), Ok(v2), Ok(v3)) => (self)(v1, v2, v3).await.into_bytes(),
            (Err(e), _, _) => e.into_bytes(),
            (_, Err(e), _) => e.into_bytes(),
            (_, _, Err(e)) => e.into_bytes(),
        }
    }
}
