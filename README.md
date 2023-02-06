# angulus

[![crates.io](https://img.shields.io/crates/v/angulus)](https://crates.io/crates/angulus)
[![docs.rs](https://docs.rs/angulus/badge.svg)](https://docs.rs/angulus)

Unit agnostic angle.

## What problem does it solve ?

Using simple floating point numbers to store an angle value is error-prone : you may add two angle with one in radians and the second in degrees or you may try to compute the cosine of a value in degrees and get an unexpected result.

`angulus` provides a type that represent an angle with no specific unit.

## Example

```rust
use angulus::*;

// Create an angle of 90°.
let alpha = 90.0_f32.deg();

// Create an angle of π/4 rad (45°).
let beta = Angle::RAD_FRAC_PI_4;

// Add the two angle without worrying about units.
let gamma = alpha + beta;

// Print the result.
println!(
    "The cosine of {} is {}",
    units::Degrees(gamma), // The angle is wrapped to display the value in degrees.
    gamma.cos()            // Compute the cosine without worrying about units.
);

// Output : The cosine of 135° is -0.70710677
```

## Features

- `serde` : Serialization/deserialization support via serde.

## License

Licensed under either of the following, at your choice:

- [Apache License, Version 2.0](https://github.com/tguichaoua/angulus/blob/main/LICENSE-APACHE), or
- [MIT license](https://github.com/tguichaoua/angulus/blob/main/LICENSE-MIT)

Unless explicitly stated otherwise, any contribution intentionally submitted
for inclusion in this crate, as defined in the Apache-2.0 license, shall
be dual-licensed as above, without any additional terms or conditions.
