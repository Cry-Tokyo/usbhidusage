#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum MonitorEnumeratedUsage {
    #[default]
    Reserved,
    Enum1,
    Enum2,
    Enum3,
    Enum4,
    Enum5_65535(u16),
}
impl<T> From<T> for MonitorEnumeratedUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value: u16 = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Reserved,
            1 => Self::Enum1,
            2 => Self::Enum2,
            3 => Self::Enum3,
            4 => Self::Enum4,
            5..=65535 => Self::Enum5_65535(value),
        }
    }
}
