//! This module provides wrappers to "colorize" an angle with a specific unit.
//!
//! ## Display
//!
//! Because angles are unit agnostic they cannot implement the [`Display`] trait.
//!
//! But unit wrappers implement the [`Display`] trait.
//! The value is displayed by writing the angle value in the desired unit followed by the unit symbol.
//!
//! For [`Angle`], the displayed value is in [the main range](Angle#the-main-range).
//!
//! ```
//! # use angulus::{Angle, ToAngle, units::{Degrees, Radians, Turns, Gradians}};
//! let angle = 90.0_f32.deg();
//!
//! assert_eq!(format!("{}", Radians(angle)), "1.5707964 rad");
//! assert_eq!(format!("{}", Degrees(angle)), "90°");
//! assert_eq!(format!("{}", Turns(angle)), "0.25 tr");
//! assert_eq!(format!("{}", Gradians(angle)), "100g");
//! ```

use core::fmt::Display;

use crate::float::Float;
use crate::{Angle, AngleUnbounded};

macro_rules! unit {
    (
        $Unit:ident, $doc:expr, $to_method:ident, $from_method:ident, $format:expr
    ) => {
        /// Unit wrapper to "colorize" an angle in
        #[doc = $doc]
        ///
        /// See the [module level documentation][self] for more details.
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct $Unit<A>(pub A);

        impl<F: Float> $Unit<Angle<F>> {
            /// The value of the angle in
            #[doc = $doc]
            ///
            /// The value is in [the main range](Angle#the-main-range).
            #[inline]
            pub fn to_value(self) -> F {
                self.0.$to_method()
            }

            /// Create an new instance from a value in
            #[doc = $doc]
            #[inline]
            pub fn from_value(x: F) -> Self {
                Self(Angle::$from_method(x))
            }
        }

        impl<F: Float> $Unit<AngleUnbounded<F>> {
            /// The value of the angle in
            #[doc = $doc]
            #[inline]
            pub fn to_value(self) -> F {
                self.0.$to_method()
            }

            /// Create an new instance from a value in
            #[doc = $doc]
            #[inline]
            pub fn from_value(x: F) -> Self {
                Self(AngleUnbounded::$from_method(x))
            }
        }

        impl<A> From<A> for $Unit<A> {
            #[inline]
            fn from(x: A) -> Self {
                Self(x)
            }
        }

        impl<F: Float + Display> Display for $Unit<Angle<F>> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $format, self.to_value())
            }
        }

        impl<F: Float + Display> Display for $Unit<AngleUnbounded<F>> {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $format, self.to_value())
            }
        }
    };
}

unit!(Radians, "radians.", to_radians, from_radians, "{} rad");
unit!(Degrees, "degrees.", to_degrees, from_degrees, "{}°");
unit!(Turns, "turns.", to_turns, from_turns, "{} tr");
unit!(Gradians, "gradians.", to_gradians, from_gradians, "{}g");
