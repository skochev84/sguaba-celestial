//! Additional celestial reference frames and coordinate systems.

use sguaba::CoordinateSystem;

use super::frames::CelestialConvention;

/// Geocentric Celestial Reference Frame (GCRF).
///
/// GCRF is effectively equivalent to ICRS but is explicitly Earth-centered.
/// It's commonly used in satellite orbit propagation and GPS applications.
///
/// # Properties
///
/// - **Origin**: Earth's center of mass
/// - **Orientation**: Aligned with ICRS
/// - **Usage**: Satellite tracking, GPS, Earth orbiting spacecraft
///
/// # Relationship to ICRS
///
/// GCRF ≈ ICRS for Earth-centered applications. The transformation between
/// them is identity for practical purposes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gcrf;

impl CoordinateSystem for Gcrf {
    type Convention = CelestialConvention;
}

// SAFETY: GCRF is aligned with ICRS by definition
unsafe impl sguaba::systems::EquivalentTo<super::frames::Icrs> for Gcrf {}
unsafe impl sguaba::systems::EquivalentTo<Gcrf> for super::frames::Icrs {}

/// Earth Mean Equator and Equinox of J2000 (EME2000).
///
/// EME2000 is similar to ICRS but uses the Earth's mean equator and equinox
/// at the J2000.0 epoch as its fundamental plane and direction.
///
/// # Properties
///
/// - **Origin**: Earth's center of mass
/// - **Fundamental plane**: Earth's mean equatorial plane at J2000.0
/// - **X axis**: Points to mean vernal equinox at J2000.0
/// - **Z axis**: Points to mean North Pole at J2000.0
///
/// # Relationship to ICRS
///
/// EME2000 differs from ICRS by a small frame bias (~80 milliarcseconds).
/// For many applications, they can be treated as equivalent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eme2000;

impl CoordinateSystem for Eme2000 {
    type Convention = CelestialConvention;
}

/// Ecliptic coordinate system.
///
/// The ecliptic frame uses the plane of Earth's orbit around the Sun
/// as its fundamental plane, useful for solar system dynamics.
///
/// # Coordinate Axes
///
/// - **X axis**: Points towards the vernal equinox
/// - **Y axis**: 90° east along the ecliptic plane
/// - **Z axis**: Perpendicular to ecliptic, towards ecliptic North Pole
///
/// # Properties
///
/// - **Origin**: Solar System Barycenter (or geocenter for Earth-centered variant)
/// - **Fundamental plane**: Mean ecliptic at J2000.0
/// - **Obliquity**: ε₀ ≈ 23.4393° (mean obliquity of ecliptic)
///
/// # Applications
///
/// - Planetary ephemerides
/// - Asteroid and comet orbits
/// - Solar system navigation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ecliptic;

impl CoordinateSystem for Ecliptic {
    type Convention = CelestialConvention;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcrf_implements_coordinate_system() {
        // Just verify it compiles and has the right convention
        fn check_system<S: CoordinateSystem>() {}
        check_system::<Gcrf>();
    }

    #[test]
    fn eme2000_implements_coordinate_system() {
        fn check_system<S: CoordinateSystem>() {}
        check_system::<Eme2000>();
    }

    #[test]
    fn ecliptic_implements_coordinate_system() {
        fn check_system<S: CoordinateSystem>() {}
        check_system::<Ecliptic>();
    }
}
