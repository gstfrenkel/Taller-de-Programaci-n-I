use crate::block_mod::tx_in_coinbase::TxInCoinbase;
use crate::block_mod::tx_out::TxOut;
use crate::block_mod::witness::Witness;
use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use crate::messages::read_from_bytes::{
    read_i32_from_bytes, read_u32_from_bytes, read_u8_from_bytes,
};
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents a Coinbase transaction in the Bitcoin protocol.
#[derive(Debug)]
pub struct Coinbase {
    pub version: i32,
    pub flag: u8,
    pub tx_in_count: CompactSizeUInt,
    pub tx_in_list: Vec<TxInCoinbase>,
    pub tx_out_count: CompactSizeUInt,
    pub tx_out_list: Vec<TxOut>,
    pub witness: Vec<Witness>,
    pub lock_time: u32,
}

impl Coinbase {
    /// Reads and constructs a `Coinbase` instance from the byte stream.
    ///
    /// # Arguments
    /// * `stream` - A mutable reference to the byte stream.
    ///
    /// # Returns
    /// A Result containing the constructed `Coinbase` if successful, otherwise a `MessageError`.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Coinbase, MessageError> {
        let version = read_i32_from_bytes(stream, true)?;
        let mut tx_in_count = CompactSizeUInt::from_bytes(stream)?;

        let is_segwit = tx_in_count.value() == 0;
        let mut flag = 0;

        if is_segwit {
            flag = read_u8_from_bytes(stream)?;
            tx_in_count = CompactSizeUInt::from_bytes(stream)?;
        }

        let mut tx_in_list: Vec<TxInCoinbase> = Vec::new();

        for _i in 0..tx_in_count.value() {
            tx_in_list.push(TxInCoinbase::from_bytes(stream)?);
        }

        let tx_out_count = CompactSizeUInt::from_bytes(stream)?;
        let mut tx_out_list: Vec<TxOut> = Vec::new();

        for _i in 0..tx_out_count.value() {
            tx_out_list.push(TxOut::from_bytes(stream)?);
        }

        let mut witness = vec![];

        if is_segwit {
            for _ in 0..tx_in_count.value() {
                witness.push(Witness::from_bytes(stream)?);
            }
        } else {
            witness = vec![];
        }

        let lock_time = read_u32_from_bytes(stream, true)?;
        let coinbase = Coinbase {
            version,
            flag,
            tx_in_count,
            tx_in_list,
            tx_out_count,
            tx_out_list,
            witness,
            lock_time,
        };

        //println!("{}", coinbase);

        Ok(coinbase)
    }

    /// Converts the `Coinbase` instance to bytes.
    ///
    /// # Returns
    /// A vector of bytes representing the `Coinbase`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(self.version.to_le_bytes());
        buff.extend(self.tx_in_count.to_bytes());

        for txin in self.tx_in_list.iter() {
            //for each?
            buff.extend(&txin.to_bytes());
        }

        buff.extend(self.tx_out_count.to_bytes());

        for txout in self.tx_out_list.iter() {
            buff.extend(&txout.to_bytes());
        }

        buff.extend(self.lock_time.to_le_bytes());

        buff
    }

    /// Computes the ID of the Coinbase transaction by hashing its serialized bytes.
    ///
    /// # Returns
    /// A vector of bytes representing the transaction ID.
    pub fn get_id(&self) -> Vec<u8> {
        sha256d::Hash::hash(&self.to_bytes())
            .to_byte_array()
            .to_vec()
    }
}

impl std::fmt::Display for Coinbase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "      Version: {}", self.version)?;
        writeln!(f, "      Flag: {}", self.flag)?;
        writeln!(f, "      Txin")?;

        for (i, txin) in self.tx_in_list.iter().enumerate() {
            writeln!(f, "          TxIn: {}", i + 1)?;
            writeln!(f, "          {:?}", txin)?;
        }

        writeln!(f, "      TxOut")?;

        for (i, txout) in self.tx_out_list.iter().enumerate() {
            writeln!(f, "          TxOut: {}", i + 1)?;
            writeln!(f, "          {:?}", txout)?;
        }

        writeln!(f, "      Witness")?;

        for (i, witness) in self.witness.iter().enumerate() {
            writeln!(f, "          Witness: {}", i + 1)?;
            writeln!(f, "          {:?}", witness)?;
        }

        writeln!(f, "LockTime: {}", self.lock_time)?;
        Ok(())
    }
}
