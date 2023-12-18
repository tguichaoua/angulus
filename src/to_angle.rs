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
    fn rad(self) -> Angle<Self> {
        Angle::from_radians(self)
    }

    fn rad_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_radians(self)
    }

    fn deg(self) -> Angle<Self> {
        Angle::from_degrees(self)
    }

    fn deg_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_degrees(self)
    }

    fn turns(self) -> Angle<Self> {
        Angle::from_turns(self)
    }

    fn turns_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_turns(self)
    }

    fn grad(self) -> Angle<Self> {
        Angle::from_gradians(self)
    }

    fn grad_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_gradians(self)
    }
}
