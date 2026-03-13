use dotenvy::dotenv;
use std::env;

use crypto_price_cli::{build_price_url, extract_prices, format_prices, parse_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let cfg = parse_config(env::args().skip(1));

    let url = build_price_url(&cfg)?;

    let client = reqwest::Client::builder()
        .user_agent("crypto-price-cli/0.1 (+https://github.com/the-phoenix/crypto-price-cli)")
        .build()?;

    let raw = client.get(url).send().await?.text().await?;
    let resp = serde_json::from_str(&raw)?;

    let prices = extract_prices(&resp, &cfg.coins);
    if prices.iter().any(|(_, p)| p.is_none()) {
        eprintln!("raw API response: {raw}");
    }

    println!("{}", format_prices(&prices));

    Ok(())
}
