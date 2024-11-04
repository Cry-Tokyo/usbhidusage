struct UnicodeUsage {
    char: char,
}
impl UnicodeUsage {
    const REPLACEMENT_CHARACTER: UnicodeUsage = UnicodeUsage::new(char::REPLACEMENT_CHARACTER);
    const fn new(char: char) -> Self {
        UnicodeUsage { char: char }
    }
}
impl<T> From<T> for UnicodeUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value: u16 = value.try_into().unwrap_or(0);
        Self::new(char::from_u32(value as u32).unwrap_or('�'))
    }
}
