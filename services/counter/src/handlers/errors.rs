use std::convert::Infallible;

use serde::{Serialize, Serializer};
//
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serialization failed: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("DB error : {0}")]
    DBError(#[from] sqlx::Error),
    #[error("infallible")]
    Infallible(#[from] Infallible),
}

impl Error {
    fn error_type(&self) -> &'static str {
        match self {
            Error::Serialize(_) => "input",
            Error::DBError { .. } => "db",
            Error::Infallible(_) => unreachable!(),
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

        MessageAndType {
            error_type: self.error_type(),
            message: &self.to_string(),
        }
        .serialize(serializer)
    }
}
