#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the secp256k1 curve.
//! Curve information: https://www.secg.org/sec2-v2.pdf
pub mod curves;
pub use curves::*;

pub mod fields;
pub use fields::*;

pub mod sec1;
pub use sec1::*;

pub mod test_vectors;
mod tests;
