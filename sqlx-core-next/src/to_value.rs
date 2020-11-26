use std::error::Error as StdError;

use crate::{Database, HasOutput, Type, TypeInfo};

/// Convert a single value in Rust to the format expected by a SQL database driver.
///
/// `ToValue` is implemented for `Option<T>` where `T` implements `ToValue`. An `Option<T>`
/// represents a nullable value. To pass an opaque `NULL` without any associated SQL type, use
/// `None::<()>`.
///
pub trait ToValue<DB: Database<R>, R>: Send + Sync + Type<DB, R> {
    /// Returns `true` if this Rust type can accept serializing into the
    /// given SQL type.
    ///
    /// This is only called by a database driver that supports
    /// strongly typed parameters (only PostgreSQL) and only if the
    /// type was erased (not hinted) by the client.
    ///
    /// ```rust,ignore
    /// sqlx::query("INSERT INTO post ( text ) VALUES ( $1 )")
    ///     //
    ///     // pass the value 5 without telling the query planner
    ///     // the type of our argument, this will cause postgres to tell
    ///     // us the type it *should* be and for SQLx to call ToSql::accepts
    ///     // to validate that the Rust type matches
    ///     .bind_erased(5)
    ///     //
    ///     // this is equivalent to `bind_erased` except SQLx will not call
    ///     // accepts to validate the parameter type
    ///     .bind_unchecked(5)
    ///     // [...]
    /// ```
    ///
    fn accepts(&self, ty: &DB::TypeInfo) -> bool {
        // UNKNOWN means that the database driver does not know what type that this parameter
        // should be; this will happen 100% of the time outside of postgres
        ty.is_unknown() || Self::compatible(ty)
    }

    /// Converts this Rust value into the equivalent SQL value, formatted as expected by
    /// the database driver, and places it into `out`.
    ///
    /// The type that the database has inferred the parameter to is given if that information
    /// is available; otherwise, a marker indicating itself as unknown will be passed.
    fn to_value(
        &self,
        ty: &DB::TypeInfo,
        out: &mut <DB as HasOutput<'_>>::Output,
    ) -> Result<(), Box<dyn StdError + 'static + Send + Sync>>;
}
