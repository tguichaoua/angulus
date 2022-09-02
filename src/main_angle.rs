use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Angle, Num};

/// Represents a geometrical angle whose value is the main angle value
/// (i.e. in the range `[-π; π]`).
///
/// The parameter `N` is the numerical type that store the value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MainAngle<N> {
    pub(crate) radians: N,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<N: Num> MainAngle<N> {
    /// The angle of value zero.
    pub const ZERO: Self = MainAngle::from_radians_unchecked(N::ZERO);
    /// The full circle angle.
    pub const FULL: Self = MainAngle::from_radians_unchecked(N::ZERO);
    /// The half of a circle angle.
    pub const HALF: Self = MainAngle::from_radians_unchecked(N::PI);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = MainAngle::from_radians_unchecked(N::FRAC_PI_2);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = MainAngle::from_radians_unchecked(N::FRAC_PI_3);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = MainAngle::from_radians_unchecked(N::FRAC_PI_4);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = MainAngle::from_radians_unchecked(N::FRAC_PI_6);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = MainAngle::from_radians_unchecked(N::FRAC_PI_8);
}

//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<N: Debug> Debug for MainAngle<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MainAngle").field(&self.radians).finish()
    }
}

impl<N: Num> Default for MainAngle<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Ctor
//-------------------------------------------------------------------

impl<N> MainAngle<N> {
    #[inline]
    pub const fn from_radians_unchecked(radians: N) -> Self {
        Self { radians }
    }
}

impl<N: Num> MainAngle<N> {
    /// Creates a new angle from a value in radians ensuring the value
    /// is in the main range.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let a = MainAngle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let b = MainAngle::from_radians(-3.0 * std::f32::consts::FRAC_PI_2);
    ///
    /// let abs_difference = (a.to_radians() - b.to_radians()).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn from_radians(radians: N) -> Self {
        let radians = radians % N::TAU;
        let radians = if radians > N::PI {
            radians - N::TAU
        } else if radians < -N::PI {
            radians + N::TAU
        } else {
            radians
        };
        Self::from_radians_unchecked(radians)
    }

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

impl<N: Copy> MainAngle<N> {
    /// The value of the angle in radians.
    #[inline]
    pub const fn to_radians(self) -> N {
        self.radians
    }
}

impl<N: Num> MainAngle<N> {
    /// The value of the angle in degrees.
    #[inline]
    pub const fn to_degrees(self) -> N {
        todo!()
    }

    /// The value of the angle in turns.
    #[inline]
    pub const fn to_turns(self) -> N {
        todo!()
    }
}

//-------------------------------------------------------------------
// Angle convertion
//-------------------------------------------------------------------

impl<N: Copy> MainAngle<N> {
    pub const fn to_angle(self) -> Angle<N> {
        Angle::from_radians(self.radians)
    }
}

impl<N: Num> From<Angle<N>> for MainAngle<N> {
    #[inline]
    fn from(angle: Angle<N>) -> Self {
        Self::from_radians(angle.to_radians())
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<N: Num> MainAngle<N> {
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

impl<N: Num> Add for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Angle::from_radians(self.radians + rhs.radians)
    }
}

impl<N: Num> Add<Angle<N>> for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn add(self, rhs: Angle<N>) -> Self::Output {
        Angle::from_radians(self.radians + rhs.radians)
    }
}

impl<N: Num> Sub for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Angle::from_radians(self.radians - rhs.radians)
    }
}

impl<N: Num> Sub<Angle<N>> for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn sub(self, rhs: Angle<N>) -> Self::Output {
        Angle::from_radians(self.radians - rhs.radians)
    }
}

impl<N: Num> Mul<N> for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Angle::from_radians(self.radians * rhs)
    }
}

impl<N: Num> Div<N> for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        Angle::from_radians(self.radians / rhs)
    }
}

impl<N: Num> Neg for MainAngle<N> {
    type Output = Angle<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Angle::from_radians(self.radians.neg())
    }
}
