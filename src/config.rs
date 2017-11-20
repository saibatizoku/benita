//! Configuration settings for sensors and network sockets, using `toml`.
//!
//! `benita` sets up sensors and network configurations using `toml` and `serde`.
pub mod errors {
    error_chain!{}
}

use std::path::PathBuf;

use errors::*;
use toml;

/// Socket connection type. Can be `Bind` or `Connect`.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ConnectionType {
    #[serde(rename = "bind")] Bind,
    #[serde(rename = "connect")] Connect,
}

impl ConnectionType {
    pub fn from_str(config_str: &str) -> Result<ConnectionType> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

impl Default for ConnectionType {
    fn default() -> ConnectionType {
        ConnectionType::Connect
    }
}

/// Configuration settings for network sockets.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct SocketConfig<'a> {
    pub url: &'a str,
    #[serde(default)] pub socket_connection: ConnectionType,
}

impl<'a> SocketConfig<'a> {
    pub fn from_str(config_str: &str) -> Result<SocketConfig> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

/// Configuration settings for I2C sensors.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct SensorConfig {
    pub path: PathBuf,
    pub address: u16,
}

impl SensorConfig {
    pub fn new(path_str: &str, address: u16) -> SensorConfig {
        let path = PathBuf::from(path_str);
        SensorConfig { path, address }
    }

    pub fn from_str(config_str: &str) -> Result<SensorConfig> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

/// Configuration settings for the calibrated sensors service.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct SensorServiceConfig<'a> {
    pub pub_url: &'a str,
    pub channel: &'a str,
    pub rep_ec_url: &'a str,
    pub rep_ph_url: &'a str,
}

impl<'a> SensorServiceConfig<'a> {
    pub fn from_str(config_str: &str) -> Result<SensorServiceConfig> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

/// Configuration settings for networked proxies.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct ProxyConfig<'a> {
    pub backend_url: &'a str,
    #[serde(default)] pub backend_connection: ConnectionType,
    pub frontend_url: &'a str,
    #[serde(default)] pub frontend_connection: ConnectionType,
}

impl<'a> ProxyConfig<'a> {
    pub fn from_str(config_str: &str) -> Result<ProxyConfig> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SensorServiceConfig for valid use cases
    #[test]
    fn reads_and_parses_sensor_service_config_toml() {
        // Files with correct fields parse
        let config_str = r#"
            pub_url = "ipc://tmp/benita.temp.ipc"
            channel = "01234-id"
            rep_ec_url = "ipc://tmp/benita.ec.ipc"
            rep_ph_url = "ipc://tmp/benita.ph.ipc"
            "#;

        let config = SensorServiceConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            SensorServiceConfig {
                pub_url: "ipc://tmp/benita.temp.ipc",
                channel: "01234-id",
                rep_ec_url: "ipc://tmp/benita.ec.ipc",
                rep_ph_url: "ipc://tmp/benita.ph.ipc",
            }
        );

        // Unknown fields are ignored
        let config_str = r#"
            pub_url = "ipc://tmp/benita.temp.ipc"
            channel = "01234-id"
            rep_ec_url = "ipc://tmp/benita.ec.ipc"
            rep_ph_url = "ipc://tmp/benita.ph.ipc"
            extra = "unseen"
            "#;

        let config = SensorServiceConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            SensorServiceConfig {
                pub_url: "ipc://tmp/benita.temp.ipc",
                channel: "01234-id",
                rep_ec_url: "ipc://tmp/benita.ec.ipc",
                rep_ph_url: "ipc://tmp/benita.ph.ipc",
            }
        );
    }

    // SensorServiceConfig for invalid use cases.
    #[test]
    fn reads_and_parses_invalid_sensor_service_config_toml_yielding_err() {
        // Files with no known fields yield error
        let config_str = r#""#;

        let config: Result<SensorServiceConfig> = SensorServiceConfig::from_str(config_str);
        assert!(config.is_err());

        // Files with invalid field values yield error
        let config_str = r#"
            backend_url = 0
            frontend_url = "tcp://127.0.0.1:5558"
            "#;

        let config: Result<SensorServiceConfig> = SensorServiceConfig::from_str(config_str);
        assert!(config.is_err());
    }

    // SensorConfig for valid use cases
    #[test]
    fn reads_and_parses_sensor_config_toml() {
        // Files with correct fields parse
        let config_str = r#"
            path = "/dev/i2c-0"
            address = 100
            "#;

        let config = SensorConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            SensorConfig {
                path: PathBuf::from("/dev/i2c-0"),
                address: 100,
            }
        );
    }

    // SensorConfig for invalid use cases
    #[test]
    fn reads_and_parses_invalid_sensor_config_toml_yielding_err() {
        // empty toml
        let config_str = r#""#;

        let config: Result<SensorConfig> = SensorConfig::from_str(config_str);
        assert!(config.is_err());

        // empty fields
        let config_str = r#"
            path =
            address =
            "#;

        let config: Result<SensorConfig> = SensorConfig::from_str(config_str);
        assert!(config.is_err());

        // Address is not a valid `u16`
        let config_str = r#"
            path = "/dev/i2c-0"
            address = 1000000
            "#;

        let config: Result<SensorConfig> = SensorConfig::from_str(config_str);
        assert!(config.is_err());
    }

    // SocketConfig for valid use cases
    #[test]
    fn reads_and_parses_socket_config_toml() {
        // Files with correct fields parse
        let config_str = r#"
            url = "ipc://temp.ipc"
            "#;

        let config = SocketConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            SocketConfig {
                url: "ipc://temp.ipc",
                socket_connection: ConnectionType::Connect,
            }
        );

        let config_str = r#"
            url = "ipc://temp.ipc"
            socket_connection = "connect"
            "#;

        let config = SocketConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            SocketConfig {
                url: "ipc://temp.ipc",
                socket_connection: ConnectionType::Connect,
            }
        );
    }
    //
    // SocketConfig for invalid use cases
    #[test]
    fn reads_and_parses_invalid_socket_config_toml_yielding_err() {
        // Files with no known fields yield error
        let config_str = r#""#;

        let config: Result<SocketConfig> = SocketConfig::from_str(config_str);
        assert!(config.is_err());

        // Files with invalid field values yield error
        let config_str = r#"
            socket_connection = "bind"
            "#;

        let config: Result<SocketConfig> = SocketConfig::from_str(config_str);
        assert!(config.is_err());
    }

    // ProxyConfig for valid use cases
    #[test]
    fn reads_and_parses_proxy_config_toml() {
        // Files with correct fields parse
        let config_str = r#"
            backend_url = "ipc://temp.ipc"
            frontend_url = "tcp://127.0.0.1:5558"
            "#;

        let config = ProxyConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            ProxyConfig {
                backend_url: "ipc://temp.ipc",
                frontend_url: "tcp://127.0.0.1:5558",
                backend_connection: ConnectionType::Connect,
                frontend_connection: ConnectionType::Connect,
            }
        );

        // Unknown fields are ignored
        let config_str = r#"
            backend_url = "ipc://temp.ipc"
            frontend_url = "tcp://127.0.0.1:5558"
            backend_connection = "bind"
            # frontend_connection = "connect" # Default value
            channel = "temperature-0123456789abcdef"
            "#;

        let config = ProxyConfig::from_str(config_str).unwrap();
        assert_eq!(
            config,
            ProxyConfig {
                backend_url: "ipc://temp.ipc",
                frontend_url: "tcp://127.0.0.1:5558",
                backend_connection: ConnectionType::Bind,
                frontend_connection: ConnectionType::Connect,
            }
        );
    }

    // ProxyConfig for invalid use cases
    #[test]
    fn reads_and_parses_invalid_proxy_config_toml_yielding_err() {
        // Files with no known fields yield error
        let config_str = r#""#;

        let config: Result<ProxyConfig> = ProxyConfig::from_str(config_str);
        assert!(config.is_err());

        // Files with invalid field values yield error
        let config_str = r#"
            backend_url = 0
            frontend_url = "tcp://127.0.0.1:5558"
            "#;

        let config: Result<ProxyConfig> = ProxyConfig::from_str(config_str);
        assert!(config.is_err());
    }
}
