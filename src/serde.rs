use serde::{Deserialize, Serialize};

use crate::{
    float::Float,
    units::{Degrees, Gradians, Radians, Turns},
    Angle, AngleUnbounded,
};

//-------------------------------------------------------------------

macro_rules! impl_angle {
    ($angle:ident) => {
        impl<F: Copy + Serialize> Serialize for $angle<F> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_radians().serialize(serializer)
            }
        }

        impl<'de, F: Float + Deserialize<'de>> Deserialize<'de> for $angle<F> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let radians = Deserialize::deserialize(deserializer)?;
                Ok($angle::from_radians(radians))
            }
        }
    };
}

impl_angle!(Angle);
impl_angle!(AngleUnbounded);

//-------------------------------------------------------------------

macro_rules! unit_impl {
    ($unit:ident) => {
        impl<F: Float + Serialize> Serialize for $unit<Angle<F>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_value().serialize(serializer)
            }
        }

        impl<F: Float + Serialize> Serialize for $unit<AngleUnbounded<F>> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_value().serialize(serializer)
            }
        }

        impl<'de, F: Float + Deserialize<'de>> Deserialize<'de> for $unit<Angle<F>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = Deserialize::deserialize(deserializer)?;
                Ok(Self::from_value(value))
            }
        }

        impl<'de, F: Float + Deserialize<'de>> Deserialize<'de> for $unit<AngleUnbounded<F>> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = Deserialize::deserialize(deserializer)?;
                Ok(Self::from_value(value))
            }
        }
    };
}

unit_impl!(Radians);
unit_impl!(Degrees);
unit_impl!(Turns);
unit_impl!(Gradians);

//-------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;
    use serde::{Deserialize, Serialize};

    use crate::{
        units::{Degrees, Gradians, Radians, Turns},
        Angle, AngleUnbounded, ToAngle,
    };

    const TOLERANCE: f32 = 0.00001;

    #[test]
    fn should_serialize() {
        #[derive(Serialize)]
        struct WithAngle {
            angle: Angle<f32>,
            rad: Radians<Angle<f32>>,
            deg: Degrees<Angle<f32>>,
            tr: Turns<AngleUnbounded<f32>>,
            g: Gradians<AngleUnbounded<f32>>,
        }

        #[derive(Deserialize)]
        struct WithoutAngle {
            angle: f32,
            rad: f32,
            deg: f32,
            tr: f32,
            g: f32,
        }

        let before = WithAngle {
            angle: 90.0.deg(),
            rad: Radians(0.5.turns()),
            deg: Degrees(90.0.deg()),
            tr: Turns(270.0.deg_unbounded()),
            g: Gradians(AngleUnbounded::TWELFTH),
        };
        let json = serde_json::to_string(&before).unwrap();
        let after: WithoutAngle = serde_json::from_str(&json).unwrap();

        assert_float_eq!(before.angle.to_radians(), after.angle, abs <= TOLERANCE);
        assert_float_eq!(before.rad.0.to_radians(), after.rad, abs <= TOLERANCE);
        assert_float_eq!(before.deg.0.to_degrees(), after.deg, abs <= TOLERANCE);
        assert_float_eq!(before.tr.0.to_turns(), after.tr, abs <= TOLERANCE);
        assert_float_eq!(before.g.0.to_gradians(), after.g, abs <= TOLERANCE);
    }

    #[test]
    fn should_deserialize() {
        #[derive(Deserialize)]
        struct Foo {
            angle: Angle<f32>,
            rad: Radians<Angle<f32>>,
            deg: Degrees<Angle<f32>>,
            tr: Turns<AngleUnbounded<f32>>,
            g: Gradians<AngleUnbounded<f32>>,
        }

        let json = serde_json::json!(
            {
                "angle":1.0,
                "rad":0.5,
                "deg":270.0,
                "tr":50.0,
                "g": 150,
            }
        );

        assert_eq!(Angle::from_degrees(270.0).to_degrees(), -90.0);

        let foo: Foo = serde_json::from_value(json).unwrap();

        assert_float_eq!(foo.angle.to_radians(), 1.0, abs <= TOLERANCE);
        assert_float_eq!(foo.rad.0.to_radians(), 0.5, abs <= TOLERANCE);
        assert_float_eq!(foo.deg.0.to_degrees(), -90.0, abs <= TOLERANCE);
        assert_float_eq!(foo.tr.0.to_turns(), 50.0, abs <= TOLERANCE);
        assert_float_eq!(foo.g.0.to_gradians(), 150.0, abs <= TOLERANCE);
    }
}
