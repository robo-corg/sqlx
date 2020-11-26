use std::result::Result as StdResult;

/// A specialized `Result` type for SQLx.
pub type Result<T> = StdResult<T, Error>;

/// Represents all the ways a method can fail within SQLx.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {}
