#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub use self::router::new;

mod handlers;
mod models;
mod router;
mod services;
