use clap::Parser;
use dotenvy::dotenv;

use crypto_price_cli::{run, Config};

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

    let args = CliArgs::parse();

    let coins = if args.coins.is_empty() {
        vec!["bitcoin".to_string(), "ethereum".to_string(), "solana".to_string()]
    } else {
        args.coins
    };

    let cfg = Config {
        coins,
        api_key: args.api_key,
        use_pro: args.pro,
        show_indicator: !args.no_indicator,
        indicator_threshold_percent: args.indicator_threshold,
    };

    let output = run(&cfg).await?;
    println!("{output}");

    Ok(())
}
