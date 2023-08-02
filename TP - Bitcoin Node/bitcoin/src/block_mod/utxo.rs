use super::outpoint::Outpoint;
use super::transaction::Transaction;
use crate::block_mod::block::Block;
use crate::block_mod::tx_in::TxIn;
use crate::block_mod::tx_out::TxOut;
use std::collections::HashMap;

/// Represents a collection of unspent transaction outputs (UTXOs).
#[derive(Debug)]
pub struct UnspentTx {
    /// The mapping of hashed transaction IDs to a map of output indices to `TxOut` objects.
    /// The outer `HashMap` uses the hashed transaction ID as the key, while the inner `HashMap`
    /// uses the transaction output's index as the key and the corresponding `TxOut` object as the value.
    utxo: HashMap<Vec<u8>, HashMap<u32, TxOut>>,
}

impl UnspentTx {
    /// Creates a new instance of `UnspentTx`.
    ///
    /// # Returns
    ///
    /// A new `UnspentTx` object.
    pub fn new() -> UnspentTx {
        UnspentTx {
            utxo: HashMap::new(),
        }
    }

    /// Updates the `UnspentTx` object by processing a new block.
    ///
    /// # Arguments
    ///
    /// * `new_block` - A reference to the `Block` object representing the new block to be processed.
    pub fn update(&mut self, new_block: &Block) {
        for transaction in new_block.get_txn_list() {
            self.update_transaction(transaction);
        }
    }

    /// Updates the `UnspentTx` object by processing a new transaction.
    ///
    /// # Arguments
    ///
    /// * `new_tx` - A reference to the `Transaction` object representing the new transaction to be processed.
    pub fn update_transaction(&mut self, new_tx: &Transaction) {
        for tx_in in new_tx.get_tx_in_list() {
            self.remove_tx_out(tx_in);
        }

        let new_tx_id = new_tx.get_id(false);

        for (index, tx_out) in new_tx.get_tx_out_list().iter().enumerate() {
            self.add_tx_out(tx_out, &new_tx_id, index);
        }
    }

    /// Removes a transaction output from the `UnspentTx` object based on the provided `TxIn`.
    ///
    /// # Arguments
    ///
    /// * `new_tx_in` - A reference to the `TxIn` object representing the transaction input that spends the output.
    fn remove_tx_out(&mut self, new_tx_in: &TxIn) {
        let tx_id = new_tx_in.get_prev_output().get_tx_id();
        let index = new_tx_in.get_prev_output().get_index();

        let transaction_outputs = match self.utxo.get_mut(tx_id) {
            Some(transaction) => transaction,
            None => {
                return;
            }
        };

        match transaction_outputs.remove(&index) {
            Some(_) => {
                if transaction_outputs.values().len() == 0 {
                    self.utxo.remove(tx_id);
                }
            }
            None => {},
        }
    }

    /// Adds a transaction output to the `UnspentTx` object.
    ///
    /// # Arguments
    ///
    /// * `new_tx_out` - A reference to the `TxOut` object representing the transaction output to be added.
    /// * `tx_id` - The hashed ID of the transaction to which the output belongs.
    /// * `index` - The index of the output within the transaction.
    fn add_tx_out(&mut self, new_tx_out: &TxOut, tx_id: &[u8], index: usize) {
        //corroborar el tipo de tx_id
        let transation_outputs = self.utxo.entry(tx_id.to_vec()).or_insert(HashMap::new());

        transation_outputs
            .entry(index as u32)
            .or_insert(new_tx_out.clone());
    }

    /// Returns the total number of transactions in the `UnspentTx` object.
    ///
    /// # Returns
    ///
    /// The total number of transactions.
    pub fn tx_count(&self) -> usize {
        let mut cant = 0;

        for transaction in self.utxo.iter() {
            cant += transaction.1.iter().len();
        }
        cant
    }

    pub fn contains_key(&self, output: &Outpoint) -> bool {
        if let Some(outputs) = self.utxo.get(output.get_tx_id()){
            println!("true");
            return outputs.contains_key(&output.get_index());
        }
        false
    }

    pub fn get_utxo(&self) -> &HashMap<Vec<u8>, HashMap<u32, TxOut>> {
        &self.utxo
    }


}

impl Default for UnspentTx {
    fn default() -> Self {
        Self::new()
    }
}
