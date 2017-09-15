//! Command-line interface for benita.
//!
//! Type `benita --help` on the command line to learn more about how to use it.

#![recursion_limit = "1024"]

extern crate benita;

use std::thread;
use std::time::Duration;

use benita::cli::benita::{benita_cli_parser, sensor_cli_parser};
use benita::errors::*;

/// Main loop.
#[allow(dead_code)]
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
    // Defines our application and parses the argument matches
    let matches = benita_cli_parser().get_matches();

    let _parse_cli = match matches.subcommand() {
        ("conductivity", Some(_conductivity_matches)) => {
            let _subcmd = sensor_cli_parser(_conductivity_matches)?;
        }
        ("temperature", Some(_temperature_matches)) => {
            let _subcmd = sensor_cli_parser(_temperature_matches)?;
        }
        ("ph", Some(_ph_matches)) => {
            let _subcmd = sensor_cli_parser(_ph_matches)?;
        }
        _ => unreachable!(),
    };

    // Inform that the service has started.
    println!("Running benita... Press <Ctrl-C> to stop.");
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
