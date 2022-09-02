use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utility::AngleConvertion;
use crate::{Num, UnboundedAngle};

/// Represents the canonical value of an angle.
///
/// The value is stored in the range `[-π; π]`.
///
/// The parameter `F` is the floating-point type used to store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Angle<F> {
    pub(crate) radians: F,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<F: Num> Angle<F> {
    /// The angle of value zero.
    pub const ZERO: Self = Angle::from_radians_unchecked(F::ZERO);
    /// The full circle angle.
    pub const FULL: Self = Angle::from_radians_unchecked(F::ZERO);
    /// The half of a circle angle.
    pub const HALF: Self = Angle::from_radians_unchecked(F::PI);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = Angle::from_radians_unchecked(F::FRAC_PI_2);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = Angle::from_radians_unchecked(F::FRAC_PI_3);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = Angle::from_radians_unchecked(F::FRAC_PI_4);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = Angle::from_radians_unchecked(F::FRAC_PI_6);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = Angle::from_radians_unchecked(F::FRAC_PI_8);
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
    pub const fn to_unbounded(self) -> UnboundedAngle<F> {
        UnboundedAngle::from_radians(self.radians)
    }
}

impl<F: Num> From<UnboundedAngle<F>> for Angle<F> {
    #[inline]
    fn from(angle: UnboundedAngle<F>) -> Self {
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
        Self::from_radians(self.radians.neg())
    }
}

//-------------------------------------------------------------------
// Misc.
//-------------------------------------------------------------------

impl<F: Num> AngleConvertion for Angle<F> {
    type N = F;

    #[inline]
    fn from_radians(radians: F) -> Self {
        Self::from_radians(radians)
    }

    #[inline]
    fn from_degrees(degrees: F) -> Self {
        Self::from_degrees(degrees)
    }

    #[inline]
    fn from_turns(turns: F) -> Self {
        Self::from_turns(turns)
    }

    #[inline]
    fn to_radians(&self) -> F {
        (*self).to_radians()
    }

    #[inline]
    fn to_degrees(&self) -> F {
        (*self).to_degrees()
    }

    #[inline]
    fn to_turns(&self) -> F {
        (*self).to_turns()
    }
}
