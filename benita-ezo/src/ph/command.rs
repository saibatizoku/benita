//! Commands from EZO PH chipset.
use errors::*;

pub use ezo_ph::command::Command;

pub use ezo_ph::command::Baud;
pub use ezo_ph::command::{CalibrationClear, CalibrationHigh, CalibrationLow, CalibrationMid,
                          CalibrationState};
pub use ezo_ph::command::{CompensatedTemperatureValue as CompensationGet, DeviceAddress,
                          TemperatureCompensation as CompensationSet};
pub use ezo_ph::command::{DeviceInformation, Factory, Find, Reading, Sleep, Status};
pub use ezo_ph::command::{Export, ExportInfo, Import};
pub use ezo_ph::command::{LedOff, LedOn, LedState};
pub use ezo_ph::command::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
pub use ezo_ph::command::Slope;

pub use devices::{I2CCommand, I2CResponse, SensorDevice};

use super::response::*;

impl_I2CCommand_for!(CalibrationHigh, ResponseStatus);
impl_I2CCommand_for!(CalibrationLow, ResponseStatus);
impl_I2CCommand_for!(CalibrationMid, ResponseStatus);
impl_I2CCommand_for!(CalibrationState, CalibrationStatus);
impl_I2CCommand_for!(CompensationGet, CompensationValue);
impl_I2CCommand_for!(CompensationSet, ResponseStatus);
impl_I2CCommand_for!(Reading, SensorReading);
impl_I2CCommand_for!(Slope, ProbeSlope);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ph_calibration_high_command_from_valid_str() {
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,1000.3324").unwrap();
        assert_eq!("CAL,HIGH,1000.33", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_calibration_high_command_from_invalid_str_yields_err() {
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,");
        assert!(request.is_err());

        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,a");
        assert!(request.is_err());

        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,23sa.31");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_low_command_from_valid_str() {
        let request = <CalibrationLow as I2CCommand>::from_str("cal,low,1000.3324").unwrap();
        assert_eq!("CAL,LOW,1000.33", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_calibration_low_command_from_invalid_str_yields_err() {
        let request = <CalibrationLow as I2CCommand>::from_str("cal,low");
        assert!(request.is_err());

        let request = <CalibrationLow as I2CCommand>::from_str("cal,low,");
        assert!(request.is_err());

        let request = <CalibrationLow as I2CCommand>::from_str("cal,low,123a2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_mid_command_from_valid_str() {
        let request = <CalibrationMid as I2CCommand>::from_str("cal,mid,-100.4").unwrap();
        assert_eq!("CAL,MID,-100.40", I2CCommand::to_string(&request));
        let request = <CalibrationMid as I2CCommand>::from_str("cal,mid,1000.3324").unwrap();
        assert_eq!("CAL,MID,1000.33", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_calibration_mid_command_from_invalid_str_yields_err() {
        let request = <CalibrationMid as I2CCommand>::from_str("cal,mid,");
        assert!(request.is_err());

        let request = <CalibrationMid as I2CCommand>::from_str("cal,mid,a");
        assert!(request.is_err());

        let request = <CalibrationMid as I2CCommand>::from_str("cal,mid,10.1a");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_calibration_status_command_from_valid_str() {
        let request = <CalibrationState as I2CCommand>::from_str("cal,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
        let request = <CalibrationState as I2CCommand>::from_str("CAL,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_calibration_status_command_from_invalid_str_yields_err() {
        let request = <CalibrationState as I2CCommand>::from_str("cal,?,");
        assert!(request.is_err());

        let request = <CalibrationState as I2CCommand>::from_str("cal,??");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_compensation_get_command_from_valid_str() {
        let request = <CompensationGet as I2CCommand>::from_str("t,?").unwrap();
        assert_eq!("T,?", I2CCommand::to_string(&request));
        let request = <CompensationGet as I2CCommand>::from_str("T,?").unwrap();
        assert_eq!("T,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_compensation_get_command_from_invalid_str_yields_err() {
        let request = <CompensationGet as I2CCommand>::from_str("t,?,");
        assert!(request.is_err());

        let request = <CompensationGet as I2CCommand>::from_str("t,?,10.5829");
        assert!(request.is_err());

        let request = <CompensationGet as I2CCommand>::from_str("t,??");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_compensation_set_command_from_valid_str() {
        let request = <CompensationSet as I2CCommand>::from_str("t,0.2999").unwrap();
        assert_eq!("T,0.300", I2CCommand::to_string(&request));
        let request = <CompensationSet as I2CCommand>::from_str("t,10.5829").unwrap();
        assert_eq!("T,10.583", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_compensation_set_command_from_invalid_str_yields_err() {
        let request = <CompensationSet as I2CCommand>::from_str("t,");
        assert!(request.is_err());

        let request = <CompensationSet as I2CCommand>::from_str("t,10.5o69");
        assert!(request.is_err());

        let request = <CompensationSet as I2CCommand>::from_str("t,10.5869,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_ph_read_command_from_valid_str() {
        let request = <Reading as I2CCommand>::from_str("r").unwrap();
        assert_eq!("R", I2CCommand::to_string(&request));
        let request = <Reading as I2CCommand>::from_str("R").unwrap();
        assert_eq!("R", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_read_command_from_invalid_str_yields_err() {
        let request = <Reading as I2CCommand>::from_str("r,");
        assert!(request.is_err());
        let request = <Reading as I2CCommand>::from_str("read");
        assert!(request.is_err());
    }
    #[test]
    fn parse_ph_slope_command_from_valid_str() {
        let request = <Slope as I2CCommand>::from_str("slope,?").unwrap();
        assert_eq!("SLOPE,?", I2CCommand::to_string(&request));
        let request = <Slope as I2CCommand>::from_str("SLOPE,?").unwrap();
        assert_eq!("SLOPE,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_ph_slope_command_from_invalid_str_yields_err() {
        let request = <Slope as I2CCommand>::from_str("SLOPEING");
        assert!(request.is_err());
    }
}
