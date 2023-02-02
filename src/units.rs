//! This module provides wrappers to "colorize" an angle with a specific unit.
//!
//! ## Display
//!
//! Because angles are unit agnostic they cannot implement the [`Display`] trait.
//!
//! But unit wrappers implement the [`Display`] trait.
//! The value is displayed by writting the angle value in the desired unit followed by the unit symbole.
//!
//! ```
//! # use angulus::{Angle, ToAngle, units::{Degrees, Radians, Turns, Gradians}};
//! # fn main() {
//! let angle = 90.0_f32.deg();
//!
//! assert_eq!(format!("{}", Radians(angle)), "1.5707964 rad");
//! assert_eq!(format!("{}", Degrees(angle)), "90°");
//! assert_eq!(format!("{}", Turns(angle)), "0.25 tr");
//! assert_eq!(format!("{}", Gradians(angle)), "100g");
//! # }
//! ```
//!
//! ## (De)Serialization
//!
//! The angle types (de)serialize into/from radians.
//! But unit wrappers will (de)serialize the value into/from the specified unit.
//!
//! ```
//! # use angulus::{Angle, units::{Degrees, Radians, Turns, Gradians}};
//! # use float_eq::assert_float_eq;
//! # use serde::{Serialize, Deserialize};
//! # fn main() {
//! #[derive(Serialize, Deserialize)]
//! struct Foo {
//!     rad: Radians<Angle<f32>>,
//!     deg: Degrees<Angle<f32>>,
//!     tr: Turns<Angle<f32>>,
//!     g: Gradians<Angle<f32>>,
//! }
//!
//! let json = serde_json::json!{
//!     {
//!         "rad": 1.0,
//!         "deg": 90.0,
//!         "tr": 0.5,
//!         "g": 50,
//!     }
//! };
//!
//! let foo: Foo = serde_json::from_value(json).unwrap();
//!
//! assert_float_eq!(foo.rad.0.to_radians(), 1.0, abs <= 0.000001);
//! assert_float_eq!(foo.deg.0.to_degrees(), 90.0, abs <= 0.000001);
//! assert_float_eq!(foo.tr.0.to_turns(), 0.5, abs <= 0.000001);
//! assert_float_eq!(foo.g.0.to_gradians(), 50.0, abs <= 0.000001);
//! # }
//! ```

use std::fmt::Display;

use crate::private::IAngle;

macro_rules! unit {
    (
        $vis:vis $name:ident, $unit:expr, $to_method:ident, $format:expr
    ) => {
        /// Unit wrapper to "colorize" an angle in
        #[doc = $unit]
        ///
        /// See the [module level documentation][self] for more details.
        $vis struct $name<A>(pub A);

        impl<A> From<A> for $name<A> {
            fn from(x: A) -> Self {
                Self(x)
            }
        }

        impl<A: IAngle> Display for $name<A>
        where
            A::Float: Display,
        {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, IAngle::$to_method(self.0))
            }
        }
    };
}

unit!(pub Radians, "radians.", to_radians, "{} rad");
unit!(pub Degrees, "degrees.", to_degrees, "{}°");
unit!(pub Turns, "turns.", to_turns, "{} tr");
unit!(pub Gradians, "gradians.", to_gradians, "{}g");
