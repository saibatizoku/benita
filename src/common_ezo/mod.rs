//! Common items for EZO-based sensors
mod api {
    use super::response::*;

    /// Trait for EZO chips and the basic API
    pub trait EzoChipAPI {
        type SensorError;
        type SensorReply;

        /// Clear the sensor's calibration settings.
        fn set_calibration_clear(
            &self,
        ) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// change the sensor's I2C address.
        fn set_device_address(
            &self,
            address: u16,
        ) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// get the sensor information.
        fn get_device_info(&self) -> ::std::result::Result<DeviceInfo, Self::SensorError>;
        /// get the sensor status.
        fn get_device_status(&self) -> ::std::result::Result<DeviceStatus, Self::SensorError>;
        /// get the export information from the sensor.
        fn get_export_info(&self) -> ::std::result::Result<ExportedInfo, Self::SensorError>;
        /// export a calibration line from the sensor.
        fn get_export_line(&self) -> ::std::result::Result<Exported, Self::SensorError>;
        /// reset the sensor device.
        fn set_factory_reset(&self) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// set the sensor to find mode.
        fn set_find_mode(&self) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// import a calibration line to the sensor.
        fn set_import_line(
            &self,
            import: &str,
        ) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// set the LED off.
        fn set_led_off(&self) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// set the LED on.
        fn set_led_on(&self) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// get the current LED status.
        fn get_led_status(&self) -> ::std::result::Result<LedStatus, Self::SensorError>;
        /// set the protocol lock off.
        fn set_protocol_lock_off(
            &self,
        ) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// set the protocol lock on.
        fn set_protocol_lock_on(
            &self,
        ) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
        /// get the current protocol lock status.
        fn get_protocol_lock_status(
            &self,
        ) -> ::std::result::Result<ProtocolLockStatus, Self::SensorError>;
        /// set the sensor to sleep (low-power) mode.
        fn set_sleep(&self) -> ::std::result::Result<Self::SensorReply, Self::SensorError>;
    }
}

pub mod command {
    //! Shared commands for EZO sensors
    pub use ezo_common::command::{Baud, CalibrationClear, DeviceAddress, DeviceInformation,
                                  Export, ExportInfo, Factory, Find, Import, LedOff, LedOn,
                                  LedState, ProtocolLockDisable, ProtocolLockEnable,
                                  ProtocolLockState, Sleep, Status};
}

pub mod response {
    //! Shared responses for EZO sensors
    pub use ezo_common::response::{DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus,
                                   ProtocolLockStatus};
}

pub use self::api::*;
pub mod replies;
pub mod requests;
