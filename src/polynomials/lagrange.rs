use crate::{Evaluation, Polynomial};
use std::ops::{Add, Mul, Sub};

/// [`secp::MaybeScalar`] does not implement [`std::ops::Div`] on itself
/// for safety reasons. The `UnsafeDiv` trait explicitly works around this.
pub trait UnsafeDiv<T> {
    type Output;

    fn unsafe_div(num: Self, denom: T) -> Self::Output;
}

mod unsafe_div_impls {
    use super::*;
    use secp::MaybeScalar;

    macro_rules! impl_unsafe_div {
        ( $($t:ty),* ) => {
            $(
                impl UnsafeDiv<$t> for $t {
                    type Output = $t;
                    fn unsafe_div(num: $t, denom: $t) -> Self::Output {
                        num / denom
                    }
                }
            )*
        };
    }

    impl_unsafe_div! {
       i8, i16, i32, i64, i128,
       u8, u16, u32, u64, u128,
       usize, f32, f64
    }

    impl UnsafeDiv<MaybeScalar> for MaybeScalar {
        type Output = MaybeScalar;

        fn unsafe_div(num: MaybeScalar, denom: MaybeScalar) -> Self::Output {
            match denom {
                MaybeScalar::Valid(d) => num / d,
                MaybeScalar::Zero => unreachable!("divided by zero scalar"),
            }
        }
    }
}

/// Evaluate a [Lagrange basis polynomial](https://en.wikipedia.org/wiki/Lagrange_polynomial).
///
/// This function returns:
/// - `0` if `x == evaluations[eval_index].input`
/// - `1` if `x == evaluations[i].input` for any other `i != eval_index`
///
/// The output is unpredictable for inputs which are not part of `evaluations`.
fn langrange_poly_evaluate<I, O>(evaluations: &[Evaluation<I, O>], eval_index: usize, x: I) -> I
where
    I: Copy
        + PartialEq
        + num_traits::One
        + num_traits::Zero
        + Sub<I, Output = I>
        + UnsafeDiv<I, Output = I>
        + Mul<I, Output = I>,
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

        // Invariant
        debug_assert!(
            !bottom.is_zero(),
            "shares include duplicate evaluation inputs, causing div-by-zero error"
        );
    }

    // top / bottom
    I::unsafe_div(top, bottom)
}

/// Represents a polynomial which can be evaluated using [Lagrange Interpolation]
/// on a set of evaluations.
///
/// [Lagrange Interpolation]: https://en.wikipedia.org/wiki/Lagrange_polynomial
pub struct LagrangePolynomial<I, O> {
    pub evaluations: Vec<Evaluation<I, O>>,
}

impl<I, O> LagrangePolynomial<I, O> {
    /// Construct a Lagrange Polynomial which interpolates the given set of evaluations.
    ///
    /// The evaluations are expected to have distinct input values.
    /// If two or more evaluations reuse the same input, evaluation and
    /// share-issuance will cause panics.
    pub fn new(evaluations: Vec<Evaluation<I, O>>) -> Self {
        Self { evaluations }
    }
}

impl<I, O> Polynomial<I, O> for LagrangePolynomial<I, O>
where
    I: Copy
        + PartialEq
        + num_traits::One
        + num_traits::Zero
        + Sub<I, Output = I>
        + UnsafeDiv<I, Output = I>
        + Mul<I, Output = I>,
    O: Copy,
    O: num_traits::Zero,
    O: Mul<I, Output = O>,
    O: Add<O, Output = O>,
{
    fn evaluate(&self, x: I) -> O {
        let mut out = O::zero();

        for (i, eval) in self.evaluations.iter().enumerate() {
            out = out + eval.output * langrange_poly_evaluate(&self.evaluations, i, x)
        }

        out
    }

    fn degree(&self) -> usize {
        match self.evaluations.len() {
            0 => 0,
            t => t - 1,
        }
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
