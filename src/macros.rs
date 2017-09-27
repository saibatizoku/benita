//! Exported macros.
//!
//! *   sensor_i2cdev!
//! *   sensor_socket!

/// Create and define a sensor available through `i2cdev`.
#[macro_export]
macro_rules! sensor_i2cdev {
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
                Ok($name { i2cdev: i2cdev })
            }
        }
    };
}

/// Create and define a network socket compatible with the `benita` network.
#[macro_export]
macro_rules! sensor_socket {
    ($name:ident , $error:ident , $doc:tt) => {
        #[ doc = $doc ]
        pub struct $name {
            socket: neuras::zmq::Socket,
            message: neuras::zmq::Message,
        }

        impl $name {
            /// Create a new network socket.
            pub fn new(socket: neuras::zmq::Socket) -> Result<$name> {
                let message = neuras::utils::create_message()?;
                Ok($name { socket, message })
            }

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
            pub fn recv(&mut self) -> Result<&neuras::zmq::Message> {
                let _recv = self.socket.recv(&mut self.message, 0)
                    .chain_err(|| ErrorKind::SocketReceive)?;
                Ok(&self.message)
            }
        }

    };
}

#[cfg(test)]
mod tests {
    use errors::*;
    use neuras;
    use neuras::utils::{bind_socket, connect_socket, create_context, zmq_req};

    #[test]
    fn macro_creates_a_sensor_socket() {
        sensor_socket!(NewSocket, SocketError, "NewSocket docs.");

        let context = create_context();
        let requester = zmq_req(&context).unwrap();
        let socket = NewSocket::new(requester);
        assert!(socket.is_ok());
    }
}
