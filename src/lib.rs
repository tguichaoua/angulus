#![deny(missing_docs)]

//! Unit agnostic angle.
//!
//! ## Overview
//!
//! Using simple floating point numbers to store an angle value is error-prone : you may add two
//! angle with one in radians and the second in degrees or you may try to compute the cosine of
//! a value in degrees and get an unexpected result.
//!
//! [`Angle`] and [`AngleUnbounded`] represent an angle value with no specific unit.
//!
//! [`Angle`] represent a canonical angle, i.e. the internal value fit the range `(-π, π]` in radians.
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
//! Conversely [`AngleUnbounded`] represent any angle value.
//!
//! ```
//! # use angulus::AngleUnbounded;
//! # fn main() {
//! let a = AngleUnbounded::from_degrees(90.0);
//! let b = AngleUnbounded::from_degrees(-270.0);
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
//! let grad = Angle::from_gradians(50.0);
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
//! let grad = 50.0.grad();
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
//! let a = Angle32::QUARTER;
//!
//! assert_eq!(a.to_radians(), std::f32::consts::FRAC_PI_2);
//! assert_eq!(a.to_degrees(), 90.0);
//! assert_eq!(a.to_turns(), 0.25);
//! assert_eq!(a.to_gradians(), 100.0);
//! # }
//! ```
//!
//! ## Display
//!
//! Since [`Angle`] and [`AngleUnbounded`] are unit agnotic they didn't implement the [`Display`][std::fmt::Display] trait.
//! But you can use one of the unit wrapper from [the units module][units] to specify a unit.
//!
//! ## Crate features
//!
//! - [`serde`] : (De)Serialization with the [serde crate](https://docs.rs/serde/latest/serde/).
//! - [`rand`] : Generate random angles with the [rand crate](https://docs.rs/rand/latest/rand/).

#[cfg(feature = "serde")]
pub mod serde;

#[allow(missing_docs)]
#[cfg(feature = "rand")]
pub mod rand;

mod angle;
pub mod float;
mod macros;
mod to_angle;
mod unbounded;
pub mod units;

pub use angle::Angle;
pub use to_angle::ToAngle;
pub use unbounded::AngleUnbounded;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// Type alias for [`Angle::<f32>`].
pub type Angle32 = Angle<f32>;

/// Type alias for [`Angle::<f64>`].
pub type Angle64 = Angle<f64>;

/// Type alias for [`AngleUnbounded::<f32>`].
pub type AngleUnbounded32 = AngleUnbounded<f32>;

/// Type alias for [`AngleUnbounded::<f64>`].
pub type AngleUnbounded64 = AngleUnbounded<f64>;
