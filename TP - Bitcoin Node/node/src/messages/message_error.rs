#[derive(Debug)]
pub enum MessageError {
    ReadFromBytes,
    InvalidInputPing,
    InvalidInputAddr,
    InvalidInputGetData,
    InvalidInputHeaders,
    InvalidInputInv,
    InvalidInputPong,
    InvalidInputVersion,
    InvalidBlockCommitment,
}

impl From<std::io::Error> for MessageError {
    fn from(_: std::io::Error) -> MessageError {
        MessageError::ReadFromBytes
    }
}

