//! Compile-time local-only / offline mode for this unofficial fork.
//!
//! Enable with `--features local_only` (the `./script/run` and OSS bundle
//! scripts do this automatically). The feature pulls in `skip_login`, which
//! installs an onboarded local test user and fails authenticated Warp cloud
//! requests instead of prompting for a Warp account.

use crate::features::FeatureFlag;
use crate::settings_view::SettingsSection;

/// Returns whether this binary was compiled in local-only mode.
pub const fn is_enabled() -> bool {
    cfg!(feature = "local_only")
}

/// Cloud-account, billing, team, and hosted-AI settings surfaces that should
/// not appear when [`is_enabled`] is true.
pub fn is_cloud_account_settings_section(section: SettingsSection) -> bool {
    matches!(
        section,
        SettingsSection::BillingAndUsage
            | SettingsSection::Teams
            | SettingsSection::Referrals
            | SettingsSection::SharedBlocks
            | SettingsSection::WarpDrive
            | SettingsSection::CloudEnvironments
            | SettingsSection::OzCloudAPIKeys
            | SettingsSection::AI
            | SettingsSection::WarpAgent
            | SettingsSection::AgentProfiles
            | SettingsSection::Knowledge
    )
}

/// Whether a settings section should be shown in this build.
pub fn is_settings_section_visible(section: SettingsSection) -> bool {
    !is_enabled() || !is_cloud_account_settings_section(section)
}

/// Cloud / account FeatureFlags that stay off in local-only builds so hosted
/// AI, billing, and Warp Drive UI degrade instead of blocking the terminal.
pub fn disabled_cloud_flags() -> &'static [FeatureFlag] {
    &[
        FeatureFlag::AgentMode,
        FeatureFlag::AgentOnboarding,
        FeatureFlag::AgentSharedSessions,
        FeatureFlag::CreatingSharedSessions,
        FeatureFlag::ViewingSharedSessions,
        FeatureFlag::CloudMode,
        FeatureFlag::CloudModeFromLocalSession,
        FeatureFlag::CloudModeImageContext,
        FeatureFlag::CloudModeSetupV2,
        FeatureFlag::CloudModeInputV2,
        FeatureFlag::CloudConversations,
        FeatureFlag::CloudEnvironments,
        FeatureFlag::CreateEnvironmentSlashCommand,
        FeatureFlag::UsageBasedPricing,
        FeatureFlag::Autoupdate,
        FeatureFlag::Changelog,
        FeatureFlag::GetStartedTab,
        FeatureFlag::WelcomeTab,
        FeatureFlag::HOAOnboardingFlow,
        FeatureFlag::SharedWithMe,
        FeatureFlag::DriveObjectsAsContext,
        FeatureFlag::CloudObjects,
        FeatureFlag::TeamApiKeys,
        FeatureFlag::FreeUserNoAi,
    ]
}

/// Apply local-only adjustments to the set of enabled feature flags.
pub fn apply_to_enabled_features(flags: &mut std::collections::HashSet<FeatureFlag>) {
    if !is_enabled() {
        return;
    }

    flags.insert(FeatureFlag::LocalOnly);
    flags.insert(FeatureFlag::SkipFirebaseAnonymousUser);
    for flag in disabled_cloud_flags() {
        flags.remove(flag);
    }
}

#[cfg(test)]
#[path = "local_only_tests.rs"]
mod tests;
