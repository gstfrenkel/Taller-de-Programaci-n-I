use crate::bech32::bech32_errors::WitnessProgramError;
use node::messages::message_error::MessageError;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum TransactionCreateError {
    InsufficientFunds,
    UnavailableOutput,
    PrivateKey,
    Decode58,
    DecodeHex,
    EncodeHex,
    GetPrivateKey,
    WitnessCreationError,
    WitnessProgramError,
}

impl From<MessageError> for TransactionCreateError {
    fn from(_: MessageError) -> TransactionCreateError {
        TransactionCreateError::WitnessCreationError
    }
}

impl From<WitnessProgramError> for TransactionCreateError {
    fn from(_: WitnessProgramError) -> TransactionCreateError {
        TransactionCreateError::WitnessProgramError
    }
}

impl From<ParseIntError> for TransactionCreateError {
    fn from(_: ParseIntError) -> TransactionCreateError {
        TransactionCreateError::DecodeHex
    }
}

impl From<std::fmt::Error> for TransactionCreateError {
    fn from(_: std::fmt::Error) -> TransactionCreateError {
        TransactionCreateError::EncodeHex
    }
}

impl From<secp256k1::Error> for TransactionCreateError {
    fn from(_: secp256k1::Error) -> TransactionCreateError {
        TransactionCreateError::GetPrivateKey
    }
}
