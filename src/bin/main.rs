//! Command-line interface for benita.
//!
//! Type `benita --help` on the command line to learn more about how to use it.

#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;

use std::thread;
use std::time::Duration;

use benita::errors::*;

use clap::{App, Arg, ArgMatches, SubCommand, AppSettings};

/// Main loop.
fn run_loop() -> Result<()> {
    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        // No work left, so we sleep, and avoid busy-working our program.
        thread::sleep(Duration::from_millis(1));
    }

    // Never reach this line...
}

fn parse_network_commands(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        ("network", Some(net_matches)) => {
            let _netcmd = match net_matches.subcommand() {
                ("client", Some(client_matches)) => {
                    let _subcmd = match client_matches.subcommand() {
                        ("req", Some(rep_matches)) => {
                            println!("REQ!");
                        },
                        ("sub", Some(sub_matches)) => {
                            println!("SUB!");
                        },
                        _ => unreachable!(),
                    };
                },
                ("server", Some(server_matches)) => {
                    let _subcmd = match server_matches.subcommand() {
                        ("rep", Some(rep_matches)) => {
                            println!("REP!");
                        },
                        ("pub", Some(sub_matches)) => {
                            println!("PUB!");
                        },
                        _ => unreachable!(),
                    };
                },
                _ => unreachable!(),
            };
        },
            _ => unreachable!(),
    }
}

/// Parse arguments, and execute the main loop.
fn parse_cli_arguments() -> Result<()> {
    // Network Client subcommands
    let network_client_cmd = SubCommand::with_name("client")
        .about("network client services")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(SubCommand::with_name("req")
                    .about("start a requester client")
                    .settings(&[AppSettings::ArgRequiredElseHelp])
                    .arg(Arg::with_name("url")
                         .help("URL to make requests to")
                         .required(true)
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("sub")
                    .about("start at subscription client")
                    .settings(&[AppSettings::ArgRequiredElseHelp])
                    .arg(Arg::with_name("url")
                         .help("publisher URL to subscribe to")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("channel")
                         .help("channel to subscribe to")
                         .required(true)
                         .takes_value(true)));

    // Network Server subcommands
    let network_server_cmd = SubCommand::with_name("server")
        .about("network server services")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(SubCommand::with_name("rep")
                    .about("start a responder server")
                    .settings(&[AppSettings::ArgRequiredElseHelp])
                    .arg(Arg::with_name("i2cdev")
                         .help("Path to i2cdev bus.")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("device_address")
                         .help("I2C device address.")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("url")
                         .help("URL to serve responses")
                         .required(true)
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("pub")
                    .about("start at publishing service for a response server")
                    .settings(&[AppSettings::ArgRequiredElseHelp])
                    .arg(Arg::with_name("rep-url")
                         .help("URL of the response server to be published.")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("pub-url")
                         .help("URL for the publisher")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::with_name("channel")
                         .help("channel for the publishing")
                         .required(true)
                         .takes_value(true)));

    // Network subcommands
    let network_cmd = SubCommand::with_name("network")
        .about("network server/client services")
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(network_client_cmd)
        .subcommand(network_server_cmd);

    // Defines our application and parses the argument matches
    let matches = App::new("benita")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Command-line interface for managing benita services.")
        .subcommand(SubCommand::with_name("conductivity")
                    .about("Control the conductivity sensor")
                    .settings(&[AppSettings::SubcommandRequiredElseHelp])
                    .subcommand(network_cmd.clone()))
        .subcommand(SubCommand::with_name("temperature")
                    .about("Control the temperature sensor")
                    .settings(&[AppSettings::SubcommandRequiredElseHelp])
                    .subcommand(network_cmd.clone()))
        .subcommand(SubCommand::with_name("ph")
                    .about("Control the pH sensor")
                    .settings(&[AppSettings::SubcommandRequiredElseHelp])
                    .subcommand(network_cmd.clone()))
        .get_matches();

    let _parse_cli = match matches.subcommand() {
        ("conductivity", Some(conductivity_matches)) => {
            let _subcmd = parse_network_commands(conductivity_matches);
        },
        ("temperature", Some(temperature_matches)) => {
            let _subcmd = parse_network_commands(temperature_matches);
        },
        ("ph", Some(ph_matches)) => {
            let _subcmd = parse_network_commands(ph_matches);
        },
        _ => unreachable!(),
    };

    // println!("Running benita... Press <Ctrl-C> to stop.");
    // let _run = run_loop()?;
    Ok(())
}

/// Main program that catches error-chains.
fn main() {
    if let Err(ref e) = parse_cli_arguments() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}