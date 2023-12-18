use crate::float::Float;
use crate::{Angle, AngleUnbounded};

/// Helper trait to convert a numerical value into an angle.
pub trait ToAngle: Sized {
    /// Creates an angle with the value as radians.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rad(self) -> Angle<Self>;

    /// Creates an unbounded angle with the value as radians.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn rad_unbounded(self) -> AngleUnbounded<Self>;

    /// Creates an angle with the value as degrees.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn deg(self) -> Angle<Self>;

    /// Creates an unbounded angle with the value as degrees.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn deg_unbounded(self) -> AngleUnbounded<Self>;

    /// Creates an angle with the value as turns.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn turns(self) -> Angle<Self>;

    /// Creates an unbounded angle with the value as turns.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn turns_unbounded(self) -> AngleUnbounded<Self>;

    /// Creates an angle with the value as gradians.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn grad(self) -> Angle<Self>;

    /// Creates an unbounded angle with the value as gradians.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn grad_unbounded(self) -> AngleUnbounded<Self>;
}

impl<F: Float> ToAngle for F {
    #[inline]
    fn rad(self) -> Angle<Self> {
        Angle::from_radians(self)
    }

    #[inline]
    fn rad_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_radians(self)
    }

    #[inline]
    fn deg(self) -> Angle<Self> {
        Angle::from_degrees(self)
    }

    #[inline]
    fn deg_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_degrees(self)
    }

    #[inline]
    fn turns(self) -> Angle<Self> {
        Angle::from_turns(self)
    }

    #[inline]
    fn turns_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_turns(self)
    }

    #[inline]
    fn grad(self) -> Angle<Self> {
        Angle::from_gradians(self)
    }

    #[inline]
    fn grad_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_gradians(self)
    }
}
