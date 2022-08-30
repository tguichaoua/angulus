use crate::{Angle, Degrees, Radians, Turns};

/// Helper trait to convert a numerical value into an angle.
pub trait ToAngle: Sized {
    fn rad(self) -> Angle<Self, Radians>;
    fn deg(self) -> Angle<Self, Degrees>;
    fn turns(self) -> Angle<Self, Turns>;
}

impl<N> ToAngle for N {
    fn rad(self) -> Angle<Self, Radians> {
        Angle::from_radians(self)
    }

    fn deg(self) -> Angle<Self, Degrees> {
        Angle::from_degrees(self)
    }

    fn turns(self) -> Angle<Self, Turns> {
        Angle::from_turns(self)
    }
}
