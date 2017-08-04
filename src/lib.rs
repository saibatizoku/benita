//! # benita
//!
//! Benita provides a client to interact with sensors over I2C. It is designed
//! for usage on embedded devices with SoC (Systems On Chip) with Linux.
//!
//! The included aquatic sensors are:
//!
//! * pH
//! * Electrical Conductivity
//! * Temperature

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate i2cdev;
extern crate neuras;
#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod errors;

use errors::*;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;


#[derive(Clone)]
pub struct SensingDevice {
    bus: u8,
    address: u16,
}

pub trait I2cCommand {
    fn to_bytes(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

pub trait I2cSensing {
    fn new(bus: u8, address: u16) -> Self;
    fn send<T: I2cCommand>(&self, cmd: T) -> Result<()>;
}

impl I2cSensing for SensingDevice {
    fn new(bus: u8, address: u16) -> SensingDevice {
        SensingDevice { bus, address }
    }
    fn send<T: I2cCommand>(&self, cmd: T) -> Result<()> {
        let bus = format!("/dev/i2c-{}", self.bus);
        let mut dev = LinuxI2CDevice::new(bus, self.address)
            .chain_err(|| "Could not open I2C device")?;
        dev.write(&cmd.to_bytes())
            .chain_err(|| "Could not send command")
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct ProxyConfig<'a> {
    pub backend: &'a str,
    pub frontend: &'a str,
}

impl<'a> ProxyConfig<'a> {
    pub fn from_str(config_str: &str) -> Result<ProxyConfig> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct ServiceConfig<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub url: &'a str,
    pub socket_type: &'a str,
    pub channels: Option<Vec<&'a str>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct Config<'a> {
    pub service: Option<Vec<ServiceConfig<'a>>>,
    pub pub_url: &'a str,
    pub channel: &'a str,
    pub rep_url: &'a str,
}

impl<'a> Config<'a> {
    pub fn from_str(config_str: &str) -> Result<Config> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_parses_config_toml() {

        // Files with correct fields parse
        let config_str = r#"
            pub_url = "tcp://127.0.0.1:5558"
            channel = "temperature-0123456789abcdef"
            rep_url = "tcp://127.0.0.1:5557"

            [[service]]
            name = "Main temperature source"
            description = "Reference temperature for this application"
            url = "tcp://127.0.0.1:5558"
            socket_type = "PUB"
            channels = ["temperature-0123456789abcdef"]
            "#;

        let config = Config::from_str(config_str).unwrap();
        let services = vec![ServiceConfig {
                                name: Some("Main temperature source"),
                                description: Some("Reference temperature for this application"),
                                url: "tcp://127.0.0.1:5558",
                                socket_type: "PUB",
                                channels: Some(vec!["temperature-0123456789abcdef"]),
                            }];
        assert_eq!(config,
                   Config {
                       pub_url: "tcp://127.0.0.1:5558",
                       channel: "temperature-0123456789abcdef",
                       rep_url: "tcp://127.0.0.1:5557",
                       service: Some(services),
                   });

        // Unknown fields are ignored
        let config_str = r#"
            pub_url = "tcp://127.0.0.1:5558"
            channel = "temperature-0123456789abcdef"
            rep_url = "tcp://127.0.0.1:5557"
            proxy_url = "tcp://127.0.0.1:5550"
            another_extra = "yeah"
            "#;

        let config = Config::from_str(config_str).unwrap();
        assert_eq!(config,
                   Config {
                       pub_url: "tcp://127.0.0.1:5558",
                       channel: "temperature-0123456789abcdef",
                       rep_url: "tcp://127.0.0.1:5557",
                       service: None,
                   });
    }

    #[test]
    fn reads_and_parses_invalid_config_toml_yielding_err() {
        // Files with no known fields yield error
        let config_str = r#""#;

        let config: Result<Config> = Config::from_str(config_str);
        assert!(config.is_err());

        // Files with invalid field values yield error
        let config_str = r#"
            pub_url = "tcp://127.0.0.1:5558"
            channel = "temperature-0123456789abcdef"
            rep_url = 1234
            "#;

        let config: Result<Config> = Config::from_str(config_str);
        assert!(config.is_err());
    }

    #[test]
    fn reads_and_parses_proxy_config_toml() {

        // Files with correct fields parse
        let config_str = r#"
            backend = "ipc://temp.ipc"
            frontend = "tcp://127.0.0.1:5558"
            "#;

        let config = ProxyConfig::from_str(config_str).unwrap();
        assert_eq!(config,
                   ProxyConfig {
                       backend: "ipc://temp.ipc",
                       frontend: "tcp://127.0.0.1:5558",
                   });

        // Unknown fields are ignored
        let config_str = r#"
            backend = "ipc://temp.ipc"
            frontend = "tcp://127.0.0.1:5558"
            channel = "temperature-0123456789abcdef"
            "#;

        let config = ProxyConfig::from_str(config_str).unwrap();
        assert_eq!(config,
                   ProxyConfig {
                       backend: "ipc://temp.ipc",
                       frontend: "tcp://127.0.0.1:5558",
                   });
    }

    #[test]
    fn reads_and_parses_invalid_proxy_config_toml_yielding_err() {
        // Files with no known fields yield error
        let config_str = r#""#;

        let config: Result<ProxyConfig> = ProxyConfig::from_str(config_str);
        assert!(config.is_err());

        // Files with invalid field values yield error
        let config_str = r#"
            backend = 0
            frontend = "tcp://127.0.0.1:5558"
            "#;

        let config: Result<ProxyConfig> = ProxyConfig::from_str(config_str);
        assert!(config.is_err());
    }
}
