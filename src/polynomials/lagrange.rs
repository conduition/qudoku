use std::ops::{Add, Div, Mul, Sub};

use crate::{Evaluation, Polynomial};

/// Evaluate a [Lagrange basis polynomial](https://en.wikipedia.org/wiki/Lagrange_polynomial).
///
/// This function returns:
/// - `0` if `x == evaluations[eval_index].input`
/// - `1` if `x == evaluations[i].input` for any other `i != eval_index`
///
/// The output is unpredictable for inputs which are not part of `evaluations`.
fn langrange_poly_evaluate<I, O>(evaluations: &[Evaluation<I, O>], eval_index: usize, x: I) -> I
where
    I: Copy + PartialEq,
    I: num_traits::One + num_traits::Zero,
    I: Sub<I, Output = I>,
    I: Div<I, Output = I>,
    I: Mul<I, Output = I>,
{
    let xj = evaluations[eval_index].input;

    // Short-circuit for efficiency.
    if x == xj {
        return I::one();
    }

    // For efficiency we compute the numerator and denominator of the lagrange polynomial separately.
    let mut top = I::one();
    let mut bottom = I::one();

    for (i, eval) in evaluations.into_iter().enumerate() {
        if i == eval_index {
            continue;
        }

        top = top * (x - eval.input);

        // Short circuit for efficiency.
        if top.is_zero() {
            return top;
        }

        bottom = bottom * (xj - eval.input);
    }

    top / bottom
}

pub struct LagrangePolynomial<I, O> {
    pub evaluations: Vec<Evaluation<I, O>>,
}

impl<I, O> LagrangePolynomial<I, O> {
    /// The evaluations are expected to have distinct input values.
    /// If two or more evaluations reuse the same input, evaluation will cause panics.
    pub fn new(evaluations: Vec<Evaluation<I, O>>) -> Self {
        Self { evaluations }
    }
}

impl<I, O> Polynomial<I, O> for LagrangePolynomial<I, O>
where
    I: Copy + PartialEq,
    I: num_traits::One + num_traits::Zero,
    I: Sub<I, Output = I>,
    I: Div<I, Output = I>,
    I: Mul<I, Output = I>,
    O: Copy,
    O: num_traits::Zero,
    O: Mul<I, Output = O>,
    O: Add<O, Output = O>,
{
    // Uses [Lagrange Interpolation](https://en.wikipedia.org/wiki/Lagrange_polynomial).
    fn evaluate(&self, x: I) -> O {
        let mut out = O::zero();

        for (i, eval) in self.evaluations.iter().enumerate() {
            out = out + eval.output * langrange_poly_evaluate(&self.evaluations, i, x)
        }

        out
    }

    fn degree(&self) -> usize {
        self.evaluations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_langrange_poly_evaluate() {
        let evaluations = vec![
            Evaluation {
                input: 0,
                output: 4,
            },
            Evaluation {
                input: 1,
                output: 1,
            },
            Evaluation {
                input: 2,
                output: 3,
            },
        ];

        assert_eq!(langrange_poly_evaluate(&evaluations, 0, 0), 1);
        assert_eq!(langrange_poly_evaluate(&evaluations, 0, 1), 0);
        assert_eq!(langrange_poly_evaluate(&evaluations, 0, 2), 0);

        assert_eq!(langrange_poly_evaluate(&evaluations, 1, 0), 0);
        assert_eq!(langrange_poly_evaluate(&evaluations, 1, 1), 1);
        assert_eq!(langrange_poly_evaluate(&evaluations, 1, 2), 0);

        assert_eq!(langrange_poly_evaluate(&evaluations, 2, 0), 0);
        assert_eq!(langrange_poly_evaluate(&evaluations, 2, 1), 0);
        assert_eq!(langrange_poly_evaluate(&evaluations, 2, 2), 1);

        let poly = LagrangePolynomial::new(evaluations);

        for eval in poly.evaluations.iter() {
            assert_eq!(poly.evaluate(eval.input), eval.output);
        }
    }
}
