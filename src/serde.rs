use serde::{Deserialize, Serialize};

use crate::{
    private::IAngle,
    units::{Degrees, Radians, Turns},
    Angle, AngleUnbounded, Num,
};

//-------------------------------------------------------------------

macro_rules! impl_angle {
    (
        $name:ident
    ) => {
        impl<F: Copy + Serialize> Serialize for $name<F> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.to_radians().serialize(serializer)
            }
        }

        impl<'de, F: Num + Deserialize<'de>> Deserialize<'de> for $name<F> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let radians = Deserialize::deserialize(deserializer)?;
                Ok($name::from_radians(radians))
            }
        }
    };
}

impl_angle!(Angle);
impl_angle!(AngleUnbounded);

//-------------------------------------------------------------------

macro_rules! unit_impl {
    (
        $unit:ident, $to_method:ident, $from_method:ident
    ) => {
        impl<A: IAngle> Serialize for $unit<A>
        where
            A::Num: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                IAngle::$to_method(self.0).serialize(serializer)
            }
        }

        impl<'de, A: IAngle> Deserialize<'de> for $unit<A>
        where
            A::Num: Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = Deserialize::deserialize(deserializer)?;
                Ok($unit(IAngle::$from_method(value)))
            }
        }
    };
}

unit_impl!(Radians, to_radians, from_radians);
unit_impl!(Degrees, to_degrees, from_degrees);
unit_impl!(Turns, to_turns, from_turns);

//-------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;
    use serde::{Deserialize, Serialize};

    use crate::{
        units::{Degrees, Radians, Turns},
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
        }

        #[derive(Deserialize)]
        struct WithoutAngle {
            angle: f32,
            rad: f32,
            deg: f32,
            tr: f32,
        }

        let before = WithAngle {
            angle: 90.0.deg(),
            rad: Radians(0.5.turns()),
            deg: Degrees(90.0.deg()),
            tr: Turns(270.0.deg_unbounded()),
        };
        let json = serde_json::to_string(&before).unwrap();
        let after: WithoutAngle = serde_json::from_str(&json).unwrap();

        assert_float_eq!(before.angle.to_radians(), after.angle, abs <= TOLERANCE);
        assert_float_eq!(before.rad.0.to_radians(), after.rad, abs <= TOLERANCE);
        assert_float_eq!(before.deg.0.to_degrees(), after.deg, abs <= TOLERANCE);
        assert_float_eq!(before.tr.0.to_turns(), after.tr, abs <= TOLERANCE);
    }

    #[test]
    fn should_deserialize() {
        #[derive(Deserialize)]
        struct Foo {
            angle: Angle<f32>,
            rad: Radians<Angle<f32>>,
            deg: Degrees<Angle<f32>>,
            tr: Turns<AngleUnbounded<f32>>,
        }

        let json = serde_json::json!(
            {
                "angle":1.0,
                "rad":0.5,
                "deg":270.0,
                "tr":50.0,
            }
        );

        assert_eq!(Angle::from_degrees(270.0).to_degrees(), -90.0);

        let foo: Foo = serde_json::from_value(json).unwrap();

        assert_float_eq!(foo.angle.to_radians(), 1.0, abs <= TOLERANCE);
        assert_float_eq!(foo.rad.0.to_radians(), 0.5, abs <= TOLERANCE);
        assert_float_eq!(foo.deg.0.to_degrees(), -90.0, abs <= TOLERANCE);
        assert_float_eq!(foo.tr.0.to_turns(), 50.0, abs <= TOLERANCE);
    }
}
