# Crabllet

[![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/KyraLabs/crabllet?utm_source=oss&utm_medium=github&utm_campaign=KyraLabs%2Fcrabllet&labelColor=171717&color=FF570A&link=https%3A%2F%2Fcoderabbit.ai&label=CodeRabbit+Reviews)](https://coderabbit.ai)

A Rusty little crab guarding your sats — Bitcoin TUI wallet written in Rust.

## About

Crabllet is an educational Bitcoin wallet with a Terminal User Interface. The goal is to understand Bitcoin cryptography from the ground up: entropy generation, key derivation, address creation, and light wallet protocols.

**Status:** Work in progress (Phase 1)

**Target:** Testnet only — never use with real funds during development.

## Features

### Implemented

- [x] BIP-39 mnemonic generation (12 or 24 words)
- [x] Cryptographically secure entropy generation
- [x] Modular architecture (crypto separated from UI)

### Roadmap

- [ ] Passphrase support (BIP-39)
- [ ] Seed derivation from mnemonic
- [ ] BIP-32 HD key derivation
- [ ] Bitcoin address generation (P2WPKH, P2PKH)
- [ ] Electrum server connection
- [ ] Balance and transaction queries
- [ ] Terminal User Interface
- [ ] Encrypted wallet persistence

## Getting Started

### Prerequisites

- Rust 2024 edition (install via [rustup](https://rustup.rs/))

### Build and Run

```bash
# Clone the repository
git clone https://github.com/KyraLabs/crabllet.git
cd crabllet

# Generate a 12-word mnemonic
cargo run

# Generate a 24-word mnemonic
cargo run -- 24
```

## Project Structure

```
crabllet/
├── src/
│   ├── main.rs              # Entry point, CLI interface
│   └── crypto/
│       ├── mod.rs           # Module exports
│       └── mnemonic.rs      # BIP-39 mnemonic generation
├── docs/
│   └── crabllet-planning.md # Development phases and planning
├── Cargo.toml
└── README.md
```

### Planned Structure

```
src/
├── main.rs          # Entry point, TUI initialization
├── lib.rs           # Public API, re-exports
├── crypto/
│   ├── mod.rs
│   ├── mnemonic.rs  # BIP-39 mnemonics
│   └── keys.rs      # BIP-32 key derivation
├── wallet/
│   ├── mod.rs
│   └── address.rs   # Address generation
├── network/
│   └── electrum.rs  # Electrum protocol client
└── tui/
    ├── mod.rs
    └── screens/     # TUI screens and widgets
```

## Development Phases

| Phase | Description | Status |
|-------|-------------|--------|
| 0 | Project foundation and setup | Done |
| 1 | Entropy and BIP-39 mnemonic generation | In Progress |
| 2 | BIP-32 HD key derivation | Pending |
| 3 | Bitcoin address generation | Pending |
| 4 | Electrum network connection | Pending |
| 5 | Balance and transaction queries | Pending |
| 6 | Basic TUI implementation | Pending |
| 7 | Secure wallet persistence | Pending |

See [docs/crabllet-planning.md](docs/crabllet-planning.md) for detailed phase descriptions.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `bip39` | BIP-39 mnemonic encoding/decoding |
| `rand` | Cryptographically secure random number generation |
| `hex` | Hexadecimal encoding for debugging |

## Learning Resources

- [BIP-39: Mnemonic code for generating deterministic keys](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP-32: Hierarchical Deterministic Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP-84: Derivation scheme for P2WPKH](https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki)

## License

MIT License — see [LICENSE](LICENSE) for details.

## Disclaimer

This is an educational project. Do not use with real Bitcoin. Always use testnet during development.
