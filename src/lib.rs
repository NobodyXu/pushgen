mod generator_ext;

pub mod structs;

pub use crate::generator_ext::GeneratorExt;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum ValueResult {
    Stop,
    MoreValues,
}

impl From<bool> for ValueResult {
    fn from(value: bool) -> Self {
        if !value {
            Self::Stop
        }
        else {
            Self::MoreValues
        }
    }
}


/// The result of generator runs.
///
/// A run can either run to completion, and no new values will
/// be produced, or it can be stopped. In case it is stopped there might be more values available
/// that can be obtained by calling [`Generator::run`](crate::Generator::run) again.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u8)]
pub enum GeneratorResult {
    /// Returned from `Generator::run` when the generator was stopped because the `output` function
    /// returned `ValueResult::Stop`
    Stopped,
    /// Returned from `Generator::run` when the generator has sent all values to the `output` function.
    /// When this has been returned the generator will never generate more values again.
    Complete
}

impl From<bool> for GeneratorResult {
    fn from(b: bool) -> Self {
        if !b {
            Self::Stopped
        }
        else {
            Self::Complete
        }
    }
}

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

    /// Run the generator, emitting values to the `output` closure. New values are emitted for
    /// as long as the closure returns [`ValueResult::MoreValues`](crate::ValueResult::MoreValues).
    /// If the closure returns [`ValueResult::Stop`](crate::ValueResult::Stop) the generator **must**
    /// return [`GeneratorResult::Stopped`](crate::GeneratorResult::Stopped).
    fn run(&mut self, output: impl FnMut(Self::Output) -> crate::ValueResult) -> GeneratorResult;
}

/// A generator that generates values from a slice.
///
///
/// ## Example
/// ```
/// # use pushgen::{SliceGenerator, GeneratorExt};
/// let data = [1, 2, 3, 4];
/// let mut sum = 0;
/// SliceGenerator::new(&data).for_each(|x| sum += x);
/// assert_eq!(sum, 10);
/// ```
pub struct SliceGenerator<'a, T>
{
    slice: &'a [T],
    index: usize
}

impl<'a, T> SliceGenerator<'a, T>
{
    pub fn new(slice: &'a[T]) -> Self {
        Self {
            slice,
            index: 0
        }
    }
}

impl<'a, T> Generator for SliceGenerator<'a, T> {
    type Output = &'a T;

    #[inline]
    fn run(&mut self, mut output: impl FnMut(Self::Output) -> ValueResult) -> GeneratorResult {
        // Read the len once. The Rust compiler seems to have trouble optimizing self.slice.len()
        // so read it once and use that in the loop condition instead.
        let len = self.slice.len();
        while self.index < len {
            // Safety: self.index < self.slice.len() always true.
            if output(unsafe { self.slice.get_unchecked(self.index) }) == ValueResult::Stop {
                self.index += 1;
                return GeneratorResult::Stopped;
            }
            self.index += 1;
        }
        GeneratorResult::Complete
    }
}
