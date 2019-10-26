//! Karbige, a rust library for checking arbitrage opportunities
//! in Korean crypto exchanges.
//!
//! # Quick Start
//!
//! The basic flow is to create an instance of Exchange structs and
//! passing them to the comparator, and then using the [`compare()`]
//! function to compare the markets and look for arbitrage opportunities.
//!
//! ```
//!
//! use karbige::{upbit::Upbit, korbit::Korbit, comparer::Comparer, market::Market}
//!
//! let mut upbit = Upbit::new();
//! let mut korbit = Korbit::new();
//!
//! let mut comparer = Comparer::new(ComparerConfig {
//!     percent_diff_margin: 0.01,
//!     exchanges: vec![&mut upbit, &mut korbit],
//! });
//!
//! let res = comparer.compare(vec![Market::KrwBtc]); // returns array of Arbitrage structs
//! println!("Arbitrage opportunities are: {:?}", res);
//!
//! ```

mod util;

pub mod common;
pub mod comparer;
pub mod errors;
pub mod korbit;
pub mod market;
pub mod upbit;
