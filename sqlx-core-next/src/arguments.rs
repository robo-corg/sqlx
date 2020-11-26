use std::borrow::Cow;
use std::collections::HashMap;

/// A tuple of SQL arguments to be bound against a query.
///
/// Often when constructing dynamic SQL queries, it can be useful to collect
/// a heterogeneous list of values as the SQL query is built, to later be
/// used to execute the query. As there is no built-in, dynamic heterogeneous
/// list type in Rust, SQLx provides `Arguments` for this purpose.
///
pub struct Arguments<'a> {
    pub(crate) positional: Vec<Argument<'a>>,
    pub(crate) named: HashMap<Cow<'a, str>, Argument<'a>>,
}

impl<'a> Default for Arguments<'a> {
    fn default() -> Self {
        Arguments {
            positional: Vec::default(),
            named: HashMap::default(),
        }
    }
}

impl<'a> Arguments<'a> {
    /// Constructs a new, empty `Arguments`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new, empty `Arguments` with the specified capacity for positional arguments.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            positional: Vec::with_capacity(capacity),
            named: HashMap::default(),
        }
    }

    /// Returns the number of positional and named arguments.
    pub fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    /// Returns `true` if this contains no arguments.
    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }
}

// pub(crate) value: &'q dyn ToValue<DB>,
// pub(crate) behavior: ArgumentBehavior,
pub struct Argument<'a> {
    value: &'a (),
}
