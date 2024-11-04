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
impl<T> From<T> for OrdinalUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Reserved,
            1 => Self::Instance1,
            2 => Self::Instance2,
            3 => Self::Instance3,
            4 => Self::Instance4,
            5..=65535 => Self::Instance5_65535(value),
        }
    }
}
