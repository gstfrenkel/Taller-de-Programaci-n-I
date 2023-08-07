use crate::user_info::UserInfo;
use node::wallet_utils::transactions::Transactions;
use std::collections::HashMap;

#[derive(Debug)]
/// Represents a collection of user accounts and tracks the currently active account.
pub struct Accounts {
    accounts: HashMap<String, UserInfo>,
    current_username: String,
}

impl Accounts {
    pub fn new() -> Accounts {
        Accounts {
            accounts: HashMap::new(),
            current_username: String::default(),
        }
    }

    pub fn add_account(
        &mut self,
        user_name: String,
        public_key: Vec<u8>,
        private_key: Vec<u8>,
        bech32: bool,
    ) {
        self.accounts.insert(
            user_name.clone(),
            UserInfo::new(public_key, private_key, bech32),
        );
        self.current_username = user_name;
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub fn get_current_account_info(&self) -> Option<&UserInfo> {
        self.accounts.get(self.get_current_username())
    }

    pub fn get_current_username(&self) -> &String {
        &self.current_username
    }

    pub fn update(&mut self, transactions: &Transactions) {
        // en una de esas no habría que actualizar todos los accounts?
        if let Some(user_info) = self.accounts.get_mut(&self.current_username) {
            user_info.update(transactions);
        }
    }

    pub fn get_accounts_count(&self) -> usize {
        self.accounts.iter().len()
    }

    pub fn set_actual_account(&mut self, active_account: String) {
        if self.accounts.get(&active_account).is_some() {
            self.current_username = active_account;
        }
    }
}

impl Default for Accounts {
    fn default() -> Self {
        Self::new()
    }
}
