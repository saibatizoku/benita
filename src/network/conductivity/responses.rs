//! Responses from the conductivity sensor. `Response`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketResponse};
use devices::conductivity::responses::{CalibrationStatus, CompensationValue, DeviceInfo,
                    DeviceStatus, Exported, ExportedInfo, LedStatus, OutputStringStatus,
                    ProbeReading, ProbeType, ProtocolLockStatus};

macro_rules! fn_response_from {
    ($name:ident) => {
            fn response_from<T: Endpoint>(endpoint: &T) -> Result<$name> {
                let rep_string = endpoint.recv()?;
                let response = $name::parse_response(&rep_string)?;
                Ok(response)
            }
    };
}

macro_rules! impl_SocketResponse_for {
    ( $name:ident ) => {
        impl SocketResponse for $name {
            fn parse_response(rep_str: &str) -> Result<$name> {
                $name::parse(rep_str)
                    .chain_err(|| ErrorKind::CommandResponse)
            }

            fn_response_from!($name);
        }
    };
}

// Basically, wrap existing responses from the original sensor crate.
impl_SocketResponse_for!(CalibrationStatus);
impl_SocketResponse_for!(CompensationValue);
impl_SocketResponse_for!(DeviceInfo);
impl_SocketResponse_for!(DeviceStatus);
impl_SocketResponse_for!(Exported);
impl_SocketResponse_for!(ExportedInfo);
impl_SocketResponse_for!(LedStatus);
impl_SocketResponse_for!(OutputStringStatus);
impl_SocketResponse_for!(ProbeReading);
impl_SocketResponse_for!(ProbeType);
impl_SocketResponse_for!(ProtocolLockStatus);
