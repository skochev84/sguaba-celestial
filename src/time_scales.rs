//! Time scale conversions for celestial mechanics.
//!
//! This module provides conversions between various astronomical time scales:
//! - UTC (Coordinated Universal Time)
//! - TAI (International Atomic Time)
//! - TT (Terrestrial Time)
//! - UT1 (Universal Time)
//! - TDB (Barycentric Dynamical Time)

use chrono::{DateTime, Datelike, Utc};

use super::constants::SECONDS_PER_DAY;
use super::errors::{CelestialError, CelestialResult};

/// Minimum valid epoch year for celestial calculations.
const MIN_VALID_YEAR: i32 = 1900;

/// Maximum valid epoch year for celestial calculations.
const MAX_VALID_YEAR: i32 = 2100;

/// Current estimated leap seconds (approximate for 2025).
/// In production, this should be updated from IERS Bulletin C.
const CURRENT_LEAP_SECONDS: f64 = 37.0;

/// TT - TAI offset in seconds (defined constant).
const TT_MINUS_TAI: f64 = 32.184;

/// Validate that an epoch is within supported range.
///
/// The celestial module supports epochs from 1900-2100. Outside this range,
/// astronomical models (precession, nutation, etc.) may not be accurate.
///
/// # Errors
///
/// Returns `CelestialError::EpochOutOfRange` if the epoch year is outside [1900, 2100].
///
/// # Example
///
/// ```no_run
/// # #[cfg(feature = "celestial")] {
/// use sguaba::celestial::validate_epoch;
/// use chrono::Utc;
///
/// let epoch = Utc::now();
/// match validate_epoch(epoch) {
///     Ok(_) => println!("Epoch is valid"),
///     Err(e) => eprintln!("Invalid epoch: {:?}", e),
/// }
/// # }
/// ```
pub fn validate_epoch(epoch: DateTime<Utc>) -> CelestialResult<()> {
    let year = epoch.year();
    if !(MIN_VALID_YEAR..=MAX_VALID_YEAR).contains(&year) {
        // Convert year limits to approximate Julian Date
        let min_jd = 2415020.5; // ~1900-01-01
        let max_jd = 2488070.5; // ~2100-01-01
        return Err(CelestialError::EpochOutOfRange {
            epoch,
            min_jd,
            max_jd,
        });
    }
    Ok(())
}

/// Convert UTC to TAI (International Atomic Time).
///
/// TAI = UTC + leap_seconds
///
/// # Note
///
/// This uses a simplified leap second count. For production applications,
/// query IERS Bulletin C for the exact leap second count at the given date.
#[must_use]
pub fn utc_to_tai(utc: DateTime<Utc>) -> f64 {
    let jd_utc = utc.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5;
    jd_utc + (CURRENT_LEAP_SECONDS / SECONDS_PER_DAY)
}

/// Convert UTC to TT (Terrestrial Time).
///
/// TT = UTC + leap_seconds + 32.184s
///
/// TT is the theoretical ideal time scale for Earth-based observations.
#[must_use]
pub fn utc_to_tt(utc: DateTime<Utc>) -> f64 {
    let jd_utc = utc.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5;
    jd_utc + ((CURRENT_LEAP_SECONDS + TT_MINUS_TAI) / SECONDS_PER_DAY)
}

/// Convert UTC to UT1 (Universal Time).
///
/// UT1 = UTC + (UT1-UTC)
///
/// # Note
///
/// This currently returns UTC (UT1-UTC ≈ 0). For sub-second accuracy,
/// query IERS Bulletin A for DUT1 values.
#[must_use]
pub fn utc_to_ut1(utc: DateTime<Utc>) -> f64 {
    // Simplified: assumes UT1-UTC ≈ 0 (within ±0.9 seconds)
    utc.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5
}

/// Convert UTC to TDB (Barycentric Dynamical Time).
///
/// TDB is the time scale for solar system dynamics, accounting for
/// relativistic effects.
///
/// # Approximation
///
/// This uses a simplified formula. The full conversion requires the
/// observer's position and velocity in the solar system barycentric frame.
///
/// TDB ≈ TT + 0.001658 sin(g) + 0.000014 sin(2g)
/// where g = 357.53 + 0.9856003 * (JD - 2451545.0) degrees
#[must_use]
pub fn utc_to_tdb(utc: DateTime<Utc>) -> f64 {
    let tt = utc_to_tt(utc);
    let t = tt - 2451545.0; // Days from J2000
    
    // Mean anomaly of Earth's orbit
    let g_deg = 357.53 + 0.9856003 * t;
    let g = g_deg.to_radians();
    
    // Periodic term (seconds)
    let periodic = 0.001658 * g.sin() + 0.000014 * (2.0 * g).sin();
    
    tt + (periodic / SECONDS_PER_DAY)
}

/// Convert TT to UTC (approximate inverse).
///
/// This is an approximation since leap seconds make the conversion non-trivial.
#[must_use]
pub fn tt_to_utc_approx(tt_jd: f64) -> f64 {
    tt_jd - ((CURRENT_LEAP_SECONDS + TT_MINUS_TAI) / SECONDS_PER_DAY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_to_tai_offset_is_positive() {
        let utc = Utc::now();
        let tai = utc_to_tai(utc);
        let utc_jd = utc.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5;
        
        // TAI should be ahead of UTC by leap seconds
        assert!((tai - utc_jd) > 0.0);
        assert!((tai - utc_jd) * SECONDS_PER_DAY > 30.0); // At least 30 seconds
    }

    #[test]
    fn utc_to_tt_offset_is_correct() {
        let utc = Utc::now();
        let tt = utc_to_tt(utc);
        let utc_jd = utc.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5;
        
        let offset_seconds = (tt - utc_jd) * SECONDS_PER_DAY;
        let expected = CURRENT_LEAP_SECONDS + TT_MINUS_TAI;
        
        assert!((offset_seconds - expected).abs() < 0.1);
    }

    #[test]
    fn tt_to_utc_roundtrip_is_approximate() {
        let utc = Utc::now();
        let tt = utc_to_tt(utc);
        let utc_back = tt_to_utc_approx(tt);
        let utc_jd = utc.timestamp() as f64 / SECONDS_PER_DAY + 2440587.5;
        
        // Should be within 1 second
        assert!((utc_back - utc_jd).abs() * SECONDS_PER_DAY < 1.0);
    }

    #[test]
    fn tdb_differs_from_tt_by_small_amount() {
        let utc = Utc::now();
        let tt = utc_to_tt(utc);
        let tdb = utc_to_tdb(utc);
        
        // TDB and TT differ by at most ~2 milliseconds
        let diff_seconds = (tdb - tt).abs() * SECONDS_PER_DAY;
        assert!(diff_seconds < 0.002);
    }
}
