//! Sensor network sockets.
#[macro_use]
mod macros;

// Common network items
pub mod common;

pub mod ph;
pub mod temperature;

pub mod errors {
    use super::common;
    use super::ph;
    use super::temperature;
    use neuras;

    error_chain! {
        links {
            Common(common::errors::Error, common::errors::ErrorKind);
            Ph(ph::errors::Error, ph::errors::ErrorKind);
            Temperature(temperature::errors::Error, temperature::errors::ErrorKind);
            // external crate error-chains
            Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
        }
    }
}

pub use conductivity::network as conductivity;

/// Important traits.
pub use self::common::{Endpoint, SocketReply, SocketRequest};
