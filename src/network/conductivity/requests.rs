//! Requests for the conductivity sensor. Requests are sent to a conductivity `Endpoint`.
use errors::*;
use network::{Endpoint, SocketRequest, SocketResponse};
use devices::conductivity::responses::ProbeReading;
use devices::conductivity::commands::Command;
use devices::conductivity::commands::Baud;
use devices::conductivity::commands::{CalibrationClear, CalibrationDry, CalibrationHigh, CalibrationLow,
                              CalibrationOnePoint, CalibrationState};
use devices::conductivity::commands::{CompensationGet, CompensationSet};
use devices::conductivity::commands::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep,
                              Status};
use devices::conductivity::commands::{Export, ExportInfo, Import};
use devices::conductivity::commands::{LedOff, LedOn, LedState};
use devices::conductivity::commands::{OutputDisableConductivity, OutputDisableSalinity,
                              OutputDisableSpecificGravity, OutputDisableTds,
                              OutputEnableConductivity, OutputEnableSalinity,
                              OutputEnableSpecificGravity, OutputEnableTds, OutputState};
use devices::conductivity::commands::{ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen};
use devices::conductivity::commands::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
use utilities::atof;

// Implements SocketRequest for commands
impl SocketRequest for CompensationGet {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<CompensationGet> {
        match req_str {
            "compensation-get" => Ok(CompensationGet),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "compensation-get".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for CompensationSet {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<CompensationSet> {
        if req_str.starts_with("compensation-set ") {
            let resp = req_str.get(17..).unwrap();
            let value = atof(resp)?;
            return Ok(CompensationSet(value));
        }
        Err(ErrorKind::RequestParse.into())
    }

    fn request_string(&self) -> String {
        format!("compensation-set {:.*}", 3, self.0)
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for Reading {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<Reading> {
        match req_str {
            "read" => Ok(Reading),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "read".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

// Implements SocketRequest for commands
impl SocketRequest for Sleep {
    type Response = ProbeReading;

    fn from_request_str(req_str: &str) -> Result<Sleep> {
        match req_str {
            "sleep" => Ok(Sleep),
            _ => Err(ErrorKind::RequestParse.into()),
        }
    }

    fn request_string(&self) -> String {
        "sleep".to_string()
    }

    fn request_to<T: Endpoint>(&self, endpoint: &T) -> Result<ProbeReading> {
        let _read = endpoint.send(self.request_string().as_bytes())
            .chain_err(|| ErrorKind::CommandRequest)?;
        let response = ProbeReading::response_from(endpoint)?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_conductivity_compensation_get_request_from_valid_str() {
        let request = CompensationGet::from_request_str("compensation-get").unwrap();
        assert_eq!("compensation-get", &request.request_string());
    }

    #[test]
    fn parse_conductivity_compensation_get_request_from_invalid_str_yields_err() {
        let request = CompensationGet::from_request_str("ompensation-get");
        assert!(request.is_err());

        let request = CompensationGet::from_request_str("compensation-get 10.5829");
        assert!(request.is_err());

        let request = CompensationGet::from_request_str("compensation-get,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_valid_str() {
        let request = CompensationSet::from_request_str("compensation-set 10.5829").unwrap();
        assert_eq!("compensation-set 10.583", &request.request_string());
    }

    #[test]
    fn parse_conductivity_compensation_set_request_from_invalid_str_yields_err() {
        let request = CompensationSet::from_request_str("compensation-set");
        assert!(request.is_err());

        let request = CompensationSet::from_request_str("compensation-set,10.5869");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_read_request_from_valid_str() {
        let request = Reading::from_request_str("read").unwrap();
        assert_eq!("read", &request.request_string());
    }

    #[test]
    fn parse_conductivity_read_request_from_invalid_str_yields_err() {
        let request = Reading::from_request_str("reading");
        assert!(request.is_err());
    }

    #[test]
    fn parse_conductivity_sleep_request_from_valid_str() {
        let request = Sleep::from_request_str("sleep").unwrap();
        assert_eq!("sleep", &request.request_string());
    }

    #[test]
    fn parse_conductivity_sleep_request_from_invalid_str_yields_err() {
        let request = Sleep::from_request_str("sleeping");
        assert!(request.is_err());
    }
}
