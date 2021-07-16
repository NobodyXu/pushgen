use crate::{
    run_gen, structs::utility::set_some, ErasedFnPointer, Generator, GeneratorResult,
    IntoGenerator, ValueResult,
};

/// Flatten generator implementation. See [`.flatten()`](crate::GeneratorExt::flatten) for details.
pub struct Flatten<Src>
where
    Src: Generator,
    Src::Output: IntoGenerator,
{
    source: Src,
    current_generator: Option<<Src::Output as IntoGenerator>::IntoGen>,
}

impl<Src> Flatten<Src>
where
    Src: Generator,
    Src::Output: IntoGenerator,
{
    #[inline]
    pub(crate) fn new(source: Src) -> Self {
        Self {
            source,
            current_generator: None,
        }
    }
}

impl<Src> Generator for Flatten<Src>
where
    Src: Generator,
    Src::Output: IntoGenerator,
{
    type Output = <<Src as Generator>::Output as IntoGenerator>::Output;

    #[inline]
    fn run(&mut self, output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        if let Some(current) = self.current_generator.as_mut() {
            if current.run(output) == GeneratorResult::Stopped {
                return GeneratorResult::Stopped;
            }
        }

        run_gen(
            &mut self.source,
            &mut (&mut self.current_generator, output),
            |pair, x| {
                let (current_generator, output) = pair;
                match set_some(*current_generator, x.into_gen()).run(*output) {
                    GeneratorResult::Stopped => ValueResult::Stop,
                    GeneratorResult::Complete => ValueResult::MoreValues,
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeneratorExt, SliceGenerator};

    #[test]
    fn vector_flatten() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10]];
        let mut output: Vec<i32> = Vec::new();
        let result = SliceGenerator::new(data.as_slice())
            .map(|x| SliceGenerator::new(x.as_slice()))
            .flatten()
            .for_each(|x| output.push(*x));

        assert_eq!(output, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(result, GeneratorResult::Complete);
    }

    #[test]
    fn slice_flatten() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let mut output = Vec::new();
        let result = SliceGenerator::new(&data)
            .map(|x| SliceGenerator::new(x))
            .flatten()
            .for_each(|x| output.push(*x));
        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn stopping_generator() {
        let data = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for x in 0..10 {
            let mut gen = crate::test::StoppingGen::new(x, &data)
                .map(|x| SliceGenerator::new(x))
                .flatten();

            let mut output = Vec::new();

            while gen.for_each(|x| output.push(*x)) == GeneratorResult::Stopped {}

            assert_eq!(output, expected);
        }
    }
}
