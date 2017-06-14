/// I2C Commands for EC EZO Chip, taken from their Datasheet.
/// This chip is used for electrical conductivity measurement. It features
/// calibration, sleep mode, scale, etc.
enum EcEzoCommands {
    Baud(u16),
    CalibrationDry,
    CalibrationSinglePoint(f64),
    CalibrationLowEnd(u16),
    CalibrationHighEnd(u16),
    CalibrationClear,
    CalibrationState,
    DataloggerPeriod(u8),
    DataloggerDisable,
    DataloggerInterval,
    DeviceAddress(u8),
    DeviceInformation,
    Export,
    ExportInfo,
    Import(String),
    Factory,
    ProbeTypePointOne,
    ProbeTypeOne,
    ProbeTypeTen,
    ProbeTypeStatus,
    Find,
    LedOn,
    LedOff,
    LedState,
    OutputDisableConductivity,
    OutputEnableConductivity,
    OutputDisableTds,
    OutputEnableTds,
    OutputDisableSalinity,
    OutputEnableSalinity,
    OutputDisableSpecificGravity,
    OutputEnableSpecificGravity,
    OutputStatus,
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockStatus,
    Reading,
    Sleep,
    Status,
    TemperatureCompensation,
}
