//! Networked services for Conductivity sensing.
pub mod requests;
pub mod replies;

mod client;
mod server;

pub use self::client::ConductivityClient;
pub use self::server::ConductivitySensorServer;
