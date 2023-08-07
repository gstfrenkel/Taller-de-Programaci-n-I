use crate::messages::{
    compact_size::CompactSizeUInt,
    message_error::MessageError,
    read_from_bytes::{read_i64_from_bytes, read_vec_from_bytes},
};
use std::io::Read;

/// Represents a transaction output (TxOut) in a transaction.
#[derive(Debug, Clone)]
pub struct TxOut {
    pub value: i64,
    pub pk_script_bytes: CompactSizeUInt,
    pub pk_script: Vec<u8>,
}

impl TxOut {
    pub fn new(value: i64, pk_script: Vec<u8>) -> TxOut {
        TxOut {
            value,
            pk_script_bytes: CompactSizeUInt::from_number(pk_script.len() as u64),
            pk_script,
        }
    }
    /// Parses a byte stream and constructs a `TxOut` (transaction output) from it.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(TxOut)` if parsing is successful.
    /// - `Err(MessageError)` if an error occurs during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<TxOut, MessageError> {
        let value = read_i64_from_bytes(stream, true)?;
        let pk_script_bytes = CompactSizeUInt::from_bytes(stream)?;
        let pk_script = read_vec_from_bytes(stream, pk_script_bytes.value() as usize)?;

        Ok(TxOut {
            value,
            pk_script_bytes,
            pk_script,
        })
    }

    /// Converts the `TxOut` (transaction output) into a byte representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `TxOut`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(self.value.to_le_bytes());
        buff.extend(self.pk_script_bytes.to_bytes());
        buff.extend(&self.pk_script);

        buff
    }

    /// Returns a reference to the value of the transaction output.
    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn get_pk_script(&self) -> Vec<u8> {
        self.pk_script.clone()
    }

    pub fn get_pk_script_bytes(&self) -> CompactSizeUInt {
        self.pk_script_bytes.clone()
    }

    pub fn is_p2wpkh(&self) -> bool {
        self.pk_script.len() == 22
            && self.pk_script.first() == Some(&0)
            && self.pk_script.get(1) == Some(&20)
    }
}
