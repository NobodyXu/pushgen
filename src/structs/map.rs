use crate::{Generator, GeneratorResult, ValueResult, ErasedFnPointer};

/// Implements a mapped generator. See [`.map()`](crate::GeneratorExt::map) for details.
pub struct Map<Gen, Func> {
    source: Gen,
    transform: Func,
}

impl<Gen, Func, Out> Map<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Out,
{
    #[inline]
    pub fn new(source: Gen, transform: Func) -> Self {
        Self { source, transform }
    }
}

impl<Gen, Func, Out> Generator for Map<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Out,
{
    type Output = Out;

    #[inline]
    fn run(&mut self, output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        let mut pair = (&mut self.transform, output);
        self.source.run(ErasedFnPointer::from_associated(&mut pair, |pair, value| {
            let (transform, output) = pair;
            output.call(transform(value))
        }))
    }
}
