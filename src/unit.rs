mod degrees;
mod radians;
mod turns;

use std::fmt::Debug;

pub use degrees::Degrees;
pub use radians::Radians;
pub use turns::Turns;

use crate::Num;

/// Hold unit information.
pub trait UnitName {
    /// The unit symbol.
    const UNIT: &'static str;
}

/// Hold unit's special value for a specific numerical type.
pub trait Unit<N>: Default + Debug {
    /// The value of the full circle angle for this unit.
    const FULL: N;

    /// The value of the half of a circle angle for this unit.
    const HALF: N;

    /// The value of the quater of a circle angle for this unit.
    const QUARTER: N;

    /// The value of the sixth of a circle angle for this unit.
    const SIXTH: N;

    /// The value of the eighth of a circle angle for this unit.
    const EIGHTH: N;

    /// The value of the twelfth of a circle angle for this unit.
    const TWELFTH: N;

    /// The value of the sixteenth of a circle angle for this unit.
    const SIXTEENTH: N;
}

/// Hold the convertion factor between two units.
pub trait ToUnit<N, U> {
    /// Factor to convert from `Self` unit to `U` unit.
    const FACTOR: N;
}

impl<N: Num, U> ToUnit<N, U> for U {
    const FACTOR: N = N::ONE;
}
