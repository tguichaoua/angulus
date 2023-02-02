use crate::{Angle, AngleUnbounded, Float};

/// Marker trait for angle types.
///
/// Used internally.
pub(crate) trait IAngle: Copy {
    /// The floating-point type used by the angle.
    type Float;

    fn from_radians(radians: Self::Float) -> Self;
    fn from_degrees(degrees: Self::Float) -> Self;
    fn from_turns(turns: Self::Float) -> Self;
    fn from_gradians(gradians: Self::Float) -> Self;

    fn to_radians(self) -> Self::Float;
    fn to_degrees(self) -> Self::Float;
    fn to_turns(self) -> Self::Float;
    fn to_gradians(self) -> Self::Float;
}

impl<F: Float> IAngle for Angle<F> {
    type Float = F;

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
    fn to_gradians(self) -> Self::Float {
        self.to_gradians()
    }
}

impl<F: Float> IAngle for AngleUnbounded<F> {
    type Float = F;

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
    fn to_gradians(self) -> Self::Float {
        self.to_gradians()
    }
}
