//! Sends remote commands to the RTD EZO chip, using the exposed a limited API.
//!
//! This server binds to the `REP_URL` argument, expected from the command line.

// error-chain recurses deeply
#![recursion_limit = "1024"]

extern crate benita;
extern crate clap;
extern crate neuras;

use benita::errors::{ErrorKind, Result, ResultExt};
use clap::{App, Arg};
use neuras::{create_context, create_message, connect_client, zmq_req};

fn parse_cli_arguments() -> Result<()> {
    let matches = App::new("benita-subscriber")
        .version("0.1.0")
        .author("Joaquin R. <globojorro@gmail.com>")
        .about("Benita IoT. Requester client.")
        .arg(Arg::with_name("rep-url")
                 .short("b")
                 .long("rep")
                 .value_name("REP_URL")
                 .help("Sets the url for the REP server")
                 .takes_value(true)
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("debug")
                 .short("d")
                 .multiple(true)
                 .help("Turn debugging information on"))
        .get_matches();

    let rep_url = match matches.value_of("rep-url") {
        Some(repurl) => repurl,
        _ => return Err(ErrorKind::ConfigParse.into()),
    };

    let _run =  run_requester(&rep_url)?;

    // Never reach this line...
    Ok(())
}

fn run_requester(rep_url: &str) -> Result<()> {
    let context = create_context();
    let requester = zmq_req(&context)?;

    let _connect = connect_client(&requester, rep_url)?;

    let mut msg = create_message()?;

    {
        println!("Requesting 'get_output_params'");
        requester.send("get_params".as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());

        println!("Requesting 'read'");
        requester.send("read".as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());

        println!("Requesting 'sleep'");
        requester.send("sleep".as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("{}", msg.as_str().unwrap());
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
