use super::models::{IndexResponse, UserResponse};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn index() -> impl IntoResponse {
    let response = IndexResponse {
        message: "Hello World".to_string(),
    };

    (StatusCode::OK, Json(response))
}

pub async fn get_users() -> impl IntoResponse {
    let users = vec![UserResponse {
        id: 1,
        name: "alice".to_string(),
    }];

    (StatusCode::OK, Json(users))
}
