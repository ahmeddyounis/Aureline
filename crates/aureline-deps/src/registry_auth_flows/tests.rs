use super::*;

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::PackageSurface;

fn packet() -> RegistryAuthFlows {
    current_registry_auth_flows().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, REGISTRY_AUTH_FLOWS_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, REGISTRY_AUTH_FLOWS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        REGISTRY_AUTH_FLOWS_PATH,
        "artifacts/deps/m5/registry-auth-flows.json"
    );
}

#[test]
fn every_row_binds_to_the_frozen_matrix() {
    let packet = packet();
    let matrix = current_m5_package_state_matrix().expect("matrix loads");
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert!(packet.all_bind_matrix());
    for row in &packet.rows {
        assert!(
            matrix.registry_cell(row.frozen_source()).is_some(),
            "row {} source has no frozen cell",
            row.row_id
        );
        for label in row.applicable_labels() {
            assert!(
                matrix.state(label).is_some(),
                "row {} surfaces unbound label {}",
                row.row_id,
                label.as_str()
            );
        }
    }
}

#[test]
fn closed_vocabularies_are_canonical() {
    let packet = packet();
    assert_eq!(
        packet.credential_source_classes,
        CredentialSourceClass::ALL.to_vec()
    );
    assert_eq!(packet.handle_states, HandleState::ALL.to_vec());
    assert_eq!(packet.continuity_states, ContinuityState::ALL.to_vec());
    assert_eq!(packet.degradation_states, DegradationState::ALL.to_vec());
    assert_eq!(
        packet.status_message_classes,
        RegistryStatusMessageClass::ALL.to_vec()
    );
    assert_eq!(packet.auth_action_kinds, AuthActionKind::ALL.to_vec());
}

#[test]
fn browser_and_device_code_are_distinguished() {
    let packet = packet();
    let browser = packet
        .row("raf:private:browser-sso:reachable")
        .expect("browser row");
    let device = packet
        .row("raf:mirror:device-code:awaiting")
        .expect("device row");
    assert!(browser.is_browser_or_device() && !browser.is_device_code());
    assert!(device.is_browser_or_device() && device.is_device_code());
    // Both map to the same frozen auth mode but stay product-distinct.
    assert_eq!(browser.auth_mode, AuthMode::BrowserOrDeviceSignIn);
    assert_eq!(device.auth_mode, AuthMode::BrowserOrDeviceSignIn);
    assert_ne!(browser.credential_source, device.credential_source);
}

#[test]
fn device_code_continuity_requires_sign_in_and_is_not_mutation_ready() {
    let packet = packet();
    let device = packet
        .row("raf:mirror:device-code:awaiting")
        .expect("device row");
    assert_eq!(device.continuity, ContinuityState::AwaitingDeviceCode);
    assert!(device.handle.is_none());
    assert!(device.trust_blocked());
    assert!(!device.mutation_ready());
    assert!(device
        .required_action_kinds()
        .contains(&AuthActionKind::SignInDeviceCode));
    assert!(device
        .offered_action_kinds()
        .contains(&AuthActionKind::SignInDeviceCode));
}

#[test]
fn handles_never_store_a_secret_body() {
    let packet = packet();
    assert!(packet.no_secret_bodies());
    for row in &packet.rows {
        if let Some(handle) = &row.handle {
            assert!(
                !handle.stores_secret_body,
                "row {} stores a body",
                row.row_id
            );
            assert_eq!(
                handle.retention,
                RetentionClass::BrokerResolvedNeverPersisted
            );
            assert!(handle.is_export_safe());
        }
    }
}

#[test]
fn revoked_handle_requires_rebind_and_blocks_trust() {
    let packet = packet();
    let row = packet
        .row("raf:private:keychain:revoked")
        .expect("revoked row");
    assert_eq!(
        row.handle.as_ref().map(|h| h.state),
        Some(HandleState::Revoked)
    );
    assert_eq!(row.reachability, DegradationState::AuthRequired);
    assert!(row.trust_blocked());
    assert!(!row.mutation_ready());
    let req = row.required_action_kinds();
    assert!(req.contains(&AuthActionKind::RebindHandle));
    assert!(req.contains(&AuthActionKind::Revoke));
    assert!(req.contains(&AuthActionKind::SwitchAccount));
}

#[test]
fn degraded_states_never_collapse_into_a_generic_message() {
    let packet = packet();
    assert!(packet.no_generic_collapse());
    for row in &packet.rows {
        assert!(
            row.message_class().is_specific(),
            "row {} renders a generic message",
            row.row_id
        );
    }
    // Each degraded reachability renders its own specific disclosure.
    let offline = packet
        .row("raf:public:anonymous:offline")
        .expect("offline row");
    assert_eq!(
        offline.message_class(),
        RegistryStatusMessageClass::OfflineSnapshotDisclosure
    );
    assert!(offline.reachability.is_degraded());
    let cache = packet
        .row("raf:private:policy-broker:cache-only")
        .expect("cache row");
    assert_eq!(
        cache.message_class(),
        RegistryStatusMessageClass::CacheOnlyDisclosure
    );
}

#[test]
fn no_results_authoritative_is_distinct_from_a_failure() {
    let packet = packet();
    let row = packet
        .row("raf:public:anonymous:no-results")
        .expect("no-results row");
    assert_eq!(row.reachability, DegradationState::NoResultsAuthoritative);
    // It is a specific, non-degraded outcome — not a connection failure.
    assert!(!row.reachability.is_degraded());
    assert!(!row.trust_blocked());
    assert_eq!(
        row.message_class(),
        RegistryStatusMessageClass::NoResultsAuthoritative
    );
}

#[test]
fn policy_blocked_offers_an_exception_and_blocks_trust() {
    let packet = packet();
    let row = packet
        .row("raf:private:browser-sso:policy-blocked")
        .expect("policy-blocked row");
    assert_eq!(row.reachability, DegradationState::PolicyBlocked);
    assert!(row.trust_blocked());
    assert!(!row.mutation_ready());
    assert!(row
        .offered_action_kinds()
        .contains(&AuthActionKind::RequestPolicyException));
}

#[test]
fn stale_mirror_discloses_but_only_blocks_mutation() {
    let packet = packet();
    let row = packet.row("raf:mirror:vault:stale").expect("stale row");
    assert_eq!(row.reachability, DegradationState::MirrorStale);
    // A stale mirror is degraded and must disclose, but does not hard-block trust.
    assert!(row.reachability.is_degraded());
    assert!(row.reachability.must_disclose());
    assert!(!row.trust_blocked());
    // A mutation still cannot proceed against stale metadata.
    assert!(!row.mutation_ready());
}

#[test]
fn reachable_fresh_rows_are_mutation_ready() {
    let packet = packet();
    let row = packet
        .row("raf:public:keychain:reachable")
        .expect("reachable row");
    assert_eq!(row.reachability, DegradationState::ReachableFresh);
    assert!(!row.trust_blocked());
    assert!(row.mutation_ready());
}

#[test]
fn every_offered_action_is_keyboard_complete() {
    let packet = packet();
    assert!(packet.all_keyboard_complete());
    for row in &packet.rows {
        for action in &row.actions {
            assert!(
                action.is_keyboard_complete(),
                "row {} action {} is not keyboard-complete",
                row.row_id,
                action.kind.as_str()
            );
            assert!(action.is_export_safe());
        }
    }
}

#[test]
fn revocable_handles_always_offer_revoke_and_switch() {
    let packet = packet();
    for row in &packet.rows {
        if row.handle.is_some() {
            let offered = row.offered_action_kinds();
            assert!(
                offered.contains(&AuthActionKind::Revoke),
                "row {} omits revoke",
                row.row_id
            );
            assert!(
                offered.contains(&AuthActionKind::SwitchAccount),
                "row {} omits switch-account",
                row.row_id
            );
        }
    }
}

#[test]
fn one_current_profile_per_registry_source() {
    let packet = packet();
    let mut seen = BTreeSet::new();
    for row in &packet.rows {
        if row.profile.is_current {
            let key = (row.source_class.as_str(), row.redacted_source_label.clone());
            assert!(
                seen.insert(key),
                "more than one current profile for source of row {}",
                row.row_id
            );
        }
    }
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert!(projection.all_consistent);
    assert!(projection.no_generic_collapse);
    assert!(projection.no_secret_bodies);
    assert!(projection.all_keyboard_complete);
    assert!(projection.all_bind_matrix);
    assert_eq!(projection.rows.len(), packet.rows.len());
    for row in &projection.rows {
        assert!(!row.stores_secret_body);
        assert!(!row.redacted_source_label.contains("://"));
        if let Some(label) = &row.redacted_account_label {
            assert!(!label.contains("://"));
        }
    }
}

#[test]
fn surface_projection_pins_write_authority() {
    let packet = packet();
    let row = packet
        .row("raf:public:keychain:reachable")
        .expect("reachable row");
    // A mutating surface can mutate a mutation-ready row.
    let desktop = row.surface_projection(PackageSurface::DesktopPackageWorkspace);
    assert!(desktop.can_mutate_here);
    assert!(!desktop.redacted);
    // An inspect surface never mutates, even a mutation-ready row.
    let ai = row.surface_projection(PackageSurface::AiContext);
    assert!(!ai.can_mutate_here);
    // A support export is redacted and never mutates.
    let support = row.surface_projection(PackageSurface::SupportExport);
    assert!(!support.can_mutate_here);
    assert!(support.redacted);
}

#[test]
fn anonymous_and_unsatisfied_sources_never_carry_a_handle() {
    let packet = packet();
    for row in &packet.rows {
        if matches!(
            row.credential_source,
            CredentialSourceClass::AnonymousAccess | CredentialSourceClass::AuthUnsatisfied
        ) {
            assert!(
                row.handle.is_none(),
                "row {} carries a handle for a non-handle source",
                row.row_id
            );
        }
    }
}

#[test]
fn detects_a_stored_secret_body() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.handle.is_some())
        .expect("a handle-backed row");
    row.handle.as_mut().unwrap().stores_secret_body = true;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, RegistryAuthFlowsViolation::SecretBodyStored { .. })));
    assert!(!packet.no_secret_bodies());
}

#[test]
fn detects_a_missing_required_action() {
    let mut packet = packet();
    let row = packet
        .row("raf:private:keychain:revoked")
        .expect("revoked row")
        .clone();
    let idx = packet
        .rows
        .iter()
        .position(|r| r.row_id == row.row_id)
        .unwrap();
    // Drop the rebind action a revoked handle requires.
    packet.rows[idx]
        .actions
        .retain(|a| a.kind != AuthActionKind::RebindHandle);
    let violations = packet.rows[idx].is_consistent();
    assert!(!violations);
    let packet_violations = packet.validate();
    assert!(packet_violations
        .iter()
        .any(|v| matches!(v, RegistryAuthFlowsViolation::MissingRequiredAction { .. })));
}

#[test]
fn detects_an_auth_mode_mismatch() {
    let mut packet = packet();
    packet.rows[0].auth_mode = AuthMode::Anonymous;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, RegistryAuthFlowsViolation::AuthModeMismatch { .. })));
}

#[test]
fn detects_a_raw_url_leak() {
    let mut packet = packet();
    packet.rows[0].redacted_source_label = "https://registry.acme.example/npm".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, RegistryAuthFlowsViolation::RawUrlLeak { .. })));
}

#[test]
fn detects_a_second_current_profile_for_one_source() {
    let mut packet = packet();
    // Make the revoked profile current for the same source as the reachable one.
    let idx = packet
        .rows
        .iter()
        .position(|r| r.row_id == "raf:private:keychain:revoked")
        .unwrap();
    packet.rows[idx].profile.is_current = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        RegistryAuthFlowsViolation::MultipleCurrentProfiles { .. }
    )));
}
