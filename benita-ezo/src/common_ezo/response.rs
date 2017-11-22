//! Shared responses for EZO sensors
use devices::I2CResponse;
use errors::*;

pub use ezo_common::response::{DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus,
                               ProtocolLockStatus, ResponseStatus};

impl_I2CResponse_for!(DeviceInfo);
impl_I2CResponse_for!(DeviceStatus);
impl_I2CResponse_for!(Exported);
impl_I2CResponse_for!(ExportedInfo);
impl_I2CResponse_for!(LedStatus);
impl_I2CResponse_for!(ProtocolLockStatus);
impl_I2CResponse_for!(ResponseStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_response_status_reply_from_valid_str() {
        let reply = <ResponseStatus as I2CResponse>::from_str("Ack").unwrap();
        assert_eq!("Ack", I2CResponse::to_string(&reply));
        let reply = <ResponseStatus as I2CResponse>::from_str("None").unwrap();
        assert_eq!("None", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_response_status_reply_from_invalid_str_yields_err() {
        let reply = <ResponseStatus as I2CResponse>::from_str("()");
        assert!(reply.is_err());
        let reply = <ResponseStatus as I2CResponse>::from_str("ack");
        assert!(reply.is_err());
        let reply = <ResponseStatus as I2CResponse>::from_str("none");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_info_reply_from_valid_str() {
        let reply = <DeviceInfo as I2CResponse>::from_str("?I,EC,0.0.0").unwrap();
        assert_eq!("?I,EC,0.0.0", I2CResponse::to_string(&reply));
        let reply = <DeviceInfo as I2CResponse>::from_str("?I,device,firmware").unwrap();
        assert_eq!("?I,device,firmware", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_device_info_reply_from_invalid_str_yields_err() {
        let reply = <DeviceInfo as I2CResponse>::from_str("?I,");
        assert!(reply.is_err());
        let reply = <DeviceInfo as I2CResponse>::from_str("?I,3");
        assert!(reply.is_err());
        let reply = <DeviceInfo as I2CResponse>::from_str("?I,S,L,4");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_status_reply_from_valid_str() {
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,P,0").unwrap();
        assert_eq!("?STATUS,P,0.000", I2CResponse::to_string(&reply));
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,S,1").unwrap();
        assert_eq!("?STATUS,S,1.000", I2CResponse::to_string(&reply));
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,B,2").unwrap();
        assert_eq!("?STATUS,B,2.000", I2CResponse::to_string(&reply));
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,W,3").unwrap();
        assert_eq!("?STATUS,W,3.000", I2CResponse::to_string(&reply));
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,U,4.505").unwrap();
        assert_eq!("?STATUS,U,4.505", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_device_status_reply_from_invalid_str_yields_err() {
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,");
        assert!(reply.is_err());
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,3");
        assert!(reply.is_err());
        let reply = <DeviceStatus as I2CResponse>::from_str("?STATUS,S,L");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_reply_from_valid_str() {
        let reply = <Exported as I2CResponse>::from_str("uptotwelvech").unwrap();
        assert_eq!("uptotwelvech", I2CResponse::to_string(&reply));
        let reply = <Exported as I2CResponse>::from_str("*DONE").unwrap();
        assert_eq!("*DONE", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_exported_reply_from_invalid_str_yields_err() {
        let reply = <Exported as I2CResponse>::from_str("uptotwelvechars");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_info_reply_from_valid_str() {
        let reply = <ExportedInfo as I2CResponse>::from_str("?EXPORT,1,1").unwrap();
        assert_eq!("?EXPORT,1,1", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_exported_info_reply_from_invalid_str_yields_err() {
        let reply = <ExportedInfo as I2CResponse>::from_str("?EXPORT,,");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_led_status_reply_from_valid_str() {
        let reply = <LedStatus as I2CResponse>::from_str("?L,0").unwrap();
        assert_eq!("?L,0", I2CResponse::to_string(&reply));
        let reply = <LedStatus as I2CResponse>::from_str("?L,1").unwrap();
        assert_eq!("?L,1", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_led_status_reply_from_invalid_str_yields_err() {
        let reply = <LedStatus as I2CResponse>::from_str("?L,");
        assert!(reply.is_err());
        let reply = <LedStatus as I2CResponse>::from_str("?L,1,0");
        assert!(reply.is_err());
        let reply = <LedStatus as I2CResponse>::from_str("?L,10");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_valid_str() {
        let reply = <ProtocolLockStatus as I2CResponse>::from_str("?PLOCK,0").unwrap();
        assert_eq!("?PLOCK,0", I2CResponse::to_string(&reply));
        let reply = <ProtocolLockStatus as I2CResponse>::from_str("?PLOCK,1").unwrap();
        assert_eq!("?PLOCK,1", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_invalid_str_yields_err() {
        let reply = <ProtocolLockStatus as I2CResponse>::from_str("?PLOCK,");
        assert!(reply.is_err());
        let reply = <ProtocolLockStatus as I2CResponse>::from_str("?PLOCK,1,0");
        assert!(reply.is_err());
        let reply = <ProtocolLockStatus as I2CResponse>::from_str("?PLOCK,off");
        assert!(reply.is_err());
    }
}
