use std::fmt::Debug;
use std::str::FromStr;

use crate::Connection;

#[cfg(feature = "async")]
use futures_core::future::BoxFuture;

/// Options which can be used to configure how a SQL connection is opened.
pub trait ConnectOptions<R>:
    'static + Clone + Default + Debug + FromStr<Err = crate::Error> + Send + Sync
{
    type Connection: Connection<R> + ?Sized;

    type Builder: ConnectOptionsBuilder<R, Options = Self>;

    /// Return a new builder.
    #[inline]
    fn builder() -> Self::Builder {
        Default::default()
    }

    /// Create a new [`ConnectOptionsBuilder`] based on these options.
    fn into_builder(self) -> Self::Builder;

    /// Establish a connection to the database.
    #[cfg(feature = "async")]
    fn connect(&self) -> BoxFuture<'_, crate::Result<Self::Connection>>
    where
        Self::Connection: Sized;

    /// Synchronously establish a connection to the database.
    ///
    /// See [`ConnectOptions::connect`].
    #[cfg(feature = "blocking")]
    fn connect_sync(&self) -> crate::Result<Self::Connection>
    where
        Self::Connection: Sized;
}

/// Builder for [`ConnectOptions`].
pub trait ConnectOptionsBuilder<R>: 'static + Default + Sized + Debug {
    type Options: ConnectOptions<R>;

    /// Invoke the builder and return a new [`ConnectOptions`].
    fn build(self) -> Self::Options;

    /// Establish a connection to the database.
    #[cfg(feature = "async")]
    #[inline]
    fn connect(
        self,
    ) -> BoxFuture<'static, crate::Result<<Self::Options as ConnectOptions<R>>::Connection>>
    where
        <Self::Options as ConnectOptions<R>>::Connection: Sized,
    {
        let options = self.build();
        Box::pin(async move { options.connect().await })
    }

    /// Synchronously establish a connection to the database.
    ///
    /// See [`ConnectOptionsBuilder::connect`].
    #[cfg(feature = "blocking")]
    #[inline]
    fn connect_sync(self) -> crate::Result<<Self::Options as ConnectOptions<R>>::Connection>
    where
        <Self::Options as ConnectOptions<R>>::Connection: Sized,
    {
        self.build().connect_sync()
    }
}
