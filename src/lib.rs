use reqwest::Url;
use serde_json::Value;

/// CoinGecko API endpoints.
pub const COINGECKO_PUBLIC_API: &str = "https://api.coingecko.com/api/v3/simple/price";
pub const COINGECKO_PRO_API: &str = "https://pro-api.coingecko.com/api/v3/simple/price";

/// Configuration for the price fetch operation.
pub struct Config {
    pub coins: Vec<String>,
    pub api_key: Option<String>,
    pub use_pro: bool,
}

/// Convert a coin ID into a symbol string (e.g., "bitcoin" -> "BTC").
pub fn symbol_from_id(id: &str) -> String {
    match id {
        "bitcoin" => "BTC".into(),
        "ethereum" => "ETH".into(),
        "solana" => "SOL".into(),
        _ => id.chars().take(3).collect::<String>().to_uppercase(),
    }
}

/// Parse the CLI arguments into a `Config`.
///
/// Supports:
/// - `--api-key <key>` / `-k <key>` (for pro usage)
/// - `--pro` to use the pro API endpoint (required when using a pro key)
/// - any other argument is treated as a coin ID
pub fn parse_config<I: IntoIterator<Item = String>>(args: I) -> Config {
    let mut coins = Vec::new();
    let mut api_key = None;
    let mut use_pro = false;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--api-key" | "-k" => api_key = iter.next(),
            "--pro" => use_pro = true,
            _ => coins.push(arg),
        }
    }

    if coins.is_empty() {
        coins = vec!["bitcoin".to_string(), "ethereum".to_string(), "solana".to_string()];
    }

    Config {
        coins,
        api_key,
        use_pro,
    }
}

/// Build the CoinGecko price URL for the given config.
///
/// If `config.use_pro` is true, it uses the pro endpoint and appends the key if present.
pub fn build_price_url(config: &Config) -> Result<Url, Box<dyn std::error::Error>> {
    let base = if config.use_pro {
        COINGECKO_PRO_API
    } else {
        COINGECKO_PUBLIC_API
    };

    let mut url = Url::parse(base)?;
    url.query_pairs_mut()
        .append_pair("ids", &config.coins.join(","))
        .append_pair("vs_currencies", "usd");

    if config.use_pro {
        if let Some(key) = config.api_key.as_deref() {
            url.query_pairs_mut().append_pair("x_cg_pro_api_key", key);
        }
    }

    Ok(url)
}

/// Extract coin prices from the JSON response.
pub fn extract_prices(resp: &Value, coins: &[String]) -> Vec<(String, Option<f64>)> {
    coins
        .iter()
        .map(|id| {
            let price = resp
                .get(id)
                .and_then(|v: &Value| v.get("usd"))
                .and_then(Value::as_f64);
            (id.clone(), price)
        })
        .collect()
}

/// Format a list of symbol/price pairs into one-line display text.
pub fn format_prices(prices: &[(String, Option<f64>)]) -> String {
    prices
        .iter()
        .map(|(id, price)| {
            let symbol = symbol_from_id(id);
            match price {
                Some(val) => format!("{symbol}: ${:.2}", val),
                None => format!("{symbol}: N/A"),
            }
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_defaults() {
        let cfg = parse_config(Vec::<String>::new());
        assert_eq!(cfg.coins, vec!["bitcoin", "ethereum", "solana"]);
        assert!(!cfg.use_pro);
        assert!(cfg.api_key.is_none());
    }

    #[test]
    fn parse_config_with_args() {
        let cfg = parse_config(vec!["bitcoin".into(), "dogecoin".into()]);
        assert_eq!(cfg.coins, vec!["bitcoin", "dogecoin"]);
    }

    #[test]
    fn build_price_url_default() {
        let cfg = Config {
            coins: vec!["bitcoin".into(), "solana".into()],
            api_key: None,
            use_pro: false,
        };
        let url = build_price_url(&cfg).unwrap();
        assert!(url.as_str().starts_with("https://api.coingecko.com"));
        assert!(url.query().unwrap().contains("ids=bitcoin%2Csolana"));
    }

    #[test]
    fn build_price_url_pro_includes_key() {
        let cfg = Config {
            coins: vec!["ethereum".into()],
            api_key: Some("test-key".into()),
            use_pro: true,
        };
        let url = build_price_url(&cfg).unwrap();
        assert!(url.as_str().starts_with("https://pro-api.coingecko.com"));
        assert!(url.query().unwrap().contains("x_cg_pro_api_key=test-key"));
    }

    #[test]
    fn format_prices_handles_missing() {
        let prices = vec![("bitcoin".into(), Some(123.45)), ("unknown".into(), None)];
        let out = format_prices(&prices);
        assert!(out.contains("BTC: $123.45"));
        assert!(out.contains("UNK: N/A"));
    }
}
