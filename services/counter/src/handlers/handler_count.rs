use super::{
    json::{Json, JsonError},
    Error,
};
use crate::{repo::Repository, AppState};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct CountOutput {
    count: i32,
}

pub async fn count<R>(state: Arc<AppState<R>>) -> Result<Json<CountOutput>, JsonError<Error>>
where
    R: Repository,
    R::Error: Into<Error>,
{
    let count = state.repo.count().await?;

    Ok(Json(CountOutput { count }))
}
