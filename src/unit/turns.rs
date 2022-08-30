use crate::Num;

use super::{Degrees, Radians, ToUnit, Unit, UnitName};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Turns;

impl UnitName for Turns {
    const UNIT: &'static str = " tr";
}

impl Unit<f32> for Turns {
    const FULL: f32 = 1.0f32;
    const HALF: f32 = 0.5f32;
    const QUARTER: f32 = 0.25f32;
    const SIXTH: f32 = 0.166_666_67_f32;
    const EIGHTH: f32 = 0.125f32;
    const TWELFTH: f32 = 0.083_333_333_f32;
    const SIXTEENTH: f32 = 0.0625f32;
}

impl Unit<f64> for Turns {
    const FULL: f64 = 1.0f64;
    const HALF: f64 = 0.5f64;
    const QUARTER: f64 = 0.25f64;
    const SIXTH: f64 = 0.166_666_67_f64;
    const EIGHTH: f64 = 0.125f64;
    const TWELFTH: f64 = 0.083_333_333_f64;
    const SIXTEENTH: f64 = 0.0625f64;
}

impl<N: Num> ToUnit<N, Radians> for Turns
where
    Radians: Unit<N>,
{
    const FACTOR: N = Radians::FULL;
}

impl<N: Num> ToUnit<N, Degrees> for Turns
where
    Degrees: Unit<N>,
{
    const FACTOR: N = Degrees::FULL;
}
