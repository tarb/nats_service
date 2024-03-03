use super::json::{Json, JsonError};
use super::Error;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct IncrementtInput {
    amount: u64,
}

#[derive(Serialize)]
pub struct IncrementOutput {
    count: u64,
}

pub async fn increment(
    state: Arc<AppState>,
    Json(inc): Json<IncrementtInput>,
) -> Result<Json<IncrementOutput>, JsonError<Error>> {
    let count = {
        let mut v = state.count.lock().await;
        *v = v.checked_add(inc.amount).ok_or_else(|| Error::Overflow {
            count: *v,
            addition: inc.amount,
        })?;
        *v
    };

    Ok(Json(IncrementOutput { count }))
}
