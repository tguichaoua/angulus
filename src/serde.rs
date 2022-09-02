use serde::{Deserialize, Serialize};

use crate::{
    units::{Degrees, Radians, Turns},
    utility::AngleConvertion,
    Angle, Num, UnboundedAngle,
};

//-------------------------------------------------------------------

impl<N: Serialize> Serialize for Angle<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.radians.serialize(serializer)
    }
}

impl<'de, N: Num + Deserialize<'de>> Deserialize<'de> for Angle<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let radians = N::deserialize(deserializer)?;
        Ok(Angle::from_radians(radians))
    }
}

//-------------------------------------------------------------------

impl<N: Serialize> Serialize for UnboundedAngle<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.radians.serialize(serializer)
    }
}

impl<'de, N: Deserialize<'de>> Deserialize<'de> for UnboundedAngle<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let radians = N::deserialize(deserializer)?;
        Ok(UnboundedAngle::from_radians(radians))
    }
}

//-------------------------------------------------------------------

impl<A: AngleConvertion> Serialize for Radians<A>
where
    A::N: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_radians().serialize(serializer)
    }
}

impl<'de, A: AngleConvertion> Deserialize<'de> for Radians<A>
where
    A::N: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let radians = A::N::deserialize(deserializer)?;
        Ok(Radians(A::from_radians(radians)))
    }
}

//-------------------------------------------------------------------

impl<A: AngleConvertion> Serialize for Degrees<A>
where
    A::N: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_degrees().serialize(serializer)
    }
}

impl<'de, A: AngleConvertion> Deserialize<'de> for Degrees<A>
where
    A::N: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let degrees = A::N::deserialize(deserializer)?;
        Ok(Degrees(A::from_degrees(degrees)))
    }
}

//-------------------------------------------------------------------

impl<A: AngleConvertion> Serialize for Turns<A>
where
    A::N: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_turns().serialize(serializer)
    }
}

impl<'de, A: AngleConvertion> Deserialize<'de> for Turns<A>
where
    A::N: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let turns = A::N::deserialize(deserializer)?;
        Ok(Turns(A::from_turns(turns)))
    }
}

//-------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;
    use serde::{Deserialize, Serialize};

    use crate::{
        units::{Degrees, Radians, Turns},
        Angle, ToAngle, UnboundedAngle,
    };

    const TOLERANCE: f32 = 0.00001;

    #[test]
    fn should_serialize() {
        #[derive(Serialize)]
        struct WithAngle {
            angle: Angle<f32>,
            rad: Radians<Angle<f32>>,
            deg: Degrees<Angle<f32>>,
            tr: Turns<UnboundedAngle<f32>>,
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
            tr: Turns<UnboundedAngle<f32>>,
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
