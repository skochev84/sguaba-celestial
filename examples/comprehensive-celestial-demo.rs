//! Comprehensive demonstration of celestial crate improvements.

fn main() {
    use chrono::Utc;
    use sguaba::engineering::{Orientation, Pose};
    use sguaba::Coordinate;
    use sguaba_celestial::builder;
    use sguaba_celestial::{transforms, GcrfCoordinateExt, IcrsCoordinateExt};
    use sguaba_celestial::{validate_epoch, Gcrf, Icrs, KeplerianElements, TleElements};
    use uom::si::angle::degree;
    use uom::si::f64::{Angle, Length};
    use uom::si::length::kilometer;

    println!("=== Celestial Crate Improvements Demo ===\n");

    // 1. BUILDER PATTERN FOR CELESTIAL COORDINATES
    println!("1. Builder Pattern for Celestial Coordinates");
    let sat_position: Coordinate<Icrs> = IcrsCoordinateExt::build(
        builder::icrs::Components {
            x: Length::new::<kilometer>(7000.0),
            y: Length::new::<kilometer>(0.0),
            z: Length::new::<kilometer>(0.0),
        }
        .into(),
    );
    println!(
        "   Satellite at {:.1} km from Earth\n",
        sat_position.distance_from_origin().get::<kilometer>()
    );

    // 2. EQUIVALENT_TO TRAIT - Zero-cost GCRF/ICRS casting
    println!("2. EquivalentTo Trait (GCRF ≡ ICRS)");
    let gcrf_coord: Coordinate<Gcrf> = GcrfCoordinateExt::build(
        builder::gcrf::Components {
            x: Length::new::<kilometer>(7000.0),
            y: Length::new::<kilometer>(0.0),
            z: Length::new::<kilometer>(0.0),
        }
        .into(),
    );
    let icrs_coord: Coordinate<Icrs> = gcrf_coord.cast();
    println!(
        "   GCRF → ICRS (zero-cost): {:.1} km\n",
        icrs_coord.distance_from_origin().get::<kilometer>()
    );

    // 3. RA/DEC CELESTIAL COORDINATES
    println!("3. RA/Dec Celestial Coordinates");
    let star_pos = <Coordinate<Icrs>>::from_ra_dec(
        Angle::new::<degree>(45.0),
        Angle::new::<degree>(30.0),
        Length::new::<kilometer>(1000.0),
    );
    let (ra, dec, dist) = star_pos.to_spherical_celestial();
    println!(
        "   Star: RA={:.1}°, Dec={:.1}°, Dist={:.0} km\n",
        ra.get::<degree>(),
        dec.get::<degree>(),
        dist.get::<kilometer>()
    );

    // 4. ENGINEERING MODULE INTEGRATION
    println!("4. Engineering Module Integration");
    let _spacecraft_pose = Pose::new(
        sat_position,
        Orientation::tait_bryan_builder()
            .yaw(Angle::new::<degree>(45.0))
            .pitch(Angle::new::<degree>(0.0))
            .roll(Angle::new::<degree>(0.0))
            .build(),
    );
    println!("   Spacecraft pose with 45° yaw\n");

    // 5. TLE PARSING AND PROPAGATION
    println!("5. TLE Support (ISS Two-Line Elements)");
    let line1 = "1 25544U 98067A   20206.18539600  .00001406  00000-0  33518-4 0  9992";
    let line2 = "2 25544  51.6461 339.8014 0001473  94.8340 265.2864 15.49309432236008";

    match TleElements::from_lines(line1, line2) {
        Ok(tle) => {
            println!(
                "   Catalog #{}, Inclination {:.1}°",
                tle.catalog_number(),
                tle.inclination().get::<degree>()
            );
            let kep = tle.to_keplerian();
            println!(
                "   Semi-major axis: {:.1} km\n",
                kep.semi_major_axis.get::<kilometer>()
            );
        }
        Err(e) => println!("   Error: {:?}\n", e),
    }

    // 6. TIME-DEPENDENT TRANSFORMS
    println!("6. Time-Dependent ICRS ↔ ECEF");
    let epoch = Utc::now();
    let transform = transforms::icrs_to_ecef_at(epoch);
    let ecef_pos = transform.transform(sat_position);
    println!(
        "   ICRS → ECEF: {:.1} km\n",
        ecef_pos.distance_from_origin().get::<kilometer>()
    );

    // 7. KEPLERIAN ORBITAL ELEMENTS
    println!("7. Keplerian Orbital Elements");
    let _elements = KeplerianElements::new(
        Length::new::<kilometer>(7000.0), // semi-major axis
        0.001,                            // eccentricity
        Angle::new::<degree>(51.6),       // inclination
        Angle::new::<degree>(0.0),        // RAAN
        Angle::new::<degree>(0.0),        // arg of perigee
        Angle::new::<degree>(0.0),        // mean anomaly
    );
    println!("   Circular LEO orbit defined\n");

    // 8. EPOCH VALIDATION
    println!("8. Epoch Validation (1900-2100)");
    match validate_epoch(Utc::now()) {
        Ok(_) => println!("   Current epoch valid ✓\n"),
        Err(e) => println!("   Error: {:?}\n", e),
    }

    // 9. TIME SCALE CONVERSIONS
    println!("9. Time Scale Conversions");
    use sguaba_celestial::{utc_to_tai, utc_to_tdb, utc_to_tt};
    let utc_now = Utc::now();
    let tai_now = utc_to_tai(utc_now);
    let tt_now = utc_to_tt(utc_now);
    let tdb_now = utc_to_tdb(utc_now);
    println!("   TAI: JD {:.6}", tai_now);
    println!("   TT:  JD {:.6}", tt_now);
    println!("   TDB: JD {:.6}\n", tdb_now);

    println!("=== Demo Complete! ===");
}
