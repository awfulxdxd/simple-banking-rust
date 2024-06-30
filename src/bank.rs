use crate::account::Account;
use crate::transaction::{Transaction, TransactionType};
use crate::user::User;
use std::collections::HashMap;

pub struct Bank {
    pub users: HashMap<u32, User>,
    pub accounts: HashMap<u32, Account>,
    pub transactions: Vec<Transaction>,
    pub next_user_id: u32,
    pub next_account_id: u32,
    pub next_transaction_id: u32,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            users: HashMap::new(),
            accounts: HashMap::new(),
            transactions: Vec::new(),
            next_user_id: 1,
            next_account_id: 1,
            next_transaction_id: 1,
        }
    }

    pub fn add_user(&mut self, name: String) -> u32 {
        let user_id = self.next_user_id;
        self.users.insert(user_id, User { id: user_id, name });
        self.next_user_id += 1;
        user_id
    }

    pub fn add_account(&mut self, user_id: u32) -> u32 {
        let account_id = self.next_account_id;
        self.accounts.insert(
            account_id,
            Account {
                id: account_id,
                user_id,
                balance: 0.0,
            },
        );
        self.next_account_id += 1;
        account_id
    }

    pub fn deposit(&mut self, account_id: u32, amount: f64) {
        if let Some(account) = self.accounts.get_mut(&account_id) {
            account.balance += amount;
            self.transactions.push(Transaction {
                id: self.next_transaction_id,
                account_id,
                transaction_type: TransactionType::Deposit,
                amount,
            });
            self.next_transaction_id += 1;
        }
    }

    pub fn withdraw(&mut self, account_id: u32, amount: f64) -> bool {
        if let Some(account) = self.accounts.get_mut(&account_id) {
            if account.balance >= amount {
                account.balance -= amount;
                self.transactions.push(Transaction {
                    id: self.next_transaction_id,
                    account_id,
                    transaction_type: TransactionType::Withdrawal,
                    amount,
                });
                self.next_transaction_id += 1;
                return true;
            }
        }
        false
    }

    pub fn transfer(&mut self, from_account_id: u32, to_account_id: u32, amount: f64) -> bool {
        if self.withdraw(from_account_id, amount) {
            self.deposit(to_account_id, amount);
            self.transactions.push(Transaction {
                id: self.next_transaction_id,
                account_id: from_account_id,
                transaction_type: TransactionType::Transfer { to_account_id },
                amount,
            });
            self.next_transaction_id += 1;
            return true;
        }
        false
    }

    pub fn get_user_accounts(&self, user_id: u32) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|&account| account.user_id == user_id)
            .collect()
    }
}
