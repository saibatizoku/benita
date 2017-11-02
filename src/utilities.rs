//! Shared useful utility functions.
use errors::*;
use neuras::zmq::Socket;
use neuras::utils::{bind_socket, connect_socket, create_context, zmq_rep};

/// simple atof conversion.
pub fn atof(s: &str) -> Result<f64> {
    let _float = s.parse().chain_err(|| ErrorKind::NumberParse)?;
    Ok(_float)
}

/// create a RESPONDER socket bound to the specified URL.
pub fn create_and_bind_responder(url: &str) -> Result<Socket> {
    // We start our ZMQ context.
    let context = create_context();
    // We configure our socket as REP, for accepting requests
    let responder = zmq_rep(&context)?;
    // We bind our socket to URL.
    let _bind_socket = bind_socket(&responder, url).chain_err(|| "problems binding to socket")?;
    Ok(responder)
}

/// create a RESPONDER socket connected to the specified URL.
pub fn create_and_connect_responder(url: &str) -> Result<Socket> {
    // We start our ZMQ context.
    let context = create_context();
    // We configure our socket as REP, for accepting requests
    let responder = zmq_rep(&context)?;
    // We bind our socket to URL.
    let _bind_socket = connect_socket(&responder, url).chain_err(|| "problems connecting to socket")?;
    Ok(responder)
}
