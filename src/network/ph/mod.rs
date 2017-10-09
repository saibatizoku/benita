//! Networked services for pH sensing.
pub mod client;
pub mod server;

pub use self::client::PhClient;
pub use self::server::PhSensorServer;
