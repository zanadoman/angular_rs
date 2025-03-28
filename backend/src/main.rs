#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use core::error::Error;
use std::env;

use sqlx::MySqlPool;
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
    let database = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    tracing::info!("{database:?}");
    axum::serve(listener, backend::new(database)?)
        .with_graceful_shutdown(async { signal::ctrl_c().await.unwrap() })
        .await?;
    Ok(())
}
