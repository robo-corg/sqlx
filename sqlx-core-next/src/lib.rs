//! Core of SQLx, the rust SQL toolkit.
//!
//! Provides a core set of traits and types that can be used to implement a database driver.
//!
//! This crate is used in the specific database driver crates (e.g., `sqlx-postgres`, `sqlx-mysql`)
//! and then the core types and the drivers are re-exported together in the main `sqlx` crate.
//!
#![warn(future_incompatible, rust_2018_idioms)]
#![cfg_attr(feature = "sqlite", deny(unsafe_code))]

mod arguments;
mod connection;
mod error;
mod execute;
mod executor;
mod options;

pub use arguments::{Argument, Arguments};
pub use connection::Connection;
pub use error::{Error, Result};
pub use execute::Execute;
pub use executor::Executor;
pub use options::{ConnectOptions, ConnectOptionsBuilder, SslMode};
