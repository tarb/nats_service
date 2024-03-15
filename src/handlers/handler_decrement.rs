use super::json::{Json, JsonError};
use super::Error;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DecrementInput {
    amount: i32,
}

#[derive(Serialize)]
pub struct DecrementOutput {
    count: i32,
}

pub async fn decrement(
    state: Arc<AppState>,
    Json(dec): Json<DecrementInput>,
) -> Result<Json<DecrementOutput>, JsonError<Error>> {
    let (count, ) = sqlx::query_as("INSERT INTO counts (integer_value) VALUES ((SELECT integer_value from counts ORDER BY created_at DESC LIMIT 1 ) - $1) RETURNING integer_value")
        .bind(dec.amount)
        .fetch_one(&state.database)
        .await?;

    Ok(Json(DecrementOutput { count }))
}
