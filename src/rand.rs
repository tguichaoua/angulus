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
            high = high + F::TAU;
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
            high = high + F::TAU;
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
