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

use std::fmt::Display;

use crate::{float::Float, Angle, AngleUnbounded};

macro_rules! unit {
    (
        $vis:vis $name:ident, $unit:expr, $to_method:ident, $from_method:ident, $format:expr
    ) => {
        /// Unit wrapper to "colorize" an angle in
        #[doc = $unit]
        ///
        /// See the [module level documentation][self] for more details.
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        $vis struct $name<A>(pub A);

        impl<F: Float> $name<Angle<F>> {
            /// The value of the angle in
            #[doc = $unit]
            #[inline]
            pub fn to_value(self) -> F {
                self.0.$to_method()
            }

            /// Create an new instance from a value in
            #[doc = $unit]
            #[inline]
            pub fn from_value(x: F) -> Self {
                Self(Angle::$from_method(x))
            }
        }

        impl<F: Float> $name<AngleUnbounded<F>> {
            /// The value of the angle in
            #[doc = $unit]
            #[inline]
            pub fn to_value(self) -> F {
                self.0.$to_method()
            }

            /// Create an new instance from a value in
            #[doc = $unit]
            #[inline]
            pub fn from_value(x: F) -> Self {
                Self(AngleUnbounded::$from_method(x))
            }
        }

        impl<A> From<A> for $name<A> {
            #[inline]
            fn from(x: A) -> Self {
                Self(x)
            }
        }

        impl<F: Float + Display> Display for $name<Angle<F>>
        {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, self.to_value())
            }
        }

        impl<F: Float + Display> Display for $name<AngleUnbounded<F>>
        {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format, self.to_value())
            }
        }

    };
}

unit!(pub Radians, "radians.", to_radians, from_radians, "{} rad");
unit!(pub Degrees, "degrees.", to_degrees, from_degrees, "{}°");
unit!(pub Turns, "turns.", to_turns, from_turns, "{} tr");
unit!(pub Gradians, "gradians.", to_gradians, from_gradians, "{}g");
