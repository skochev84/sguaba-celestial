//! Error types for celestial coordinate operations.

use chrono::{DateTime, Utc};
use std::fmt;

/// Errors that can occur during celestial coordinate operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CelestialError {
    /// Requested epoch is outside the valid range for the transformation.
    EpochOutOfRange {
        /// The requested epoch
        epoch: DateTime<Utc>,
        /// Minimum valid Julian Date
        min_jd: f64,
        /// Maximum valid Julian Date
        max_jd: f64,
    },

    /// Time scale conversion failed.
    TimeScaleConversionFailed {
        /// Description of the conversion failure
        reason: String,
    },

    /// Invalid celestial coordinates (e.g., declination outside [-90°, 90°]).
    InvalidCoordinates {
        /// Description of the invalid coordinates
        reason: String,
    },

    /// Numerical precision issue in coordinate transformation.
    NumericalPrecisionError {
        /// Description of the precision issue
        reason: String,
    },
}

impl fmt::Display for CelestialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EpochOutOfRange { epoch, min_jd, max_jd } => {
                write!(
                    f,
                    "Epoch {} is outside valid range [JD {}, JD {}]",
                    epoch, min_jd, max_jd
                )
            }
            Self::TimeScaleConversionFailed { reason } => {
                write!(f, "Time scale conversion failed: {}", reason)
            }
            Self::InvalidCoordinates { reason } => {
                write!(f, "Invalid celestial coordinates: {}", reason)
            }
            Self::NumericalPrecisionError { reason } => {
                write!(f, "Numerical precision error: {}", reason)
            }
        }
    }
}

impl std::error::Error for CelestialError {}

/// Result type for celestial operations.
pub type CelestialResult<T> = Result<T, CelestialError>;
