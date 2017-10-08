//! Networked services for Conductivity sensing.
pub mod client;
pub mod server;

pub use self::client::ConductivityClient;
pub use self::server::ConductivitySensorServer;
