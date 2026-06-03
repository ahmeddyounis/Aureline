//! Fixture-driven coverage for stabilized provider account/install-grant
//! registry records.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_provider::{
    audit_stable_registry_packet, seeded_stable_provider_account_install_grant_registry_packet,
    validate_stable_registry_packet, ActionModeClass, RegistryHealthStateClass,
    StableProviderAccountInstallGrantRegistryPacket,
    STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/providers/m4/stabilize-provider-account-install-grant-registry/registry_packet.json")
}

fn load_or_seed_packet() -> StableProviderAccountInstallGrantRegistryPacket {
    let path = fixture_path();
    if path.exists() {
        let text = fs::read_to_string(&path).expect("read stable registry fixture");
        serde_json::from_str(&text).expect("parse stable registry fixture")
    } else {
        seeded_stable_provider_account_install_grant_registry_packet()
    }
}

#[test]
fn seeded_packet_passes_validation() {
    let packet = seeded_stable_provider_account_install_grant_registry_packet();
    assert!(validate_stable_registry_packet(&packet).is_ok());
    assert!(packet.raw_escape_hatches_absent());
}

#[test]
fn packet_covers_required_health_states() {
    let packet = load_or_seed_packet();
    let report = validate_stable_registry_packet(&packet);
    assert!(report.is_ok(), "stable registry fixture failed validation");

    let health_states: Vec<_> = packet
        .provider_accounts
        .iter()
        .map(|a| a.health_state)
        .chain(packet.install_grants.iter().map(|g| g.health_state))
        .collect();

    assert!(health_states.contains(&RegistryHealthStateClass::Healthy));
    assert!(health_states.contains(&RegistryHealthStateClass::DegradedStaleCredentials));
    assert!(health_states.contains(&RegistryHealthStateClass::BlockedPolicyLockedMapping));
}

#[test]
fn packet_covers_required_action_modes() {
    let packet = load_or_seed_packet();
    let action_modes: Vec<_> = packet
        .provider_accounts
        .iter()
        .map(|a| a.action_mode)
        .chain(packet.install_grants.iter().map(|g| g.action_mode))
        .chain(packet.mapping_rows.iter().map(|m| m.action_mode))
        .collect();

    assert!(action_modes.contains(&ActionModeClass::FullEdit));
    assert!(action_modes.contains(&ActionModeClass::CommentOrLink));
    assert!(action_modes.contains(&ActionModeClass::ReadOnly));
    assert!(action_modes.contains(&ActionModeClass::OfflineCaptureOnly));
}

#[test]
fn packet_inspection_matches_counts() {
    let packet = load_or_seed_packet();
    assert_eq!(
        packet.inspection.provider_account_count,
        packet.provider_accounts.len()
    );
    assert_eq!(
        packet.inspection.install_grant_count,
        packet.install_grants.len()
    );
    assert_eq!(
        packet.inspection.mapping_row_count,
        packet.mapping_rows.len()
    );
}

#[test]
fn schema_version_is_stable() {
    let packet = load_or_seed_packet();
    assert_eq!(
        packet.schema_version,
        STABLE_PROVIDER_ACCOUNT_INSTALL_GRANT_REGISTRY_SCHEMA_VERSION
    );
}

#[test]
fn invalid_packet_with_duplicate_ids_fails() {
    let mut packet = seeded_stable_provider_account_install_grant_registry_packet();
    if let Some(first) = packet.provider_accounts.first().cloned() {
        packet.provider_accounts.push(first);
        let result = validate_stable_registry_packet(&packet);
        assert!(
            result.is_err(),
            "duplicate account ids should fail validation"
        );
    }
}

#[test]
fn audit_returns_empty_for_valid_packet() {
    let packet = seeded_stable_provider_account_install_grant_registry_packet();
    let defects = audit_stable_registry_packet(&packet);
    assert!(defects.is_empty());
}
