//! Command-line parsers for `Conductivity` services.
use clap::{App, AppSettings, Arg, SubCommand};

pub struct ConductivityApp;

impl ConductivityApp {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        App::new("conductivity")
            .bin_name("conductivity")
            .about("Control the conductivity sensor")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .subcommand(ConductivityServerApp::new())
    }
}

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
                    .conflicts_with_all(&["config"]),
            )
    }
}

/// Parses the command for temperature compensation of Conductivity readings.
pub struct ConductivityCompensateCommand;

impl ConductivityCompensateCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("compensate")
            .about("Compensate command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
            .arg(
                Arg::with_name("TEMP")
                    .help("Sets the compensation temperature.")
                    .takes_value(true)
                    .index(1)
                    .required(true)
            )
    }
}

/// Parses the command for taking a reading from the Conductivity sensor.
pub struct ConductivityReadingCommand;

impl ConductivityReadingCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("sleep")
            .about("Sleep command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

/// Parses the command for putting the Conductivity sensor to sleep (low-power mode).
pub struct ConductivitySleepCommand;

impl ConductivitySleepCommand {
    pub fn new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("sleep")
            .about("Sleep command.")
            .settings(&[AppSettings::DisableHelpSubcommand])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_valid_server_cli_input() {
        let cli_app = ConductivityApp::new();
        let arg_vec = vec!["conductivity", "server", "ipc://server"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        println!("matches: {:?}", &matches);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_server_cli_input_yields_err() {
        let mut cli_app = ConductivityApp::new();

        let arg_vec = vec!["server", "conductivity"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["prefixed", "conductivity", "server"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());

        let arg_vec = vec!["conductivity", "server", "arg1", "arg2"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    #[test]
    fn parsing_valid_compensate_command_input() {
        let cli_app = ConductivityCompensateCommand::new();
        let arg_vec = vec!["compensate", "arg"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_compensate_command_input_yields_err() {
        let mut cli_app = ConductivityCompensateCommand::new();
        let arg_vec = vec!["compensate"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    #[test]
    fn parsing_valid_read_command_input() {
        let cli_app = ConductivityReadingCommand::new();
        let arg_vec = vec!["read"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_read_command_input_yields_err() {
        let mut cli_app = ConductivityReadingCommand::new();
        let arg_vec = vec!["read", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }

    #[test]
    fn parsing_valid_sleep_command_input() {
        let cli_app = ConductivitySleepCommand::new();
        let arg_vec = vec!["sleep"];
        let matches = cli_app.get_matches_from_safe(arg_vec);
        assert!(matches.is_ok());
    }

    #[test]
    fn parsing_invalid_sleep_command_input_yields_err() {
        let mut cli_app = ConductivitySleepCommand::new();
        let arg_vec = vec!["sleep", "arg"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
