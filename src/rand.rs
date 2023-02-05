//! Generate random angles with the [rand crate](https://docs.rs/rand/latest/rand/).
//!
//! ## Provided implementations
//!
//! [`Angle`] and [`AngleUnbounded`] (and [their unit wrapped equivalent][crate::units]) can be generated
//! with the [`Standard`] distribution, and so with [`rand::random`](https://docs.rs/rand/latest/rand/fn.random.html) with the following
//! ranges and distributions:
//!
//! - [`Angle`]: Uniformly distributed on the full circle.
//! - [`AngleUnbounded`]: Uniformly distributed in the range `(-π, π]` radians.
//!
//! **Note**: The unit wrapper has no influence on the generated value.
//!
//! ## Uniform ranges
//!
//! Angle types can also be generated from a range using [`rand::Rng::gen_range`].
//!
//! ### [`Angle`]
//!
//! Because [`Angle`] did not implements [`PartialOrd`], the generated angle will belong to the part of the
//! circle between the bounds in counterclockwise. I.e. the order of the bounds will determine
//! which part of the circle the generated angle belongs to.
//!
//! ```
//! # use angulus::*;
//! # use ::rand::*;
//! let top = Angle32::DEG_90;
//! let bottom = -Angle32::DEG_90;
//!
//! let mut rng = thread_rng();
//!
//! // Generate an angle on the "left" part of the circle.
//! let a = rng.gen_range(top..=bottom);
//! assert!(a.cos() <= 0.0);
//!
//! // Generate an angle on the "right" part of the circle.
//! let a = rng.gen_range(bottom..=top);
//! assert!(a.cos() >= 0.0);
//! ```
//!
//! ### [`AngleUnbounded`]
//!
//! Since [`AngleUnbounded`] implements [`PartialOrd`], the order matter and the range may be empty,
//! resulting in panic.
//!
//! ```no_run
//! # use angulus::*;
//! # use ::rand::*;
//! let low = AngleUnbounded32::ZERO;
//! let high = AngleUnbounded32::DEG_90;
//! let x = thread_rng().gen_range(low..=high);
//! ```
//!
//! ```should_panic
//! # use angulus::*;
//! # use ::rand::*;
//! # let low = AngleUnbounded32::ZERO;
//! # let high = AngleUnbounded32::DEG_90;
//! let x = thread_rng().gen_range(high..=low);
//! // panic: "cannot sample empty range"
//! ```

use std::ops::{Range, RangeInclusive};

use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleRange, SampleUniform, UniformFloat, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

use crate::{
    float::Float,
    units::{Degrees, Gradians, Radians, Turns},
    Angle, AngleUnbounded,
};

//-------------------------------------------------------------------
// Standard Distribution
//-------------------------------------------------------------------

impl<F: Float> Distribution<Angle<F>> for Standard
where
    Self: Distribution<F>,
{
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Angle<F> {
        // Standard distribution generates float in the range [0, 1).
        let x = rng.gen::<F>();

        let x = x * F::TAU; // [0, 1) --> [0, TAU)
        let x = x - F::PI; // [0, TAU) --> [-PI, PI)
        let x = -x; // [-PI, PI) --> (-PI, PI]

        debug_assert!(-F::PI < x && x <= F::PI);

        Angle::from_radians_unchecked(x)
    }
}

impl<F: Float> Distribution<AngleUnbounded<F>> for Standard
where
    Self: Distribution<Angle<F>>,
{
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> AngleUnbounded<F> {
        rng.gen::<Angle<F>>().to_unbounded()
    }
}

macro_rules! impl_unit {
    (
        $($unit:ident),+
    ) => {
        $(
            impl<T> Distribution<$unit<T>> for Standard
            where
                Self: Distribution<T>,
            {
                #[inline]
                fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $unit<T> {
                    $unit(rng.gen())
                }
            }
        )+
    };
}

impl_unit!(Radians, Degrees, Turns, Gradians);

//-------------------------------------------------------------------
// Range
//-------------------------------------------------------------------

/// The back-end implementing [`UniformSampler`] for [`Angle`].
///
/// Unless you are implementing [`UniformSampler`] for your own type, this type
/// should not be used directly, use [`Uniform`][rand::distributions::uniform::Uniform] instead.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformAngle<F>(UniformFloat<F>);

impl<F: Float> SampleUniform for Angle<F>
where
    UniformFloat<F>: UniformSampler<X = F>,
    F: SampleBorrow<F>,
{
    type Sampler = UniformAngle<F>;
}

impl<F: Float> UniformSampler for UniformAngle<F>
where
    UniformFloat<F>: UniformSampler<X = F>,
    F: SampleBorrow<F>,
{
    type X = Angle<F>;

    #[inline]
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow().to_radians();
        let mut high = high.borrow().to_radians();
        if low > high {
            high += F::TAU;
        }
        Self(UniformFloat::new(low, high))
    }

    #[inline]
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow().to_radians();
        let mut high = high.borrow().to_radians();
        if low > high {
            high += F::TAU;
        }
        Self(UniformFloat::new_inclusive(low, high))
    }

    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        let x = self.0.sample(rng);
        Angle::from_radians(x)
    }
}

impl<F: Float> SampleRange<Angle<F>> for Range<Angle<F>>
where
    UniformFloat<F>: UniformSampler<X = F>,
    F: SampleBorrow<F>,
{
    #[inline]
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Angle<F> {
        UniformAngle::sample_single(self.start, self.end, rng)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl<F: Float> SampleRange<Angle<F>> for RangeInclusive<Angle<F>>
where
    UniformFloat<F>: UniformSampler<X = F>,
    F: SampleBorrow<F>,
{
    #[inline]
    fn sample_single<R: rand::RngCore + ?Sized>(self, rng: &mut R) -> Angle<F> {
        UniformAngle::sample_single_inclusive(self.start(), self.end(), rng)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        false
    }
}

//-------------------------------------------------------------------

/// The back-end implementing [`UniformSampler`] for [`AngleUnbounded`].
///
/// Unless you are implementing [`UniformSampler`] for your own type, this type
/// should not be used directly, use [`Uniform`][rand::distributions::uniform::Uniform] instead.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniformAngleUnbounded<F>(UniformFloat<F>);

impl<F: Copy> SampleUniform for AngleUnbounded<F>
where
    UniformFloat<F>: UniformSampler<X = F>,
    F: SampleBorrow<F>,
{
    type Sampler = UniformAngleUnbounded<F>;
}

impl<F: Copy> UniformSampler for UniformAngleUnbounded<F>
where
    UniformFloat<F>: UniformSampler<X = F>,
    F: SampleBorrow<F>,
{
    type X = AngleUnbounded<F>;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow().to_radians();
        let high = high.borrow().to_radians();
        Self(UniformFloat::new(low, high))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = low.borrow().to_radians();
        let high = high.borrow().to_radians();
        Self(UniformFloat::new_inclusive(low, high))
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        let x = self.0.sample(rng);
        AngleUnbounded::from_radians(x)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{units::*, Angle32, Angle64, AngleUnbounded32, AngleUnbounded64};

    #[test]
    fn check_rand_impl() {
        let mut rng = rand::thread_rng();

        // Check if the code compile
        macro_rules! check {
            (
                $($angle:ident),+
            ) => {
                $(
                    let _: $angle = rand::random();
                    let _: Radians<$angle> = rand::random();
                    let _: Degrees<$angle> = rand::random();
                    let _: Turns<$angle> = rand::random();
                    let _: Gradians<$angle> = rand::random();

                    let _: $angle = rng.gen_range($angle::ZERO..$angle::RAD_PI);
                    let _: $angle = rng.gen_range($angle::ZERO..=$angle::RAD_PI);
                )+
            };
        }

        check!(Angle32, Angle64, AngleUnbounded32, AngleUnbounded64);
    }
}
