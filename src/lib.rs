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
