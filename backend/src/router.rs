use core::error::Error;

use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header::CONTENT_TYPE},
};
use sqlx::MySqlPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::Level;

use crate::Authenticator;

#[tracing::instrument(level = "debug", skip(database))]
pub fn new(database: MySqlPool) -> Result<Router, Box<dyn Error>> {
    Ok(authentication::router()
        .layer(Authenticator::new(database.clone())?)
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_headers([CONTENT_TYPE])
                .allow_origin("http://localhost:4200".parse::<HeaderValue>()?),
        )
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
