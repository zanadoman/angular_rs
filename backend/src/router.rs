use core::error::Error;

use axum::{
    Router,
    http::{HeaderValue, header::CONTENT_TYPE},
};
use sqlx::MySqlPool;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Level;

use crate::Authenticator;

#[tracing::instrument(level = "debug", skip(pool))]
pub fn new(pool: MySqlPool) -> Result<Router, Box<dyn Error>> {
    Ok(authentication::router()
        .layer(Authenticator::new(pool.clone())?)
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_headers([CONTENT_TYPE])
                .allow_origin("http://localhost:4200".parse::<HeaderValue>()?),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
        )
        .with_state(pool))
}

mod authentication;
