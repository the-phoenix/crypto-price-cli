# crypto-price-cli

A lightweight Rust CLI that fetches and prints cryptocurrency prices (USD) using the CoinGecko API.

## 🚀 Features

- Fetches current price for a list of symbols (defaults to BTC / ETH / SOL)
- Supports custom coin lists via CLI args
- Prints prices in a single line (e.g., `BTC: $71308.00 📈 | ETH: $2109.80 🚀 | SOL: $88.95 📉`)
- Shows 24h price change indicators (gradual: 📈/📉, rapid: 🚀/💔) based on configurable threshold (default 2.5%)
- Supports CoinGecko **Pro** API keys via `--pro` and `--api-key`
- Loads `.env` for optional key configuration
- Modular, testable design with unit tests

## 🧰 Getting Started

### Prerequisites

- Rust (1.70+ recommended)

### Install

```bash
cargo build --release
```

### Run

Default coins (BTC / ETH / SOL) with indicators:

```bash
cargo run --quiet
# Output: BTC: $71308.00⬈ | ETH: $2109.80⬆ | SOL: $88.95⬊
```

Custom coins:

```bash
cargo run --quiet -- bitcoin solana
```

Disable indicators:

```bash
cargo run --quiet -- --no-indicator
# Output: BTC: $71308.00 | ETH: $2109.80 | SOL: $88.95
```

Set custom indicator threshold (e.g., 3% instead of default 5%):

```bash
cargo run --quiet -- --indicator-threshold 3
```

### Pro API key (optional)

If you have a CoinGecko Pro API key, put it in a `.env` file:

```env
COINGECKO_API_KEY=your_pro_api_key_here
```

Then run with `--pro`:

```bash
cargo run --quiet -- --pro
```

Or pass the key directly:

```bash
cargo run --quiet -- --pro --api-key your_pro_api_key_here bitcoin solana
```

All flags can be combined:

```bash
cargo run --quiet -- --pro --api-key your_key --indicator-threshold 2 bitcoin ethereum
```

## 🧩 Code Structure

- `src/main.rs` – CLI entry point (minimal logic)
- `src/lib.rs` – core logic and helpers (parsing, URL building, JSON extraction, formatting)
- `run()` in `src/lib.rs` is the single high-level workflow used by `main.rs`
- Unit tests are located inside `src/lib.rs`

## 🔧 Indicator behavior note

- Up/down indicators are set in `Indicator::as_str()` in `src/lib.rs`:
  - gradual up: `📈`
  - rapid up: `🚀`
  - gradual down: `📉`
  - rapid down: `💔`
- Default threshold for rapid/gradual is `--indicator-threshold 2.5` (or configured via CLI).
## 🧪 Run Tests

```bash
cargo test
```

## 🤝 Contributing

This repository is maintained as a personal productivity project and is designed to improve local shell workflows (e.g., zsh). You are welcome to fork the codebase, adapt it to your own requirements, and submit PRs for enhancements that would make this tool more useful for the community.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Implement changes and add tests
4. Execute tests: `cargo test`
5. Open a pull request

Please keep PRs focused, well-scoped, and consistent with the existing coding style.

## 📄 License

This project is licensed under the **MIT License**. See [LICENSE](./LICENSE) for details.
