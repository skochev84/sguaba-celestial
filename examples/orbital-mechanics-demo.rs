//! Comprehensive orbital mechanics example.
//!
//! Demonstrates the full suite of celestial coordinate features:
//! - Keplerian orbital elements
//! - RA/Dec coordinate conversions
//! - Time-tagged coordinates
//! - Velocity transformations
//! - Multiple time scales
//! - Orbit propagation

use chrono::{Duration, Utc};
use sguaba_celestial::{Icrs, IcrsCoordinateExt, KeplerianElements, TimedCoordinate, VelocityTransformExt};
use sguaba_celestial::transforms;
use sguaba::Coordinate;
use uom::si::angle::degree;
use uom::si::f64::{Angle, Length};
use uom::si::length::{kilometer, meter};

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     🛰️  ORBITAL MECHANICS DEMONSTRATION 🌍                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Define a LEO satellite orbit using Keplerian elements
    println!("━━━ ORBITAL ELEMENTS ━━━\n");
    
    let elements = KeplerianElements::new(
        Length::new::<kilometer>(7000.0),      // Semi-major axis (600 km altitude)
        0.001,                                   // Nearly circular (e ≈ 0)
        Angle::new::<degree>(51.6),            // ISS-like inclination
        Angle::new::<degree>(45.0),            // RAAN
        Angle::new::<degree>(0.0),             // Argument of periapsis
        Angle::new::<degree>(0.0),             // True anomaly (at periapsis)
    );

    println!("  Semi-major axis:  {:.1} km", elements.semi_major_axis.get::<kilometer>());
    println!("  Eccentricity:     {:.4}", elements.eccentricity);
    println!("  Inclination:      {:.2}°", elements.inclination.get::<degree>());
    println!("  RAAN:             {:.2}°", elements.raan.get::<degree>());
    println!("  Arg of periapsis: {:.2}°", elements.argument_of_periapsis.get::<degree>());
    println!("  True anomaly:     {:.2}°", elements.true_anomaly.get::<degree>());

    // Convert to state vectors
    println!("\n━━━ STATE VECTORS (ICRS) ━━━\n");
    
    let (position, velocity) = elements.to_state_vectors();
    let [x, y, z] = position.to_cartesian();
    
    println!("  Position:");
    println!("    X: {:>10.2} km", x.get::<kilometer>());
    println!("    Y: {:>10.2} km", y.get::<kilometer>());
    println!("    Z: {:>10.2} km", z.get::<kilometer>());
    println!("    Magnitude: {:.2} km", position.distance_from_origin().get::<kilometer>());
    
    println!("\n  Velocity:");
    println!("    Vx: {:>8.3} km/s", velocity[0] / 1000.0);
    println!("    Vy: {:>8.3} km/s", velocity[1] / 1000.0);
    println!("    Vz: {:>8.3} km/s", velocity[2] / 1000.0);
    let v_mag = (velocity[0].powi(2) + velocity[1].powi(2) + velocity[2].powi(2)).sqrt();
    println!("    Speed: {:.3} km/s", v_mag / 1000.0);

    // Convert to RA/Dec representation
    println!("\n━━━ CELESTIAL COORDINATES ━━━\n");
    
    let (ra, dec, dist) = position.to_spherical_celestial();
    
    println!("  Right Ascension: {:.4}° ({:.4}h)", 
        ra.get::<degree>(), 
        ra.get::<degree>() / 15.0  // Convert to hours
    );
    println!("  Declination:     {:.4}°", dec.get::<degree>());
    println!("  Distance:        {:.2} km", dist.get::<kilometer>());
    
    // Verify roundtrip conversion
    let pos_from_radec = <Coordinate<Icrs>>::from_ra_dec(ra, dec, dist);
    let error = position.distance_from(&pos_from_radec);
    println!("\n  Roundtrip error: {:.3e} m", error.get::<meter>());

    // Time-tagged coordinates
    println!("\n━━━ TIME-TAGGED TRACKING ━━━\n");
    
    let epoch = Utc::now();
    let timed_pos = TimedCoordinate::new(position, epoch);
    
    println!("  Epoch: {}", timed_pos.epoch().format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  Position at epoch: ({:.1}, {:.1}, {:.1}) km",
        x.get::<kilometer>(),
        y.get::<kilometer>(),
        z.get::<kilometer>()
    );

    // Orbit propagation
    println!("\n━━━ ORBIT PROPAGATION ━━━\n");
    
    let times = vec![0, 45, 90, 135, 180]; // Minutes
    
    println!("  Time    RA        Dec      Altitude");
    println!("  ────────────────────────────────────");
    
    for &minutes in &times {
        let target_epoch = epoch + Duration::minutes(minutes);
        let propagated = elements.propagate_to(target_epoch, epoch);
        let (pos, _vel) = propagated.to_state_vectors();
        let (ra, dec, _dist) = pos.to_spherical_celestial();
        let altitude = pos.distance_from_origin().get::<kilometer>() - 6378.137;
        
        println!("  {:3}min  {:>7.2}°  {:>7.2}°  {:>6.1} km",
            minutes,
            ra.get::<degree>(),
            dec.get::<degree>(),
            altitude
        );
    }

    // ICRS to ECEF transformation with velocity
    println!("\n━━━ FRAME TRANSFORMATION (ICRS → ECEF) ━━━\n");
    
    let transform = transforms::icrs_to_ecef_at(epoch);
    let pos_ecef = transform.transform(position);
    let vel_ecef = transform.transform_velocity(position, velocity);
    
    let [xe, ye, ze] = pos_ecef.to_cartesian();
    
    println!("  ECEF Position:");
    println!("    X: {:>10.2} km", xe.get::<kilometer>());
    println!("    Y: {:>10.2} km", ye.get::<kilometer>());
    println!("    Z: {:>10.2} km", ze.get::<kilometer>());
    
    println!("\n  ECEF Velocity:");
    println!("    Vx: {:>8.3} km/s", vel_ecef[0] / 1000.0);
    println!("    Vy: {:>8.3} km/s", vel_ecef[1] / 1000.0);
    println!("    Vz: {:>8.3} km/s", vel_ecef[2] / 1000.0);

    // Verify inverse velocity transformation
    let transform_inv = transforms::ecef_to_icrs_at(epoch);
    let vel_icrs_back = transform_inv.transform_velocity(pos_ecef, vel_ecef);
    
    let vel_error = ((vel_icrs_back[0] - velocity[0]).powi(2)
        + (vel_icrs_back[1] - velocity[1]).powi(2)
        + (vel_icrs_back[2] - velocity[2]).powi(2))
        .sqrt();
    
    println!("\n  Velocity roundtrip error: {:.3e} m/s", vel_error);

    // Time scale conversions
    println!("\n━━━ TIME SCALE CONVERSIONS ━━━\n");
    
    use sguaba_celestial::constants::utc_to_julian_date;
    use sguaba_celestial::{utc_to_tai, utc_to_tt, utc_to_tdb};
    
    let utc_jd = utc_to_julian_date(epoch);
    let tai_jd = utc_to_tai(epoch);
    let tt_jd = utc_to_tt(epoch);
    let tdb_jd = utc_to_tdb(epoch);
    
    println!("  UTC (JD):  {:.6}", utc_jd);
    println!("  TAI (JD):  {:.6}  (+{:.3} s)", tai_jd, (tai_jd - utc_jd) * 86400.0);
    println!("  TT (JD):   {:.6}  (+{:.3} s)", tt_jd, (tt_jd - utc_jd) * 86400.0);
    println!("  TDB (JD):  {:.6}  (+{:.3} s)", tdb_jd, (tdb_jd - utc_jd) * 86400.0);

    println!("\n✅ All orbital mechanics features demonstrated successfully!");
}

