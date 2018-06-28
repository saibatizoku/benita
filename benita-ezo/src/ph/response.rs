//! Responses from EZO PH chipset.
pub use common_ezo::response::{
    DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus, ProtocolLockStatus, ResponseStatus,
};
use devices::I2CResponse;
use errors::*;
pub use ezo_ph::response::{CalibrationStatus, CompensationValue, ProbeSlope, SensorReading};

impl_I2CResponse_for!(CalibrationStatus);
impl_I2CResponse_for!(CompensationValue);
impl_I2CResponse_for!(ProbeSlope);
impl_I2CResponse_for!(SensorReading);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_calibration_status_reply_from_valid_str() {
        let reply = CalibrationStatus::from_str("?CAL,0").unwrap();
        assert_eq!("?CAL,0", I2CResponse::to_string(&reply));
        let reply = CalibrationStatus::from_str("?CAL,1").unwrap();
        assert_eq!("?CAL,1", I2CResponse::to_string(&reply));
        let reply = CalibrationStatus::from_str("?CAL,2").unwrap();
        assert_eq!("?CAL,2", I2CResponse::to_string(&reply));
        let reply = CalibrationStatus::from_str("?CAL,3").unwrap();
        assert_eq!("?CAL,3", I2CResponse::to_string(&reply));
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
        assert_eq!("?T,0.000", I2CResponse::to_string(&reply));
        let reply = CompensationValue::from_str("?T,-15.23").unwrap();
        assert_eq!("?T,-15.230", I2CResponse::to_string(&reply));
        let reply = CompensationValue::from_str("?T,1500.0446").unwrap();
        assert_eq!("?T,1500.045", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_compensation_value_reply_from_invalid_str_yields_err() {
        let reply = CompensationValue::from_str("?T,");
        assert!(reply.is_err());
        let reply = CompensationValue::from_str("?T,C11");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_probe_slope_reply_from_valid_str() {
        let reply = ProbeSlope::from_str("?SLOPE,10,0").unwrap();
        assert_eq!("?SLOPE,10.000,0.000", I2CResponse::to_string(&reply));
        let reply = ProbeSlope::from_str("?SLOPE,1,320000").unwrap();
        assert_eq!("?SLOPE,1.000,320000.000", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_probe_slope_reply_from_invalid_str_yields_err() {
        let reply = ProbeSlope::from_str("?SLOPE,D,320_001");
        assert!(reply.is_err());
    }

    #[test]
    fn parse_sensor_reading_reply_from_valid_str() {
        let reply = <SensorReading as I2CResponse>::from_str("0.1").unwrap();
        assert_eq!("0.100", I2CResponse::to_string(&reply));
    }

    #[test]
    fn parse_sensor_reading_reply_from_invalid_str_yields_err() {
        let reply = <SensorReading as I2CResponse>::from_str("");
        assert!(reply.is_err());
        let reply = <SensorReading as I2CResponse>::from_str("1.0,0.05");
        assert!(reply.is_err());
    }
}
