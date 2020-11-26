use crate::ConnectOptions;

#[cfg(feature = "async")]
use futures_core::future::BoxFuture;

/// A unique connection (session) with a specific database.
///
/// With a client/server model, this is equivalent to a network connection
/// to the server.
///
/// SQL statements will be executed and results returned within the context
/// of this single SQL connection.
///
pub trait Connection<R>: 'static + Send {
    type Options: ConnectOptions<R, Connection = Self>;

    /// Establish a new database connection.
    ///
    /// A value of [`Options`](#associatedtype.Options) is parsed from the provided connection string. This parsing
    /// is database-specific.
    ///
    /// ```rust,ignore
    /// use sqlx::postgres::PgConnection;
    /// use sqlx::rt::AsyncStd;
    ///
    /// let mut conn = PgConnection::<AsyncStd>::connect(
    ///     "postgres://postgres:password@localhost/database",
    /// ).await?;
    /// ```
    ///
    /// You may also programmatically construct the connection options instead of parsing
    /// them from a URI.
    ///
    /// ```rust,ignore
    /// use sqlx::postgres::PgConnectOptions;
    /// use sqlx::rt::AsyncStd;
    ///
    /// let mut conn: PgConnection<AsyncStd> = PgConnectOptions::builder()
    ///     .host("localhost")
    ///     .ssl_mode(SslMode::VerifyIdentity)
    ///     .connect().await?;
    /// ```
    ///
    #[cfg(feature = "async")]
    #[inline]
    fn connect(uri: &str) -> BoxFuture<'static, crate::Result<Self>>
    where
        Self: Sized,
    {
        let opt = uri.parse::<Self::Options>();
        Box::pin(async move { opt?.connect().await })
    }

    /// Establish a new database connection.
    ///
    /// Synchronous [`Connection::connect`].
    #[cfg(feature = "blocking")]
    #[inline]
    fn connect_sync(uri: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        uri.parse::<Self::Options>()
            .and_then(|opt| opt.connect_sync())
    }

    /// Explicitly close this database connection.
    ///
    /// This method is **not required** for safe and consistent operation. However, it is
    /// recommended to call it instead of letting a connection `drop` as the database backend
    /// will be faster at cleaning up resources.
    ///
    #[cfg(feature = "async")]
    fn close(self) -> BoxFuture<'static, crate::Result<()>>;

    /// Explicitly close this database connection.
    ///
    /// Synchronous [`Connection::close`].
    #[cfg(feature = "blocking")]
    fn close_sync(self) -> crate::Result<()>;

    /// Checks if a connection to the database is still valid.
    ///
    /// The method of operation greatly depends on the database driver. In MySQL, there is an
    /// explicit [`COM_PING`](https://dev.mysql.com/doc/internals/en/com-ping.html) command. In
    /// PostgreSQL, `ping` will issue a query consisting of a comment `/* SQLx ping */` which,
    /// in effect, does nothing apart from getting a response from the server.
    ///
    #[cfg(feature = "async")]
    fn ping(&mut self) -> BoxFuture<'_, crate::Result<()>>;

    /// Checks if a connection to the database is still valid.
    ///
    /// Synchronous [`Connection::ping`].
    #[cfg(feature = "blocking")]
    fn ping_sync(&mut self) -> crate::Result<()>;
}
