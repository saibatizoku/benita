/// Commands for RTD EZO Chip, taken from their Datasheet.
/// This chip is used for temperature measurement. It features
/// calibration, sleep mode, scale, etc.
enum RtdEzoCommands {
    Baud,
    CalibrationTemperature,
    CalibrationClear,
    CalibrationState,
    DataloggerPeriod,
    DataloggerDisable,
    DataloggerInterval,
    DeviceAddress,
    DeviceInformation,
    Export,
    ExportInfo,
    Import,
    Factory,
    Find,
    LedOn,
    LedOff,
    LedState,
    MemoryClear,
    MemoryRecall,
    MemoryRecallLastLocation,
    ProtocolLockEnable,
    ProtocolLockDisable,
    ProtocolLockStatus,
    Reading,
    ScaleCelsius,
    ScaleKelvin,
    ScaleFahrenheit,
    ScaleStatus,
    Sleep,
    Status
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
