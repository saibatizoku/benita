//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketReply};
use devices::conductivity::responses::{CalibrationStatus, CompensationValue, DeviceInfo,
                                       DeviceStatus, Exported, ExportedInfo, LedStatus,
                                       OutputStringStatus, ProbeReading, ProbeType,
                                       ProtocolLockStatus};

macro_rules! fn_response_from {
    ($name:ident) => {
            fn response_from<T: Endpoint>(endpoint: &T) -> Result<$name> {
                let rep_string = endpoint.recv()?;
                let response = $name::parse_response(&rep_string)?;
                Ok(response)
            }
    };
}

macro_rules! impl_SocketReply_for {
    ( $name:ident ) => {
        impl SocketReply for $name {
            fn parse_response(rep_str: &str) -> Result<$name> {
                $name::parse(rep_str)
                    .chain_err(|| ErrorKind::CommandReply)
            }

            fn_response_from!($name);
        }
    };
}

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(CompensationValue);
impl_SocketReply_for!(DeviceInfo);
impl_SocketReply_for!(DeviceStatus);
impl_SocketReply_for!(Exported);
impl_SocketReply_for!(ExportedInfo);
impl_SocketReply_for!(LedStatus);
impl_SocketReply_for!(OutputStringStatus);
impl_SocketReply_for!(ProbeReading);
impl_SocketReply_for!(ProbeType);
impl_SocketReply_for!(ProtocolLockStatus);
