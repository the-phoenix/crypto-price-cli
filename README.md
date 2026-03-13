# crypto-price-cli

A lightweight Rust CLI that fetches and prints cryptocurrency prices (USD) using the CoinGecko API.

## 🚀 Features

- Fetches current price for a list of symbols (defaults to BTC / ETH / SOL)
- Supports custom coin lists via CLI args
- Prints prices in a single line (e.g., `BTC: $71308.00 | ETH: $2109.80 | SOL: $88.95`)
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

Default coins (BTC / ETH / SOL):

```bash
cargo run --quiet
```

Custom coins:

```bash
cargo run --quiet -- bitcoin solana
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

## 🧩 Code Structure

- `src/main.rs` – CLI entry point (minimal logic)
- `src/lib.rs` – core logic and helpers (parsing, URL building, JSON extraction, formatting)
- Unit tests are located inside `src/lib.rs`

## 🧪 Run Tests

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make changes and add tests
4. Run tests: `cargo test`
5. Open a pull request

Please keep PRs small and focused, and follow the existing style.

## 📄 License

This project is licensed under the **MIT License**. See [LICENSE](./LICENSE) for details.
