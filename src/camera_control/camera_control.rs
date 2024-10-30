#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum CameraControlUsage {
    #[default]
    Undefined,
    Reserved01_1F(u16),
    CameraAuto_focus,
    CameraShutter,
    Reserved22_FFFF(u16),
}
impl From<&u16> for CameraControlUsage {
    fn from(value: &u16) -> Self {
        match value {
            0 => Self::Undefined,
            1..32 => Self::Reserved01_1F(*value),
            32 => Self::CameraAuto_focus,
            33 => Self::CameraShutter,
            34..=65535 => Self::Reserved22_FFFF(*value),
        }
    }
}
