//! A nested threshold system to complement
//! [Shamir's Secret Sharing (SSS)](https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing)
//! groups with an arbitrary amount of additional secrets, at no additional
//! storage burden on the part of the shareholders.
//!
//! See [the README](https://github.com/conduition/qudoku) for a general
//! description of what this package does.
#![doc = include_str!("../USAGE.md")]

mod hashing;
mod ops;
mod polynomials;
mod sharing;

pub use hashing::*;
pub use polynomials::*;
pub use sharing::*;

// Re-Exports
pub use secp;
pub use sha2;

#[cfg(feature = "rand")]
pub fn random_coefficients<R: rand::RngCore + rand::CryptoRng>(
    rng: &mut R,
    n: usize,
) -> Vec<secp::MaybeScalar> {
    (0..n).map(|_| secp::Scalar::random(rng).into()).collect()
}
