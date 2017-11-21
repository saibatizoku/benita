//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
pub mod errors {
    error_chain!{}
}

use errors::*;
use network::Endpoint;

pub use conductivity::response::{CalibrationStatus, CompensationValue, DeviceInfo, DeviceStatus,
                                 Exported, ExportedInfo, LedStatus, OutputStringStatus, ProbeType,
                                 ProtocolLockStatus, SensorReading};
pub use network::SocketReply;

// Basically, wrap existing responses from the original sensor crate.
impl_SocketReply_for!(CalibrationStatus);
impl_SocketReply_for!(CompensationValue);
impl_SocketReply_for!(OutputStringStatus);
impl_SocketReply_for!(SensorReading);
impl_SocketReply_for!(ProbeType);

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
    }

    #[test]
    fn parse_calibration_status_reply_from_invalid_str_yields_err() {
        let reply = CalibrationStatus::from_str("?CAL,");
        assert!(reply.is_err());
        let reply = CalibrationStatus::from_str("?CAL,3");
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
    fn parse_output_string_status_reply_from_valid_str() {
        let reply = OutputStringStatus::from_str("?O,EC,TDS,S,SG").unwrap();
        assert_eq!("EC,TDS,S,SG", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_output_string_status_reply_from_invalid_str_yields_err() {
        let reply = OutputStringStatus::from_str("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = SensorReading::from_str("0.1").unwrap();
        assert_eq!("0.1", SocketReply::to_string(&reply));
        let reply = SensorReading::from_str("1.0,0.05").unwrap();
        assert_eq!("1,0.05", SocketReply::to_string(&reply));
        let reply = SensorReading::from_str("10.0").unwrap();
        assert_eq!("10", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = SensorReading::from_str("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_type_reply_from_valid_str() {
        let reply = ProbeType::from_str("?K,0.1").unwrap();
        assert_eq!("0.1", SocketReply::to_string(&reply));
        let reply = ProbeType::from_str("?K,1.0").unwrap();
        assert_eq!("1.0", SocketReply::to_string(&reply));
        let reply = ProbeType::from_str("?K,10.0").unwrap();
        assert_eq!("10.0", SocketReply::to_string(&reply));
    }

    #[test]
    fn parse_probe_type_reply_from_invalid_str_yields_err() {
        let reply = ProbeType::from_str("");
        assert!(reply.is_err());
    }
}
