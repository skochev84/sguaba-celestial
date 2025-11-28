//! Celestial coordinate system definitions.

use sguaba::CoordinateSystem;
use uom::si::f64::Length;
use uom::si::length::meter;

/// Celestial coordinate convention (right-handed XYZ).
///
/// Used for inertial celestial reference frames like ICRS and MCI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CelestialConvention;

/// Components for celestial coordinate systems (X, Y, Z).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CelestialComponents {
    /// X component
    pub x: Length,
    /// Y component
    pub y: Length,
    /// Z component
    pub z: Length,
}

impl From<CelestialComponents> for [Length; 3] {
    fn from(c: CelestialComponents) -> Self {
        [c.x, c.y, c.z]
    }
}

impl From<[Length; 3]> for CelestialComponents {
    fn from([x, y, z]: [Length; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<CelestialComponents> for [f64; 3] {
    fn from(c: CelestialComponents) -> Self {
        [
            c.x.get::<meter>(),
            c.y.get::<meter>(),
            c.z.get::<meter>(),
        ]
    }
}

impl From<[f64; 3]> for CelestialComponents {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self {
            x: Length::new::<meter>(x),
            y: Length::new::<meter>(y),
            z: Length::new::<meter>(z),
        }
    }
}

/// International Celestial Reference System (ICRS).
///
/// ICRS is the fundamental celestial reference frame adopted by the International
/// Astronomical Union (IAU) in 1997. It provides a quasi-inertial reference frame
/// for describing positions and motions of celestial objects.
///
/// # Coordinate Axes
///
/// - **X axis**: Points towards the vernal equinox at the J2000.0 epoch (RA = 0h)
/// - **Y axis**: 90° east in the equatorial plane (RA = 6h)
/// - **Z axis**: Points towards the North Celestial Pole
///
/// # Properties
///
/// - **Origin**: Earth's center of mass (geocenter)
/// - **Orientation**: Aligned with the mean equator and equinox at J2000.0
/// - **Definition**: Established by observed positions of distant extragalactic sources
/// - **Handedness**: Right-handed
///
/// # Time Dependence
///
/// ICRS itself is time-independent and inertial. However, transforms between ICRS
/// and Earth-fixed frames (like [`sguaba::systems::Ecef`]) are time-dependent due to 
/// Earth's rotation and precession.
///
/// # Accuracy
///
/// Transformations to/from ECEF achieve < 30 milliarcseconds accuracy (2020-2050)
/// using the IAU 2006/2000A precession model and Earth Rotation Angle (ERA).
///
/// # Limitations
///
/// - Nutation (short-period wobbles) is not modeled
/// - No polar motion corrections
/// - No corrections for tidal effects on Earth's rotation
///
/// # References
///
/// - [IAU Resolution B2 (1997)](https://www.iau.org/static/resolutions/IAU1997_French.pdf)
/// - [IERS Technical Note 36](https://www.iers.org/IERS/EN/Publications/TechnicalNotes/tn36.html)
/// - [USNO Circular 179](https://aa.usno.navy.mil/publications/docs/Circular_179.pdf)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Icrs;

impl CoordinateSystem for Icrs {
    type Convention = CelestialConvention;
}

/// Moon-Centered Inertial (MCI) reference frame.
///
/// MCI is a selenocentric (Moon-centered) inertial reference frame aligned with
/// the Moon's principal axes, commonly used for lunar orbit determination and
/// surface operations.
///
/// # Coordinate Axes
///
/// Based on IAU 2009 lunar orientation model:
///
/// - **X axis**: Towards mean lunar sub-Earth point
/// - **Y axis**: 90° east along lunar equator
/// - **Z axis**: Towards lunar North pole
///
/// # Properties
///
/// - **Origin**: Moon's center of mass (selenocenter)
/// - **Orientation**: Mean lunar principal axes (IAU 2009 constants)
/// - **Handedness**: Right-handed
/// - **Time dependence**: Approximately inertial (libration not modeled)
///
/// # Accuracy
///
/// Transform to ICRS achieves arcsecond-level accuracy using simplified IAU 2009
/// constants. Does not account for:
///
/// - Physical libration
/// - Time-varying lunar orientation
/// - Topocentric corrections
///
/// # IAU 2009 Constants
///
/// - Right ascension: α = 269.9949°
/// - Declination: δ = 66.5392°
/// - Prime meridian: W = 38.3213°
///
/// # References
///
/// - [IAU WGCCRE 2009](https://astrogeology.usgs.gov/search/map/Docs/WGCCRE/WGCCRE2009reprint)
/// - [Archinal et al. 2011](https://doi.org/10.1007/s10569-010-9320-4)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mci;

impl CoordinateSystem for Mci {
    type Convention = CelestialConvention;
}

// ======================================================================================
// CELESTIAL COORDINATE HELPERS
// ======================================================================================

// Coordinate<Icrs> methods moved to IcrsCoordinateExt trait in ext module

#[cfg(test)]
mod celestial_coords_tests {
    use super::*;
    use crate::IcrsCoordinateExt;
    use uom::si::angle::degree;
    use uom::si::f64::{Angle, Length};
    use uom::si::length::kilometer;

    #[test]
    fn ra_dec_roundtrip() {
        let ra_in = Angle::new::<degree>(45.0);
        let dec_in = Angle::new::<degree>(30.0);
        let dist_in = Length::new::<kilometer>(1000.0);
        
        let pos = sguaba::Coordinate::<Icrs>::from_ra_dec(ra_in, dec_in, dist_in);
        let (ra_out, dec_out, dist_out) = pos.to_spherical_celestial();
        
        assert!((ra_out.get::<degree>() - ra_in.get::<degree>()).abs() < 0.001);
        assert!((dec_out.get::<degree>() - dec_in.get::<degree>()).abs() < 0.001);
        assert!((dist_out.get::<kilometer>() - dist_in.get::<kilometer>()).abs() < 0.001);
    }

    #[test]
    fn ra_dec_north_pole() {
        let dec = Angle::new::<degree>(90.0);
        let dist = Length::new::<kilometer>(1000.0);
        
        let pos = sguaba::Coordinate::<Icrs>::from_ra_dec(
            Angle::new::<degree>(0.0),
            dec,
            dist,
        );
        
        let [x, y, z] = pos.to_cartesian();
        assert!(x.get::<kilometer>().abs() < 0.001);
        assert!(y.get::<kilometer>().abs() < 0.001);
        assert!((z.get::<kilometer>() - 1000.0).abs() < 0.001);
    }
}

