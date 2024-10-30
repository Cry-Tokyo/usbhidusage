#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum ButtonUsage {
    #[default]
    NoButtonPressed,
    Button1primary_trigger,
    Button2secondary,
    Button3tertiary,
    Button4,
    Button5_65535(u16),
}
impl From<&u16> for ButtonUsage {
    fn from(value: &u16) -> Self {
        match value {
            0 => Self::NoButtonPressed,
            1 => Self::Button1primary_trigger,
            2 => Self::Button2secondary,
            3 => Self::Button3tertiary,
            4 => Self::Button4,
            5..=65535 => Self::Button5_65535(*value),
        }
    }
}
