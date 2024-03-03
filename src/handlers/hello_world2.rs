use super::json::{Json, JsonError};
use serde::{Deserialize, Serialize};

use super::Error;

#[derive(Deserialize)]
pub struct InputThing {
    username: String,
}

#[derive(Serialize)]
pub struct OutputThing {
    id: u64,
    username: String,
}

pub async fn hello_world2(
    Json(args1): Json<InputThing>,
    Json(args2): Json<InputThing>,
) -> Result<Json<OutputThing>, JsonError<Error>> {
    args1.username;
    args2.username;

    // so this is pretty cool, this serde_json error will get automatically converted to our
    // errors.Error enum, then converted again to the wrapped JsonError<Error> output
    let _a = serde_json::to_vec(&4)?;

    Ok(Json(OutputThing {
        id: 1,
        username: String::from("hello world"),
    }))
}
