use core::error::Error;
use std::env;

use axum::{Router, response::Html, routing};
use tokio::{net::TcpListener, signal};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()?)
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .init();
    let listener = TcpListener::bind(&env::var("APP_ADDRESS")?).await?;
    tracing::info!("{listener:?}");
    axum::serve(
        listener,
        Router::new().route(
            "/",
            routing::get(async || -> Html<&'static str> {
                Html("<h1>Hello, World!</h1>")
            }),
        ),
    )
    .with_graceful_shutdown(async { signal::ctrl_c().await.unwrap() })
    .await?;
    Ok(())
}
