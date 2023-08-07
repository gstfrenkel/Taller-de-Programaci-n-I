use super::wallet_tx::WalletTx;
use crate::{
    block_mod::tx_out::TxOut,
    messages::{
        message_error::MessageError,
        read_from_bytes::{
            fill_command, read_i64_from_bytes, read_u32_from_bytes, read_vec_from_bytes,
        },
    },
};
use std::io::Read;

// Represents a collection of transaction data.
#[derive(Debug)]
pub struct Transactions {
    command_name: String,
    confirmed_txs_send: Vec<WalletTx>,
    confirmed_txs_recv: Vec<WalletTx>,
    unconfirmed_txs_send: Vec<WalletTx>,
    unconfirmed_txs_recv: Vec<WalletTx>,
    utxo: Vec<(Vec<u8>, u32, TxOut)>,
    used_txouts: Vec<(TxOut, i64)>,
    last_update_time: u32,
}

impl Transactions {
    /// Creates a new instance of `Transactions`.
    ///
    /// # Arguments
    ///
    /// * `confirmed_txs_send`: A vector of `WalletTx` representing the confirmed outgoing transactions.
    /// * `confirmed_txs_recv`: A vector of `WalletTx` representing the confirmed incoming transactions.
    /// * `unconfirmed_txs_send`: A vector of `WalletTx` representing the unconfirmed outgoing transactions.
    /// * `unconfirmed_txs_recv`: A vector of `WalletTx` representing the unconfirmed incoming transactions.
    /// * `utxo`: A vector of tuples `(Vec<u8>, u32, TxOut)` representing the Unspent Transaction Outputs (UTXOs).
    /// * `last_update_time`: An unsigned integer representing the last time the transaction data was updated.
    ///
    /// # Returns
    ///
    /// A new instance of `Transactions` with the provided data.
    pub fn new(
        confirmed_txs_send: Vec<WalletTx>,
        confirmed_txs_recv: Vec<WalletTx>,
        unconfirmed_txs_send: Vec<WalletTx>,
        unconfirmed_txs_recv: Vec<WalletTx>,
        utxo: Vec<(Vec<u8>, u32, TxOut)>,
        used_txouts: Vec<(TxOut, i64)>,
        last_update_time: u32,
    ) -> Transactions {
        Transactions {
            command_name: "transactions".to_string(),
            confirmed_txs_send,
            confirmed_txs_recv,
            unconfirmed_txs_send,
            unconfirmed_txs_recv,
            utxo,
            used_txouts,
            last_update_time,
        }
    }

    /// Converts the `Transactions` struct to its byte representation.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the `Transactions` struct.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = fill_command(self.command_name.as_str()).as_bytes().to_vec();

        buffer.extend((self.confirmed_txs_send.len() as u32).to_le_bytes());
        for tx in self.confirmed_txs_send.iter() {
            buffer.extend(tx.to_bytes());
        }

        buffer.extend((self.confirmed_txs_recv.len() as u32).to_le_bytes());
        for tx in self.confirmed_txs_recv.iter() {
            buffer.extend(tx.to_bytes());
        }

        buffer.extend((self.unconfirmed_txs_send.len() as u32).to_le_bytes());
        for tx in self.unconfirmed_txs_send.iter() {
            buffer.extend(tx.to_bytes());
        }

        buffer.extend((self.unconfirmed_txs_recv.len() as u32).to_le_bytes());
        for tx in self.unconfirmed_txs_recv.iter() {
            buffer.extend(tx.to_bytes());
        }

        buffer.extend((self.utxo.len() as u32).to_le_bytes());
        for utxo in self.utxo.iter() {
            buffer.extend(&utxo.0);
            buffer.extend(utxo.1.to_le_bytes());
            buffer.extend(utxo.2.to_bytes());
        }

        buffer.extend((self.used_txouts.len() as u32).to_le_bytes());
        for (txout, amount) in self.used_txouts.iter() {
            buffer.extend(txout.to_bytes());
            buffer.extend(amount.to_le_bytes());
        }

        buffer.extend(self.last_update_time.to_le_bytes());

        buffer
    }

    /// Parses a byte stream into a `Transactions` struct.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a type that implements the `Read` trait, representing the byte stream to parse.
    ///
    /// # Returns
    ///
    /// A result containing the parsed `Transactions` struct if successful, or a `MessageError` if an error occurred during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Transactions, MessageError> {
        let mut confirmed_txs_send: Vec<WalletTx> = vec![];
        let mut confirmed_txs_recv: Vec<WalletTx> = vec![];
        let mut unconfirmed_txs_send: Vec<WalletTx> = vec![];
        let mut unconfirmed_txs_recv: Vec<WalletTx> = vec![];
        let mut utxo: Vec<(Vec<u8>, u32, TxOut)> = vec![];
        let mut used_txouts = vec![];

        let confirmed_send_count = read_u32_from_bytes(stream, true)?;
        for _ in 0..confirmed_send_count {
            confirmed_txs_send.push(WalletTx::from_bytes(stream)?);
        }

        let confirmed_recv_count = read_u32_from_bytes(stream, true)?;
        for _ in 0..confirmed_recv_count {
            confirmed_txs_recv.push(WalletTx::from_bytes(stream)?);
        }

        let unconfirmed_send_count = read_u32_from_bytes(stream, true)?;
        for _ in 0..unconfirmed_send_count {
            unconfirmed_txs_send.push(WalletTx::from_bytes(stream)?);
        }

        let unconfirmed_recv_count = read_u32_from_bytes(stream, true)?;
        for _ in 0..unconfirmed_recv_count {
            unconfirmed_txs_recv.push(WalletTx::from_bytes(stream)?);
        }

        let utxo_count = read_u32_from_bytes(stream, true)?;
        for _ in 0..utxo_count {
            let txid = read_vec_from_bytes(stream, 32)?;
            let index = read_u32_from_bytes(stream, true)?;
            let txout = TxOut::from_bytes(stream)?;

            utxo.push((txid, index, txout));
        }

        let used_txouts_count = read_u32_from_bytes(stream, true)?;
        for _ in 0..used_txouts_count {
            let txout = TxOut::from_bytes(stream)?;
            let amount = read_i64_from_bytes(stream, true)?;
            used_txouts.push((txout, amount));
        }

        let last_update_time = read_u32_from_bytes(stream, true)?;

        Ok(Transactions {
            command_name: "transactions".to_string(),
            confirmed_txs_send,
            confirmed_txs_recv,
            unconfirmed_txs_send,
            unconfirmed_txs_recv,
            utxo,
            used_txouts,
            last_update_time,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.confirmed_txs_recv.is_empty()
            && self.confirmed_txs_send.is_empty()
            && self.unconfirmed_txs_recv.is_empty()
            && self.unconfirmed_txs_send.is_empty()
    }

    pub fn get_confirmed_txs_send(&self) -> Vec<WalletTx> {
        self.confirmed_txs_send.clone()
    }

    pub fn get_confirmed_txs_recv(&self) -> Vec<WalletTx> {
        self.confirmed_txs_recv.clone()
    }

    pub fn get_unconfirmed_txs_send(&self) -> Vec<WalletTx> {
        self.unconfirmed_txs_send.clone()
    }

    pub fn get_unconfirmed_txs_recv(&self) -> Vec<WalletTx> {
        self.unconfirmed_txs_recv.clone()
    }

    pub fn get_utxo(&self) -> Vec<(Vec<u8>, u32, TxOut)> {
        self.utxo.clone()
    }

    pub fn get_used_txouts(&self) -> Vec<(TxOut, i64)> {
        self.used_txouts.clone()
    }

    pub fn get_last_update(&self) -> u32 {
        self.last_update_time
    }
}
