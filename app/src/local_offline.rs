//! Runtime helper for the local-first / offline fork mode.
//!
//! Prefer this over scattering `FeatureFlag::LocalOffline` checks. Safe after
//! [`crate::init_feature_flags`] has run.

use warp_core::features::FeatureFlag;
use warp_core::local_offline as core;

/// Returns whether this process should run as a local-first client.
///
/// Order: `WARP_LOCAL_OFFLINE` env override, then the compile-time feature,
/// then [`FeatureFlag::LocalOffline`].
pub fn is_enabled() -> bool {
    if let Some(enabled) = core::env_override() {
        return enabled;
    }
    cfg!(feature = "local_offline") || FeatureFlag::LocalOffline.is_enabled()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tests_are_not_local_offline_unless_env_or_feature() {
        if core::env_override().is_some() {
            return;
        }
        assert_eq!(is_enabled(), cfg!(feature = "local_offline"));
    }
}
