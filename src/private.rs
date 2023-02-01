use crate::{Angle, AngleUnbounded, Num};

/// Marker trait for angle types.
///
/// Used internally.
pub(crate) trait IAngle: Copy {
    /// The numerical type used by the angle.
    type Num;

    fn from_radians(radians: Self::Num) -> Self;
    fn from_degrees(degrees: Self::Num) -> Self;
    fn from_turns(turns: Self::Num) -> Self;
    fn from_gradians(gradians: Self::Num) -> Self;

    fn to_radians(self) -> Self::Num;
    fn to_degrees(self) -> Self::Num;
    fn to_turns(self) -> Self::Num;
    fn to_gradians(self) -> Self::Num;
}

impl<F: Num> IAngle for Angle<F> {
    type Num = F;

    #[inline]
    fn from_radians(radians: F) -> Self {
        Self::from_radians(radians)
    }

    #[inline]
    fn from_degrees(degrees: F) -> Self {
        Self::from_degrees(degrees)
    }

    #[inline]
    fn from_turns(turns: F) -> Self {
        Self::from_turns(turns)
    }

    #[inline]
    fn from_gradians(gradians: F) -> Self {
        Self::from_gradians(gradians)
    }

    #[inline]
    fn to_radians(self) -> F {
        self.to_radians()
    }

    #[inline]
    fn to_degrees(self) -> F {
        self.to_degrees()
    }

    #[inline]
    fn to_turns(self) -> F {
        self.to_turns()
    }

    #[inline]
    fn to_gradians(self) -> Self::Num {
        self.to_gradians()
    }
}

impl<F: Num> IAngle for AngleUnbounded<F> {
    type Num = F;

    #[inline]
    fn from_radians(radians: F) -> Self {
        Self::from_radians(radians)
    }

    #[inline]
    fn from_degrees(degrees: F) -> Self {
        Self::from_degrees(degrees)
    }

    #[inline]
    fn from_turns(turns: F) -> Self {
        Self::from_turns(turns)
    }

    #[inline]
    fn from_gradians(gradians: F) -> Self {
        Self::from_gradians(gradians)
    }

    #[inline]
    fn to_radians(self) -> F {
        self.to_radians()
    }

    #[inline]
    fn to_degrees(self) -> F {
        self.to_degrees()
    }

    #[inline]
    fn to_turns(self) -> F {
        self.to_turns()
    }

    #[inline]
    fn to_gradians(self) -> Self::Num {
        self.to_gradians()
    }
}
