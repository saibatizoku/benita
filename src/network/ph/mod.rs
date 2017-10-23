//! Networked services for pH sensing.
pub mod replies;

mod client;
mod server;

pub use self::client::PhClient;
pub use self::server::PhSensorServer;
