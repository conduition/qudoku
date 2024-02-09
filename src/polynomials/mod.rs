mod evaluation;
mod lagrange;
mod standard;

pub use evaluation::*;
pub use lagrange::*;
pub use standard::*;

pub trait Polynomial<I, O> {
    /// Evaluate the polynomial on a given input.
    fn evaluate(&self, input: I) -> O;

    /// Returns the degree of the polynomial, which is usually the number of coefficients
    /// minus 1. If the polynomial has no coefficients, it has degree zero.
    fn degree(&self) -> usize;

    /// Returns the number of evaluations needed to interpolate this polynomial,
    /// which is just the number of coefficients in the polynomial.
    fn interpolation_threshold(&self) -> usize {
        self.degree() + 1
    }
}
