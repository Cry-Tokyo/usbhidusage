#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum MonitorUsage {
    #[default]
    Undefined,
    MonitorControl,
    EDIDInformation,
    VDIFInformation,
    VESAVersion,
    Reserved05_FFFF(u16),
}
impl<T> From<T> for MonitorUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::MonitorControl,
            2 => Self::EDIDInformation,
            3 => Self::VDIFInformation,
            4 => Self::VESAVersion,
            5..=65535 => Self::Reserved05_FFFF(value),
        }
    }
}
