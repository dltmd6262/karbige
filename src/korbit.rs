use crate::common::{Exchange, ExchangeName, Ticker};
use crate::errors::FetchError;
use crate::market::Market;
use crate::util::parse_market_string;
use chrono::{DateTime, TimeZone, Utc};

pub struct Korbit {
    ticker_refresh_period: u64,
    all_tickers: Vec<Ticker>,
    last_fetched: DateTime<Utc>,
    name: ExchangeName,
}

impl Korbit {
    pub fn new() -> Korbit {
        Korbit {
            ticker_refresh_period: 1000,
            all_tickers: vec![],
            last_fetched: Utc.ymd(1, 1, 1).and_hms(1, 1, 1),
            name: ExchangeName::KORBIT,
        }
    }

    fn refetch_all_tickers(&mut self) -> Result<(), FetchError> {
        let body = reqwest::get("https://api.korbit.co.kr/v1/ticker/detailed/all")?.text()?;

        let parsed_body_map = match serde_json::from_str(&body)? {
            serde_json::Value::Object(map) => map,
            _ => return Ok(()),
        };
        let tickers = parsed_body_map
            .iter()
            .filter(|(pair, _)| match parse_market_string(pair, self.name) {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|(pair, data)| Ticker {
                market: parse_market_string(pair, self.name).unwrap(),
                last: match &data["last"] {
                    serde_json::Value::Number(l) => l.as_f64().unwrap_or(0.0),
                    serde_json::Value::String(l) => l.parse::<f64>().unwrap_or(0.0),
                    _ => 0.0,
                },
            })
            .collect();

        self.all_tickers = tickers;
        self.last_fetched = Utc::now();

        Ok(())
    }

    fn get_ticker(&mut self, market: Option<Market>) -> Result<Vec<Ticker>, FetchError> {
        if Utc::now()
            .signed_duration_since(self.last_fetched)
            .num_milliseconds()
            > self.ticker_refresh_period as i64
        {
            self.refetch_all_tickers()?;
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

impl Exchange for Korbit {
    fn get_name(&self) -> ExchangeName {
        self.name
    }

    fn fetch_ticker(&mut self, market: Market) -> Result<Vec<Ticker>, FetchError> {
        self.get_ticker(Some(market))
    }

    fn fetch_tickers(&mut self) -> Result<Vec<Ticker>, FetchError> {
        self.get_ticker(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn korbit_fetch_all_tickers() {
        let mut korbit = Korbit::new();
        korbit.refetch_all_tickers().unwrap();
        let btc_pair = korbit
            .all_tickers
            .iter()
            .find(|t| t.market == Market::KrwBtc)
            .unwrap();

        assert!(btc_pair.last > 0.0);
    }
}
