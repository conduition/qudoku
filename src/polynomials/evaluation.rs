/// Represents a polynomial evaluation at a certain input and output, which
/// may be of different types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Evaluation<I, O> {
    /// The input `x` value which is fed into a polynomial function.
    pub input: I,

    /// The output `y` value which a polynomial function outputs.
    pub output: O,
}

impl<I, O> Evaluation<I, O> {
    /// Construct a new polynomial evaluation, which is effectively a
    /// tuple of `(input, output)`.
    pub fn new(input: I, output: O) -> Self {
        Evaluation { input, output }
    }
}
