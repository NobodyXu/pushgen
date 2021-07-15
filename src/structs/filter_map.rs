use crate::{Generator, GeneratorResult, ValueResult, ErasedFnPointer};

/// Implements a mapped generator. See [`.map()`](crate::GeneratorExt::map) for details.
pub struct FilterMap<Gen, Func> {
    source: Gen,
    transform: Func,
}

impl<Gen, Func, Out> FilterMap<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    #[inline]
    pub fn new(source: Gen, transform: Func) -> Self {
        Self { source, transform }
    }
}

impl<Gen, Func, Out> Generator for FilterMap<Gen, Func>
where
    Gen: Generator,
    Func: FnMut(Gen::Output) -> Option<Out>,
{
    type Output = Out;

    #[inline]
    fn run(&mut self, mut output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        let mut pair = (&mut self.transform, &mut output);

        self.source.run(
            ErasedFnPointer::from_associated(&mut pair, |pair, x| {
                let (transform, output) = pair;
                if let Some(x) = transform(x) {
                    output.call(x)
                } else {
                    ValueResult::MoreValues
                }
            })
        )
    }
}
