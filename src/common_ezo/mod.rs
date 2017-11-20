//! Common items for EZO-based sensors
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

pub mod replies;
pub mod requests;
