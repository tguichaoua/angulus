use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{AngleUnbounded, Num};

/// Represents the canonical value of an angle.
///
/// The value is stored in the range `[-π; π]`.
///
/// The parameter `F` is the floating-point type used to store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Angle<F> {
    radians: F,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<F: Num> Angle<F> {
    /// The angle of value zero.
    pub const ZERO: Self = Angle::from_radians_unchecked(F::ZERO);

    /// [Machine epsilon] value for [`Angle`].
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: Self = Angle::from_radians_unchecked(F::DOUBLE_EPSILON);
}

impl<F: Num> Angle<F> {
    /// The angle of π radians.
    pub const RAD_PI: Self = Angle::from_radians_unchecked(F::PI);
    /// The angle of π/2 radians.
    pub const RAD_FRAC_PI_2: Self = Angle::from_radians_unchecked(F::FRAC_PI_2);
    /// The angle of π/3 radians.
    pub const RAD_FRAC_PI_3: Self = Angle::from_radians_unchecked(F::FRAC_PI_3);
    /// The angle of π/4 radians.
    pub const RAD_FRAC_PI_4: Self = Angle::from_radians_unchecked(F::FRAC_PI_4);
    /// The angle of π/6 radians.
    pub const RAD_FRAC_PI_6: Self = Angle::from_radians_unchecked(F::FRAC_PI_6);
    /// The angle of π/8 radians.
    pub const RAD_FRAC_PI_8: Self = Angle::from_radians_unchecked(F::FRAC_PI_8);
}

impl<F: Num> Angle<F> {
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

impl<F: Num> Angle<F> {
    /// The angle of a half of a circle (1/2 turns).
    pub const HALF: Self = Self::RAD_PI;
    /// The angle of a quarter of a circle (1/4 turns).
    pub const QUARTER: Self = Self::RAD_FRAC_PI_2;
    /// The angle of a sixth of a circle (1/6 turns).
    pub const SIXTH: Self = Self::RAD_FRAC_PI_3;
    ///  The angle of a eighth of a circle (1/6 turns).
    pub const EIGHTH: Self = Self::RAD_FRAC_PI_4;
    ///  The angle of a twelfth of a circle (1/12 turns).
    pub const TWELFTH: Self = Self::RAD_FRAC_PI_6;
    ///  The angle of a sixteenth of a circle (1/16 turns).
    pub const SIXTEENTH: Self = Self::RAD_FRAC_PI_8;
}

//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<F: Debug> Debug for Angle<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Angle").field(&self.radians).finish()
    }
}

impl<F: Num> Default for Angle<F> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Ctor
//-------------------------------------------------------------------

impl<F> Angle<F> {
    /// Creates a new angle from a value in radians assuming it is already in
    /// the the range `[-π; π]`.
    #[inline]
    pub const fn from_radians_unchecked(radians: F) -> Self {
        Self { radians }
    }
}

impl<F: Num> Angle<F> {
    /// Creates a new angle from a value in radians.
    #[inline]
    pub fn from_radians(radians: F) -> Self {
        let radians = radians % F::TAU;
        let radians = if radians > F::PI {
            radians - F::TAU
        } else if radians < -F::PI {
            radians + F::TAU
        } else {
            radians
        };
        Self::from_radians_unchecked(radians)
    }

    /// Creates a new angle from a value in degrees.
    #[inline]
    pub fn from_degrees(degrees: F) -> Self {
        Self::from_radians(degrees * F::DEG_TO_RAD)
    }

    /// Creates a new angle from a value in turns.
    #[inline]
    pub fn from_turns(turns: F) -> Self {
        Self::from_radians(turns * F::TURNS_TO_RAD)
    }
}

//-------------------------------------------------------------------
// Getters
//-------------------------------------------------------------------

impl<F: Copy> Angle<F> {
    /// The value of the angle in radians.
    #[inline]
    pub const fn to_radians(self) -> F {
        self.radians
    }
}

impl<F: Num> Angle<F> {
    /// The value of the angle in degrees.
    #[inline]
    pub fn to_degrees(self) -> F {
        self.radians * F::RAD_TO_DEG
    }

    /// The value of the angle in turns.
    #[inline]
    pub fn to_turns(self) -> F {
        self.radians * F::RAD_TO_TURNS
    }
}

//-------------------------------------------------------------------
// Angle convertion
//-------------------------------------------------------------------

impl<F: Copy> Angle<F> {
    /// Converts this angle into an unbounded angle.
    pub const fn to_unbounded(self) -> AngleUnbounded<F> {
        AngleUnbounded::from_radians(self.radians)
    }
}

impl<F: Num> From<AngleUnbounded<F>> for Angle<F> {
    #[inline]
    fn from(angle: AngleUnbounded<F>) -> Self {
        Self::from_radians(angle.to_radians())
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<F: Num> Angle<F> {
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

impl<F: Num> Add for Angle<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

impl<F: Num> Sub for Angle<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

impl<F: Num> Mul<F> for Angle<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

impl Mul<Angle<f32>> for f32 {
    type Output = Angle<f32>;

    #[inline]
    fn mul(self, rhs: Angle<f32>) -> Self::Output {
        rhs * self
    }
}

impl Mul<Angle<f64>> for f64 {
    type Output = Angle<f64>;

    #[inline]
    fn mul(self, rhs: Angle<f64>) -> Self::Output {
        rhs * self
    }
}

impl<F: Num> Div<F> for Angle<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

impl<F: Num> Neg for Angle<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_radians(-self.radians)
    }
}
