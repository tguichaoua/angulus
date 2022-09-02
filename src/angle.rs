use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{MainAngle, Num};

/// Represents a geometrical angle.
///
/// The parameter `N` is the numerical type that store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Angle<N> {
    pub(crate) radians: N,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<N: Num> Angle<N> {
    /// The angle of value zero.
    pub const ZERO: Self = Angle::from_radians(N::ZERO);

    /// The full circle angle.
    pub const FULL: Self = Angle::from_radians(N::TAU);
    /// The half of a circle angle.
    pub const HALF: Self = Angle::from_radians(N::PI);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = Angle::from_radians(N::FRAC_PI_2);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = Angle::from_radians(N::FRAC_PI_3);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = Angle::from_radians(N::FRAC_PI_4);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = Angle::from_radians(N::FRAC_PI_6);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = Angle::from_radians(N::FRAC_PI_8);
}

//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<N: Debug> Debug for Angle<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Angle").field(&self.radians).finish()
    }
}

impl<N: Num> Default for Angle<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Ctor
//-------------------------------------------------------------------

impl<N> Angle<N> {
    /// Creates a new angle from a value in radians.
    #[inline]
    pub const fn from_radians(radians: N) -> Self {
        Self { radians }
    }
}

impl<N: Num> Angle<N> {
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

impl<N: Copy> Angle<N> {
    /// The value of the angle in radians.
    #[inline]
    pub const fn to_radians(self) -> N {
        self.radians
    }
}

impl<N: Num> Angle<N> {
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

impl<N: Num> Angle<N> {
    /// Compute the main value of this angle.
    #[inline]
    pub fn to_main_angle(self) -> MainAngle<N> {
        MainAngle::from_radians(self.radians)
    }
}

impl<N> From<MainAngle<N>> for Angle<N> {
    #[inline]
    fn from(main_angle: MainAngle<N>) -> Self {
        Self::from_radians(main_angle.radians)
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<N: Num> Angle<N> {
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

impl<N: Num> Add for Angle<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

impl<N: Num> Sub for Angle<N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

impl<N: Num> Mul<N> for Angle<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

impl<N: Num> Div<N> for Angle<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

impl<N: Num> Neg for Angle<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_radians(self.radians.neg())
    }
}
