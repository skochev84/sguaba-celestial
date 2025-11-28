# sguaba-celestial

Celestial reference frames for the [sguaba](https://github.com/helsing-ai/sguaba) coordinate system library.

This crate provides celestial coordinate systems and transformations for space applications, extracted as a standalone package from the main sguaba library.

## Features

- **Celestial Reference Frames**: ICRS, GCRF, EME2000, Ecliptic, and Moon-Centered Inertial (MCI)
- **High-Precision Transforms**: IAU 2006/2000A precession with sub-arcsecond accuracy
- **Velocity Transformations**: Transform velocity vectors between reference frames
- **Time Scales**: UTC, TAI, TT, TDB conversions for astronomical calculations
- **Orbital Mechanics**: Keplerian elements, state vectors, TLE parsing
- **Spherical Coordinates**: Right Ascension/Declination conversions
- **Builder Patterns**: Type-safe construction via extension traits

## Coordinate Systems

### Inertial Frames

- **ICRS** (International Celestial Reference System): Primary Earth-centered inertial frame
- **GCRF** (Geocentric Celestial Reference Frame): Equivalent to ICRS, common in satellite operations
- **EME2000** (Earth Mean Equator and Equinox of J2000): J2000 epoch-based frame
- **MCI** (Moon-Centered Inertial): Lunar missions and Moon-relative coordinates

### Additional Systems

- **Ecliptic**: Solar system dynamics, planetary missions
- **ECEF** (via sguaba): Earth-fixed frame for ground station tracking

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
sguaba-celestial = "0.1.0"
```

### Builder Pattern

```rust
use sguaba_celestial::{Icrs, IcrsCoordinateExt, builder};
use sguaba::Coordinate;
use uom::si::f64::Length;
use uom::si::length::kilometer;

// Build coordinates using extension trait
let position: Coordinate<Icrs> = IcrsCoordinateExt::build(
    builder::icrs::Components {
        x: Length::new::<kilometer>(7000.0),
        y: Length::new::<kilometer>(0.0),
        z: Length::new::<kilometer>(0.0),
    }.into()
);
```

### Transform between frames

```rust
use sguaba_celestial::{Icrs, transforms};
use sguaba::systems::Ecef;
use chrono::Utc;

// Create transform from ICRS to ECEF at current time
let transform = transforms::icrs_to_ecef_at(Utc::now());
// Apply: let position_ecef = transform.transform(position_icrs);
```

### Velocity Transformations

```rust
use sguaba_celestial::{Icrs, VelocityTransformExt, transforms};
use sguaba::systems::Ecef;
use chrono::Utc;

// Transform velocity vectors between frames
let transform = transforms::icrs_to_ecef_at(Utc::now());
let velocity_icrs = [7500.0, 0.0, 0.0]; // m/s in ICRS
let velocity_ecef = transform.transform_velocity(position_icrs, velocity_icrs);
```

### Right Ascension/Declination

```rust
use sguaba_celestial::{Icrs, IcrsCoordinateExt};
use sguaba::Coordinate;
use uom::si::f64::{Angle, Length};
use uom::si::angle::degree;
use uom::si::length::kilometer;

// Create from RA/Dec
let position = Coordinate::<Icrs>::from_ra_dec(
    Angle::new::<degree>(45.0),   // Right Ascension
    Angle::new::<degree>(30.0),   // Declination
    Length::new::<kilometer>(7000.0),
);

// Convert back to spherical
let (ra, dec, distance) = position.to_spherical_celestial();
```

### Keplerian Orbital Elements

```rust
use sguaba_celestial::{KeplerianElements, Icrs};
use uom::si::f64::{Angle, Length};
use uom::si::angle::degree;
use uom::si::length::kilometer;

let elements = KeplerianElements::<Icrs> {
    semi_major_axis: Length::new::<kilometer>(7000.0),
    eccentricity: 0.001,
    inclination: Angle::new::<degree>(51.6),
    right_ascension: Angle::new::<degree>(0.0),
    argument_of_periapsis: Angle::new::<degree>(0.0),
    true_anomaly: Angle::new::<degree>(0.0),
};

let state_vector = elements.to_state_vector();
```

### TLE Parsing

```rust
use sguaba_celestial::TleElements;

let tle1 = "1 25544U 98067A   21275.52119560  .00016717  00000-0  10270-3 0  9005";
let tle2 = "2 25544  51.6442 247.4627 0003572  69.9862 290.1574 15.48919393309738";

let tle = TleElements::from_lines("ISS", tle1, tle2).unwrap();
let state = tle.propagate_to_epoch();
```

## Transform Accuracy

For the epoch range 2020-2050:

- **ICRS ↔ ECEF**: < 30 milliarcseconds using IAU 2006/2000A precession + ERA
- **MCI ↔ ICRS**: Arcsecond-level using IAU 2009 lunar orientation constants

## Extension Traits

Due to Rust's orphan rules, methods on coordinate types are provided via extension traits:

```rust
use sguaba_celestial::{IcrsCoordinateExt, MciCoordinateExt, GcrfCoordinateExt};

// Coordinate construction and conversion methods:
// - IcrsCoordinateExt::build() - Builder pattern for ICRS coordinates
// - Coordinate::<Icrs>::from_ra_dec() - RA/Dec to Cartesian
// - coordinate.to_spherical_celestial() - Cartesian to RA/Dec
```

```rust
use sguaba_celestial::VelocityTransformExt;

// Velocity transformation methods:
// - transform.transform_velocity(position, velocity) - Transform velocity vectors
```

## Limitations

- Polar motion not included (returns identity)
- UT1-UTC correction not applied (UTC treated as UT1)
- Lunar libration not modeled in MCI frame
- Nutation corrections available but not enabled by default

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

This crate is extending [sguaba](https://github.com/helsing-ai/sguaba). Please direct contributions and issues to the main repository.
