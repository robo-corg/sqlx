use crate::Arguments;

/// Represents a SQL command that can be executed by SQLx.
///
/// When executing queries using an [`Executor`][crate::Executor] the SQL query that is
/// executed is **prepared** if it has arguments. A prepared query uses the database-specific
/// protocol to bind parameters for efficiency and to prevent vulnerabilities such as [SQL injection].
///
/// ```rust,ignore
/// // query with arguments, uses prepared queries and is safe from SQL injection
/// query("SELECT ?").bind(&10).fetch_one(&mut conn).await?;
///
/// // query with string formatting
/// // does not use prepared queries and may be vulnerable to SQL injection
/// conn.fetch_one(format!("SELECT {}", 10)).fetch_one(&mut conn).await?;
/// ```
///
/// Generally, a prepared SQL query is strictly a single statement (optionally terminated in `;`).
/// On the other hand, an unprepared SQL query can consist of multiple SQL statements (separated by `;`).
///
/// ```rust,ignore
/// // rows => [10, 20, 30]
/// let rows: Vec<(i32,)> = conn.fetch_all("SELECT 10; SELECT 20; SELECT 30").await?;
/// ```
///
/// Further, a prepared SQL query will be cached at the statement level. This enables efficient
/// query plan reuse on subsequent invocations of the same SQL query string.
///
/// ```rust,ignore
/// // unprepared, not cached
/// conn.fetch_one("SELECT 10").await?;
///
/// // prepared and cached
/// query("SELECT 10").fetch_one(&mut *conn).await?;
///
/// // prepared and not cached
/// query("SELECT 10").transient().fetch_one(&mut *conn).await?;
/// ```
///
/// [SQL injection]: https://owasp.org/www-community/attacks/SQL_Injection   
///
// 'q: query string
// 'a: argument values
pub trait Execute<'q, 'a> {
    /// Returns the SQL command to be executed.
    fn sql(&self) -> &str;

    /// Returns the arguments for any bind parameters in the SQL command.
    ///
    /// A value of `None` for `arguments()` is different from an empty list of
    /// arguments. The latter instructs SQLx to prepare the SQL command
    /// (with no arguments) and then execute it. The former
    /// will result in a simple and unprepared SQL command.
    ///
    fn arguments(&self) -> Option<&Arguments<'a>> {
        None
    }

    /// Returns `true` if the SQL statement should be cached for reuse.
    ///
    /// Note that this only has an effect if `arguments` returns non-`None`; as otherwise,
    /// this is a simple and unprepared query and cannot be cached at the statement level.
    ///  
    fn persistent(&self) -> bool {
        false
    }
}

impl<'q, 'a> Execute<'q, 'a> for &'q str {
    fn sql(&self) -> &str {
        self
    }
}

impl<'a> Execute<'static, 'a> for String {
    fn sql(&self) -> &str {
        self.as_str()
    }
}

impl<'q, 'a> Execute<'q, 'a> for (&'q str, Arguments<'a>) {
    fn sql(&self) -> &str {
        self.0
    }

    fn arguments(&self) -> Option<&Arguments<'a>> {
        Some(&self.1)
    }
}

impl<'q, 'a> Execute<'q, 'a> for (String, Arguments<'a>) {
    fn sql(&self) -> &str {
        self.0.as_str()
    }

    fn arguments(&self) -> Option<&Arguments<'a>> {
        Some(&self.1)
    }
}

impl<'q, 'a, E> Execute<'q, 'a> for &E
where
    E: Execute<'q, 'a>,
{
    fn sql(&self) -> &str {
        (*self).sql()
    }

    fn arguments(&self) -> Option<&Arguments<'a>> {
        (*self).arguments()
    }

    fn persistent(&self) -> bool {
        (*self).persistent()
    }
}
