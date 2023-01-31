//! Unit agnostic angle.
//!
//! ## Overview
//!
//! Using simple floating point numbers to store an angle value is error-prone : you may add two
//! angle with one in radians and the second in degrees or you may try to compute the cosine of
//! a value in degrees and get an unexpected result.
//!
//! [`Angle`] and [`UnboundedAngle`] represent an angle value with no specific unit.
//!
//! [`Angle`] represent a canonical angle, i.e. the internal value fit the range `(-π; π]` in radians.
//!
//! For example, `90°` and `-270°` have different value but are the same angle.
//!
//! ```
//! # use angulus::Angle;
//! # fn main() {
//! let a = Angle::from_degrees(90.0);
//! let b = Angle::from_degrees(-270.0);
//!
//! assert_eq!(a, b);
//! # }
//! ```
//!
//! Conversely [`UnboundedAngle`] represent any angle value.
//!
//! ```
//! # use angulus::UnboundedAngle;
//! # fn main() {
//! let a = UnboundedAngle::from_degrees(90.0);
//! let b = UnboundedAngle::from_degrees(-270.0);
//!
//! assert_ne!(a, b);
//! # }
//! ```
//!
//! ## From value to angle
//!
//! To create an angle from a value, you can use the `from_*` methods with the unit of the value...
//!
//! ```
//! # use angulus::*;
//! # fn main() {
//! let deg = Angle::from_degrees(90.0);
//! let rad = Angle::from_radians(3.14);
//! let turns = Angle::from_turns(0.75);
//! # }
//! ```
//!
//! or you use the [`ToAngle`] trait directly on the value.
//!
//! ```
//! # use angulus::*;
//! # fn main() {
//! let deg = 90.0.deg();
//! let rad = 3.14.rad();
//! let turns = 0.75.turns();
//! # }
//! ```
//!
//! ## From angle to value
//!
//! To convert back an angle to a value you can use the `to_*` methods with the desired unit.
//!
//! ```
//! # use angulus::*;
//! # fn main() {
//! let a = Angle::<f32>::QUARTER;
//!
//! assert_eq!(a.to_radians(), std::f32::consts::FRAC_PI_2);
//! assert_eq!(a.to_degrees(), 90.0);
//! assert_eq!(a.to_turns(), 0.25);
//! # }
//! ```
//!
//! ## Display
//!
//! Since [`Angle`] and [`UnboundedAngle`] are unit agnotic they didn't implement the [`Display`][std::fmt::Display] trait.
//! But you can use one of the unit wrapper from [the units module][units] to specify a unit.
//!
//! ## Serde support
//!
//! The `serde` feature flag enable the support of [serde](https://crates.io/crates/serde).
//!
//! Even if [`Angle`] and [`UnboundedAngle`] are unit agnostic they (de)serialize from/into radians
//! for convenience.
//! But you can use one of the unit wrapper from [the units module][units] to specify a unit.

#[cfg(feature = "serde")]
mod serde;

mod angle;
mod num;
mod private;
mod to_angle;
mod unbounded;
pub mod units;

pub use angle::Angle;
pub use num::Num;
pub use to_angle::ToAngle;
pub use unbounded::UnboundedAngle;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
