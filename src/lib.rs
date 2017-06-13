/// Commands for RTD EZO Chip, taken from their Datasheet.
enum RTD_EZO_COMMANDS {
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
    Lon,
    Loff,
    Lstate,
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
