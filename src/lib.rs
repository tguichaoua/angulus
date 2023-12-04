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
//! ## [`Angle`] vs [`AngleUnbounded`]
//!
//! Both represent a point on the circle as a unit agnostic angle.
//!
//! But [`Angle`] considere two different values of the same point as the same angle :
//!
//! ```
//! # use angulus::Angle;
//! let a = Angle::from_degrees(90.0);
//! let b = Angle::from_degrees(450.0);
//!
//! assert_eq!(a, b);
//! ```
//!
//! While [`AngleUnbounded`] considere those value as two different angle :
//!
//! ```
//! # use angulus::AngleUnbounded;
//! let a = AngleUnbounded::from_degrees(90.0);
//! let b = AngleUnbounded::from_degrees(450.0);
//!
//! assert_ne!(a, b);
//! ```
//!
//! ## From value to angle
//!
//! To create an angle from a value, you can use the `from_*` methods with the unit of the value...
//!
//! ```
//! # use angulus::Angle;
//! let deg = Angle::from_degrees(90.0);
//! let rad = Angle::from_radians(3.14);
//! let turns = Angle::from_turns(0.75);
//! let grad = Angle::from_gradians(50.0);
//! ```
//!
//! or you use the [`ToAngle`] trait directly on the value.
//!
//! ```
//! # use angulus::ToAngle;
//! let deg = 90.0.deg();
//! let rad = 3.14.rad();
//! let turns = 0.75.turns();
//! let grad = 50.0.grad();
//! ```
//!
//! ## From angle to value
//!
//! To convert back an angle to a value you can use the `to_*` methods with the desired unit.
//!
//! ```
//! # use angulus::Angle32;
//! let a = Angle32::QUARTER;
//!
//! assert_eq!(a.to_radians(), std::f32::consts::FRAC_PI_2);
//! assert_eq!(a.to_degrees(), 90.0);
//! assert_eq!(a.to_turns(), 0.25);
//! assert_eq!(a.to_gradians(), 100.0);
//! ```
//!
//! ## Display
//!
//! Since [`Angle`] and [`AngleUnbounded`] are unit agnotic they didn't implement the [`Display`][std::fmt::Display] trait.
//! But you can use one of the unit wrapper from [the units module][units] to specify a unit.
//!
//! ## Crate features
//!
//! - `std`: by default angulus links to the standard library. Disable this feature to remove this dependency and be able to use angulus in `#![no_std]` crates.
//! - `libm`: use the [libm crate](https://docs.rs/libm/latest/libm/) for the math methods (sin, cos, tan) when `std` is disabled.
//! - `serde`: enable serialization and deserialization with the [serde crate](https://docs.rs/serde/latest/serde/).
//! - `rand`: enable generation of random angle with the [rand crate](https://docs.rs/rand/latest/rand/).

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "serde")]
pub mod serde;

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
#[cfg(all(
    doctest,
    any(feature = "std", feature = "libm") // Readme uses math methods.
))]
pub struct ReadmeDoctests;

/// Type alias for [`Angle::<f32>`].
pub type Angle32 = Angle<f32>;

/// Type alias for [`Angle::<f64>`].
pub type Angle64 = Angle<f64>;

/// Type alias for [`AngleUnbounded::<f32>`].
pub type AngleUnbounded32 = AngleUnbounded<f32>;

/// Type alias for [`AngleUnbounded::<f64>`].
pub type AngleUnbounded64 = AngleUnbounded<f64>;

/// Re-exports the most important elements of the crate.
///
/// ## Usage
/// ```
/// use angulus::prelude::*;
/// ```
pub mod prelude {
    pub use crate::units::*;
    pub use crate::{
        Angle, Angle32, Angle64, AngleUnbounded, AngleUnbounded32, AngleUnbounded64, ToAngle,
    };
}
