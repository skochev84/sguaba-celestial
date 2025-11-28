//! Transformation functions for celestial reference frames.
//!
//! This module provides free functions for creating transforms between celestial frames.
//! These are the standalone equivalents of the impl methods on RigidBodyTransform.

use chrono::{DateTime, Utc};

use sguaba::math::RigidBodyTransform;
use sguaba::systems::Ecef;
use sguaba::Vector;

use crate::constants::{icrs_to_ecef_rotation, mci_to_icrs_rotation};
use crate::frames::{Icrs, Mci};
use crate::rotation_helper::rotation_from_quaternion;

// =======================================================================================
// TRANSFORM CONSTRUCTORS
// =======================================================================================

/// Constructs the transform from ICRS to ECEF at the specified time.
///
/// This accounts for Earth's precession and rotation, allowing conversion
/// between the inertial celestial frame and Earth-fixed coordinates.
///
/// # Accuracy
///
/// < 30 milliarcseconds (2020-2050) using IAU 2006/2000A precession + ERA.
#[must_use]
pub fn icrs_to_ecef_at(time: DateTime<Utc>) -> RigidBodyTransform<Icrs, Ecef> {
    let quat = icrs_to_ecef_rotation(time);
    unsafe {
        let rotation = rotation_from_quaternion(quat);
        RigidBodyTransform::new(Vector::zero(), rotation)
    }
}

/// Constructs the transform from ECEF to ICRS at the specified time.
///
/// This is the inverse of [icrs_to_ecef_at].
#[must_use]
pub fn ecef_to_icrs_at(time: DateTime<Utc>) -> RigidBodyTransform<Ecef, Icrs> {
    icrs_to_ecef_at(time).inverse()
}

/// Constructs the transform from MCI (Moon-Centered Inertial) to ICRS.
///
/// Uses IAU 2009 lunar orientation constants. This transform is approximately
/// time-independent as it uses mean lunar orientation.
#[must_use]
pub fn mci_to_icrs() -> RigidBodyTransform<Mci, Icrs> {
    let quat = *mci_to_icrs_rotation();
    unsafe {
        let rotation = rotation_from_quaternion(quat);
        RigidBodyTransform::new(Vector::zero(), rotation)
    }
}

/// Constructs the transform from ICRS to MCI (Moon-Centered Inertial).
///
/// This is the inverse of [mci_to_icrs].
#[must_use]
pub fn icrs_to_mci() -> RigidBodyTransform<Icrs, Mci> {
    mci_to_icrs().inverse()
}
