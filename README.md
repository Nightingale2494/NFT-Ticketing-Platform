<div align="center">

# 🎟️ TicketChain

### Decentralized NFT Ticketing on Stellar Soroban

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue?style=for-the-badge&logo=stellar)](https://stellar.org)
[![Next.js](https://img.shields.io/badge/Next.js-14-black?style=for-the-badge&logo=next.js)](https://nextjs.org)
[![Rust](https://img.shields.io/badge/Rust-Smart_Contract-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Tailwind](https://img.shields.io/badge/Tailwind-CSS-38bdf8?style=for-the-badge&logo=tailwindcss)](https://tailwindcss.com)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

**Own your tickets as NFTs. No scalpers. No fraud. No middlemen.**

[Live Demo](#) · [Smart Contract](#-deployed-contract) · [Report Bug](#) · [Request Feature](#)

</div>

---

## 📖 Table of Contents

- [About](#-about)
- [Features](#-features)
- [Tech Stack](#-tech-stack)
- [Deployed Contract](#-deployed-contract)
- [Project Structure](#-project-structure)
- [Getting Started](#-getting-started)
- [Smart Contract API](#-smart-contract-api)
- [UI Pages](#-ui-pages)
- [Wallet Integration](#-wallet-integration)
- [Security](#-security)
- [Resources](#-resources)

---

## 🌟 About

TicketChain is a fully decentralized NFT-based event ticketing platform built on the **Stellar Soroban** smart contract platform. Event organizers can create events and mint NFT tickets on-chain, while users can purchase, hold, and transfer tickets using their **Freighter wallet** — all without any backend server.

Every ticket is a unique NFT stored on the Stellar blockchain. Ownership is verifiable, transfers are trustless, and resale prices can be capped by the organizer at the contract level.

---

## ✨ Features

| Feature | Description |
|---|---|
| 🎪 **Event Creation** | Organizers deploy events on-chain with supply, price, and resale rules |
| 🎟️ **NFT Tickets** | Each ticket is a unique on-chain NFT with metadata |
| 💳 **Wallet Connect** | Seamless Freighter wallet integration |
| 🛒 **Marketplace** | Browse, search, filter, and sort events |
| 👜 **My Tickets** | View your full NFT ticket collection |
| 🔄 **Resale** | Transfer tickets with on-chain enforced max resale price |
| ✅ **Verification** | Trustless ticket ownership verification at the gate |
| 🌐 **No Backend** | 100% frontend + smart contract, fully decentralized |

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| ⛓️ Blockchain | Stellar Soroban (Rust) |
| 🖥️ Frontend | Next.js 14, React 18 |
| 🎨 Styling | Tailwind CSS |
| 🗂️ State | Zustand |
| 🎬 Animations | Framer Motion |
| 👛 Wallet | Freighter API v1.7.1 |
| 📡 SDK | @stellar/stellar-sdk |

---

## 🔗 Deployed Contract

The smart contract is deployed and live on **Stellar Testnet**.

| | |
|---|---|
| **Contract ID** | `CCAIW7A5VP53QVR66XX4RGP5K243IZVCIS2BUSAOBVE7HKJOFAQX5F6J` |
| **Network** | Stellar Testnet |
| **Admin** | `GAWZ5DJY6YIGJFXLJ7W5WI5Y66DVA44T6JC7PGD74P7E3FMP2CZRLY5K` |
| **Explorer** | [View on Stellar Expert ↗](https://stellar.expert/explorer/testnet/contract/CCAIW7A5VP53QVR66XX4RGP5K243IZVCIS2BUSAOBVE7HKJOFAQX5F6J) |
| **Deploy Tx** | [View Transaction ↗](https://stellar.expert/explorer/testnet/tx/76e58aa7d90c4bc42a3e2e072aca36be24ab64601a659c53fa2b62a1bdcb6dc8) |
| **Network Passphrase** | `Test SDF Network ; September 2015` |
| **RPC URL** | `https://soroban-testnet.stellar.org` |

---

## 🧱 Project Structure

```
NFT-Ticketing-Platform/
│
├── 📁 contracts/
│   └── 📁 ticket_nft/
│       ├── 📁 src/
│       │   └── 📄 lib.rs              # Soroban smart contract (Rust)
│       └── 📄 Cargo.toml             # Contract dependencies
│
├── 📁 frontend/
│   ├── 📁 app/
│   │   ├── 📄 layout.tsx             # Root layout + providers
│   │   ├── 📄 globals.css            # Global styles
│   │   ├── 📄 page.tsx               # Home / Dashboard
│   │   ├── 📁 marketplace/
│   │   │   └── 📄 page.tsx           # Browse & buy tickets
│   │   ├── 📁 create-event/
│   │   │   └── 📄 page.tsx           # Create event form
│   │   └── 📁 my-tickets/
│   │       └── 📄 page.tsx           # NFT collection + verification
│   │
│   ├── 📁 components/
│   │   ├── 📄 Navbar.tsx             # Navigation + wallet connect
│   │   ├── 📄 EventCard.tsx          # Event display card
│   │   ├── 📄 TicketCard.tsx         # NFT ticket card + transfer
│   │   ├── 📄 CreateEventForm.tsx    # Event creation form
│   │   └── 📄 Toast.tsx              # Toast notifications
│   │
│   ├── 📁 hooks/
│   │   └── 📄 useContract.ts         # Contract interaction hooks
│   │
│   ├── 📁 lib/
│   │   ├── 📄 constants.ts           # Network config + helpers
│   │   ├── 📄 wallet.ts              # Freighter wallet integration
│   │   └── 📄 contract.ts            # Soroban contract layer
│   │
│   ├── 📁 store/
│   │   └── 📄 index.ts               # Zustand global state
│   │
│   ├── 📄 package.json
│   ├── 📄 next.config.js
│   ├── 📄 tailwind.config.js
│   └── 📄 tsconfig.json
│
├── 📄 Cargo.toml                     # Rust workspace
└── 📄 README.md
```

---

## 🚀 Getting Started

### Prerequisites

Before you begin, make sure you have:

- [Node.js](https://nodejs.org) v18+
- [Rust](https://rustup.rs) + wasm target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- [Freighter Wallet](https://freighter.app) browser extension
- A Stellar testnet account with XLM

### 1. Clone the repository

```bash
git clone https://github.com/Nightingale2494/NFT-Ticketing-Platform.git
cd NFT-Ticketing-Platform
```

### 2. Install Rust WASM target

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install Stellar CLI

```bash
cargo install stellar-cli --features opt
```

### 4. Build the smart contract

```bash
stellar contract build
```

### 5. Deploy to Testnet

> ⚠️ The contract is already deployed — skip this step if you just want to run the frontend against the existing deployment.

```bash
# Generate a deployer keypair
stellar keys generate deployer --network testnet

# Fund it with testnet XLM
stellar keys fund deployer --network testnet

# Deploy
stellar contract deploy \
  --wasm target/wasm32v1-none/release/ticket_nft.wasm \
  --source deployer \
  --network testnet

# Initialize with your admin address
stellar contract invoke \
  --id YOUR_CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin YOUR_PUBLIC_KEY
```

### 6. Configure the frontend

```bash
cd frontend
cp .env.local.example .env.local
```

Edit `.env.local`:

```env
NEXT_PUBLIC_CONTRACT_ID=CCAIW7A5VP53QVR66XX4RGP5K243IZVCIS2BUSAOBVE7HKJOFAQX5F6J
NEXT_PUBLIC_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
NEXT_PUBLIC_RPC_URL=https://soroban-testnet.stellar.org
NEXT_PUBLIC_HORIZON_URL=https://horizon-testnet.stellar.org
```

### 7. Install dependencies and run

```bash
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser. Connect your Freighter wallet and you're live! 🎉

---

## 🧾 Smart Contract API

The contract exposes 16 functions covering the full ticketing lifecycle.

### Core Functions

| Function | Auth Required | Description |
|---|---|---|
| `initialize(admin)` | — | Initialize contract with admin address |
| `create_event(organizer, name, date, description, supply, price, max_resale, metadata_uri)` | ✅ Organizer | Deploy a new event on-chain |
| `mint_ticket(organizer, event_id, buyer, metadata_uri)` | ✅ Organizer | Mint an NFT ticket to a buyer |
| `transfer_ticket(from, to, ticket_id, sale_price)` | ✅ Owner | Transfer or resell a ticket |
| `list_for_sale(owner, ticket_id, resale_price)` | ✅ Owner | List a ticket on the resale market |
| `delist_from_sale(owner, ticket_id)` | ✅ Owner | Remove listing from resale market |
| `deactivate_event(organizer, event_id)` | ✅ Organizer | Deactivate an event |

### Query Functions

| Function | Description |
|---|---|
| `get_event(event_id)` | Get full event details |
| `get_all_event_ids()` | Get all event IDs |
| `get_ticket(ticket_id)` | Get full ticket details |
| `get_tickets(owner)` | Get all ticket IDs owned by an address |
| `get_event_tickets(event_id)` | Get all ticket IDs for an event |
| `verify_ticket(owner, event_id)` | Returns `true` if address owns a ticket for the event |
| `get_event_count()` | Total number of events created |
| `get_ticket_count()` | Total number of tickets minted |

### Data Types

```rust
// Event stored on-chain
struct EventData {
    id: u64,
    name: String,
    date: u64,              // Unix timestamp
    description: String,
    organizer: Address,
    total_supply: u64,
    minted_count: u64,
    ticket_price: i128,     // in stroops (1 XLM = 10,000,000 stroops)
    max_resale_price: i128, // 0 = no restriction
    is_active: bool,
}

// NFT Ticket stored on-chain
struct TicketData {
    id: u64,
    event_id: u64,
    owner: Address,
    original_owner: Address,
    purchase_price: i128,
    is_for_sale: bool,
    resale_price: i128,
    metadata_uri: String,
}
```

### Storage Schema

```
DataKey::Event(event_id)        → EventData
DataKey::Ticket(ticket_id)      → TicketData
DataKey::EventTickets(event_id) → Vec<ticket_id>
DataKey::OwnerTickets(address)  → Vec<ticket_id>
DataKey::EventCounter           → u64
DataKey::TicketCounter          → u64
DataKey::Admin                  → Address
```

---

## 🎨 UI Pages

| Route | Description |
|---|---|
| `/` | Hero, live stats, how-it-works, featured events |
| `/marketplace` | Browse all events with search, filter, and sort |
| `/create-event` | Deploy a new event on-chain with full validation |
| `/my-tickets` | View owned NFT tickets, transfer/resell, verify attendance |

---

## 👛 Wallet Integration

Uses `@stellar/freighter-api` v1.7.1 for all wallet operations:

```ts
// Connect wallet — triggers Freighter popup
const walletInfo = await connectFreighterWallet();
// → { address, network, networkPassphrase }

// Sign a transaction XDR
const signedXdr = await signXdr(txXdr, networkPassphrase);

// Get address silently (no popup)
const address = await getWalletAddress();

// Validate the user is on the correct network
const valid = await validateNetwork(NETWORK_PASSPHRASE);
```

---

## 🛡️ Security

- **`require_auth()`** — all write operations verify the caller's signature on-chain
- **Supply cap** — minting beyond `total_supply` is rejected at the contract level
- **Resale price enforcement** — transfers above `max_resale_price` are rejected on-chain
- **Organizer-only minting** — only the event creator can mint tickets for their event
- **Trustless verification** — `verify_ticket` cannot be spoofed or faked
- **No backend** — no server to hack, no database to breach

---

## 🧪 Running Tests

```bash
cd contracts/ticket_nft
cargo test
```

Three tests are included covering:
- Event creation and ticket minting
- Ticket transfer with resale price enforcement  
- Supply cap enforcement

---

## 🌐 Resources

- [Soroban Documentation](https://developers.stellar.org/docs/smart-contracts)
- [Stellar Expert Testnet Explorer](https://stellar.expert/explorer/testnet)
- [Freighter Wallet](https://freighter.app)
- [Stellar Laboratory](https://laboratory.stellar.org)
- [Get Testnet XLM (Friendbot)](https://laboratory.stellar.org/#account-creator)
- [Stellar CLI Docs](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)

---

## ⚠️ Important Notes

1. **No backend** — the entire platform runs on frontend + Soroban smart contract only
2. The contract must be **initialized** before use (already done for the deployed contract above)
3. Set `max_resale_price = 0` to allow unlimited resale pricing
4. All prices are in **stroops** on-chain (1 XLM = 10,000,000 stroops)
5. Always test on **Testnet** before deploying to Mainnet

---

<div align="center">

Built with ❤️ on [Stellar Soroban](https://stellar.org) · [Nightingale2494](https://github.com/Nightingale2494)

</div>
