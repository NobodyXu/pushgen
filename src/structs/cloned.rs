use crate::{run_gen, ErasedFnPointer, Generator, GeneratorResult, ValueResult};

/// A generator that clones the elements of an underlying generator. See `[.cloned()](crate::GeneratorExt::cloned)
/// for details
pub struct Cloned<Src> {
    source: Src,
}

impl<Src> Cloned<Src> {
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self { source }
    }
}

impl<'a, Src, T> Generator for Cloned<Src>
where
    T: 'a + Clone,
    Src: Generator<Output = &'a T>,
{
    type Output = T;

    fn run(&mut self, mut output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        run_gen(&mut self.source, &mut output, |output, x| {
            output.call(x.clone())
        })
    }
}
