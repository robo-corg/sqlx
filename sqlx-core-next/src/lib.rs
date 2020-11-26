//! Core of SQLx, the rust SQL toolkit.
//!
//! Provides a core set of traits and types that can be used to implement a database driver.
//!
//! This crate is used in the specific database driver crates (e.g., `sqlx-postgres`, `sqlx-mysql`)
//! and then the core types and the drivers are re-exported together in the main `sqlx` crate.
//!
#![warn(future_incompatible, rust_2018_idioms)]
#![deny(unsafe_code)]

mod arguments;
mod connection;
mod database;
mod error;
mod execute;
mod executor;
// mod from_value;
mod options;
mod to_value;
mod r#type;
mod type_info;

pub use arguments::{Argument, Arguments};
pub use connection::Connection;
pub use database::{Database, HasOutput, HasTypeId};
pub use error::{Error, Result};
pub use execute::Execute;
pub use executor::Executor;
pub use type_info::TypeInfo;
// pub use from_value::FromValue;
pub use options::{ConnectOptions, ConnectOptionsBuilder, SslMode};
pub use r#type::Type;
pub use to_value::ToValue;
