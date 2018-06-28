//! Shared useful utility functions.
use super::errors::*;
use zmq::{Context, Socket, SocketType};

/// simple atof conversion.
pub fn atof(s: &str) -> Result<f64> {
    let _float = s.parse::<f64>().context(ErrorKind::NumberParse)?;
    Ok(_float)
}

/// create a REQUESTER socket bound to the specified URL.
pub fn create_and_bind_requester(url: &str) -> Result<Socket> {
    // We start our ZMQ context.
    let context = Context::new();
    // We configure our socket as REQ, for accepting requests
    let requester = context
        .socket(SocketType::REQ)
        .context(ErrorKind::SocketCreate)?;
    // We bind our socket to URL.
    let _bind_socket = requester.bind(url).context(ErrorKind::SocketBind)?;
    Ok(requester)
}

/// create a REQUESTER socket connected to the specified URL.
pub fn create_and_connect_requester(url: &str) -> Result<Socket> {
    // We start our ZMQ context.
    let context = Context::new();
    // We configure our socket as REQ, for accepting requests
    let requester = context
        .socket(SocketType::REQ)
        .context(ErrorKind::SocketCreate)?;
    // We bind our socket to URL.
    let _bind_socket = requester.connect(url).context(ErrorKind::SocketConnect)?;
    Ok(requester)
}

/// create a RESPONDER socket bound to the specified URL.
pub fn create_and_bind_responder(url: &str) -> Result<Socket> {
    // We start our ZMQ context.
    let context = Context::new();
    // We configure our socket as REP, for accepting requests
    let responder = context
        .socket(SocketType::REP)
        .context(ErrorKind::SocketCreate)?;
    // We bind our socket to URL.
    let _bind_socket = responder.bind(url).context(ErrorKind::SocketBind)?;
    Ok(responder)
}

/// create a RESPONDER socket connected to the specified URL.
pub fn create_and_connect_responder(url: &str) -> Result<Socket> {
    // We start our ZMQ context.
    let context = Context::new();
    // We configure our socket as REP, for accepting requests
    let responder = context
        .socket(SocketType::REP)
        .context(ErrorKind::SocketCreate)?;
    // We bind our socket to URL.
    let _bind_socket = responder.connect(url).context(ErrorKind::SocketConnect)?;
    Ok(responder)
}
