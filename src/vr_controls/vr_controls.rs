use std::default;

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum VrControlsUsage {
    #[default]
    Undefined,
    Belt,
    BodySuit,
    Flexor,
    Glove,
    HeadTracker,
    HeadMountedDisplay,
    HandTracker,
    Oculometer,
    Vest,
    AnimatronicDevice,
    Reserved0B_1F(u16),
    StereoEnable = 32,
    DisplayEnable,
    Reserved22_FFFF(u16),
}
impl From<&u16> for VrControlsUsage {
    fn from(value: &u16) -> Self {
        match value {
            0 => Self::Undefined,
            1 => Self::Belt,
            2 => Self::BodySuit,
            3 => Self::Flexor,
            4 => Self::Glove,
            5 => Self::HeadTracker,
            6 => Self::HeadMountedDisplay,
            7 => Self::HandTracker,
            8 => Self::Oculometer,
            9 => Self::Vest,
            10 => Self::AnimatronicDevice,
            11..32 => Self::Reserved0B_1F(*value),
            32 => Self::StereoEnable,
            33 => Self::DisplayEnable,
            34..=65535 => Self::Reserved22_FFFF(*value),
        }
    }
}
