use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    block_mod::{blockchain::BlockChain, mempool::Mempool, utxo::UnspentTx},
    messages::{read_from_bytes::read_string_from_bytes, tx::Tx},
    network::broadcasting::broadcast_new_txn,
    proof_of_inclusion_mod::proof_of_inclusion::send_proof,
    settings_mod::settings::Settings,
    wallet_utils::{
        broadcast_txn::BroadcastTxn, get_proof::GetProof, get_transactions::GetTransactions,
        tx_filter::get_wallet_txns,
    },
};

use super::update_wallet_error::UpdateWalletError;

/// Updates the wallet by processing incoming commands from the connected `TcpStream`.
///
/// # Arguments
///
/// * `wallet` - The `TcpStream` representing the connection to the wallet.
/// * `blockchain` - A reference to the `BlockChain` wrapped in an `Arc<Mutex>`.
/// * `utxo` - A reference to the `UnspentTx` wrapped in an `Arc<Mutex>`.
/// * `mempool` - A reference to the `Mempool` wrapped in an `Arc<Mutex>`.
/// * `settings` - A reference to the `Settings` wrapped in an `Arc`.
/// * `streams` - A vector of `TcpStream` wrapped in `Arc<Mutex>`, representing connections to other nodes.
///
/// # Errors
///
/// Returns an `UpdateWalletError` if there is an error reading from or writing to the `TcpStream`,
/// parsing the incoming command, sending the proof, or broadcasting the transaction.
pub fn update_wallet(
    mut wallet: TcpStream,
    blockchain: Arc<Mutex<BlockChain>>,
    utxo: Arc<Mutex<UnspentTx>>,
    mempool: Arc<Mutex<Mempool>>,
    settings: Arc<Settings>,
    streams: Vec<Arc<Mutex<TcpStream>>>,
) -> Result<(), UpdateWalletError> {
    loop {
        let command_name =
            read_string_from_bytes(&mut wallet, 12).map_err(|_| UpdateWalletError::Read)?;

        match command_name.as_str() {
            "get_txs" => {
                println!("Message Get Transactions received.\n");

                let locked_utxo = match utxo.lock() {
                    Ok(locked_utxo) => locked_utxo,
                    Err(_) => {
                        println!("Failed to lock UTXO set.");
                        continue;
                    }
                };

                for (_, inner_map) in locked_utxo.get_utxo().iter() {
                    for (_, tx_out) in inner_map.iter() {
                        if tx_out.get_pk_script()
                            == [
                                118, 169, 20, 146, 119, 213, 38, 45, 53, 68, 169, 103, 41, 106, 12,
                                219, 70, 253, 172, 54, 247, 192, 132, 136, 172,
                            ]
                            .to_vec()
                        {
                            println!("\n\n{}\n\n", tx_out.get_value());
                        }
                    }
                }
                drop(locked_utxo);

                let get_transactions =
                    GetTransactions::from_bytes(command_name.to_string(), &mut wallet)
                        .map_err(|_| UpdateWalletError::Read)?;
                let transactions = get_wallet_txns(&blockchain, &utxo, &mempool, get_transactions)
                    .map_err(|_| UpdateWalletError::GetTxn)?;

                wallet
                    .write_all(&transactions.to_bytes())
                    .map_err(|_| UpdateWalletError::Write)?;
            }
            "get_proof" => {
                println!("Message Get Proof received.\n");
                let get_proof = GetProof::from_bytes(command_name.to_string(), &mut wallet)
                    .map_err(|_| UpdateWalletError::Read)?;
                send_proof(
                    get_proof.get_block_header(),
                    get_proof.get_tx_id(),
                    &blockchain,
                    &mut wallet,
                )
                .map_err(|_| UpdateWalletError::SendProof)?;
            }
            "broadcast_tx" => {
                println!("Message Broadcast Tx received\n.");

                let broadcast_txn = BroadcastTxn::from_bytes(command_name.to_string(), &mut wallet)
                    .map_err(|_| UpdateWalletError::Read)?;
                let tx_msg = Tx::new(settings.get_start_string(), broadcast_txn.get_txn());

                broadcast_new_txn(tx_msg, &streams).map_err(|_| UpdateWalletError::BroadcastTx)?;
            }
            _ => {}
        }
    }
}
