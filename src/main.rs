mod account;
mod bank;
mod transaction;
mod user;

use bank::Bank;
use std::io;

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
