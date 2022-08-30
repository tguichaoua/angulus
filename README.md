# angulus

[![crates.io](https://img.shields.io/crates/v/angulus)](https://crates.io/crates/angulus)
[![docs.rs](https://docs.rs/angulus/badge.svg)](https://docs.rs/angulus)

Wrapper for angle with unit convertion.

## Features

- `serde` : Serialization/deserialization support via serde.

## Example

```rs
use angulus::*;

fn main() {
    let a = 90.0.deg();
    let b = Angle::FULL + a;
    let c = b.main_angle();
    let d = a.to_radians();

    // Unit convetion
    let rad = Angle::from_radians(std::f32::consts::FRAC_PI_2);
    let deg = rad.to_degrees();
    let abs_difference = (deg.to_value() - 90.0).abs();
    assert!(abs_difference <= f32::EPSILON);

    // Calculate the main angle value
    let main_value = 450.0.deg().main_angle().to_value();
    let abs_difference = (main_value - 90.0).abs();
    assert!(abs_difference <= f32::EPSILON);

    // Formating with unit symbole
    assert_eq!(format!("{}", 90.0.deg()), "90°");
    assert_eq!(format!("{}", Angle::QUARTER), "1.5707963267948966 rad");

    // sin, cos, tan
    let deg = 90.0.deg();
    let rad = std::f32::consts::FRAC_PI_2.rad();

    let abs_difference = (deg.sin() - 1.0).abs();
    assert!(abs_difference <= f32::EPSILON);

    let abs_difference = (rad.sin() - 1.0).abs();
    assert!(abs_difference <= f32::EPSILON);
}
```

## License

Licensed under either of the following, at your choice:

- [Apache License, Version 2.0](https://github.com/tguichaoua/angulus/blob/main/LICENSE-APACHE), or
- [MIT license](https://github.com/tguichaoua/angulus/blob/main/LICENSE-MIT)

Unless explicitly stated otherwise, any contribution intentionally submitted
for inclusion in this crate, as defined in the Apache-2.0 license, shall
be dual-licensed as above, without any additional terms or conditions.
