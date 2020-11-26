use crate::{Database, HasTypeId, TypeInfo};

/// Indicates that a SQL type is supported for a database.
pub trait Type<DB: Database<R>, R> {
    /// Returns the canonical SQL type identifier for this Rust type.
    ///
    /// When binding arguments, this is used to tell the database what is about to be sent; which,
    /// the database then uses to guide query plans. This can be overridden by `Encode::produces`.
    ///
    /// A map of SQL types to Rust types is populated with this and used
    /// to determine the type that is returned from the anonymous struct type from `query!`.
    ///
    fn type_id() -> <DB as HasTypeId<'static, R>>::TypeId;

    /// Determines if this Rust type is compatible with the given SQL type.
    fn compatible(ty: &DB::TypeInfo) -> bool {
        ty.id() == Self::type_id()
    }
}

// for references, the underlying SQL type is identical
impl<R, T: ?Sized + Type<DB, R>, DB: Database<R>> Type<DB, R> for &'_ T {
    fn type_id() -> <DB as HasTypeId<'static, R>>::TypeId {
        <T as Type<DB, R>>::type_id()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <T as Type<DB, R>>::compatible(ty)
    }
}

// for optionals, the underlying SQL type is identical
impl<R, T: Type<DB, R>, DB: Database<R>> Type<DB, R> for Option<T> {
    fn type_id() -> <DB as HasTypeId<'static, R>>::TypeId {
        <T as Type<DB, R>>::type_id()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <T as Type<DB, R>>::compatible(ty)
    }
}
