#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub use self::router::new;

mod authenticator;
mod handlers;
mod models;
mod router;

use self::authenticator::Authenticator;
