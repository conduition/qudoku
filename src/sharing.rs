use crate::{Evaluation, LagrangePolynomial, Polynomial, StandardFormPolynomial};
use secp::{MaybePoint, MaybeScalar};

/// Represents a secret share held by a shareholder.
pub type SecretShare = Evaluation<MaybeScalar, MaybeScalar>;

/// Represents a point share intended for distribution.
/// Derived by multiplying a secret share with a fixed point `Q`.
pub type PointShare = Evaluation<MaybeScalar, MaybePoint>;

/// Represents the secret-sharing polynomial available to the dealer in its
/// original standard form, composed of a set of scalar coefficients.
pub type SecretSharingPolynomial = StandardFormPolynomial<MaybeScalar>;

/// Represents the point-sharing polynomial available to the dealer in standard
/// form, composed of a set of point coefficients.
pub type PointSharingPolynomial = StandardFormPolynomial<MaybePoint>;

/// Represents a secret-sharing polynomial interpolated from a set of shares.
pub type InterpolatedSecretPolynomial = LagrangePolynomial<MaybeScalar, MaybeScalar>;

/// Represents a point-sharing polynomial interpolated from a set of shares.
pub type InterpolatedPointPolynomial = LagrangePolynomial<MaybeScalar, MaybePoint>;

macro_rules! impl_issue_share {
    ( $t:ty, $share:ty ) => {
        impl $t {
            /// Issue a share at the given `input` index.
            pub fn issue_share(&self, input: MaybeScalar) -> $share {
                Evaluation {
                    input,
                    output: self.evaluate(input),
                }
            }
        }
    };
}

impl_issue_share! { SecretSharingPolynomial, SecretShare }
impl_issue_share! { PointSharingPolynomial, PointShare }
impl_issue_share! { InterpolatedSecretPolynomial, SecretShare }
impl_issue_share! { InterpolatedPointPolynomial, PointShare }
