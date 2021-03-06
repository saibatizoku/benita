//! Replies from EZO sensors.
pub use super::response::*;

use errors::*;
use network::{Endpoint, SocketReply};

impl_SocketReply_for!(DeviceInfo);
impl_SocketReply_for!(DeviceStatus);
impl_SocketReply_for!(Exported);
impl_SocketReply_for!(ExportedInfo);
impl_SocketReply_for!(LedStatus);
impl_SocketReply_for!(ProtocolLockStatus);
impl_SocketReply_for!(ResponseStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_response_status_reply_from_valid_str() {
        let reply = <ResponseStatus as SocketReply>::from_str("Ack").unwrap();
        assert_eq!("Ack", SocketReply::to_string(&reply));
        let reply = <ResponseStatus as SocketReply>::from_str("None").unwrap();
        assert_eq!("None", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_response_status_reply_from_invalid_str_yields_err() {
        let reply = <ResponseStatus as SocketReply>::from_str("()");
        assert!(reply.is_err());
        let reply = <ResponseStatus as SocketReply>::from_str("ack");
        assert!(reply.is_err());
        let reply = <ResponseStatus as SocketReply>::from_str("none");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_info_reply_from_valid_str() {
        let reply = <DeviceInfo as SocketReply>::from_str("?I,EC,0.0.0").unwrap();
        assert_eq!("EC,0.0.0", SocketReply::to_string(&reply));
        let reply = <DeviceInfo as SocketReply>::from_str("?I,device,firmware").unwrap();
        assert_eq!("device,firmware", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_device_info_reply_from_invalid_str_yields_err() {
        let reply = <DeviceInfo as SocketReply>::from_str("?I,");
        assert!(reply.is_err());
        let reply = <DeviceInfo as SocketReply>::from_str("?I,3");
        assert!(reply.is_err());
        let reply = <DeviceInfo as SocketReply>::from_str("?I,S,L,4");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_status_reply_from_valid_str() {
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,P,0").unwrap();
        assert_eq!("powered-off,0.000", SocketReply::to_string(&reply));
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,S,1").unwrap();
        assert_eq!("software-reset,1.000", SocketReply::to_string(&reply));
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,B,2").unwrap();
        assert_eq!("brown-out,2.000", SocketReply::to_string(&reply));
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,W,3").unwrap();
        assert_eq!("watchdog,3.000", SocketReply::to_string(&reply));
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,U,4.505").unwrap();
        assert_eq!("unknown,4.505", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_device_status_reply_from_invalid_str_yields_err() {
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,");
        assert!(reply.is_err());
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,3");
        assert!(reply.is_err());
        let reply = <DeviceStatus as SocketReply>::from_str("?STATUS,S,L");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_reply_from_valid_str() {
        let reply = <Exported as SocketReply>::from_str("uptotwelvech").unwrap();
        assert_eq!("uptotwelvech", SocketReply::to_string(&reply));
        let reply = <Exported as SocketReply>::from_str("*DONE").unwrap();
        assert_eq!("DONE", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_exported_reply_from_invalid_str_yields_err() {
        let reply = <Exported as SocketReply>::from_str("uptotwelvechars");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_info_reply_from_valid_str() {
        let reply = <ExportedInfo as SocketReply>::from_str("?EXPORT,1,1").unwrap();
        assert_eq!("1,1", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_exported_info_reply_from_invalid_str_yields_err() {
        let reply = <ExportedInfo as SocketReply>::from_str("?EXPORT,,");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_led_status_reply_from_valid_str() {
        let reply = <LedStatus as SocketReply>::from_str("?L,0").unwrap();
        assert_eq!("off", SocketReply::to_string(&reply));
        let reply = <LedStatus as SocketReply>::from_str("?L,1").unwrap();
        assert_eq!("on", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_led_status_reply_from_invalid_str_yields_err() {
        let reply = <LedStatus as SocketReply>::from_str("?L,");
        assert!(reply.is_err());
        let reply = <LedStatus as SocketReply>::from_str("?L,1,0");
        assert!(reply.is_err());
        let reply = <LedStatus as SocketReply>::from_str("?L,10");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_valid_str() {
        let reply = <ProtocolLockStatus as SocketReply>::from_str("?PLOCK,0").unwrap();
        assert_eq!("off", SocketReply::to_string(&reply));
        let reply = <ProtocolLockStatus as SocketReply>::from_str("?PLOCK,1").unwrap();
        assert_eq!("on", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_invalid_str_yields_err() {
        let reply = <ProtocolLockStatus as SocketReply>::from_str("?PLOCK,");
        assert!(reply.is_err());
        let reply = <ProtocolLockStatus as SocketReply>::from_str("?PLOCK,1,0");
        assert!(reply.is_err());
        let reply = <ProtocolLockStatus as SocketReply>::from_str("?PLOCK,off");
        assert!(reply.is_err());
    }
}
