use std::{sync::{Mutex, Arc}};
use gtk::{Builder, prelude::BuilderExtManual, Label, LabelExt};

use node::{block_mod::{transaction::Transaction, tx_out::TxOut}, messages::read_from_bytes::encode_hex, wallet_utils::wallet_tx::WalletTx};

use glib::{Receiver, clone, Continue};
use gtk::{ListStore, prelude::GtkListStoreExtManual, GtkListStoreExt};

use crate::{accounts::Accounts, interface_error::InterfaceError};
use super::{create_transactions::pk_script_from_pubkey, create_transactios_constants::*};

/// Calculates the received balance for a specific public key script in a transaction.
///
/// This function takes a transaction and a public key script as input and calculates
/// the total received balance for that specific public key script in the transaction.
/// It iterates through each transaction output (tx_out) in the transaction and checks
/// if the provided public key script matches the tx_out's script. If there is a match,
/// the value of the tx_out is added to the balance.
///
/// # Arguments
///
/// * `transaction` - The transaction to calculate the received balance from.
/// * `pk_script` - The public key script to match against the transaction outputs.
///
/// # Returns
///
/// The received balance for the specified public key script in the transaction,
/// expressed as a floating-point value in BTC.
pub fn received_balance(transaction: &Transaction, pk_script: &Vec<u8>) -> f64 {
    let mut balance = 0;

    for tx_out in transaction.get_tx_out_list() {
        if *pk_script == tx_out.get_pk_script() {
            balance += tx_out.get_value();
        }
    }
    balance as f64 / BTC_TO_SATOSHI
}

/// Calculates the sent balance for a specific public key script in a transaction.
///
/// This function takes a transaction and a public key script as input and calculates
/// the total sent balance for that specific public key script in the transaction.
/// It iterates through each transaction output (tx_out) in the transaction and checks
/// if the provided public key script does not match the tx_out's script. If there is no
/// match, indicating that the funds are being sent from the specified script, the value
/// of the tx_out is added to the balance.
///
/// # Arguments
///
/// * `transaction` - The transaction to calculate the sent balance from.
/// * `pk_script` - The public key script to match against the transaction outputs.
///
/// # Returns
///
/// The sent balance for the specified public key script in the transaction,
/// expressed as a floating-point value in BTC.
pub fn send_balance(transaction: &Transaction, pk_script: &[u8]) -> f64 {
    let mut balance = 0;

    for tx_out in transaction.get_tx_out_list() {
        if *pk_script != tx_out.get_pk_script() {
            balance += tx_out.get_value();
        }
    }
    
    balance as f64 / BTC_TO_SATOSHI
}

/// Calculates the total available funds from a list of UTXOs.
///
/// This function computes the total available funds from a provided list of unspent transaction outputs (UTXOs).
///
/// # Arguments
///
/// * `utxo`: A vector of tuples containing UTXO information (address, index, and TxOut).
///
/// # Returns
///
/// The total available funds as a `f64` value in Bitcoin.
fn available_funds(utxo: Vec<(Vec<u8>, u32, TxOut)>) -> f64{
    let mut value = 0;

    for txout in utxo{
        value += txout.2.get_value();
    }

    value as f64 / BTC_TO_SATOSHI
}

/// Calculates the total pending funds from a list of used transaction outputs.
///
/// This function computes the total pending funds from a provided list of used transaction outputs.
///
/// # Arguments
///
/// * `used_txouts`: A vector of tuples containing used TxOuts and corresponding amounts.
///
/// # Returns
///
/// The total pending funds as a `f64` value in Bitcoin.
fn pending_funds(used_txouts: Vec<(TxOut, i64)>) -> f64{
    let mut value = 0.0;

    for (_, amount) in used_txouts{
        value += amount as f64;
    }

    value / BTC_TO_SATOSHI
}

/// Updates the transaction list view for sent transactions.
///
/// This function takes a vector of `WalletTx` representing sent transactions, a reference to a `ListStore`
/// where the transaction data is displayed, the current state, transaction type, and the public key script
/// of the recipient. It iterates through each transaction, retrieves its ID, reverses it, and inserts the
/// transaction data into the list store. The transaction state, date, type, encoded ID, and the sent balance
/// are inserted into the corresponding columns of the list store.
///
/// # Arguments
///
/// * `transactions` - The vector of sent transactions to update the list view.
/// * `store` - The reference to the `ListStore` where the transaction data is displayed.
/// * `state` - The current state of the transactions.
/// * `tx_type` - The type of the transactions.
/// * `pub_key` - The public key script of the recipient.
///
/// # Returns
///
/// Returns `Ok(())` if the update is successful, or an `InterfaceError` if there is an error
/// encoding the transaction ID as hex.
fn update_tx_send(transactions: &Vec<WalletTx>, store: &ListStore, state: &str, tx_type: &str, pub_key: &[u8])-> Result<(), InterfaceError> {
    for tx in transactions {
        let mut txn = tx.get_tx().get_id(false);
        txn.reverse();
        
        store.insert_with_values(None, &[0,1,2,3,4], &[
            &state,
            &tx.get_date(),
            &tx_type,
            &encode_hex(&txn).map_err(|_| InterfaceError::DecodeHex)?,
            &format!("-{}", send_balance(tx.get_tx(), pub_key))
        ]);
    }
    Ok(()) 
}

/// Updates the transaction list view for received transactions.
///
/// This function takes a vector of `WalletTx` representing received transactions, a reference to a `ListStore`
/// where the transaction data is displayed, the current state, transaction type, and the public key script
/// of the recipient. It iterates through each transaction, retrieves its ID, reverses it, and inserts the
/// transaction data into the list store. The transaction state, date, type, encoded ID, and the received balance
/// are inserted into the corresponding columns of the list store.
///
/// # Arguments
///
/// * `transactions` - The vector of received transactions to update the list view.
/// * `store` - The reference to the `ListStore` where the transaction data is displayed.
/// * `state` - The current state of the transactions.
/// * `tx_type` - The type of the transactions.
/// * `pk_script` - The public key script of the recipient.
///
/// # Returns
///
/// Returns `Ok(())` if the update is successful, or an `InterfaceError` if there is an error
/// encoding the transaction ID as hex.
fn update_tx_recv(transactions: &Vec<WalletTx>, store: &ListStore, state: &str, tx_type: &str, pk_script: &Vec<u8>)-> Result<(), InterfaceError> {
    for tx in transactions {
        let mut txn = tx.get_tx().get_id(false);
        txn.reverse();
        
        store.insert_with_values(None, &[0,1,2,3,4], &[
            &state,
            &tx.get_date(),
            &tx_type,
            &encode_hex(&txn).map_err(|_| InterfaceError::DecodeHex)?,
            &format!("{}", received_balance(tx.get_tx(), pk_script))
        ]);
    } 
    Ok(())
}

/// Updates the transaction list view for all transactions.
///
/// This function takes a `ListStore` where the transaction data is displayed and a shared reference
/// to the `Accounts` structure. It clears the list store and retrieves the actual account from the
/// locked accounts. It obtains the public key and generates the public key script. It then calls the
/// `update_tx_send` and `update_tx_recv` functions to update the transaction list view for confirmed
/// and unconfirmed sent and received transactions. Finally, it drops the lock on the accounts and
/// returns `Ok(())` if the update is successful.
///
/// # Arguments
///
/// * `store` - The `ListStore` where the transaction data is displayed.
/// * `accounts` - The shared reference to the `Accounts` structure.
///
/// # Returns
///
/// Returns `Ok(())` if the update is successful, or an `InterfaceError` if there is an error acquiring
/// or releasing the lock on the accounts.
pub fn update_transactions(store: ListStore, accounts: Arc<Mutex<Accounts>>) -> Result<(), InterfaceError>{
    let locked_accounts = accounts.lock().map_err(|_| InterfaceError::LockAccounts)?;
    store.clear();

    let actual_account = locked_accounts.get_current_account_info().ok_or(InterfaceError::LockAccounts)?;
    let pub_key = actual_account.get_public_key();
    let pk_script = pk_script_from_pubkey(&pub_key, actual_account.get_bech32());

    update_tx_send(actual_account.get_confirmed_txs_send(), &store, CONFIRMED, SENT, &pk_script)?;
    update_tx_recv(actual_account.get_confirmed_txs_recv(), &store, CONFIRMED, RECEIVED, &pk_script)?;

    update_tx_send(actual_account.get_unconfirmed_txs_send(), &store, UNCONFIRMED, SENT, &pk_script)?;
    update_tx_recv(actual_account.get_unconfirmed_txs_recv(), &store, UNCONFIRMED, RECEIVED, &pk_script)?;

    drop(locked_accounts);
    Ok(())
}

/// Updates the balance labels in the user interface.
///
/// This function updates the available, pending, and total balance labels in the user interface
/// based on the account information. It takes the labels and the shared `Accounts` object as input.
/// It retrieves the actual account from the `Accounts` object and calculates the available and pending
/// balances using the `get_balance` function. The total balance is computed as the sum of the available
/// and pending balances. The labels are then updated with the respective balance values.
///
/// # Arguments
///
/// * `available` - The label for the available balance.
/// * `pending` - The label for the pending balance.
/// * `total` - The label for the total balance.
/// * `accounts` - The shared `Accounts` object.
///
/// # Returns
///
/// Returns `Ok(())` if the balance labels are successfully updated, or an `InterfaceError` if there
/// is an error acquiring the lock on the `Accounts` object.
fn update_balance_labels(available: &Label, pending: &Label, total: &Label, accounts: Arc<Mutex<Accounts>>) -> Result<(), InterfaceError>{
    let locked_accounts = accounts.lock().map_err(|_| InterfaceError::LockAccounts)?;
    let mut available_value = 0.0;
    let mut pending_value = 0.0;

    if let Some(user) = locked_accounts.get_current_account_info() {
        //available_value = get_balance(user.get_confirmed_txs_recv(), user.get_confirmed_txs_send(), &pk_script);
        available_value = available_funds(user.get_utxo());

        //pending_value = get_balance(user.get_unconfirmed_txs_recv(), user.get_unconfirmed_txs_send(), &pk_script);
        pending_value = pending_funds(user.get_used_txouts());
    }

    available.set_text(&format!("{} BTC", available_value));
    pending.set_text(&format!("{} BTC", pending_value));
    total.set_text(&format!("{} BTC", available_value + pending_value)); 

    drop(locked_accounts);
    Ok(())
}

/// Updates the transaction list and balance labels in the user interface.
///
/// This function updates the transaction list and balance labels in the user interface based on
/// the account information. It takes the builder, the list store, the shared `Accounts` object,
/// and a receiver for transaction update signals as input. It retrieves the labels for the available,
/// pending, and total balances from the builder. It then attaches a signal handler to the transaction
/// update receiver. Whenever a transaction update signal is received, the function calls the
/// `update_transactions` function to update the transaction list in the user interface, and the
/// `update_balance_labels` function to update the balance labels. If there is any error during the
/// update process, the function returns `Continue(false)` to stop further signal processing.
/// Otherwise, it returns `Continue(true)` to continue processing subsequent signals.
///
/// # Arguments
///
/// * `builder` - The builder object containing the user interface elements.
/// * `store` - The list store for the transaction list.
/// * `accounts` - The shared `Accounts` object.
/// * `txs_recv` - The receiver for transaction update signals.
///
/// # Returns
///
/// Returns `Ok(())` if the transaction list and balance labels are successfully updated, or an
/// `InterfaceError` if there is an error retrieving the necessary user interface elements or
/// during the update process.
pub fn update_transaction_list(builder: &Builder, store: ListStore, accounts: Arc<Mutex<Accounts>>, txs_recv: Receiver<bool>)-> Result<(), InterfaceError>{
    let available: Label = builder.get_object(DISPONIBLE_VALUE).ok_or(InterfaceError::MissingLabel)?;
    let pending: Label= builder.get_object(PENDIENTE_VALUE).ok_or(InterfaceError::MissingLabel)?;
    let total: Label= builder.get_object(TOTAL_VALUE).ok_or(InterfaceError::MissingLabel)?;

    txs_recv.attach(
        None,
        clone!(@weak store => @default-return Continue(false),
            move |_| {
                if update_transactions(store, accounts.clone()).is_err() || update_balance_labels(&available, &pending, &total, accounts.clone()).is_err(){
                    return Continue(false);
                }

                Continue(true)
            }
        ),
    );

    Ok(())
}
