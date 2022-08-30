use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Angle, Degrees, Num, Radians, ToUnit, Turns, Unit, UnitName};

/// Represents a geometrical angle whose value is the main angle value.
///
/// I.e the value is in the range :
/// - `[0; 2π)` for radians
/// - `[0; 360)` for degrees
/// - `[0; 1)` for turns
///
/// The parameter `N` is the numerical type that store the value.
///
/// The parameter `U` is the unit of the angle.
/// It can be one of the following :
/// - [`Radians`]
/// - [`Degrees`]
/// - [`Turns`]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MainAngle<N, U = Radians>(pub(crate) Angle<N, U>);

impl<N, U> MainAngle<N, U> {
    /// Wraps the given value, assuming it is already in the main range.
    #[inline]
    pub const fn new_unchecked(value: N) -> Self {
        Self(Angle::new(value))
    }

    /// Convert this [`MainAngle`] into a regular [`Angle`].
    #[inline]
    pub fn to_angle(self) -> Angle<N, U> {
        self.0
    }

    /// The numerical value of the angle.
    #[inline]
    pub fn to_value(self) -> N {
        self.0.to_value()
    }
}

impl<N: Num, U: Unit<N>> MainAngle<N, U> {
    /// Creates a new main angle ensuring the value is in the
    /// main range.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let a = MainAngle::<f32, Degrees>::new(90.0);
    /// let b = MainAngle::<f32, Degrees>::new(-270.0);
    ///
    /// let abs_difference = (a.to_value() - b.to_value()).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn new(value: N) -> Self {
        let value = value.rem_euclid(U::FULL);
        Self::new_unchecked(value)
    }
}

impl<N: Num, U: Unit<N>> From<Angle<N, U>> for MainAngle<N, U> {
    #[inline]
    fn from(angle: Angle<N, U>) -> Self {
        angle.main_angle()
    }
}

impl<N: Num, U> Default for MainAngle<N, U> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Format
//-------------------------------------------------------------------

impl<N: Debug, U: Unit<N>> Debug for MainAngle<N, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MainAngle")
            .field(&self.0.value)
            .field(&U::default())
            .finish()
    }
}

impl<N: Display, U: UnitName> Display for MainAngle<N, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<N: Num, U> MainAngle<N, U> {
    /// The angle of value zero.
    pub const ZERO: Self = Self::new_unchecked(N::ZERO);
    /// The main angle of the full circle angle.
    ///
    /// Note: the value is zero because the upper bound of the main angle
    /// range is excluded.
    pub const FULL: Self = Self::new_unchecked(N::ZERO);
}

impl<N: Num, U: Unit<N>> MainAngle<N, U> {
    /// The half of a circle angle.
    pub const HALF: Self = Self::new_unchecked(U::HALF);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = Self::new_unchecked(U::QUARTER);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = Self::new_unchecked(U::SIXTH);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = Self::new_unchecked(U::EIGHTH);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = Self::new_unchecked(U::TWELFTH);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = Self::new_unchecked(U::SIXTEENTH);
}

//-------------------------------------------------------------------
// from units
//-------------------------------------------------------------------

impl<N: Num> MainAngle<N, Radians>
where
    Radians: Unit<N>,
{
    /// Creates a new main angle with a value in radians ensuring the
    /// value is in the main range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use angulus::*;
    /// let x = MainAngle::from_radians(std::f32::consts::TAU + std::f32::consts::FRAC_PI_2);
    ///
    /// let abs_difference = (x.to_value() - std::f32::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn from_radians(radians: N) -> Self {
        Self::new(radians)
    }
}

impl<N: Num> MainAngle<N, Degrees>
where
    Degrees: Unit<N>,
{
    /// Creates a new main angle with a value in degrees ensuring the
    /// value is in the main range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use angulus::*;
    /// let x = MainAngle::from_degrees(450.0f32);
    ///
    /// let abs_difference = (x.to_value() - 90.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn from_degrees(degrees: N) -> Self {
        Self::new(degrees)
    }
}

impl<N: Num> MainAngle<N, Turns>
where
    Turns: Unit<N>,
{
    /// Creates a new main angle with a value in radians ensuring the
    /// value is in the main range.
    ///
    /// # Examples
    ///
    /// ```
    /// # use angulus::*;
    /// let x = MainAngle::from_turns(1.25f32);
    ///
    /// let abs_difference = (x.to_value() - 0.25).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn from_turns(turns: N) -> Self {
        Self::new(turns)
    }
}

//-------------------------------------------------------------------
// Unit convertion
//-------------------------------------------------------------------

impl<N: Num, U> MainAngle<N, U> {
    /// Converts the angle unit to another.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let rad = MainAngle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let deg = rad.to_unit::<Degrees>();
    ///
    /// let abs_difference = (deg.to_value() - 90.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_unit<V>(self) -> MainAngle<N, V>
    where
        U: ToUnit<N, V>,
    {
        MainAngle(self.0.to_unit::<V>())
    }

    /// Converts the angle unit to radians.
    ///
    /// Shortcut for `angle.to_unit::<Radians>()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let deg = MainAngle::from_degrees(90.0f32);
    /// let rad = deg.to_radians();
    ///
    /// let abs_difference = (rad.to_value() - std::f32::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_radians(self) -> MainAngle<N, Radians>
    where
        U: ToUnit<N, Radians>,
    {
        self.to_unit()
    }

    /// Converts the angle unit to degrees.
    ///
    /// Shortcut for `angle.to_unit::<Degrees>()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let rad = MainAngle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let deg = rad.to_degrees();
    ///
    /// let abs_difference = (deg.to_value() - 90.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_degrees(self) -> MainAngle<N, Degrees>
    where
        U: ToUnit<N, Degrees>,
    {
        self.to_unit()
    }

    /// Converts the angle unit to turns.
    ///
    /// Shortcut for `angle.to_unit::<Turns>()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let rad = MainAngle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let tr = rad.to_turns();
    ///
    /// let abs_difference = (tr.to_value() - 0.25).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_turns(self) -> MainAngle<N, Turns>
    where
        U: ToUnit<N, Turns>,
    {
        self.to_unit()
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<N: Num, U> MainAngle<N, U>
where
    U: ToUnit<N, Radians>,
{
    /// Computes the sine.
    #[inline]
    pub fn sin(self) -> N {
        self.0.sin()
    }
    /// Computes the cosine.
    #[inline]
    pub fn cos(self) -> N {
        self.0.cos()
    }
    /// Computes the tangent.
    #[inline]
    pub fn tan(self) -> N {
        self.0.tan()
    }
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    #[inline]
    pub fn sin_cos(self) -> (N, N) {
        self.0.sin_cos()
    }
}

//-------------------------------------------------------------------
// Ops
//-------------------------------------------------------------------

impl<N: Num, U> Add for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

impl<N: Num, U> Add<Angle<N, U>> for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn add(self, rhs: Angle<N, U>) -> Self::Output {
        self.0 + rhs
    }
}

impl<N: Num, U> Sub for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<N: Num, U> Sub<Angle<N, U>> for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn sub(self, rhs: Angle<N, U>) -> Self::Output {
        self.0 - rhs
    }
}

impl<N: Num, U> Mul<N> for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        self.0 * rhs
    }
}

impl<N: Num, U> Div<N> for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        self.0 / rhs
    }
}

impl<N: Num, U> Neg for MainAngle<N, U> {
    type Output = Angle<N, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.0.neg()
    }
}
