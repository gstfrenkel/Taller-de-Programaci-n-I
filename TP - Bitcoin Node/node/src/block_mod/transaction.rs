use crate::block_mod::tx_in::TxIn;
use crate::block_mod::tx_out::TxOut;
use crate::block_mod::witness::Witness;
use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use crate::messages::read_from_bytes::{read_i32_from_bytes, read_u32_from_bytes, read_u8_from_bytes};
use bitcoin_hashes::{sha256d, sha256};
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents a Transaction in the Bitcoin protocol.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: i32,
    pub flag: u8,
    pub tx_in_count: CompactSizeUInt,
    pub tx_in_list: Vec<TxIn>,
    pub tx_out_count: CompactSizeUInt,
    pub tx_out_list: Vec<TxOut>,
    pub witness: Vec<Witness>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn new(version: i32, tx_in_list: Vec<TxIn>, tx_out_list: Vec<TxOut>, lock_time: u32, segwit: bool) -> Self {
        let mut flag = 0x00;

        if segwit{
            flag = 0x01;
        }

        Transaction{
            version,
            flag,
            tx_in_count: CompactSizeUInt::from_number(tx_in_list.len() as u64),
            tx_in_list,
            tx_out_count: CompactSizeUInt::from_number(tx_out_list.len() as u64),
            tx_out_list,
            witness: Vec::new(),
            lock_time  
        }
    }

    /// Creates a new `Transaction` instance from the provided byte stream.
    ///
    /// # Arguments
    /// * `stream` - A mutable reference to the byte stream.
    ///
    /// # Returns
    /// A `Result` containing the parsed `Transaction` instance or a `MessageError` if parsing fails.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Transaction, MessageError> {
        let version = read_i32_from_bytes(stream, true)?;        
        let mut tx_in_count = CompactSizeUInt::from_bytes(stream)?;
        let mut flag = 0;

        let is_segwit = tx_in_count.value() == 0;

        if is_segwit{
            flag = read_u8_from_bytes(stream)?;
            tx_in_count = CompactSizeUInt::from_bytes(stream)?;
        }

        let mut tx_in_list: Vec<TxIn> = Vec::new();

        for _i in 0..tx_in_count.value() {
            tx_in_list.push(TxIn::from_bytes(stream)?);
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

        Ok(Transaction {
            version,
            flag,
            tx_in_count,
            tx_in_list,
            tx_out_count,
            tx_out_list,
            witness,
            lock_time,
        })
    }

    /// Converts the `Transaction` instance to a byte representation.
    ///
    /// # Returns
    /// A vector of bytes representing the `Transaction` instance. 
    pub fn as_bytes(&self, segwit: bool) -> Vec<u8> {
        let mut buff = self.version.to_le_bytes().to_vec();

        if segwit && self.is_segwit(){
            buff.push(0x00);
            buff.push(self.flag);
        }

        buff.extend(self.tx_in_count.as_bytes());

        for txin in self.tx_in_list.iter() {
            buff.extend(&txin.as_bytes());
        }

        buff.extend(self.tx_out_count.as_bytes());

        for txout in self.tx_out_list.iter() {
            buff.extend(&txout.as_bytes());
        }

        if segwit && self.is_segwit(){
            for witness in self.witness.clone(){
                buff.extend(witness.as_bytes());
            }
        }

        buff.extend(self.lock_time.to_le_bytes());

        buff
    }

    ///Calculates the transaction ID by hashing the serialized bytes of the `BlockHeader`.
    ///
    /// # Returns
    /// A vector of bytes representing the transaction.
    pub fn get_id(&self, segwit: bool) -> Vec<u8> {
        sha256d::Hash::hash(&self.as_bytes(segwit))
            .to_byte_array()
            .to_vec()
    }

    /// Returns a reference to the list of transaction inputs.
    pub fn get_tx_in_list(&self) -> &Vec<TxIn> {
        &self.tx_in_list
    }

    /// Returns a reference to the list of transaction outputs.
    pub fn get_tx_out_list(&self) -> &Vec<TxOut> {
        &self.tx_out_list
    }

    pub fn get_witness(&self) -> &Vec<Witness>{
        &self.witness
    }

    pub fn is_segwit(&self) -> bool{
        self.flag != 0
    }

    pub fn get_witness_pubkey(&self, index: usize) -> Vec<u8>{
        if self.witness.len() <= index{
            return vec![];
        }

        self.witness[index].get_pubkey()
    }

    /// Computes the signature hash for the transaction at the given input index with the provided public key script.
    /// The signature hash is used for generating a digital signature that verifies the integrity of the transaction.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction input for which to compute the signature hash.
    /// * `pk_script` - The public key script associated with the transaction input.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the computed signature hash.
    pub fn p2pkh_signature_hash(&mut self, index: usize, pk_script: &[u8]) -> Vec<u8> {
        let mut buffer = self.version.to_le_bytes().to_vec();
        let mut aux_txin: TxIn;

        buffer.extend(self.tx_in_count.as_bytes());

        for (i, txin) in self.tx_in_list.iter().enumerate() {
            if i == index {
                aux_txin = TxIn::new(txin.get_prev_output().get_tx_id().clone(), txin.get_prev_output().get_index(), pk_script.to_vec(), txin.get_sequence());
            } else {
                aux_txin = TxIn::new(txin.get_prev_output().get_tx_id().clone(), txin.get_prev_output().get_index(), vec![], txin.get_sequence());
            }
            buffer.extend(&aux_txin.as_bytes());
        }

        buffer.extend(self.tx_out_count.as_bytes());

        for txout in self.tx_out_list.iter() {
            buffer.extend(txout.as_bytes());
        }

        buffer.extend(self.lock_time.to_le_bytes());
        buffer.extend((1_u32).to_le_bytes());

        sha256::Hash::hash(&buffer).as_byte_array().to_vec()
    }

    /// Computes the signature hash for a Pay-to-Witness-Public-Key-Hash (P2WPKH) transaction input.
    ///
    /// This function takes the index of the transaction input, the public key script of the corresponding
    /// output (pk_script), and a list of amounts associated with each input. It then computes the signature
    /// hash required for signing the P2WPKH input.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction input for which the signature hash is being computed.
    /// * `pk_script` - The public key script of the corresponding transaction output, excluding the length byte.
    /// * `amount_list` - A list of amounts associated with each input in the transaction.
    ///
    /// # Returns
    ///
    /// The computed signature hash as a byte vector.
    pub fn p2wpkh_signature_hash(&self, index: usize, pk_script: Vec<u8>, amount_list: Vec<i64>) -> Vec<u8>{
        let txins = &self.get_tx_in_list();

        let mut signature = self.version.to_le_bytes().to_vec();
        let mut previous_outpoints = vec![];
        let mut previous_sequences = vec![];

        for txin in txins.iter(){
            previous_outpoints.extend(txin.get_prev_output().as_bytes());
            previous_sequences.extend(txin.get_sequence().to_le_bytes());
        }

        signature.extend(sha256d::Hash::hash(&previous_outpoints).to_byte_array());
        signature.extend(sha256d::Hash::hash(&previous_sequences).to_byte_array());
        signature.extend(txins[index].get_prev_output().as_bytes());
        signature.extend(get_script_code(&pk_script[2..]));
        signature.extend(amount_list[index].to_le_bytes()); 
        signature.extend([0xff, 0xff, 0xff, 0xff]);

        let mut outputs = vec![];

        for txout in self.get_tx_out_list().iter(){
            outputs.extend(txout.as_bytes());
        }

        signature.extend(sha256d::Hash::hash(&outputs).to_byte_array());
        signature.extend(self.lock_time.to_le_bytes());
        signature.extend(1_u32.to_le_bytes());

        sha256::Hash::hash(&signature).as_byte_array().to_vec()
    }

    /// Sets the signature for the transaction input at the given index with the provided signature script.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction input for which to set the signature.
    /// * `signature_script` - The signature script to set for the transaction input.
    pub fn set_signature(&mut self, index: usize, signature_script: Vec<u8>) {
        self.tx_in_list[index].set_signature(signature_script);
    }

    pub fn set_witness(&mut self, stack_items: Vec<Vec<u8>>){
        self.witness.push(Witness::new(stack_items));
    }
}

fn get_script_code(pubkey_hash: &[u8]) -> Vec<u8>{
    let mut buffer = vec![0x19, 0x76, 0xa9, 0x14];
    buffer.extend(pubkey_hash);
    buffer.extend(vec![0x88, 0xac]);
    buffer
}
  
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "      Version: {}", self.version)?;
        writeln!(f, "      Flag: {}", self.flag)?;
        writeln!(f, "      Txin")?;

        for (i, txin) in self.tx_in_list.iter().enumerate() {
            writeln!(f, "          TxIn: {}", i)?;
            writeln!(f, "          {:?}", txin)?;
        }

        writeln!(f, "      TxOut")?;

        for (i, txout) in self.tx_out_list.iter().enumerate() {
            writeln!(f, "          TxOut: {}", i)?;
            writeln!(f, "          {:?}", txout)?;
        }

        writeln!(f, "      Witness")?;

        for (i, witness) in self.witness.iter().enumerate() {
            writeln!(f, "          Witness: {}", i)?;
            writeln!(f, "          {:?}", witness)?;
        }

        writeln!(f, "LockTime: {}", self.lock_time)?;
        Ok(())
    }
}

#[cfg(test)]
mod block_test {
    use hex::decode;
    use crate::block_mod::transaction::Transaction;
    use crate::messages::message_error::MessageError;

    #[test]
    fn segwit_tx_parsing() -> Result<(), MessageError> {
        let data = decode("020000000001011216d10ae3afe6119529c0a01abe7833641e0e9d37eb880ae5547cfb7c6c7bca0000000000fdffffff0246b31b00000000001976a914c9bc003bf72ebdc53a9572f7ea792ef49a2858d788ac731f2001020000001976a914d617966c3f29cfe50f7d9278dd3e460e3f084b7b88ac02473044022059570681a773748425ddd56156f6af3a0a781a33ae3c42c74fafd6cc2bd0acbc02200c4512c250f88653fae4d73e0cab419fa2ead01d6ba1c54edee69e15c1618638012103e7d8e9b09533ae390d0db3ad53cc050a54f89a987094bffac260f25912885b834b2c2500").unwrap();

        let mut stream = &data[..];

        if let Ok(transaction) = Transaction::from_bytes(&mut stream) {
            assert_eq!(data, transaction.as_bytes(true));
        } else {
            assert!(false);
        }
        
        Ok(())
    }
}
