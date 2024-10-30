#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum LedUsage {
    #[default]
    Undefined,
    NumLock,
    CapsLock,
    ScrollLock,
    Compose,
    Kana,
    Power,
    Shift,
    DoNotDisturb,
    Mute,
    ToneEnable,
    HighCutFilter,
    LowCutFilter,
    EqualizerEnable,
    SoundFieldOn,
    SurroundOn,
    Repeat,
    Stereo,
    SamplingRateDetect,
    Spinning,
    CAV,
    CLV,
    RecordingFormatDetect,
    Off_Hook,
    Ring,
    MessageWaiting,
    DataMode,
    BatteryOperation,
    BatteryOK,
    BatteryLow,
    Speaker,
    Headset,
    Hold,
    Microphone,
    Coverage,
    NightMode,
    SendCalls,
    CallPickup,
    Conference,
    Stand_by,
    CameraOn,
    CameraOff,
    On_Line,
    Off_Line,
    Busy,
    Ready,
    Paper_Out,
    Paper_Jam,
    Remote,
    Forward,
    Reverse,
    Stop,
    Rewind,
    FastForward,
    Play,
    Pause,
    Record,
    Error,
    UsageSelectedIndicator,
    UsageInUseIndicator,
    UsageMultiModeIndicator,
    IndicatorOn,
    IndicatorFlash,
    IndicatorSlowBlink,
    IndicatorFastBlink,
    IndicatorOff,
    FlashOnTime,
    SlowBlinkOnTime,
    SlowBlinkOffTime,
    FastBlinkOnTime,
    FastBlinkOffTime,
    UsageIndicatorColor,
    IndicatorRed,
    IndicatorGreen,
    IndicatorAmber,
    GenericIndicator,
    SystemSuspend,
    ExternalPowerConnected,
    IndicatorBlue,
    IndicatorOrange,
    GoodStatus,
    WarningStatus,
    RGBLED,
    RedLEDChannel,
    BlueLEDChannel,
    GreenLEDChannel,
    LEDIntensity,
    SystemMicrophoneMute,
    Reserved58_5F(u16),
    PlayerIndicator = 96,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
    Player8,
    Reserved69_FFFF(u16),
}
impl From<&u16> for LedUsage {
    fn from(value: &u16) -> Self {
        match value {
            0 => Self::Undefined,
            1 => Self::NumLock,
            2 => Self::CapsLock,
            3 => Self::ScrollLock,
            4 => Self::Compose,
            5 => Self::Kana,
            6 => Self::Power,
            7 => Self::Shift,
            8 => Self::DoNotDisturb,
            9 => Self::Mute,
            10 => Self::ToneEnable,
            11 => Self::HighCutFilter,
            12 => Self::LowCutFilter,
            13 => Self::EqualizerEnable,
            14 => Self::SoundFieldOn,
            15 => Self::SurroundOn,
            16 => Self::Repeat,
            17 => Self::Stereo,
            18 => Self::SamplingRateDetect,
            19 => Self::Spinning,
            20 => Self::CAV,
            21 => Self::CLV,
            22 => Self::RecordingFormatDetect,
            23 => Self::Off_Hook,
            24 => Self::Ring,
            25 => Self::MessageWaiting,
            26 => Self::DataMode,
            27 => Self::BatteryOperation,
            28 => Self::BatteryOK,
            29 => Self::BatteryLow,
            30 => Self::Speaker,
            31 => Self::Headset,
            32 => Self::Hold,
            33 => Self::Microphone,
            34 => Self::Coverage,
            35 => Self::NightMode,
            36 => Self::SendCalls,
            37 => Self::CallPickup,
            38 => Self::Conference,
            39 => Self::Stand_by,
            40 => Self::CameraOn,
            41 => Self::CameraOff,
            42 => Self::On_Line,
            43 => Self::Off_Line,
            44 => Self::Busy,
            45 => Self::Ready,
            46 => Self::Paper_Out,
            47 => Self::Paper_Jam,
            48 => Self::Remote,
            49 => Self::Forward,
            50 => Self::Reverse,
            51 => Self::Stop,
            52 => Self::Rewind,
            53 => Self::FastForward,
            54 => Self::Play,
            55 => Self::Pause,
            56 => Self::Record,
            57 => Self::Error,
            58 => Self::UsageSelectedIndicator,
            59 => Self::UsageInUseIndicator,
            60 => Self::UsageMultiModeIndicator,
            61 => Self::IndicatorOn,
            62 => Self::IndicatorFlash,
            63 => Self::IndicatorSlowBlink,
            64 => Self::IndicatorFastBlink,
            65 => Self::IndicatorOff,
            66 => Self::FlashOnTime,
            67 => Self::SlowBlinkOnTime,
            68 => Self::SlowBlinkOffTime,
            69 => Self::FastBlinkOnTime,
            70 => Self::FastBlinkOffTime,
            71 => Self::UsageIndicatorColor,
            72 => Self::IndicatorRed,
            73 => Self::IndicatorGreen,
            74 => Self::IndicatorAmber,
            75 => Self::GenericIndicator,
            76 => Self::SystemSuspend,
            77 => Self::ExternalPowerConnected,
            78 => Self::IndicatorBlue,
            79 => Self::IndicatorOrange,
            80 => Self::GoodStatus,
            81 => Self::WarningStatus,
            82 => Self::RGBLED,
            83 => Self::RedLEDChannel,
            84 => Self::BlueLEDChannel,
            85 => Self::GreenLEDChannel,
            86 => Self::LEDIntensity,
            87 => Self::SystemMicrophoneMute,
            88..96 => Self::Reserved58_5F(*value),
            96 => Self::PlayerIndicator,
            97 => Self::Player1,
            98 => Self::Player2,
            99 => Self::Player3,
            100 => Self::Player4,
            101 => Self::Player5,
            102 => Self::Player6,
            103 => Self::Player7,
            104 => Self::Player8,
            105..=65535 => Self::Reserved69_FFFF(*value),
        }
    }
}
