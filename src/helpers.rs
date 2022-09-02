use crate::{Angle, Num};

/// Helper trait to convert a numerical value into an angle.
pub trait ToAngle: Sized {
    fn rad(self) -> Angle<Self>;
    fn deg(self) -> Angle<Self>;
    fn turns(self) -> Angle<Self>;
}

impl<N: Num> ToAngle for N {
    fn rad(self) -> Angle<Self> {
        Angle::from_radians(self)
    }

    fn deg(self) -> Angle<Self> {
        Angle::from_degrees(self)
    }

    fn turns(self) -> Angle<Self> {
        Angle::from_turns(self)
    }
}
