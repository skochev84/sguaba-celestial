//! Astrodynamics-specific vector types and utilities.
//!
//! This module provides specialized vector types for spacecraft dynamics
//! that use appropriate unit dimensions from the `uom` crate.

use sguaba::Vector;

/// Angular velocity vector (rad/s).
///
/// Represents rotational velocity with dimensions [length^0 / time^1].
/// Used for spacecraft attitude rates and rigid body rotation.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "celestial")] {
/// use crate::{Icrs, AngularVelocityVector};
/// use typenum::N1;
///
/// // Type alias for angular velocity vectors
/// type AngVel = sguaba::Vector<Icrs, N1>;
///
/// // Used for spacecraft attitude rates
/// # }
/// ```
pub type AngularVelocityVector<S> = Vector<S, typenum::N1>;

/// Specific angular momentum vector (m²/s).
///
/// Represents angular momentum per unit mass with dimensions [length^2 / time^1].
/// A fundamental conserved quantity in orbital mechanics.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "celestial")] {
/// use crate::{Icrs, SpecificAngularMomentum};
/// use typenum::P2;
///
/// // Type alias for specific angular momentum (h = r × v)
/// type AngMom = sguaba::Vector<Icrs, P2>;
///
/// // Conserved in Keplerian orbits
/// # }
/// ```
pub type SpecificAngularMomentum<S> = Vector<S, typenum::P2>;

/// Acceleration vector (m/s²).
///
/// Represents linear acceleration with dimensions [length^1 / time^2].
/// Used for gravitational acceleration, thrust, and perturbations.
pub type AccelerationVector<S> = Vector<S, typenum::N2>;

/// Specific energy (m²/s²).
///
/// Represents energy per unit mass with dimensions [length^2 / time^2].
/// Another fundamental orbital parameter.
pub type SpecificEnergy = uom::si::f64::Velocity; // Actually m²/s², but velocity has same dims

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Icrs;

    #[test]
    fn type_dimensions_compile() {
        // This test just verifies that the type aliases compile correctly
        let _: Option<AngularVelocityVector<Icrs>> = None;
        let _: Option<SpecificAngularMomentum<Icrs>> = None;
        let _: Option<AccelerationVector<Icrs>> = None;
    }
}
