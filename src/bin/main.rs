//! Command-line interface for benita.
//!
//! Type `benita --help` on the command line to learn more about how to use it.

#![recursion_limit = "1024"]

extern crate benita;
extern crate chrono;
extern crate clap;
extern crate neuras;

use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

use benita::config::SensorServiceConfig as Config;
use benita::errors::{ErrorKind, Result};

use clap::{App, Arg, SubCommand, AppSettings};

/// Main loop.
fn run_loop() -> Result<()> {
    // Reactor-type loop, it will run as long as the current program runs.
    loop {
        // No work left, so we sleep, and avoid busy-working our program.
        thread::sleep(Duration::from_millis(1));
    }

    // Never reach this line...
}

/// Parse arguments, and execute the main loop.
fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Command-line interface for managing benita services.")
        .subcommand(SubCommand::with_name("temperature")
                    .about("Control the temperature sensor")
                    .settings(&[AppSettings::SubcommandRequiredElseHelp])
                    .subcommand(SubCommand::with_name("web")
                                .about("web server/client services")
                                .settings(&[AppSettings::SubcommandRequiredElseHelp])
                                .subcommand(SubCommand::with_name("client")
                                            .about("web client services")
                                            .settings(&[AppSettings::SubcommandRequiredElseHelp])
                                            .subcommand(SubCommand::with_name("start")
                                                        .about("start web client"))
                                            .subcommand(SubCommand::with_name("status")
                                                        .about("web client status"))
                                            .subcommand(SubCommand::with_name("stop")
                                                        .about("stop web client")))
                                .subcommand(SubCommand::with_name("server")
                                            .about("web server services")
                                            .settings(&[AppSettings::SubcommandRequiredElseHelp])
                                            .subcommand(SubCommand::with_name("start")
                                                        .about("start web server"))
                                            .subcommand(SubCommand::with_name("status")
                                                        .about("web server status"))
                                            .subcommand(SubCommand::with_name("stop")
                                                        .about("stop web server")))))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let mut input = String::new();
    let mut config = Config::default();

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", &c);
        let _read = File::open(&c)
            .and_then(|mut f| f.read_to_string(&mut input))
            .unwrap();

        config = Config::from_str(&input)?;
    }
    println!("Running benita... Press <Ctrl-C> to stop.");
    let _run = run_loop()?;
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
