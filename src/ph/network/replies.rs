//! Replies from the pH sensor. `Reply`s are received after a `Request`.
pub mod errors {
    error_chain!{}
}

use errors::*;
use network::{Endpoint, SocketReply};

pub use ph::response::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus, Exported,
                       ExportedInfo, LedStatus, ProbeSlope, ProtocolLockStatus, SensorReading};

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(CompensationValue);
impl_SocketReply_for!(ProbeSlope);
impl_SocketReply_for!(SensorReading);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calibration_status_reply_from_valid_str() {
        let reply = CalibrationStatus::from_str("?CAL,0").unwrap();
        assert_eq!("none", SocketReply::to_string(&reply));
        let reply = CalibrationStatus::from_str("?CAL,1").unwrap();
        assert_eq!("one-point", SocketReply::to_string(&reply));
        let reply = CalibrationStatus::from_str("?CAL,2").unwrap();
        assert_eq!("two-point", SocketReply::to_string(&reply));
        let reply = CalibrationStatus::from_str("?CAL,3").unwrap();
        assert_eq!("three-point", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = CalibrationStatus::from_str("?CAL,");
        assert!(reply.is_err());
        let reply = CalibrationStatus::from_str("?CAL,4");
        assert!(reply.is_err());
        let reply = CalibrationStatus::from_str("?CAL,11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_compensation_value_reply_from_valid_str() {
        let reply = CompensationValue::from_str("?T,0").unwrap();
        assert_eq!("0.000", SocketReply::to_string(&reply));
        let reply = CompensationValue::from_str("?T,-15.23").unwrap();
        assert_eq!("-15.230", SocketReply::to_string(&reply));
        let reply = CompensationValue::from_str("?T,1500.0446").unwrap();
        assert_eq!("1500.045", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_compensation_value_reply_from_invalid_str_yields_err() {
        let reply = CompensationValue::from_str("?T,");
        assert!(reply.is_err());
        let reply = CompensationValue::from_str("?T,C11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_info_reply_from_valid_str() {
        let reply = DeviceInfo::from_str("?I,EC,0.0.0").unwrap();
        assert_eq!("EC,0.0.0", SocketReply::to_string(&reply));
        let reply = DeviceInfo::from_str("?I,device,firmware").unwrap();
        assert_eq!("device,firmware", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_device_info_reply_from_invalid_str_yields_err() {
        let reply = DeviceInfo::from_str("?I,");
        assert!(reply.is_err());
        let reply = DeviceInfo::from_str("?I,3");
        assert!(reply.is_err());
        let reply = DeviceInfo::from_str("?I,S,L,4");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_device_status_reply_from_valid_str() {
        let reply = DeviceStatus::from_str("?STATUS,P,0").unwrap();
        assert_eq!("powered-off,0.000", SocketReply::to_string(&reply));
        let reply = DeviceStatus::from_str("?STATUS,S,1").unwrap();
        assert_eq!("software-reset,1.000", SocketReply::to_string(&reply));
        let reply = DeviceStatus::from_str("?STATUS,B,2").unwrap();
        assert_eq!("brown-out,2.000", SocketReply::to_string(&reply));
        let reply = DeviceStatus::from_str("?STATUS,W,3").unwrap();
        assert_eq!("watchdog,3.000", SocketReply::to_string(&reply));
        let reply = DeviceStatus::from_str("?STATUS,U,4.505").unwrap();
        assert_eq!("unknown,4.505", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_device_status_reply_from_invalid_str_yields_err() {
        let reply = DeviceStatus::from_str("?STATUS,");
        assert!(reply.is_err());
        let reply = DeviceStatus::from_str("?STATUS,3");
        assert!(reply.is_err());
        let reply = DeviceStatus::from_str("?STATUS,S,L");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_reply_from_valid_str() {
        let reply = Exported::from_str("uptotwelvech").unwrap();
        assert_eq!("uptotwelvech", SocketReply::to_string(&reply));
        let reply = Exported::from_str("*DONE").unwrap();
        assert_eq!("DONE", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_exported_reply_from_invalid_str_yields_err() {
        let reply = Exported::from_str("uptotwelvechars");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_exported_info_reply_from_valid_str() {
        let reply = ExportedInfo::from_str("?EXPORT,1,1").unwrap();
        assert_eq!("1,1", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_exported_info_reply_from_invalid_str_yields_err() {
        let reply = ExportedInfo::from_str("?EXPORT,,");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_led_status_reply_from_valid_str() {
        let reply = LedStatus::from_str("?L,0").unwrap();
        assert_eq!("off", SocketReply::to_string(&reply));
        let reply = LedStatus::from_str("?L,1").unwrap();
        assert_eq!("on", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_led_status_reply_from_invalid_str_yields_err() {
        let reply = LedStatus::from_str("?L,");
        assert!(reply.is_err());
        let reply = LedStatus::from_str("?L,1,0");
        assert!(reply.is_err());
        let reply = LedStatus::from_str("?L,10");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_slope_reply_from_valid_str() {
        let reply = ProbeSlope::from_str("?SLOPE,10,0").unwrap();
        assert_eq!("10.000,0.000", SocketReply::to_string(&reply));
        let reply = ProbeSlope::from_str("?SLOPE,1,320000").unwrap();
        assert_eq!("1.000,320000.000", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_probe_slope_reply_from_invalid_str_yields_err() {
        let reply = ProbeSlope::from_str("?SLOPE,D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_valid_str() {
        let reply = ProtocolLockStatus::from_str("?PLOCK,0").unwrap();
        assert_eq!("off", SocketReply::to_string(&reply));
        let reply = ProtocolLockStatus::from_str("?PLOCK,1").unwrap();
        assert_eq!("on", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_protocol_lock_status_reply_from_invalid_str_yields_err() {
        let reply = ProtocolLockStatus::from_str("?PLOCK,");
        assert!(reply.is_err());
        let reply = ProtocolLockStatus::from_str("?PLOCK,1,0");
        assert!(reply.is_err());
        let reply = ProtocolLockStatus::from_str("?PLOCK,off");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = SensorReading::from_str("0.1").unwrap();
        assert_eq!("0.100", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = SensorReading::from_str("");
        assert!(reply.is_err());
        let reply = SensorReading::from_str("1.0,0.05");
        assert!(reply.is_err());
    }
}
