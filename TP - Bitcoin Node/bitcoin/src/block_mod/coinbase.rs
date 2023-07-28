use crate::block_mod::tx_in_coinbase::TxInCoinbase;
use crate::block_mod::tx_out::TxOut;
use crate::block_mod::witness::Witness;
use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use crate::messages::read_from_bytes::{read_i32_from_bytes, read_u32_from_bytes, read_u8_from_bytes};
use crate::proof_of_inclusion_mod::proof_of_inclusion::compute_hash;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

const COMMITMENT_MIN_LEN: u64 = 38;
const COMMITMENT_START_BYTES: &[u8] = &[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];

/// Represents a Coinbase transaction in the Bitcoin protocol.
#[derive(Debug)]
pub struct Coinbase {
    version: i32,
    flag: u8,
    tx_in_count: CompactSizeUInt,
    tx_in_list: Vec<TxInCoinbase>,
    tx_out_count: CompactSizeUInt,
    tx_out_list: Vec<TxOut>,
    witness: Vec<Witness>,
    lock_time: u32,
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

        if is_segwit{
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

        if is_segwit{
            for _ in 0..tx_in_count.value(){
                witness.push(Witness::from_bytes(stream)?);
            }
        } else{
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
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(self.version.to_le_bytes());
        buff.extend(self.tx_in_count.as_bytes());

        for txin in self.tx_in_list.iter() {
            //for each?
            buff.extend(&txin.as_bytes());
        }

        buff.extend(self.tx_out_count.as_bytes());

        for txout in self.tx_out_list.iter() {
            buff.extend(&txout.as_bytes());
        }

        buff.extend(self.lock_time.to_le_bytes());

        buff
    }

    /// Computes the ID of the Coinbase transaction by hashing its serialized bytes.
    ///
    /// # Returns
    /// A vector of bytes representing the transaction ID.
    pub fn get_id(&self) -> Vec<u8> {
        sha256d::Hash::hash(&self.as_bytes())
            .to_byte_array()
            .to_vec()
    }    
    
    pub fn is_commitment_valid(&self, witness_root_hash: Vec<u8>) -> bool{
        let witness_reserved_value = self.tx_in_list[0].script();
        let commitment_hash = compute_hash(witness_root_hash, witness_reserved_value);

        for tx_out in &self.tx_out_list{
            let pk_script = tx_out.get_pk_script();
            let pk_script_bytes = tx_out.get_pk_script_bytes().value();

            if pk_script_bytes >= COMMITMENT_MIN_LEN &&
               pk_script[0..6] == COMMITMENT_START_BYTES.to_vec() &&
               pk_script[6..38] == commitment_hash
            {
                return true;
            }
        }

        false
    }

    pub fn has_witnesses(&self) -> bool{
        for witness in &self.witness{
            if !witness.is_empty(){
                return true;
            }
        }

        false
    }
}




/*  Código viejo, pero lo dejo por si acaso. De última, se vuela!!!! 
fn calculate_witness_root(witness_hashes: Vec<Vec<u8>>) -> Vec<u8> {
    if witness_hashes.is_empty() {
        return Vec::new();
    }

    let mut intermediate_results = Vec::new();
    let mut current_level = witness_hashes.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        for i in (0..current_level.len()).step_by(2) {
            let combined_hash = compute_hash(current_level[i].clone(), current_level[i+1].clone());

            next_level.push(combined_hash);
        }

        intermediate_results.extend(next_level.iter().cloned());
        current_level = next_level;
    }

    current_level[0].clone()
}*/



impl std::fmt::Display for Coinbase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "      Version: {}", self.version)?;
        writeln!(f, "      Flag: {}", self.flag)?;
        writeln!(f, "      Txin")?;

        for (i, txin) in self.tx_in_list.iter().enumerate() {
            writeln!(f, "          TxIn: {}", i+1)?;
            writeln!(f, "          {:?}", txin)?;
        }

        writeln!(f, "      TxOut")?;

        for (i, txout) in self.tx_out_list.iter().enumerate() {
            writeln!(f, "          TxOut: {}", i+1)?;
            writeln!(f, "          {:?}", txout)?;
        }

        writeln!(f, "      Witness")?;

        for (i, witness) in self.witness.iter().enumerate() {
            writeln!(f, "          Witness: {}", i+1)?;
            writeln!(f, "          {:?}", witness)?;
        }

        writeln!(f, "LockTime: {}", self.lock_time)?;
        Ok(())
    }
}
