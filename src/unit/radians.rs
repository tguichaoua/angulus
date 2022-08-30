use super::{Degrees, ToUnit, Turns, Unit, UnitName};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Radians;

impl UnitName for Radians {
    const UNIT: &'static str = " rad";
}

impl Unit<f32> for Radians {
    const FULL: f32 = std::f32::consts::TAU;
    const HALF: f32 = std::f32::consts::PI;
    const QUARTER: f32 = std::f32::consts::FRAC_PI_2;
    const SIXTH: f32 = std::f32::consts::FRAC_PI_3;
    const EIGHTH: f32 = std::f32::consts::FRAC_PI_4;
    const TWELFTH: f32 = std::f32::consts::FRAC_PI_6;
    const SIXTEENTH: f32 = std::f32::consts::FRAC_PI_8;
}

impl Unit<f64> for Radians {
    const FULL: f64 = std::f64::consts::TAU;
    const HALF: f64 = std::f64::consts::PI;
    const QUARTER: f64 = std::f64::consts::FRAC_PI_2;
    const SIXTH: f64 = std::f64::consts::FRAC_PI_3;
    const EIGHTH: f64 = std::f64::consts::FRAC_PI_4;
    const TWELFTH: f64 = std::f64::consts::FRAC_PI_6;
    const SIXTEENTH: f64 = std::f64::consts::FRAC_PI_8;
}

impl ToUnit<f32, Degrees> for Radians {
    const FACTOR: f32 = 57.295_78_f32;
}
impl ToUnit<f64, Degrees> for Radians {
    const FACTOR: f64 = 57.295_779_513_082_32_f64;
}

impl ToUnit<f32, Turns> for Radians {
    const FACTOR: f32 = 0.159_154_94_f32;
}
impl ToUnit<f64, Turns> for Radians {
    const FACTOR: f64 = 0.159_154_943_092_f64;
}
