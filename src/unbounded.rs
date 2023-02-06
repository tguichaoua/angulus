use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{float::Float, forward_ref_binop, forward_ref_op_assign, forward_ref_unop, Angle};

/// Represents a point on the circle as an unit agnostic angle.
///
/// The parameter `F` is the floating-point type used to store the value.
///
/// ## Behaviour
/// Unlike [`Angle`], two different values of the same point on the circle are different
/// angles :
///
/// ```
/// # use angulus::AngleUnbounded;
/// let a = AngleUnbounded::from_degrees(90.0);
/// let b = AngleUnbounded::from_degrees(450.0);
///
/// assert_ne!(a, b);
/// ```

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct AngleUnbounded<F> {
    radians: F,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<F: Float> AngleUnbounded<F> {
    /// The angle of value zero.
    pub const ZERO: Self = AngleUnbounded::from_radians(F::ZERO);

    /// [Machine epsilon] value for [`AngleUnbounded`].
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: Self = AngleUnbounded::from_radians(F::DOUBLE_EPSILON);
}

impl<F: Float> AngleUnbounded<F> {
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

impl<F: Float> AngleUnbounded<F> {
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

impl<F: Float> AngleUnbounded<F> {
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

impl<F: Float> AngleUnbounded<F> {
    /// The angle of 200g.
    pub const GRAD_200: Self = Self::RAD_PI;
    /// The angle of 100g.
    pub const GRAD_100: Self = Self::RAD_FRAC_PI_2;
    /// The angle of 66.6g.
    pub const GRAD_66_6: Self = Self::RAD_FRAC_PI_3;
    ///  The angle of 50g.
    pub const GRAD_50: Self = Self::RAD_FRAC_PI_4;
    ///  The angle of 33.3g.
    pub const GRAD_33_3: Self = Self::RAD_FRAC_PI_6;
    ///  The angle of 25g.
    pub const GRAD_25: Self = Self::RAD_FRAC_PI_8;
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

impl<F: Float> Default for AngleUnbounded<F> {
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

impl<F: Float> AngleUnbounded<F> {
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

    /// Creates a new unbounded angle from a value in gradians.
    #[inline]
    pub fn from_gradians(gradians: F) -> Self {
        Self::from_radians(gradians * F::GRAD_TO_RAD)
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

impl<F: Float> AngleUnbounded<F> {
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

    /// The value of the unbounded angle in gradians.
    #[inline]
    pub fn to_gradians(self) -> F {
        self.radians * F::RAD_TO_GRAD
    }
}

//-------------------------------------------------------------------
// MainAngle convertion
//-------------------------------------------------------------------

impl<F: Float> AngleUnbounded<F> {
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

impl<F: Float> AngleUnbounded<F> {
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

impl<F: Float> Add for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

forward_ref_binop!(impl<F: Float> Add, add for AngleUnbounded<F>, AngleUnbounded<F>);

impl<F: Float> AddAssign for AngleUnbounded<F> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.radians += rhs.radians;
    }
}

forward_ref_op_assign!(impl<F: Float> AddAssign, add_assign for AngleUnbounded<F>, AngleUnbounded<F>);

impl<F: Float> Sub for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

forward_ref_binop!(impl<F: Float> Sub, sub for AngleUnbounded<F>, AngleUnbounded<F>);

impl<F: Float> SubAssign for AngleUnbounded<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.radians -= rhs.radians
    }
}

forward_ref_op_assign!(impl<F: Float> SubAssign, sub_assign for AngleUnbounded<F>, AngleUnbounded<F>);

impl<F: Float> Mul<F> for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

forward_ref_binop!(impl<F: Float> Mul, mul for AngleUnbounded<F>, F);

impl Mul<AngleUnbounded<f32>> for f32 {
    type Output = AngleUnbounded<f32>;

    #[inline]
    fn mul(self, rhs: AngleUnbounded<f32>) -> Self::Output {
        rhs * self
    }
}

forward_ref_binop!(impl Mul, mul for f32, AngleUnbounded<f32>);

impl Mul<AngleUnbounded<f64>> for f64 {
    type Output = AngleUnbounded<f64>;

    #[inline]
    fn mul(self, rhs: AngleUnbounded<f64>) -> Self::Output {
        rhs * self
    }
}

forward_ref_binop!(impl Mul, mul for f64, AngleUnbounded<f64>);

impl<F: Float> MulAssign<F> for AngleUnbounded<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self.radians *= rhs;
    }
}

forward_ref_op_assign!(impl<F: Float> MulAssign, mul_assign for AngleUnbounded<F>, F);

impl<F: Float> Div<F> for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

forward_ref_binop!(impl<F: Float> Div, div for AngleUnbounded<F>, F);

impl<F: Float> DivAssign<F> for AngleUnbounded<F> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self.radians /= rhs;
    }
}

forward_ref_op_assign!(impl<F: Float> DivAssign, div_assign for AngleUnbounded<F>, F);

impl<F: Float> Neg for AngleUnbounded<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_radians(-self.radians)
    }
}

forward_ref_unop!(impl<F: Float> Neg, neg for AngleUnbounded<F>);
