# Crabllet — Development Planning

A Rusty little crab guarding your sats — Bitcoin TUI wallet

## Overview

This document outlines the development phases for Crabllet, a Bitcoin wallet with a Terminal User Interface built in Rust. The initial goal is to create a read-only wallet capable of generating addresses and checking balances on testnet.

---

## Phase 0: Project Foundation

**Objective:** Set up the Rust project structure and understand the core dependencies.

**What you'll learn:**
- Cargo workspace organization for a multi-module project
- How Bitcoin libraries in Rust are structured
- The separation between cryptographic primitives and Bitcoin-specific logic

**Key decisions:**
- Define module boundaries (keys, addresses, network, tui)
- Choose between `rust-bitcoin` + `rust-secp256k1` directly vs higher abstractions
- Set up basic error handling patterns

**Outcome:** A compiling Rust project with dependencies configured and a clear folder structure.

---

## Phase 1: Entropy and Seed Generation

**Objective:** Generate cryptographically secure random data and convert it into a BIP-39 mnemonic phrase.

**What you'll learn:**
- Why entropy quality matters for wallet security
- How BIP-39 converts random bytes into human-readable words
- The role of checksums in mnemonic validation
- How a mnemonic becomes a seed through PBKDF2

**Core concepts:**
- Entropy (128-256 bits of randomness)
- Mnemonic words (12-24 words from a standardized wordlist)
- Seed derivation (mnemonic + optional passphrase → 512-bit seed)

**Outcome:** Ability to generate a new wallet from scratch and display the recovery phrase.

---

## Phase 2: HD Key Derivation

**Objective:** Implement hierarchical deterministic key derivation following BIP-32.

**What you'll learn:**
- How a single seed can generate unlimited keys
- The math behind child key derivation (HMAC-SHA512)
- Difference between hardened and non-hardened derivation
- Extended public vs extended private keys (xpub/xprv)

**Core concepts:**
- Master key derivation from seed
- Derivation paths (e.g., m/84'/0'/0'/0/0)
- Why hardened derivation exists (security implications)
- BIP-44/49/84 path standards and what they mean

**Outcome:** Given a seed, derive any key at any path in the HD tree.

---

## Phase 3: Address Generation

**Objective:** Convert public keys into Bitcoin addresses users can share to receive funds.

**What you'll learn:**
- The different Bitcoin address types and their history
- Hash functions used (SHA-256, RIPEMD-160)
- Base58Check and Bech32 encoding
- Why address formats evolved (fees, features, error detection)

**Address types to implement:**
- P2WPKH (Native SegWit, bc1q...) — recommended default
- P2PKH (Legacy, 1...) — for compatibility understanding
- Testnet equivalents (tb1q..., m/n...)

**Outcome:** Generate receive addresses from the HD keys and display them to the user.

---

## Phase 4: Network Connection

**Objective:** Connect to the Bitcoin network to query blockchain data.

**What you'll learn:**
- How light wallets work without downloading the full blockchain
- The Electrum protocol (JSON-RPC over TCP/SSL)
- Subscription model for address monitoring
- Trade-offs between privacy and convenience

**Core concepts:**
- Electrum server communication
- Script hashes (how Electrum indexes addresses)
- Blockchain height and transaction history
- Public servers vs running your own

**Outcome:** Establish a connection to a testnet Electrum server and verify it works.

---

## Phase 5: Balance and Transaction History

**Objective:** Query and display the wallet's balance and past transactions.

**What you'll learn:**
- UTXO model (Unspent Transaction Outputs)
- How balance is calculated (sum of all UTXOs you control)
- Transaction structure basics (inputs, outputs, fees)
- Confirmation status and what it means

**Core concepts:**
- UTXO tracking and storage
- Confirmed vs unconfirmed balance
- Transaction history retrieval
- Address gap limit (how wallets know when to stop looking)

**Outcome:** Display total balance and list of transactions for the wallet.

---

## Phase 6: Basic TUI

**Objective:** Build a terminal interface to interact with wallet functionality.

**What you'll learn:**
- Terminal rendering with ratatui
- Event handling (keyboard input, async updates)
- State management in a TUI application
- Responsive layouts for different terminal sizes

**Initial screens:**
- Wallet creation / recovery flow
- Main dashboard (balance, recent transactions)
- Receive screen (address display with QR code in ASCII)
- Address list view

**Outcome:** A usable terminal interface that ties together all previous phases.

---

## Phase 7: Persistence

**Objective:** Save and load wallet data securely between sessions.

**What you'll learn:**
- Secure storage of sensitive data (encrypted seeds)
- File formats for wallet data
- Key derivation for encryption (from user password)
- What to store vs what to re-derive

**Data to persist:**
- Encrypted seed/mnemonic
- Account metadata (labels, settings)
- Address cache (to speed up startup)
- Transaction labels (user annotations)

**Outcome:** Close and reopen Crabllet without losing wallet state.

---

## Future Phases (Out of Scope for Now)

These will be planned in detail after the read-only wallet is complete:

- **Transaction Building:** Constructing and signing transactions
- **Fee Estimation:** Calculating appropriate fees based on mempool
- **Coin Selection:** Choosing which UTXOs to spend
- **PSBT Support:** Partially Signed Bitcoin Transactions for hardware wallet compatibility
- **Multiple Accounts:** Managing several wallets
- **Advanced Features:** Silent Payments, CoinJoin integration, etc.

---

## Technical Stack

| Component | Library/Tool |
|-----------|--------------|
| Bitcoin primitives | `rust-bitcoin` |
| Elliptic curve crypto | `rust-secp256k1` |
| BIP-39 mnemonics | `bip39` crate (or manual implementation for learning) |
| Electrum client | `electrum-client` (or custom implementation) |
| TUI framework | `ratatui` + `crossterm` |
| Async runtime | `tokio` |
| Serialization | `serde` + `serde_json` |
| Encryption | `aes-gcm` or `chacha20poly1305` |

---

## Development Principles

1. **Learn by doing:** Implement things manually first, then evaluate if a library does it better
2. **Testnet only:** Never test with real funds during development
3. **Security conscious:** Even for learning, build good habits around key handling
4. **Incremental progress:** Each phase produces something runnable and testable
5. **Document as you go:** Comments explaining the "why" are more valuable than the "what"
