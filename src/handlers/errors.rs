use super::json::JsonError;
use serde::{Serialize, Serializer};
//
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serialization failed: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("decrement below 0 - count:{count}, subtracting:{subtract}")]
    Underflow { count: u64, subtract: u64 },
    #[error("increment above max value - count:{count}, adding:{addition}")]
    Overflow { count: u64, addition: u64 },
    #[error("DB error : {0}")]
    DBError(#[from] sqlx::Error),
}

impl Error {
    fn error_type(&self) -> &'static str {
        match self {
            Error::Serialize(_) => "input",
            Error::Overflow { .. } => "overflow",
            Error::Underflow { .. } => "underflow",
            Error::DBError { .. } => "db",
        }
    }
}

// format the errors as { "errorType":"x", "message":"y"}
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

        JsonError::new(MessageAndType {
            error_type: self.error_type(),
            message: &self.to_string(),
        })
        .serialize(serializer)
    }
}
