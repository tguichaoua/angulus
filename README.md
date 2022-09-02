# angulus

[![crates.io](https://img.shields.io/crates/v/angulus)](https://crates.io/crates/angulus)
[![docs.rs](https://docs.rs/angulus/badge.svg)](https://docs.rs/angulus)

Provides types for angle manipulation.

## Features

- `serde` : Serialization/deserialization support via serde.

## Example

```rust
use angulus::{*, units::*};

fn main() {
    let alpha = Angle::DEG_90;
    let beta = Angle::RAD_FRAC_PI_4;
    let gamma: Angle<f32> = alpha + beta;

    // in radians : 1.5707964 rad + 0.7853982 rad = 2.3561945 rad
    println!(
        "in radians : {} + {} = {}",
        Radians(alpha),
        Radians(beta),
        Radians(gamma),
    );

    // in degrees : 90° + 45° = 135°
    println!(
        "in degrees : {} + {} = {}",
        Degrees(alpha),
        Degrees(beta),
        Degrees(gamma),
    );
}
```

## License

Licensed under either of the following, at your choice:

- [Apache License, Version 2.0](https://github.com/tguichaoua/angulus/blob/main/LICENSE-APACHE), or
- [MIT license](https://github.com/tguichaoua/angulus/blob/main/LICENSE-MIT)

Unless explicitly stated otherwise, any contribution intentionally submitted
for inclusion in this crate, as defined in the Apache-2.0 license, shall
be dual-licensed as above, without any additional terms or conditions.
