use super::json::{Json, JsonError};
use super::Error;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct IncrementtInput {
    amount: i32,
}

#[derive(Serialize)]
pub struct IncrementOutput {
    count: i32,
}

pub async fn increment(
    state: Arc<AppState>,
    Json(inc): Json<IncrementtInput>,
) -> Result<Json<IncrementOutput>, JsonError<Error>> {
    let (count,): (i32,) = sqlx::query_as("INSERT INTO counts (integer_value) VALUES ((SELECT integer_value from counts ORDER BY created_at DESC LIMIT 1 ) + $1) RETURNING integer_value")
        .bind(inc.amount)
        .fetch_one(&state.database)
        .await?;

    Ok(Json(IncrementOutput { count }))
}
