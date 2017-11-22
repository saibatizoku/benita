//! Commands from EZO EC chipset.
use errors::*;

pub use ezo_ec::command::Command;

pub use ezo_ec::command::Baud;
pub use ezo_ec::command::{CalibrationClear, CalibrationDry, CalibrationHigh, CalibrationLow,
                          CalibrationOnePoint, CalibrationState};
pub use ezo_ec::command::{CompensatedTemperatureValue as CompensationGet,
                          TemperatureCompensation as CompensationSet};
pub use ezo_ec::command::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep, Status};
pub use ezo_ec::command::{Export, ExportInfo, Import};
pub use ezo_ec::command::{LedOff, LedOn, LedState};
pub use ezo_ec::command::{OutputDisableConductivity, OutputDisableSalinity,
                          OutputDisableSpecificGravity, OutputDisableTds,
                          OutputEnableConductivity, OutputEnableSalinity,
                          OutputEnableSpecificGravity, OutputEnableTds, OutputState};
pub use ezo_ec::command::{ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen};
pub use ezo_ec::command::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};

pub use devices::{I2CCommand, I2CResponse, SensorDevice};

use super::response::*;

impl_I2CCommand_for!(CalibrationDry, ResponseStatus);
impl_I2CCommand_for!(CalibrationHigh, ResponseStatus);
impl_I2CCommand_for!(CalibrationLow, ResponseStatus);
impl_I2CCommand_for!(CalibrationOnePoint, ResponseStatus);
impl_I2CCommand_for!(CalibrationState, CalibrationStatus);
impl_I2CCommand_for!(CompensationGet, CompensationValue);
impl_I2CCommand_for!(CompensationSet, ResponseStatus);
impl_I2CCommand_for!(Reading, SensorReading);
impl_I2CCommand_for!(OutputDisableConductivity, ResponseStatus);
impl_I2CCommand_for!(OutputDisableSalinity, ResponseStatus);
impl_I2CCommand_for!(OutputDisableSpecificGravity, ResponseStatus);
impl_I2CCommand_for!(OutputDisableTds, ResponseStatus);
impl_I2CCommand_for!(OutputEnableConductivity, ResponseStatus);
impl_I2CCommand_for!(OutputEnableSalinity, ResponseStatus);
impl_I2CCommand_for!(OutputEnableSpecificGravity, ResponseStatus);
impl_I2CCommand_for!(OutputEnableTds, ResponseStatus);
impl_I2CCommand_for!(OutputState, OutputStringStatus);
impl_I2CCommand_for!(ProbeTypeOne, ResponseStatus);
impl_I2CCommand_for!(ProbeTypePointOne, ResponseStatus);
impl_I2CCommand_for!(ProbeTypeState, ProbeType);
impl_I2CCommand_for!(ProbeTypeTen, ResponseStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_conductivity_calibration_dry_request_from_valid_str() {
        let request = <CalibrationDry as I2CCommand>::from_str("cal,dry").unwrap();
        assert_eq!("CAL,DRY", I2CCommand::to_string(&request));
        let request = <CalibrationDry as I2CCommand>::from_str("CAL,DRY").unwrap();
        assert_eq!("CAL,DRY", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_calibration_dry_request_from_invalid_str_yields_err() {
        let request = <CalibrationDry as I2CCommand>::from_str("cal,dry,");
        assert!(request.is_err());

        let request = <CalibrationDry as I2CCommand>::from_str("cal,dry,123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_high_request_from_valid_str() {
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,1000.3324").unwrap();
        assert_eq!(
            "CAL,HIGH,1000.33",
            I2CCommand::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_calibration_high_request_from_invalid_str_yields_err() {
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,");
        assert!(request.is_err());
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,s");
        assert!(request.is_err());
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,1s");
        assert!(request.is_err());
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,high,1,1");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_low_request_from_valid_str() {
        let request = <CalibrationLow as I2CCommand>::from_str("cal,low,1000.3324").unwrap();
        assert_eq!(
            "CAL,LOW,1000.33",
            I2CCommand::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_calibration_low_request_from_invalid_str_yields_err() {
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,low,");
        assert!(request.is_err());
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,low,s");
        assert!(request.is_err());
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,low,1s");
        assert!(request.is_err());
        let request = <CalibrationHigh as I2CCommand>::from_str("cal,low,1,1");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_onepoint_request_from_valid_str() {
        let request = <CalibrationOnePoint as I2CCommand>::from_str("CAL,1000.3324").unwrap();
        assert_eq!(
            "CAL,1000.33",
            I2CCommand::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_calibration_onepoint_request_from_invalid_str_yields_err() {
        let request = <CalibrationOnePoint as I2CCommand>::from_str("cal,");
        assert!(request.is_err());

        let request = <CalibrationOnePoint as I2CCommand>::from_str("cal,a");
        assert!(request.is_err());

        let request = <CalibrationOnePoint as I2CCommand>::from_str("cal,123a2342");
        assert!(request.is_err());

        let request = <CalibrationOnePoint as I2CCommand>::from_str("cal,123,2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_calibration_status_request_from_valid_str() {
        let request = <CalibrationState as I2CCommand>::from_str("cal,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
        let request = <CalibrationState as I2CCommand>::from_str("CAL,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_calibration_status_request_from_invalid_str_yields_err() {
        let request = <CalibrationState as I2CCommand>::from_str("cal,?,");
        assert!(request.is_err());

        let request = <CalibrationState as I2CCommand>::from_str("cal,?,1");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_valid_str() {
        let request = <CompensationGet as I2CCommand>::from_str("t,?").unwrap();
        assert_eq!("T,?", I2CCommand::to_string(&request));
        let request = <CompensationGet as I2CCommand>::from_str("T,?").unwrap();
        assert_eq!("T,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_invalid_str_yields_err() {
        let request = <CompensationGet as I2CCommand>::from_str("t,?,");
        assert!(request.is_err());

        let request = <CompensationGet as I2CCommand>::from_str("t,? ");
        assert!(request.is_err());

        let request = <CompensationGet as I2CCommand>::from_str("t,??");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_valid_str() {
        let request = <CompensationSet as I2CCommand>::from_str("t,10.5829").unwrap();
        assert_eq!(
            "T,10.583",
            I2CCommand::to_string(&request)
        );
        let request = <CompensationSet as I2CCommand>::from_str("T,10.5829").unwrap();
        assert_eq!(
            "T,10.583",
            I2CCommand::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_invalid_str_yields_err() {
        let request = <CompensationSet as I2CCommand>::from_str("T,");
        assert!(request.is_err());
        let request = <CompensationSet as I2CCommand>::from_str("t,10.5869,");
        assert!(request.is_err());
        let request = <CompensationSet as I2CCommand>::from_str("t,10.5869,12");
        assert!(request.is_err());
        let request = <CompensationSet as I2CCommand>::from_str("t,10.58s69");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_conductivity_off_request_from_valid_str() {
        let request = <OutputDisableConductivity as I2CCommand>::from_str("o,ec,0").unwrap();
        assert_eq!(
            "O,EC,0",
            I2CCommand::to_string(&request)
        );
        let request = <OutputDisableConductivity as I2CCommand>::from_str("O,EC,0").unwrap();
        assert_eq!(
            "O,EC,0",
            I2CCommand::to_string(&request)
        );
    }

    #[test]
    fn parse_conductivity_output_conductivity_off_request_from_invalid_str_yields_err() {
        let request = <OutputDisableConductivity as I2CCommand>::from_str("o,ec,");
        assert!(request.is_err());
        let request = <OutputDisableConductivity as I2CCommand>::from_str("o,ec,0,");
        assert!(request.is_err());
        let request = <OutputDisableConductivity as I2CCommand>::from_str("o,ec,00");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_salinity_off_request_from_valid_str() {
        let request = <OutputDisableSalinity as I2CCommand>::from_str("o,s,0").unwrap();
        assert_eq!("O,S,0", I2CCommand::to_string(&request));
        let request = <OutputDisableSalinity as I2CCommand>::from_str("O,S,0").unwrap();
        assert_eq!("O,S,0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_salinity_off_request_from_invalid_str_yields_err() {
        let request = <OutputDisableSalinity as I2CCommand>::from_str("o,s,");
        assert!(request.is_err());

        let request = <OutputDisableSalinity as I2CCommand>::from_str("o,s,0,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_sg_off_request_from_valid_str() {
        let request = <OutputDisableSpecificGravity as I2CCommand>::from_str("o,sg,0").unwrap();
        assert_eq!("O,SG,0", I2CCommand::to_string(&request));
        let request = <OutputDisableSpecificGravity as I2CCommand>::from_str("O,SG,0").unwrap();
        assert_eq!("O,SG,0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_sg_off_request_from_invalid_str_yields_err() {
        let request = <OutputDisableSpecificGravity as I2CCommand>::from_str("o,sg,");
        assert!(request.is_err());

        let request = <OutputDisableSpecificGravity as I2CCommand>::from_str("o,sg,0,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_tds_off_request_from_valid_str() {
        let request = <OutputDisableTds as I2CCommand>::from_str("o,tds,0").unwrap();
        assert_eq!("O,TDS,0", I2CCommand::to_string(&request));
        let request = <OutputDisableTds as I2CCommand>::from_str("O,TDS,0").unwrap();
        assert_eq!("O,TDS,0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_tds_off_request_from_invalid_str_yields_err() {
        let request = <OutputDisableTds as I2CCommand>::from_str("o,tds,");
        assert!(request.is_err());

        let request = <OutputDisableTds as I2CCommand>::from_str("o,tds,0,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_conductivity_on_request_from_valid_str() {
        let request = <OutputEnableConductivity as I2CCommand>::from_str("o,ec,1").unwrap();
        assert_eq!("O,EC,1", I2CCommand::to_string(&request));
        let request = <OutputEnableConductivity as I2CCommand>::from_str("O,EC,1").unwrap();
        assert_eq!("O,EC,1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_conductivity_on_request_from_invalid_str_yields_err() {
        let request = <OutputEnableConductivity as I2CCommand>::from_str("o,ec,");
        assert!(request.is_err());

        let request = <OutputEnableConductivity as I2CCommand>::from_str("o,ec,1,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_salinity_on_request_from_valid_str() {
        let request = <OutputEnableSalinity as I2CCommand>::from_str("o,s,1").unwrap();
        assert_eq!("O,S,1", I2CCommand::to_string(&request));
        let request = <OutputEnableSalinity as I2CCommand>::from_str("O,S,1").unwrap();
        assert_eq!("O,S,1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_salinity_on_request_from_invalid_str_yields_err() {
        let request = <OutputEnableSalinity as I2CCommand>::from_str("o,s,");
        assert!(request.is_err());

        let request = <OutputEnableSalinity as I2CCommand>::from_str("o,s,1,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_sg_on_request_from_valid_str() {
        let request = <OutputEnableSpecificGravity as I2CCommand>::from_str("o,sg,1").unwrap();
        assert_eq!("O,SG,1", I2CCommand::to_string(&request));
        let request = <OutputEnableSpecificGravity as I2CCommand>::from_str("O,SG,1").unwrap();
        assert_eq!("O,SG,1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_sg_on_request_from_invalid_str_yields_err() {
        let request = <OutputEnableSpecificGravity as I2CCommand>::from_str("o,sg,");
        assert!(request.is_err());

        let request = <OutputEnableSpecificGravity as I2CCommand>::from_str("o,sg,1,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_tds_on_request_from_valid_str() {
        let request = <OutputEnableTds as I2CCommand>::from_str("o,tds,1").unwrap();
        assert_eq!("O,TDS,1", I2CCommand::to_string(&request));
        let request = <OutputEnableTds as I2CCommand>::from_str("O,TDS,1").unwrap();
        assert_eq!("O,TDS,1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_tds_on_request_from_invalid_str_yields_err() {
        let request = <OutputEnableTds as I2CCommand>::from_str("o,tds,");
        assert!(request.is_err());

        let request = <OutputEnableTds as I2CCommand>::from_str("o,tds,1,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_output_status_request_from_valid_str() {
        let request = <OutputState as I2CCommand>::from_str("o,?").unwrap();
        assert_eq!("O,?", I2CCommand::to_string(&request));
        let request = <OutputState as I2CCommand>::from_str("O,?").unwrap();
        assert_eq!("O,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_output_status_request_from_invalid_str_yields_err() {
        let request = <OutputState as I2CCommand>::from_str("o,");
        assert!(request.is_err());

        let request = <OutputState as I2CCommand>::from_str("o,?,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_one_request_from_valid_str() {
        let request = <ProbeTypeOne as I2CCommand>::from_str("k,1.0").unwrap();
        assert_eq!("K,1.0", I2CCommand::to_string(&request));
        let request = <ProbeTypeOne as I2CCommand>::from_str("K,1.0").unwrap();
        assert_eq!("K,1.0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_one_request_from_invalid_str_yields_err() {
        let request = <ProbeTypeOne as I2CCommand>::from_str("k,1.0 ");
        assert!(request.is_err());

        let request = <ProbeTypeOne as I2CCommand>::from_str("k,1.000000");
        assert!(request.is_err());

        let request = <ProbeTypeOne as I2CCommand>::from_str("k,1.0 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_point_one_request_from_valid_str() {
        let request = <ProbeTypePointOne as I2CCommand>::from_str("k,0.1").unwrap();
        assert_eq!("K,0.1", I2CCommand::to_string(&request));
        let request = <ProbeTypePointOne as I2CCommand>::from_str("K,0.1").unwrap();
        assert_eq!("K,0.1", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_point_one_request_from_invalid_str_yields_err() {
        let request = <ProbeTypePointOne as I2CCommand>::from_str("k,0.1,");
        assert!(request.is_err());

        let request = <ProbeTypePointOne as I2CCommand>::from_str("k,0.11");
        assert!(request.is_err());

        let request = <ProbeTypePointOne as I2CCommand>::from_str("k,.1");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_ten_request_from_valid_str() {
        let request = <ProbeTypeTen as I2CCommand>::from_str("k,10.0").unwrap();
        assert_eq!("K,10.0", I2CCommand::to_string(&request));
        let request = <ProbeTypeTen as I2CCommand>::from_str("K,10.0").unwrap();
        assert_eq!("K,10.0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_ten_request_from_invalid_str_yields_err() {
        let request = <ProbeTypeTen as I2CCommand>::from_str("k,10 ");
        assert!(request.is_err());

        let request = <ProbeTypeTen as I2CCommand>::from_str("k,1000000");
        assert!(request.is_err());

        let request = <ProbeTypeTen as I2CCommand>::from_str("k,10 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_probe_type_status_request_from_valid_str() {
        let request = <ProbeTypeState as I2CCommand>::from_str("k,?").unwrap();
        assert_eq!("K,?", I2CCommand::to_string(&request));
        let request = <ProbeTypeState as I2CCommand>::from_str("K,?").unwrap();
        assert_eq!("K,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_probe_type_status_request_from_invalid_str_yields_err() {
        let request = <ProbeTypeState as I2CCommand>::from_str("k,?,");
        assert!(request.is_err());
        let request = <ProbeTypeState as I2CCommand>::from_str("k,?10");
        assert!(request.is_err());
        let request = <ProbeTypeState as I2CCommand>::from_str("k,10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_read_request_from_valid_str() {
        let request = <Reading as I2CCommand>::from_str("r").unwrap();
        assert_eq!("R", I2CCommand::to_string(&request));
        let request = <Reading as I2CCommand>::from_str("R").unwrap();
        assert_eq!("R", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_conductivity_read_request_from_invalid_str_yields_err() {
        let request = <Reading as I2CCommand>::from_str("r,");
        assert!(request.is_err());
        let request = <Reading as I2CCommand>::from_str("read");
        assert!(request.is_err());
    }
}
