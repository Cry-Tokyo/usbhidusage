#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum ButtonUsage {
    #[default]
    NoButtonPressed,
    Button1PrimaryTrigger,
    Button2Secondary,
    Button3Tertiary,
    Button4,
    Button5_65535(u16),
}
impl<T> From<T> for ButtonUsage
where
    T: Into<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::NoButtonPressed,
            1 => Self::Button1PrimaryTrigger,
            2 => Self::Button2Secondary,
            3 => Self::Button3Tertiary,
            4 => Self::Button4,
            5..=65535 => Self::Button5_65535(value),
        }
    }
}
