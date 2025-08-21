# Full-Stack Solana dApp with Rust Backend

## Overview

This project is a complete, full-stack decentralized application built on the Solana blockchain. It demonstrates a robust architecture featuring an on-chain smart contract, a decoupled backend service layer, and a client interface. The entire stack is built with Rust, showcasing a modern, high-performance approach to Web3 development.

The core functionality is a simple staking program where users can theoretically stake SPL tokens. The project's primary purpose is to serve as a portfolio piece that showcases the end-to-end development lifecycle of a Web3 application, from local testing to live Devnet deployment.

## Technical Architecture

The application operates with a clear, three-tier architecture:

```
+------------------+      +-------------------------+      +-------------------+      +----------------------+
|                  |      |                         |      |                   |      |                      |
|  CLI Frontend    |----->|  Actix-web Backend API  |----->|  Solana Devnet RPC  |----->|  Anchor Smart Contract |
|  (Rust)          |      |  (Rust)                 |      |  (Public Node)    |      |  (On-Chain Program)  |
|                  |      |                         |      |                   |      |                      |
+------------------+      +-------------------------+      +-------------------+      +----------------------+
       |                          |                                                            |
(Sends HTTP requests)      (Exposes /stake, /unstake, /balance)                       (Holds staking logic & state)
```

## Key Technical Skills Showcased

- **Languages**: Rust
- **Backend framework**: Actix-web (Asynchronous, RESTful API development)
- **Blockchain framework**: Anchor (Solana smart contract development)
- **Blockchain interaction**: Solana JSON RPC, solana-client, spl-token CLI
- **Core concepts**: Asynchronous programming (async/await), environment variables (dotenv), API Design, blockchain transactions, On-chain vs. Off-chain logic, SPL Tokens
- **Tooling**: Cargo, Solana CLI, WSL (Windows Subsystem for Linux)

## Live Devnet Program

The smart contract for this project is live and verifiable on the Solana Devnet.

**Program ID**: `7dg8dMgyMHuY5zJ94AeRF8BbucZTDeioSJ76eWMRSPWd`

You can view the program and its deployment transaction on the [Solana Explorer](https://explorer.solana.com/address/7dg8dMgyMHuY5zJ94AeRF8BbucZTDeioSJ76eWMRSPWd?cluster=devnet).

## Project Showcase

### Step 1: Create a New Token to Stake

First, we use the spl-token CLI to create a new type of "dummy" token on Devnet that we can use for staking.

```bash
spl-token create-token
```

![Create Token](screenshots/token-creation.png)

### Step 2: Create an Account to Hold the New Tokens

We create an associated token account in our wallet to hold the tokens we're about to mint.

```bash
spl-token create-account <TOKEN_MINT_ADDRESS>
```

![Create Token Account](screenshots/token-account.png)

### Step 3: Mint Tokens to Our Account

We mint 1,000 of our new tokens to our wallet, giving us something to stake.

```bash
spl-token mint <TOKEN_MINT_ADDRESS> 1000
```

![Mint Tokens](screenshots/token-minting.png)

### Step 4: "Stake" Tokens Using the API

We call the `/stake` endpoint on our backend via the CLI. The backend acknowledges the request and returns a success message.

```bash
cargo run -- stake --user $(solana address) --amount 100
```

![Stake Tokens](screenshots/staking.png)

### Step 5: "Unstake" Tokens Using the API

Finally, we call the `/unstake` endpoint, proving the final piece of the API is functional.

```bash
cargo run -- unstake --user $(solana address) --amount 50
```

![Unstake Tokens](screenshots/unstaking.png)

## Installation and Setup

### Prerequisites

- Rust and Cargo (latest stable version)
- Solana CLI tools
- Node.js and npm (for optional frontend components)
- Git

### Getting Started

1. Clone the repository:
```bash
git clone <repository-url>
cd solana-dapp-staking
```

2. Install dependencies:
```bash
cargo build
```

3. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Build and deploy the smart contract:
```bash
anchor build
anchor deploy
```

5. Start the backend server:
```bash
cargo run --bin server
```

6. Use the CLI interface:
```bash
cargo run -- --help
```

## API Endpoints

The backend exposes the following RESTful endpoints:

- `POST /stake` - Stake tokens
- `POST /unstake` - Unstake tokens
- `GET /balance/{user_address}` - Check staked balance

## Project Structure

```
├── programs/                 # Solana programs (smart contracts)
├── src/
│   ├── bin/
│   │   ├── server.rs        # Backend API server
│   │   └── cli.rs           # Command-line interface
│   └── lib.rs               # Shared library code
├── tests/                   # Integration tests
├── migrations/              # Deployment scripts
└── scripts/                 # Utility scripts
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Solana Labs for the excellent developer tools
- The Anchor framework team
- The Rust community for incredible resources and support
