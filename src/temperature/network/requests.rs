//! Requests for the temperature sensor. Requests are sent to a temperature `Endpoint`.
pub mod errors {
    error_chain!{}
}

use errors::*;
use network::{Endpoint, ReplyStatus, SocketReply, SocketRequest};

pub use common_ezo::command::*;
pub use temperature::device::commands::*;

use temperature::device::responses::*;
use utilities::atof;


impl_SocketRequest_for! {
    CalibrationTemperature: ReplyStatus,
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
    DataloggerDisable: ReplyStatus,
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
    DataloggerPeriod: ReplyStatus,
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
    MemoryClear: ReplyStatus,
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
    ScaleCelsius: ReplyStatus,
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
    ScaleFahrenheit: ReplyStatus,
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
    ScaleKelvin: ReplyStatus,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_temperature_calibration_set_request_from_valid_str() {
        let request =
            CalibrationTemperature::from_str("calibration-set 1000.3324").unwrap();
        assert_eq!("calibration-set 1000.332", &request.to_string());
    }

    #[test]
    fn parse_temperature_calibration_set_request_from_invalid_str_yields_err() {
        let request = CalibrationTemperature::from_str("calibration-set");
        assert!(request.is_err());

        let request = CalibrationTemperature::from_str("calibration-sets");
        assert!(request.is_err());

        let request = CalibrationTemperature::from_str("calibration-set 123 2342");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_calibration_status_request_from_valid_str() {
        let request = CalibrationState::from_str("calibration-status").unwrap();
        assert_eq!("calibration-status", &request.to_string());
    }

    #[test]
    fn parse_temperature_calibration_status_request_from_invalid_str_yields_err() {
        let request = CalibrationState::from_str("calibration-statuss");
        assert!(request.is_err());

        let request = CalibrationState::from_str("calibration-status 123");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_off_request_from_valid_str() {
        let request = DataloggerDisable::from_str("datalogger-off").unwrap();
        assert_eq!("datalogger-off", &request.to_string());
    }

    #[test]
    fn parse_temperature_datalogger_off_request_from_invalid_str_yields_err() {
        let request = DataloggerDisable::from_str("datalogger-off ");
        assert!(request.is_err());

        let request = DataloggerDisable::from_str("datalogger-off,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_set_request_from_valid_str() {
        let request = DataloggerPeriod::from_str("datalogger-set 10").unwrap();
        assert_eq!("datalogger-set 10", &request.to_string());
    }

    #[test]
    fn parse_temperature_datalogger_set_request_from_invalid_str_yields_err() {
        let request = DataloggerInterval::from_str("datalogger-set ");
        assert!(request.is_err());

        let request = DataloggerInterval::from_str("datalogger-set 9");
        assert!(request.is_err());

        let request = DataloggerInterval::from_str("datalogger-set 1_000_000_000");
        assert!(request.is_err());

        let request = DataloggerInterval::from_str("datalogger-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_datalogger_status_request_from_valid_str() {
        let request = DataloggerInterval::from_str("datalogger-status").unwrap();
        assert_eq!("datalogger-status", &request.to_string());
    }

    #[test]
    fn parse_temperature_datalogger_status_request_from_invalid_str_yields_err() {
        let request = DataloggerInterval::from_str("datalogger-status ");
        assert!(request.is_err());

        let request = DataloggerInterval::from_str("datalogger-status 1_000_000_000");
        assert!(request.is_err());

        let request = DataloggerInterval::from_str("datalogger-status,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_clear_request_from_valid_str() {
        let request = MemoryClear::from_str("memory-clear").unwrap();
        assert_eq!("memory-clear", &request.to_string());
    }

    #[test]
    fn parse_temperature_memory_clear_request_from_invalid_str_yields_err() {
        let request = MemoryClear::from_str("memory-clearo");
        assert!(request.is_err());

        let request = MemoryClear::from_str("memory-clear 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_recall_request_from_valid_str() {
        let request = MemoryRecall::from_str("memory-recall").unwrap();
        assert_eq!("memory-recall", &request.to_string());
    }

    #[test]
    fn parse_temperature_memory_recall_request_from_invalid_str_yields_err() {
        let request = MemoryRecall::from_str("memory-recallo");
        assert!(request.is_err());

        let request = MemoryRecall::from_str("memory-recall 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_memory_recall_last_request_from_valid_str() {
        let request = MemoryRecallLast::from_str("memory-recall-last").unwrap();
        assert_eq!("memory-recall-last", &request.to_string());
    }

    #[test]
    fn parse_temperature_memory_recall_last_request_from_invalid_str_yields_err() {
        let request = MemoryRecallLast::from_str("memory-recall-lasto");
        assert!(request.is_err());

        let request = MemoryRecallLast::from_str("memory-recall-last 10");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_read_request_from_valid_str() {
        let request = Reading::from_str("read").unwrap();
        assert_eq!("read", &request.to_string());
    }

    #[test]
    fn parse_temperature_read_request_from_invalid_str_yields_err() {
        let request = Reading::from_str("reading");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_celsius_request_from_valid_str() {
        let request = ScaleCelsius::from_str("scale-celsius").unwrap();
        assert_eq!("scale-celsius", &request.to_string());
    }

    #[test]
    fn parse_temperature_scale_celsius_request_from_invalid_str_yields_err() {
        let request = ScaleCelsius::from_str("scale-celsiusing");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_fahrenheit_request_from_valid_str() {
        let request = ScaleFahrenheit::from_str("scale-fahrenheit").unwrap();
        assert_eq!("scale-fahrenheit", &request.to_string());
    }

    #[test]
    fn parse_temperature_scale_fahrenheit_request_from_invalid_str_yields_err() {
        let request = ScaleFahrenheit::from_str("scale-fahrenheiting");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_kelvin_request_from_valid_str() {
        let request = ScaleKelvin::from_str("scale-kelvin").unwrap();
        assert_eq!("scale-kelvin", &request.to_string());
    }

    #[test]
    fn parse_temperature_scale_kelvin_request_from_invalid_str_yields_err() {
        let request = ScaleKelvin::from_str("scale-kelvining");
        assert!(request.is_err());
    }

    #[test]
    fn parse_temperature_scale_status_request_from_valid_str() {
        let request = ScaleState::from_str("scale-status").unwrap();
        assert_eq!("scale-status", &request.to_string());
    }

    #[test]
    fn parse_temperature_scale_status_request_from_invalid_str_yields_err() {
        let request = ScaleState::from_str("scale-statusing");
        assert!(request.is_err());
    }
}
