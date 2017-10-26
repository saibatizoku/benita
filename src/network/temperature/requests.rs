//! Requests for the temperature sensor. Requests are sent to a temperature `Endpoint`.
use errors::*;

use network::{Endpoint, SocketReply, SocketRequest};
use network::common::OkReply;

pub use devices::temperature::commands::Baud;
pub use devices::temperature::commands::Command;
pub use devices::temperature::commands::{CalibrationClear, CalibrationState,
                                         CalibrationTemperature};
pub use devices::temperature::commands::{DataloggerDisable, DataloggerInterval, DataloggerPeriod};
pub use devices::temperature::commands::{DeviceAddress, DeviceInformation, Factory, Find, Reading,
                                         Sleep, Status};
pub use devices::temperature::commands::{Export, ExportInfo, Import};
pub use devices::temperature::commands::{LedOff, LedOn, LedState};
pub use devices::temperature::commands::{MemoryClear, MemoryRecall, MemoryRecallLast};
pub use devices::temperature::commands::{ProtocolLockDisable, ProtocolLockEnable,
                                         ProtocolLockState};
pub use devices::temperature::commands::{ScaleCelsius, ScaleFahrenheit, ScaleKelvin, ScaleState};

use devices::temperature::responses::{CalibrationStatus, DataLoggerStorageIntervalSeconds,
                                      DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus,
                                      MemoryReading, ProtocolLockStatus, SensorReading,
                                      TemperatureScale};
use utilities::atof;

use ezo_common::BpsRate;

impl_SocketRequest_for! {
    Baud: OkReply,
    req_str: {
        if req_str.starts_with("baud ") {
            let resp = req_str.get(5..).unwrap();
            let bps_num = resp.parse::<u32>()
                    .chain_err(|| ErrorKind::NumberParse)?;
            let bps = BpsRate::parse_u32(bps_num)
                    .chain_err(|| ErrorKind::RequestParse)?;
            Ok(Baud(bps))
        } else {
            Err(ErrorKind::RequestParse.into())
        }
    },
    req_out: {
        format!("baud {}", &req_out.0.parse())
    }
}

impl_SocketRequest_for! {
    CalibrationClear: OkReply,
    req_str: {
        match req_str {
            "calibration-clear" => Ok(CalibrationClear),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "calibration-clear".to_string()
    }
}

impl_SocketRequest_for! {
    CalibrationTemperature: OkReply,
    req_str: {
        if req_str.starts_with("calibration-set ") {
            let resp = req_str.get(16..).unwrap();
            let value = atof(resp)?;
            return Ok(CalibrationTemperature(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("calibration-set {:.*}", 3, req_out.0)
    }
}

impl_SocketRequest_for! {
    CalibrationState: CalibrationStatus,
    req_str: {
        match req_str {
            "calibration-status" => Ok(CalibrationState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "calibration-status".to_string()
    }
}

impl_SocketRequest_for! {
    DataloggerDisable: OkReply,
    req_str: {
        match req_str {
            "datalogger-off" => Ok(DataloggerDisable),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "datalogger-off".to_string()
    }
}

impl_SocketRequest_for! {
    DataloggerInterval: DataLoggerStorageIntervalSeconds,
    req_str: {
        match req_str {
            "datalogger-status" => Ok(DataloggerInterval),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "datalogger-status".to_string()
    }
}

impl_SocketRequest_for! {
    DataloggerPeriod: OkReply,
    req_str: {
        if req_str.starts_with("datalogger-set ") {
            let resp = req_str.get(15..).unwrap();
            let value =  resp.parse::<u32>()
                    .chain_err(|| ErrorKind::NumberParse)?;
            return Ok(DataloggerPeriod(value));
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("datalogger-set {}", req_out.0)
    }
}

impl_SocketRequest_for! {
    DeviceAddress: OkReply,
    req_str: {
        if req_str.starts_with("device-address ") {
            let resp = req_str.get(15..).unwrap();
            let addr = resp.parse::<u16>()
                    .chain_err(|| ErrorKind::NumberParse)?;
            Ok(DeviceAddress(addr))
        } else {
            Err(ErrorKind::RequestParse.into())
        }
    },
    req_out: {
        format!("device-address {}", &req_out.0)
    }
}

impl_SocketRequest_for! {
    DeviceInformation: DeviceInfo,
    req_str: {
        match req_str {
            "device-info" => Ok(DeviceInformation),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "device-info".to_string()
    }
}

impl_SocketRequest_for! {
    Export: Exported,
    req_str: {
        match req_str {
            "export" => Ok(Export),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "export".to_string()
    }
}

impl_SocketRequest_for! {
    ExportInfo: ExportedInfo,
    req_str: {
        match req_str {
            "export-info" => Ok(ExportInfo),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "export-info".to_string()
    }
}

impl_SocketRequest_for! {
    Import: OkReply,
    req_str: {
        if req_str.starts_with("import ") {
            let resp = req_str.get(7..).unwrap();
            match resp.len() {
                1...12 => return Ok(Import(resp.to_string())),
                _ => return Err(ErrorKind::RequestParse.into()),
            }
        }
        Err(ErrorKind::RequestParse.into())
    },
    req_out: {
        format!("import {}", req_out.0)
    }
}

impl_SocketRequest_for! {
    Factory: OkReply,
    req_str: {
        match req_str {
            "factory" => Ok(Factory),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "factory".to_string()
    }
}

impl_SocketRequest_for! {
    Find: OkReply,
    req_str: {
        match req_str {
            "find" => Ok(Find),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "find".to_string()
    }
}

impl_SocketRequest_for! {
    LedOff: OkReply,
    req_str: {
        match req_str {
            "led-off" => Ok(LedOff),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "led-off".to_string()
    }
}

impl_SocketRequest_for! {
    LedOn: OkReply,
    req_str: {
        match req_str {
            "led-on" => Ok(LedOn),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "led-on".to_string()
    }
}

impl_SocketRequest_for! {
    LedState: LedStatus,
    req_str: {
        match req_str {
            "led-status" => Ok(LedState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "led-status".to_string()
    }
}

impl_SocketRequest_for! {
    MemoryClear: OkReply,
    req_str: {
        match req_str {
            "memory-clear" => Ok(MemoryClear),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "memory-clear".to_string()
    }
}

impl_SocketRequest_for! {
    MemoryRecall: MemoryReading,
    req_str: {
        match req_str {
            "memory-recall" => Ok(MemoryRecall),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "memory-recall".to_string()
    }
}

impl_SocketRequest_for! {
    MemoryRecallLast: MemoryReading,
    req_str: {
        match req_str {
            "memory-recall-last" => Ok(MemoryRecallLast),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "memory-recall-last".to_string()
    }
}

impl_SocketRequest_for! {
    ProtocolLockDisable: OkReply,
    req_str: {
        match req_str {
            "protocol-lock-off" => Ok(ProtocolLockDisable),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "protocol-lock-off".to_string()
    }
}

impl_SocketRequest_for! {
    ProtocolLockEnable: OkReply,
    req_str: {
        match req_str {
            "protocol-lock-on" => Ok(ProtocolLockEnable),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "protocol-lock-on".to_string()
    }
}

impl_SocketRequest_for! {
    ProtocolLockState: ProtocolLockStatus,
    req_str: {
        match req_str {
            "protocol-lock-status" => Ok(ProtocolLockState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "protocol-lock-status".to_string()
    }
}

impl_SocketRequest_for! {
    Reading: SensorReading,
    req_str: {
        match req_str {
            "read" => Ok(Reading),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "read".to_string()
    }
}

impl_SocketRequest_for! {
    ScaleCelsius: OkReply,
    req_str: {
        match req_str {
            "scale-celsius" => Ok(ScaleCelsius),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "scale-celsius".to_string()
    }
}

impl_SocketRequest_for! {
    ScaleFahrenheit: OkReply,
    req_str: {
        match req_str {
            "scale-fahrenheit" => Ok(ScaleFahrenheit),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "scale-fahrenheit".to_string()
    }
}

impl_SocketRequest_for! {
    ScaleKelvin: OkReply,
    req_str: {
        match req_str {
            "scale-kelvin" => Ok(ScaleKelvin),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "scale-kelvin".to_string()
    }
}

impl_SocketRequest_for! {
    ScaleState: TemperatureScale,
    req_str: {
        match req_str {
            "scale-status" => Ok(ScaleState),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "scale-status".to_string()
    }
}

impl_SocketRequest_for! {
    Sleep: OkReply,
    req_str: {
        match req_str {
            "sleep" => Ok(Sleep),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "sleep".to_string()
    }
}

impl_SocketRequest_for! {
    Status: DeviceStatus,
    req_str: {
        match req_str {
            "status" => Ok(Status),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    },
    _req_out: {
        "status".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid_baud_request(test_str: &str, bps: BpsRate) {
        let request = Baud::from_request_str(test_str).unwrap();
        assert_eq!(test_str, &request.to_request_string());
        assert_eq!(bps, request.0);
    }
    #[test]
    fn parse_temperature_baud_request_from_valid_str() {
        assert_valid_baud_request("baud 300", BpsRate::Bps300);
        assert_valid_baud_request("baud 1200", BpsRate::Bps1200);
        assert_valid_baud_request("baud 2400", BpsRate::Bps2400);
        assert_valid_baud_request("baud 9600", BpsRate::Bps9600);
        assert_valid_baud_request("baud 19200", BpsRate::Bps19200);
        assert_valid_baud_request("baud 38400", BpsRate::Bps38400);
        assert_valid_baud_request("baud 57600", BpsRate::Bps57600);
        assert_valid_baud_request("baud 115200", BpsRate::Bps115200);
    }

    #[test]
    fn parse_temperature_baud_request_from_invalid_str_yields_err() {
        let request = Baud::from_request_str("baud");
        assert!(request.is_err());

        let request = Baud::from_request_str("bauds 300");
        assert!(request.is_err());

        let request = Baud::from_request_str("baud 0");
        assert!(request.is_err());

        let request = Baud::from_request_str("baud 10.5829");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_calibration_clear_request_from_valid_str() {
        let request = CalibrationClear::from_request_str("calibration-clear").unwrap();
        assert_eq!("calibration-clear", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_calibration_clear_request_from_invalid_str_yields_err() {
        let request = CalibrationClear::from_request_str("calibration-clearEXTRA");
        assert!(request.is_err());

        let request = CalibrationClear::from_request_str("calibration-clear 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_calibration_set_request_from_valid_str() {
        let request =
            CalibrationTemperature::from_request_str("calibration-set 1000.3324").unwrap();
        assert_eq!("calibration-set 1000.332", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_calibration_set_request_from_invalid_str_yields_err() {
        let request = CalibrationTemperature::from_request_str("calibration-set");
        assert!(request.is_err());

        let request = CalibrationTemperature::from_request_str("calibration-sets");
        assert!(request.is_err());

        let request = CalibrationTemperature::from_request_str("calibration-set 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_calibration_status_request_from_valid_str() {
        let request = CalibrationState::from_request_str("calibration-status").unwrap();
        assert_eq!("calibration-status", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_calibration_status_request_from_invalid_str_yields_err() {
        let request = CalibrationState::from_request_str("calibration-statuss");
        assert!(request.is_err());

        let request = CalibrationState::from_request_str("calibration-status 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_off_request_from_valid_str() {
        let request = DataloggerDisable::from_request_str("datalogger-off").unwrap();
        assert_eq!("datalogger-off", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_datalogger_off_request_from_invalid_str_yields_err() {
        let request = DataloggerDisable::from_request_str("datalogger-off ");
        assert!(request.is_err());

        let request = DataloggerDisable::from_request_str("datalogger-off,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_set_request_from_valid_str() {
        let request = DataloggerPeriod::from_request_str("datalogger-set 10").unwrap();
        assert_eq!("datalogger-set 10", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_datalogger_set_request_from_invalid_str_yields_err() {
        let request = DataloggerInterval::from_request_str("datalogger-set ");
        assert!(request.is_err());

        let request = DataloggerInterval::from_request_str("datalogger-set 9");
        assert!(request.is_err());

        let request = DataloggerInterval::from_request_str("datalogger-set 1_000_000_000");
        assert!(request.is_err());

        let request = DataloggerInterval::from_request_str("datalogger-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_status_request_from_valid_str() {
        let request = DataloggerInterval::from_request_str("datalogger-status").unwrap();
        assert_eq!("datalogger-status", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_datalogger_status_request_from_invalid_str_yields_err() {
        let request = DataloggerInterval::from_request_str("datalogger-status ");
        assert!(request.is_err());

        let request = DataloggerInterval::from_request_str("datalogger-status 1_000_000_000");
        assert!(request.is_err());

        let request = DataloggerInterval::from_request_str("datalogger-status,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_device_address_request_from_valid_str() {
        let request = DeviceAddress::from_request_str("device-address 90").unwrap();
        assert_eq!("device-address 90", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_device_address_request_from_invalid_str_yields_err() {
        let request = DeviceAddress::from_request_str("device-address");
        assert!(request.is_err());

        let request = DeviceAddress::from_request_str("device-address10.5");
        assert!(request.is_err());

        let request = DeviceAddress::from_request_str("device-address 10.5");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_device_info_request_from_valid_str() {
        let request = DeviceInformation::from_request_str("device-info").unwrap();
        assert_eq!("device-info", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_device_info_request_from_invalid_str_yields_err() {
        let request = DeviceInformation::from_request_str("device-infoo");
        assert!(request.is_err());

        let request = DeviceInformation::from_request_str("device-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_export_request_from_valid_str() {
        let request = Export::from_request_str("export").unwrap();
        assert_eq!("export", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_export_request_from_invalid_str_yields_err() {
        let request = Export::from_request_str("exporto");
        assert!(request.is_err());

        let request = Export::from_request_str("export 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_export_info_request_from_valid_str() {
        let request = ExportInfo::from_request_str("export-info").unwrap();
        assert_eq!("export-info", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_export_info_request_from_invalid_str_yields_err() {
        let request = ExportInfo::from_request_str("export-infoo");
        assert!(request.is_err());

        let request = ExportInfo::from_request_str("export-info 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_import_request_from_valid_str() {
        let request = Import::from_request_str("import 123456789012").unwrap();
        assert_eq!("import 123456789012", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_import_request_from_invalid_str_yields_err() {
        let request = Import::from_request_str("import");
        assert!(request.is_err());

        let request = Import::from_request_str("import ");
        assert!(request.is_err());

        let request = Import::from_request_str("import 1234567890123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_factory_request_from_valid_str() {
        let request = Factory::from_request_str("factory").unwrap();
        assert_eq!("factory", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_factory_request_from_invalid_str_yields_err() {
        let request = Factory::from_request_str("factoryo");
        assert!(request.is_err());

        let request = Factory::from_request_str("factory 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_find_request_from_valid_str() {
        let request = Find::from_request_str("find").unwrap();
        assert_eq!("find", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_find_request_from_invalid_str_yields_err() {
        let request = Find::from_request_str("findo");
        assert!(request.is_err());

        let request = Find::from_request_str("find 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_led_off_request_from_valid_str() {
        let request = LedOff::from_request_str("led-off").unwrap();
        assert_eq!("led-off", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_led_off_request_from_invalid_str_yields_err() {
        let request = LedOff::from_request_str("led-offo");
        assert!(request.is_err());

        let request = LedOff::from_request_str("led-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_led_on_request_from_valid_str() {
        let request = LedOn::from_request_str("led-on").unwrap();
        assert_eq!("led-on", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_led_on_request_from_invalid_str_yields_err() {
        let request = LedOn::from_request_str("led-ono");
        assert!(request.is_err());

        let request = LedOn::from_request_str("led-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_led_status_request_from_valid_str() {
        let request = LedState::from_request_str("led-status").unwrap();
        assert_eq!("led-status", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_led_status_request_from_invalid_str_yields_err() {
        let request = LedState::from_request_str("led-statuso");
        assert!(request.is_err());

        let request = LedState::from_request_str("led-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_clear_request_from_valid_str() {
        let request = MemoryClear::from_request_str("memory-clear").unwrap();
        assert_eq!("memory-clear", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_memory_clear_request_from_invalid_str_yields_err() {
        let request = MemoryClear::from_request_str("memory-clearo");
        assert!(request.is_err());

        let request = MemoryClear::from_request_str("memory-clear 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_recall_request_from_valid_str() {
        let request = MemoryRecall::from_request_str("memory-recall").unwrap();
        assert_eq!("memory-recall", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_memory_recall_request_from_invalid_str_yields_err() {
        let request = MemoryRecall::from_request_str("memory-recallo");
        assert!(request.is_err());

        let request = MemoryRecall::from_request_str("memory-recall 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_recall_last_request_from_valid_str() {
        let request = MemoryRecallLast::from_request_str("memory-recall-last").unwrap();
        assert_eq!("memory-recall-last", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_memory_recall_last_request_from_invalid_str_yields_err() {
        let request = MemoryRecallLast::from_request_str("memory-recall-lasto");
        assert!(request.is_err());

        let request = MemoryRecallLast::from_request_str("memory-recall-last 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_protocol_lock_off_request_from_valid_str() {
        let request = ProtocolLockDisable::from_request_str("protocol-lock-off").unwrap();
        assert_eq!("protocol-lock-off", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_protocol_lock_off_request_from_invalid_str_yields_err() {
        let request = ProtocolLockDisable::from_request_str("protocol-lock-offo");
        assert!(request.is_err());

        let request = ProtocolLockDisable::from_request_str("protocol-lock-off 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_protocol_lock_on_request_from_valid_str() {
        let request = ProtocolLockEnable::from_request_str("protocol-lock-on").unwrap();
        assert_eq!("protocol-lock-on", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_protocol_lock_on_request_from_invalid_str_yields_err() {
        let request = ProtocolLockEnable::from_request_str("protocol-lock-ono");
        assert!(request.is_err());

        let request = ProtocolLockEnable::from_request_str("protocol-lock-on 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_protocol_lock_status_request_from_valid_str() {
        let request = ProtocolLockState::from_request_str("protocol-lock-status").unwrap();
        assert_eq!("protocol-lock-status", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_protocol_lock_status_request_from_invalid_str_yields_err() {
        let request = ProtocolLockState::from_request_str("protocol-lock-statuso");
        assert!(request.is_err());

        let request = ProtocolLockState::from_request_str("protocol-lock-status 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_read_request_from_valid_str() {
        let request = Reading::from_request_str("read").unwrap();
        assert_eq!("read", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_read_request_from_invalid_str_yields_err() {
        let request = Reading::from_request_str("reading");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_celsius_request_from_valid_str() {
        let request = ScaleCelsius::from_request_str("scale-celsius").unwrap();
        assert_eq!("scale-celsius", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_scale_celsius_request_from_invalid_str_yields_err() {
        let request = ScaleCelsius::from_request_str("scale-celsiusing");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_fahrenheit_request_from_valid_str() {
        let request = ScaleFahrenheit::from_request_str("scale-fahrenheit").unwrap();
        assert_eq!("scale-fahrenheit", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_scale_fahrenheit_request_from_invalid_str_yields_err() {
        let request = ScaleFahrenheit::from_request_str("scale-fahrenheiting");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_kelvin_request_from_valid_str() {
        let request = ScaleKelvin::from_request_str("scale-kelvin").unwrap();
        assert_eq!("scale-kelvin", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_scale_kelvin_request_from_invalid_str_yields_err() {
        let request = ScaleKelvin::from_request_str("scale-kelvining");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_status_request_from_valid_str() {
        let request = ScaleState::from_request_str("scale-status").unwrap();
        assert_eq!("scale-status", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_scale_status_request_from_invalid_str_yields_err() {
        let request = ScaleState::from_request_str("scale-statusing");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_sleep_request_from_valid_str() {
        let request = Sleep::from_request_str("sleep").unwrap();
        assert_eq!("sleep", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_sleep_request_from_invalid_str_yields_err() {
        let request = Sleep::from_request_str("sleeping");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_status_request_from_valid_str() {
        let request = Status::from_request_str("status").unwrap();
        assert_eq!("status", &request.to_request_string());
    }

    #[test]
    fn parse_temperature_status_request_from_invalid_str_yields_err() {
        let request = Status::from_request_str("statusing");
        assert!(request.is_err());
    }
}
