use super::{
    json::{Json, JsonError},
    Error,
};
use crate::{repo::Repository, AppState};
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

pub async fn decrement<R>(
    state: Arc<AppState<R>>,
    Json(dec): Json<DecrementInput>,
) -> Result<Json<DecrementOutput>, JsonError<Error>>
where
    R: Repository,
    R::Error: Into<Error>,
{
    let count = state.repo.decrement(dec.amount).await?;

    Ok(Json(DecrementOutput { count }))
}
