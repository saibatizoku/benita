//! Sends remote commands to the RTD EZO chip, using the exposed a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.
extern crate benita;
extern crate clap;
extern crate failure;
extern crate neuras;
extern crate zmq;

use benita::ezo::common_ezo::EzoChipAPI;
use benita::ezo::conductivity::network::ConductivityRequester;
use benita::ezo::conductivity::ConductivityAPI;
use benita::ezo::errors::ErrorKind;

use std::result;
use clap::{App, Arg};
use failure::Error;
use zmq::{Context, REQ};

type Result<T> = result::Result<T, Error>;

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-subscriber")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. Requester client.")
        .arg(
            Arg::with_name("rep-url")
                .short("b")
                .long("rep")
                .value_name("REP_URL")
                .help("Sets the url for the REP server")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .multiple(true)
                .help("Turn debugging information on"),
        )
        .get_matches();

    let rep_url = match matches.value_of("rep-url") {
        Some(repurl) => repurl,
        _ => return Err(ErrorKind::ConfigParse)?,
    };

    let _run = run_client(&rep_url)?;

    // Never reach this line...
    Ok(())
}

fn run_client(rep_url: &str) -> Result<()> {
    let context = Context::new();
    let req_socket = context.socket(REQ)?;
    let _connect = req_socket.connect(rep_url)?;

    let ec_client = ConductivityRequester::new(req_socket)?;

    {
        println!("Requesting 'get_output_params'");
        let output_params = ec_client.get_output_params()?;
        println!("{}", output_params);

        println!("Requesting 'read'");
        let read = ec_client.get_reading()?;
        println!("{}", read);

        println!("Requesting 'sleep'");
        let sleep = ec_client.set_sleep()?;
        println!("{}", sleep);
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = parse_cli_arguments() {
        println!("error: {:?}", e.cause());
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        let backtrace = e.backtrace();
        println!("backtrace: {:?}", backtrace);
        ::std::process::exit(1);
    }
}
