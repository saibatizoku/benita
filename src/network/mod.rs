//! Sensor network sockets.
#[macro_use]
mod macros;

mod traits;

// Common network items
pub mod common;

pub mod temperature;

pub mod errors {
    use super::common;
    use super::temperature;
    use neuras;

    error_chain! {
        links {
            Common(common::errors::Error, common::errors::ErrorKind);
            Temperature(temperature::errors::Error, temperature::errors::ErrorKind);
            // external crate error-chains
            Neuras(neuras::errors::Error, neuras::errors::ErrorKind);
        }
    }
}

pub use conductivity::network as conductivity;
pub use ph::network as ph;

/// Important traits.
pub use self::common::{Endpoint, SocketReply, SocketRequest};
