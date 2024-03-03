use super::json::{Json, JsonError};
use super::Error;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DecrementInput {
    amount: u64,
}

#[derive(Serialize)]
pub struct DecrementOutput {
    count: u64,
}

pub async fn decrement(
    state: Arc<AppState>,
    Json(dec): Json<DecrementInput>,
) -> Result<Json<DecrementOutput>, JsonError<Error>> {
    let count = {
        let mut v = state.count.lock().await;
        *v = v.checked_sub(dec.amount).ok_or_else(|| Error::Underflow {
            count: *v,
            subtract: dec.amount,
        })?;
        *v
    };

    Ok(Json(DecrementOutput { count }))
}
