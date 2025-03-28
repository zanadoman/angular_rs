use core::error::Error;

use axum::{Router, extract::Request};
use sqlx::MySqlPool;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::services::Authenticator;

#[tracing::instrument(level = "debug", skip(database))]
pub fn new(database: MySqlPool) -> Result<Router, Box<dyn Error>> {
    Ok(authentication::router()
        .layer(Authenticator::new(database.clone())?)
        .layer(TraceLayer::new_for_http().make_span_with(
            |request: &Request| {
                tracing::span! {
                    Level::INFO,
                    "request",
                    method = %request.method(),
                    route = %request.uri()
                }
            },
        ))
        .with_state(database))
}

mod authentication;
