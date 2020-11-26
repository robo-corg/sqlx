use crate::Database;
#[cfg(feature = "async")]
use futures_core::future::BoxFuture;

/// A type that is, contains, or can provide a database connection to use for executing
/// queries against the database.
///
/// There is no guarantee that successive queries run on the same physical database connection.
///
/// A [`Connection`][crate::Connection] is an `Executor` that guarantees that successive
/// queries are ran on the same physical database connection.
///
/// Implemented for the following:
///
///  - `&mut impl Connection`
///  - `&Pool`
///
/// Notably used by the finalizer methods for queries and their builders.
///
/// ```rust,ignore
/// query("SELECT ?, 20, false")
///     .bind(&50)
///     // fetch_one takes impl Executor
///     .fetch_one(&mut conn).await?;
///     // fetch_one(&pool).await?;
/// ```
///
/// It may be necessary to manually re-borrow the connection if your block receives it as a mutable
/// reference. This is because the finalizer methods (such as `fetch` and `execute`) take
/// the executor reference by value (`<E> (executor: E)`) to allow both `&Pool` and `&mut impl Connection`
/// to be accepted. Rust is unable to tell that this reference needs to be re-borrowed and thus moves
/// it instead of doing a re-borrow and moving that temporary and scoped borrow.
///
/// ```rust,ignore
/// fn my_query_method(conn: &mut impl Connection) -> sqlx::Result<(String,)> {
///     query("SELECT 'Hello, World'").fetch_one(&mut *conn).await?
/// }
/// ```
///
// 'q: query string
// 'a: argument values
// 'c: connection
// 'x: single execution
pub trait Executor<R> {
    type Database: Database<R>;

    #[cfg(feature = "async")]
    fn execute<'x, 'q: 'x, 'a: 'x, 'c: 'x, E>(self, command: E)
        -> BoxFuture<'x, crate::Result<()>>;

    #[cfg(feature = "blocking")]
    fn execute_sync<'q, 'a, 'c, E>(self, command: E) -> crate::Result<()>;
}
