use serde::{Deserialize, Serialize};

use crate::{Angle, Num, UnboundedAngle};

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

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{ToAngle, UnboundedAngle};

    #[test]
    fn should_serialize() {
        #[derive(Serialize)]
        struct Foo {
            angle: UnboundedAngle<f32>,
        }

        let foo = Foo {
            angle: 56.0.rad_unbounded(),
        };
        let json = serde_json::to_string(&foo).unwrap();

        assert_eq!(json, "{\"angle\":56.0");
    }

    #[test]
    fn should_deserialize() {
        #[derive(Deserialize)]
        struct Foo {
            angle: UnboundedAngle<f32>,
        }

        let foo: Foo = serde_json::from_str("{\"angle\":56.0").unwrap();
        assert_eq!(foo.angle, 56.0.rad_unbounded());
    }
}
