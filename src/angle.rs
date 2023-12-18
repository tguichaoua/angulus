use core::fmt::Debug;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::float::Float;
use crate::macros::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};
use crate::AngleUnbounded;

/// Represents a point on the circle as a unit-agnostic angle.
///
/// The parameter `F` is the floating-point type used to store the value.
///
/// # Behaviour
///
/// Two different values of the same point on the circle are the same [`Angle`].
///
/// ```
/// # use angulus::Angle;
/// let a = Angle::from_degrees(90.0);
/// let b = Angle::from_degrees(450.0);
///
/// assert_eq!(a, b);
/// ```
///
/// # Why doesn't it implement [`PartialOrd`] ?
///
/// Because [`Angle`]s represent points on the circle (i.e., not a numerical value), they cannot be ordered.
///
/// # The `NaN` angle
///
/// An angle can be `NaN` in the following cases:
///
/// - Create an angle from a non-finite value;
/// ```
/// # use angulus::Angle;
/// let a = Angle::from_radians(f32::INFINITY);
/// assert!(a.is_nan());
///
/// let b = Angle::from_radians(f32::NAN);
/// assert!(b.is_nan());
/// ```
/// - Doing an operation that result into a non-finite value;
/// ```
/// # use angulus::Angle;
/// let a = Angle::DEG_90;
/// let b = a / 0.0;
///
/// assert!(b.is_nan());
/// ```
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    /// Creates a new angle from a value in radians assuming it is already in
    /// the the range `[-2π, 2π]`.
    #[inline]
    fn from_radians_partially_unchecked(radians: F) -> Self {
        debug_assert!(radians.is_nan() || (-F::TAU <= radians && radians <= F::TAU));
        let radians = if radians > F::PI {
            radians - F::TAU
        } else if radians <= -F::PI {
            radians + F::TAU
        } else {
            radians
        };
        Self::from_radians_unchecked(radians)
    }

    /// Creates a new angle from a value in radians.
    #[inline]
    pub fn from_radians(radians: F) -> Self {
        Self::from_radians_partially_unchecked(radians % F::TAU)
    }

    /// Creates a new angle from a value in degrees.
    #[inline]
    pub fn from_degrees(degrees: F) -> Self {
        Self::from_radians(degrees * F::DEG_TO_RAD)
    }

    /// Creates a new angle from a value in turns.
    #[inline]
    pub fn from_turns(turns: F) -> Self {
        // NOTE: to avoid `NaN` when handling big values, we have to operate the REM before
        // converting the value into radians.
        Self::from_radians_partially_unchecked((turns % F::ONE) * F::TURNS_TO_RAD)
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
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub const fn to_radians(self) -> F {
        self.radians
    }
}

impl<F: Float> Angle<F> {
    /// The value of the angle in degrees.
    ///
    /// This value is in the range `(-180, 180]`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn to_degrees(self) -> F {
        self.radians * F::RAD_TO_DEG
    }

    /// The value of the angle in turns.
    ///
    /// This value is in the range `(-0.5, 0.5]`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn to_turns(self) -> F {
        self.radians * F::RAD_TO_TURNS
    }

    /// The value of the angle in gradians.
    ///
    /// This value is in the range `(-200, 200]`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn to_gradians(self) -> F {
        self.radians * F::RAD_TO_GRAD
    }
}

impl<F: Float> Angle<F> {
    /// Returns `true` if this angle is NaN.
    ///
    /// See [`Angle` documentation][Angle] for more details.
    #[must_use]
    #[inline]
    pub fn is_nan(self) -> bool {
        self.radians.is_nan()
    }
}

//-------------------------------------------------------------------
// Angle conversion
//-------------------------------------------------------------------

impl<F: Copy> Angle<F> {
    /// Converts this angle into an unbounded angle in [the main range](crate#the-main-range).
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
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
// Floating point type conversion
//-------------------------------------------------------------------

impl Angle<f32> {
    /// Converts the floating point type to [`f64`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn to_f64(self) -> Angle<f64> {
        let radians = f64::from(self.radians);
        debug_assert!(
            radians.is_nan()
                || (-core::f64::consts::PI < radians && radians <= core::f64::consts::PI)
        );
        // Notes: f32 to f64 conversion is lossless, we don't need to check the range.
        Angle::from_radians_unchecked(radians)
    }
}

impl Angle<f64> {
    /// Converts the floating point type to [`f32`].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn to_f32(self) -> Angle<f32> {
        #[allow(clippy::cast_possible_truncation)]
        let radians = self.radians as f32;
        Angle::from_radians(radians)
    }
}

impl From<Angle<f64>> for Angle<f32> {
    #[inline]
    fn from(value: Angle<f64>) -> Self {
        value.to_f32()
    }
}

impl From<Angle<f32>> for Angle<f64> {
    #[inline]
    fn from(value: Angle<f32>) -> Self {
        value.to_f64()
    }
}

//-------------------------------------------------------------------
// Maths
//-------------------------------------------------------------------

#[cfg(any(feature = "std", feature = "libm"))]
impl<F: crate::float::FloatMath> Angle<F> {
    /// Computes the sine.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn sin(self) -> F {
        self.radians.sin()
    }

    /// Computes the cosine.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn cos(self) -> F {
        self.radians.cos()
    }

    /// Computes the tangent.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn tan(self) -> F {
        self.radians.tan()
    }

    /// Simultaneously computes the sine and cosine. Returns `(sin(x), cos(x))`.
    #[must_use = "this returns the result of the operation, without modifying the original"]
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
        debug_assert!(self.radians.is_nan() || (-F::PI < self.radians && self.radians <= F::PI));
        if self.radians == F::PI {
            self
        } else {
            Self::from_radians_unchecked(-self.radians)
        }
    }
}

forward_ref_unop!(impl<F: Float> Neg, neg for Angle<F>);

impl<F: Float + Sum> Sum for Angle<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Angle::from_radians(iter.map(|x| x.radians).sum())
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Angle, Angle32};

    #[test]
    fn angle_pi_eq_neg_pi() {
        assert_eq!(
            Angle::from_radians(core::f32::consts::PI),
            Angle::from_radians(-core::f32::consts::PI),
        );
    }

    #[test]
    fn angle_sum_is_accurate() {
        const ANGLES: [f32; 20] = [
            0.711_889,
            0.612_456_56,
            -1.165_211_3,
            -1.452_307,
            -0.587_785_5,
            0.593_006_5,
            0.012_860_533,
            -1.142_349_6,
            -1.302_776_9,
            0.510_909_6,
            2.365_249_2,
            1.743_016_8,
            1.165_635_7,
            -2.191_822_8,
            2.505_914_4,
            0.498_677,
            2.496_595,
            -0.108_386_315,
            0.991_436_9,
            -2.835_525_5,
        ];

        let angles = ANGLES.map(Angle32::from_radians);

        let sum: Angle32 = angles.iter().copied().sum();
        let add = angles.iter().copied().fold(Angle::ZERO, |a, b| a + b);

        assert_float_eq!(sum.to_radians(), add.to_radians(), abs <= 1e-5);
    }

    #[test]
    fn angle_from_nan_is_nan() {
        macro_rules! test {
            (
                $($nan:expr),*
            ) => {
                $(
                    assert!(Angle::from_radians($nan).is_nan());
                    assert!(Angle::from_degrees($nan).is_nan());
                    assert!(Angle::from_turns($nan).is_nan());
                    assert!(Angle::from_gradians($nan).is_nan());
                )*
            };
        }
        test!(f32::NAN, f64::NAN);
    }

    #[test]
    fn angle_from_infinity_is_nan() {
        macro_rules! test {
            (
                $($inf:expr),*
            ) => {
                $(
                    assert!(Angle::from_radians($inf).is_nan());
                    assert!(Angle::from_degrees($inf).is_nan());
                    assert!(Angle::from_turns($inf).is_nan());
                    assert!(Angle::from_gradians($inf).is_nan());
                )*
            };
        }
        test!(
            f32::INFINITY,
            f32::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY
        );
    }

    #[test]
    fn angle_from_big_value_is_not_nan() {
        macro_rules! test {
            (
                $($big_value:expr),*
            ) => {
                $(
                    assert!(!Angle::from_radians($big_value).is_nan());
                    assert!(!Angle::from_degrees($big_value).is_nan());
                    assert!(!Angle::from_turns($big_value).is_nan());
                    assert!(!Angle::from_gradians($big_value).is_nan());
                )*
            };
        }
        test!(f32::MAX, f32::MIN, f64::MAX, f64::MIN);
    }
}
