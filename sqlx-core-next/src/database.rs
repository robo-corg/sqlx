use crate::{Connection, TypeInfo};
use std::fmt::Debug;
use std::hash::Hash;

/// A database driver.
///
/// This trait encapsulates a complete set of traits that implement a driver for a
/// specific database (e.g., MySQL, PostgreSQL).
///
// 'ty: TypeInfo
//  'x: single execution
pub trait Database<R>: Sized + for<'ty> HasTypeId<'ty, R> + for<'x> HasOutput<'x> {
    /// The concrete [`Connection`] implementation for this database.
    type Connection: Connection<R, Database = Self>;

    /// The concrete [`TypeInfo`] implementation for this database.
    type TypeInfo: TypeInfo<R, Database = Self>;
}

/// Associates [`Database`] with a Rust type to hold a unique SQL type identifier
/// of a generic lifetime.
// 'ty: TypeInfo
pub trait HasTypeId<'ty, R> {
    type Database: Database<R>;

    /// The concrete type to hold a unique identifier for a SQL type for this database.  
    type TypeId: Eq + Debug + Hash + Copy;
}

/// Associates [`Database`] with a `Output` of a generic lifetime.
// 'x: single execution
pub trait HasOutput<'x> {
    /// The concrete type to hold the output for `Encode` for this database. This may be
    /// a simple alias to `&'x mut Vec<u8>`.
    type Output;
}
