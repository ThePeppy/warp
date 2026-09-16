use super::*;
use crate::settings_view::SettingsSection;

#[test]
fn reports_compile_time_state() {
    assert_eq!(is_enabled(), cfg!(feature = "local_only"));
}

#[test]
fn classifies_cloud_account_settings_sections() {
    assert!(is_cloud_account_settings_section(
        SettingsSection::BillingAndUsage
    ));
    assert!(is_cloud_account_settings_section(SettingsSection::Teams));
    assert!(is_cloud_account_settings_section(
        SettingsSection::Referrals
    ));
    assert!(is_cloud_account_settings_section(
        SettingsSection::SharedBlocks
    ));
    assert!(is_cloud_account_settings_section(
        SettingsSection::WarpDrive
    ));
    assert!(is_cloud_account_settings_section(
        SettingsSection::CloudEnvironments
    ));
    assert!(is_cloud_account_settings_section(
        SettingsSection::OzCloudAPIKeys
    ));
    assert!(is_cloud_account_settings_section(SettingsSection::AI));
    assert!(is_cloud_account_settings_section(
        SettingsSection::WarpAgent
    ));
    assert!(is_cloud_account_settings_section(
        SettingsSection::AgentProfiles
    ));
    assert!(is_cloud_account_settings_section(
        SettingsSection::Knowledge
    ));

    assert!(!is_cloud_account_settings_section(
        SettingsSection::Appearance
    ));
    assert!(!is_cloud_account_settings_section(
        SettingsSection::Features
    ));
    assert!(!is_cloud_account_settings_section(
        SettingsSection::Keybindings
    ));
    assert!(!is_cloud_account_settings_section(SettingsSection::Privacy));
    assert!(!is_cloud_account_settings_section(SettingsSection::About));
    assert!(!is_cloud_account_settings_section(SettingsSection::Account));
    assert!(!is_cloud_account_settings_section(
        SettingsSection::AgentMCPServers
    ));
    assert!(!is_cloud_account_settings_section(
        SettingsSection::ThirdPartyCLIAgents
    ));
    assert!(!is_cloud_account_settings_section(
        SettingsSection::CodeIndexing
    ));
}

#[test]
fn visibility_follows_local_only_flag() {
    if is_enabled() {
        assert!(!is_settings_section_visible(
            SettingsSection::BillingAndUsage
        ));
        assert!(is_settings_section_visible(SettingsSection::Appearance));
        assert!(is_settings_section_visible(SettingsSection::Account));
    } else {
        assert!(is_settings_section_visible(
            SettingsSection::BillingAndUsage
        ));
        assert!(is_settings_section_visible(SettingsSection::Teams));
    }
}

#[test]
fn strips_hosted_cloud_flags_when_enabled() {
    use std::collections::HashSet;

    let mut flags = HashSet::from([
        FeatureFlag::AgentMode,
        FeatureFlag::Ligatures,
        FeatureFlag::SettingsFile,
    ]);
    apply_to_enabled_features(&mut flags);

    if is_enabled() {
        assert!(flags.contains(&FeatureFlag::LocalOnly));
        assert!(flags.contains(&FeatureFlag::SkipFirebaseAnonymousUser));
        assert!(flags.contains(&FeatureFlag::Ligatures));
        assert!(!flags.contains(&FeatureFlag::AgentMode));
    } else {
        assert!(!flags.contains(&FeatureFlag::LocalOnly));
        assert!(flags.contains(&FeatureFlag::AgentMode));
    }
}
