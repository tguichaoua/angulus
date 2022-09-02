use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utility::AngleConvertion;
use crate::{Num, UnboundedAngle};

/// Represents a geometrical angle whose value is the main angle value
/// (i.e. in the range `[-π; π]`).
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
    pub const ZERO: Self = Angle::from_radians_unchecked(N::ZERO);
    /// The full circle angle.
    pub const FULL: Self = Angle::from_radians_unchecked(N::ZERO);
    /// The half of a circle angle.
    pub const HALF: Self = Angle::from_radians_unchecked(N::PI);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = Angle::from_radians_unchecked(N::FRAC_PI_2);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = Angle::from_radians_unchecked(N::FRAC_PI_3);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = Angle::from_radians_unchecked(N::FRAC_PI_4);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = Angle::from_radians_unchecked(N::FRAC_PI_6);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = Angle::from_radians_unchecked(N::FRAC_PI_8);
}

//-------------------------------------------------------------------
// Standard traits
//-------------------------------------------------------------------

impl<N: Debug> Debug for Angle<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MainAngle").field(&self.radians).finish()
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
    #[inline]
    pub const fn from_radians_unchecked(radians: N) -> Self {
        Self { radians }
    }
}

impl<N: Num> Angle<N> {
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
// Angle convertion
//-------------------------------------------------------------------

impl<N: Copy> Angle<N> {
    pub const fn to_angle(self) -> UnboundedAngle<N> {
        UnboundedAngle::from_radians(self.radians)
    }
}

impl<N: Num> From<UnboundedAngle<N>> for Angle<N> {
    #[inline]
    fn from(angle: UnboundedAngle<N>) -> Self {
        Self::from_radians(angle.to_radians())
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
    type Output = UnboundedAngle<N>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        UnboundedAngle::from_radians(self.radians + rhs.radians)
    }
}

impl<N: Num> Add<UnboundedAngle<N>> for Angle<N> {
    type Output = UnboundedAngle<N>;

    #[inline]
    fn add(self, rhs: UnboundedAngle<N>) -> Self::Output {
        UnboundedAngle::from_radians(self.radians + rhs.radians)
    }
}

impl<N: Num> Sub for Angle<N> {
    type Output = UnboundedAngle<N>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        UnboundedAngle::from_radians(self.radians - rhs.radians)
    }
}

impl<N: Num> Sub<UnboundedAngle<N>> for Angle<N> {
    type Output = UnboundedAngle<N>;

    #[inline]
    fn sub(self, rhs: UnboundedAngle<N>) -> Self::Output {
        UnboundedAngle::from_radians(self.radians - rhs.radians)
    }
}

impl<N: Num> Mul<N> for Angle<N> {
    type Output = UnboundedAngle<N>;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        UnboundedAngle::from_radians(self.radians * rhs)
    }
}

impl<N: Num> Div<N> for Angle<N> {
    type Output = UnboundedAngle<N>;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        UnboundedAngle::from_radians(self.radians / rhs)
    }
}

impl<N: Num> Neg for Angle<N> {
    type Output = UnboundedAngle<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        UnboundedAngle::from_radians(self.radians.neg())
    }
}

//-------------------------------------------------------------------
// Misc.
//-------------------------------------------------------------------

impl<N: Num> AngleConvertion for Angle<N> {
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
