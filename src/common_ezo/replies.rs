//! Replies from EZO sensors.
use errors::*;
use network::{Endpoint, SocketReply};

pub use super::response::*;

impl_SocketReply_for!(DeviceInfo);
impl_SocketReply_for!(DeviceStatus);
impl_SocketReply_for!(Exported);
impl_SocketReply_for!(ExportedInfo);
impl_SocketReply_for!(LedStatus);
impl_SocketReply_for!(ProtocolLockStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_device_info_reply_from_valid_str() {
        let reply = DeviceInfo::parse_response("?I,EC,0.0.0").unwrap();
        assert_eq!("EC,0.0.0", &reply.to_reply_string());
        let reply = DeviceInfo::parse_response("?I,device,firmware").unwrap();
        assert_eq!("device,firmware", &reply.to_reply_string());
    }

    #[test]
    fn parse_device_info_reply_from_invalid_str_yields_err() {
        let reply = DeviceInfo::parse_response("?I,");
        assert!(reply.is_err());
        let reply = DeviceInfo::parse_response("?I,3");
        assert!(reply.is_err());
        let reply = DeviceInfo::parse_response("?I,S,L,4");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_status_reply_from_valid_str() {
        let reply = DeviceStatus::parse_response("?STATUS,P,0").unwrap();
        assert_eq!("powered-off,0.000", &reply.to_reply_string());
        let reply = DeviceStatus::parse_response("?STATUS,S,1").unwrap();
        assert_eq!("software-reset,1.000", &reply.to_reply_string());
        let reply = DeviceStatus::parse_response("?STATUS,B,2").unwrap();
        assert_eq!("brown-out,2.000", &reply.to_reply_string());
        let reply = DeviceStatus::parse_response("?STATUS,W,3").unwrap();
        assert_eq!("watchdog,3.000", &reply.to_reply_string());
        let reply = DeviceStatus::parse_response("?STATUS,U,4.505").unwrap();
        assert_eq!("unknown,4.505", &reply.to_reply_string());
    }

    #[test]
    fn parse_device_status_reply_from_invalid_str_yields_err() {
        let reply = DeviceStatus::parse_response("?STATUS,");
        assert!(reply.is_err());
        let reply = DeviceStatus::parse_response("?STATUS,3");
        assert!(reply.is_err());
        let reply = DeviceStatus::parse_response("?STATUS,S,L");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_reply_from_valid_str() {
        let reply = Exported::parse_response("uptotwelvech").unwrap();
        assert_eq!("uptotwelvech", &reply.to_reply_string());
        let reply = Exported::parse_response("*DONE").unwrap();
        assert_eq!("DONE", &reply.to_reply_string());
    }

    #[test]
    fn parse_exported_reply_from_invalid_str_yields_err() {
        let reply = Exported::parse_response("uptotwelvechars");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_info_reply_from_valid_str() {
        let reply = ExportedInfo::parse_response("?EXPORT,1,1").unwrap();
        assert_eq!("1,1", &reply.to_reply_string());
    }

    #[test]
    fn parse_exported_info_reply_from_invalid_str_yields_err() {
        let reply = ExportedInfo::parse_response("?EXPORT,,");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_led_status_reply_from_valid_str() {
        let reply = LedStatus::parse_response("?L,0").unwrap();
        assert_eq!("off", &reply.to_reply_string());
        let reply = LedStatus::parse_response("?L,1").unwrap();
        assert_eq!("on", &reply.to_reply_string());
    }

    #[test]
    fn parse_led_status_reply_from_invalid_str_yields_err() {
        let reply = LedStatus::parse_response("?L,");
        assert!(reply.is_err());
        let reply = LedStatus::parse_response("?L,1,0");
        assert!(reply.is_err());
        let reply = LedStatus::parse_response("?L,10");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_valid_str() {
        let reply = ProtocolLockStatus::parse_response("?PLOCK,0").unwrap();
        assert_eq!("off", &reply.to_reply_string());
        let reply = ProtocolLockStatus::parse_response("?PLOCK,1").unwrap();
        assert_eq!("on", &reply.to_reply_string());
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_invalid_str_yields_err() {
        let reply = ProtocolLockStatus::parse_response("?PLOCK,");
        assert!(reply.is_err());
        let reply = ProtocolLockStatus::parse_response("?PLOCK,1,0");
        assert!(reply.is_err());
        let reply = ProtocolLockStatus::parse_response("?PLOCK,off");
        assert!(reply.is_err());
    }
}
