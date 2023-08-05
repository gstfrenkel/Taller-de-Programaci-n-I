use bitcoin_hashes::{sha256d, Hash};

use super::coinbase::Coinbase;
use super::transaction::Transaction;
use crate::block_mod::block_header::BlockHeader;
use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use std::io::Read;
use std::process::exit;
use std::vec;

/// Represents a block in Bitcoin's blockchain.
#[derive(Debug)]
pub struct Block {
    block_header: BlockHeader,
    txn_count: CompactSizeUInt,
    coinbase: Coinbase,
    txn_list: Vec<Transaction>,
}

impl Block {
    /// Parses a byte stream and constructs a `Block` from it.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(Block)` if parsing is successful.
    /// - `Err(MessageError)` if an error occurs during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Block, MessageError> {
        let block_header = BlockHeader::from_bytes(stream)?;
        let txn_count = CompactSizeUInt::from_bytes(stream)?;
        let coinbase = Coinbase::from_bytes(stream)?;
        let mut txn_list = Vec::new();
        
        for _i in 0..txn_count.value() - 1 {
            txn_list.push(Transaction::from_bytes(stream)?);
        }

        let block = Block {
            block_header,
            txn_count,
            coinbase,
            txn_list,
        };

        if !block.proof_of_inclusion(){
            println!("Proof of inclusion has failed.");
            return Err(MessageError::InvalidBlockCommitment);
        }

        if !block.is_commitment_valid(){
            println!("Commitment validation has failed.");
            for tx in block.get_txn_list(){
                println!("{:?}\n", tx);
            }
            println!("No Anduvo\n");
            exit(-1);
            return Err(MessageError::InvalidBlockCommitment);
        }/*else{
            for tx in block.get_txn_list(){

                //println!("{:?}\n", tx);
            }

            println!("Anduvo");
            exit(-1);
        }*/
        //println!("Anduvo\n");

        Ok(block)
    }

    /// Returns the header of the block.
    pub fn get_header(&self) -> &BlockHeader {
        &self.block_header
    }

    /// Returns a reference to the merkle root hash of the block.
    pub fn get_merkle_root(&self) -> &Vec<u8> {
        self.block_header.get_merkle_root()
    }

    /// Returns a vector containing the transaction IDs of the block.
    pub fn get_txn_ids(&self) -> Vec<Vec<u8>> {
        let mut txn_ids: Vec<Vec<u8>> = vec![self.coinbase.get_id()];

        txn_ids.extend(self.txn_list.iter().map(|t| t.get_id(false)));

        txn_ids
    }

    /// Returns a vector with the transactions of the block.
    pub fn get_txn_list(&self) -> &Vec<Transaction> {
        &self.txn_list
    }

    pub fn proof_of_inclusion(&self) -> bool {
        let mut ids = self.get_txn_ids();

        *self.block_header.get_merkle_root() == calculate_merkle_root(&mut ids)
    }

    pub fn get_previuos_block_header(&self) -> &Vec<u8> {
        self.block_header.get_previuos_block_header()
    }

    pub fn proof_of_work(&self) -> bool{
        self.block_header.proof_of_work()
    }

    fn has_witnesses(&self) -> bool{
        self.coinbase.has_witnesses()
    }

    fn is_commitment_valid(&self) -> bool{
        if !self.has_witnesses(){
            return true;
        }

        let mut buffer = vec![vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]];

        for tx in self.get_txn_list(){
            //buffer.push(tx.get_id(tx.is_segwit()));
            if tx.is_segwit(){
                buffer.push(tx.get_id(true));
            } else{
                //println!("Hay casos sin witness");
            }
        }

        let witness_root_hash = calculate_merkle_root(&mut buffer);

        self.coinbase.is_commitment_valid(witness_root_hash)
    }
}

/// Recursively calculates the Merkle root by concatenating and hashing the levels of the Merkle tree.
///
/// # Arguments
/// * `txn_list` - A mutable reference to the transaction ID list.
///
/// # Returns
/// The calculated Merkle root as a vector of bytes.
fn calculate_merkle_root(txn_list: &mut Vec<Vec<u8>>) -> Vec<u8> {
    let mut new_level_txn_list: Vec<Vec<u8>> = Vec::new();
    let mut merkle_root_hash: Vec<u8> = Vec::new();

    // Return when the length of the list is one
    if txn_list.len() == 1 {
        merkle_root_hash.extend(txn_list[0].as_slice());
        return merkle_root_hash;
    }

    // If list is odd, duplicate and append the last item
    if txn_list.len() % 2 != 0 {
        if let Some(last_txn) = txn_list.last() {
            txn_list.push(last_txn.clone());
        }
    }

    // Concatenation and hashing
    for i in (0..(txn_list.len() - 1)).step_by(2) {
        let mut concatenated_hash: Vec<u8> = Vec::new();
        concatenated_hash.extend(&txn_list[i]);
        concatenated_hash.extend(&txn_list[i + 1]);
        let hash = sha256d::Hash::hash(&concatenated_hash)
            .to_byte_array()
            .to_vec();

        new_level_txn_list.push(hash);
    }

    calculate_merkle_root(&mut new_level_txn_list)
}


impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.block_header)?;
        writeln!(f, "Transaction count: {:?}", self.txn_count)?;
        writeln!(f, "Transaction coinbase:")?;
        writeln!(f, "      {:?}", self.coinbase)?;

        for (i, t) in self.txn_list.iter().enumerate() {
            writeln!(f, "Transaction number: {}", i + 1)?;
            writeln!(f, "       {:?}", t)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod block_test {
    use super::*;
    use crate::block_mod::block::Block;
    use crate::messages::message_error::MessageError;
    use std::fs::OpenOptions;

    #[test]
    fn test_new_block_from_bytes() -> Result<(), MessageError> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open("data/test_block.bin")?;

        let block = match Block::from_bytes(&mut file) {
            Ok(header) => header,
            Err(_) => return Ok(()),
        };

        let mut tx_ids = block.get_txn_ids();

        assert_eq!(calculate_merkle_root(&mut tx_ids), *block.get_merkle_root());

        Ok(())
    }
}



#[cfg(test)]
mod awdasd {
    use bitcoin_hashes::{sha256d, Hash};
    use hex::decode;
    use crate::block_mod::transaction::Transaction;
    use crate::messages::message_error::MessageError;
    use crate::messages::read_from_bytes::{decode_hex, encode_hex};

    use super::calculate_merkle_root;

    #[test]
    fn adasdwd() -> Result<(), MessageError> {
        let mut a = decode_hex("66beaceb4be99da1e9824448231ab4fd37bacaee912381e779b37cf0e1dadad7").unwrap();
        let mut b = decode_hex("aecb37e25954e15489e25548eb663ffdfd8a1362cac757ad62e9614453d2a577").unwrap();
        let mut c = decode_hex("5b211bc589cbdf5ad86cab1e2fe91f01c8ab934d21536b35864d30a3ff778456").unwrap();
        let mut d = decode_hex("66beaceb4be99da1e9824448231ab4fd37bacaee912381e779b37cf0e1dadad7").unwrap();
        a.reverse();
        b.reverse();
        c.reverse();
        d.reverse();

        let mut e = vec![a, b, c, d];
        let d = calculate_merkle_root(&mut e);

        println!("{:?}", encode_hex(&d).unwrap());

        let f = sha256d::Hash::hash(&d)
            .to_byte_array()
            .to_vec();

        println!("{:?}", encode_hex(&f).unwrap());


        Ok(())
    }
}