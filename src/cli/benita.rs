//! Command-line argument parsers for common services.
use errors::*;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

/// A parser for `benita` sensor network sub-commands.
pub fn sensor_network_cli_parser(matches: &ArgMatches) -> Result<()> {
    let _netcmd = match matches.subcommand() {
        ("client", Some(_client_matches)) => {
            let _subcmd = match _client_matches.subcommand() {
                ("req", Some(_rep_matches)) => {
                    println!("REQ!");
                }
                ("sub", Some(_sub_matches)) => {
                    println!("SUB!");
                }
                _ => unreachable!(),
            };
        }
        ("server", Some(_server_matches)) => {
            let _subcmd = match _server_matches.subcommand() {
                ("rep", Some(_rep_matches)) => {
                    println!("REP!");
                }
                ("pub", Some(_sub_matches)) => {
                    println!("PUB!");
                }
                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    };
    Ok(())
}

/// A parser for `benita` sensor sub-commands.
pub fn sensor_cli_parser(matches: &ArgMatches) -> Result<()> {
    let _parse_matches = match matches.subcommand() {
        ("network", Some(net_matches)) => {
            let _netcmd = sensor_network_cli_parser(net_matches)?;
        }
        _ => unreachable!(),
    };
    Ok(())
}

/// `benita` command-line application for sensor and network services.
pub fn benita_cli_parser<'a, 'b>() -> App<'a, 'b> {
    // Network Client subcommands
    let network_client_cmd = SubCommand::with_name("client")
        .about("network client services")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(
            SubCommand::with_name("req")
                .about("start a requester client")
                .settings(&[AppSettings::ArgRequiredElseHelp])
                .arg(
                    Arg::with_name("url")
                        .help("URL to make requests to")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("sub")
                .about("start at subscription client")
                .settings(&[AppSettings::ArgRequiredElseHelp])
                .arg(
                    Arg::with_name("url")
                        .help("publisher URL to subscribe to")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("channel")
                        .help("channel to subscribe to")
                        .required(true)
                        .takes_value(true),
                ),
        );

    // Network Server subcommands
    let network_server_cmd = SubCommand::with_name("server")
        .about("network server services")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(
            SubCommand::with_name("rep")
                .about("start a responder server")
                .settings(&[AppSettings::ArgRequiredElseHelp])
                .arg(
                    Arg::with_name("i2cdev")
                        .help("Path to i2cdev bus.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("device_address")
                        .help("I2C device address.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("url")
                        .help("URL to serve responses")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pub")
                .about("start at publishing service for a response server")
                .settings(&[AppSettings::ArgRequiredElseHelp])
                .arg(
                    Arg::with_name("rep-url")
                        .help("URL of the response server to be published.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("pub-url")
                        .help("URL for the publisher")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("channel")
                        .help("channel for the publishing")
                        .required(true)
                        .takes_value(true),
                ),
        );

    // Network subcommands
    let network_cmd = SubCommand::with_name("network")
        .about("network server/client services")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(network_client_cmd)
        .subcommand(network_server_cmd);

    App::new("benita")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Command-line interface for managing benita services.")
        .subcommand(
            SubCommand::with_name("conductivity")
                .about("Control the conductivity sensor")
                .settings(&[AppSettings::SubcommandRequiredElseHelp])
                .subcommand(network_cmd.clone()),
        )
        .subcommand(
            SubCommand::with_name("temperature")
                .about("Control the temperature sensor")
                .settings(&[AppSettings::SubcommandRequiredElseHelp])
                .subcommand(network_cmd.clone()),
        )
        .subcommand(
            SubCommand::with_name("ph")
                .about("Control the pH sensor")
                .settings(&[AppSettings::SubcommandRequiredElseHelp])
                .subcommand(network_cmd.clone()),
        )
}
