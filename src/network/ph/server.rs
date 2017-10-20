//! Server for pH sensing.
use errors::*;
use devices::ph::PhSensor;

use neuras;


// Define the network server socket for directly interacting with the
// pH sensor via I2C.
network_socket! {
    PhSensorServer,
    PhSensor,
    "Socket that responds to pH sensor commands."
}

impl PhSensorServer {
    sensor_socket_commands!(device_common);
}

impl PhSensorServer {
    sensor_socket_commands!(calibration_common);

    /// Set the calibration high-point for the sensor.
    pub fn set_calibration_high(&mut self, c: f64) -> Result<String> {
        let _response = self.sensor
            .set_calibration_high(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("calibration-high {}", c))
    }

    /// Set the calibration low-point for the sensor.
    pub fn set_calibration_low(&mut self, c: f64) -> Result<String> {
        let _response = self.sensor
            .set_calibration_low(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("calibration-low {}", c))
    }

    /// Set the calibration mid-point for the sensor.
    pub fn set_calibration_mid(&mut self, c: f64) -> Result<String> {
        let _response = self.sensor
            .set_calibration_mid(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("calibration-mid {}", c))
    }
}

impl PhSensorServer {
    sensor_socket_commands!(temperature_compensation);
}

impl PhSensorServer {
    /// Get the current slope for the pH sensor.
    pub fn get_slope(&mut self) -> Result<String> {
        let response = self.sensor
            .get_slope()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(format!("slope {}", response))
    }
}
