//! Sensor network sockets.
#[macro_use]
// Common network items
pub mod common;

pub mod conductivity;
pub mod ph;
pub mod temperature;


mod errors {
    error_chain! {
    }
}

/// Device Errors.
pub use self::errors::*;

// Important traits.
pub use self::common::{Endpoint, SocketReply, SocketRequest};
