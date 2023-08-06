use std::{time::Duration, thread, sync::{Mutex, Arc}, net::TcpStream, io::Write};
use node::{wallet_utils::{get_transactions::GetTransactions, transactions::Transactions}, messages::read_from_bytes::read_string_from_bytes};
use crate::{transactions::create_transactions::pk_script_from_pubkey, accounts::Accounts, interface_error::InterfaceError};

/// Updates the wallet by retrieving and processing transactions from the node.
///
/// This function continuously loops and updates the wallet by retrieving and processing
/// transactions from the node. It takes the shared `Accounts` object, the shared `TcpStream`
/// representing the connection to the node, and a sender for transaction update signals as input.
/// Within the loop, it locks the `Accounts` object to access the current user's information. If
/// there is an actual account, it retrieves the necessary information, such as the public key and
/// the last update timestamp. It then locks the `TcpStream` to communicate with the node and
/// requests transactions using the `GetTransactions` command. The retrieved transactions are
/// processed and updated in the user's account. Finally, a transaction update signal is sent using
/// the provided sender. The loop continues to execute after a brief sleep of 10 seconds.
///
/// # Arguments
///
/// * `accounts` - The shared `Accounts` object.
/// * `node` - The shared `TcpStream` representing the connection to the node.
/// * `txs_sender` - The sender for transaction update signals.
///
/// # Returns
///
/// Returns `Ok(())` if the wallet is successfully updated, or an `InterfaceError` if there is an
/// error while retrieving transactions, processing them, or sending the transaction update signal.
pub fn update_wallet(accounts: Arc<Mutex<Accounts>>, node: Arc<Mutex<TcpStream>>, txs_sender: glib::Sender<bool>) -> Result<(), InterfaceError>{
    loop {
        let mut locked_accounts = accounts.lock().map_err(|_| InterfaceError::LockAccounts)?;

        if let Some(user_info) = locked_accounts.get_current_account_info() {
            let mut locked_node = node.lock().map_err(|_| InterfaceError::LockNode)?;
            let pk_script = pk_script_from_pubkey(&user_info.get_public_key(), user_info.get_bech32());
            let get_transactions = GetTransactions::new(pk_script.clone(), user_info.get_public_key(), user_info.get_last_update());

            locked_node.write_all(&get_transactions.as_bytes()).map_err(|_| InterfaceError::Write)?;

            let _ = read_string_from_bytes(&mut *locked_node, 12).map_err(|_| InterfaceError::Read)?;
            let transactions = Transactions::from_bytes(&mut *locked_node).map_err(|_| InterfaceError::Read)?;
            
            drop(locked_node);

            if !transactions.is_empty(){
                println!("\n------------------------------------------------------------\n{}'s account with public key {:?}\n", locked_accounts.get_current_username(), &user_info.get_public_key());
                println!("Get Transaction message sent with public key script: {:?}", pk_script);
            }

            locked_accounts.update(&transactions);
            
            txs_sender.send(true).map_err(|_| InterfaceError::Send)?;
        }
        
        drop(locked_accounts);
        thread::sleep(Duration::from_secs(5));
    }
}
