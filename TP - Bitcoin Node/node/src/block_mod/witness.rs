use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use std::io::Read;
use std::vec;

/// Represents a witness field for a transaction input.
#[derive(Debug, Clone)]
pub struct Witness {
    pub stack_item_count: CompactSizeUInt,
    pub stack_items: Vec<Vec<u8>>,
}

impl Witness {
    /// Creates a new Witness with the given stack items.
    ///
    /// This function constructs a new `Witness` instance with the provided `stack_items`. The
    /// `stack_item_count` field is calculated based on the length of `stack_items`.
    ///
    /// # Arguments
    ///
    /// * `stack_items` - A vector containing the stack items for the witness.
    ///
    /// # Returns
    ///
    /// A new `Witness` instance.
    pub fn new(stack_items: Vec<Vec<u8>>) -> Self {
        Witness {
            stack_item_count: CompactSizeUInt::from_number(stack_items.len() as u64),
            stack_items,
        }
    }

    /// Deserialize a Witness from a byte stream.
    ///
    /// This function reads the bytes from the given `stream` and constructs a `Witness` instance.
    /// It returns a `Result` where the `Ok` variant contains the deserialized `Witness`, or an
    /// `Err` variant with a `MessageError` if deserialization fails.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a type that implements the `Read` trait.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Witness, MessageError> {
        let stack_item_count = CompactSizeUInt::from_bytes(stream)?;
        let mut stack_items = vec![];

        for _ in 0..stack_item_count.value() {
            let item_length = CompactSizeUInt::from_bytes(stream)?;

            let mut item = vec![0u8; item_length.value().try_into()?];

            stream.read_exact(&mut item)?;

            stack_items.push(item);
        }

        Ok(Witness {
            stack_item_count,
            stack_items,
        })
    }

    /// Converts the Witness to its byte representation.
    ///
    /// This function serializes the `Witness` struct into a byte vector, suitable for storage or
    /// transmission.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the serialized `Witness`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];

        buffer.extend(self.stack_item_count.to_bytes());

        for item in self.stack_items.clone() {
            buffer.extend(CompactSizeUInt::from_number(item.len() as u64).to_bytes());
            buffer.extend(item);
        }

        buffer
    }

    /// Retrieves the public key from the Witness.
    ///
    /// This function returns the public key stored in the Witness stack items. If the Witness's
    /// `stack_item_count` is less than 2, an empty vector is returned.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the public key.
    pub fn get_pubkey(&self) -> Vec<u8> {
        if self.stack_item_count.value() < 2 {
            return vec![];
        }

        self.stack_items[1].clone()
    }
}
