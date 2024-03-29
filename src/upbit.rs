use crate::common::{Exchange, ExchangeName, Ticker};
use crate::errors::FetchError;
use crate::market::Market;
use crate::util::parse_market_string;
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;

pub struct Upbit {
    ticker_refresh_period: u64,
    all_tickers: Vec<Ticker>,
    last_fetched: DateTime<Utc>,
    name: ExchangeName,
    http_client: Client,
}

impl Upbit {
    pub fn new() -> Upbit {
        Upbit {
            ticker_refresh_period: 1000,
            all_tickers: vec![],
            last_fetched: Utc.ymd(1, 1, 1).and_hms(1, 1, 1),
            name: ExchangeName::UPBIT,
            http_client: Client::new(),
        }
    }

    async fn get_all_market_str(&self) -> Result<String, FetchError> {
        let body = self
            .http_client
            .get("https://api.upbit.com/v1/market/all")
            .send()
            .await?
            .text()
            .await?;
        let parsed_body_array = match serde_json::from_str(&body)? {
            serde_json::Value::Array(a) => a,
            _ => return Ok(String::from("")),
        };
        Ok(parsed_body_array
            .iter()
            .map(|m| match m["market"].clone() {
                serde_json::Value::String(m) => m,
                _ => String::from(""),
            })
            .collect::<Vec<String>>()
            .join(","))
    }

    async fn refetch_all_tickers(&mut self) -> Result<(), FetchError> {
        let url = format!(
            "https://api.upbit.com/v1/ticker?markets={}",
            self.get_all_market_str().await?
        );

        let body = self.http_client.get(&url).send().await?.text().await?;

        let parsed_body_array = match serde_json::from_str(&body)? {
            serde_json::Value::Array(a) => a,
            _ => return Ok(()),
        };
        let tickers = parsed_body_array
            .iter()
            .map(|data| {
                let market = match data["market"].clone() {
                    serde_json::Value::String(s) => s,
                    _ => String::from(""),
                };

                let trade_price = match data["trade_price"].clone() {
                    serde_json::Value::Number(l) => l.as_f64().unwrap_or(0.0),
                    serde_json::Value::String(l) => l.parse::<f64>().unwrap_or(0.0),
                    _ => 0.0,
                };

                (market, trade_price)
            })
            .filter(|data| match parse_market_string(&data.0, self.name) {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|data| Ticker {
                market: parse_market_string(&data.0, self.name).unwrap(),
                last: data.1,
            })
            .collect();

        self.all_tickers = tickers;
        self.last_fetched = Utc::now();

        Ok(())
    }

    async fn get_ticker(&mut self, market: Option<Market>) -> Result<Vec<Ticker>, FetchError> {
        if Utc::now()
            .signed_duration_since(self.last_fetched)
            .num_milliseconds()
            > self.ticker_refresh_period as i64
        {
            self.refetch_all_tickers().await?;
        }

        match market {
            Some(m) => {
                let filtered: Vec<Ticker> = self
                    .all_tickers
                    .iter()
                    .filter(|t| t.market == m)
                    .cloned()
                    .collect();
                Ok(filtered)
            }
            None => Ok(self.all_tickers.clone()),
        }
    }
}

#[async_trait]
impl Exchange for Upbit {
    fn get_name(&self) -> ExchangeName {
        self.name
    }
    async fn fetch_ticker(&mut self, market: Market) -> Result<Vec<Ticker>, FetchError> {
        self.get_ticker(Some(market)).await
    }

    async fn fetch_tickers(&mut self) -> Result<Vec<Ticker>, FetchError> {
        self.get_ticker(None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn upbit_fetch_all_tickers() {
        let mut upbit = Upbit::new();
        upbit.refetch_all_tickers().await.unwrap();
        let btc_pair = upbit
            .all_tickers
            .iter()
            .find(|t| t.market == Market::KrwBtc)
            .unwrap();

        assert!(btc_pair.last > 0.0);
    }
}
