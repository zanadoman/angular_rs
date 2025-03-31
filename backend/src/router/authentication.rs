use axum::{Router, routing};
use sqlx::MySqlPool;

use crate::{handlers::authentication, services::Authenticator};

#[tracing::instrument(level = "debug")]
pub fn router() -> Router<MySqlPool> {
    Router::new()
        .route("/logout", routing::post(authentication::logout))
        .route_layer(axum_login::login_required!(Authenticator))
        .route("/login", routing::post(authentication::login))
        .route("/register", routing::post(authentication::register))
}
