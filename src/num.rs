use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Marker for type that can be used as numerical type for [`Angle`][crate::Angle]
/// and [`MainAngle`][crate::MainAngle].
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
    /// The additive identity element of Self (aka `0`).
    const ZERO: Self;
    /// The multiplicative identity element of `Self` (aka `1`).
    const ONE: Self;

    /// Computes the sine (in radians).
    fn sin(self) -> Self;
    /// Computes the cosine (in radians).
    fn cos(self) -> Self;
    /// Computes the tangent (in radians).
    fn tan(self) -> Self;
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    fn sin_cos(self) -> (Self, Self);

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()`.
    fn rem_euclid(self, rhs: Self) -> Self;
}

impl Num for f32 {
    const ZERO: Self = 0.0f32;
    const ONE: Self = 1.0f32;

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
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid(rhs)
    }
}

impl Num for f64 {
    const ZERO: Self = 0.0f64;
    const ONE: Self = 1.0f64;

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
    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        self.rem_euclid(rhs)
    }
}
