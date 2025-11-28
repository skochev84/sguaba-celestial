//! Helper module for creating rotations from quaternions when the Rotation fields are private.
//!
//! This module provides a workaround for constructing Rotation<From, To> instances
//! from nalgebra quaternions when we cannot access the private fields of sguaba::math::Rotation.

use nalgebra::{Quaternion, Unit};
use sguaba::math::Rotation;

type UnitQuaternion = Unit<Quaternion<f64>>;

/// Create a Rotation<From, To> from a UnitQuaternion using Euler angles as an intermediary.
///
/// This is a workaround for the fact that Rotation's fields are private and there's no
/// public constructor that takes a quaternion directly.
///
/// # Safety
///
/// This has the same safety requirements as Rotation itself - you must ensure that
/// the quaternion represents the correct rotation from From to To.
pub unsafe fn rotation_from_quaternion<From, To>(quat: UnitQuaternion) -> Rotation<From, To> {
    // Extract Euler angles from the quaternion
    let (roll, pitch, yaw) = quat.euler_angles();
    
    // Use the Tait-Bryan builder to reconstruct the rotation
    // Note: euler_angles() returns in (roll, pitch, yaw) order
    // but tait_bryan_builder expects (yaw, pitch, roll)
    use uom::si::f64::Angle;
    use uom::si::angle::radian;
    
    Rotation::tait_bryan_builder()
        .yaw(Angle::new::<radian>(yaw))
        .pitch(Angle::new::<radian>(pitch))
        .roll(Angle::new::<radian>(roll))
        .build()
}
