//! This module provides wrapper for angles to indicate in wich unit to convert them
//! to display them or (de)serialize them.
//!
//! ## Display
//!
//! Angle types didn't implement [`Display`] trait because they don't have canonical unit.
//!
//! Unit wrapper implement the [`Display`] trait by writting the angle value in the desired
//! unit followed by the unit symbole.
//!
//! ```
//! # use angulus::{Angle, ToAngle, units::{Degrees, Radians, Turns}};
//! # fn main() {
//! let angle = 90.0_f32.deg();
//!
//! let rad = Radians(angle);
//! let deg = Degrees(angle);
//! let turns = Turns(angle);
//!
//! assert_eq!(format!("{rad}"), "3.14 rad");
//! assert_eq!(format!("{deg}"), "90.0°");
//! assert_eq!(format!("{turns}"), "0.25 tr");
//! # }
//! ```
//!
//! ## (De)Serialization
//!
//! By default, angle types (de)serialize into/from radians.
//! By using a unit wrapper, the value is (de)serialize into/from the desire unit.
//!
//! ```
//! # use angulus::{Angle, units::{Degrees, Radians, Turns}};
//! # use float_eq::assert_float_eq;
//! # use serde::{Serialize, Deserialize};
//! # fn main() {
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!     rad: Radians<Angle<f32>>,
//!     deg: Degrees<Angle<f32>>,
//!     tr: Turns<Angle<f32>>,
//! }
//!
//! let json = serde_json::json!{
//!     {
//!         "rad": 1.0,
//!         "deg": 90.0,
//!         "tr": 0.5,
//!     }
//! };
//!
//! let foo: Foo = serde_json::from_value(json).unwrap();
//!
//! assert_float_eq!(foo.rad.0.to_radians(), 1.0, abs <= 0.000001);
//! assert_float_eq!(foo.deg.0.to_degrees(), 90.0, abs <= 0.000001);
//! assert_float_eq!(foo.tr.0.to_turns(), 0.5, abs <= 0.000001);
//! # }
//! ```

use std::fmt::Display;

use crate::utility::AngleConvertion;

//-------------------------------------------------------------------

/// Unit wrapper to manipulate angle in radians.
///
/// See the [module level documentation][self] for more details.
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

/// Unit wrapper to manipulate angle in degrees.
///
/// See the [module level documentation][self] for more details.
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

/// Unit wrapper to manipulate angle in turns.
///
/// See the [module level documentation][self] for more details.
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
