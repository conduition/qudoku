use std::ops::{Add, Mul};

use crate::Polynomial;

/// Evaluate a standard-form polynomial using [Horner's method].
///
/// The `coefficients` are assumed to be presented in ascending order of degree,
/// starting with the constant term `coefficients[0]`.
///
/// The coefficient type `T`, input type `I`, and output type `O` are generic so
/// that this function can be reused for any kind of [`Polynomial`].
///
/// [Horner's method]: https://en.wikipedia.org/wiki/Horner%27s_method
fn horner_poly_evaluate<I, T, O>(x: I, coefficients: &[T]) -> O
where
    O: Copy,
    O: num_traits::Zero + Mul<I, Output = O> + Add<T, Output = O>,
    I: Copy,
    T: Copy,
{
    let mut out = O::zero();

    // Start from highest-degree coefficients.
    // Example with a degree 3 polynomial, with coefficients [a0, a1, a2, a3]:
    //   f(x) = a0 + x(a1 + x(a2 + x(a3)))
    for &a in coefficients.into_iter().rev() {
        out = out * x + a
    }

    out
}

/// Represents a polynomial function expressed in standard form with
/// coefficients of type `T`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StandardFormPolynomial<T> {
    /// The ordered set of coefficients, starting with the constant term.
    pub coefficients: Vec<T>,
}

impl<T> StandardFormPolynomial<T> {
    /// The `coefficients` are assumed to be presented in ascending order of degree,
    /// starting with the constant term `coefficients[0]`.
    ///
    /// Panics if coefficients is empty.
    pub fn new(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    /// Returns the degree of the polynomial, which is usually the number of coefficients
    /// minus 1. If the polynomial has no coefficients, it has degree zero. If trailing
    /// coefficients are zero, those are not counted towards the degree.
    pub fn degree(&self) -> usize
    where
        T: num_traits::Zero,
    {
        let mut degree = match self.coefficients.len() {
            0 => 0,
            t => t - 1,
        };

        // Do not count trailing zero coefficients.
        for coeff in self.coefficients.iter().rev() {
            if coeff.is_zero() && degree > 0 {
                degree -= 1;
            } else {
                break;
            }
        }

        return degree;
    }
}

impl<I, T> Polynomial<I, T> for StandardFormPolynomial<T>
where
    I: Copy,
    T: Copy + num_traits::Zero,
    T: Mul<I, Output = T> + Add<T, Output = T>,
{
    fn evaluate(&self, x: I) -> T {
        horner_poly_evaluate(x, self.coefficients.as_ref())
    }

    fn degree(&self) -> usize {
        StandardFormPolynomial::degree(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_degree() {
        assert_eq!(StandardFormPolynomial::<i32>::new(vec![]).degree(), 0);
        assert_eq!(StandardFormPolynomial::new(vec![0]).degree(), 0);
        assert_eq!(StandardFormPolynomial::new(vec![1]).degree(), 0);
        assert_eq!(StandardFormPolynomial::new(vec![2, 2]).degree(), 1);
        assert_eq!(StandardFormPolynomial::new(vec![3, 3, 3]).degree(), 2);
        assert_eq!(StandardFormPolynomial::new(vec![0, 0, 0]).degree(), 0);
    }

    #[test]
    fn test_polynomial_evaluate() {
        // f(x) = 1 + 3x + 2x^2
        let poly = StandardFormPolynomial::new(vec![1, 3, 2]);

        // f(0) = 1
        assert_eq!(poly.evaluate(0), 1);

        // f(1) = 1 + 3 + 2 = 6
        assert_eq!(poly.evaluate(1), 6);

        // f(2) = 1 + 6 + 8 = 15
        assert_eq!(poly.evaluate(2), 15);

        // f(3) = 1 + 9 + 18 = 28
        assert_eq!(poly.evaluate(3), 28);

        // f(4) = 1 + 12 + 32
        assert_eq!(poly.evaluate(4), 45);
    }
}
