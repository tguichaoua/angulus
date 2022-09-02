use crate::{Angle, Num, UnboundedAngle};

/// Helper trait to convert a numerical value into an angle.
pub trait ToAngle: Sized {
    /// Creates an angle with the value as radians.
    fn rad(self) -> Angle<Self>;
    /// Creates an unboudned angle with the value as radians.
    fn rad_unbounded(self) -> UnboundedAngle<Self>;
    /// Creates an angle with the value as degrees.
    fn deg(self) -> Angle<Self>;
    /// Creates an unboudned angle with the value as degrees.
    fn deg_unbounded(self) -> UnboundedAngle<Self>;
    /// Creates an angle with the value as turns.
    fn turns(self) -> Angle<Self>;
    /// Creates an unboudned angle with the value as turns.
    fn turns_unbounded(self) -> UnboundedAngle<Self>;
}

impl<F: Num> ToAngle for F {
    fn rad(self) -> Angle<Self> {
        Angle::from_radians(self)
    }

    fn rad_unbounded(self) -> UnboundedAngle<Self> {
        UnboundedAngle::from_radians(self)
    }

    fn deg(self) -> Angle<Self> {
        Angle::from_degrees(self)
    }

    fn deg_unbounded(self) -> UnboundedAngle<Self> {
        UnboundedAngle::from_degrees(self)
    }

    fn turns(self) -> Angle<Self> {
        Angle::from_turns(self)
    }

    fn turns_unbounded(self) -> UnboundedAngle<Self> {
        UnboundedAngle::from_turns(self)
    }
}
