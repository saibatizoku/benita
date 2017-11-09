//! Server for pH sensing.
use super::replies::*;

use api::ph::PhAPI;
use errors::*;
use devices::ph::PhSensor;
use network::common::{Endpoint, ReplyStatus};

use neuras;


// Define the network socket for directly interacting with the
// pH sensor via I2C.
network_sensor_socket! {
    PhResponder,
    PhSensor,
    "Socket that responds to pH sensor commands."
}

impl PhAPI for PhResponder {
    type DefaultReply = ReplyStatus;

    sensor_socket_commands!(device_common);

    sensor_socket_commands!(calibration_common);

    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_high(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_low(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration mid-point for the sensor.
    fn set_calibration_mid(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_mid(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    sensor_socket_commands!(temperature_compensation);

    /// Get the current slope for the pH sensor.
    fn get_slope(&self) -> Result<ProbeSlope> {
        let response = self.sensor
            .get_slope()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }
}
