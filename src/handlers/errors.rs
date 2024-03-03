use serde::{Serialize, Serializer};
use super::json::JsonError;

// 
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serialization failed: {0}")]
    SerializeError(#[from] serde_json::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct MessageAndType<'a> {
            #[serde(rename = "errorType")]
            error_type: &'a str,
            message: &'a str,
        }

        match self {
            Error::SerializeError(v) => {
                let message = &v.to_string();
                JsonError::new(MessageAndType {
                    error_type: "input",
                    message: message,
                }).serialize(serializer)
            }
        }
    }
}

