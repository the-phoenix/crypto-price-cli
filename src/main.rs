use clap::Parser;
use dotenvy::dotenv;

use crypto_price_cli::{build_price_url, extract_prices, format_prices, Config};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Coin IDs to fetch (default: bitcoin, ethereum, solana)
    coins: Vec<String>,

    /// API key for CoinGecko Pro
    #[arg(short = 'k', long = "api-key")]
    api_key: Option<String>,

    /// Use CoinGecko Pro API endpoint (requires API key)
    #[arg(long)]
    pro: bool,

    /// Disable showing the up/down indicator (default: shown)
    #[arg(long = "no-indicator")]
    no_indicator: bool,

    /// Percent change threshold to treat movement as "rapid" (defaults to 2.5)
    #[arg(long = "indicator-threshold", default_value_t = 2.5)]
    indicator_threshold: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let CliArgs = CliArgs::parse();

    let coins = if CliArgs.coins.is_empty() {
        vec!["bitcoin".to_string(), "ethereum".to_string(), "solana".to_string()]
    } else {
        CliArgs.coins
    };

    let cfg = Config {
        coins,
        api_key: CliArgs.api_key,
        use_pro: CliArgs.pro,
        show_indicator: !CliArgs.no_indicator,
        indicator_threshold_percent: CliArgs.indicator_threshold,
    };

    let url = build_price_url(&cfg)?;

    let client = reqwest::Client::builder()
        .user_agent("crypto-price-cli/0.1 (+https://github.com/the-phoenix/crypto-price-cli)")
        .build()?;

    let raw = client.get(url).send().await?.text().await?;
    let resp = serde_json::from_str(&raw)?;

    let prices = extract_prices(&resp, &cfg.coins);
    if prices.iter().any(|(_, p, _)| p.is_none()) {
        eprintln!("raw API response: {raw}");
    }

    println!(
        "{}",
        format_prices(&prices, cfg.show_indicator, cfg.indicator_threshold_percent)
    );

    Ok(())
}
