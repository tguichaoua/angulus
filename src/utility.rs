pub(crate) trait AngleConvertion {
    type N;

    fn from_radians(radians: Self::N) -> Self;
    fn from_degrees(degrees: Self::N) -> Self;
    fn from_turns(turns: Self::N) -> Self;

    fn to_radians(&self) -> Self::N;
    fn to_degrees(&self) -> Self::N;
    fn to_turns(&self) -> Self::N;
}
