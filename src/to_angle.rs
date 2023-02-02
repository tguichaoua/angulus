use crate::{Angle, AngleUnbounded, Float};

/// Helper trait to convert a numerical value into an angle.
pub trait ToAngle: Sized {
    /// Creates an angle with the value as radians.
    fn rad(self) -> Angle<Self>;
    /// Creates an unbounded angle with the value as radians.
    fn rad_unbounded(self) -> AngleUnbounded<Self>;
    /// Creates an angle with the value as degrees.
    fn deg(self) -> Angle<Self>;
    /// Creates an unbounded angle with the value as degrees.
    fn deg_unbounded(self) -> AngleUnbounded<Self>;
    /// Creates an angle with the value as turns.
    fn turns(self) -> Angle<Self>;
    /// Creates an unbounded angle with the value as turns.
    fn turns_unbounded(self) -> AngleUnbounded<Self>;
    /// Creates an angle with the value as gradians.
    fn g(self) -> Angle<Self>;
    /// Creates an unbounded angle with the value as gradians.
    fn g_unbounded(self) -> AngleUnbounded<Self>;
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

    fn g(self) -> Angle<Self> {
        Angle::from_gradians(self)
    }

    fn g_unbounded(self) -> AngleUnbounded<Self> {
        AngleUnbounded::from_gradians(self)
    }
}
