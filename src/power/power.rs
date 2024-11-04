#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum PowerUsage {
    #[default]
    Undefined,
    iName,
    PresentStatus,
    ChangedStatus,
    UPS,
    PowerSupply,
    Reserved06_0F(u16),
    BatterySystem = 0x10,
    BatterySystemId,
    Battery,
    BatteryId,
    Charger,
    ChargerId,
    PowerConverter,
    PowerConverterId,
    OutletSystem,
    OutletSystemId,
    Input,
    InputId,
    Output,
    OutputId,
    Flow,
    FlowId,
    Outlet,
    OutletId,
    Gang,
    GangId,
    PowerSummary,
    PowerSummaryId,
    Reserved26_2F(u16),
    Voltage = 0x30,
    Current,
    Frequency,
    ApparentPower,
    ActivePower,
    PercentLoad,
    Temperature,
    Humidity,
    BadCount,
    Reserved39_3F(u16),
    ConfigVoltage = 0x40,
    ConfigCurrent,
    ConfigFrequency,
    ConfigApparentPower,
    ConfigActivePower,
    ConfigPercentLoad,
    ConfigTemperature,
    ConfigHumidity,
    Reserved48_4F(u16),
    SwitchOnControl = 0x50,
    SwitchOffControl,
    ToggleControl,
    LowVoltageTransfer,
    HighVoltageTransfer,
    DelayBeforeReboot,
    DelayBeforeStartup,
    DelayBeforeShutdown,
    Test,
    ModuleReset,
    AudibleAlarmControl,
    Reserved5B_5F(u16),
    Present = 0x60,
    Good,
    InternalFailure,
    VoltagOutOfRange,
    FrequencyOutOfRange,
    Overload,
    OverCharged,
    OverTemperature,
    ShutdownRequested,
    ShutdownImminent,
    Reserved6A,
    SwitchOn_Off,
    Switchable,
    Used,
    Boost,
    Buck,
    Initialized,
    Tested,
    AwaitingPower,
    CommunicationLost,
    Reserved74_FC(u16),
    iManufacturer = 0xFD,
    iProduct,
    iSerialNumber,
    Reserved100_FFFF(u16),
}
impl<T> From<T> for PowerUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::iName,
            2 => Self::PresentStatus,
            3 => Self::ChangedStatus,
            4 => Self::UPS,
            5 => Self::PowerSupply,
            6..16 => Self::Reserved06_0F(value),
            16 => Self::BatterySystem,
            17 => Self::BatterySystemId,
            18 => Self::Battery,
            19 => Self::BatteryId,
            20 => Self::Charger,
            21 => Self::ChargerId,
            22 => Self::PowerConverter,
            23 => Self::PowerConverterId,
            24 => Self::OutletSystem,
            25 => Self::OutletSystemId,
            26 => Self::Input,
            27 => Self::InputId,
            28 => Self::Output,
            29 => Self::OutputId,
            30 => Self::Flow,
            31 => Self::FlowId,
            32 => Self::Outlet,
            33 => Self::OutletId,
            34 => Self::Gang,
            35 => Self::GangId,
            36 => Self::PowerSummary,
            37 => Self::PowerSummaryId,
            38..48 => Self::Reserved26_2F(value),
            48 => Self::Voltage,
            49 => Self::Current,
            50 => Self::Frequency,
            51 => Self::ApparentPower,
            52 => Self::ActivePower,
            53 => Self::PercentLoad,
            54 => Self::Temperature,
            55 => Self::Humidity,
            56 => Self::BadCount,
            57..64 => Self::Reserved39_3F(value),
            64 => Self::ConfigVoltage,
            65 => Self::ConfigCurrent,
            66 => Self::ConfigFrequency,
            67 => Self::ConfigApparentPower,
            68 => Self::ConfigActivePower,
            69 => Self::ConfigPercentLoad,
            70 => Self::ConfigTemperature,
            71 => Self::ConfigHumidity,
            72..80 => Self::Reserved48_4F(value),
            80 => Self::SwitchOnControl,
            81 => Self::SwitchOffControl,
            82 => Self::ToggleControl,
            83 => Self::LowVoltageTransfer,
            84 => Self::HighVoltageTransfer,
            85 => Self::DelayBeforeReboot,
            86 => Self::DelayBeforeStartup,
            87 => Self::DelayBeforeShutdown,
            88 => Self::Test,
            89 => Self::ModuleReset,
            90 => Self::AudibleAlarmControl,
            91..96 => Self::Reserved5B_5F(value),
            96 => Self::Present,
            97 => Self::Good,
            98 => Self::InternalFailure,
            99 => Self::VoltagOutOfRange,
            100 => Self::FrequencyOutOfRange,
            101 => Self::Overload,
            102 => Self::OverCharged,
            103 => Self::OverTemperature,
            104 => Self::ShutdownRequested,
            105 => Self::ShutdownImminent,
            106 => Self::Reserved6A,
            107 => Self::SwitchOn_Off,
            108 => Self::Switchable,
            109 => Self::Used,
            110 => Self::Boost,
            111 => Self::Buck,
            112 => Self::Initialized,
            113 => Self::Tested,
            114 => Self::AwaitingPower,
            115 => Self::CommunicationLost,
            116..253 => Self::Reserved74_FC(value),
            253 => Self::iManufacturer,
            254 => Self::iProduct,
            255 => Self::iSerialNumber,
            256..=65535 => Self::Reserved100_FFFF(value),
        }
    }
}
