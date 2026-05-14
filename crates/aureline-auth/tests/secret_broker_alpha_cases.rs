//! Fixture-driven coverage for the secret-broker alpha.
//!
//! These tests parse the protected fixtures under
//! `/fixtures/auth/secret_broker_alpha`, validate every claimed row, project
//! the UI/status surface rows, and project metadata-only support exports that
//! omit runtime handle ids and raw secret material.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    AffectedCapabilityClass, ContinuityStateClass, RecoveryActionClass, SecretBrokerAlphaPacket,
    SecretBrokerDenialReason, SecretBrokerRowError, SecretReferenceMode, TrustStoreClass,
    UnlockStateClass,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/auth/secret_broker_alpha")
}

fn load_string(file_name: &str) -> String {
    let path = fixture_dir().join(file_name);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()))
}

fn load_packet(file_name: &str) -> SecretBrokerAlphaPacket {
    serde_json::from_str(&load_string(file_name))
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a packet: {err}"))
}

#[test]
fn baseline_fixture_proves_handle_session_only_and_delegated_rows() {
    let packet = load_packet("baseline_handle_session_delegated.json");
    packet.validate().expect("baseline packet validates");

    let modes: Vec<SecretReferenceMode> = packet
        .rows
        .iter()
        .map(|row| row.secret_ref.reference_mode)
        .collect();
    assert!(modes.contains(&SecretReferenceMode::Handle));
    assert!(modes.contains(&SecretReferenceMode::SessionOnly));
    assert!(modes.contains(&SecretReferenceMode::Delegated));

    let surface_rows = packet.surface_rows();
    assert_eq!(surface_rows.len(), 3);
    assert!(surface_rows.iter().any(|row| {
        row.capability_class == AffectedCapabilityClass::RegistryAuth
            && row.reference_mode == SecretReferenceMode::Handle
            && row.storage_mode_token == "system_credential_store"
    }));
    assert!(surface_rows.iter().any(|row| {
        row.capability_class == AffectedCapabilityClass::ProviderReconnect
            && row.reference_mode == SecretReferenceMode::SessionOnly
            && row.continuity_state_token == "degraded_session_only_visible"
    }));
    assert!(surface_rows.iter().any(|row| {
        row.capability_class == AffectedCapabilityClass::TunnelReuse
            && row.reference_mode == SecretReferenceMode::Delegated
            && row.projection_mode_token == "token_exchange"
    }));

    let export = packet.support_export("secret-support-export:baseline", "2026-05-14T00:00:00Z");
    assert!(export.redaction_safe());
    assert_eq!(export.rows.len(), 3);
    let encoded = serde_json::to_string(&export).expect("serialize support export");
    assert!(!encoded.contains("credential-handle:"));
    assert!(!encoded.contains("session-secret:"));
    assert!(!encoded.contains("delegated-credential:"));
}

#[test]
fn failure_fixture_surfaces_specific_paused_states_and_recovery_actions() {
    let packet = load_packet("failure_locked_unavailable_trust_changed.json");
    packet.validate().expect("failure packet validates");
    assert!(packet.has_visible_continuity_result());

    let locked = packet
        .rows
        .iter()
        .find(|row| row.capability_class == AffectedCapabilityClass::RegistryAuth)
        .expect("registry auth row");
    assert_eq!(
        locked.continuity.continuity_state,
        ContinuityStateClass::PausedCredentialStoreLocked
    );
    assert_eq!(locked.storage.unlock_state, UnlockStateClass::Locked);
    assert_eq!(
        locked.continuity.denial_reason,
        Some(SecretBrokerDenialReason::TrustStoreLocked)
    );
    assert!(locked
        .continuity
        .recovery_actions
        .contains(&RecoveryActionClass::RetryAfterCredentialStoreUnlock));

    let unavailable = packet
        .rows
        .iter()
        .find(|row| row.capability_class == AffectedCapabilityClass::ManagedSignInRefresh)
        .expect("managed refresh row");
    assert_eq!(
        unavailable.continuity.continuity_state,
        ContinuityStateClass::PausedCredentialStoreUnavailable
    );
    assert_eq!(
        unavailable.storage.unlock_state,
        UnlockStateClass::Unavailable
    );
    assert!(unavailable
        .continuity
        .recovery_actions
        .contains(&RecoveryActionClass::ReauthenticateInSystemBrowser));

    let trust_changed = packet
        .rows
        .iter()
        .find(|row| row.capability_class == AffectedCapabilityClass::DatabaseAttach)
        .expect("database attach row");
    assert_eq!(
        trust_changed.continuity.continuity_state,
        ContinuityStateClass::PausedTrustStoreChanged
    );
    assert_eq!(
        trust_changed.continuity.denial_reason,
        Some(SecretBrokerDenialReason::TrustStateDowngraded)
    );
    assert!(trust_changed
        .continuity
        .recovery_actions
        .contains(&RecoveryActionClass::RebindAfterTrustStoreChange));

    for row in &packet.rows {
        assert!(row.continuity.local_work_continues);
        assert!(row.continuity.credentialed_actions_paused);
        assert!(!row.storage.plaintext_persistence_allowed);
        assert!(!row.storage.silent_in_memory_promotion_allowed);
        assert!(!row.storage.stale_ticket_reuse_allowed);
        assert!(!row.storage.raw_secret_material_present);
    }
}

#[test]
fn support_export_for_failure_fixture_is_metadata_only() {
    let packet = load_packet("failure_locked_unavailable_trust_changed.json");
    let export = packet.support_export("secret-support-export:failure", "2026-05-14T00:01:00Z");

    assert!(export.redaction_safe());
    assert!(export
        .rows
        .iter()
        .all(|row| !row.raw_secret_values_exported && !row.raw_handle_ids_exported));
    assert!(export
        .rows
        .iter()
        .any(|row| row.capability_class_token == "registry_auth"
            && row.denial_reason_token.as_deref() == Some("trust_store_locked")));
    assert!(export.rows.iter().any(|row| {
        row.capability_class_token == "managed_sign_in_refresh"
            && row.denial_reason_token.as_deref() == Some("trust_store_unavailable")
    }));
    assert!(export.rows.iter().any(|row| {
        row.capability_class_token == "database_attach"
            && row.denial_reason_token.as_deref() == Some("trust_state_downgraded")
    }));

    let encoded = serde_json::to_string(&export).expect("serialize support export");
    assert!(!encoded.contains("credential-handle:"));
    assert!(!encoded.contains("provider_refresh_token_value"));
    assert!(!encoded.contains("database_password_value"));
    assert!(!encoded.contains("generic auth failure"));
}

#[test]
fn packet_helpers_create_actionable_local_safe_failure_results() {
    let mut packet = load_packet("baseline_handle_session_delegated.json");

    let locked = packet.mark_credential_store_locked(
        TrustStoreClass::OsKeychain,
        &[AffectedCapabilityClass::RegistryAuth],
    );
    assert_eq!(locked, 1);
    packet.validate().expect("locked helper output validates");
    let row = packet
        .rows
        .iter()
        .find(|row| row.capability_class == AffectedCapabilityClass::RegistryAuth)
        .expect("registry auth row");
    assert_eq!(
        row.continuity.continuity_state,
        ContinuityStateClass::PausedCredentialStoreLocked
    );
    assert_eq!(row.storage.unlock_state, UnlockStateClass::Locked);
    assert!(row.continuity.local_work_continues);

    let mut packet = load_packet("baseline_handle_session_delegated.json");
    let unavailable = packet.mark_credential_store_unavailable(
        TrustStoreClass::OsKeychain,
        &[AffectedCapabilityClass::RegistryAuth],
    );
    assert_eq!(unavailable, 1);
    packet
        .validate()
        .expect("unavailable helper output validates");
    let row = packet
        .rows
        .iter()
        .find(|row| row.capability_class == AffectedCapabilityClass::RegistryAuth)
        .expect("registry auth row");
    assert_eq!(
        row.continuity.continuity_state,
        ContinuityStateClass::PausedCredentialStoreUnavailable
    );
    assert_eq!(row.storage.unlock_state, UnlockStateClass::Unavailable);
    assert!(row.continuity.local_work_continues);
}

#[test]
fn guardrails_reject_plaintext_persistence_silent_promotion_and_stale_reuse() {
    let packet = load_packet("baseline_handle_session_delegated.json");
    let mut row = packet.rows[0].clone();
    row.storage.plaintext_persistence_allowed = true;
    assert!(matches!(
        row.validate(),
        Err(SecretBrokerRowError::PlaintextPersistenceAllowed { .. })
    ));

    let mut row = packet.rows[0].clone();
    row.storage.silent_in_memory_promotion_allowed = true;
    assert!(matches!(
        row.validate(),
        Err(SecretBrokerRowError::SilentInMemoryPromotionAllowed { .. })
    ));

    let mut row = packet.rows[0].clone();
    row.storage.stale_ticket_reuse_allowed = true;
    assert!(matches!(
        row.validate(),
        Err(SecretBrokerRowError::StaleTicketReuseAllowed { .. })
    ));
}

#[test]
fn every_secret_broker_alpha_fixture_parses_and_validates() {
    let dir = fixture_dir();
    let mut sweep_count = 0;
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let entry = entry.expect("readable directory entry");
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("json") {
            continue;
        }
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let packet: SecretBrokerAlphaPacket = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        packet
            .validate()
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", path.display()));
        sweep_count += 1;
    }
    assert_eq!(sweep_count, 2);
}
