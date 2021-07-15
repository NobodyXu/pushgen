use crate::{Generator, GeneratorResult, SliceGenerator, ValueResult, ErasedFnPointer};

pub struct StoppingGen<'a, T> {
    stop_at: i32,
    stopped_data: Option<&'a T>,
    data: SliceGenerator<'a, T>,
}

impl<'a, T> StoppingGen<'a, T> {
    pub fn new(stop_at: i32, data: &'a [T]) -> Self {
        Self {
            stop_at,
            stopped_data: None,
            data: SliceGenerator::new(data),
        }
    }
}

impl<'a, T> Generator for StoppingGen<'a, T> {
    type Output = &'a T;

    fn run(&mut self, output: ErasedFnPointer<Self::Output, ValueResult>) -> GeneratorResult {
        if self.stop_at == 0 {
            self.stop_at -= 1;
            return GeneratorResult::Stopped;
        }

        if let Some(x) = self.stopped_data.take() {
            if output.call(x) == ValueResult::Stop {
                return GeneratorResult::Stopped;
            }
        }

        let mut tup = (&mut self.stopped_data, &mut self.stop_at, output);

        let result = self.data.run(ErasedFnPointer::from_associated(&mut tup, |tup, x| {
            let (stored_stop, stop_at, output) = tup;

            let old_stop_at = **stop_at;
            **stop_at -= 1;
            if old_stop_at == 0 {
                **stored_stop = Some(x);
                ValueResult::Stop
            } else {
                output.call(x)
            }
        }));
        if result == GeneratorResult::Complete {
            self.stop_at = -1;
        }
        result
    }
}
