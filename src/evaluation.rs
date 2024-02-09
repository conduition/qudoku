/// Represents a polynomial evaluation at a certain input and output, which
/// may be of different types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Evaluation<I, O> {
    pub input: I,
    pub output: O,
}

impl<I, O> Evaluation<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Evaluation { input, output }
    }
}
