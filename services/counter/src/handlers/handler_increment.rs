use super::{
    json::{Json, JsonError},
    Error,
};
use crate::{repo::Repository, AppState};
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

pub async fn increment<R>(
    state: Arc<AppState<R>>,
    Json(inc): Json<IncrementtInput>,
) -> Result<Json<IncrementOutput>, JsonError<Error>>
where
    R: Repository,
    R::Error: Into<Error>,
{
    let count = state.repo.increment(inc.amount).await?;

    Ok(Json(IncrementOutput { count }))
}
