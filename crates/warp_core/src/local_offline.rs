//! Local-first / offline mode for this AGPL client fork.
//!
//! When enabled, the desktop client does not require a Warp account and does
//! not contact Warp-hosted auth, billing, Drive, or Oz servers. Cloud features
//! that depend on those proprietary backends are hidden or no-op.
//!
//! Control:
//! - Compile-time: Cargo feature `local_offline`
//! - Runtime: `WARP_LOCAL_OFFLINE=1` (on) or `WARP_LOCAL_OFFLINE=0` (off)
//!
//! The OSS channel of this fork defaults to on unless the env var disables it.
//! This module is safe to call before feature flags are initialized.

/// Environment variable that overrides local-offline mode.
pub const ENV_VAR: &str = "WARP_LOCAL_OFFLINE";

/// Small AGPL-safe product label for About, window title, and channel strings.
///
/// Distinguishes this local-first fork from official Warp builds without
/// stripping trademarks or implying an official Warp release.
pub const PRODUCT_LABEL: &str = "Warp OSS local";

/// Loopback dead-end used when local-offline mode must not reach production.
pub const DEAD_END_HTTP_ROOT: &str = "http://127.0.0.1:0";

/// Loopback dead-end WebSocket URL for RTC / GraphQL subscriptions.
pub const DEAD_END_WS_URL: &str = "ws://127.0.0.1:0/graphql/v2";

/// Parses a `WARP_LOCAL_OFFLINE` value. Returns `None` for unrecognized input.
pub fn parse_env_value(value: &str) -> Option<bool> {
    match value.trim() {
        v if v.eq_ignore_ascii_case("1")
            || v.eq_ignore_ascii_case("true")
            || v.eq_ignore_ascii_case("on")
            || v.eq_ignore_ascii_case("yes") =>
        {
            Some(true)
        }
        v if v.eq_ignore_ascii_case("0")
            || v.eq_ignore_ascii_case("false")
            || v.eq_ignore_ascii_case("off")
            || v.eq_ignore_ascii_case("no") =>
        {
            Some(false)
        }
        _ => None,
    }
}

/// Explicit env override, if the variable is set to a recognized value.
pub fn env_override() -> Option<bool> {
    std::env::var(ENV_VAR)
        .ok()
        .as_deref()
        .and_then(parse_env_value)
}

/// Whether local-offline mode is requested via env or the compile-time feature.
///
/// Does not consult [`crate::features::FeatureFlag`] — safe before flag init.
pub fn is_compiled_or_env_enabled() -> bool {
    if let Some(enabled) = env_override() {
        return enabled;
    }
    cfg!(feature = "local_offline")
}

/// Default for the OSS channel of this fork: on unless explicitly disabled.
pub fn default_enabled_for_oss() -> bool {
    env_override().unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_env_value_recognizes_truthy_and_falsey() {
        for value in ["1", "true", "TRUE", "on", "Yes"] {
            assert_eq!(parse_env_value(value), Some(true), "{value}");
        }
        for value in ["0", "false", "FALSE", "off", "No"] {
            assert_eq!(parse_env_value(value), Some(false), "{value}");
        }
        assert_eq!(parse_env_value(""), None);
        assert_eq!(parse_env_value("maybe"), None);
    }

    #[test]
    fn product_label_is_distinguishable_and_not_deceptive() {
        assert!(PRODUCT_LABEL.contains("OSS"));
        assert!(PRODUCT_LABEL.contains("local"));
        assert!(!PRODUCT_LABEL.eq_ignore_ascii_case("Warp"));
    }

    #[test]
    fn dead_end_urls_are_loopback_and_unroutable() {
        assert!(DEAD_END_HTTP_ROOT.starts_with("http://127.0.0.1:0"));
        assert!(DEAD_END_WS_URL.starts_with("ws://127.0.0.1:0"));
    }
}
