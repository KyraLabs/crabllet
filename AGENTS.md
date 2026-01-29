# Crabllet — Repository Guidelines

A Rusty little crab guarding your sats — Bitcoin TUI wallet

## Project Structure

```
crabllet/
├── src/
│   ├── main.rs          # Entry point, TUI initialization
│   ├── lib.rs           # Public API, re-exports
│   ├── crypto/          # Cryptographic operations
│   │   └── mod.rs
│   ├── wallet/          # Wallet logic, address management
│   │   └── mod.rs
│   ├── network/         # Electrum client, blockchain queries
│   │   └── mod.rs
│   └── tui/             # Terminal interface, screens, widgets
│       └── mod.rs
├── tests/               # Integration tests
├── Cargo.toml
├── AGENTS.md
├── README.md
└── PLANNING.md
```

Modules start as single `mod.rs` files. Split into submodules when a file exceeds ~300 lines or handles multiple distinct responsibilities.

## Module Responsibilities

- `crypto/` — Entropy generation, BIP-39 mnemonics, BIP-32 HD key derivation. No network or UI code.
- `wallet/` — Address generation, UTXO tracking, balance calculation. Uses crypto, no UI code.
- `network/` — Electrum protocol, server connection, blockchain data fetching. No UI code.
- `tui/` — Screens, widgets, event handling, rendering. Calls into wallet/network but contains no business logic.

## Build & Development Commands

- `cargo build` — Compile debug build.
- `cargo run` — Run Crabllet TUI.
- `cargo test` — Run all tests.
- `cargo fmt` — Format code with rustfmt.
- `cargo clippy --all-targets` — Lints must pass before commits.

## Coding Style

- Rust 2021 edition, 4-space indentation.
- `snake_case` for functions and variables, `PascalCase` for types, `SCREAMING_SNAKE` for constants.
- Document public APIs with `///` doc comments.
- Prefer `thiserror` for custom error types.
- Use `Result<T, E>` for fallible operations, avoid `.unwrap()` except in tests.

## Bitcoin-Specific Conventions

- All development and testing against **testnet only**.
- Never log or print private keys, seeds, or mnemonics in release builds.
- Use `secrecy` crate for sensitive data in memory when available.
- Amounts internally in **satoshis** (u64), convert to BTC only for display.
- Address strings should always include network prefix validation.

## Testing Guidelines

- Unit tests live in the same file under `#[cfg(test)] mod tests`.
- Use descriptive test names: `generates_valid_bip39_mnemonic`, `derives_correct_address_at_path`.
- Test vectors from BIPs are preferred over invented test data.
- Network tests should use testnet public Electrum servers or mocks.

## Dependencies

Core libraries (add as needed per phase):

| Purpose | Crate |
|---------|-------|
| Bitcoin primitives | `bitcoin` |
| Secp256k1 operations | `secp256k1` |
| BIP-39 mnemonics | `bip39` |
| Electrum client | `electrum-client` |
| TUI framework | `ratatui` |
| Terminal backend | `crossterm` |
| Async runtime | `tokio` |
| Serialization | `serde`, `serde_json` |
| Error handling | `thiserror`, `anyhow` |

## Commit Guidelines

- Base work on `main` branch.
- Commit messages: imperative mood, ≤50 chars subject line.
- Reference planning phases in commits when applicable, e.g., `[Phase 1] Add mnemonic generation`.
- Keep commits focused — one logical change per commit.

## Security Reminders

- This is a learning project but treat keys as if they were real.
- Never commit wallet files, seeds, or test mnemonics with funds.
- Use `.gitignore` to exclude any local wallet data.
- Zeroize sensitive data when done (seed bytes, private keys).
