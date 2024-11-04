#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum FIDOUsage {
    #[default]
    Undefined,
    U2FAuthenticatorDevice,
    Reserved02_1F(u16),
    InputReportData = 32,
    OutputReportData,
    Reserved22_FFFF(u16),
}
impl<T> From<T> for FIDOUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::U2FAuthenticatorDevice,
            2..32 => Self::Reserved02_1F(value),
            32 => Self::InputReportData,
            33 => Self::OutputReportData,
            34..=65535 => Self::Reserved22_FFFF(value),
        }
    }
}
