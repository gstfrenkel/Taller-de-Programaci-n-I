use std::io::Read;

use super::{
    compact_size::CompactSizeUInt, header::MessageHeader, message_constants::INV_COMMAND,
    message_error::MessageError,
};
use crate::messages::inventory::Inventory;

/// Represents an inventory message.
#[derive(Debug)]
pub struct Inv {
    _header: MessageHeader,
    _count: CompactSizeUInt,
    inventory_list: Vec<Inventory>,
    data_type: u32,
}

impl Inv {
    /// Parses an inventory message from the provided byte stream.
    pub fn from_bytes(header: MessageHeader, stream: &mut dyn Read) -> Result<Inv, MessageError> {
        if header.get_command_name() != INV_COMMAND {
            return Err(MessageError::InvalidInputInv);
        }

        let count = CompactSizeUInt::from_bytes(stream)?;
        let mut inventory_list = Vec::new();

        for _ in 0..count.value() {
            inventory_list.push(Inventory::from_bytes(stream)?)
        }

        let data_type = inventory_list
            .last()
            .ok_or(MessageError::ReadFromBytes)?
            .get_type();

        Ok(Inv {
            _header: header,
            _count: count,
            inventory_list,
            data_type,
        })
    }

    /// Returns a clone of the list of inventory items.
    pub fn get_inventories(&self) -> Vec<Inventory> {
        self.inventory_list.clone()
    }

    /// Returns a clone of the list of inventory items.
    pub fn get_type(&self) -> u32 {
        self.data_type
    }
}
