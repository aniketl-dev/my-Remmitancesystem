# 🌍 Stellar Soroban Remittance System

## 📌 Project Description

This project is a basic remittance system built using Soroban smart contracts on the Stellar blockchain. It enables users to securely send and record payments in a decentralized environment without relying on traditional intermediaries.

The goal of this project is to demonstrate how blockchain-based smart contracts can simplify cross-border remittance systems by ensuring transparency, security, and low transaction costs.

<img width="1920" height="1080" alt="Screenshot 2026-04-13 140929" src="https://github.com/user-attachments/assets/2e7699ff-5e08-408c-af5e-1088728c9c32" />

---

## 🚀 What it does

* Allows users to send payments to another wallet address
* Records transaction details (sender, receiver, amount) on-chain
* Stores transaction history securely in contract storage
* Enables retrieval of all previous transactions
* Provides a basic balance check function (for demonstration)

---

## ✨ Features

* 🔐 Secure transactions with sender authentication
* ⚡ Fast execution powered by Stellar Soroban
* 📜 Persistent on-chain transaction history
* 🧩 Type-safe implementation using Rust structs
* 🌐 Ready for cross-border remittance use cases
* 💡 Simple and beginner-friendly smart contract design

---

## 🛠️ Smart Contract Functions

### `send_payment(from, to, amount)`

Records a payment transaction from sender to receiver.

### `get_transactions()`

Fetches all stored remittance transactions from the contract.

### `balance_of(user)`

Returns a mock balance value (used for demonstration purposes).

---

## 🔗 Deployed Smart Contract Link

**Contract Address:**
CAALZQZR4PBZ7GULUBP5CNYKOPNT6ITLMHORM5PCTHVNVM37P4XW3JRP

---

## 🧪 Tech Stack

* Stellar Blockchain
* Soroban Smart Contracts
* Rust Programming Language

---

## 📦 Project Structure

contracts/
└── contract/
    ├── src/
    │    └── lib.rs
    └── Cargo.toml

README.md

---

## ⚙️ How to Build & Deploy

### 1. Install Soroban CLI

Follow official Stellar documentation to install Soroban tools.

### 2. Build the Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Deploy the Contract

```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/contract.wasm
```

---

## 📈 Future Improvements

* 💸 Integrate real token transfers (Stellar assets)
* 🆔 Add unique transaction IDs
* 🔒 Implement escrow-based remittance
* 📱 Build a frontend interface
* 🌍 Multi-currency support
* 🔔 Transaction notifications

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork the project and submit a pull request.

---

## 📜 License

This project is open-source and available under the MIT License.
