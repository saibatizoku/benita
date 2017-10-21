/// Create and define a device network socket compatible with the `benita` network.
#[macro_export]
macro_rules! device_socket {
    // Name identifier and documentation for the new network socket struct.
    ($name:ident ,
     $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            socket: neuras::zmq::Socket,
        }

        impl $name {
            /// Create a new network socket.
            pub fn new(socket: neuras::zmq::Socket) -> Result<$name> {
                Ok( $name { socket } )
            }
        }

        endpoint_trait_impl!($name);
    };
    // Simple sensor socket.
    ($name:ident ,
     $sensor:ident ,
     $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            socket: neuras::zmq::Socket,
            sensor: $sensor,
        }

        impl $name {
            /// Create a new network socket.
            pub fn new(socket: neuras::zmq::Socket, sensor: $sensor) -> Result<$name> {
                Ok( $name { socket, sensor } )
            }
        }

        endpoint_trait_impl!($name);
    };
}

macro_rules! endpoint_trait_impl {
    ($name:ident) => {
        impl Endpoint for $name {
            /// Binds the socket to the given URL.
            fn bind(&self, url: &str) -> Result<()> {
                let _bind = neuras::utils::bind_socket(&self.socket, url)
                    .chain_err(|| ErrorKind::SocketBind)?;
                Ok(())
            }

            /// Connects the socket to the given URL.
            fn connect(&self, url: &str) -> Result<()> {
                let _connect = neuras::utils::connect_socket(&self.socket, url)
                    .chain_err(|| ErrorKind::SocketConnect)?;
                Ok(())
            }

            /// Sends a message over the network socket.
            fn send(&self, msg: &[u8]) -> Result<()> {
                let _send = self.socket.send(msg, 0)
                    .chain_err(|| ErrorKind::SocketSend)?;
                Ok(())
            }

            /// Receives a message from the network socket.
            fn recv(&self) -> Result<String> {
                let received_result = self.socket.recv_string(0)
                    .chain_err(|| ErrorKind::SocketReceive)?;
                // We match against the resulting `Resulṭ<String, Vec<u8>>`
                let response = match received_result {
                    Ok(r) => r,
                    _ => return Err(ErrorKind::ResponseParse.into()),
                };
                Ok(response)
            }
        }
    };
}

// Define a command sent over a network socket.
macro_rules! socket_command {
    // Simple socket to send commands over.
    ( $name:ident , $trait:ty ,
      $socket:ty ,
      response: $response:ty ,
      $resp:ident : $runfn:block,
      $doc:tt ) => {
        #[ doc = $doc ]
        pub struct $name;

        impl $trait for $name {
            type Socket = $socket;
            type Response = $response;

            fn run(&self, socket: &$socket) -> Result<$response> {
                let $resp = socket;
                $runfn
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use errors::*;

    use neuras;
    use neuras::utils::{create_context, zmq_req};

    #[allow(unused)]
    #[test]
    fn macro_creates_a_device_network_socket() {
        device_socket!(NewSocket, "NewSocket docs.");

        let context = create_context();
        let requester = zmq_req(&context).unwrap();
        let socket = NewSocket::new(requester);
        assert!(socket.is_ok());
    }
}
