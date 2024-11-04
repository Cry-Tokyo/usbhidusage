#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[repr(u16)]
pub enum ArcadeUsage {
    #[default]
    Undefined,
    GeneralPurposeIOCard,
    CoinDoor,
    WatchdogTimer,
    Reserved04_2F(u16),
    GeneralPurposeAnalogInputState,
    GeneralPurposeDigitalInputState,
    GeneralPurposeOpticalInputState,
    GeneralPurposeDigitalOutputState,
    NumberofCoinDoors,
    CoinDrawerDropCount,
    CoinDrawerStart,
    CoinDrawerService,
    CoinDrawerTilt,
    CoinDoorTest,
    Reserved3A_3F(u16),
    CoinDoorLockout,
    WatchdogTimeout,
    WatchdogAction,
    WatchdogReboot,
    WatchdogRestart,
    AlarmInput,
    CoinDoorCounter,
    IODirectionMapping,
    SetIODirectionMapping,
    ExtendedOpticalInputState,
    PinPadInputState,
    PinPadStatus,
    PinPadOutput,
    PinPadCommand,
    Reserved4E_FFFF(u16),
}
impl<T> From<T> for ArcadeUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value: u16 = value.try_into().unwrap_or(0);

        match value {
            0 => Self::Undefined,
            1 => Self::GeneralPurposeIOCard,
            2 => Self::CoinDoor,
            3 => Self::WatchdogTimer,
            4..48 => Self::Reserved04_2F(value),
            48 => Self::GeneralPurposeAnalogInputState,
            49 => Self::GeneralPurposeDigitalInputState,
            50 => Self::GeneralPurposeOpticalInputState,
            51 => Self::GeneralPurposeDigitalOutputState,
            52 => Self::NumberofCoinDoors,
            53 => Self::CoinDrawerDropCount,
            54 => Self::CoinDrawerStart,
            55 => Self::CoinDrawerService,
            56 => Self::CoinDrawerTilt,
            57 => Self::CoinDoorTest,
            58..64 => Self::Reserved3A_3F(value),
            64 => Self::CoinDoorLockout,
            65 => Self::WatchdogTimeout,
            66 => Self::WatchdogAction,
            67 => Self::WatchdogReboot,
            68 => Self::WatchdogRestart,
            69 => Self::AlarmInput,
            70 => Self::CoinDoorCounter,
            71 => Self::IODirectionMapping,
            72 => Self::SetIODirectionMapping,
            73 => Self::ExtendedOpticalInputState,
            74 => Self::PinPadInputState,
            75 => Self::PinPadStatus,
            76 => Self::PinPadOutput,
            77 => Self::PinPadCommand,
            78..=65535 => Self::Reserved4E_FFFF(value),
        }
    }
}
