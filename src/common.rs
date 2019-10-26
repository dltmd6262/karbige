use crate::errors::FetchError;
use crate::market::Market;

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

pub trait Exchange {
    fn get_name(&self) -> ExchangeName;
    fn fetch_ticker(&mut self, market: Market) -> Result<Vec<Ticker>, FetchError>;
    fn fetch_tickers(&mut self) -> Result<Vec<Ticker>, FetchError>;
}
