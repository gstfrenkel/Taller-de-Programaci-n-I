use std::collections::HashMap;
use crate::user_info::UserInfo;
use bitcoin::wallet_utils::transactions::Transactions;

#[derive(Debug)]
/// Represents a collection of user accounts and tracks the currently active account.
pub struct Accounts {
    accounts: HashMap<String, UserInfo>,
    actual_username: String
}

impl Accounts{
    pub fn new() -> Accounts{
        Accounts{
            accounts: HashMap::new(),
            actual_username: String::default()
        }
    }

    pub fn add_account(&mut self, user_name: String, public_key: Vec<u8>, private_key: Vec<u8>, address: String) {

        self.accounts.insert(user_name.clone(), UserInfo::new(public_key, private_key, address));
        self.actual_username = user_name;
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub fn get_actual_account(&self) -> Option<&UserInfo>{
        self.accounts.get(&self.actual_username)
    }

    pub fn update(&mut self, transactions: &Transactions) {// en una de esas no habría que actualizar todos los accounts?
        if let Some(user_info) = self.accounts.get_mut(&self.actual_username) {
            user_info.update(transactions);
        }
    }

    pub fn get_accounts_count(&self)-> usize {
        self.accounts.iter().len() 
    }

    pub fn set_actual_account(&mut self, active_account: String) {
        if self.accounts.get(&active_account).is_some(){
            self.actual_username = active_account;
        }
    }
}

impl Default for Accounts {
    fn default() -> Self {
        Self::new()
    }
}
    