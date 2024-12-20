#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum GenericDesktopUsage {
    #[default]
    Undefined,
    Pointer,
    Mouse,
    Reserved03,
    Joystick,
    Gamepad,
    Keyboard,
    Keypad,
    MultiAxisController,
    TabletPCSystemControls,
    WaterCoolingDevice,
    ComputerChassisDevice,
    WirelessRadioControls,
    PortableDeviceControl,
    SystemMultiAxisController,
    SpatialController,
    AssistiveControl,
    DeviceDock,
    DockableDevice,
    CallStateManagementControl,
    Reserved14_25(u16),
    X = 48,
    Y,
    Z,
    Rx,
    Ry,
    Rz,
    Slider,
    Dial,
    Wheel,
    HatSwitch,
    CountedBuffer,
    ByteCount,
    MotionWakeup,
    Start,
    Select,
    Reserved3F,
    Vx,
    Vy,
    Vz,
    Vbrx,
    Vbry,
    Vbrz,
    Vno,
    FeatureNotification,
    ResolutionMultiplier,
    Qx,
    Qy,
    Qz,
    Qw,
    Reserved4D_7F(u16),
    SystemControl = 128,
    SystemPowerDown,
    SystemSleep,
    SystemWakeUp,
    SystemContextMenu,
    SystemMainMenu,
    SystemAppMenu,
    SystemMenuHelp,
    SystemMenuExit,
    SystemMenuSelect,
    SystemMenuRight,
    SystemMenuLeft,
    SystemMenuUp,
    SystemMenuDown,
    SystemColdRestart,
    SystemWarmRestart,
    DpadUp,
    DpadDown,
    DpadRight,
    DpadLeft,
    IndexTrigger,
    PalmTrigger,
    Thumbstick,
    SystemFunctionShift,
    SystemFunctionShiftLock,
    SystemFunctionShiftLockIndicator,
    SystemDismissNotification,
    SystemDoNotDisturb,
    Reserved9C_9F(u16),
    SystemDock = 160,
    SystemUndock,
    SystemSetup,
    SystemBreak,
    SystemDebuggerBreak,
    ApplicationBreak,
    ApplicationDebuggerBreak,
    SystemSpeakerMute,
    SystemHibernate,
    SystemMicrophoneMute,
    ReservedAA_A7(u16),
    SystemDisplayInvert = 176,
    SystemDisplayInternal,
    SystemDisplayExternal,
    SystemDisplayBoth,
    SystemDisplayDual,
    SystemDisplayToggleInt_ExtMode,
    SystemDisplaySwapPrimary_Secondary,
    SystemDisplayToggleLCDAutoscale,
    ReservedB8_BF(u16),
    SensorZone = 192,
    RPM,
    CoolantLevel,
    CoolantCriticalLevel,
    CoolantPump,
    ChassisEnclosure,
    WirelessRadioButton,
    WirelessRadioLED,
    WirelessRadioSliderSwitch,
    SystemDisplayRotationLockButton,
    SystemDisplayRotationLockSliderSwitch,
    ControlEnable,
    ReservedCC_CF(u16),
    DockableDeviceUniqueID = 208,
    DockableDeviceVendorID,
    DockableDevicePrimaryUsagePage,
    DockableDevicePrimaryUsageID,
    DockableDeviceDockingState,
    DockableDeviceDisplayOcclusion,
    DockableDeviceObjectType,
    ReservedD7_DF(u16),
    CallActiveLED = 224,
    CallMuteToggle,
    CallMuteLED,
    ReservedE3_FFFF(u16),
}
impl<T> From<T> for GenericDesktopUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0u16 => Self::Undefined,
            1 => Self::Pointer,
            2 => Self::Mouse,
            3 => Self::Reserved03,
            4 => Self::Joystick,
            5 => Self::Gamepad,
            6 => Self::Keyboard,
            7 => Self::Keypad,
            8 => Self::MultiAxisController,
            9 => Self::TabletPCSystemControls,
            10 => Self::WaterCoolingDevice,
            11 => Self::ComputerChassisDevice,
            12 => Self::WirelessRadioControls,
            13 => Self::PortableDeviceControl,
            14 => Self::SystemMultiAxisController,
            15 => Self::SpatialController,
            16 => Self::AssistiveControl,
            17 => Self::DeviceDock,
            18 => Self::DockableDevice,
            19 => Self::CallStateManagementControl,
            20..48 => Self::Reserved14_25(value),
            48 => Self::X,
            49 => Self::Y,
            50 => Self::Z,
            51 => Self::Rx,
            52 => Self::Ry,
            53 => Self::Rz,
            54 => Self::Slider,
            55 => Self::Dial,
            56 => Self::Wheel,
            57 => Self::HatSwitch,
            58 => Self::CountedBuffer,
            59 => Self::ByteCount,
            60 => Self::MotionWakeup,
            61 => Self::Start,
            62 => Self::Select,
            63 => Self::Reserved3F,
            64 => Self::Vx,
            65 => Self::Vy,
            66 => Self::Vz,
            67 => Self::Vbrx,
            68 => Self::Vbry,
            69 => Self::Vbrz,
            70 => Self::Vno,
            71 => Self::FeatureNotification,
            72 => Self::ResolutionMultiplier,
            73 => Self::Qx,
            74 => Self::Qy,
            75 => Self::Qz,
            76 => Self::Qw,
            77..128 => Self::Reserved4D_7F(value),
            128 => Self::SystemControl,
            129 => Self::SystemPowerDown,
            130 => Self::SystemSleep,
            131 => Self::SystemWakeUp,
            132 => Self::SystemContextMenu,
            133 => Self::SystemMainMenu,
            134 => Self::SystemAppMenu,
            135 => Self::SystemMenuHelp,
            136 => Self::SystemMenuExit,
            137 => Self::SystemMenuSelect,
            138 => Self::SystemMenuRight,
            139 => Self::SystemMenuLeft,
            140 => Self::SystemMenuUp,
            141 => Self::SystemMenuDown,
            142 => Self::SystemColdRestart,
            143 => Self::SystemWarmRestart,
            144 => Self::DpadUp,
            145 => Self::DpadDown,
            146 => Self::DpadRight,
            147 => Self::DpadLeft,
            148 => Self::IndexTrigger,
            149 => Self::PalmTrigger,
            150 => Self::Thumbstick,
            151 => Self::SystemFunctionShift,
            152 => Self::SystemFunctionShiftLock,
            153 => Self::SystemFunctionShiftLockIndicator,
            154 => Self::SystemDismissNotification,
            155 => Self::SystemDoNotDisturb,
            156..160 => Self::Reserved9C_9F(value),
            160 => Self::SystemDock,
            161 => Self::SystemUndock,
            162 => Self::SystemSetup,
            163 => Self::SystemBreak,
            164 => Self::SystemDebuggerBreak,
            165 => Self::ApplicationBreak,
            166 => Self::ApplicationDebuggerBreak,
            167 => Self::SystemSpeakerMute,
            168 => Self::SystemHibernate,
            169 => Self::SystemMicrophoneMute,
            170..176 => Self::ReservedAA_A7(value),
            176 => Self::SystemDisplayInvert,
            177 => Self::SystemDisplayInternal,
            178 => Self::SystemDisplayExternal,
            179 => Self::SystemDisplayBoth,
            180 => Self::SystemDisplayDual,
            181 => Self::SystemDisplayToggleInt_ExtMode,
            182 => Self::SystemDisplaySwapPrimary_Secondary,
            183 => Self::SystemDisplayToggleLCDAutoscale,
            184..192 => Self::ReservedB8_BF(value),
            192 => Self::SensorZone,
            193 => Self::RPM,
            194 => Self::CoolantLevel,
            195 => Self::CoolantCriticalLevel,
            196 => Self::CoolantPump,
            197 => Self::ChassisEnclosure,
            198 => Self::WirelessRadioButton,
            199 => Self::WirelessRadioLED,
            200 => Self::WirelessRadioSliderSwitch,
            201 => Self::SystemDisplayRotationLockButton,
            202 => Self::SystemDisplayRotationLockSliderSwitch,
            203 => Self::ControlEnable,
            204..208 => Self::ReservedCC_CF(value),
            208 => Self::DockableDeviceUniqueID,
            209 => Self::DockableDeviceVendorID,
            210 => Self::DockableDevicePrimaryUsagePage,
            211 => Self::DockableDevicePrimaryUsageID,
            212 => Self::DockableDeviceDockingState,
            213 => Self::DockableDeviceDisplayOcclusion,
            214 => Self::DockableDeviceObjectType,
            215..224 => Self::ReservedD7_DF(value),
            224 => Self::CallActiveLED,
            225 => Self::CallMuteToggle,
            226 => Self::CallMuteLED,
            227..=65535 => Self::ReservedE3_FFFF(value),
        }
    }
}
