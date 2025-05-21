xuta.project{**AT**}gmail.com

Xuta is a Web3 dApp built to tokenize real-world athlete equity using smart contracts and burn-based financial mechanics.
We partner with licensed football agents to issue Player ICOs, where users purchase on-chain tokens representing a share of a player’s future transfer value. Tokens are burned to claim earnings, and tradable on secondary markets.

The platform is built on Solana using smart contracts for agent-player agreements, token issuance, and payout mechanics. Compliance with FIFA's third-party ownership ban is maintained through legal structuring and smart contract enforcement.

Xuta bridges real-world assets and DeFi with a focus on scalability, security, and fan engagement.

--

🚀 Getting Started

(Instalation for BE and FE are each in their own repos)

Follow the steps below to set up, build, and deploy the XutaSC smart contracts using the Anchor framework on Solana.

✅ Prerequisites
Make sure you have the following installed:

Rust

Node.js and Yarn

Solana CLI

Anchor

# Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Anchor

cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked


📦 Install Dependencies

git clone https://github.com/FilipeBispo/XutaSC.git

cd XutaSC

yarn install


🔧 Configuration

Update the Anchor.toml file to match your deployment target:

toml

[provider]

cluster = "https://api.devnet.solana.com"  # Or "localhost", "mainnet", etc.

wallet = "~/.config/solana/id.json"


⚙️ Build the Program

anchor build


🧪 Run Tests

anchor test

This will automatically spin up a local validator and test the smart contracts.

📤 Deploy the Program

To Localnet: (which is what we are doing right now)

# Start a local Solana cluster

solana-test-validator

In another terminal:

anchor deploy --provider.cluster localnet --program-keypair ../XUTAAsrE6AGc3xzvKtz6VNab6QuwVx41MD7HB7K5zVa.json --program-name xuta_sc

##

📁 Project Structure

 programs/       # Anchor smart contracts (Rust)
 
 tests/          # Integration tests
 
