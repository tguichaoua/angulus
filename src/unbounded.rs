use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{utility::AngleConvertion, Angle, Num};

/// Represents an angle that may not be the canonical value.
///
/// The value of this angle is not bound into a range, unlike [`Angle`].
///
/// ```
/// # use angulus::*;
/// # use float_eq::assert_float_eq;
/// # fn main() {
/// let angle = (3.0_f32 * 90.0_f32.deg()).to_degrees();
/// let unboudned = (3.0_f32 * 90.0_f32.deg_unbounded()).to_degrees();
///
/// assert_float_eq!(angle, -90.0, ulps <= 1);
/// assert_float_eq!(unboudned, 270.0, ulps <= 1);
/// # }
/// ```
///
/// The parameter `F` is the floating-point type used to store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnboundedAngle<F> {
    pub(crate) radians: F,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<F: Num> UnboundedAngle<F> {
    /// The angle of value zero.
    pub const ZERO: Self = UnboundedAngle::from_radians(F::ZERO);

    /// The full circle angle.
    pub const FULL: Self = UnboundedAngle::from_radians(F::TAU);
    /// The half of a circle angle.
    pub const HALF: Self = UnboundedAngle::from_radians(F::PI);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = UnboundedAngle::from_radians(F::FRAC_PI_2);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = UnboundedAngle::from_radians(F::FRAC_PI_3);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = UnboundedAngle::from_radians(F::FRAC_PI_4);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = UnboundedAngle::from_radians(F::FRAC_PI_6);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = UnboundedAngle::from_radians(F::FRAC_PI_8);
}

//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<F: Debug> Debug for UnboundedAngle<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnboundedAngle")
            .field(&self.radians)
            .finish()
    }
}

impl<F: Num> Default for UnboundedAngle<F> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Ctor
//-------------------------------------------------------------------

impl<F> UnboundedAngle<F> {
    /// Creates a new unbounded angle from a value in radians.
    #[inline]
    pub const fn from_radians(radians: F) -> Self {
        Self { radians }
    }
}

impl<F: Num> UnboundedAngle<F> {
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

impl<F: Copy> UnboundedAngle<F> {
    /// The value of the unbounded angle in radians.
    #[inline]
    pub const fn to_radians(self) -> F {
        self.radians
    }
}

impl<F: Num> UnboundedAngle<F> {
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

impl<F: Num> UnboundedAngle<F> {
    /// Converts this angle into a bounded angle.
    #[inline]
    pub fn to_bounded(self) -> Angle<F> {
        Angle::from_radians(self.radians)
    }
}

impl<F> From<Angle<F>> for UnboundedAngle<F> {
    #[inline]
    fn from(main_angle: Angle<F>) -> Self {
        Self::from_radians(main_angle.radians)
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<F: Num> UnboundedAngle<F> {
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

impl<F: Num> Add for UnboundedAngle<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

impl<F: Num> Sub for UnboundedAngle<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

impl<F: Num> Mul<F> for UnboundedAngle<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

impl Mul<UnboundedAngle<f32>> for f32 {
    type Output = UnboundedAngle<f32>;

    #[inline]
    fn mul(self, rhs: UnboundedAngle<f32>) -> Self::Output {
        rhs * self
    }
}

impl Mul<UnboundedAngle<f64>> for f64 {
    type Output = UnboundedAngle<f64>;

    #[inline]
    fn mul(self, rhs: UnboundedAngle<f64>) -> Self::Output {
        rhs * self
    }
}

impl<F: Num> Div<F> for UnboundedAngle<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

impl<F: Num> Neg for UnboundedAngle<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_radians(self.radians.neg())
    }
}

//-------------------------------------------------------------------
// Misc.
//-------------------------------------------------------------------

impl<F: Num> AngleConvertion for UnboundedAngle<F> {
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
