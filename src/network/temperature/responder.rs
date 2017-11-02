//! Server for Temperature sensing.
use super::replies::*;

use errors::*;
use devices::temperature::TemperatureSensor;
use network::common::{Endpoint, OkReply};

use neuras;


// Define the network socket for directly interacting with the
// Temperature sensor via I2C.
network_sensor_socket! {
    TemperatureResponder,
    TemperatureSensor,
    "Socket that responds to Temperature sensor commands."
}

impl TemperatureResponder {
    sensor_socket_commands!(device_common);
}

impl TemperatureResponder {
    sensor_socket_commands!(calibration_common);

    /// Set the calibration mid-point for the sensor.
    pub fn set_calibration_temperature(&mut self, c: f64) -> Result<OkReply> {
        let _response = self.sensor
            .set_calibration_temperature(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
}

impl TemperatureResponder {
    /// set the data-logger interval.
    pub fn set_data_logger_interval(&mut self, c: u32) -> Result<OkReply> {
        let _response = self.sensor
            .set_data_logger_interval(c)
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// get the current data-logger status.
    pub fn get_data_logger_status(&mut self) -> Result<DataLoggerStorageIntervalSeconds> {
        let response = self.sensor
            .get_data_logger_status()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// set data-logger off.
    pub fn set_data_logger_off(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_data_logger_off()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }
}

impl TemperatureResponder {
    /// clear memory readings.
    pub fn set_memory_clear(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_memory_clear()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// recall the next reading in the memory stack.
    pub fn get_memory_recall(&mut self) -> Result<MemoryReading> {
        let response = self.sensor
            .get_memory_recall()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }

    /// recall the last reading and position in the memory stack.
    pub fn get_memory_recall_last(&mut self) -> Result<MemoryReading> {
        let response = self.sensor
            .get_memory_recall_last()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }
}

impl TemperatureResponder {
    /// set scale to Celsius.
    pub fn set_scale_to_celsius(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_scale_to_celsius()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set scale to Fahrenheit.
    pub fn set_scale_to_fahrenheit(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_scale_to_fahrenheit()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// set scale to Kelvin.
    pub fn set_scale_to_kelvin(&mut self) -> Result<OkReply> {
        let _response = self.sensor
            .set_scale_to_kelvin()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(OkReply)
    }

    /// get current scale.
    pub fn get_scale(&mut self) -> Result<TemperatureScale> {
        let response = self.sensor
            .get_scale()
            .chain_err(|| ErrorKind::CommandRequest)?;
        Ok(response)
    }
}