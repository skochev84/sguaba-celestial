//! Two-Line Element (TLE) set support for satellite tracking.
//!
//! TLE sets are the standard format for distributing satellite orbital elements.
//! They are used with the SGP4/SDP4 propagation models for predicting satellite positions.

use super::errors::{CelestialError, CelestialResult};
use super::frames::Icrs;
use super::orbital::KeplerianElements;
use sguaba::Coordinate;
use chrono::{DateTime, Duration, TimeZone, Utc};
use uom::si::angle::degree;
use uom::si::f64::{Angle, Length};
use uom::si::length::kilometer;

/// A Two-Line Element set representing satellite orbital parameters.
///
/// TLE format is standardized by NORAD/Space Track and consists of
/// two 69-character lines encoding orbital elements and metadata.
///
/// # Example TLE
///
/// ```text
/// ISS (ZARYA)
/// 1 25544U 98067A   20206.18539600  .00001406  00000-0  33518-4 0  9992
/// 2 25544  51.6461 339.8014 0001473  94.8340 265.2864 15.49309432236008
/// ```
///
/// # Example
///
/// ```no_run
/// # #[cfg(feature = "celestial")] {
/// use sguaba::celestial::TleElements;
///
/// let line1 = "1 25544U 98067A   20206.18539600  .00001406  00000-0  33518-4 0  9992";
/// let line2 = "2 25544  51.6461 339.8014 0001473  94.8340 265.2864 15.49309432236008";
///
/// match TleElements::from_lines(line1, line2) {
///     Ok(tle) => {
///         println!("Satellite: {} at epoch {:?}", tle.catalog_number(), tle.epoch());
///     }
///     Err(e) => eprintln!("Parse error: {:?}", e),
/// }
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TleElements {
    /// Satellite catalog number
    catalog_number: u32,
    /// Epoch (time of element set)
    epoch: DateTime<Utc>,
    /// Inclination (degrees)
    inclination: Angle,
    /// Right ascension of ascending node (degrees)
    raan: Angle,
    /// Eccentricity
    eccentricity: f64,
    /// Argument of perigee (degrees)
    arg_perigee: Angle,
    /// Mean anomaly (degrees)
    mean_anomaly: Angle,
    /// Mean motion (revolutions per day)
    mean_motion: f64,
}

impl TleElements {
    /// Parse a TLE from two lines.
    ///
    /// # Arguments
    ///
    /// * `line1` - The first line of the TLE (69 characters)
    /// * `line2` - The second line of the TLE (69 characters)
    ///
    /// # Errors
    ///
    /// Returns `CelestialError::InvalidCoordinates` if the TLE format is invalid.
    pub fn from_lines(line1: &str, line2: &str) -> CelestialResult<Self> {
        if line1.len() < 69 || line2.len() < 69 {
            return Err(CelestialError::InvalidCoordinates {
                reason: "TLE lines must be 69 characters".into(),
            });
        }

        // Verify line numbers
        if !line1.starts_with('1') || !line2.starts_with('2') {
            return Err(CelestialError::InvalidCoordinates {
                reason: "Invalid TLE line numbers".into(),
            });
        }

        // Parse catalog number (columns 3-7)
        let catalog_number = line1[2..7]
            .trim()
            .parse::<u32>()
            .map_err(|_| CelestialError::InvalidCoordinates { 
                reason: "Invalid catalog number".into() 
            })?;

        // Parse epoch (columns 19-32 of line 1)
        let epoch_year = line1[18..20]
            .parse::<i32>()
            .map_err(|_| CelestialError::InvalidCoordinates { 
                reason: "Invalid epoch year".into() 
            })?;
        let epoch_year = if epoch_year < 57 {
            2000 + epoch_year
        } else {
            1900 + epoch_year
        };

        let epoch_day = line1[20..32]
            .parse::<f64>()
            .map_err(|_| CelestialError::InvalidCoordinates { 
                reason: "Invalid epoch day".into() 
            })?;

        let epoch = tle_epoch_to_datetime(epoch_year, epoch_day)?;

        // Parse orbital elements from line 2
        let inclination = Angle::new::<degree>(
            line2[8..16]
                .trim()
                .parse::<f64>()
                .map_err(|_| CelestialError::InvalidCoordinates { 
                    reason: "Invalid inclination".into() 
                })?,
        );

        let raan = Angle::new::<degree>(
            line2[17..25]
                .trim()
                .parse::<f64>()
                .map_err(|_| CelestialError::InvalidCoordinates { 
                    reason: "Invalid RAAN".into() 
                })?,
        );

        let eccentricity = {
            let ecc_str = format!("0.{}", &line2[26..33]);
            ecc_str
                .parse::<f64>()
                .map_err(|_| CelestialError::InvalidCoordinates { 
                    reason: "Invalid eccentricity".into() 
                })?
        };

        let arg_perigee = Angle::new::<degree>(
            line2[34..42]
                .trim()
                .parse::<f64>()
                .map_err(|_| CelestialError::InvalidCoordinates { 
                    reason: "Invalid argument of perigee".into() 
                })?,
        );

        let mean_anomaly = Angle::new::<degree>(
            line2[43..51]
                .trim()
                .parse::<f64>()
                .map_err(|_| CelestialError::InvalidCoordinates { 
                    reason: "Invalid mean anomaly".into() 
                })?,
        );

        let mean_motion = line2[52..63]
            .trim()
            .parse::<f64>()
            .map_err(|_| CelestialError::InvalidCoordinates { 
                reason: "Invalid mean motion".into() 
            })?;

        Ok(Self {
            catalog_number,
            epoch,
            inclination,
            raan,
            eccentricity,
            arg_perigee,
            mean_anomaly,
            mean_motion,
        })
    }

    /// Get the satellite catalog number.
    #[must_use]
    pub fn catalog_number(&self) -> u32 {
        self.catalog_number
    }

    /// Get the epoch of the TLE.
    #[must_use]
    pub fn epoch(&self) -> DateTime<Utc> {
        self.epoch
    }

    /// Get the inclination.
    #[must_use]
    pub fn inclination(&self) -> Angle {
        self.inclination
    }

    /// Get the eccentricity.
    #[must_use]
    pub fn eccentricity(&self) -> f64 {
        self.eccentricity
    }

    /// Convert TLE to Keplerian elements.
    ///
    /// This conversion computes the semi-major axis from the mean motion
    /// using Earth's gravitational parameter.
    #[must_use]
    pub fn to_keplerian(&self) -> KeplerianElements {
        // Convert mean motion (rev/day) to rad/s
        let n = self.mean_motion * 2.0 * std::f64::consts::PI / 86400.0;

        // Compute semi-major axis from mean motion: n² = μ / a³
        let mu = super::constants::MU_EARTH;
        let a = (mu / (n * n)).powf(1.0 / 3.0);

        KeplerianElements::new(
            Length::new::<kilometer>(a / 1000.0),
            self.eccentricity,
            self.inclination,
            self.raan,
            self.arg_perigee,
            self.mean_anomaly,
        )
    }

    /// Propagate the TLE to a future epoch using simplified two-body dynamics.
    ///
    /// **Note**: This is a simplified propagation. For accurate satellite tracking,
    /// use a proper SGP4/SDP4 implementation that accounts for perturbations.
    ///
    /// # Errors
    ///
    /// Returns error if epoch is outside valid range or if numerical issues occur.
    pub fn propagate_to(&self, target_epoch: DateTime<Utc>) -> CelestialResult<Coordinate<Icrs>> {
        let elements = self.to_keplerian();
        let propagated = elements.propagate_to(target_epoch, self.epoch);
        let (position, _velocity) = propagated.to_state_vectors();
        Ok(position)
    }
}

/// Convert TLE epoch (year + day-of-year) to DateTime.
fn tle_epoch_to_datetime(year: i32, day_of_year: f64) -> CelestialResult<DateTime<Utc>> {
    let jan1 = Utc
        .with_ymd_and_hms(year, 1, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| CelestialError::InvalidCoordinates { 
            reason: "Invalid year".into() 
        })?;

    let whole_days = day_of_year.floor() as i64 - 1;
    let fractional_day = day_of_year - day_of_year.floor();
    let seconds = (fractional_day * 86400.0).round() as i64;

    let epoch = jan1 + Duration::days(whole_days) + Duration::seconds(seconds);

    Ok(epoch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn parse_iss_tle() {
        let line1 = "1 25544U 98067A   20206.18539600  .00001406  00000-0  33518-4 0  9992";
        let line2 = "2 25544  51.6461 339.8014 0001473  94.8340 265.2864 15.49309432236008";

        let tle = TleElements::from_lines(line1, line2).unwrap();

        assert_eq!(tle.catalog_number(), 25544);
        assert!((tle.inclination().get::<degree>() - 51.6461).abs() < 0.001);
        assert!((tle.eccentricity() - 0.0001473).abs() < 0.000001);
    }

    #[test]
    fn tle_epoch_conversion() {
        let dt = tle_epoch_to_datetime(2020, 206.18539600).unwrap();

        assert_eq!(dt.year(), 2020);
        // Day 206 is July 24
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 24);
    }

    #[test]
    fn tle_to_keplerian() {
        let line1 = "1 25544U 98067A   20206.18539600  .00001406  00000-0  33518-4 0  9992";
        let line2 = "2 25544  51.6461 339.8014 0001473  94.8340 265.2864 15.49309432236008";

        let tle = TleElements::from_lines(line1, line2).unwrap();
        let kep = tle.to_keplerian();

        // ISS orbit should be around 6700-6800 km
        let a_km = kep.semi_major_axis.get::<kilometer>();
        assert!(a_km > 6700.0 && a_km < 6900.0);
    }
}
