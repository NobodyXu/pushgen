use crate::{Generator, GeneratorResult, ValueResult, ErasedFnPointer};

/// Implements a filtered generator. See [`.filter()`](crate::GeneratorExt::filter) for more details.
pub struct Filter<Gen, Pred> {
    generator: Gen,
    predicate: Pred,
}

impl<Gen, Pred> Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    #[inline]
    pub(crate) fn new(generator: Gen, predicate: Pred) -> Self {
        Self {
            generator,
            predicate,
        }
    }
}

impl<Gen, Pred> Generator for Filter<Gen, Pred>
where
    Gen: Generator,
    Pred: FnMut(&Gen::Output) -> bool,
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, mut output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        let mut pair = (&mut self.predicate, &mut output);
        self.generator.run(
            ErasedFnPointer::from_associated(&mut pair, |pair, x| {
                let (predicate, output) = pair;
                if predicate(&x) {
                    output.call(x)
                } else {
                    ValueResult::MoreValues
                }
            })
        )
    }
}
