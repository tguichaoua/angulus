use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{utility::AngleConvertion, Angle, Num};

/// Represents a geometrical angle.
///
/// The parameter `N` is the numerical type that store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UnboundedAngle<N> {
    pub(crate) radians: N,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<N: Num> UnboundedAngle<N> {
    /// The angle of value zero.
    pub const ZERO: Self = UnboundedAngle::from_radians(N::ZERO);

    /// The full circle angle.
    pub const FULL: Self = UnboundedAngle::from_radians(N::TAU);
    /// The half of a circle angle.
    pub const HALF: Self = UnboundedAngle::from_radians(N::PI);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = UnboundedAngle::from_radians(N::FRAC_PI_2);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = UnboundedAngle::from_radians(N::FRAC_PI_3);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = UnboundedAngle::from_radians(N::FRAC_PI_4);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = UnboundedAngle::from_radians(N::FRAC_PI_6);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = UnboundedAngle::from_radians(N::FRAC_PI_8);
}

//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<N: Debug> Debug for UnboundedAngle<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Angle").field(&self.radians).finish()
    }
}

impl<N: Num> Default for UnboundedAngle<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Ctor
//-------------------------------------------------------------------

impl<N> UnboundedAngle<N> {
    /// Creates a new angle from a value in radians.
    #[inline]
    pub const fn from_radians(radians: N) -> Self {
        Self { radians }
    }
}

impl<N: Num> UnboundedAngle<N> {
    /// Creates a new angle from a value in degrees.
    #[inline]
    pub fn from_degrees(degrees: N) -> Self {
        Self::from_radians(degrees * N::DEG_TO_RAD)
    }

    /// Creates a new angle from a value in turns.
    #[inline]
    pub fn from_turns(turns: N) -> Self {
        Self::from_radians(turns * N::TURNS_TO_RAD)
    }
}

//-------------------------------------------------------------------
// Getters
//-------------------------------------------------------------------

impl<N: Copy> UnboundedAngle<N> {
    /// The value of the angle in radians.
    #[inline]
    pub const fn to_radians(self) -> N {
        self.radians
    }
}

impl<N: Num> UnboundedAngle<N> {
    /// The value of the angle in degrees.
    #[inline]
    pub fn to_degrees(self) -> N {
        self.radians * N::RAD_TO_DEG
    }

    /// The value of the angle in turns.
    #[inline]
    pub fn to_turns(self) -> N {
        self.radians * N::RAD_TO_TURNS
    }
}

//-------------------------------------------------------------------
// MainAngle convertion
//-------------------------------------------------------------------

impl<N: Num> UnboundedAngle<N> {
    /// Compute the main value of this angle.
    #[inline]
    pub fn to_main_angle(self) -> Angle<N> {
        Angle::from_radians(self.radians)
    }
}

impl<N> From<Angle<N>> for UnboundedAngle<N> {
    #[inline]
    fn from(main_angle: Angle<N>) -> Self {
        Self::from_radians(main_angle.radians)
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<N: Num> UnboundedAngle<N> {
    /// Computes the sine.
    #[inline]
    pub fn sin(self) -> N {
        self.radians.sin()
    }
    /// Computes the cosine.
    #[inline]
    pub fn cos(self) -> N {
        self.radians.cos()
    }
    /// Computes the tangent.
    #[inline]
    pub fn tan(self) -> N {
        self.radians.tan()
    }
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    #[inline]
    pub fn sin_cos(self) -> (N, N) {
        self.radians.sin_cos()
    }
}

//-------------------------------------------------------------------
// Ops
//-------------------------------------------------------------------

impl<N: Num> Add for UnboundedAngle<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

impl<N: Num> Sub for UnboundedAngle<N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

impl<N: Num> Mul<N> for UnboundedAngle<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

impl<N: Num> Div<N> for UnboundedAngle<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

impl<N: Num> Neg for UnboundedAngle<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_radians(self.radians.neg())
    }
}

//-------------------------------------------------------------------
// Misc.
//-------------------------------------------------------------------

impl<N: Num> AngleConvertion for UnboundedAngle<N> {
    type N = N;

    #[inline]
    fn from_radians(radians: N) -> Self {
        Self::from_radians(radians)
    }

    #[inline]
    fn from_degrees(degrees: N) -> Self {
        Self::from_degrees(degrees)
    }

    #[inline]
    fn from_turns(turns: N) -> Self {
        Self::from_turns(turns)
    }

    #[inline]
    fn to_radians(&self) -> N {
        (*self).to_radians()
    }

    #[inline]
    fn to_degrees(&self) -> N {
        (*self).to_degrees()
    }

    #[inline]
    fn to_turns(&self) -> N {
        (*self).to_turns()
    }
}
