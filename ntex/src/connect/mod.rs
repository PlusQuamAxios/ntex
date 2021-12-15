//! Tcp connector service
use std::future::Future;

mod error;
mod message;
mod resolve;
mod service;

#[cfg(feature = "http-framework")]
mod uri;

#[cfg(feature = "openssl")]
pub mod openssl;

//#[cfg(feature = "rustls")]
//pub mod rustls;

pub use self::error::ConnectError;
pub use self::message::{Address, Connect};
pub use self::resolve::Resolver;
pub use self::service::Connector;

use crate::io::Io;

/// Resolve and connect to remote host
pub fn connect<T, U>(message: U) -> impl Future<Output = Result<Io, ConnectError>>
where
    T: Address + 'static,
    Connect<T>: From<U>,
{
    service::ConnectServiceResponse::new(Box::pin(
        Resolver::new().lookup(message.into()),
    ))
}
