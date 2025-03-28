#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub use router::new;

mod controllers;
mod models;
mod router;
mod services;
