//! Library Error, and ErrorKind definitions.
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};
use std::result;

pub use failure::ResultExt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Copy, Clone, Eq, Debug, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "could not parse address")]
    AddressParse,
    #[fail(display = "could not parse command")]
    CommandParse,
    #[fail(display = "command request failed")]
    CommandRequest,
    #[fail(display = "command reply failed")]
    CommandReply,
    #[fail(display = "command response failed")]
    CommandResponse,
    #[fail(display = "could not parse configuration file")]
    ConfigParse,
    #[fail(display = "socket could not bind to the network URL")]
    SocketBind,
    #[fail(display = "the socket couldn't be created")]
    SocketCreate,
    #[fail(display = "socket could not connect to the network URL")]
    SocketConnect,
    #[fail(display = "message could not be sent to the network")]
    SocketSend,
    #[fail(display = "message could not be received from the network")]
    SocketReceive,
    #[fail(display = "our network has gone neurotic")]
    Neurotic,
    #[fail(display = "this is not a number")]
    NumberParse,
    #[fail(display = "could not parse request")]
    RequestParse,
    #[fail(display = "could not parse response")]
    ResponseParse,
    #[fail(display = "trouble with the sensor")]
    SensorTrouble,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}
