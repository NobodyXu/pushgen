use crate::{Generator, GeneratorResult, ValueResult, ErasedFnPointer};

/// Box a generator, type-erasing the actual generator type.
/// See [`.boxed()`](crate::GeneratorExt::boxed) for details.
pub struct BoxedGenerator<T> {
    source: Box<dyn Generator<Output = T>>,
}

impl<T> BoxedGenerator<T> {
    #[inline]
    pub(crate) fn new(source: impl Generator<Output = T> + 'static) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

impl<T> Generator for BoxedGenerator<T> {
    type Output = T;

    #[inline]
    fn run(&mut self, output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        self.source.as_mut().run(output)
    }
}
