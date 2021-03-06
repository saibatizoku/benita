//! Server for Temperature sensing.
use super::super::device::TemperatureSensor;
use super::super::TemperatureAPI;
use super::replies::*;

use common_ezo::EzoChipAPI;
use errors::*;
use network::{Endpoint, ReplyStatus};

use zmq::Socket;

// Define the network socket for directly interacting with the
// Temperature sensor via I2C.
network_sensor_socket! {
    TemperatureResponder,
    TemperatureSensor,
    "Socket that responds to Temperature sensor commands."
}

impl EzoChipAPI for TemperatureResponder {
    type SensorError = Error;
    type SensorReply = ReplyStatus;

    sensor_socket_commands!(device_common);
    sensor_socket_commands!(calibration_common);
}

impl TemperatureAPI for TemperatureResponder {
    type Error = Error;
    type DefaultReply = ReplyStatus;

    sensor_socket_commands!(calibration_status);
    sensor_socket_commands!(reading);

    /// Set the calibration mid-point for the sensor.
    fn set_calibration_temperature(&self, c: f64) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_calibration_temperature(c)
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set the data-logger interval.
    fn set_data_logger_interval(&self, c: u32) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_data_logger_interval(c)
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// get the current data-logger status.
    fn get_data_logger_status(&self) -> Result<DataLoggerStorageIntervalSeconds> {
        let response = self
            .sensor
            .get_data_logger_status()
            .context(ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// set data-logger off.
    fn set_data_logger_off(&self) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_data_logger_off()
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// clear memory readings.
    fn set_memory_clear(&self) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_memory_clear()
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// recall the next reading in the memory stack.
    fn get_memory_recall(&self) -> Result<MemoryReading> {
        let response = self
            .sensor
            .get_memory_recall()
            .context(ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// recall the last reading and position in the memory stack.
    fn get_memory_recall_last(&self) -> Result<MemoryReading> {
        let response = self
            .sensor
            .get_memory_recall_last()
            .context(ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// set scale to Celsius.
    fn set_scale_to_celsius(&self) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_scale_to_celsius()
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set scale to Fahrenheit.
    fn set_scale_to_fahrenheit(&self) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_scale_to_fahrenheit()
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// set scale to Kelvin.
    fn set_scale_to_kelvin(&self) -> Result<ReplyStatus> {
        let _response = self
            .sensor
            .set_scale_to_kelvin()
            .context(ErrorKind::CommandRequest)?;
        Ok(ReplyStatus::Ok)
    }

    /// get current scale.
    fn get_scale(&self) -> Result<TemperatureScale> {
        let response = self.sensor.get_scale().context(ErrorKind::CommandRequest)?;
        Ok(response)
    }
}
