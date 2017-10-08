//! Command-line parsers for `Conductivity` services.
pub mod subcommands;

use cli::shared::is_url;
use self::subcommands::*;

use clap::{App, AppSettings, Arg, SubCommand};

/// Main command-line interface.
pub struct ConductivityApp;

impl ConductivityApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("conductivity")
            .bin_name("conductivity")
            .about("Control the conductivity sensor")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Main command-line interface.
pub struct ConductivitySocketApp;

impl ConductivitySocketApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        ConductivityApp::new()
            .subcommand(ConductivityServerApp::new())
            .subcommand(ConductivityClientApp::new())
    }
}

/// Simple command-line interface.
pub struct ConductivityCommandApp;

impl ConductivityCommandApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        ConductivityApp::new()
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommand(ConductivityCalibrationCommand::new())
            .subcommand(ConductivityCompensationCommand::new())
            .subcommand(ConductivityOutputParamsCommand::new())
            .subcommand(ConductivityProbeTypeCommand::new())
            .subcommand(ConductivityDeviceCommand::new())
            .subcommand(ConductivityFindCommand::new())
            .subcommand(ConductivityLedCommand::new())
            .subcommand(ConductivityProtocolLockCommand::new())
            .subcommand(ConductivityReadCommand::new())
            .subcommand(ConductivitySleepCommand::new())
    }
}

/// Conductivity Server command-line interface .
pub struct ConductivityServerApp;

impl ConductivityServerApp {
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

/// Conductivity Client command-line interface .
pub struct ConductivityClientApp;

impl ConductivityClientApp {
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
            .subcommand(ConductivityCalibrationCommand::new())
            .subcommand(ConductivityCompensationCommand::new())
            .subcommand(ConductivityOutputParamsCommand::new())
            .subcommand(ConductivityProbeTypeCommand::new())
            .subcommand(ConductivityDeviceCommand::new())
            .subcommand(ConductivityFindCommand::new())
            .subcommand(ConductivityLedCommand::new())
            .subcommand(ConductivityProtocolLockCommand::new())
            .subcommand(ConductivityReadCommand::new())
            .subcommand(ConductivitySleepCommand::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the server socket app.
    #[test]
    fn parsing_valid_server_cli_input() {
        let cli_app = ConductivitySocketApp::new();
        let arg_vec = vec!["conductivity", "server", "ipc://server"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_server_cli_input_yields_err() {
        let mut cli_app = ConductivitySocketApp::new();

        let arg_vec = vec!["server", "conductivity"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "conductivity", "server"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["conductivity", "server", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Tests for the client socket app.
    #[test]
    fn parsing_valid_client_cli_input() {
        let cli_app = ConductivitySocketApp::new();
        let arg_vec = vec!["conductivity", "client", "ipc://server", "read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_client_cli_input_yields_err() {
        let mut cli_app = ConductivitySocketApp::new();

        let arg_vec = vec!["client", "conductivity"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "conductivity", "client"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["conductivity", "client", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["conductivity", "client", "not_url", "read"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Tests for the command-line app.
    #[test]
    fn parsing_valid_command_line_input() {
        let mut cli_app = ConductivityCommandApp::new();

        let arg_vec = vec!["conductivity", "compensation", "set", "12.42"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "compensation", "get"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "calibration", "clear"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "calibration", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "calibration", "high", "10400.420"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "calibration", "low", "1040.0"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "calibration", "single", "140.0"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "device", "info"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "device", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "find"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "led", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "led", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "led", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "all"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "none"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "ec", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "ec", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "salinity", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "salinity", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "sg", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "sg", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "tds", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "output", "tds", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "probe-type", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "probe-type", "0.1"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "probe-type", "1.0"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "probe-type", "10.0"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "protocol-lock", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "protocol-lock", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "protocol-lock", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "read"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["conductivity", "sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_command_line_input_yields_err() {
        let mut cli_app = ConductivityCommandApp::new();

        let arg_vec = vec!["client", "conductivity"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "conductivity", "client"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["conductivity", "client", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
