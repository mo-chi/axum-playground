use super::controllers::{get_users, index};
use axum::{handler::get, routing::BoxRoute, Router};

pub fn routes() -> Router<BoxRoute> {
    let users = Router::new().route("/users", get(get_users)).boxed();

    Router::new()
        .route("/", get(index))
        .nest("/api", users)
        .boxed()
}
