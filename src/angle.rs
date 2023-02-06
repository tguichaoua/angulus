use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
    float::Float, forward_ref_binop, forward_ref_op_assign, forward_ref_unop, AngleUnbounded,
};

/// Represents a point on the circle as an unit agnostic angle.
///
/// The parameter `F` is the floating-point type used to store the value.
///
/// ## Behaviour
///
/// Two different values of the same point on the circle are the same [`Angle`] :
///
/// ```
/// # use angulus::Angle;
/// let a = Angle::from_degrees(90.0);
/// let b = Angle::from_degrees(450.0);
///
/// assert_eq!(a, b);
/// ```
///
/// To preserve the difference use [`AngleUnbounded`].
///
/// ## Why does it doesn't implement [`PartialOrd`] ?
///
/// Because [`Angle`]s represents points on the circle (i.e. not a numerical value), they cannot be ordered.
///
/// ## The main range
///
/// The main range for an angle is :
///
/// - `(-π, π]` radians
/// - `(-180, 180]` degrees
/// - `(-0.5, 0.5]` turns
/// - `(-200, 200]` gradians
///
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Angle<F> {
    radians: F,
}

//-------------------------------------------------------------------
// Const
//-------------------------------------------------------------------

impl<F: Float> Angle<F> {
    /// The angle of value zero.
    pub const ZERO: Self = Angle::from_radians_unchecked(F::ZERO);

    /// [Machine epsilon] value for [`Angle`].
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: Self = Angle::from_radians_unchecked(F::DOUBLE_EPSILON);
}

impl<F: Float> Angle<F> {
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

impl<F: Float> Angle<F> {
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

impl<F: Float> Angle<F> {
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

impl<F: Float> Angle<F> {
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

impl<F: Debug> Debug for Angle<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Angle").field(&self.radians).finish()
    }
}

impl<F: Float> Default for Angle<F> {
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
    /// the the range `(-π, π]`.
    #[inline]
    pub const fn from_radians_unchecked(radians: F) -> Self {
        Self { radians }
    }
}

impl<F: Float> Angle<F> {
    /// Creates a new angle from a value in radians.
    #[inline]
    pub fn from_radians(radians: F) -> Self {
        let radians = radians % F::TAU;
        let radians = if radians > F::PI {
            radians - F::TAU
        } else if radians <= -F::PI {
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

    /// Creates a new angle from a value in gradians.
    #[inline]
    pub fn from_gradians(gradians: F) -> Self {
        Self::from_radians(gradians * F::GRAD_TO_RAD)
    }
}

//-------------------------------------------------------------------
// Getters
//-------------------------------------------------------------------

impl<F: Copy> Angle<F> {
    /// The value of the angle in radians.
    ///
    /// This value is in the range `(-π, π]`.
    #[inline]
    pub const fn to_radians(self) -> F {
        self.radians
    }
}

impl<F: Float> Angle<F> {
    /// The value of the angle in degrees.
    ///
    /// This value is in the range `(-180, 180]`.
    #[inline]
    pub fn to_degrees(self) -> F {
        self.radians * F::RAD_TO_DEG
    }

    /// The value of the angle in turns.
    ///
    /// This value is in the range `(-0.5, 0.5]`.
    #[inline]
    pub fn to_turns(self) -> F {
        self.radians * F::RAD_TO_TURNS
    }

    /// The value of the angle in gradians.
    ///
    /// This value is in the range `(-200, 200]`.
    #[inline]
    pub fn to_gradians(self) -> F {
        self.radians * F::RAD_TO_GRAD
    }
}

//-------------------------------------------------------------------
// Angle convertion
//-------------------------------------------------------------------

impl<F: Copy> Angle<F> {
    /// Converts this angle into an unbounded angle in [the main range](Angle#the-main-range).
    pub const fn to_unbounded(self) -> AngleUnbounded<F> {
        AngleUnbounded::from_radians(self.radians)
    }
}

impl<F: Float> From<AngleUnbounded<F>> for Angle<F> {
    #[inline]
    fn from(angle: AngleUnbounded<F>) -> Self {
        Self::from_radians(angle.to_radians())
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

impl<F: Float> Angle<F> {
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

impl<F: Float> Add for Angle<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians + rhs.radians)
    }
}

forward_ref_binop!(impl<F: Float> Add, add for Angle<F>, Angle<F>);

impl<F: Float> AddAssign for Angle<F> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

forward_ref_op_assign!(impl<F: Float> AddAssign, add_assign for Angle<F>, Angle<F>);

impl<F: Float> Sub for Angle<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians - rhs.radians)
    }
}

forward_ref_binop!(impl<F: Float> Sub, sub for Angle<F>, Angle<F>);

impl<F: Float> SubAssign for Angle<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

forward_ref_op_assign!(impl<F: Float> SubAssign, sub_assign for Angle<F>, Angle<F>);

impl<F: Float> Mul<F> for Angle<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

forward_ref_binop!(impl<F: Float> Mul, mul for Angle<F>, F);

impl Mul<Angle<f32>> for f32 {
    type Output = Angle<f32>;

    #[inline]
    fn mul(self, rhs: Angle<f32>) -> Self::Output {
        rhs * self
    }
}

forward_ref_binop!(impl Mul, mul for f32, Angle<f32>);

impl Mul<Angle<f64>> for f64 {
    type Output = Angle<f64>;

    #[inline]
    fn mul(self, rhs: Angle<f64>) -> Self::Output {
        rhs * self
    }
}

forward_ref_binop!(impl Mul, mul for f64, Angle<f64>);

impl<F: Float> MulAssign<F> for Angle<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        *self = *self * rhs;
    }
}

forward_ref_op_assign!(impl<F: Float> MulAssign, mul_assign for Angle<F>, F);

impl<F: Float> Div<F> for Angle<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

forward_ref_binop!(impl<F: Float> Div, div for Angle<F>, F);

impl<F: Float> DivAssign<F> for Angle<F> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        *self = *self / rhs;
    }
}

forward_ref_op_assign!(impl<F: Float> DivAssign, div_assign for Angle<F>, F);

impl<F: Float> Neg for Angle<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        debug_assert!(-F::PI < self.radians && self.radians <= F::PI);
        if self.radians == F::PI {
            self
        } else {
            Self::from_radians_unchecked(-self.radians)
        }
    }
}

forward_ref_unop!(impl<F: Float> Neg, neg for Angle<F>);

#[cfg(test)]
mod tests {
    use super::Angle;

    #[test]
    fn pi_eq_neg_pi() {
        assert_eq!(
            Angle::from_radians(std::f32::consts::PI),
            Angle::from_radians(-std::f32::consts::PI),
        )
    }
}
