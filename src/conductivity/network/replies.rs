//! Replies from the conductivity sensor. `Reply`s are received after a `Request`.
pub mod errors {
    error_chain! {
    }
}

use errors::*;
use network::{Endpoint, SocketReply};

pub use conductivity::response::*;

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
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = SensorReading::parse_response("0.1").unwrap();
        assert_eq!("0.1", &reply.to_reply_string());
        let reply = SensorReading::parse_response("1.0,0.05").unwrap();
        assert_eq!("1,0.05", &reply.to_reply_string());
        let reply = SensorReading::parse_response("10.0").unwrap();
        assert_eq!("10", &reply.to_reply_string());
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = SensorReading::parse_response("");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_type_reply_from_valid_str() {
        let reply = ProbeType::parse_response("?K,0.1").unwrap();
        assert_eq!("0.1", &reply.to_reply_string());
        let reply = ProbeType::parse_response("?K,1.0").unwrap();
        assert_eq!("1.0", &reply.to_reply_string());
        let reply = ProbeType::parse_response("?K,10.0").unwrap();
        assert_eq!("10.0", &reply.to_reply_string());
    }

    #[test]
    fn parse_probe_type_reply_from_invalid_str_yields_err() {
        let reply = ProbeType::parse_response("");
        assert!(reply.is_err());
    }
}
