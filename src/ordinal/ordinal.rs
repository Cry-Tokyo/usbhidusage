#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum OrdinalUsage {
    #[default]
    Reserved,
    Instance1,
    Instance2,
    Instance3,
    Instance4,
    Instance5_65535(u16),
}
impl From<&u16> for OrdinalUsage {
    fn from(value: &u16) -> Self {
        match value {
            0 => Self::Reserved,
            1 => Self::Instance1,
            2 => Self::Instance2,
            3 => Self::Instance3,
            4 => Self::Instance4,
            5..=65535 => Self::Instance5_65535(*value),
        }
    }
}
