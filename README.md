xuta.project{**AT**}gmail.com

🚀 Getting Started
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
To Localnet:

# Start a local Solana cluster
solana-test-validator

In another terminal:
solana config set --url localhost

anchor deploy
To Devnet:

solana config set --url https://api.devnet.solana.com
anchor deploy

##

📁 Project Structure
.
 programs/       # Anchor smart contracts (Rust)

 migrations/     # Deployment scripts
 
 tests/          # Integration tests
 
 Anchor.toml     # Anchor configuration
 
 Cargo.toml      # Rust project config
 
 README.md       # Project documentation
