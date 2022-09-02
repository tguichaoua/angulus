use std::fmt::Display;

use crate::utility::AngleConvertion;

//-------------------------------------------------------------------

pub struct Radians<A>(pub A);

impl<A> From<A> for Radians<A> {
    fn from(x: A) -> Self {
        Self(x)
    }
}

impl<A: AngleConvertion> Display for Radians<A>
where
    A::N: Display,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} rad", self.0.to_radians())
    }
}

//-------------------------------------------------------------------

pub struct Degrees<A>(pub A);

impl<A> From<A> for Degrees<A> {
    fn from(x: A) -> Self {
        Self(x)
    }
}

impl<A: AngleConvertion> Display for Degrees<A>
where
    A::N: Display,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°", self.0.to_degrees())
    }
}

//-------------------------------------------------------------------

pub struct Turns<A>(pub A);

impl<A> From<A> for Turns<A> {
    fn from(x: A) -> Self {
        Self(x)
    }
}

impl<A: AngleConvertion> Display for Turns<A>
where
    A::N: Display,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} tr", self.0.to_turns())
    }
}
