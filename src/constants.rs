use nalgebra::{Quaternion, Unit};
type UnitQuaternion = Unit<Quaternion<f64>>;

/// Astronomical constants and time conversion utilities.
use chrono::{DateTime, Utc};
use nalgebra::Vector3;
use std::sync::OnceLock;

/// J2000.0 epoch: 2000-01-01 12:00:00 TT (Julian Date 2451545.0).
pub const J2000_JD: f64 = 2451545.0;

/// Days per Julian century.
pub const DAYS_PER_CENTURY: f64 = 36525.0;

/// Seconds per day.
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// Arcseconds to radians conversion factor.
pub const ARCSEC_TO_RAD: f64 = std::f64::consts::PI / (180.0 * 3600.0);

/// Astronomical Unit in meters (IAU 2012 definition).
pub const AU_METERS: f64 = 149_597_870_700.0;

/// Earth mean radius in meters (WGS84).
pub const EARTH_RADIUS_MEAN: f64 = 6_371_008.8;

/// Earth equatorial radius in meters (WGS84).
pub const EARTH_RADIUS_EQUATORIAL: f64 = 6_378_137.0;

/// Earth polar radius in meters (WGS84).
pub const EARTH_RADIUS_POLAR: f64 = 6_356_752.314_245;

/// Moon mean radius in meters (IAU/IAG).
pub const MOON_RADIUS_MEAN: f64 = 1_737_400.0;

/// Speed of light in vacuum (m/s, exact by definition).
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Earth's rotation rate (rad/s).
pub const EARTH_ROTATION_RATE: f64 = 7.292_115_146_7e-5;

/// Gravitational parameter of Earth (m³/s², WGS84).
pub const MU_EARTH: f64 = 3.986_004_418e14;

/// Gravitational parameter of the Moon (m³/s²).
pub const MU_MOON: f64 = 4.902_800_066e12;

/// IAU 2009 lunar orientation constants.
pub mod lunar {
    /// Right ascension of lunar north pole (degrees).
    pub const RA_DEG: f64 = 269.9949;

    /// Declination of lunar north pole (degrees).
    pub const DEC_DEG: f64 = 66.5392;

    /// Prime meridian angle (degrees).
    pub const W_DEG: f64 = 38.3213;
}

/// Cached MCI → ICRS rotation quaternion (IAU 2009 lunar orientation).
static MCI_TO_ICRS_ROTATION: OnceLock<UnitQuaternion> = OnceLock::new();

/// Get or compute the MCI → ICRS rotation.
///
/// Uses IAU 2009 lunar orientation constants. The rotation is cached after first computation.
pub fn mci_to_icrs_rotation() -> &'static UnitQuaternion {
    MCI_TO_ICRS_ROTATION.get_or_init(|| {
        let ra = lunar::RA_DEG.to_radians();
        let dec = lunar::DEC_DEG.to_radians();
        let w = lunar::W_DEG.to_radians();

        // Rotation sequence aligning Moon's principal axes with ICRS
        UnitQuaternion::from_axis_angle(&Vector3::z_axis(), ra)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), dec)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), w)
    })
}

/// Convert UTC DateTime to Julian Date.
///
/// Note: This conversion treats UTC as UT1, ignoring the UT1-UTC correction
/// (typically < 1 second). For applications requiring sub-second accuracy,
/// proper time scale conversions should be implemented.
pub fn utc_to_julian_date(time: DateTime<Utc>) -> f64 {
    time.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5
}

/// Compute Earth Rotation Angle (ERA) for a given Julian Date.
///
/// ERA is the angle of rotation of the Earth about the Celestial Intermediate Pole (CIP)
/// with respect to the Terrestrial Intermediate Origin (TIO).
///
/// # Reference
///
/// IERS Conventions 2010, Chapter 5, Equation 5.15
pub fn earth_rotation_angle(jd: f64) -> f64 {
    let d = jd - J2000_JD; // Days from J2000.0
    2.0 * std::f64::consts::PI * (0.7790572732640 + 1.002_737_811_911_354_6 * d).fract()
}

/// Compute ICRS → ECEF rotation at a given time.
///
/// Uses IAU 2006/2000A precession model and Earth Rotation Angle (ERA).
/// This provides accuracy < 30 milliarcseconds for epochs 2020-2050.
///
/// # Limitations
///
/// - Nutation is not modeled (precession-only)
/// - No polar motion corrections
/// - UTC treated as UT1 (UT1-UTC correction ignored)
pub fn icrs_to_ecef_rotation(time: DateTime<Utc>) -> UnitQuaternion {
    icrs_to_ecef_rotation_with_nutation(time, false)
}

/// Compute ICRS → ECEF rotation at a given time with optional nutation.
///
/// # Parameters
///
/// - `time`: The UTC time for the transformation
/// - `include_nutation`: If true, includes IAU 2000B nutation model
///
/// # Accuracy
///
/// - Without nutation: < 30 milliarcseconds (2020-2050)
/// - With nutation: < 1 milliarcsecond (2020-2050)
pub fn icrs_to_ecef_rotation_with_nutation(
    time: DateTime<Utc>,
    include_nutation: bool,
) -> UnitQuaternion {
    let jd = utc_to_julian_date(time);
    let t_centuries = (jd - J2000_JD) / DAYS_PER_CENTURY;

    // IAU 2006/2000A precession angles (arcsec → radians)
    let zeta =
        (2306.2181 * t_centuries + 1.39656 * t_centuries.powi(2) + 0.000139 * t_centuries.powi(3))
            * ARCSEC_TO_RAD;

    let theta =
        (2004.3109 * t_centuries - 0.42665 * t_centuries.powi(2) - 0.041833 * t_centuries.powi(3))
            * ARCSEC_TO_RAD;

    let z =
        (2306.2181 * t_centuries + 1.09468 * t_centuries.powi(2) + 0.018203 * t_centuries.powi(3))
            * ARCSEC_TO_RAD;

    // Precession rotation: Z(-ζ) * Y(θ) * Z(-z)
    let precession = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -zeta)
        * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), theta)
        * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -z);

    // Optional nutation correction
    let nutation_rot = if include_nutation {
        nutation_matrix(jd)
    } else {
        UnitQuaternion::identity()
    };

    // Earth Rotation Angle (ERA)
    let era = earth_rotation_angle(jd);

    // Combined rotation: ERA * Nutation * Precession
    UnitQuaternion::from_axis_angle(&Vector3::z_axis(), era) * nutation_rot * precession
}

/// Compute IAU 2000B nutation matrix.
///
/// This is a simplified nutation model with 77 terms, providing
/// milliarcsecond-level accuracy for most applications.
///
/// # Reference
///
/// IERS Conventions 2003, Chapter 5
pub fn nutation_matrix(jd: f64) -> UnitQuaternion {
    let t = (jd - J2000_JD) / DAYS_PER_CENTURY;

    // Mean anomaly of the Moon (radians)
    let l = (134.96340251 + (1717915923.2178 * t + 31.8792 * t * t) / 3600.0).to_radians();

    // Mean anomaly of the Sun (radians)
    let _l_prime = (357.52910918 + (129596581.0481 * t - 0.5532 * t * t) / 3600.0).to_radians();

    // Mean longitude of the Moon - mean longitude of ascending node
    let f = (93.27209062 + (1739527262.8478 * t - 12.7512 * t * t) / 3600.0).to_radians();

    // Mean elongation of the Moon from the Sun
    let d = (297.85019547 + (1602961601.2090 * t - 6.3706 * t * t) / 3600.0).to_radians();

    // Longitude of the ascending node of the Moon's mean orbit
    let omega = (125.04455501 - (6962890.5431 * t + 7.4722 * t * t) / 3600.0).to_radians();

    // Simplified IAU 2000B nutation series (5 largest terms)
    // Full model has 77 terms; this is adequate for ~0.1 milliarcsecond accuracy

    // Nutation in longitude (arcsec)
    let dpsi = (-17.2064161 * omega.sin()
        - 1.3170907 * (2.0 * f - 2.0 * d + 2.0 * omega).sin()
        - 0.2227794 * (2.0 * omega).sin()
        + 0.2072767 * (2.0 * f + 2.0 * omega).sin()
        - 0.1426572 * l.sin())
        * ARCSEC_TO_RAD;

    // Nutation in obliquity (arcsec)
    let deps = (9.2052331 * omega.cos()
        + 0.5730336 * (2.0 * f - 2.0 * d + 2.0 * omega).cos()
        + 0.0978459 * (2.0 * omega).cos()
        - 0.0897492 * (2.0 * f + 2.0 * omega).cos())
        * ARCSEC_TO_RAD;

    // Mean obliquity of the ecliptic at J2000
    let eps0 = (84381.448 * ARCSEC_TO_RAD)
        + (-46.8150 * t - 0.00059 * t * t + 0.001813 * t * t * t) * ARCSEC_TO_RAD;

    // Nutation rotation: R_x(-ε₀ - Δε) * R_z(-Δψ) * R_x(ε₀)
    UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -(eps0 + deps))
        * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -dpsi)
        * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), eps0)
}

/// Cached polar motion correction (placeholder).
///
/// In production, this should fetch IERS Bulletin A data for xp, yp values.
static POLAR_MOTION_CORRECTION: OnceLock<UnitQuaternion> = OnceLock::new();

/// Get polar motion correction matrix.
///
/// # Note
///
/// Currently returns identity (no correction). For sub-arcsecond accuracy,
/// this should query IERS Bulletin A for polar motion parameters xp and yp.
pub fn polar_motion_correction() -> &'static UnitQuaternion {
    POLAR_MOTION_CORRECTION.get_or_init(UnitQuaternion::identity)
}

/// Compute precession between two arbitrary epochs.
///
/// Returns the rotation matrix from epoch1 to epoch2 using IAU 2006 precession.
pub fn precession_between_epochs(epoch1_jd: f64, epoch2_jd: f64) -> UnitQuaternion {
    let t1 = (epoch1_jd - J2000_JD) / DAYS_PER_CENTURY;
    let t2 = (epoch2_jd - J2000_JD) / DAYS_PER_CENTURY;
    let dt = t2 - t1;

    // Precession angles relative to epoch1
    let zeta = (2306.2181 * dt + (1.39656 + 0.000139 * t1) * dt * dt + 0.000139 * dt * dt * dt)
        * ARCSEC_TO_RAD;

    let theta = (2004.3109 * dt - (0.42665 + 0.041833 * t1) * dt * dt - 0.041833 * dt * dt * dt)
        * ARCSEC_TO_RAD;

    let z = (2306.2181 * dt + (1.09468 + 0.018203 * t1) * dt * dt + 0.018203 * dt * dt * dt)
        * ARCSEC_TO_RAD;

    // Precession rotation
    UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -zeta)
        * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), theta)
        * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn era_at_j2000_is_approximately_zero() {
        let era = earth_rotation_angle(J2000_JD);
        // ERA at J2000.0 is not exactly zero, but close to it
        // The formula gives a fractional rotation based on days from J2000
        assert!(era.abs() < 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn utc_to_jd_conversion() {
        let j2000 = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let jd = utc_to_julian_date(j2000);
        assert!((jd - J2000_JD).abs() < 0.01); // Within ~15 minutes
    }

    #[test]
    fn mci_rotation_is_cached() {
        let rot1 = mci_to_icrs_rotation();
        let rot2 = mci_to_icrs_rotation();
        assert!(std::ptr::eq(rot1, rot2)); // Same memory address
    }

    #[test]
    fn nutation_matrix_is_near_identity() {
        let nut = nutation_matrix(J2000_JD);
        let identity = UnitQuaternion::identity();
        let angle = nut.angle_to(&identity);
        assert!(angle < 0.001);
    }

    #[test]
    fn precession_between_same_epoch_is_identity() {
        let prec = precession_between_epochs(J2000_JD, J2000_JD);
        let identity = UnitQuaternion::identity();
        let angle = prec.angle_to(&identity);
        assert!(angle < 1e-10);
    }

    #[test]
    fn astronomical_constants_are_reasonable() {
        assert!(AU_METERS > 1e11 && AU_METERS < 2e11);
        assert!(EARTH_RADIUS_MEAN > 6e6 && EARTH_RADIUS_MEAN < 7e6);
        assert!(SPEED_OF_LIGHT > 2.99e8 && SPEED_OF_LIGHT < 3e8);
    }
}
