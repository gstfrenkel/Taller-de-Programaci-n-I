use hex::FromHexError;
use std::num::TryFromIntError;

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
    DecodeHex,
    TryInto,
}

impl From<std::io::Error> for MessageError {
    fn from(_: std::io::Error) -> MessageError {
        MessageError::ReadFromBytes
    }
}

impl From<FromHexError> for MessageError {
    fn from(_: FromHexError) -> MessageError {
        MessageError::DecodeHex
    }
}

impl From<TryFromIntError> for MessageError {
    fn from(_: TryFromIntError) -> MessageError {
        MessageError::TryInto
    }
}
