# Usage

This example demonstrates basic Shamir Secret Sharing, and then extends it by creating derivative secrets.

```rust
#![allow(non_snake_case)]
#![cfg(feature = "rand")]
use secp::{MaybeScalar, Scalar};
use qudoku::{
  InterpolatedPointPolynomial, InterpolatedSecretPolynomial, Polynomial, SecretShare, SecretSharingPolynomial,
};

// Construct a secret sharing polynomial.
let primary_secret = MaybeScalar::from_hex(
  "efe45825dcdc69bd70f09fba9930835558aebf043cfc86c1c6c6b1925c2d2035"
).unwrap();

// Sample random coefficients to construct the secret polynomial.
let mut coefficients = qudoku::random_coefficients(&mut rand::rngs::OsRng, 3);
coefficients[0] = primary_secret; // Set f(0) = secret
let secret_polynomial = SecretSharingPolynomial::new(coefficients);

// Tests the polynomial is correct.
assert_eq!(secret_polynomial.evaluate(MaybeScalar::Zero), primary_secret);
assert_ne!(secret_polynomial.evaluate(MaybeScalar::one()), primary_secret);

// Issue some shares of the secret polynomial.
let shares: Vec<SecretShare> = (1..6)
  .map(MaybeScalar::from)
  .map(|i| secret_polynomial.issue_share(i))
  .collect();

// The shares have threshold of polynomial.degree() + 1 = 3.
assert_eq!(secret_polynomial.degree(), 2);

// Interpolate the polynomial using three or more shares.
let interpolated_polynomial = InterpolatedSecretPolynomial::new(
  shares[2..5].iter().cloned().collect(),
);

// Recover the primary secret.
assert_eq!(interpolated_polynomial.evaluate(MaybeScalar::Zero), primary_secret);

/**********************************************
   Now is where the qudoku custom logic starts.
 **********************************************/

// Pick a provably-honest `Q` point by hashing some public input data:
let Q = qudoku::hash_to_point(b"hello world!");

// Construct a Q-ified version of the secret-sharing polynomial.
//
//   Z(x) = f(x) * Q
let Z = Q * secret_polynomial;

// Pre-generate and publish some number of shares of Z(x).
let mut z_shares = vec![
  Z.issue_share(Scalar::random(&mut rand::rngs::OsRng).into()),
  Z.issue_share(Scalar::random(&mut rand::rngs::OsRng).into()),
];

// Now Z(x) effectively has a threshold of 1. Anyone who knows the preshares
// only needs 1 additional share to interpolate Z(x). Any shareholder can
// produce such a share.
z_shares.push(shares[4] * Q);

// We can now interpolate Z(x) and derive the secret c.
let interpolated_Z = InterpolatedPointPolynomial::new(z_shares);
assert_eq!(
  interpolated_Z.evaluate(MaybeScalar::Zero),
  Z.evaluate(MaybeScalar::Zero)
);

// The same primary secret + the same Q point = a consistent secret c.
let c = interpolated_Z.derive_secret(MaybeScalar::Zero);
assert_eq!(
  hex::encode(&c),
  "ec4f719940a443dd5377124a07e12424757db3e0976e9d206c5652ae3441c545"
);
```
