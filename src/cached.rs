//! Cached coordinate transformations for performance optimization.
//!
//! Provides caching infrastructure for expensive transformations that depend on time.

use sguaba::math::RigidBodyTransform;
use sguaba::CoordinateSystem;
use chrono::{DateTime, Duration, Utc};
use std::sync::{Arc, RwLock};

/// A cached transformation between coordinate systems with epoch-based invalidation.
///
/// This type caches a `RigidBodyTransform` and automatically invalidates it
/// when the epoch changes beyond a tolerance threshold.
///
/// # Example
///
/// ```no_run
/// # #[cfg(feature = "celestial")] {
/// use crate::{Icrs, CachedTransform};
/// use sguaba::systems::Ecef;
/// use chrono::Utc;
///
/// let mut cached = CachedTransform::<Icrs, Ecef>::new(
///     chrono::Duration::seconds(60), // Invalidate after 60 seconds
/// );
///
/// let epoch = Utc::now();
/// // First call computes transform
/// let transform1 = cached.get_or_compute(epoch, |e| {
///     // Expensive computation here
///     # todo!()
/// });
///
/// // Second call with same epoch reuses cached value
/// let transform2 = cached.get_or_compute(epoch, |e| {
///     // This won't be called
///     # todo!()
/// });
/// # }
/// ```
#[derive(Debug)]
pub struct CachedTransform<From: CoordinateSystem, To: CoordinateSystem> {
    cached: Arc<RwLock<Option<CacheEntry<From, To>>>>,
    tolerance: Duration,
}

#[derive(Debug, Clone)]
struct CacheEntry<From: CoordinateSystem, To: CoordinateSystem> {
    transform: RigidBodyTransform<From, To>,
    epoch: DateTime<Utc>,
}

impl<From: CoordinateSystem, To: CoordinateSystem> CachedTransform<From, To> {
    /// Create a new cached transform with the specified time tolerance.
    ///
    /// The cached transform will be invalidated if requested at an epoch
    /// that differs from the cached epoch by more than `tolerance`.
    #[must_use]
    pub fn new(tolerance: Duration) -> Self {
        Self {
            cached: Arc::new(RwLock::new(None)),
            tolerance,
        }
    }

    /// Get the cached transform or compute a new one.
    ///
    /// If the cache is empty or the epoch differs by more than the tolerance,
    /// `compute_fn` will be called to generate a new transform.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The time at which the transform is needed
    /// * `compute_fn` - Function to compute the transform if cache miss
    pub fn get_or_compute<F>(&self, epoch: DateTime<Utc>, compute_fn: F) -> RigidBodyTransform<From, To>
    where
        F: FnOnce(DateTime<Utc>) -> RigidBodyTransform<From, To>,
        From: Clone,
        To: Clone,
    {
        // Try to read from cache
        {
            let cache_read = self.cached.read().unwrap();
            if let Some(entry) = cache_read.as_ref() {
                let time_diff = (epoch - entry.epoch).num_seconds().abs();
                if time_diff <= self.tolerance.num_seconds() {
                    return entry.transform;
                }
            }
        }

        // Cache miss - compute new transform
        let new_transform = compute_fn(epoch);

        // Update cache
        {
            let mut cache_write = self.cached.write().unwrap();
            *cache_write = Some(CacheEntry {
                transform: new_transform,
                epoch,
            });
        }

        new_transform
    }

    /// Clear the cached transform.
    pub fn invalidate(&self) {
        let mut cache_write = self.cached.write().unwrap();
        *cache_write = None;
    }

    /// Check if the cache contains a valid entry for the given epoch.
    #[must_use]
    pub fn is_valid_for(&self, epoch: DateTime<Utc>) -> bool {
        let cache_read = self.cached.read().unwrap();
        if let Some(entry) = cache_read.as_ref() {
            let time_diff = (epoch - entry.epoch).num_seconds().abs();
            time_diff <= self.tolerance.num_seconds()
        } else {
            false
        }
    }
}

impl<From: CoordinateSystem, To: CoordinateSystem> Clone for CachedTransform<From, To> {
    fn clone(&self) -> Self {
        Self {
            cached: Arc::clone(&self.cached),
            tolerance: self.tolerance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Icrs, Mci};
    use sguaba::math::RigidBodyTransform;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn cache_hit_reuses_transform() {
        let cached = CachedTransform::<Icrs, Mci>::new(Duration::seconds(60));
        let epoch = Utc::now();

        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = Arc::clone(&call_count);

        // First call
        let _t1 = cached.get_or_compute(epoch, |_| {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
            // SAFETY: Test identity transform
            unsafe { RigidBodyTransform::identity() }
        });

        let call_count_clone = Arc::clone(&call_count);

        // Second call with same epoch - should reuse
        let _t2 = cached.get_or_compute(epoch, |_| {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
            unsafe { RigidBodyTransform::identity() }
        });

        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn cache_miss_on_time_change() {
        let cached = CachedTransform::<Icrs, Mci>::new(Duration::seconds(10));
        let epoch1 = Utc::now();
        let epoch2 = epoch1 + Duration::seconds(20);

        let call_count = Arc::new(AtomicUsize::new(0));

        // First call
        {
            let call_count_clone = Arc::clone(&call_count);
            let _t1 = cached.get_or_compute(epoch1, |_| {
                call_count_clone.fetch_add(1, Ordering::SeqCst);
                unsafe { RigidBodyTransform::identity() }
            });
        }

        // Second call with different epoch - should recompute
        {
            let call_count_clone = Arc::clone(&call_count);
            let _t2 = cached.get_or_compute(epoch2, |_| {
                call_count_clone.fetch_add(1, Ordering::SeqCst);
                unsafe { RigidBodyTransform::identity() }
            });
        }

        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn invalidate_clears_cache() {
        let cached = CachedTransform::<Icrs, Mci>::new(Duration::seconds(60));
        let epoch = Utc::now();

        // Populate cache
        let _ = cached.get_or_compute(epoch, |_| unsafe { RigidBodyTransform::identity() });
        assert!(cached.is_valid_for(epoch));

        // Invalidate
        cached.invalidate();
        assert!(!cached.is_valid_for(epoch));
    }
}
