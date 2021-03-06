//! Sensor network sockets.
#[macro_use]
mod macros;
mod traits;
// Common network items
mod common;

pub use conductivity::network as conductivity;
pub use ph::network as ph;
pub use temperature::network as temperature;

/// Important traits.
pub use self::common::{Endpoint, ReplyStatus, SocketReply, SocketRequest};
