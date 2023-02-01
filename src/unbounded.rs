use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{Angle, Num};

/// Represents an angle that may not be the canonical value.
///
/// The value of this angle is not bound into a range, unlike [`Angle`].
///
/// ```
/// # use angulus::*;
/// # use float_eq::assert_float_eq;
/// # fn main() {
/// let angle = (3.0_f32 * 90.0_f32.deg()).to_degrees();
/// let unbounded = (3.0_f32 * 90.0_f32.deg_unbounded()).to_degrees();
///
/// assert_float_eq!(angle, -90.0, ulps <= 1);
/// assert_float_eq!(unbounded, 270.0, ulps <= 1);
/// # }
/// ```
///
/// The parameter `F` is the floating-point type used to store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AngleUnbounded<F> {
    radians: F,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<F: Num> AngleUnbounded<F> {
    /// The angle of value zero.
    pub const ZERO: Self = AngleUnbounded::from_radians(F::ZERO);

    /// [Machine epsilon] value for [`AngleUnbounded`].
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: Self = AngleUnbounded::from_radians(F::DOUBLE_EPSILON);
}

impl<F: Num> AngleUnbounded<F> {
    /// The angle of π radians.
    pub const RAD_PI: Self = AngleUnbounded::from_radians(F::PI);
    /// The angle of π/2 radians.
    pub const RAD_FRAC_PI_2: Self = AngleUnbounded::from_radians(F::FRAC_PI_2);
    /// The angle of π/3 radians.
    pub const RAD_FRAC_PI_3: Self = AngleUnbounded::from_radians(F::FRAC_PI_3);
    /// The angle of π/4 radians.
    pub const RAD_FRAC_PI_4: Self = AngleUnbounded::from_radians(F::FRAC_PI_4);
    /// The angle of π/6 radians.
    pub const RAD_FRAC_PI_6: Self = AngleUnbounded::from_radians(F::FRAC_PI_6);
    /// The angle of π/8 radians.
    pub const RAD_FRAC_PI_8: Self = AngleUnbounded::from_radians(F::FRAC_PI_8);
}

impl<F: Num> AngleUnbounded<F> {
    /// The angle of 180°.
    pub const DEG_180: Self = Self::RAD_PI;
    /// The angle of 90°.
    pub const DEG_90: Self = Self::RAD_FRAC_PI_2;
    /// The angle of 60°.
    pub const DEG_60: Self = Self::RAD_FRAC_PI_3;
    /// The angle of 45°.
    pub const DEG_45: Self = Self::RAD_FRAC_PI_4;
    /// The angle of 30°.
    pub const DEG_30: Self = Self::RAD_FRAC_PI_6;
    /// The angle of 22.5°.
    pub const DEG_22_5: Self = Self::RAD_FRAC_PI_8;
}

impl<F: Num> AngleUnbounded<F> {
    /// The angle of a half of a circle (1/2 turns).
    pub const HALF: Self = Self::RAD_PI;
    /// The angle of a quarter of a circle (1/4 turns).
    pub const QUARTER: Self = Self::RAD_FRAC_PI_2;
    /// The angle of a sixth of a circle (1/6 turns).
    pub const SIXTH: Self = Self::RAD_FRAC_PI_3;
    ///  The angle of a eighth of a circle (1/8 turns).
    pub const EIGHTH: Self = Self::RAD_FRAC_PI_4;
    ///  The angle of a twelfth of a circle (1/12 turns).
    pub const TWELFTH: Self = Self::RAD_FRAC_PI_6;
    ///  The angle of a sixteenth of a circle (1/16 turns).
    pub const SIXTEENTH: Self = Self::RAD_FRAC_PI_8;
}
//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<F: Debug> Debug for AngleUnbounded<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AngleUnbounded")
            .field(&self.radians)
            .finish()
    }
}

impl<F: Num> Default for AngleUnbounded<F> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Ctor
//-------------------------------------------------------------------

impl<F> AngleUnbounded<F> {
    /// Creates a new unbounded angle from a value in radians.
    #[inline]
    pub const fn from_radians(radians: F) -> Self {
        Self { radians }
    }
}

impl<F: Num> AngleUnbounded<F> {
    /// Creates a new unbounded angle from a value in degrees.
    #[inline]
    pub fn from_degrees(degrees: F) -> Self {
        Self::from_radians(degrees * F::DEG_TO_RAD)
    }

    /// Creates a new unbounded angle from a value in turns.
    #[inline]
    pub fn from_turns(turns: F) -> Self {
        Self::from_radians(turns * F::TURNS_TO_RAD)
    }
}

//-------------------------------------------------------------------
// Getters
//-------------------------------------------------------------------

impl<F: Copy> AngleUnbounded<F> {
    /// The value of the unbounded angle in radians.
    #[inline]
    pub const fn to_radians(self) -> F {
        self.radians
    }
}

impl<F: Num> AngleUnbounded<F> {
    /// The value of the unbounded angle in degrees.
    #[inline]
    pub fn to_degrees(self) -> F {
        self.radians * F::RAD_TO_DEG
    }

    /// The value of the unbounded angle in turns.
    #[inline]
    pub fn to_turns(self) -> F {
        self.radians * F::RAD_TO_TURNS
    }
}

//-------------------------------------------------------------------
// MainAngle convertion
//-------------------------------------------------------------------

impl<F: Num> AngleUnbounded<F> {
    /// Converts this angle into a bounded angle.
    #[inline]
    pub fn to_bounded(self) -> Angle<F> {
        Angle::from_radians(self.radians)
    }
}

impl<F: Copy> From<Angle<F>> for AngleUnbounded<F> {
    #[inline]
    fn from(angle: Angle<F>) -> Self {
        Self::from_radians(angle.to_radians())
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<F: Num> AngleUnbounded<F> {
    /// Computes the sine.
    #[inline]
    pub fn sin(self) -> F {
        self.radians.sin()
    }
    /// Computes the cosine.
    #[inline]
    pub fn cos(self) -> F {
        self.radians.cos()
    }
    /// Computes the tangent.
    #[inline]
    pub fn tan(self) -> F {
        self.radians.tan()
    }
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    #[inline]
    pub fn sin_cos(self) -> (F, F) {
        self.radians.sin_cos()
    }
}

//-------------------------------------------------------------------
// Ops
//-------------------------------------------------------------------

impl<F: Num> Add for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

impl<F: Num> Sub for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

impl<F: Num> Mul<F> for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

impl Mul<AngleUnbounded<f32>> for f32 {
    type Output = AngleUnbounded<f32>;

    #[inline]
    fn mul(self, rhs: AngleUnbounded<f32>) -> Self::Output {
        rhs * self
    }
}

impl Mul<AngleUnbounded<f64>> for f64 {
    type Output = AngleUnbounded<f64>;

    #[inline]
    fn mul(self, rhs: AngleUnbounded<f64>) -> Self::Output {
        rhs * self
    }
}

impl<F: Num> Div<F> for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

impl<F: Num> Neg for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_radians(-self.radians)
    }
}
