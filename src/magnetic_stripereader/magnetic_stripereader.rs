#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum MagneticStripeReaderUsage {
    #[default]
    Undefined,
    MSRDeviceRead_Only,
    Reserved02_10(u16),
    Track1Length = 17,
    Track2Length,
    Track3Length,
    TrackJISLength,
    Reserved15_1F(u16),
    TrackData = 32,
    Track1Data,
    Track2Data,
    Track3Data,
    TrackJISData,
    Reserved25_FFFF(u16),
}
impl<T> From<T> for MagneticStripeReaderUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::MSRDeviceRead_Only,
            2..17 => Self::Reserved02_10(value),
            17 => Self::Track1Length,
            18 => Self::Track2Length,
            19 => Self::Track3Length,
            20 => Self::TrackJISLength,
            21..32 => Self::Reserved15_1F(value),
            32 => Self::TrackData,
            33 => Self::Track1Data,
            34 => Self::Track2Data,
            35 => Self::Track3Data,
            36 => Self::TrackJISData,
            37..=65535 => Self::Reserved25_FFFF(value),
        }
    }
}
