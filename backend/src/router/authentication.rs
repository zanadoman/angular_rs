use axum::{Router, routing};
use sqlx::MySqlPool;

use crate::controllers::authentication;

#[tracing::instrument(level = "debug")]
pub fn router() -> Router<MySqlPool> {
    Router::new()
        .route("/register", routing::post(authentication::register))
        .route("/login", routing::post(authentication::login))
        .route("/logout", routing::post(authentication::logout))
}
