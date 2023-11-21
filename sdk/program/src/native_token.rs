//! Definitions for the native MLN token and its fractional lamports.

#![allow(clippy::arithmetic_side_effects)]

/// There are 10^9 lamports in one MLN
pub const LAMPORTS_PER_MLN: u64 = 1_000_000_000;

/// Approximately convert fractional native tokens (lamports) into native tokens (MLN)
pub fn lamports_to_mln(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_MLN as f64
}

/// Approximately convert native tokens (MLN) into fractional native tokens (lamports)
pub fn mln_to_lamports(mln: f64) -> u64 {
    (mln * LAMPORTS_PER_MLN as f64) as u64
}

use std::fmt::{Debug, Display, Formatter, Result};
pub struct Mln(pub u64);

impl Mln {
    fn write_in_mln(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "𝇊{}.{:09}",
            self.0 / LAMPORTS_PER_MLN,
            self.0 % LAMPORTS_PER_MLN
        )
    }
}

impl Display for Mln {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_mln(f)
    }
}

impl Debug for Mln {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.write_in_mln(f)
    }
}
