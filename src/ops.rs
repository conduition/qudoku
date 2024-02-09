use crate::{
    Evaluation, InterpolatedPointPolynomial, InterpolatedSecretPolynomial, PointSharingPolynomial,
    SecretSharingPolynomial,
};
use secp::{Point, G};
use std::ops::Mul;

/// Allows multiplying a secret sharing polynomial by a given fixed point.
impl Mul<&SecretSharingPolynomial> for Point {
    type Output = PointSharingPolynomial;

    fn mul(self, rhs: &SecretSharingPolynomial) -> Self::Output {
        let mut point_coeffs = Vec::with_capacity(rhs.degree() + 1);
        for &scalar in rhs.coefficients.iter() {
            point_coeffs.push(scalar * self);
        }
        PointSharingPolynomial::new(point_coeffs)
    }
}
impl Mul<Point> for &SecretSharingPolynomial {
    type Output = PointSharingPolynomial;
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}
impl Mul<SecretSharingPolynomial> for Point {
    type Output = PointSharingPolynomial;
    fn mul(self, rhs: SecretSharingPolynomial) -> Self::Output {
        self * &rhs
    }
}
impl Mul<Point> for SecretSharingPolynomial {
    type Output = PointSharingPolynomial;
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

/// Allows multiplying a secret sharing polynomial by the secp256k1 generator point.
impl Mul<&SecretSharingPolynomial> for G {
    type Output = PointSharingPolynomial;
    fn mul(self, rhs: &SecretSharingPolynomial) -> Self::Output {
        rhs * Point::generator()
    }
}
impl Mul<G> for &SecretSharingPolynomial {
    type Output = PointSharingPolynomial;
    fn mul(self, _: G) -> Self::Output {
        self * Point::generator()
    }
}
impl Mul<SecretSharingPolynomial> for G {
    type Output = PointSharingPolynomial;
    fn mul(self, rhs: SecretSharingPolynomial) -> Self::Output {
        rhs * Point::generator()
    }
}
impl Mul<G> for SecretSharingPolynomial {
    type Output = PointSharingPolynomial;
    fn mul(self, _: G) -> Self::Output {
        self * Point::generator()
    }
}

/// Allows multiplying an interpolated secret-sharing polynomial by a given fixed point.
impl Mul<&InterpolatedSecretPolynomial> for Point {
    type Output = InterpolatedPointPolynomial;

    fn mul(self, rhs: &InterpolatedSecretPolynomial) -> Self::Output {
        let point_evaluations = rhs
            .evaluations
            .iter()
            .map(|eval| Evaluation {
                input: eval.input,
                output: eval.output * self,
            })
            .collect();

        InterpolatedPointPolynomial::new(point_evaluations)
    }
}
impl Mul<Point> for &InterpolatedSecretPolynomial {
    type Output = InterpolatedPointPolynomial;
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}
impl Mul<InterpolatedSecretPolynomial> for Point {
    type Output = InterpolatedPointPolynomial;
    fn mul(self, rhs: InterpolatedSecretPolynomial) -> Self::Output {
        self * &rhs
    }
}
impl Mul<Point> for InterpolatedSecretPolynomial {
    type Output = InterpolatedPointPolynomial;
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

/// Allows multiplying an interpolated secret-sharing polynomial by the secp256k1 generator point.
impl Mul<&InterpolatedSecretPolynomial> for G {
    type Output = InterpolatedPointPolynomial;

    fn mul(self, rhs: &InterpolatedSecretPolynomial) -> Self::Output {
        rhs * Point::generator()
    }
}
impl Mul<G> for &InterpolatedSecretPolynomial {
    type Output = InterpolatedPointPolynomial;
    fn mul(self, _: G) -> Self::Output {
        self * Point::generator()
    }
}
impl Mul<InterpolatedSecretPolynomial> for G {
    type Output = InterpolatedPointPolynomial;
    fn mul(self, rhs: InterpolatedSecretPolynomial) -> Self::Output {
        &rhs * Point::generator()
    }
}
impl Mul<G> for InterpolatedSecretPolynomial {
    type Output = InterpolatedPointPolynomial;
    fn mul(self, _: G) -> Self::Output {
        &self * Point::generator()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Polynomial;
    use secp::{MaybeScalar, Scalar};

    #[test]
    fn test_secret_sharing_mul_point() {
        let f = SecretSharingPolynomial::new(vec![
            MaybeScalar::from(4),
            MaybeScalar::from(1),
            MaybeScalar::from(8),
        ]);

        let Z1 = &f * G;

        // UNSAFE: do not use a Q point with a known dlog. Generate them using `hash_to_point`.
        let Q = G * Scalar::try_from(100000).unwrap();
        let Z2 = &f * Q;

        let i = MaybeScalar::from(49);
        assert_eq!(Z1.evaluate(i), f.evaluate(i) * G);
        assert_eq!(Z2.evaluate(i), f.evaluate(i) * Q);

        let secret_shares = (9..12)
            .map(MaybeScalar::from)
            .map(|x| Evaluation {
                input: x,
                output: f.evaluate(x),
            })
            .collect();
        let interpolated_f = InterpolatedSecretPolynomial::new(secret_shares);

        // Sanity check
        assert_eq!(interpolated_f.evaluate(i), f.evaluate(i));

        assert_eq!((&interpolated_f * G).evaluate(i), Z1.evaluate(i));
        assert_eq!((&interpolated_f * Q).evaluate(i), Z2.evaluate(i));
    }
}
