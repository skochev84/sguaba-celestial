//! Lunar surface operations example.
//!
//! Demonstrates MCI (Moon-Centered Inertial) frame usage for lunar
//! surface coordinates and transformations to/from ICRS.

use sguaba::Coordinate;
use sguaba_celestial::{builder::mci::Components, transforms, Mci, MciCoordinateExt};
use uom::si::f64::Length;
use uom::si::length::{kilometer, meter};

fn main() {
    println!("=== Lunar Surface Operations ===\n");

    // Lunar surface point at Moon's equator
    // Moon's mean radius: ~1737.4 km
    let lunar_radius = Length::new::<meter>(1_737_400.0);

    let surface_points: Vec<(&str, Coordinate<Mci>)> = vec![
        (
            "Sub-Earth point",
            MciCoordinateExt::build(
                Components {
                    x: lunar_radius,
                    y: Length::new::<meter>(0.0),
                    z: Length::new::<meter>(0.0),
                }
                .into(),
            ),
        ),
        (
            "East limb",
            MciCoordinateExt::build(
                Components {
                    x: Length::new::<meter>(0.0),
                    y: lunar_radius,
                    z: Length::new::<meter>(0.0),
                }
                .into(),
            ),
        ),
        (
            "North pole",
            MciCoordinateExt::build(
                Components {
                    x: Length::new::<meter>(0.0),
                    y: Length::new::<meter>(0.0),
                    z: lunar_radius,
                }
                .into(),
            ),
        ),
        (
            "South pole",
            MciCoordinateExt::build(
                Components {
                    x: Length::new::<meter>(0.0),
                    y: Length::new::<meter>(0.0),
                    z: -lunar_radius,
                }
                .into(),
            ),
        ),
    ];

    let mci_to_icrs = transforms::mci_to_icrs();

    println!("Lunar surface points in MCI and ICRS frames:\n");
    println!(
        "{:>18} | {:>40} | {:>40}",
        "Location", "MCI (km)", "ICRS (km)"
    );
    println!("{:-<104}", "");

    for (name, coord_mci) in &surface_points {
        let coord_icrs = mci_to_icrs.transform(*coord_mci);

        let [mci_x, mci_y, mci_z] = coord_mci.to_cartesian();
        let [icrs_x, icrs_y, icrs_z] = coord_icrs.to_cartesian();

        println!(
            "{:>18} | ({:>7.1}, {:>7.1}, {:>7.1}) | ({:>7.1}, {:>7.1}, {:>7.1})",
            name,
            mci_x.get::<kilometer>(),
            mci_y.get::<kilometer>(),
            mci_z.get::<kilometer>(),
            icrs_x.get::<kilometer>(),
            icrs_y.get::<kilometer>(),
            icrs_z.get::<kilometer>(),
        );
    }

    // Verify inverse transformation
    println!("\n=== Transformation Verification ===");

    let icrs_to_mci = transforms::icrs_to_mci();
    let test_coord_mci: Coordinate<Mci> = MciCoordinateExt::build(
        Components {
            x: lunar_radius,
            y: Length::new::<meter>(0.0),
            z: Length::new::<meter>(0.0),
        }
        .into(),
    );

    let test_coord_icrs = mci_to_icrs.transform(test_coord_mci);
    let roundtrip_mci = icrs_to_mci.transform(test_coord_icrs);

    let error = test_coord_mci.distance_from(&roundtrip_mci);
    println!("Roundtrip error: {:.9} m", error.get::<meter>());

    if error.get::<meter>() < 1e-6 {
        println!("✓ MCI ↔ ICRS transformation accurate to < 1 μm");
    }

    // Show frame orientation difference
    println!("\n=== MCI vs ICRS Orientation ===");
    println!("MCI is aligned with Moon's mean principal axes");
    println!("ICRS is aligned with Earth's equatorial plane at J2000");
    println!("\nIAU 2009 Lunar Orientation:");
    println!("  Right Ascension: {:.4}°", 269.9949);
    println!("  Declination:     {:.4}°", 66.5392);
    println!("  Prime Meridian:  {:.4}°", 38.3213);
}
