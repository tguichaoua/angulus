//! Provide types for angle manipulation.
//!
//! ## Overview
//!
//! An [`Angle`] is the canonical representation of an angle.
//!
//! For example, `90°` and `-270°` have different value but is the same angle.
//!
//! ```
//! # use angulus::Angle;
//! # fn main() {
//! let a = Angle::from_degrees(90.0);
//! let b = Angle::from_degrees(-270.0);
//!
//! assert_eq!(a.to_degrees(), b.to_degrees());
//! # }
//! ```
//!
//! Conversely [`UnboundedAngle`] represent angle using the provided value.
//!
//! ```
//! # use angulus::UnboundedAngle;
//! # fn main() {
//! let a = UnboundedAngle::from_degrees(90.0);
//! let b = UnboundedAngle::from_degrees(-270.0);
//!
//! assert_ne!(a.to_degrees(), b.to_degrees());
//! # }
//! ```
//!
//! ### Create an angle
//!
//! An [`Angle`] can be created either with the [`ToAngle`] helper trait.
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
//! Or via the `from_*` methods.
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
//! ### Get the angle value
//!
//! The value of the angle can be retrieved into different units using the
//! `to_*` methods.
//!
//! ```
//! # use angulus::*;
//! # fn main() {
//! let rad = 90.0.deg().to_radians(); // convert 90° into radians
//! let deg = 0.5.turns().to_degrees(); // convert 0.5 turns into degrees
//! let turns = 0.75.rad().to_turns(); // convert 0.75 rad into turns
//! # }
//! ```
//!
//! ## [Serde](https://crates.io/crates/serde) support
//!
//! (De)Serialization is supported via serde with the `serde` feature flag.
//!
//! By default angles are (de)serialize from/into radians.
//!
//! To (de)serialize from/into a specific unit see [the units module][units].

#[cfg(feature = "serde")]
mod serde;

mod angle;
mod helpers;
mod num;
mod unbounded;
pub mod units;
mod utility;

pub use angle::Angle;
pub use helpers::*;
pub use num::Num;
pub use unbounded::UnboundedAngle;
