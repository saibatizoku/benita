//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
use errors::*;
use network::{Endpoint, SocketReply};

pub use devices::conductivity::responses::{CalibrationStatus, CompensationValue, DeviceInfo,
                                           DeviceStatus, Exported, ExportedInfo, LedStatus,
                                           OutputStringStatus, ProbeReading, ProbeType,
                                           ProtocolLockStatus};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calibration_status_reply_from_valid_str() {
        let reply = CalibrationStatus::parse_response("?CAL,0").unwrap();
        assert_eq!("none", &reply.to_reply_string());
        let reply = CalibrationStatus::parse_response("?CAL,1").unwrap();
        assert_eq!("one-point", &reply.to_reply_string());
        let reply = CalibrationStatus::parse_response("?CAL,2").unwrap();
        assert_eq!("two-point", &reply.to_reply_string());
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = CalibrationStatus::parse_response("?CAL,");
        assert!(reply.is_err());
        let reply = CalibrationStatus::parse_response("?CAL,3");
        assert!(reply.is_err());
        let reply = CalibrationStatus::parse_response("?CAL,11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_compensation_value_reply_from_valid_str() {
        let reply = CompensationValue::parse_response("?T,0").unwrap();
        assert_eq!("0.000", &reply.to_reply_string());
        let reply = CompensationValue::parse_response("?T,-15.23").unwrap();
        assert_eq!("-15.230", &reply.to_reply_string());
        let reply = CompensationValue::parse_response("?T,1500.0446").unwrap();
        assert_eq!("1500.045", &reply.to_reply_string());
    }

    #[test]
    fn parse_compensation_value_reply_from_invalid_str_yields_err() {
        let reply = CompensationValue::parse_response("?T,");
        assert!(reply.is_err());
        let reply = CompensationValue::parse_response("?T,C11");
        assert!(reply.is_err());
    }

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
    fn parse_output_string_status_reply_from_valid_str() {
        let reply = OutputStringStatus::parse_response("?O,EC,TDS,S,SG").unwrap();
        assert_eq!("EC,TDS,S,SG", &reply.to_reply_string());
    }

    #[test]
    fn parse_output_string_status_reply_from_invalid_str_yields_err() {
        let reply = OutputStringStatus::parse_response("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_reading_reply_from_valid_str() {
        let reply = ProbeReading::parse_response("0.1").unwrap();
        assert_eq!("0.1", &reply.to_reply_string());
        let reply = ProbeReading::parse_response("1.0,0.05").unwrap();
        assert_eq!("1,0.05", &reply.to_reply_string());
        let reply = ProbeReading::parse_response("10.0").unwrap();
        assert_eq!("10", &reply.to_reply_string());
    }

    #[test]
    fn parse_probe_reading_reply_from_invalid_str_yields_err() {
        let reply = ProbeReading::parse_response("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_type_reply_from_valid_str() {
        let reply = ProbeType::parse_response("?K,0.1").unwrap();
        assert_eq!("0.1", &reply.to_reply_string());
        let reply = ProbeType::parse_response("?K,1.0").unwrap();
        assert_eq!("1.0", &reply.to_reply_string());
        let reply = ProbeType::parse_response("?K,10.0").unwrap();
        assert_eq!("10", &reply.to_reply_string());
    }

    #[test]
    fn parse_probe_type_reply_from_invalid_str_yields_err() {
        let reply = ProbeType::parse_response("");
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
