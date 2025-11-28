//! Extension traits for adding celestial functionality to sguaba types.
//!
//! Since Rust's orphan rules prevent us from implementing methods on foreign types
//! (like `sguaba::Coordinate` or `sguaba::math::RigidBodyTransform`) from an external crate,
//! we provide extension traits that users can import to get the same functionality.
//!
//! # Usage
//!
//! ```ignore
//! use sguaba_celestial::prelude::*; // Imports all extension traits
//! ```

use chrono::{DateTime, Utc};
use sguaba::{math::RigidBodyTransform, systems::Ecef, Coordinate};
use uom::si::f64::{Angle, Length};

use crate::{Icrs, Mci};

/// Extension methods for ICRS coordinates.
///
/// Provides celestial coordinate conversions (RA/Dec) for ICRS frames.
pub trait IcrsCoordinateExt {
    /// Convert to spherical celestial coordinates (Right Ascension, Declination, Distance).
    ///
    /// # Returns
    ///
    /// `(ra, dec, distance)` where:
    /// - `ra`: Right ascension (0 to 2π radians)
    /// - `dec`: Declination (-π/2 to π/2 radians)
    /// - `distance`: Radial distance from origin
    fn to_spherical_celestial(&self) -> (Angle, Angle, Length);

    /// Construct ICRS coordinate from spherical celestial coordinates.
    ///
    /// # Parameters
    ///
    /// - `ra`: Right ascension (any value, will be normalized to 0-2π)
    /// - `dec`: Declination (must be in range [-π/2, π/2])
    /// - `distance`: Radial distance from origin
    fn from_ra_dec(ra: Angle, dec: Angle, distance: Length) -> Self;

    /// Build ICRS coordinate from cartesian components.
    ///
    /// # Parameters
    ///
    /// - `components`: Cartesian X, Y, Z components
    fn build(components: crate::frames::CelestialComponents) -> Self;
}

impl IcrsCoordinateExt for Coordinate<Icrs> {
    fn to_spherical_celestial(&self) -> (Angle, Angle, Length) {
        use uom::si::angle::radian;
        use uom::si::length::meter;

        let [x, y, z] = self.to_cartesian();
        let distance = self.distance_from_origin();

        let x_val = x.get::<meter>();
        let y_val = y.get::<meter>();
        let z_val = z.get::<meter>();

        // Right ascension: atan2(y, x)
        let ra = Angle::new::<radian>(y_val.atan2(x_val));
        let ra = if ra.get::<radian>() < 0.0 {
            Angle::new::<radian>(ra.get::<radian>() + 2.0 * std::f64::consts::PI)
        } else {
            ra
        };

        // Declination: asin(z / r)
        let r = distance.get::<meter>();
        let dec = if r > 0.0 {
            Angle::new::<radian>((z_val / r).asin())
        } else {
            Angle::new::<radian>(0.0)
        };

        (ra, dec, distance)
    }

    fn from_ra_dec(ra: Angle, dec: Angle, distance: Length) -> Self {
        use uom::si::angle::radian;
        use uom::si::length::meter;

        let ra_rad = ra.get::<radian>();
        let dec_rad = dec.get::<radian>();
        let r = distance.get::<meter>();

        let (sin_dec, cos_dec) = dec_rad.sin_cos();
        let (sin_ra, cos_ra) = ra_rad.sin_cos();

        let x = r * cos_dec * cos_ra;
        let y = r * cos_dec * sin_ra;
        let z = r * sin_dec;

        #[allow(deprecated)]
        Self::from_cartesian(
            Length::new::<meter>(x),
            Length::new::<meter>(y),
            Length::new::<meter>(z),
        )
    }

    fn build(components: crate::frames::CelestialComponents) -> Self {
        #[allow(deprecated)]
        Self::from_cartesian(components.x, components.y, components.z)
    }
}

/// Extension methods for MCI coordinates.
pub trait MciCoordinateExt {
    /// Build MCI coordinate from cartesian components.
    fn build(components: crate::frames::CelestialComponents) -> Self;
}

impl MciCoordinateExt for Coordinate<Mci> {
    fn build(components: crate::frames::CelestialComponents) -> Self {
        #[allow(deprecated)]
        Self::from_cartesian(components.x, components.y, components.z)
    }
}

/// Extension methods for GCRF coordinates.
pub trait GcrfCoordinateExt {
    /// Build GCRF coordinate from cartesian components.
    fn build(components: crate::frames::CelestialComponents) -> Self;
}

impl GcrfCoordinateExt for Coordinate<crate::Gcrf> {
    fn build(components: crate::frames::CelestialComponents) -> Self {
        #[allow(deprecated)]
        Self::from_cartesian(components.x, components.y, components.z)
    }
}

/// Extension methods for time-dependent celestial transforms.
pub trait CelestialTransformExt {
    /// Constructs the transform from ICRS to ECEF at the specified time.
    fn icrs_to_ecef_at(time: DateTime<Utc>) -> RigidBodyTransform<Icrs, Ecef>;

    /// Constructs the transform from ECEF to ICRS at the specified time.
    fn ecef_to_icrs_at(time: DateTime<Utc>) -> RigidBodyTransform<Ecef, Icrs>;

    /// Constructs the transform from MCI to ICRS.
    fn mci_to_icrs() -> RigidBodyTransform<Mci, Icrs>;

    /// Constructs the transform from ICRS to MCI.
    fn icrs_to_mci() -> RigidBodyTransform<Icrs, Mci>;
}

// Note: We can't implement this as inherent methods on RigidBodyTransform,
// so users must call these as: CelestialTransformExt::icrs_to_ecef_at(time)
// or use the convenience functions in the transforms module

/// Extension methods for velocity transformations.
pub trait VelocityTransformExt<From, To> {
    /// Transform a velocity vector from one frame to another.
    ///
    /// For rotating frames (like ICRS to ECEF), this accounts for the frame rotation
    /// by applying the rotation to the velocity vector.
    ///
    /// # Parameters
    ///
    /// - `position`: The position at which the velocity is defined (unused for pure rotations)
    /// - `velocity`: The velocity vector in the source frame [vx, vy, vz] in m/s
    ///
    /// # Returns
    ///
    /// The velocity vector in the target frame [vx, vy, vz] in m/s
    fn transform_velocity(
        &self,
        position: sguaba::Coordinate<From>,
        velocity: [f64; 3],
    ) -> [f64; 3];
}

impl<From, To> VelocityTransformExt<From, To> for RigidBodyTransform<From, To>
where
    From: sguaba::CoordinateSystem,
    To: sguaba::CoordinateSystem,
{
    fn transform_velocity(
        &self,
        _position: sguaba::Coordinate<From>,
        velocity: [f64; 3],
    ) -> [f64; 3] {
        // Transform velocity as a vector (direction only, not position)
        // Create a coordinate at the origin plus the velocity vector
        use uom::si::f64::Length;
        use uom::si::length::meter;

        // Create a coordinate at the origin plus the velocity vector
        let origin = sguaba::Coordinate::<From>::origin();
        #[allow(deprecated)]
        let velocity_point = sguaba::Coordinate::<From>::from_cartesian(
            Length::new::<meter>(velocity[0]),
            Length::new::<meter>(velocity[1]),
            Length::new::<meter>(velocity[2]),
        );

        // Transform both
        let origin_transformed = self.transform(origin);
        let velocity_transformed = self.transform(velocity_point);

        // Get the difference to recover the transformed velocity
        let [vx_orig, vy_orig, vz_orig] = origin_transformed.to_cartesian();
        let [vx_new, vy_new, vz_new] = velocity_transformed.to_cartesian();

        [
            (vx_new - vx_orig).get::<meter>(),
            (vy_new - vy_orig).get::<meter>(),
            (vz_new - vz_orig).get::<meter>(),
        ]
    }
}
