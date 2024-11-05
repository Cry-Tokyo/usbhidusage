#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum GamingDeviceUsage {
    #[default]

}
impl<T> From<T> for GamingDeviceUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {

        }
    }
}
