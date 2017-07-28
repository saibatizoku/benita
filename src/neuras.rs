
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_request_client () {
        let ctx = zmq::Context::new();
        let req = zmq_req(&ctx).unwrap();
        assert_eq!(req.get_socket_type(), Ok(zmq::REQ) );
    }

    // With these tests, we are making sure that we can properly parse
    // the urls that our crate uses
    #[test]
    fn decode_inproc_socket () {
        let socket_addr = "inproc:/tmp/hello";
        let parsed: Url = socket_addr.parse().unwrap();
        assert_eq!(parsed.scheme(), "inproc");
        assert_eq!(parsed.host_str(), None);
    }

    #[test]
    fn decode_ipc_socket () {
        let socket_addr = "ipc:/tmp/hello";
        let parsed: Url = socket_addr.parse().unwrap();
        assert_eq!(parsed.scheme(), "ipc");
        assert_eq!(parsed.host_str(), None);
    }

    #[test]
    fn decode_generic_tcp_socket () {
        let socket_addr = "tcp://*:5566";
        let parsed: Url = socket_addr.parse().unwrap();
        assert_eq!(parsed.scheme(), "tcp");
        assert_eq!(parsed.port(), Some (5566) );
        assert_eq!(parsed.host_str(), Some ("*") );
    }
}
