//! Command-line parsers for `Temperature` services.
pub mod subcommands;

use cli::shared::is_url;
use self::subcommands::*;

use clap::{App, AppSettings, Arg, SubCommand};

/// Main command-line interface.
pub struct TemperatureApp;

impl TemperatureApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("temperature")
            .bin_name("temperature")
            .about("Control the Temperature sensor")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommand(TemperatureServerApp::new())
            .subcommand(TemperatureClientApp::new())
    }
}

/// Temperature Server command-line interface .
pub struct TemperatureServerApp;

impl TemperatureServerApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("server")
            .about("REP service instance")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("rep-server-url")
                    .short("r")
                    .long("rep-server")
                    .value_name("REP_URL")
                    .help("Sets the url for the REP server")
                    .takes_value(true)
                    .index(1)
                    .required(true)
                    .validator(is_url)
                    .conflicts_with_all(&["config"])
            )
    }
}

/// Temperature Client command-line interface .
pub struct TemperatureClientApp;

impl TemperatureClientApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("client")
            .about("REQ client instance")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("URL")
                    .help("Sets the url for the client to connect to.")
                    .takes_value(true)
                    .index(1)
                    .required(true)
                    .validator(is_url)
                    .conflicts_with_all(&["config"])
            )
            .subcommands(vec![
                         TemperatureCalibrationCommand::new(),
                         TemperatureDeviceCommand::new(),
                         TemperatureFindCommand::new(),
                         TemperatureLedCommand::new(),
                         TemperatureProtocolLockCommand::new(),
                         TemperatureReadCommand::new(),
                         TemperatureSleepCommand::new(),
                         ])
    }
}

/// Temperature command-line interface .
pub struct TemperatureCommandApp;

impl TemperatureCommandApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        TemperatureApp::new()
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommands(vec![
                         TemperatureCalibrationCommand::new(),
                         TemperatureDeviceCommand::new(),
                         TemperatureFindCommand::new(),
                         TemperatureLedCommand::new(),
                         TemperatureProtocolLockCommand::new(),
                         TemperatureReadCommand::new(),
                         TemperatureSleepCommand::new(),
                        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the full CLI app.
    #[test]
    fn parsing_valid_server_cli_input() {
        let cli_app = TemperatureApp::new();
        let arg_vec = vec!["temperature", "server", "ipc://server"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_server_cli_input_yields_err() {
        let mut cli_app = TemperatureApp::new();

        let arg_vec = vec!["server", "temperature"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "temperature", "server"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["temperature", "server", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    #[test]
    fn parsing_valid_client_cli_input() {
        let cli_app = TemperatureApp::new();
        let arg_vec = vec!["temperature", "client", "ipc://server", "read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_client_cli_input_yields_err() {
        let mut cli_app = TemperatureApp::new();

        let arg_vec = vec!["client", "temperature"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "temperature", "client"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["temperature", "client", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["temperature", "client", "not_url", "read"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Tests for the command-line app.
    #[test]
    fn parsing_valid_command_line_input() {
        let mut cli_app = TemperatureCommandApp::new();

        let arg_vec = vec!["temperature", "calibration", "set", "12.42"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "calibration", "clear"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "calibration", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "device", "info"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "device", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "find"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "led", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "led", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "led", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "protocol-lock", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "protocol-lock", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "protocol-lock", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "read"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["temperature", "sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_command_line_input_yields_err() {
        let mut cli_app = TemperatureCommandApp::new();

        let arg_vec = vec!["client", "temperature"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "temperature", "client"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["temperature", "client", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
