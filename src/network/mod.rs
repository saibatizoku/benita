pub mod conductivity;
pub mod ph;
pub mod services;

use errors::*;

/// API for network commands.
pub trait SocketCommand {
    type Socket;
    type Response;

    fn run(&self, socket: &mut Self::Socket) -> Result<Self::Response>;
}
