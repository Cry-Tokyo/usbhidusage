#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum GamingDeviceUsage {
    #[default]
    Undefined,
    Background_NonuserControls,
    Reserved02_1F(u16),
    BatteryStrength = 32,
    WirelessChannel,
    WirelessID,
    DiscoverWirelessControl,
    SecurityCodeCharacterEntered,
    SecurityCodeCharacterErased,
    SecurityCodeCleared,
    SequenceID,
    SequenceIDReset,
    RFSignalStrength,
    SoftwareVersion,
    ProtocolVersion,
    HardwareVersion,
    Major,
    Minor,
    Revision,
    Handedness,
    EitherHand,
    LeftHand,
    RightHand,
    BothHands,
    Reserved35_3F(u16),
    GripPoseOffset = 64,
    PointerPoseOffset,
    Reserved42_FFFF(u16),
}
impl<T> From<T> for GamingDeviceUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::Background_NonuserControls,
            2..32 => Self::Reserved02_1F(value),
            32 => Self::BatteryStrength,
            33 => Self::WirelessChannel,
            34 => Self::WirelessID,
            35 => Self::DiscoverWirelessControl,
            36 => Self::SecurityCodeCharacterEntered,
            37 => Self::SecurityCodeCharacterErased,
            38 => Self::SecurityCodeCleared,
            39 => Self::SequenceID,
            40 => Self::SequenceIDReset,
            41 => Self::RFSignalStrength,
            42 => Self::SoftwareVersion,
            43 => Self::ProtocolVersion,
            44 => Self::HardwareVersion,
            45 => Self::Major,
            46 => Self::Minor,
            47 => Self::Revision,
            48 => Self::Handedness,
            49 => Self::EitherHand,
            50 => Self::LeftHand,
            51 => Self::RightHand,
            52 => Self::BothHands,
            53..64 => Self::Reserved35_3F(value),
            64 => Self::GripPoseOffset,
            65 => Self::PointerPoseOffset,
            66..=65535 => Self::Reserved42_FFFF(value),
        }
    }
}
