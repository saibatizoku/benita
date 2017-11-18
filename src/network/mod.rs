//! Sensor network sockets.
#[macro_use]
mod macros;

// Common network items
pub mod common;

pub mod conductivity;
pub mod ph;
pub mod temperature;

pub mod errors {
    use neuras;

    error_chain! {
        links {
            // external crate error-chains
            Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
        }
    }
}

/// Important traits.
pub use self::common::{Endpoint, SocketReply, SocketRequest};
