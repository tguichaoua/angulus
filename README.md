# angulus

[<img alt="github" src="https://img.shields.io/badge/github-tguichaoua/angulus-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/tguichaoua/angulus)
[<img alt="crates.io" src="https://img.shields.io/crates/v/angulus.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/angulus)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-angulus-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/angulus)
[<img alt="msrv" src="https://img.shields.io/badge/msrv-1.61.0-dea584.svg?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://github.com/rust-lang/rust/releases/tag/1.61.0)

Unit agnostic angle.

## What problem does it solve ?

Using simple floating point numbers to store an angle value is error-prone : you may add two angle with one in radians and the second in degrees or you may try to compute the cosine of a value in degrees and get an unexpected result.

`angulus` provides a type that represent an angle with no specific unit.

## Example

```rust
use angulus::{Angle, ToAngle};
use angulus::units::Degrees;

// Create an angle of 90°.
let alpha = 90.0_f32.deg();

// Create an angle of π/4 rad (45°).
let beta = Angle::RAD_FRAC_PI_4;

// Add the two angle without worrying about units.
let gamma = alpha + beta;

// Print the result.
println!(
    "The cosine of {} is {}",
    Degrees(gamma), // The angle is wrapped to display the value in degrees.
    gamma.cos()     // Compute the cosine without worrying about units.
);

// Output : The cosine of 135° is -0.70710677
```

## Features

- `std`: by default angulus links to the standard library. Disable this feature to remove this dependency and be able to use angulus in `#![no_std]` crates.
- `libm`: use the [libm crate](https://docs.rs/libm/latest/libm/) for the math methods (sin, cos, tan) when `std` is disabled.
- `serde`: enable serialization and deserialization with the [serde crate](https://docs.rs/serde/latest/serde/).
- `rand`: enable generation of random angle with the [rand crate](https://docs.rs/rand/latest/rand/).

## Minimum Supported Rust Version

This crate requires Rust 1.61.0 or later.

## License

Licensed under either of the following, at your choice:

- [Apache License, Version 2.0](https://github.com/tguichaoua/angulus/blob/main/LICENSE-APACHE), or
- [MIT license](https://github.com/tguichaoua/angulus/blob/main/LICENSE-MIT)

Unless explicitly stated otherwise, any contribution intentionally submitted
for inclusion in this crate, as defined in the Apache-2.0 license, shall
be dual-licensed as above, without any additional terms or conditions.
