//! Time-tagged coordinate types.

use chrono::{DateTime, Utc};

use sguaba::{Coordinate, CoordinateSystem, Vector};

/// Velocity vector type (meters per second)
pub type VelocityVector<S> = Vector<S, typenum::N1>;

/// A coordinate with an associated timestamp (epoch).
///
/// This type represents a position at a specific moment in time, which is
/// essential for celestial mechanics where frame orientations change over time.
///
/// # Example
///
/// ```no_run
/// # #[cfg(feature = "celestial")] {
/// use crate::{Icrs, TimedCoordinate};
/// use sguaba::Coordinate;
/// use chrono::Utc;
/// use uom::si::f64::Length;
/// use uom::si::length::kilometer;
///
/// #[allow(deprecated)]
/// let position = Coordinate::<Icrs>::from_cartesian(
///     Length::new::<kilometer>(7000.0),
///     Length::new::<kilometer>(0.0),
///     Length::new::<kilometer>(0.0),
/// );
///
/// let timed_pos = TimedCoordinate::new(position, Utc::now());
/// println!("Position at epoch: {:?}", timed_pos.epoch());
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimedCoordinate<S: CoordinateSystem> {
    /// The position coordinate
    position: Coordinate<S>,
    /// The epoch (time) at which this position is valid
    epoch: DateTime<Utc>,
}

impl<S: CoordinateSystem> TimedCoordinate<S> {
    /// Create a new time-tagged coordinate.
    #[must_use]
    pub const fn new(position: Coordinate<S>, epoch: DateTime<Utc>) -> Self {
        Self { position, epoch }
    }

    /// Get the position coordinate.
    #[must_use]
    pub const fn position(&self) -> &Coordinate<S> {
        &self.position
    }

    /// Get the epoch.
    #[must_use]
    pub const fn epoch(&self) -> DateTime<Utc> {
        self.epoch
    }

    /// Destructure into position and epoch.
    #[must_use]
    pub fn into_parts(self) -> (Coordinate<S>, DateTime<Utc>) {
        (self.position, self.epoch)
    }

    /// Update the position while keeping the same epoch.
    #[must_use]
    pub const fn with_position(self, position: Coordinate<S>) -> Self {
        Self {
            position,
            epoch: self.epoch,
        }
    }

    /// Update the epoch while keeping the same position.
    #[must_use]
    pub const fn with_epoch(self, epoch: DateTime<Utc>) -> Self {
        Self {
            position: self.position,
            epoch,
        }
    }
}

/// A complete ephemeris state: position, velocity, and epoch.
///
/// This type represents a full state vector for orbital mechanics,
/// combining position and velocity at a specific moment in time.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "celestial")] {
/// use crate::{Icrs, EphemerisState};
/// use sguaba::{Coordinate, Vector};
/// use chrono::Utc;
/// use uom::si::f64::{Length, Velocity};
/// use uom::si::length::kilometer;
/// use uom::si::velocity::meter_per_second;
///
/// let position = Coordinate::<Icrs>::from_cartesian(
///     Length::new::<kilometer>(7000.0),
///     Length::new::<kilometer>(0.0),
///     Length::new::<kilometer>(0.0),
/// );
///
/// let velocity = Vector::from_cartesian(
///     Velocity::new::<meter_per_second>(0.0),
///     Velocity::new::<meter_per_second>(7546.0),
///     Velocity::new::<meter_per_second>(0.0),
/// );
///
/// let state = EphemerisState::new(position, velocity, Utc::now());
/// # }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EphemerisState<S: CoordinateSystem> {
    /// The position coordinate
    position: Coordinate<S>,
    /// The velocity vector
    velocity: VelocityVector<S>,
    /// The epoch (time) at which this state is valid
    epoch: DateTime<Utc>,
}

impl<S: CoordinateSystem> EphemerisState<S> {
    /// Create a new ephemeris state.
    #[must_use]
    pub const fn new(position: Coordinate<S>, velocity: VelocityVector<S>, epoch: DateTime<Utc>) -> Self {
        Self { position, velocity, epoch }
    }

    /// Get the position coordinate.
    #[must_use]
    pub const fn position(&self) -> &Coordinate<S> {
        &self.position
    }

    /// Get the velocity vector.
    #[must_use]
    pub const fn velocity(&self) -> &VelocityVector<S> {
        &self.velocity
    }

    /// Get the epoch.
    #[must_use]
    pub const fn epoch(&self) -> DateTime<Utc> {
        self.epoch
    }

    /// Destructure into position, velocity, and epoch.
    #[must_use]
    pub fn into_parts(self) -> (Coordinate<S>, VelocityVector<S>, DateTime<Utc>) {
        (self.position, self.velocity, self.epoch)
    }

    /// Update the position while keeping velocity and epoch.
    #[must_use]
    pub const fn with_position(self, position: Coordinate<S>) -> Self {
        Self { position, velocity: self.velocity, epoch: self.epoch }
    }

    /// Update the velocity while keeping position and epoch.
    #[must_use]
    pub const fn with_velocity(self, velocity: VelocityVector<S>) -> Self {
        Self { position: self.position, velocity, epoch: self.epoch }
    }

    /// Update the epoch while keeping position and velocity.
    #[must_use]
    pub const fn with_epoch(self, epoch: DateTime<Utc>) -> Self {
        Self { position: self.position, velocity: self.velocity, epoch }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Icrs;
    use uom::si::f64::Length;
    use uom::si::length::meter;

    #[test]
    fn timed_coordinate_creation() {
        #[allow(deprecated)]
        let pos = Coordinate::<Icrs>::from_cartesian(
            Length::new::<meter>(1000.0),
            Length::new::<meter>(2000.0),
            Length::new::<meter>(3000.0),
        );
        let time = Utc::now();
        
        let timed = TimedCoordinate::new(pos, time);
        
        assert_eq!(timed.position(), &pos);
        assert_eq!(timed.epoch(), time);
    }

    #[test]
    fn timed_coordinate_with_methods() {
        #[allow(deprecated)]
        let pos1 = Coordinate::<Icrs>::from_cartesian(
            Length::new::<meter>(1000.0),
            Length::new::<meter>(0.0),
            Length::new::<meter>(0.0),
        );
        #[allow(deprecated)]
        let pos2 = Coordinate::<Icrs>::from_cartesian(
            Length::new::<meter>(2000.0),
            Length::new::<meter>(0.0),
            Length::new::<meter>(0.0),
        );
        
        let time1 = Utc::now();
        let time2 = time1 + chrono::Duration::hours(1);
        
        let timed = TimedCoordinate::new(pos1, time1);
        let updated = timed.with_position(pos2).with_epoch(time2);
        
        assert_eq!(updated.position(), &pos2);
        assert_eq!(updated.epoch(), time2);
    }
}
