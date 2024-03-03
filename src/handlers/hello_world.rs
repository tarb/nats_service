use super::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn hello_world(Json(args): Json<CreateUser>) -> Json<User> {
    args.username;

    Json(User {
        id: 1,
        username: String::from("hello world"),
    })
}
