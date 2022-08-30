use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{Degrees, MainAngle, Num, Radians, ToUnit, Turns, Unit, UnitName};

/// Represents a geometrical angle.
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
pub struct Angle<N, U = Radians> {
    pub(crate) value: N,
    _unit: PhantomData<U>,
}

impl<N, U> Angle<N, U> {
    /// Create an angle from the given value.
    #[inline]
    pub const fn new(value: N) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }

    /// The numerical value of the angle.
    #[inline]
    pub fn to_value(self) -> N {
        self.value
    }
}

impl<N: Num, U: Unit<N>> Angle<N, U> {
    /// Compute the main value of this angle.
    #[inline]
    pub fn main_angle(self) -> MainAngle<N, U> {
        MainAngle::new(self.value)
    }
}

impl<N, U> From<MainAngle<N, U>> for Angle<N, U> {
    #[inline]
    fn from(main_angle: MainAngle<N, U>) -> Self {
        main_angle.to_angle()
    }
}

impl<N: Num, U> Default for Angle<N, U> {
    fn default() -> Self {
        Self::ZERO
    }
}

//-------------------------------------------------------------------
// Format
//-------------------------------------------------------------------

impl<N: Debug, U: Unit<N>> Debug for Angle<N, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Angle")
            .field(&self.value)
            .field(&U::default())
            .finish()
    }
}

impl<N: Display, U: UnitName> Display for Angle<N, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, U::UNIT)
    }
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<N: Num, U> Angle<N, U> {
    /// The angle of value zero.
    pub const ZERO: Self = Angle::new(N::ZERO);
}

impl<N, U: Unit<N>> Angle<N, U> {
    /// The full circle angle.
    pub const FULL: Self = Angle::new(U::FULL);
    /// The half of a circle angle.
    pub const HALF: Self = Angle::new(U::HALF);
    /// The quarter of a circle angle.
    pub const QUARTER: Self = Angle::new(U::QUARTER);
    /// The sixth of a circle angle.
    pub const SIXTH: Self = Angle::new(U::SIXTH);
    /// The eighth of a circle angle.
    pub const EIGHTH: Self = Angle::new(U::EIGHTH);
    /// The twelfth of a circle angle.
    pub const TWELFTH: Self = Angle::new(U::TWELFTH);
    /// The sixteenth of a circle angle.
    pub const SIXTEENTH: Self = Angle::new(U::SIXTEENTH);
}

//-------------------------------------------------------------------
// from units
//-------------------------------------------------------------------

impl<N> Angle<N, Radians> {
    /// Creates a new angle with a value in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use angulus::*;
    /// let x = Angle::from_radians(std::f32::consts::FRAC_PI_2);
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub const fn from_radians(radians: N) -> Self {
        Self::new(radians)
    }
}

impl<N> Angle<N, Degrees> {
    /// Creates a new angle with a value in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// # use angulus::*;
    /// let x = Angle::from_degrees(90.0f32);
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub const fn from_degrees(degrees: N) -> Self {
        Self::new(degrees)
    }
}

impl<N> Angle<N, Turns> {
    /// Creates a new angle with a value in turns.
    ///
    /// # Examples
    ///
    /// ```
    /// # use angulus::*;
    /// let x = Angle::from_turns(0.25f32);
    ///
    /// let abs_difference = (x.sin() - 1.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub const fn from_turns(turns: N) -> Self {
        Self::new(turns)
    }
}

//-------------------------------------------------------------------
// Unit convertion
//-------------------------------------------------------------------

impl<N: Num, U> Angle<N, U> {
    /// Converts the angle unit to another.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let rad = Angle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let deg = rad.to_unit::<Degrees>();
    ///
    /// let abs_difference = (deg.to_value() - 90.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_unit<V>(self) -> Angle<N, V>
    where
        U: ToUnit<N, V>,
    {
        Angle::new(self.value * U::FACTOR)
    }

    /// Converts the angle unit to radians.
    ///
    /// Shortcut for `angle.to_unit::<Radians>()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use angulus::*;
    /// let deg = Angle::from_degrees(90.0f32);
    /// let rad = deg.to_radians();
    ///
    /// let abs_difference = (rad.to_value() - std::f32::consts::FRAC_PI_2).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_radians(self) -> Angle<N, Radians>
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
    /// let rad = Angle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let deg = rad.to_degrees();
    ///
    /// let abs_difference = (deg.to_value() - 90.0).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_degrees(self) -> Angle<N, Degrees>
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
    /// let rad = Angle::from_radians(std::f32::consts::FRAC_PI_2);
    /// let tr = rad.to_turns();
    ///
    /// let abs_difference = (tr.to_value() - 0.25).abs();
    ///
    /// assert!(abs_difference <= f32::EPSILON);
    /// ```
    #[inline]
    pub fn to_turns(self) -> Angle<N, Turns>
    where
        U: ToUnit<N, Turns>,
    {
        self.to_unit()
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<N: Num, U> Angle<N, U>
where
    U: ToUnit<N, Radians>,
{
    /// Computes the sine.
    #[inline]
    pub fn sin(self) -> N {
        self.to_radians().value.sin()
    }
    /// Computes the cosine.
    #[inline]
    pub fn cos(self) -> N {
        self.to_radians().value.cos()
    }
    /// Computes the tangent.
    #[inline]
    pub fn tan(self) -> N {
        self.to_radians().value.tan()
    }
    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    #[inline]
    pub fn sin_cos(self) -> (N, N) {
        self.to_radians().value.sin_cos()
    }
}

//-------------------------------------------------------------------
// Ops
//-------------------------------------------------------------------

impl<N: Num, U> Add for Angle<N, U> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<N: Num, U> Sub for Angle<N, U> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl<N: Num, U> Mul<N> for Angle<N, U> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

impl<N: Num, U> Div<N> for Angle<N, U> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

impl<N: Num, U> Neg for Angle<N, U> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(self.value.neg())
    }
}
