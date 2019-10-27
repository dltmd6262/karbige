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
//! ```

mod util;

pub mod common;
pub mod comparer;
pub mod errors;
pub mod korbit;
pub mod market;
pub mod upbit;
