//! Configure sensor and network settings in `toml`.
//!
//! `benita` sets up sensors and network configurations using `toml` and `serde`.

use errors::*;
use toml;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct ProxyConfig<'a> {
    pub backend_url: &'a str,
    pub frontend_url: &'a str,
}

impl<'a> ProxyConfig<'a> {
    pub fn from_str(config_str: &str) -> Result<ProxyConfig> {
        toml::from_str(config_str).chain_err(|| ErrorKind::ConfigParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_parses_proxy_config_toml() {
        // Files with correct fields parse
        let config_str = r#"
            backend_url = "ipc://temp.ipc"
            frontend_url = "tcp://127.0.0.1:5558"
            "#;

        let config = ProxyConfig::from_str(config_str).unwrap();
        assert_eq!(config,
                   ProxyConfig {
                       backend_url: "ipc://temp.ipc",
                       frontend_url: "tcp://127.0.0.1:5558",
                   });

        // Unknown fields are ignored
        let config_str = r#"
            backend_url = "ipc://temp.ipc"
            frontend_url = "tcp://127.0.0.1:5558"
            channel = "temperature-0123456789abcdef"
            "#;

        let config = ProxyConfig::from_str(config_str).unwrap();
        assert_eq!(config,
                   ProxyConfig {
                       backend_url: "ipc://temp.ipc",
                       frontend_url: "tcp://127.0.0.1:5558",
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
            backend_url = 0
            frontend_url = "tcp://127.0.0.1:5558"
            "#;

        let config: Result<ProxyConfig> = ProxyConfig::from_str(config_str);
        assert!(config.is_err());
    }
}
