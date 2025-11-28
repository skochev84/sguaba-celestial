//! Optional celestial reference frames for space applications.
//!
//! This module provides support for celestial coordinate systems commonly used in
//! astrodynamics and space navigation:
//!
//! - [`Icrs`]: International Celestial Reference System (Earth-centered inertial)
//! - [`Mci`]: Moon-Centered Inertial frame
//! - [`Gcrf`]: Geocentric Celestial Reference Frame (equivalent to ICRS)
//! - [`Eme2000`]: Earth Mean Equator and Equinox of J2000
//! - [`Ecliptic`]: Ecliptic coordinate system
//!
//! # Features
//!
//! Celestial frames are opt-in via the `celestial` feature flag to avoid unnecessary
//! dependencies for terrestrial-only applications.
//!
//! # Accuracy
//!
//! Transform accuracies for the epoch range 2020-2050:
//!
//! - **ICRS ↔ ECEF**: < 30 milliarcseconds (mas) using IAU 2006/2000A precession + ERA
//! - **ICRS ↔ ECEF (with nutation)**: < 1 mas using IAU 2000B nutation model
//! - **MCI ↔ ICRS**: Arcsecond-level using IAU 2009 lunar orientation constants
//!
//! # Limitations
//!
//! - Polar motion not included (default implementation returns identity)
//! - UT1-UTC correction not applied (UTC treated as UT1)
//! - Lunar libration not included in MCI frame
//!
//! # Coordinate System Selection
//!
//! - **ICRS**: General-purpose inertial frame for deep space, satellites
//! - **GCRF**: Explicitly Earth-centered, common in satellite orbit propagation
//! - **MCI**: Lunar missions, Moon-centered orbits
//! - **ECEF**: Ground stations, Earth-fixed tracking
//! - **Ecliptic**: Solar system dynamics, planetary missions
//!
//! # Examples
//!
//! Transform between celestial and Earth-fixed frames:
//!
//! ```no_run
//! # #[cfg(feature = "celestial")] {
//! use sguaba::{celestial::Icrs, systems::Ecef, math::RigidBodyTransform};
//! use chrono::Utc;
//!
//! // Transform to ECEF at current time
//! let now = Utc::now();
//! let icrs_to_ecef = RigidBodyTransform::icrs_to_ecef_at(now);
//! // let sat_ecef = icrs_to_ecef.transform(sat_icrs);
//! # }
//! ```
//!
//! Convert between Right Ascension/Declination and Cartesian:
//!
//! ```no_run
//! # #[cfg(feature = "celestial")] {
//! use sguaba::{celestial::Icrs, Coordinate};
//! use uom::si::f64::{Angle, Length};
//! use uom::si::angle::degree;
//! use uom::si::length::kilometer;
//!
//! // Create from RA/Dec
//! let pos = Coordinate::<Icrs>::from_ra_dec(
//!     Angle::new::<degree>(45.0),
//!     Angle::new::<degree>(30.0),
//!     Length::new::<kilometer>(7000.0),
//! );
//!
//! // Convert back to RA/Dec
//! let (ra, dec, dist) = pos.to_spherical_celestial();
//! # }
//! ```


mod ext;
pub use ext::*;

mod rotation_helper;

pub mod additional_frames;
pub mod astrodynamics;
pub mod builder;
pub mod cached;
pub mod constants;
pub mod errors;
pub mod frames;
pub mod orbital;
pub mod time_scales;
pub mod timed;
pub mod tle;
pub mod transforms;

pub use additional_frames::{Ecliptic, Eme2000, Gcrf};
pub use astrodynamics::{AccelerationVector, AngularVelocityVector, SpecificAngularMomentum};
pub use cached::CachedTransform;
pub use errors::{CelestialError, CelestialResult};
pub use frames::{CelestialComponents, CelestialConvention, Icrs, Mci};
pub use orbital::KeplerianElements;
pub use timed::{EphemerisState, TimedCoordinate, VelocityVector};
pub use tle::TleElements;

// Re-export commonly used time scale functions
pub use time_scales::{utc_to_tai, utc_to_tdb, utc_to_tt, utc_to_ut1, validate_epoch};

