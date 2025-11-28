//! Spaceship intercept mission example.
//!
//! A human spaceship must intercept an alien mothership in deep space.
//! Demonstrates ICRS coordinate calculations, relative positions, velocities,
//! and time-to-intercept computations in an inertial reference frame.

use sguaba::Coordinate;
use sguaba_celestial::{builder::icrs::Components, Icrs, IcrsCoordinateExt};
use uom::si::f64::{Length, Time, Velocity};
use uom::si::length::kilometer;
use uom::si::time::second;
use uom::si::velocity::{kilometer_per_second, meter_per_second};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║        🚀 SPACESHIP INTERCEPT MISSION 🛸                 ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Human spaceship initial position (near Earth)
    let human_ship_pos: Coordinate<Icrs> = IcrsCoordinateExt::build(
        Components {
            x: Length::new::<kilometer>(150_000_000.0), // ~1 AU from origin (X)
            y: Length::new::<kilometer>(0.0),
            z: Length::new::<kilometer>(0.0),
        }
        .into(),
    );

    // Alien mothership position (approaching from outer solar system)
    let alien_ship_pos: Coordinate<Icrs> = IcrsCoordinateExt::build(
        Components {
            x: Length::new::<kilometer>(450_000_000.0), // ~3 AU
            y: Length::new::<kilometer>(120_000_000.0),
            z: Length::new::<kilometer>(-50_000_000.0),
        }
        .into(),
    );

    println!("📍 Initial Positions (ICRS frame):\n");

    let [hx, hy, hz] = human_ship_pos.to_cartesian();
    println!("  Human Spaceship:");
    println!("    X: {:>12.0} km", hx.get::<kilometer>());
    println!("    Y: {:>12.0} km", hy.get::<kilometer>());
    println!("    Z: {:>12.0} km", hz.get::<kilometer>());
    println!(
        "    Distance from Sun: {:.2} AU",
        human_ship_pos.distance_from_origin().get::<kilometer>() / 149_597_870.7
    );

    let [ax, ay, az] = alien_ship_pos.to_cartesian();
    println!("\n  Alien Mothership:");
    println!("    X: {:>12.0} km", ax.get::<kilometer>());
    println!("    Y: {:>12.0} km", ay.get::<kilometer>());
    println!("    Z: {:>12.0} km", az.get::<kilometer>());
    println!(
        "    Distance from Sun: {:.2} AU",
        alien_ship_pos.distance_from_origin().get::<kilometer>() / 149_597_870.7
    );

    // Calculate separation distance
    let separation = human_ship_pos.distance_from(&alien_ship_pos);
    println!("\n🎯 Target Analysis:");
    println!(
        "  Separation distance: {:.2} million km",
        separation.get::<kilometer>() / 1_000_000.0
    );
    println!(
        "  Separation distance: {:.3} AU",
        separation.get::<kilometer>() / 149_597_870.7
    );

    // Mission scenarios with different velocities
    println!("\n⚡ Intercept Scenarios:\n");

    let scenarios = vec![
        (
            "Ion Drive (slow & steady)",
            Velocity::new::<kilometer_per_second>(40.0),
        ),
        (
            "Fusion Drive (fast)",
            Velocity::new::<kilometer_per_second>(150.0),
        ),
        (
            "Warp Drive (theoretical)",
            Velocity::new::<kilometer_per_second>(5000.0),
        ),
    ];

    for (name, velocity) in scenarios {
        let time_to_intercept = separation / velocity;
        let hours = time_to_intercept.get::<second>() / 3600.0;
        let days = hours / 24.0;

        println!(
            "  {} @ {:.0} km/s:",
            name,
            velocity.get::<kilometer_per_second>()
        );

        if days < 1.0 {
            println!("    Time to intercept: {:.1} hours", hours);
        } else if days < 30.0 {
            println!("    Time to intercept: {:.1} days", days);
        } else {
            println!(
                "    Time to intercept: {:.1} days ({:.2} months)",
                days,
                days / 30.0
            );
        }
    }

    // Simulate course correction scenario
    println!("\n📡 Course Correction Simulation:");
    println!("  Alien mothership detected changing position...\n");

    // Alien ship moves
    let alien_ship_new_pos: Coordinate<Icrs> = IcrsCoordinateExt::build(
        Components {
            x: Length::new::<kilometer>(445_000_000.0), // Moved closer!
            y: Length::new::<kilometer>(125_000_000.0),
            z: Length::new::<kilometer>(-48_000_000.0),
        }
        .into(),
    );

    let new_separation = human_ship_pos.distance_from(&alien_ship_new_pos);
    let position_change = separation - new_separation;

    println!(
        "  New separation: {:.2} million km",
        new_separation.get::<kilometer>() / 1_000_000.0
    );
    println!(
        "  Position change: {:.0} km closer!",
        position_change.get::<kilometer>()
    );

    // Calculate required velocity for 7-day intercept
    let target_time = Time::new::<second>(7.0 * 24.0 * 3600.0);
    let required_velocity = new_separation / target_time;

    println!("\n  For 7-day intercept:");
    println!(
        "    Required velocity: {:.1} km/s",
        required_velocity.get::<kilometer_per_second>()
    );
    println!(
        "    ({:.2}% speed of light)",
        required_velocity.get::<meter_per_second>() / 299_792_458.0 * 100.0
    );

    // Multiple waypoint navigation
    println!("\n🗺️  Three-Point Navigation Course:");

    let waypoint_1: Coordinate<Icrs> = IcrsCoordinateExt::build(
        Components {
            x: Length::new::<kilometer>(250_000_000.0),
            y: Length::new::<kilometer>(30_000_000.0),
            z: Length::new::<kilometer>(-10_000_000.0),
        }
        .into(),
    );

    let waypoint_2: Coordinate<Icrs> = IcrsCoordinateExt::build(
        Components {
            x: Length::new::<kilometer>(350_000_000.0),
            y: Length::new::<kilometer>(75_000_000.0),
            z: Length::new::<kilometer>(-30_000_000.0),
        }
        .into(),
    );

    let leg1 = human_ship_pos.distance_from(&waypoint_1);
    let leg2 = waypoint_1.distance_from(&waypoint_2);
    let leg3 = waypoint_2.distance_from(&alien_ship_new_pos);
    let total_distance = leg1 + leg2 + leg3;

    println!(
        "  Leg 1 (Earth → WP1): {:.2} million km",
        leg1.get::<kilometer>() / 1_000_000.0
    );
    println!(
        "  Leg 2 (WP1 → WP2):   {:.2} million km",
        leg2.get::<kilometer>() / 1_000_000.0
    );
    println!(
        "  Leg 3 (WP2 → Alien): {:.2} million km",
        leg3.get::<kilometer>() / 1_000_000.0
    );
    println!("  ─────────────────────────────────────");
    println!(
        "  Total distance:      {:.2} million km",
        total_distance.get::<kilometer>() / 1_000_000.0
    );

    let direct_distance = human_ship_pos.distance_from(&alien_ship_new_pos);
    let extra_distance = total_distance - direct_distance;
    let extra_percent =
        (extra_distance.get::<kilometer>() / direct_distance.get::<kilometer>()) * 100.0;
    println!(
        "  Extra distance:      {:.2} million km ({:.1}% longer)",
        extra_distance.get::<kilometer>() / 1_000_000.0,
        extra_percent
    );

    println!("\n✨ Mission Status: Coordinates calculated successfully!");
    println!("   All positions verified in ICRS inertial reference frame.");
    println!("   Ready for intercept course plotting! 🎯");
}
