use crate::{Generator, ValueResult};
use core::num::Wrapping;

/// Trait to represent types that can be created by summing up a generator.
///
/// The trait is used to implement the [`sum()`] method on generators. Types which implement this
/// trait can be generated by the [`sum()`] method. This trait is generally interacted with via
/// [`GeneratorExt::sum`].
///
/// [`GeneratorExt::sum`]: crate::GeneratorExt::sum
/// [`sum()`]: crate::traits::Sum::sum
///
pub trait Sum<A = Self>: Sized {
    /// Calculate the sum from a given generator.
    fn sum<G>(gen: G) -> Self
    where
        G: Generator<Output = A>;
}

/// Trait to represent types that can be created by multiplying values from a generator.
///
/// The trait is used to implement the [`product()`] method on generators. Types which implement this
/// trait can be generated by the [`product()`] method. This trait is generally interacted with via
/// [`GeneratorExt::product`].
///
/// [`GeneratorExt::product`]: crate::GeneratorExt::product
/// [`product()`]: crate::traits::Product::product
///
pub trait Product<A = Self>: Sized {
    /// Calculate the product using the given generator.
    fn product<G>(gen: G) -> Self
    where
        G: Generator<Output = A>;
}

macro_rules! integer_sum_product {
    (@impls $zero:expr, $one:expr, $($a:ty)*) => ($(
    impl Sum for $a {
        #[inline]
        fn sum<G: Generator<Output=Self>>(mut gen: G) -> Self {
            let mut ret = $zero;
            gen.run(
                |x| {
                    ret += x;
                    ValueResult::MoreValues
                }
            );
            ret
        }
    }

    impl<'a> Sum<&'a $a> for $a {
        #[inline]
        fn sum<G: Generator<Output=&'a Self>>(mut gen: G) -> Self {
            let mut ret = $zero;
            gen.run(
                |x| {
                    ret += x;
                    ValueResult::MoreValues
                }
            );
            ret
        }
    }

    impl Product for $a {
        #[inline]
        fn product<G: Generator<Output=Self>>(mut gen: G) -> Self {
            let mut ret = $one;
            gen.run(|x| {
                ret *= x;
                ValueResult::MoreValues
            }
            );
            ret
        }
    }

    impl<'a> Product<&'a $a> for $a {
        #[inline]
        fn product<G: Generator<Output=&'a Self>>(mut gen: G) -> Self {
            let mut ret = $one;
            gen.run(|x| {
                ret *= x;
                ValueResult::MoreValues
            }
            );
            ret
        }
    }
    )*);

    ($($a:ty)*) => (
        integer_sum_product!(@impls 0, 1,
                $($a)*);
        integer_sum_product!(@impls Wrapping(0), Wrapping(1),
                $(Wrapping<$a>)*);
    );
}

macro_rules! float_sum_product {
    ($($a:ty)*) => ($(
        impl Sum for $a {
        #[inline]
        fn sum<G: Generator<Output=Self>>(mut gen: G) -> Self {
            let mut ret = 0.0;
            gen.run(
                |x| {
                    ret += x;
                    ValueResult::MoreValues
                }
            );
            ret
        }
    }

    impl<'a> Sum<&'a $a> for $a {
        #[inline]
        fn sum<G: Generator<Output=&'a Self>>(mut gen: G) -> Self {
            let mut ret = 0.0;
            gen.run(
                |x| {
                    ret += x;
                    ValueResult::MoreValues
                }
            );
            ret
        }
    }

    impl Product for $a {
        #[inline]
        fn product<G: Generator<Output=Self>>(mut gen: G) -> Self {
            let mut ret = 1.0;
            gen.run(|x| {
                ret *= x;
                ValueResult::MoreValues
            }
            );
            ret
        }
    }

    impl<'a> Product<&'a $a> for $a {
        #[inline]
        fn product<G: Generator<Output=&'a Self>>(mut gen: G) -> Self {
            let mut ret = 1.0;
            gen.run(|x| {
                ret *= x;
                ValueResult::MoreValues
            }
            );
            ret
        }
    })*)
}

integer_sum_product! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
float_sum_product! { f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoGenerator, SliceGenerator};

    #[test]
    fn sum() {
        let data = [1, 2, 3, 4];
        assert_eq!(i32::sum(data.into_gen()), 10);
        assert_eq!(i32::sum(SliceGenerator::new(&data)), 10);

        let data = [Wrapping(u32::MAX), Wrapping(1)];
        assert_eq!(Wrapping::<u32>::sum(data.into_gen()), Wrapping(0));
    }

    #[test]
    fn product() {
        let data = [2, 3, 4];
        let expected = 2 * 3 * 4;
        assert_eq!(i32::product(data.into_gen()), expected);
        assert_eq!(i32::product(SliceGenerator::new(&data)), expected);
    }
}
