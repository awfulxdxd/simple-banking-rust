use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Account {
    id: u32,
    user_id: u32,
    balance: f64,
}

#[derive(Debug)]
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer { to_account_id: u32 },
}

#[derive(Debug)]
struct Transaction {
    id: u32,
    account_id: u32,
    transaction_type: TransactionType,
    amount: f64,
}

struct Bank {
    users: HashMap<u32, User>,
    accounts: HashMap<u32, Account>,
    transactions: Vec<Transaction>,
    next_user_id: u32,
    next_account_id: u32,
    next_transaction_id: u32,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            users: HashMap::new(),
            accounts: HashMap::new(),
            transactions: Vec::new(),
            next_user_id: 1,
            next_account_id: 1,
            next_transaction_id: 1,
        }
    }

    fn add_user(&mut self, name: String) -> u32 {
        let user_id = self.next_user_id;
        self.users.insert(user_id, User { id: user_id, name });
        self.next_user_id += 1;
        user_id
    }

    fn add_account(&mut self, user_id: u32) -> u32 {
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

    fn deposit(&mut self, account_id: u32, amount: f64) {
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

    fn withdraw(&mut self, account_id: u32, amount: f64) -> bool {
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

    fn transfer(&mut self, from_account_id: u32, to_account_id: u32, amount: f64) -> bool {
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

    fn get_user_accounts(&self, user_id: u32) -> Vec<&Account> {
        self.accounts
            .values()
            .filter(|&account| account.user_id == user_id)
            .collect()
    }
}

fn main() {
    println!("Hello! Welcome to our banking system!!!");

    let mut bank = Bank::new();
    let mut input = String::new();

    loop {
        println!("Please enter your name:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let name = input.trim().to_string();

        if name.is_empty() {
            println!("Name cannot be empty. Please try again.");
            continue;
        }

        let user_id = bank.add_user(name.clone());
        let account_id = bank.add_account(user_id);

        loop {
            println!("Hello, {}! Please choose an action:", name);
            println!("1. Deposit");
            println!("2. Withdraw");
            println!("3. Transfer");
            println!("4. Check balance");
            println!("5. Exit");

            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let choice: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number between 1 and 5.");
                    continue;
                }
            };

            match choice {
                1 => {
                    println!("Enter amount to deposit:");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let amount: f64 = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input. Please enter a valid amount.");
                            continue;
                        }
                    };
                    bank.deposit(account_id, amount);
                    println!("Deposited ${} to your account.", amount);
                }
                2 => {
                    println!("Enter amount to withdraw:");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let amount: f64 = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input. Please enter a valid amount.");
                            continue;
                        }
                    };
                    if bank.withdraw(account_id, amount) {
                        println!("Withdrew ${} from your account.", amount);
                    } else {
                        println!("Insufficient balance.");
                    }
                }
                3 => {
                    println!("Enter account ID to transfer to:");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let to_account_id: u32 = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input. Please enter a valid account ID.");
                            continue;
                        }
                    };
                    println!("Enter amount to transfer:");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let amount: f64 = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input. Please enter a valid amount.");
                            continue;
                        }
                    };
                    if bank.transfer(account_id, to_account_id, amount) {
                        println!("Transferred ${} to account {}.", amount, to_account_id);
                    } else {
                        println!("Transfer failed. Please check the details and try again.");
                    }
                }
                4 => {
                    if let Some(account) = bank.accounts.get(&account_id) {
                        println!("Your account balance is: ${:.2}", account.balance);
                    } else {
                        println!("Account not found.");
                    }
                }
                5 => {
                    println!("Goodbye, {}!", name);
                    return;
                }
                _ => {
                    println!("Invalid choice. Please enter a number between 1 and 5.");
                }
            }
        }
    }
}
