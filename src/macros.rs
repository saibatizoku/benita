//! Exported macros.
//!
//! *   sensor_i2cdev!
//! *   network_socket!

/// Create and define a sensor available through `i2cdev`.
#[macro_export]
macro_rules! sensor_i2cdev {
    // Name identifier and documentation for the new I2C sensor struct.
    ($name:ident , $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            i2cdev: LinuxI2CDevice,
        }

        impl $name {
            /// Creates a new handle for the I2C Sensor connected
            /// at the designated path and address.
            ///
            /// This device uses a file-descriptor through `i2cdev`. To use it, the path
            /// to the I2C bus, and the `u16` address location, are needed.
            pub fn new(i2c_path: &str, i2c_address: u16) -> Result<$name> {
                let i2cdev = LinuxI2CDevice::new(i2c_path, i2c_address)
                    .chain_err(|| ErrorKind::SensorTrouble)?;
                Ok( $name { i2cdev: i2cdev } )
            }
        }
    };
}

/// Create and define a network socket compatible with the `benita` network.
#[macro_export]
macro_rules! network_socket {
    // Name identifier and documentation for the new network socket struct.
    ($name:ident , $doc:tt) => {
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

        network_socket_impl!($name);
    };
    ($name:ident , $sensor:ident , $doc:tt) => {
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

        network_socket_impl!($name);
    };
}

macro_rules! network_socket_impl {
    ($name:ident) => {
        impl $name {
            /// Binds the socket to the given URL.
            pub fn bind(&self, url: &str) -> Result<()> {
                let _bind = neuras::utils::bind_socket(&self.socket, url)
                    .chain_err(|| ErrorKind::SocketBind)?;
                Ok(())
            }

            /// Connects the socket to the given URL.
            pub fn connect(&self, url: &str) -> Result<()> {
                let _connect = neuras::utils::connect_socket(&self.socket, url)
                    .chain_err(|| ErrorKind::SocketConnect)?;
                Ok(())
            }

            /// Sends a message over the network socket.
            pub fn send(&self, msg: &[u8]) -> Result<()> {
                let _send = self.socket.send(msg, 0)
                    .chain_err(|| ErrorKind::SocketSend)?;
                Ok(())
            }

            /// Receives a message from the network socket.
            pub fn recv(&self) -> Result<String> {
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

/// Define a command sent over a network socket.
#[macro_export]
macro_rules! socket_command {
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

            fn run(&self, socket: &mut $socket) -> Result<$response> {
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
    fn macro_creates_a_network_socket() {
        network_socket!(NewSocket, "NewSocket docs.");

        let context = create_context();
        let requester = zmq_req(&context).unwrap();
        let socket = NewSocket::new(requester);
        assert!(socket.is_ok());
    }
}
