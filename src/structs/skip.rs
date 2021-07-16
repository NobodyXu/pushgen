use crate::{run_gen, ErasedFnPointer, Generator, GeneratorResult, ValueResult};

/// Skip over a set amount of values. See [`.skip()`](crate::GeneratorExt::skip) for more details.
pub struct Skip<Gen> {
    generator: Gen,
    amount: usize,
}

impl<Gen> Skip<Gen> {
    #[inline]
    pub(crate) fn new(generator: Gen, amount: usize) -> Self {
        Self { generator, amount }
    }
}

impl<Gen> Generator for Skip<Gen>
where
    Gen: Generator,
{
    type Output = Gen::Output;

    #[inline]
    fn run(&mut self, output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        if self.amount > 0 {
            let skip_run = run_gen(&mut self.generator, &mut self.amount, |amount, _| {
                *amount -= 1;
                (*amount != 0).into()
            });

            if skip_run == GeneratorResult::Complete {
                return GeneratorResult::Complete;
            } else if self.amount > 0 {
                return GeneratorResult::Stopped;
            }
        }

        self.generator.run(output)
    }
}
