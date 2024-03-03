use super::json::Json;
use crate::AppState;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct CountOutput {
    count: u64,
}

pub async fn count(state: Arc<AppState>) -> Json<CountOutput> {
    let count = *state.count.lock().await;

    Json(CountOutput { count })
}
