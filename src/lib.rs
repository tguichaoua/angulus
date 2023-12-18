//! This crate provides two types ([`Angle`] and [`AngleUnbounded`]) that both represent an angle value
//! with no specific unit (radian, degree, etc.).
//!
//! They can be used in place of `f32` and `f64` for angle manipulations.
//!
//! # [`Angle`] vs [`AngleUnbounded`]
//!
//! [`Angle`] is a specific point of the circle.
//!
//! ```
//! # use angulus::Angle;
//! let a = Angle::from_degrees(90.0);
//! let b = Angle::from_degrees(450.0);
//!
//! assert_eq!(a, b);
//! ```
//!
//! While [`AngleUnbounded`] preserves the "number of turns".
//!
//! ```
//! # use angulus::AngleUnbounded;
//! let a = AngleUnbounded::from_degrees(90.0);
//! let b = AngleUnbounded::from_degrees(450.0);
//!
//! assert_ne!(a, b);
//! ```
//!
//! # The main range
//!
//! The main range for an angle is :
//!
//! - `(-π, π]` radians
//! - `(-180, 180]` degrees
//! - `(-0.5, 0.5]` turns
//! - `(-200, 200]` gradians
//!
//! # Display
//!
//! Since [`Angle`] and [`AngleUnbounded`] are unit-agnostic, they cannot implement the [`Display`][std::fmt::Display] trait.
//!
//! To display an angle with a specific unit, wrap it in one of the unit struct of [the `units` module][units].
//!
//! # Crate features
//!
//! - `std`: by default angulus links to the standard library. Disable this feature to remove this dependency and be able to use angulus in `#![no_std]` crates.
//! - `libm`: use the [libm crate](https://docs.rs/libm/latest/libm/) for the math methods (sin, cos, tan) when `std` is disabled.
//! - `serde`: enable serialization and deserialization with the [serde crate](https://docs.rs/serde/latest/serde/).
//! - `rand`: enable generation of random angle with the [rand crate](https://docs.rs/rand/latest/rand/).

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
