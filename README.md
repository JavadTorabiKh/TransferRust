# 🔗 Rust-Language Blockchain Transaction API

A blazing-fast and secure API built with **Rust** to create and submit transactions on a blockchain network.

> Perfect for powering wallets, dApps, backend services, or blockchain automation tools.

---

## 🚀 Features

- 🦀 Written in **Rust** for performance and safety
- 🔐 Secure transaction signing and submission
- 🌐 Connects directly with your blockchain node
- ⚙️ Easy to integrate into any Web3 stack
- 📡 RESTful API interface for simple interaction

---

## 📦 Tech Stack

- **Language:** Rust
- **Web Framework:** [Actix Web](https://actix.rs/)
- **Blockchain Support:** Compatible with networks like **Sui**, custom blockchains, or other Move-based chains
- **Serialization:** JSON (via `serde`)
- **Security:** Optional JWT or API key auth (configurable)

---

## 🛠️ Getting Started

### Prerequisites

- Rust (latest stable version)  
  Install via [rustup.rs](https://rustup.rs)
- Access to a blockchain node (e.g., a Sui full node or devnet node)

### Clone & Build

```bash
    git clone https://github.com/JavadTorabiKh/TransferRust.git
    cd rust-blockchain-api
    cargo build --release
```

### Run the API
```bash
    cargo run
```
The API will be available at:
http://localhost:8000

---

## 🔧 Configuration
You can configure the API via environment variables or a .env file:

```env
    BLOCKCHAIN_RPC_URL=https://fullnode.devnet.sui.io
    PRIVATE_KEY=your_private_key_here
    PORT=8000
```

---


## 📮 Example API Usage
Create Transaction
POST /transaction

Request Body:

```json
    {
    "recipient": "0xabc123...",
    "amount": 1000000,
    "token": "SUI"
    }
```
Response:

```json
    {
    "status": "success",
    "tx_hash": "0xdeadbeef1234567890"
    }
```

---


## 📁 Project Structure

```bash
    src/
    ├── main.rs          # Application entry point
    ├── handlers.rs      # API route handlers
    ├── blockchain.rs    # Blockchain interaction logic
    ├── models.rs        # Request/response models
    └── config.rs        # App configuration
```

---


## 🧪 Testing

```bash
    cargo test
```
You can also use Postman or curl to test endpoints manually.

---


## 🤝 Contributing
Pull requests are welcome! If you’d like to help improve this project, feel free to fork it and submit a PR.

---


## 📜 License
Licensed under the MIT License.

📬 Contact
Created by @your-username
Feel free to reach out for questions, suggestions, or collaborations!
