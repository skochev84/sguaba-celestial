//! Orbital mechanics support for celestial coordinates.
//!
//! This module provides integration between orbital mechanics and the
//! celestial coordinate systems, including Keplerian orbital elements.

use chrono::{DateTime, Utc};
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};

use sguaba::Coordinate;

#[allow(unused_imports)]
use super::constants::{MU_EARTH, J2000_JD, utc_to_julian_date};
use super::frames::Icrs;

/// Keplerian orbital elements.
///
/// These six elements uniquely define an orbit in the two-body problem.
///
/// # Elements
///
/// - **a**: Semi-major axis (size of orbit)
/// - **e**: Eccentricity (shape of orbit, 0 = circular, <1 = elliptical)
/// - **i**: Inclination (tilt of orbital plane relative to reference plane)
/// - **Ω**: Right ascension of ascending node (RAAN)
/// - **ω**: Argument of periapsis
/// - **ν**: True anomaly (position in orbit at epoch)
///
/// # Reference Frame
///
/// These elements are typically defined relative to ICRS or EME2000.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeplerianElements {
    /// Semi-major axis
    pub semi_major_axis: Length,
    
    /// Eccentricity (dimensionless, 0 ≤ e < 1 for elliptical orbits)
    pub eccentricity: f64,
    
    /// Inclination (angle between orbital plane and reference plane)
    pub inclination: Angle,
    
    /// Right ascension of ascending node (RAAN)
    pub raan: Angle,
    
    /// Argument of periapsis
    pub argument_of_periapsis: Angle,
    
    /// True anomaly (angular position in orbit)
    pub true_anomaly: Angle,
    
    /// Gravitational parameter (μ = GM, default is Earth's)
    pub mu: f64,
}

impl Default for KeplerianElements {
    fn default() -> Self {
        Self {
            semi_major_axis: Length::new::<uom::si::length::meter>(7_000_000.0), // ~600 km altitude
            eccentricity: 0.0,
            inclination: Angle::new::<radian>(0.0),
            raan: Angle::new::<radian>(0.0),
            argument_of_periapsis: Angle::new::<radian>(0.0),
            true_anomaly: Angle::new::<radian>(0.0),
            mu: MU_EARTH,
        }
    }
}

impl KeplerianElements {
    /// Create a new set of Keplerian elements with Earth's μ.
    #[must_use]
    pub fn new(
        semi_major_axis: Length,
        eccentricity: f64,
        inclination: Angle,
        raan: Angle,
        argument_of_periapsis: Angle,
        true_anomaly: Angle,
    ) -> Self {
        Self {
            semi_major_axis,
            eccentricity,
            inclination,
            raan,
            argument_of_periapsis,
            true_anomaly,
            mu: MU_EARTH,
        }
    }

    /// Create Keplerian elements with a custom gravitational parameter.
    #[must_use]
    pub fn with_mu(mut self, mu: f64) -> Self {
        self.mu = mu;
        self
    }

    /// Convert to position and velocity in ICRS frame.
    ///
    /// Uses the classical orbital elements to compute Cartesian state vectors.
    ///
    /// # Returns
    ///
    /// `(position, velocity)` tuple in ICRS frame.
    #[must_use]
    pub fn to_state_vectors(&self) -> (Coordinate<Icrs>, [f64; 3]) {
        use uom::si::length::meter;
        
        let a = self.semi_major_axis.get::<meter>();
        let e = self.eccentricity;
        let i = self.inclination.get::<radian>();
        let raan = self.raan.get::<radian>();
        let omega = self.argument_of_periapsis.get::<radian>();
        let nu = self.true_anomaly.get::<radian>();

        // Orbital radius
        let r = a * (1.0 - e * e) / (1.0 + e * nu.cos());

        // Position and velocity in orbital plane (perifocal frame)
        let x_pqw = r * nu.cos();
        let y_pqw = r * nu.sin();
        let z_pqw = 0.0;

        let p = a * (1.0 - e * e);
        let vx_pqw = -(self.mu / p).sqrt() * nu.sin();
        let vy_pqw = (self.mu / p).sqrt() * (e + nu.cos());
        let vz_pqw = 0.0;

        // Rotation matrices
        let (sin_omega, cos_omega) = omega.sin_cos();
        let (sin_i, cos_i) = i.sin_cos();
        let (sin_raan, cos_raan) = raan.sin_cos();

        // Transform to ICRS (3-1-3 rotation sequence)
        let x = (cos_raan * cos_omega - sin_raan * sin_omega * cos_i) * x_pqw
            + (-cos_raan * sin_omega - sin_raan * cos_omega * cos_i) * y_pqw
            + (sin_raan * sin_i) * z_pqw;

        let y = (sin_raan * cos_omega + cos_raan * sin_omega * cos_i) * x_pqw
            + (-sin_raan * sin_omega + cos_raan * cos_omega * cos_i) * y_pqw
            + (-cos_raan * sin_i) * z_pqw;

        let z = (sin_omega * sin_i) * x_pqw
            + (cos_omega * sin_i) * y_pqw
            + (cos_i) * z_pqw;

        let vx = (cos_raan * cos_omega - sin_raan * sin_omega * cos_i) * vx_pqw
            + (-cos_raan * sin_omega - sin_raan * cos_omega * cos_i) * vy_pqw
            + (sin_raan * sin_i) * vz_pqw;

        let vy = (sin_raan * cos_omega + cos_raan * sin_omega * cos_i) * vx_pqw
            + (-sin_raan * sin_omega + cos_raan * cos_omega * cos_i) * vy_pqw
            + (-cos_raan * sin_i) * vz_pqw;

        let vz = (sin_omega * sin_i) * vx_pqw
            + (cos_omega * sin_i) * vy_pqw
            + (cos_i) * vz_pqw;

        #[allow(deprecated)]
        let position = Coordinate::<Icrs>::from_cartesian(
            Length::new::<meter>(x),
            Length::new::<meter>(y),
            Length::new::<meter>(z),
        );

        let velocity = [vx, vy, vz];

        (position, velocity)
    }

    /// Propagate orbit to a new epoch using simple Keplerian motion.
    ///
    /// # Note
    ///
    /// This uses two-body dynamics only (no perturbations). For accurate
    /// long-term propagation, use a numerical integrator with perturbation models.
    #[must_use]
    pub fn propagate_to(&self, target_epoch: DateTime<Utc>, current_epoch: DateTime<Utc>) -> Self {
        use uom::si::length::meter;
        
        let dt = (utc_to_julian_date(target_epoch) - utc_to_julian_date(current_epoch)) * 86400.0; // seconds
        
        let a = self.semi_major_axis.get::<meter>();
        let n = (self.mu / a.powi(3)).sqrt(); // Mean motion (rad/s)
        
        // Mean anomaly change
        let delta_m = n * dt;
        
        // Current mean anomaly (simplified from true anomaly)
        let e = self.eccentricity;
        let nu = self.true_anomaly.get::<radian>();
        let ecc_anomaly = 2.0 * ((nu / 2.0).tan() / ((1.0 + e) / (1.0 - e)).sqrt()).atan();
        let mean_anomaly = ecc_anomaly - e * ecc_anomaly.sin();
        
        // New mean anomaly
        let new_mean_anomaly = mean_anomaly + delta_m;
        
        // Solve Kepler's equation for new eccentric anomaly (Newton-Raphson)
        let mut e_anom = new_mean_anomaly;
        for _ in 0..10 {
            e_anom = e_anom - (e_anom - e * e_anom.sin() - new_mean_anomaly) / (1.0 - e * e_anom.cos());
        }
        
        // New true anomaly
        let new_nu = 2.0 * (((1.0 + e) / (1.0 - e)).sqrt() * (e_anom / 2.0).tan()).atan();
        
        Self {
            true_anomaly: Angle::new::<radian>(new_nu),
            ..*self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::length::kilometer;

    #[test]
    fn circular_orbit_conversion() {
        let elements = KeplerianElements::default();
        let (pos, _vel) = elements.to_state_vectors();
        
        // For circular orbit with zero angles, position should be along X axis
        let distance = pos.distance_from_origin();
        assert!((distance.get::<kilometer>() - 7000.0).abs() < 0.1);
    }

    #[test]
    fn orbit_propagation_changes_true_anomaly() {
        let elements = KeplerianElements::default();
        let epoch1 = Utc::now();
        let epoch2 = epoch1 + chrono::Duration::hours(2);
        
        let propagated = elements.propagate_to(epoch2, epoch1);
        
        // True anomaly should have changed
        assert!(propagated.true_anomaly.get::<radian>() != elements.true_anomaly.get::<radian>());
    }
}
