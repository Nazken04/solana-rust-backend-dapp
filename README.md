Full-Stack Solana dApp with a Rust Backend
Overview
This project is a complete, full-stack decentralized application built on the Solana blockchain. It demonstrates a robust architecture featuring an on-chain smart contract, a decoupled backend service layer, and a client interface. The entire stack is built with Rust, showcasing a modern, high-performance approach to Web3 development.
The core functionality is a simple staking program where users can theoretically stake SPL tokens. The project's primary purpose is to serve as a portfolio piece that showcases the end-to-end development lifecycle of a Web3 application, from local testing to live Devnet deployment.

Technical architecture
The application operates with a clear, three-tier architecture:
+------------------+      +-------------------------+      +-------------------+      +----------------------+
|                  |      |                         |      |                   |      |                      |
|  CLI Frontend    |----->|  Actix-web Backend API  |----->|  Solana Devnet RPC  |----->|  Anchor Smart Contract |
|  (Rust)          |      |  (Rust)                 |      |  (Public Node)    |      |  (On-Chain Program)  |
|                  |      |                         |      |                   |      |                      |
+------------------+      +-------------------------+      +-------------------+      +----------------------+
       |                          |                                                            |
(Sends HTTP rquests)      (Exposes /stake, /unstake, /balance)                       (Holds staking logic & state)


Key technical skills showcased
Languages: Rust
Backend framework: Actix-web (Asynchronous, RESTful API development)
Blockchain framework: Anchor (Solana smart contract development)
Blockchain interaction: Solana JSON RPC, solana-client, spl-token CLI
Core concepts: Asynchronous programming (async/await), environment vriables (dotenv), API Design, blockchain transactions, On-chain vs. Off-chain logic, SPL Tokens.
Tooling: Cargo, Solana CLI, WSL (Windows Subsystem for Linux)


Live devnet program
The smart contract for this project is live and verifiable on the Solana Devnet.
Program ID: 7dg8dMgyMHuY5zJ94AeRF8BbucZTDeioSJ76eWMRSPWd
You can view the program and its deployment transaction on the Solana Explorer.


Project showcase: 

Step 1: Create a new token to stake
First, we use the spl-token CLI to create a new type of "dummy" token on Devnet that we can use for staking.
code
!(Снимок экрана 2025-08-21 183437.png)

Step 2: Create an account to hold the new tokens
We create an associated token account in our wallet to hold the tokens we're about to mint.
code
spl-token create-account <TOKEN_MINT_ADDRESS>
!(Снимок экрана 2025-08-21 183512.png)


Step 3: Mint tokens to our account
We mint 1,000 of our new tokens to our wallet, giving us something to stake.
code
spl-token mint <TOKEN_MINT_ADDRESS> 1000
!(Снимок экрана 2025-08-21 183530.png)


Step 4: "Stake" tokens using the API
We call the /stake endpoint on our backend via the CLI. The backend acknowledges the request and returns a success message.
code
cargo run -- stake --user $(solana address) --amount 100
!(Снимок экрана 2025-08-21 183702.png)


Step 6: "Unstake" tokens using the API
Finally, we call the /unstake endpoint, proving the final piece of the API is functional.
code
!(Снимок экрана 2025-08-21 183720.png)
