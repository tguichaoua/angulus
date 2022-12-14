use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Marker trait for floating-point types that can be used as numerical type
/// for [`Angle`][crate::Angle] and [`UnboundedAngle`][crate::UnboundedAngle].
pub trait Num:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
    + Sized
{
    /// The additive identity element of `Self` (aka `0`).
    const ZERO: Self;
    /// The multiplicative identity element of `Self` (aka `1`).
    const ONE: Self;

    /// [Machine epsilon] value for `Self`.
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    const EPSILON: Self;

    /// The double of `EPSILON`.
    ///
    /// Required by [`Angle::EPSILON`] because const trait multiplication is unstable.
    const DOUBLE_EPSILON: Self;

    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    const TAU: Self;
    /// Archimedes' constant (π)
    const PI: Self;
    /// π/2
    const FRAC_PI_2: Self;
    /// π/3
    const FRAC_PI_3: Self;
    /// π/4
    const FRAC_PI_4: Self;
    /// π/6
    const FRAC_PI_6: Self;
    /// π/8
    const FRAC_PI_8: Self;

    /// Convertion factor from degrees to radians.
    const DEG_TO_RAD: Self;
    /// Convertion factor from radians to degrees.
    const RAD_TO_DEG: Self;

    /// Convertion factor from turns to radians.
    const TURNS_TO_RAD: Self;
    /// Convertion factor from radians to turns.
    const RAD_TO_TURNS: Self;

    /// Computes the sine (in radians).
    fn sin(self) -> Self;
    /// Computes the cosine (in radians).
    fn cos(self) -> Self;
    /// Computes the tangent (in radians).
    fn tan(self) -> Self;
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self, Self);
}

impl Num for f32 {
    const ZERO: Self = 0.0f32;
    const ONE: Self = 1.0f32;
    const EPSILON: Self = f32::EPSILON;
    const DOUBLE_EPSILON: Self = 2.0 * Self::EPSILON;

    const TAU: Self = std::f32::consts::TAU;
    const PI: Self = std::f32::consts::PI;
    const FRAC_PI_2: Self = std::f32::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = std::f32::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = std::f32::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = std::f32::consts::FRAC_PI_6;
    const FRAC_PI_8: Self = std::f32::consts::FRAC_PI_8;

    const DEG_TO_RAD: Self = 0.017_453_292_f32;
    const RAD_TO_DEG: Self = 57.295_78_f32;

    const TURNS_TO_RAD: Self = std::f32::consts::TAU;
    const RAD_TO_TURNS: Self = 0.159_154_94_f32;

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }
    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }
    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
}

impl Num for f64 {
    const ZERO: Self = 0.0f64;
    const ONE: Self = 1.0f64;
    const EPSILON: Self = f64::EPSILON;
    const DOUBLE_EPSILON: Self = 2.0 * Self::EPSILON;

    const TAU: Self = std::f64::consts::TAU;
    const PI: Self = std::f64::consts::PI;
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    const FRAC_PI_8: Self = std::f64::consts::FRAC_PI_8;

    const DEG_TO_RAD: Self = 0.017_453_292_519_943_295_f64;
    const RAD_TO_DEG: Self = 57.295_779_513_082_32_f64;

    const TURNS_TO_RAD: Self = std::f64::consts::TAU;
    const RAD_TO_TURNS: Self = 0.159_154_943_092_f64;

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }
    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }
    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
}
