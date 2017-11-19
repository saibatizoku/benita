//! Common network items.
pub mod errors {
    error_chain! {
    }
}

use errors::*;

pub use super::traits::*;

/// `ok` reply.
#[derive(PartialEq)]
pub enum ReplyStatus {
    Ok,
    Err,
}

impl ReplyStatus {
    fn parse(rep_str: &str) -> Result<ReplyStatus> {
        match rep_str {
            "ok" => Ok(ReplyStatus::Ok),
            "err" => Ok(ReplyStatus::Err),
            _ => Err(ErrorKind::ResponseParse.into()),
        }
    }
}

impl ::std::fmt::Debug for ReplyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ReplyStatus::Ok => write!(f, "ok"),
            ReplyStatus::Err => write!(f, "err"),
        }
    }
}

impl ::std::fmt::Display for ReplyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_SocketReply_for!(ReplyStatus);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_status_reply_from_valid_str() {
        let reply = ReplyStatus::parse_response("ok").unwrap();
        assert_eq!(reply, ReplyStatus::Ok);

        let reply = ReplyStatus::parse_response("err").unwrap();
        assert_eq!(reply, ReplyStatus::Err);
    }

    #[test]
    fn create_status_reply_from_invalid_str_yields_err() {
        let reply = ReplyStatus::parse_response("okerr");
        assert!(reply.is_err());
    }
}
