use super::json::{Json, JsonError};
use super::Error;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct CountOutput {
    count: i32,
}

pub async fn count(state: Arc<AppState>) -> Result<Json<CountOutput>, JsonError<Error>> {
    let (count,): (i32,) =
        sqlx::query_as("SELECT integer_value FROM counts ORDER BY created_at DESC LIMIT 1")
            .fetch_one(&state.database)
            .await?;

    Ok(Json(CountOutput { count }))
}
