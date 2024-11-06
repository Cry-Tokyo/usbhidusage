#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
#[warn(
    deprecated,
    reason = "This function does not currently handle all possible values of the input type as the"
)]
pub enum GamingDeviceUsage {
    #[default]
    ACK = 0x40,
    Enable,
    Disable,
    SelfTest,
    RequestGATReport,
    CalculateCRC = 0x47,
    NumberofNoteDataEntries = 0x0210,
    ReadNoteTable,
    ExtendTimeout,
    AcceptNote_Ticket,
    ReturnNote_Ticket,
    ReadNoteAcceptorMetrics = 0x021A,
    Unknown(u16),
}
impl<T> From<T> for GamingDeviceUsage
where
    T: TryInto<u16>,
{
    #[warn(
        deprecated,
        reason = "This function does not currently handle all possible values of the input type as the"
    )]
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            64 => Self::ACK,
            65 => Self::Enable,
            66 => Self::Disable,
            67 => Self::SelfTest,
            68 => Self::RequestGATReport,
            71 => Self::CalculateCRC,
            528 => Self::NumberofNoteDataEntries,
            529 => Self::ReadNoteTable,
            530 => Self::ExtendTimeout,
            531 => Self::AcceptNote_Ticket,
            532 => Self::ReturnNote_Ticket,
            538 => Self::ReadNoteAcceptorMetrics,
            _ => Self::Unknown(value),
        }
    }
}
