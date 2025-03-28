use core::error::Error;

use axum::{Router, extract::Request, http::HeaderValue};
use sqlx::MySqlPool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Level;

use crate::services::Authenticator;

#[tracing::instrument(level = "debug", skip(database))]
pub fn new(database: MySqlPool) -> Result<Router, Box<dyn Error>> {
    Ok(authentication::router()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4200".parse::<HeaderValue>()?)
                .allow_headers(Any),
        )
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
