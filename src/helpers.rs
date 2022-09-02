use crate::{Angle, Num, UnboundedAngle};

/// Helper trait to convert a numerical value into an angle.
pub trait ToAngle: Sized {
    fn rad(self) -> Angle<Self>;
    fn rad_unbounded(self) -> UnboundedAngle<Self>;
    fn deg(self) -> Angle<Self>;
    fn deg_unbounded(self) -> UnboundedAngle<Self>;
    fn turns(self) -> Angle<Self>;
    fn turns_unbounded(self) -> UnboundedAngle<Self>;
}

impl<N: Num> ToAngle for N {
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
