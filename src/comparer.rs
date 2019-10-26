use crate::common::{Exchange, ExchangeName, Ticker};
use crate::market::Market;

pub struct Comparer<'a> {
    config: ComparerConfig<'a>,
}

pub struct ComparerConfig<'a> {
    percent_diff_margin: f64,
    exchanges: Vec<&'a mut dyn Exchange>,
}

#[derive(Clone, Debug, Copy)]
pub struct Arbitrage {
    from: ExchangeName,
    to: ExchangeName,
    percent_diff: f64,
}

#[derive(Clone, Debug, Copy)]
struct TickerWithName {
    ticker: Ticker,
    name: ExchangeName,
}

impl<'a> Comparer<'a> {
    pub fn new(config: ComparerConfig) -> Comparer {
        Comparer { config: config }
    }

    pub fn compare(&mut self, markets: Vec<Market>) -> Vec<Arbitrage> {
        let mut res: Vec<Arbitrage> = vec![];

        for market in markets {
            let mut temp: Vec<TickerWithName> = vec![];

            for ex in self.config.exchanges.iter_mut() {
                let ex_price = ex.fetch_ticker(market).unwrap();
                temp.push(TickerWithName {
                    ticker: ex_price[0],
                    name: ex.get_name(),
                });
            }

            for i in 0..temp.len() {
                for j in i..temp.len() {
                    let higher = if temp[i].ticker.last > temp[j].ticker.last {
                        temp[i]
                    } else {
                        temp[j]
                    };
                    let lower = if temp[i].ticker.last > temp[j].ticker.last {
                        temp[j]
                    } else {
                        temp[i]
                    };

                    if (higher.ticker.last - lower.ticker.last) / lower.ticker.last
                        > self.config.percent_diff_margin
                    {
                        res.push(Arbitrage {
                            from: lower.name,
                            to: higher.name,
                            percent_diff: (higher.ticker.last - lower.ticker.last)
                                / lower.ticker.last,
                        })
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::korbit::Korbit;
    use crate::upbit::Upbit;

    #[test]
    fn compare_btc() {
        let mut upbit = Upbit::new();
        let mut korbit = Korbit::new();

        let mut comparer = Comparer::new(ComparerConfig {
            percent_diff_margin: 0.00000001,
            exchanges: vec![&mut upbit, &mut korbit],
        });

        let res = comparer.compare(vec![Market::KrwBtc]);

        assert!(res.len() != 0);
    }
}
