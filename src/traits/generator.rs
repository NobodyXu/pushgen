use crate::{GeneratorResult, ValueResult};
use core::num::NonZeroUsize;
use either::Either;

/// Trait for generating values into a closure.
///
/// When a `Generator` is [`run()`](crate::Generator::run) it generates values that are fed an `output` closure.
/// It continues to feed values to the closure for as long as it can, unless the closure returns
/// [`ValueResult::Stop`](crate::ValueResult::Stop).
///
/// When all values have been generated the `run()` method returns [`GeneratorResult::Complete`](crate::GeneratorResult::Complete).
/// If `output` returns [`ValueResult::Stop`](crate::ValueResult::Stop) for any value
/// the generator must not call `output` with any further values and return [`GeneratorResult::Stopped`](crate::GeneratorResult::Stopped)
/// as well.
///
/// **The generator must not assume that it won't be called again after it returns**.
///
/// ## Example
///
/// A generic generator can be written like this:
/// ```
/// use pushgen::{Generator, ValueResult, GeneratorResult};
/// struct GenericGenerator<Out, Gen>
/// where
///     Gen: FnMut() -> Option<Out>,
/// {
///     generator: Gen,
/// }
///
/// impl<Out, Gen> Generator for GenericGenerator<Out, Gen>
///     where
///         Gen: FnMut() -> Option<Out>,
/// {
///     type Output = Out;
///
///     fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
///         while let Some(value) = (self.generator)() {
///             if output(value) == ValueResult::Stop {
///                 return GeneratorResult::Stopped;
///             }
///         }
///         GeneratorResult::Complete
///     }
/// }
/// ```
pub trait Generator {
    /// Data-type generated by the generator.
    type Output;

    /// Run the generator, emitting values to the `output` closure.
    ///
    /// New values are emitted for
    /// as long as the closure returns [`ValueResult::MoreValues`](crate::ValueResult::MoreValues).
    /// If the closure returns [`ValueResult::Stop`](crate::ValueResult::Stop) the generator **must**
    /// return [`GeneratorResult::Stopped`](crate::GeneratorResult::Stopped).
    fn run(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult;

    /// Try to advance the generator `n` values, ignoring them.
    ///
    /// This function has a default implementation but should be implemented by adaptors and
    /// source generators for additional performance gains.
    ///
    /// ## Returns
    ///
    /// The number of steps that the generator was actually advanced, and if the generator was
    /// stopped or completed.
    ///
    /// ## Examples
    /// ```
    /// use pushgen::{IntoGenerator, Generator, GeneratorExt, GeneratorResult};
    /// use core::num::NonZeroUsize;
    /// let data = [1, 2, 3, 4, 5];
    /// let mut gen = data.into_gen();
    /// let advance_result = gen.try_advance(NonZeroUsize::new(3).unwrap());
    /// assert_eq!(advance_result, (3, GeneratorResult::Stopped));
    /// assert_eq!(gen.next(), Ok(4));
    /// assert_eq!(gen.next(), Ok(5));
    /// ```
    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        let amount_to_advance = n.get();
        let mut amount_left = amount_to_advance;
        let result = self.run(|_| {
            amount_left -= 1;
            if amount_left == 0 {
                ValueResult::Stop
            } else {
                ValueResult::MoreValues
            }
        });

        (amount_to_advance - amount_left, result)
    }
}

/// A generator able to produce values from in reverse order.
///
/// A generator that implements `ReverseGenerator` can produce values in reverse order.
///
/// Both forward and reverse generation work on the same range and do not cross: a generator is complete
/// when the forward and reverse generator meets.
///
/// ## Examples
///
/// Basic usage:
///
/// ```
/// use pushgen::{SliceGenerator, GeneratorResult, GeneratorExt};
/// let numbers = [1, 2, 3, 4, 5, 6];
/// let mut gen = SliceGenerator::new(&numbers);
///
/// assert_eq!(Ok(&1), gen.next());
/// assert_eq!(Ok(&6), gen.next_back());
/// assert_eq!(Ok(&5), gen.next_back());
/// assert_eq!(Ok(&2), gen.next());
/// assert_eq!(Ok(&3), gen.next());
/// assert_eq!(Ok(&4), gen.next());
/// assert_eq!(Err(GeneratorResult::Complete), gen.next());
/// assert_eq!(Err(GeneratorResult::Complete), gen.next_back());
/// ```
pub trait ReverseGenerator: Generator {
    /// Run a generator backwards, producing values from the end to the beginning.
    fn run_back(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult;

    /// Tries to advance the generator from the back by `n` values.
    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        let amount_to_advance = n.get();
        let mut amount_left = amount_to_advance;
        let result = self.run_back(|_| {
            amount_left -= 1;
            if amount_left == 0 {
                ValueResult::Stop
            } else {
                ValueResult::MoreValues
            }
        });

        (amount_to_advance - amount_left, result)
    }
}

impl<L, R> Generator for Either<L, R>
where
    L: Generator,
    R: Generator<Output = L::Output>,
{
    type Output = L::Output;

    #[inline]
    fn run(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        match self {
            Either::Left(left) => left.run(output),
            Either::Right(right) => right.run(output),
        }
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        match self {
            Either::Left(left) => left.try_advance(n),
            Either::Right(right) => right.try_advance(n),
        }
    }
}

impl<L, R> ReverseGenerator for Either<L, R>
where
    L: ReverseGenerator,
    R: ReverseGenerator<Output = L::Output>,
{
    #[inline]
    fn run_back(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        match self {
            Either::Left(left) => left.run_back(output),
            Either::Right(right) => right.run_back(output),
        }
    }

    #[inline]
    fn try_advance_back(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        match self {
            Either::Left(left) => left.try_advance_back(n),
            Either::Right(right) => right.try_advance_back(n),
        }
    }
}

impl<T: Generator> Generator for &mut T {
    type Output = T::Output;

    #[inline]
    fn run(&mut self, output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        (**self).run(output)
    }

    #[inline]
    fn try_advance(&mut self, n: NonZeroUsize) -> (usize, GeneratorResult) {
        (**self).try_advance(n)
    }
}
