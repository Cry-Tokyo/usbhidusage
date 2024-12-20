#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum VesaVirtualControlsUsage {
    #[default]
    Undefined,
    Degauss,
    Reserved02_0F(u16),
    Brightness = 0x10,
    Reserved11,
    Contrast,
    Reserved13_15(u16),
    RedVideoGain = 0x16,
    Reserved17,
    GreenVideoGain,
    Reserved19,
    BlueVideoGain,
    Reserved1B,
    Focus,
    Reserved1D_1F(u16),
    HorizontalPosition = 0x20,
    Reserved21,
    HorizontalSize,
    Reserved23,
    HorizontalPincushion,
    Reserved25,
    HorizontalPincushionBalance,
    Reserved27,
    HorizontalMisconvergence,
    Reserved29,
    HorizontalLinearity,
    Reserved2B,
    HorizontalLinearityBalance,
    Reserved2D_2F(u16),
    VerticalPosition = 0x30,
    Reserved31,
    VerticalSize,
    Reserved33,
    VerticalPincushion,
    Reserved35,
    VerticalPincushionBalance,
    Reserved37,
    VerticalMisconvergence,
    Reserved39,
    VerticalLinearity,
    Reserved3B,
    VerticalLinearityBalance,
    Reserved3D_3F(u16),
    ParallelogramDistortion_KeyBalance = 0x40,
    Reserved41,
    TrapezoidalDistortion_Key,
    Reserved43,
    Tilt_Rotation,
    Reserved45,
    TopCornerDistortionControl,
    Reserved47,
    TopCornerDistortionBalance,
    Reserved49,
    BottomCornerDistortionControl,
    Reserved4B,
    BottomCornerDistortionBalance,
    Reserved4D_55(u16),
    HorizontalMoiré = 0x56,
    Reserved57,
    VerticalMoiré,
    Reserved59_5D(u16),
    InputLevelSelect = 0x5E,
    Reserved5F,
    InputSourceSelect,
    Reserved61_6F(u16),
    RedVideoBlackLevel = 0x6C,
    Reserved6D,
    GreenVideoBlackLevel,
    Reserved6F,
    BlueVideoBlackLevel,
    Reserved71_A1(u16),
    AutoSizeCenter = 0xA2,
    ReservedA3,
    PolarityHorizontalSynchronization,
    ReservedA5,
    PolarityVerticalSynchronization,
    ReservedA7,
    SynchronizationType,
    ReservedA9,
    ScreenOrientation,
    ReservedAB,
    HorizontalFrequency,
    ReservedAD,
    VerticalFrequency,
    ReservedAF,
    Settings,
    ReservedB1_C9(u16),
    OnScreenDisplay = 0xCA,
    ReservedCB_D3(u16),
    StereoMode = 0xD4,
    ReservedD5_FFFF(u16),
}
impl<T> From<T> for VesaVirtualControlsUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::Degauss,
            2..16 => Self::Reserved02_0F(value),
            16 => Self::Brightness,
            17 => Self::Reserved11,
            18 => Self::Contrast,
            19..22 => Self::Reserved13_15(value),
            22 => Self::RedVideoGain,
            23 => Self::Reserved17,
            24 => Self::GreenVideoGain,
            25 => Self::Reserved19,
            26 => Self::BlueVideoGain,
            27 => Self::Reserved1B,
            28 => Self::Focus,
            29..32 => Self::Reserved1D_1F(value),
            32 => Self::HorizontalPosition,
            33 => Self::Reserved21,
            34 => Self::HorizontalSize,
            35 => Self::Reserved23,
            36 => Self::HorizontalPincushion,
            37 => Self::Reserved25,
            38 => Self::HorizontalPincushionBalance,
            39 => Self::Reserved27,
            40 => Self::HorizontalMisconvergence,
            41 => Self::Reserved29,
            42 => Self::HorizontalLinearity,
            43 => Self::Reserved2B,
            44 => Self::HorizontalLinearityBalance,
            45..48 => Self::Reserved2D_2F(value),
            48 => Self::VerticalPosition,
            49 => Self::Reserved31,
            50 => Self::VerticalSize,
            51 => Self::Reserved33,
            52 => Self::VerticalPincushion,
            53 => Self::Reserved35,
            54 => Self::VerticalPincushionBalance,
            55 => Self::Reserved37,
            56 => Self::VerticalMisconvergence,
            57 => Self::Reserved39,
            58 => Self::VerticalLinearity,
            59 => Self::Reserved3B,
            60 => Self::VerticalLinearityBalance,
            61..64 => Self::Reserved3D_3F(value),
            64 => Self::ParallelogramDistortion_KeyBalance,
            65 => Self::Reserved41,
            66 => Self::TrapezoidalDistortion_Key,
            67 => Self::Reserved43,
            68 => Self::Tilt_Rotation,
            69 => Self::Reserved45,
            70 => Self::TopCornerDistortionControl,
            71 => Self::Reserved47,
            72 => Self::TopCornerDistortionBalance,
            73 => Self::Reserved49,
            74 => Self::BottomCornerDistortionControl,
            75 => Self::Reserved4B,
            76 => Self::BottomCornerDistortionBalance,
            77..86 => Self::Reserved4D_55(value),
            86 => Self::HorizontalMoiré,
            87 => Self::Reserved57,
            88 => Self::VerticalMoiré,
            89..94 => Self::Reserved59_5D(value),
            94 => Self::InputLevelSelect,
            95 => Self::Reserved5F,
            96 => Self::InputSourceSelect,
            97..108 => Self::Reserved61_6F(value),
            108 => Self::RedVideoBlackLevel,
            109 => Self::Reserved6D,
            110 => Self::GreenVideoBlackLevel,
            111 => Self::Reserved6F,
            112 => Self::BlueVideoBlackLevel,
            113..162 => Self::Reserved71_A1(value),
            162 => Self::AutoSizeCenter,
            163 => Self::ReservedA3,
            164 => Self::PolarityHorizontalSynchronization,
            165 => Self::ReservedA5,
            166 => Self::PolarityVerticalSynchronization,
            167 => Self::ReservedA7,
            168 => Self::SynchronizationType,
            169 => Self::ReservedA9,
            170 => Self::ScreenOrientation,
            171 => Self::ReservedAB,
            172 => Self::HorizontalFrequency,
            173 => Self::ReservedAD,
            174 => Self::VerticalFrequency,
            175 => Self::ReservedAF,
            176 => Self::Settings,
            177..202 => Self::ReservedB1_C9(value),
            202 => Self::OnScreenDisplay,
            203..212 => Self::ReservedCB_D3(value),
            212 => Self::StereoMode,
            213..=65535 => Self::ReservedD5_FFFF(value),
        }
    }
}
