//! Wrappers to represent an angle with a specific unit.
//!
//! Wrapping an [`Angle`] or an [`AngleUnbounded`] with these wrappers enables [`Display`] capability.
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
        /// Unit wrapper for the
        #[doc = $doc]
        /// unit.
        ///
        /// See the [module level documentation][self] for more details.
        #[derive(Debug, Copy, Clone)]
        #[repr(transparent)]
        pub struct $Unit<A>(pub A);

        impl<F: Float> $Unit<Angle<F>> {
            /// Returns the value of the angle in the
            #[doc = $doc]
            /// unit.
            ///
            /// The value is in [the main range](crate#the-main-range).
            #[inline]
            pub fn to_value(self) -> F {
                self.0.$to_method()
            }

            /// Converts a value in
            #[doc = $doc]
            /// into an angle.
            #[inline]
            pub fn from_value(x: F) -> Self {
                Self(Angle::$from_method(x))
            }
        }

        impl<F: Float> $Unit<AngleUnbounded<F>> {
            /// Returns the value of the angle in the
            #[doc = $doc]
            /// unit.
            #[inline]
            pub fn to_value(self) -> F {
                self.0.$to_method()
            }

            /// Converts a value in
            #[doc = $doc]
            /// into an angle.
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

unit!(Radians, "radian", to_radians, from_radians, "{} rad");
unit!(Degrees, "degree", to_degrees, from_degrees, "{}°");
unit!(Turns, "turn", to_turns, from_turns, "{} tr");
unit!(Gradians, "gradian", to_gradians, from_gradians, "{}g");
