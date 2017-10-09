//! Command-line parsers for `pH` services.
pub mod subcommands;

use cli::shared::is_url;
use self::subcommands::*;

use clap::{App, AppSettings, Arg, SubCommand};

/// Main command-line interface.
pub struct PhApp;

impl PhApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("ph")
            .bin_name("ph")
            .about("Control the pH sensor")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommand(PhServerApp::new())
            .subcommand(PhClientApp::new())
    }
}

/// pH Server command-line interface .
pub struct PhServerApp;

impl PhServerApp {
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
                    .takes_value(true),
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
                    .conflicts_with_all(&["config"]),
            )
    }
}

/// pH Client command-line interface .
pub struct PhClientApp;

impl PhClientApp {
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
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("URL")
                    .help("Sets the url for the client to connect to.")
                    .takes_value(true)
                    .index(1)
                    .required(true)
                    .validator(is_url)
                    .conflicts_with_all(&["config"]),
            )
            .subcommands(vec![
                PhCalibrationCommand::new(),
                PhCompensationCommand::new(),
                PhDeviceCommand::new(),
                PhFindCommand::new(),
                PhLedCommand::new(),
                PhProtocolLockCommand::new(),
                PhReadCommand::new(),
                PhSleepCommand::new(),
            ])
    }
}


/// Simple command-line interface.
pub struct PhCommandApp;

impl PhCommandApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        PhApp::new()
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommands(vec![
                PhCalibrationCommand::new(),
                PhCompensationCommand::new(),
                PhDeviceCommand::new(),
                PhFindCommand::new(),
                PhLedCommand::new(),
                PhProtocolLockCommand::new(),
                PhReadCommand::new(),
                PhSleepCommand::new(),
            ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the full CLI app.
    #[test]
    fn parsing_valid_server_cli_input() {
        let cli_app = PhApp::new();
        let arg_vec = vec!["ph", "server", "ipc://server"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_server_cli_input_yields_err() {
        let mut cli_app = PhApp::new();

        let arg_vec = vec!["server", "ph"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "ph", "server"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["ph", "server", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    #[test]
    fn parsing_valid_client_cli_input() {
        let cli_app = PhApp::new();
        let arg_vec = vec!["ph", "client", "ipc://server", "read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_client_cli_input_yields_err() {
        let mut cli_app = PhApp::new();

        let arg_vec = vec!["client", "ph"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "ph", "client"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["ph", "client", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["ph", "client", "not_url", "read"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    // Tests for the command-line app.
    #[test]
    fn parsing_valid_command_line_input() {
        let mut cli_app = PhCommandApp::new();

        let arg_vec = vec!["ph", "compensation", "set", "12.42"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "compensation", "get"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "calibration", "clear"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "calibration", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "calibration", "high", "12.42"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "calibration", "mid", "12.42"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "calibration", "low", "12.42"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "device", "info"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "device", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "find"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "led", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "led", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "led", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "protocol-lock", "off"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "protocol-lock", "on"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "protocol-lock", "status"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "read"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());

        let arg_vec = vec!["ph", "sleep"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_command_line_input_yields_err() {
        let mut cli_app = PhCommandApp::new();

        let arg_vec = vec!["client", "ph"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "ph", "client"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["ph", "client", "not_url"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
