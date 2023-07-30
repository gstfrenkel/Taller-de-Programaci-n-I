use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use bitcoin_hashes::{Hash, sha256d};
use std::io::Read;
use std::vec;

#[derive(Debug, Clone)]
pub struct Witness{
    pub stack_item_count: CompactSizeUInt,
    pub stack_items: Vec<Vec<u8>>,
}

impl Witness {
    pub fn new(stack_items: Vec<Vec<u8>>) -> Self{
        Witness {
            stack_item_count: CompactSizeUInt::from_number(stack_items.len().try_into().unwrap()),
            stack_items,
        }
    }

    pub fn from_bytes(stream: &mut dyn Read) -> Result<Witness, MessageError> {        
        let stack_item_count = CompactSizeUInt::from_bytes(stream)?;
        let mut stack_items = vec![];

        for _ in 0..stack_item_count.value(){
            let item_length = CompactSizeUInt::from_bytes(stream)?;

            let mut item = vec![0u8; item_length.value().try_into().unwrap()];

            stream.read_exact(&mut item)?;

            stack_items.push(item);
        }

        Ok(Witness { stack_item_count, stack_items })
    }

    pub fn as_bytes(&self) -> Vec<u8>{
        let mut buffer = vec![];

        buffer.extend(self.stack_item_count.as_bytes());

        for item in self.stack_items.clone(){
            buffer.extend(CompactSizeUInt::from_number(item.len().try_into().unwrap()).as_bytes());
            buffer.extend(item);
        }

        buffer
    }

    pub fn hash_items(&self) -> Vec<u8>{
        let mut buffer = vec![];

        for item in self.stack_items.clone(){
            buffer.extend(item);
        }

        sha256d::Hash::hash(&buffer).to_byte_array().to_vec()
    }

    pub fn is_empty(&self) -> bool{
        if self.stack_item_count.value() == 0{
            return true;
        }

        for item in &self.stack_items{
            for byte in item{
                if *byte != 0 {
                    return false;
                }
            }
        }
        
        true
    }
}