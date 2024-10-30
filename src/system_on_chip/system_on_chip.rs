#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum SoCUsage {
    #[default]
    Undefined,
    SocControl,
    FirmwareTransfer,
    FirmwareFileId,
    FileOffsetInBytes,
    FileTransferSizeMaxInBytes,
    FilePayload,
    FilePayloadSizeInBytes,
    FilePayloadContainsLastBytes,
    FileTransferStop,
    FileTransferTillEnd,
    Reserved0B_FFFF(u16),
}
impl From<&u16> for SoCUsage {
    fn from(value: &u16) -> Self {
        match value {
            0 => Self::Undefined,
            1 => Self::SocControl,
            2 => Self::FirmwareTransfer,
            3 => Self::FirmwareFileId,
            4 => Self::FileOffsetInBytes,
            5 => Self::FileTransferSizeMaxInBytes,
            6 => Self::FilePayload,
            7 => Self::FilePayloadSizeInBytes,
            8 => Self::FilePayloadContainsLastBytes,
            9 => Self::FileTransferStop,
            10 => Self::FileTransferTillEnd,
            11..=65535 => Self::Reserved0B_FFFF(*value),
        }
    }
}
