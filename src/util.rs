use crate::common::ExchangeName;
use crate::errors::PairParsingError;
use crate::market::Market;

const KORBIT_MARKETS: [(&'static str, Market); 3] = [
    ("btc_krw", Market::KrwBtc),
    ("eth_krw", Market::KrwEth),
    ("etc_krw", Market::KrwEtc),
];

const UPBIT_MARKETS: [(&'static str, Market); 3] = [
    ("KRW-BTC", Market::KrwBtc),
    ("KRW-ETH", Market::KrwEth),
    ("KRW-ETC", Market::KrwEtc),
];

pub fn parse_market_string(
    market_string: &str,
    ex: ExchangeName,
) -> Result<Market, PairParsingError> {
    let market_const = match ex {
        ExchangeName::KORBIT => KORBIT_MARKETS,
        ExchangeName::UPBIT => UPBIT_MARKETS,
    };

    for &(s, c) in &market_const {
        if s == market_string {
            return Ok(c);
        }
    }

    Err(PairParsingError)
}

pub fn get_market_string(market: Market, ex: ExchangeName) -> Result<String, PairParsingError> {
    let market_const = match ex {
        ExchangeName::KORBIT => KORBIT_MARKETS,
        ExchangeName::UPBIT => UPBIT_MARKETS,
    };

    for &(s, c) in &market_const {
        if c == market {
            return Ok(s.to_string());
        }
    }

    Err(PairParsingError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upbit_market_str_conversion() {
        let market = parse_market_string("KRW-BTC", ExchangeName::UPBIT).unwrap();
        assert_eq!(market, Market::KrwBtc);

        let market_str = get_market_string(Market::KrwBtc, ExchangeName::UPBIT).unwrap();
        assert_eq!(market_str, "KRW-BTC");
    }

    #[test]
    fn korbit_market_str_conversion() {
        let market = parse_market_string("btc_krw", ExchangeName::KORBIT).unwrap();
        assert_eq!(market, Market::KrwBtc);

        let market_str = get_market_string(Market::KrwBtc, ExchangeName::KORBIT).unwrap();
        assert_eq!(market_str, "btc_krw");
    }
}
