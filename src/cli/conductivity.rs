//! Command-line parsers for `Conductivity` services.
use errors::*;

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

        let arg_vec = vec!["conductivity", "server", "one", "two"];
        let matches = cli_app.get_matches_from_safe_borrow(arg_vec);
        assert!(matches.is_err());
    }
}
