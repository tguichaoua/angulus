use serde::{Deserialize, Serialize};

use crate::{Angle, MainAngle, Num, Unit};

impl<N: Serialize, U> Serialize for Angle<N, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, N: Deserialize<'de>, U> Deserialize<'de> for Angle<N, U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = N::deserialize(deserializer)?;
        Ok(Angle::new(value))
    }
}

impl<N: Serialize, U> Serialize for MainAngle<N, U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.value.serialize(serializer)
    }
}

impl<'de, N: Num + Deserialize<'de>, U: Unit<N>> Deserialize<'de> for MainAngle<N, U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = N::deserialize(deserializer)?;
        Ok(MainAngle::new(value))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{Angle, Degrees, MainAngle, ToAngle};

    #[test]
    fn should_serialize() {
        #[derive(Serialize)]
        struct Foo {
            angle: Angle<f32>,
            main: MainAngle<f32, Degrees>,
        }

        let foo = Foo {
            angle: 56.0.rad(),
            main: 361.0.deg().main_angle(),
        };
        let json = serde_json::to_string(&foo).unwrap();

        assert_eq!(json, "{\"angle\":56.0,\"main\":1.0}");
    }

    #[test]
    fn should_deserialize() {
        #[derive(Deserialize)]
        struct Foo {
            angle: Angle<f32>,
            main: MainAngle<f32, Degrees>,
        }

        let foo: Foo = serde_json::from_str("{\"angle\":56.0,\"main\":361.0}").unwrap();
        assert_eq!(foo.angle, 56.0.rad());
        assert_eq!(foo.main.to_value(), 1.0);
    }
}
