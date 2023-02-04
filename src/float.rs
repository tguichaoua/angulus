//! Defines which types can be used as a floating-point value for the angle.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

mod private {
    pub trait Sealed {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Marker trait for floating-point types that can be used as numerical type
/// for [`Angle`][crate::Angle] and [`AngleUnbounded`][crate::AngleUnbounded].
///
/// This trait is sealed and is implemented for [`f32`] and [`f64`].
pub trait Float:
    private::Sealed
    + Copy
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + Neg<Output = Self>
    + PartialOrd
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
    /// Required by [`Angle::EPSILON`][crate::Angle::EPSILON] because const trait multiplication is unstable.
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

    /// Convertion factor from gradians to radians.
    const GRAD_TO_RAD: Self;
    /// Convertion factor from radians to gradians.
    const RAD_TO_GRAD: Self;

    /// Computes the sine (in radians).
    fn sin(self) -> Self;
    /// Computes the cosine (in radians).
    fn cos(self) -> Self;
    /// Computes the tangent (in radians).
    fn tan(self) -> Self;
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self, Self);
}

impl Float for f32 {
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

    const DEG_TO_RAD: Self = std::f32::consts::PI / 180.0;
    const RAD_TO_DEG: Self = 180.0 / std::f32::consts::PI;

    const TURNS_TO_RAD: Self = std::f32::consts::TAU;
    const RAD_TO_TURNS: Self = 1.0 / std::f32::consts::TAU;

    const GRAD_TO_RAD: Self = std::f32::consts::PI / 200.0;
    const RAD_TO_GRAD: Self = 200.0 / std::f32::consts::PI;

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

impl Float for f64 {
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

    const DEG_TO_RAD: Self = std::f64::consts::PI / 180.0;
    const RAD_TO_DEG: Self = 180.0 / std::f64::consts::PI;

    const TURNS_TO_RAD: Self = std::f64::consts::TAU;
    const RAD_TO_TURNS: Self = 1.0 / std::f64::consts::TAU;

    const GRAD_TO_RAD: Self = std::f64::consts::PI / 200.0;
    const RAD_TO_GRAD: Self = 200.0 / std::f64::consts::PI;

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
