//! Server for pH sensing.
use super::replies::*;
use super::super::PhAPI;
use super::super::device::PhSensor;

use common_ezo::EzoChipAPI;
use errors::*;
use network::{Endpoint, ReplyStatus};

use zmq::Socket;


// Define the network socket for directly interacting with the
// pH sensor via I2C.
network_sensor_socket! {
    PhResponder,
    PhSensor,
    "Socket that responds to pH sensor commands."
}

impl EzoChipAPI for PhResponder {
    type SensorError = Error;
    type SensorReply = ReplyStatus;

    sensor_socket_commands!(device_common);
    sensor_socket_commands!(calibration_common);
}

impl PhAPI for PhResponder {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_socket_commands!(calibration_status);
    sensor_socket_commands!(reading);

    /// Set the calibration high-point for the sensor.
    fn set_calibration_high(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_high(c)
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration low-point for the sensor.
    fn set_calibration_low(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_low(c)
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// Set the calibration mid-point for the sensor.
    fn set_calibration_mid(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self.sensor
            .set_calibration_mid(c)
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    sensor_socket_commands!(temperature_compensation);

    /// Get the current slope for the pH sensor.
    fn get_slope(&self) -> Result<ProbeSlope> {
        let response = self.sensor
            .get_slope()
            .context(ErrorKind::CommandRequest)?;
        Ok(response)
    }
}
