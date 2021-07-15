use crate::{Generator, GeneratorResult, ValueResult, ErasedFnPointer};

/// Implements a chained generator. See [`.chain()`](crate::GeneratorExt::chain) for details.
pub struct Chain<First, Second> {
    first: First,
    second: Second,
    first_active: bool,
}

impl<First, Second> Chain<First, Second> {
    #[inline]
    pub(crate) fn new(first: First, second: Second) -> Self {
        Self {
            first,
            second,
            first_active: true,
        }
    }
}

impl<First, Second> Generator for Chain<First, Second>
where
    First: Generator,
    Second: Generator<Output = First::Output>,
{
    type Output = First::Output;

    #[inline]
    fn run(&mut self, output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        if self.first_active {
            let result = self.first.run(output);
            if result == GeneratorResult::Stopped {
                return GeneratorResult::Stopped;
            }
            self.first_active = false;
        }
        self.second.run(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::chain::Chain;
    use crate::SliceGenerator;
    use crate::{Generator, GeneratorResult, ValueResult, ErasedFnPointer};

    #[test]
    fn basic_chain() {
        let data = [1, 2, 3];
        let mut output: Vec<i32> = Vec::new();
        let result = Chain::new(SliceGenerator::new(&data), SliceGenerator::new(&data))
            .run(
                ErasedFnPointer::from_associated(&mut output, |output, x| {
                    output.push(*x);
                    ValueResult::MoreValues
                }
            )
        );

        assert_eq!(result, GeneratorResult::Complete);
        assert_eq!(output, [1, 2, 3, 1, 2, 3]);
    }
}
