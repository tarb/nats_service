use crate::message::{FromMessage, IntoBytes};
use async_nats::Message;
use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use super::Error;


#[derive(Debug, Clone, Default)]
pub struct Json<T>(pub T);

impl<T, M> FromMessage<M> for Json<T>
where
    T: DeserializeOwned,
{
    type Error = JsonError<Error>;

    fn from_message(message: &Message) -> Result<Self, Self::Error> {
        #[derive(Deserialize)]
        struct Args<T> {
            args: T,
        }

        serde_json::from_slice::<Args<T>>(&message.payload)
            .map(|v| Self(v.args))
            .map_err(|e| JsonError::new(Error::from(e)))
    }
}

impl<T: Serialize> IntoBytes for Json<T> {
    fn into_bytes(&self) -> Bytes {
        #[derive(Serialize)]
        #[serde(tag = "type", rename = "result")]
        struct Args<T> {
            data: T,
        }

        match serde_json::to_vec(&Args { data: &self.0 }) {
            Ok(v) => bytes::Bytes::from(v),
            Err(e) => JsonError::new(Error::from(e)).into_bytes(),
        }
    }
}

impl<T: IntoBytes, E: IntoBytes> IntoBytes for Result<T, E> {
    fn into_bytes(&self) -> Bytes {
        match self {
            Ok(t) => t.into_bytes(),
            Err(e) => e.into_bytes(),
        }
    }
}


#[derive(Serialize)]
#[serde(tag = "type", rename = "error")]
pub struct JsonError<T> {
    error: T,
}

impl<T> JsonError<T> {
    pub fn new(v: T) -> Self {
        Self{error:v}
    }
}

impl<T: Into<Error>> From<T> for JsonError<Error> {
    fn from(value: T) -> Self {
        Self{ error: value.into() }
    }
}

impl IntoBytes for JsonError<Error> {
    fn into_bytes(&self) -> Bytes {
        const FALLBACK_ERR: &'static [u8] =
            r#"{"type":"error", "error": { "message": "unknown", "errorType": "serialize" }}"#
                .as_bytes();

        match serde_json::to_vec(&self) {
            Ok(v) => bytes::Bytes::from(v),
            Err(_e) => bytes::Bytes::from(FALLBACK_ERR),
        }
    }
}
