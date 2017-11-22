//! Commands from EZO RTD chipset.
use errors::*;

pub use ezo_rtd::command::Baud;
pub use ezo_rtd::command::Command;
pub use ezo_rtd::command::{CalibrationClear, CalibrationState, CalibrationTemperature};
pub use ezo_rtd::command::{DataloggerDisable, DataloggerInterval, DataloggerPeriod};
pub use ezo_rtd::command::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep, Status};
pub use ezo_rtd::command::{Export, ExportInfo, Import};
pub use ezo_rtd::command::{LedOff, LedOn, LedState};
pub use ezo_rtd::command::{MemoryClear, MemoryRecall, MemoryRecallLast};
pub use ezo_rtd::command::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
pub use ezo_rtd::command::{ScaleCelsius, ScaleFahrenheit, ScaleKelvin, ScaleState};

pub use devices::{I2CCommand, I2CResponse, SensorDevice};

use super::response::*;

macro_rules! impl_I2CCommand_for {
    ( $name:ident , $response:ty ) => {
        impl I2CCommand for $name {
            type Response = $response;

            fn from_str(s: &str) -> Result<$name> {
                let cmd = s.parse::<$name>()
                    .chain_err(|| ErrorKind::CommandParse)?;
                Ok(cmd)
            }

            fn to_string(&self) -> String {
                <$name as Command>::get_command_string(&self)
            }

            fn write<T: SensorDevice>(&self, device: &T) -> Result<$response> {
                unimplemented!();
            }
        }
    }
}

impl_I2CCommand_for!(CalibrationState, CalibrationStatus);
impl_I2CCommand_for!(CalibrationTemperature, ReplyStatus);
impl_I2CCommand_for!(DataloggerDisable, ReplyStatus);
impl_I2CCommand_for!(DataloggerInterval, DataLoggerStorageIntervalSeconds);
impl_I2CCommand_for!(DataloggerPeriod, ReplyStatus);
impl_I2CCommand_for!(Reading, SensorReading);
impl_I2CCommand_for!(MemoryClear, ReplyStatus);
impl_I2CCommand_for!(MemoryRecall, MemoryReading);
impl_I2CCommand_for!(MemoryRecallLast, MemoryReading);
impl_I2CCommand_for!(ScaleCelsius, ReplyStatus);
impl_I2CCommand_for!(ScaleFahrenheit, ReplyStatus);
impl_I2CCommand_for!(ScaleKelvin, ReplyStatus);
impl_I2CCommand_for!(ScaleState, TemperatureScale);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_temperature_calibration_set_request_from_valid_str() {
        let request = <CalibrationTemperature as I2CCommand>::from_str("cal,100.34").unwrap();
        assert_eq!("CAL,100.34", I2CCommand::to_string(&request));
        let request = <CalibrationTemperature as I2CCommand>::from_str("CAL,1000.3324").unwrap();
        assert_eq!("CAL,1000.33", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_calibration_set_request_from_invalid_str_yields_err() {
        let request = <CalibrationTemperature as I2CCommand>::from_str("cal,");
        assert!(request.is_err());

        let request = <CalibrationTemperature as I2CCommand>::from_str("cal,sets");
        assert!(request.is_err());

        let request = <CalibrationTemperature as I2CCommand>::from_str("cal,123a2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_calibration_status_request_from_valid_str() {
        let request = <CalibrationState as I2CCommand>::from_str("cal,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
        let request = <CalibrationState as I2CCommand>::from_str("Cal,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
        let request = <CalibrationState as I2CCommand>::from_str("CAL,?").unwrap();
        assert_eq!("CAL,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_calibration_status_request_from_invalid_str_yields_err() {
        let request = <CalibrationState as I2CCommand>::from_str("cal,?s");
        assert!(request.is_err());

        let request = <CalibrationState as I2CCommand>::from_str("cal, ?");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_off_request_from_valid_str() {
        let request = <DataloggerDisable as I2CCommand>::from_str("d,0").unwrap();
        assert_eq!("D,0", I2CCommand::to_string(&request));
        let request = <DataloggerDisable as I2CCommand>::from_str("D,0").unwrap();
        assert_eq!("D,0", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_datalogger_off_request_from_invalid_str_yields_err() {
        let request = <DataloggerDisable as I2CCommand>::from_str("d,0 ");
        assert!(request.is_err());

        let request = <DataloggerDisable as I2CCommand>::from_str("d,0.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_set_request_from_valid_str() {
        let request = <DataloggerPeriod as I2CCommand>::from_str("d,10").unwrap();
        assert_eq!("D,10", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_datalogger_set_request_from_invalid_str_yields_err() {
        let request = <DataloggerInterval as I2CCommand>::from_str("d,");
        assert!(request.is_err());

        let request = <DataloggerInterval as I2CCommand>::from_str("d,11,");
        assert!(request.is_err());

        let request = <DataloggerInterval as I2CCommand>::from_str("d,9");
        assert!(request.is_err());

        let request = <DataloggerInterval as I2CCommand>::from_str("d,1_000_000_000");
        assert!(request.is_err());

        let request = <DataloggerInterval as I2CCommand>::from_str("d,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_status_request_from_valid_str() {
        let request = <DataloggerInterval as I2CCommand>::from_str("d,?").unwrap();
        assert_eq!("D,?", I2CCommand::to_string(&request));
        let request = <DataloggerInterval as I2CCommand>::from_str("D,?").unwrap();
        assert_eq!("D,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_datalogger_status_request_from_invalid_str_yields_err() {
        let request = <DataloggerInterval as I2CCommand>::from_str("d,?,");
        assert!(request.is_err());

        let request = <DataloggerInterval as I2CCommand>::from_str("d,? 1_000_000_000");
        assert!(request.is_err());

        let request = <DataloggerInterval as I2CCommand>::from_str("d,?,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_clear_request_from_valid_str() {
        let request = <MemoryClear as I2CCommand>::from_str("m,clear").unwrap();
        assert_eq!("M,CLEAR", I2CCommand::to_string(&request));
        let request = <MemoryClear as I2CCommand>::from_str("M,CLEAR").unwrap();
        assert_eq!("M,CLEAR", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_memory_clear_request_from_invalid_str_yields_err() {
        let request = <MemoryClear as I2CCommand>::from_str("M,CLEARo");
        assert!(request.is_err());

        let request = <MemoryClear as I2CCommand>::from_str("M,CLEAR 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_recall_request_from_valid_str() {
        let request = <MemoryRecall as I2CCommand>::from_str("m").unwrap();
        assert_eq!("M", I2CCommand::to_string(&request));
        let request = <MemoryRecall as I2CCommand>::from_str("M").unwrap();
        assert_eq!("M", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_memory_recall_request_from_invalid_str_yields_err() {
        let request = <MemoryRecall as I2CCommand>::from_str("m,");
        assert!(request.is_err());

        let request = <MemoryRecall as I2CCommand>::from_str("M 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_recall_last_request_from_valid_str() {
        let request = <MemoryRecallLast as I2CCommand>::from_str("m,?").unwrap();
        assert_eq!("M,?", I2CCommand::to_string(&request));
        let request = <MemoryRecallLast as I2CCommand>::from_str("M,?").unwrap();
        assert_eq!("M,?", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_memory_recall_last_request_from_invalid_str_yields_err() {
        let request = <MemoryRecallLast as I2CCommand>::from_str("m,?,");
        assert!(request.is_err());

        let request = <MemoryRecallLast as I2CCommand>::from_str("m,? ");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_read_request_from_valid_str() {
        let request = <Reading as I2CCommand>::from_str("r").unwrap();
        assert_eq!("R", I2CCommand::to_string(&request));
        let request = <Reading as I2CCommand>::from_str("R").unwrap();
        assert_eq!("R", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_read_request_from_invalid_str_yields_err() {
        let request = <Reading as I2CCommand>::from_str("r,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_celsius_request_from_valid_str() {
        let request = <ScaleCelsius as I2CCommand>::from_str("s,c").unwrap();
        assert_eq!("S,C", I2CCommand::to_string(&request));
        let request = <ScaleCelsius as I2CCommand>::from_str("S,C").unwrap();
        assert_eq!("S,C", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_scale_celsius_request_from_invalid_str_yields_err() {
        let request = <ScaleCelsius as I2CCommand>::from_str("s,c,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_fahrenheit_request_from_valid_str() {
        let request = <ScaleFahrenheit as I2CCommand>::from_str("s,f").unwrap();
        assert_eq!("S,F", I2CCommand::to_string(&request));
        let request = <ScaleFahrenheit as I2CCommand>::from_str("S,F").unwrap();
        assert_eq!("S,F", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_scale_fahrenheit_request_from_invalid_str_yields_err() {
        let request = <ScaleFahrenheit as I2CCommand>::from_str("s,f,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_kelvin_request_from_valid_str() {
        let request = <ScaleKelvin as I2CCommand>::from_str("s,k").unwrap();
        assert_eq!("S,K", I2CCommand::to_string(&request));
        let request = <ScaleKelvin as I2CCommand>::from_str("S,K").unwrap();
        assert_eq!("S,K", I2CCommand::to_string(&request));
    }

    #[test]
    fn parse_temperature_scale_kelvin_request_from_invalid_str_yields_err() {
        let request = <ScaleKelvin as I2CCommand>::from_str("s,k,");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_status_request_from_valid_str() {
        let request = <ScaleState as I2CCommand>::from_str("s,?").unwrap();
        assert_eq!(
            "S,?",
            <ScaleState as I2CCommand>::to_string(&request)
        );
        let request = <ScaleState as I2CCommand>::from_str("S,?").unwrap();
        assert_eq!(
            "S,?",
            <ScaleState as I2CCommand>::to_string(&request)
        );
    }

    #[test]
    fn parse_temperature_scale_status_request_from_invalid_str_yields_err() {
        let request = <ScaleState as I2CCommand>::from_str("s,?,");
        assert!(request.is_err());
    }
}
