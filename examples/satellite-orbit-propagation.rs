//! Satellite orbit propagation example.
//!
//! Demonstrates time-dependent ICRS ↔ ECEF transformations for tracking
//! a satellite in low Earth orbit over multiple time steps.

use chrono::{Duration, Utc};
use sguaba_celestial::{builder::icrs::Components, transforms, Icrs, IcrsCoordinateExt};
use sguaba::Coordinate;
use uom::si::f64::Length;
use uom::si::length::meter;

fn main() {
    println!("=== Satellite Orbit Propagation ===\n");

    // Satellite initial position in ICRS (simplified circular orbit)
    // At ~7000 km from Earth center (LEO altitude ~600 km)
    let initial_position_icrs: Coordinate<Icrs> = IcrsCoordinateExt::build(
        Components {
            x: Length::new::<meter>(7_000_000.0),
            y: Length::new::<meter>(0.0),
            z: Length::new::<meter>(0.0),
        }
        .into(),
    );

    let [x, y, z] = initial_position_icrs.to_cartesian();
    println!("Initial ICRS position: [{:.0}, {:.0}, {:.0}] m",
        x.get::<meter>(),
        y.get::<meter>(),
        z.get::<meter>()
    );

    // Track satellite position over 24 hours
    let start_time = Utc::now();
    let time_steps = vec![0, 6, 12, 18, 24]; // Hours

    println!("\nSatellite position in ECEF over time:");
    println!("{:>6} {:>12} {:>12} {:>12} {:>12}", 
        "Time", "X (km)", "Y (km)", "Z (km)", "Distance (km)");
    println!("{:-<62}", "");

    for &hours in &time_steps {
        let observation_time = start_time + Duration::hours(hours);
        
        // Transform ICRS → ECEF at this time
        let icrs_to_ecef = transforms::icrs_to_ecef_at(observation_time);
        let position_ecef = icrs_to_ecef.transform(initial_position_icrs);

        let distance = position_ecef.distance_from_origin();
        let [x, y, z] = position_ecef.to_cartesian();

        println!("{:>4}h  {:>12.1} {:>12.1} {:>12.1} {:>12.1}",
            hours,
            x.get::<meter>() / 1000.0,
            y.get::<meter>() / 1000.0,
            z.get::<meter>() / 1000.0,
            distance.get::<meter>() / 1000.0
        );
    }

    // Demonstrate roundtrip transformation
    println!("\n=== Roundtrip Verification ===");
    let test_time = start_time;
    let icrs_to_ecef = transforms::icrs_to_ecef_at(test_time);
    let ecef_to_icrs = transforms::ecef_to_icrs_at(test_time);

    let ecef_pos = icrs_to_ecef.transform(initial_position_icrs);
    let icrs_back = ecef_to_icrs.transform(ecef_pos);

    let error = initial_position_icrs.distance_from(&icrs_back);
    println!("Roundtrip error: {:.9} m", error.get::<meter>());

    if error.get::<meter>() < 1e-6 {
        println!("✓ Roundtrip transformation accurate to < 1 μm");
    }

    // Show Earth rotation effect
    println!("\n=== Earth Rotation Effect ===");
    let pos_0h = transforms::icrs_to_ecef_at(start_time)
        .transform(initial_position_icrs);
    let pos_24h = transforms::icrs_to_ecef_at(start_time + Duration::hours(24))
        .transform(initial_position_icrs);

    let rotation_diff = pos_0h.distance_from(&pos_24h);
    println!("Position difference after 24h (same ICRS pos):");
    println!("  {:.1} m (due to Earth rotation)", rotation_diff.get::<meter>());
    println!("\nNote: Same inertial position appears at different");
    println!("      Earth-fixed coordinates as Earth rotates.");
}

