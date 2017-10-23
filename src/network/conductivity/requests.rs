//! Requests for the conductivity sensor. Requests are sent to a conductivity `Endpoint`.
use errors::*;

use network::{Endpoint, SocketRequest, SocketResponse};
use network::common::OkResponse;

pub use devices::conductivity::commands::Baud;
pub use devices::conductivity::commands::{CalibrationClear, CalibrationDry, CalibrationHigh, CalibrationLow,
                              CalibrationOnePoint, CalibrationState};
pub use devices::conductivity::commands::{CompensationGet, CompensationSet};
pub use devices::conductivity::commands::{DeviceAddress, DeviceInformation, Factory, Find, Reading, Sleep,
                              Status};
pub use devices::conductivity::commands::{Export, ExportInfo, Import};
pub use devices::conductivity::commands::{LedOff, LedOn, LedState};
pub use devices::conductivity::commands::{OutputDisableConductivity, OutputDisableSalinity,
                              OutputDisableSpecificGravity, OutputDisableTds,
                              OutputEnableConductivity, OutputEnableSalinity,
                              OutputEnableSpecificGravity, OutputEnableTds, OutputState};
pub use devices::conductivity::commands::{ProbeTypeOne, ProbeTypePointOne, ProbeTypeState, ProbeTypeTen};
pub use devices::conductivity::commands::{ProtocolLockDisable, ProtocolLockEnable, ProtocolLockState};
pub use devices::conductivity::responses::CompensationValue;
use utilities::atof;

use ezo_common::BpsRate;

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

    fn assert_valid_baud_request(test_str: &str, bps: BpsRate) {
        let request = Baud::from_request_str(test_str).unwrap();
        assert_eq!(test_str, &request.request_string());
        assert_eq!(bps, request.0);
    }
    #[test]
    fn parse_conductivity_baud_request_from_valid_str() {
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
    fn parse_conductivity_baud_request_from_invalid_str_yields_err() {
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
