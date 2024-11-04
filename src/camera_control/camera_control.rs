#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum CameraControlUsage {
    #[default]
    Undefined,
    Reserved01_1F(u16),
    CameraAutoFocus = 32,
    CameraShutter,
    Reserved22_FFFF(u16),
}
impl<T> From<T> for CameraControlUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1..32 => Self::Reserved01_1F(value),
            32 => Self::CameraAutoFocus,
            33 => Self::CameraShutter,
            34..=65535 => Self::Reserved22_FFFF(value),
        }
    }
}
