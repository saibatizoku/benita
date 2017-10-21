//! Sensor network sockets.
#[macro_use]
// Common network items
pub mod common;

pub mod conductivity;
pub mod ph;
pub mod temperature;

// Important traits.
pub use self::common::{Endpoint, SocketRequest, SocketResponse};
