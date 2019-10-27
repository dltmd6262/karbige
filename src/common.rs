use crate::errors::FetchError;
use crate::market::Market;
use async_trait::async_trait;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ExchangeName {
    KORBIT,
    UPBIT,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Ticker {
    pub market: Market,
    pub last: f64,
}

#[async_trait]
pub trait Exchange {
    fn get_name(&self) -> ExchangeName;
    async fn fetch_ticker(&mut self, market: Market) -> Result<Vec<Ticker>, FetchError>;
    async fn fetch_tickers(&mut self) -> Result<Vec<Ticker>, FetchError>;
}
