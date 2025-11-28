//! Builder pattern support for celestial coordinates.
//!
//! Provides `Components` types for constructing celestial coordinates
//! in a way that matches sguaba's builder API.

use super::frames::CelestialComponents;

/// Components for building ICRS coordinates.
pub mod icrs {
    use super::*;
    use uom::si::f64::Length;

    /// Cartesian components for ICRS frame.
    pub struct Components {
        /// X component (towards vernal equinox)
        pub x: Length,
        /// Y component (90° east in equatorial plane)
        pub y: Length,
        /// Z component (towards North Celestial Pole)
        pub z: Length,
    }

    impl From<Components> for CelestialComponents {
        fn from(c: Components) -> Self {
            CelestialComponents {
                x: c.x,
                y: c.y,
                z: c.z,
            }
        }
    }
}

/// Components for building MCI coordinates.
pub mod mci {
    use super::*;
    use uom::si::f64::Length;

    /// Cartesian components for MCI (Moon-Centered Inertial) frame.
    pub struct Components {
        /// X component (towards mean lunar sub-Earth point)
        pub x: Length,
        /// Y component (90° east along lunar equator)
        pub y: Length,
        /// Z component (towards lunar North pole)
        pub z: Length,
    }

    impl From<Components> for CelestialComponents {
        fn from(c: Components) -> Self {
            CelestialComponents {
                x: c.x,
                y: c.y,
                z: c.z,
            }
        }
    }
}

/// Components for building GCRF coordinates.
pub mod gcrf {
    use super::*;
    use uom::si::f64::Length;

    /// Cartesian components for GCRF frame.
    pub struct Components {
        /// X component
        pub x: Length,
        /// Y component
        pub y: Length,
        /// Z component
        pub z: Length,
    }

    impl From<Components> for CelestialComponents {
        fn from(c: Components) -> Self {
            CelestialComponents {
                x: c.x,
                y: c.y,
                z: c.z,
            }
        }
    }
}
