# 🚀 Blockchain Transaction Generator [Rust]

A blazing-fast tool to create and broadcast blockchain transactions using the power of **Rust** 🦀.

---

## 🔧 Getting Started

Before running the project, make sure you have the following installed:

- 🦀 **Rust** (Stable): [Install Rust](https://www.rust-lang.org/tools/install)
- 📦 **Cargo**: Comes with Rust
- 🌐 Access to a blockchain node (HTTP or WebSocket RPC)

---

## ⚙️ Installation & Usage

```bash
    # Clone the repository
    git clone https://github.com/JavadTorabiKh/TransferRust.git

    # Change into the project directory
    cd blockchain-tx-rust

    # Build and run the project
    cargo run
```

---

## ✨ Features

- Generate and sign blockchain transactions
- Connect to any blockchain node via HTTP/WebSocket
- Fully configurable (network, keys, gas, etc.)
- Lightweight, secure, and written in pure Rust

---

## 🧪 Example Usage
```bash
    cargo run -- \
    --network https://mainnet.infura.io/v3/YOUR_API_KEY \
    --from 0xYourAddress \
    --to 0xRecipientAddress \
    --amount 0.05 \
    --private-key YOUR_PRIVATE_KEY
```

✅ Supports both mainnet and testnets (e.g., Goerli, Sepolia).
⚠️ Always test on a testnet before using mainnet funds!

---