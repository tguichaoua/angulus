//! Wrapper for angle with unit convertion.
//!
//! ## Overview
//!
//! The main type of this crate is [`Angle<N, U>`].
//!
//! Where `N` is the numeric type used to store the value of the angle.
//! It can be either `f32` or `f64`.
//!
//! The second generic parameter define the unit of the angle.
//! It can be one of the following :
//! - [`Radians`]
//! - [`Degrees`]
//! - [`Turns`]
//!
//! ### Create an angle
//! An [`Angle`] can be crated with the [`ToAngle`] helper trait.
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
//! ### Convert into another unit
//!
//! It's possible to convert an angle from a unit to another via the `to_*` methods.
//!
//! ```
//! # use angulus::*;
//! # fn main() {
//! let rad = 90.0.deg().to_radians();
//! let deg = 0.5.turns().to_degrees();
//! let turns = 0.75.rad().to_turns();
//! # }
//! ```
//!
//! ### The main angle
//!
//! Because angle represent a position on a circle, two different value can
//! be the same angle (e.g. `90°` and `-270°`).
//!
//! The main angle is the value of an angle in the range `[0; 360)` (or equivalent for
//! the different units).
//!
//! [`MainAngle`] ensure the value is in the main range.
//!
//! It can be computed with the `main_angle` method.
//!
//! ```
//! # use angulus::*;
//! # fn main() {
//! let a = 90.0.deg().main_value();
//! let b = (-270.0).deg().main_value();
//!
//! let abs_difference = (a.to_value() - b.to_value()).abs();
//! assert!(abs_difference <= f32::EPSILON);
//! # }
//! ```
//!
//! ## Example
//!
//! ```
//! use angulus::*;
//!
//! # fn main() {
//! // Extention function to convert numerical value to angle.
//! let deg = 90.0f32.deg();
//!
//! // Convertion from degrees to radians.
//! let rad = deg.to_radians();
//!
//! // Correctly handle sin, cos, and tan whatever the unit.
//! let abs_difference = (deg.sin() - 1.0).abs();
//! assert!(abs_difference <= f32::EPSILON);
//!
//! let abs_difference = (rad.sin() - 1.0).abs();
//! assert!(abs_difference <= f32::EPSILON);
//! # }
//! ```
//! ## [Serde](https://crates.io/crates/serde) support
//!
//! Serialization/deserialization support via serde with the `serde` feature.
//!
//! ```
//! use angulus::*;
//! use serde::{Serialize, Deserialize};
//!
//! # fn main() {
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!     rad: Angle<f32>,
//!     deg: Angle<f32, Degrees>,
//!     turns: Angle<f32, Turns>,
//! }
//!
//! let s = r#"
//! {
//!     "rad": 3.141592654,
//!     "deg": 180.0,
//!     "turns": 0.5
//! }
//! "#;
//!
//! let foo: Foo = serde_json::from_str(s).unwrap();
//!
//! let abs_difference = (foo.rad.cos() - (-1.0)).abs();
//! assert!(abs_difference <= f32::EPSILON);
//!
//! let abs_difference = (foo.deg.cos() - (-1.0)).abs();
//! assert!(abs_difference <= f32::EPSILON);
//!
//! let abs_difference = (foo.turns.cos() - (-1.0)).abs();
//! assert!(abs_difference <= f32::EPSILON);
//! # }
//! ```

#[cfg(feature = "serde")]
mod serde;

mod angle;
mod helpers;
mod main_angle;
mod num;
mod unit;

pub use angle::Angle;
pub use helpers::*;
pub use main_angle::MainAngle;
pub use num::Num;
pub use unit::*;
