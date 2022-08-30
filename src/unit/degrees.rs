use super::{Radians, ToUnit, Turns, Unit, UnitName};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Degrees;

impl UnitName for Degrees {
    const UNIT: &'static str = "°";
}

impl Unit<f32> for Degrees {
    const FULL: f32 = 360.0f32;
    const HALF: f32 = 180.0f32;
    const QUARTER: f32 = 90.0f32;
    const SIXTH: f32 = 60.0f32;
    const EIGHTH: f32 = 45.0f32;
    const TWELFTH: f32 = 30.0f32;
    const SIXTEENTH: f32 = 22.5f32;
}

impl Unit<f64> for Degrees {
    const FULL: f64 = 360.0f64;
    const HALF: f64 = 180.0f64;
    const QUARTER: f64 = 90.0f64;
    const SIXTH: f64 = 60.0f64;
    const EIGHTH: f64 = 45.0f64;
    const TWELFTH: f64 = 30.0f64;
    const SIXTEENTH: f64 = 22.5f64;
}

impl ToUnit<f32, Radians> for Degrees {
    const FACTOR: f32 = 0.017_453_292_f32;
}
impl ToUnit<f64, Radians> for Degrees {
    const FACTOR: f64 = 0.017_453_292_519_943_295_f64;
}

impl ToUnit<f32, Turns> for Degrees {
    const FACTOR: f32 = 0.002_777_777_8_f32;
}
impl ToUnit<f64, Turns> for Degrees {
    const FACTOR: f64 = 0.002_777_777_777_777_778_f64;
}
