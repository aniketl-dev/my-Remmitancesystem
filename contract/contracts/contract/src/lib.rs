#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, Address, Vec,
};

#[contract]
pub struct RemittanceContract;

// Define a transaction struct (FIX)
#[contracttype]
#[derive(Clone)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[contractimpl]
impl RemittanceContract {

    // Send payment
    pub fn send_payment(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        // Authenticate sender
        from.require_auth();

        // Create transaction struct
        let tx = Transaction {
            from: from.clone(),
            to: to.clone(),
            amount,
        };

        // Storage key
        let key = symbol_short!("txs");

        // Get existing transactions
        let mut txs: Vec<Transaction> =
            env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        // Add new transaction
        txs.push_back(tx);

        // Save back
        env.storage().instance().set(&key, &txs);
    }

    // Get all transactions
    pub fn get_transactions(env: Env) -> Vec<Transaction> {
        let key = symbol_short!("txs");
        env.storage().instance().get(&key).unwrap_or(Vec::new(&env))
    }

    // Dummy balance function
    pub fn balance_of(_env: Env, user: Address) -> i128 {
        user.require_auth();

        // Placeholder balance
        1000
    }
}