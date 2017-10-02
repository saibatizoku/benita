//! Sends remote commands to the RTD EZO chip, using the exposed a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;
extern crate neuras;

use benita::errors::{ErrorKind, Result};
use benita::network::conductivity::ConductivityClient;

use clap::{App, Arg};
use neuras::utils::{connect_socket, create_context, zmq_req};

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
        _ => return Err(ErrorKind::ConfigParse.into()),
    };

    let _run = run_client(&rep_url)?;

    // Never reach this line...
    Ok(())
}

fn run_client(rep_url: &str) -> Result<()> {
    let context = create_context();
    let req_socket = zmq_req(&context)?;
    let _connect = connect_socket(&req_socket, rep_url)?;

    let ec_client = ConductivityClient::new(req_socket)?;

    {
        println!("Requesting 'get_output_params'");
        let output_params = ec_client.get_output_params()?;
        println!("{}", output_params);

        println!("Requesting 'read'");
        let read = ec_client.send_read()?;
        println!("{}", read);

        println!("Requesting 'sleep'");
        let sleep = ec_client.send_sleep()?;
        println!("{}", sleep);
    }
    Ok(())
}

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
