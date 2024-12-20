#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[repr(u16)]
pub enum BatterySystemUsage {
    #[default]
    Undefined,
    SmartBatteryBatteryMode,
    SmartBatteryBatteryStatus,
    SmartBatteryAlarmWarning,
    SmartBatteryChargerMode,
    SmartBatteryChargerStatus,
    SmartBatteryChargerSpecInfo,
    SmartBatterySelectorState,
    SmartBatterySelectorPresets,
    SmartBatterySelectorInfo,
    Reserved0A_0F(u16),
    OptionalMfgFunction1 = 0x10,
    OptionalMfgFunction2,
    OptionalMfgFunction3,
    OptionalMfgFunction4,
    OptionalMfgFunction5,
    ConnectionToSMBus,
    OutputConnection,
    ChargerConnection,
    BatteryInsertion,
    UseNext,
    OKToUse,
    BatterySupported,
    SelectorRevision,
    ChargingIndicator,
    Reserved1E_27(u16),
    ManufacturerAccess = 0x28,
    RemainingCapacityLimit,
    RemainingTimeLimit,
    AtRate,
    CapacityMode,
    BroadcastToCharger,
    PrimaryBattery,
    ChargeController,
    Reserved30_3F(u16),
    TerminateCharge = 0x40,
    TerminateDischarge,
    BelowRemainingCapacityLimit,
    RemainingTimeLimitExpired,
    Charging,
    Discharging,
    FullyCharged,
    FullyDischarged,
    ConditioningFlag,
    AtRateOK,
    SmartBatteryErrorCode,
    NeedReplacement,
    Reserved4C_5F(u16),
    AtRateTimeToFull = 0x60,
    AtRateTimeToEmpty,
    AverageCurrent,
    MaxError,
    RelativeStateOfCharge,
    AbsoluteStateOfCharge,
    RemainingCapacity,
    FullChargeCapacity,
    RunTimeToEmpty,
    AverageTimeToEmpty,
    AverageTimeToFull,
    CycleCount,
    Reserved6C_7F(u16),
    BatteryPackModelLevel = 0x80,
    InternalChargeController,
    PrimaryBatterySupport,
    DesignCapacity,
    SpecificationInfo,
    ManufactureDate,
    SerialNumber,
    iManufacturerName,
    iDeviceName,
    iDeviceChemistry,
    ManufacturerData,
    Rechargable,
    WarningCapacityLimit,
    CapacityGranularity1,
    CapacityGranularity2,
    iOEMInformation,
    Reserved90_BF(u16),
    InhibitCharge = 0xC0,
    EnablePolling,
    ResetToZero,
    ReservedC3_CF(u16),
    ACPresent = 0xD0,
    BatteryPresent,
    PowerFail,
    AlarmInhibited,
    ThermistorUnderRange,
    ThermistorHot,
    ThermistorCold,
    ThermistorOverRange,
    VoltageOutOfRange,
    CurrentOutOfRange,
    CurrentNotRegulated,
    VoltageNotRegulated,
    MasterMode,
    ReservedDD_EF(u16),
    ChargerSelectorSupport = 0xF0,
    ChargerSpec,
    Level2,
    Level3,
    ReservedF4_FFFF(u16),
}
impl<T> From<T> for BatterySystemUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value: u16 = value.try_into().unwrap_or(0);

        match value {
            0 => Self::Undefined,
            1 => Self::SmartBatteryBatteryMode,
            2 => Self::SmartBatteryBatteryStatus,
            3 => Self::SmartBatteryAlarmWarning,
            4 => Self::SmartBatteryChargerMode,
            5 => Self::SmartBatteryChargerStatus,
            6 => Self::SmartBatteryChargerSpecInfo,
            7 => Self::SmartBatterySelectorState,
            8 => Self::SmartBatterySelectorPresets,
            9 => Self::SmartBatterySelectorInfo,
            10..16 => Self::Reserved0A_0F(value),
            16 => Self::OptionalMfgFunction1,
            17 => Self::OptionalMfgFunction2,
            18 => Self::OptionalMfgFunction3,
            19 => Self::OptionalMfgFunction4,
            20 => Self::OptionalMfgFunction5,
            21 => Self::ConnectionToSMBus,
            22 => Self::OutputConnection,
            23 => Self::ChargerConnection,
            24 => Self::BatteryInsertion,
            25 => Self::UseNext,
            26 => Self::OKToUse,
            27 => Self::BatterySupported,
            28 => Self::SelectorRevision,
            29 => Self::ChargingIndicator,
            30..40 => Self::Reserved1E_27(value),
            40 => Self::ManufacturerAccess,
            41 => Self::RemainingCapacityLimit,
            42 => Self::RemainingTimeLimit,
            43 => Self::AtRate,
            44 => Self::CapacityMode,
            45 => Self::BroadcastToCharger,
            46 => Self::PrimaryBattery,
            47 => Self::ChargeController,
            48..64 => Self::Reserved30_3F(value),
            64 => Self::TerminateCharge,
            65 => Self::TerminateDischarge,
            66 => Self::BelowRemainingCapacityLimit,
            67 => Self::RemainingTimeLimitExpired,
            68 => Self::Charging,
            69 => Self::Discharging,
            70 => Self::FullyCharged,
            71 => Self::FullyDischarged,
            72 => Self::ConditioningFlag,
            73 => Self::AtRateOK,
            74 => Self::SmartBatteryErrorCode,
            75 => Self::NeedReplacement,
            76..96 => Self::Reserved4C_5F(value),
            96 => Self::AtRateTimeToFull,
            97 => Self::AtRateTimeToEmpty,
            98 => Self::AverageCurrent,
            99 => Self::MaxError,
            100 => Self::RelativeStateOfCharge,
            101 => Self::AbsoluteStateOfCharge,
            102 => Self::RemainingCapacity,
            103 => Self::FullChargeCapacity,
            104 => Self::RunTimeToEmpty,
            105 => Self::AverageTimeToEmpty,
            106 => Self::AverageTimeToFull,
            107 => Self::CycleCount,
            108..128 => Self::Reserved6C_7F(value),
            128 => Self::BatteryPackModelLevel,
            129 => Self::InternalChargeController,
            130 => Self::PrimaryBatterySupport,
            131 => Self::DesignCapacity,
            132 => Self::SpecificationInfo,
            133 => Self::ManufactureDate,
            134 => Self::SerialNumber,
            135 => Self::iManufacturerName,
            136 => Self::iDeviceName,
            137 => Self::iDeviceChemistry,
            138 => Self::ManufacturerData,
            139 => Self::Rechargable,
            140 => Self::WarningCapacityLimit,
            141 => Self::CapacityGranularity1,
            142 => Self::CapacityGranularity2,
            143 => Self::iOEMInformation,
            144..192 => Self::Reserved90_BF(value),
            192 => Self::InhibitCharge,
            193 => Self::EnablePolling,
            194 => Self::ResetToZero,
            195..208 => Self::ReservedC3_CF(value),
            208 => Self::ACPresent,
            209 => Self::BatteryPresent,
            210 => Self::PowerFail,
            211 => Self::AlarmInhibited,
            212 => Self::ThermistorUnderRange,
            213 => Self::ThermistorHot,
            214 => Self::ThermistorCold,
            215 => Self::ThermistorOverRange,
            216 => Self::VoltageOutOfRange,
            217 => Self::CurrentOutOfRange,
            218 => Self::CurrentNotRegulated,
            219 => Self::VoltageNotRegulated,
            220 => Self::MasterMode,
            221..240 => Self::ReservedDD_EF(value),
            240 => Self::ChargerSelectorSupport,
            241 => Self::ChargerSpec,
            242 => Self::Level2,
            243 => Self::Level3,
            244..=65535 => Self::ReservedF4_FFFF(value),
        }
    }
}
