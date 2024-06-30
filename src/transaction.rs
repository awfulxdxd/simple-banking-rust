#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer { to_account_id: u32 },
}

#[derive(Debug)]
pub struct Transaction {
    pub id: u32,
    pub account_id: u32,
    pub transaction_type: TransactionType,
    pub amount: f64,
}
