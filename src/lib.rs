use reqwest::Url;
use serde_json::Value;

/// CoinGecko API endpoints.
pub const COINGECKO_PUBLIC_API: &str = "https://api.coingecko.com/api/v3/simple/price";
pub const COINGECKO_PRO_API: &str = "https://pro-api.coingecko.com/api/v3/simple/price";

/// Indicator types for price change visualization.
#[derive(Debug, Clone, Copy)]
enum Indicator {
    GradualUp,
    RapidUp,
    GradualDown,
    RapidDown,
    None,
}

impl Indicator {
    fn as_str(&self) -> &'static str {
        match self {
            Indicator::GradualUp => " ⬈",
            Indicator::RapidUp => " ⬆",
            Indicator::GradualDown => " ⬊",
            Indicator::RapidDown => " ⬇",
            Indicator::None => "",
        }
    }
}

/// Configuration for the price fetch operation.
pub struct Config {
    pub coins: Vec<String>,
    pub api_key: Option<String>,
    pub use_pro: bool,
    /// Whether to show an up/down indicator based on 24h price change.
    pub show_indicator: bool,
    /// Percent change threshold to switch from gradual (⬈/⬊) to rapid (⬆/⬇).
    pub indicator_threshold_percent: f64,
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
/// - `--no-indicator` to disable showing an up/down arrow in the output
/// - any other argument is treated as a coin ID
pub fn parse_config<I: IntoIterator<Item = String>>(args: I) -> Config {
    let mut coins = Vec::new();
    let mut api_key = None;
    let mut use_pro = false;
    let mut show_indicator = true;
    let mut indicator_threshold_percent = 5.0;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--api-key" | "-k" => api_key = iter.next(),
            "--pro" => use_pro = true,
            "--no-indicator" => show_indicator = false,
            "--indicator" => show_indicator = true,
            "--indicator-threshold" => {
                if let Some(val) = iter.next() {
                    if let Ok(parsed) = val.parse::<f64>() {
                        indicator_threshold_percent = parsed;
                    }
                }
            }
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
        show_indicator,
        indicator_threshold_percent,
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

    if config.show_indicator {
        url.query_pairs_mut().append_pair("include_24hr_change", "true");
    }

    if config.use_pro {
        if let Some(key) = config.api_key.as_deref() {
            url.query_pairs_mut().append_pair("x_cg_pro_api_key", key);
        }
    }

    Ok(url)
}

/// Extract coin prices (and optionally 24h change) from the JSON response.
pub fn extract_prices(resp: &Value, coins: &[String]) -> Vec<(String, Option<f64>, Option<f64>)> {
    coins
        .iter()
        .map(|id| {
            let price = resp
                .get(id)
                .and_then(|v: &Value| v.get("usd"))
                .and_then(Value::as_f64);
            let change_24h = resp
                .get(id)
                .and_then(|v: &Value| v.get("usd_24h_change"))
                .and_then(Value::as_f64);
            (id.clone(), price, change_24h)
        })
        .collect()
}

/// Format a list of symbol/price pairs into one-line display text.
///
/// If `show_indicator` is true, this will append an indicator depending on whether the
/// price is higher or lower than 24h ago.
///
/// - Small change (gradual): ⬈ / ⬊
/// - Large change (rapid):  ⬆ / ⬇
pub fn format_prices(
    prices: &[(String, Option<f64>, Option<f64>)],
    show_indicator: bool,
    indicator_threshold_percent: f64,
) -> String {
    prices
        .iter()
        .map(|(id, price, change_24h)| {
            let symbol = symbol_from_id(id);
            match price {
                Some(val) => {
                    let indicator = if show_indicator {
                        match change_24h {
                            Some(c) if *c > 0.0 => {
                                if *c >= indicator_threshold_percent {
                                    Indicator::RapidUp
                                } else {
                                    Indicator::GradualUp
                                }
                            }
                            Some(c) if *c < 0.0 => {
                                if *c <= -indicator_threshold_percent {
                                    Indicator::RapidDown
                                } else {
                                    Indicator::GradualDown
                                }
                            }
                            _ => Indicator::None,
                        }
                    } else {
                        Indicator::None
                    };
                    format!("{symbol}: ${:.2}{}", val, indicator.as_str())
                }
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
        assert!(cfg.show_indicator);
        assert_eq!(cfg.indicator_threshold_percent, 5.0);
    }

    #[test]
    fn parse_config_with_args() {
        let cfg = parse_config(vec!["--no-indicator".into(), "bitcoin".into(), "dogecoin".into()]);
        assert_eq!(cfg.coins, vec!["bitcoin", "dogecoin"]);
        assert!(!cfg.show_indicator);
        assert_eq!(cfg.indicator_threshold_percent, 5.0);
    }

    #[test]
    fn parse_config_threshold_override() {
        let cfg = parse_config(vec!["--indicator-threshold".into(), "3.5".into(), "bitcoin".into()]);
        assert_eq!(cfg.coins, vec!["bitcoin"]);
        assert_eq!(cfg.indicator_threshold_percent, 3.5);
    }

    #[test]
    fn build_price_url_default() {
        let cfg = Config {
            coins: vec!["bitcoin".into(), "solana".into()],
            api_key: None,
            use_pro: false,
            show_indicator: true,
            indicator_threshold_percent: 5.0,
        };
        let url = build_price_url(&cfg).unwrap();
        assert!(url.as_str().starts_with("https://api.coingecko.com"));
        assert!(url.query().unwrap().contains("ids=bitcoin%2Csolana"));
        assert!(url.query().unwrap().contains("include_24hr_change=true"));
    }

    #[test]
    fn build_price_url_pro_includes_key() {
        let cfg = Config {
            coins: vec!["ethereum".into()],
            api_key: Some("test-key".into()),
            use_pro: true,
            show_indicator: true,
            indicator_threshold_percent: 5.0,
        };
        let url = build_price_url(&cfg).unwrap();
        assert!(url.as_str().starts_with("https://pro-api.coingecko.com"));
        assert!(url.query().unwrap().contains("x_cg_pro_api_key=test-key"));
        assert!(url.query().unwrap().contains("include_24hr_change=true"));
    }

    #[test]
    fn format_prices_handles_missing() {
        let prices = vec![("bitcoin".into(), Some(123.45), None), ("unknown".into(), None, None)];
        let out = format_prices(&prices, false, 5.0);
        assert!(out.contains("BTC: $123.45"));
        assert!(out.contains("UNK: N/A"));
    }

    #[test]
    fn format_prices_indicators() {
        let prices = vec![
            ("bitcoin".into(), Some(1.0), Some(0.5)),
            ("ethereum".into(), Some(1.0), Some(5.0)),
            ("solana".into(), Some(1.0), Some(-0.5)),
            ("dogecoin".into(), Some(1.0), Some(-5.0)),
        ];

        let out = format_prices(&prices, true, 5.0);
        assert!(out.contains("BTC: $1.00 ⬈"));
        assert!(out.contains("ETH: $1.00 ⬆"));
        assert!(out.contains("SOL: $1.00 ⬊"));
        assert!(out.contains("DOG: $1.00 ⬇"));
    }
}
