use super::*;

const PACKET_ID: &str = CREDENTIAL_STATE_ROW_VAULT_PICKER_PACKET_ID;

fn packet() -> CredentialStateRowVaultPickerControlsPacket {
    seeded_credential_state_row_vault_picker_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        CREDENTIAL_STATE_ROW_VAULT_PICKER_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        CREDENTIAL_STATE_ROW_VAULT_PICKER_SCHEMA_VERSION
    );
}

#[test]
fn health_state_is_derived_not_asserted() {
    use CredentialHealthClass as Health;
    use M5CredentialLifecycleState as Life;

    // Active → healthy.
    let d = resolve_credential_health(Life::ActiveCurrent);
    assert_eq!(d.health_class, Health::Healthy);
    assert!(d.is_healthy);
    assert!(!d.is_revoked_or_expired);

    // Refresh / rotation → attention, still usable, never healthy.
    for state in [Life::RefreshNeeded, Life::RotationDue] {
        let d = resolve_credential_health(state);
        assert_eq!(d.health_class, Health::AttentionNeeded);
        assert!(!d.is_healthy);
        assert!(d.needs_attention_note);
    }

    // Revoked / expired never read as healthy.
    let d = resolve_credential_health(Life::Revoked);
    assert_eq!(d.health_class, Health::Revoked);
    assert!(!d.is_healthy);
    assert!(d.is_revoked_or_expired);
    assert!(d.needs_revoked_note);

    let d = resolve_credential_health(Life::Expired);
    assert_eq!(d.health_class, Health::Expired);
    assert!(d.is_revoked_or_expired);
    assert!(d.needs_expired_note);

    let d = resolve_credential_health(Life::Superseded);
    assert_eq!(d.health_class, Health::Superseded);
    assert!(!d.is_healthy);
    assert!(d.needs_superseded_note);
}

#[test]
fn portability_is_derived_not_asserted() {
    use M5CredentialRevealPosture as Reveal;
    use M5CredentialStorageMode as Storage;
    use M5CredentialStoreCapability as Capability;
    use VaultPortabilityClass as Portability;

    // Persist-across-restart + reveal-on-demand → portable.
    let d = resolve_vault_portability(
        Storage::OsKeychain,
        &[Capability::PersistAcrossRestart],
        Reveal::RevealOnDemand,
    );
    assert_eq!(d.portability_class, Portability::Portable);
    assert!(d.is_portable);

    // A store that blocks export is never portable.
    let d = resolve_vault_portability(
        Storage::ExternalReference,
        &[Capability::StoreExportBlocked],
        Reveal::MaskedLastFour,
    );
    assert_eq!(d.portability_class, Portability::ExportBlocked);
    assert!(!d.is_portable);
    assert!(d.is_export_blocked);
    assert!(d.needs_export_blocked_note);

    // Session-memory store is session-only, non-portable.
    let d = resolve_vault_portability(
        Storage::SessionMemoryOnly,
        &[Capability::SessionOnly],
        Reveal::ClipboardScoped,
    );
    assert_eq!(d.portability_class, Portability::SessionOnlyNonPortable);
    assert!(d.needs_session_only_note);

    // Broker handle / handle-only reveal → handle-reference-only.
    let d = resolve_vault_portability(
        Storage::SecretBrokerHandle,
        &[Capability::HardwareBacked],
        Reveal::HandleOnly,
    );
    assert_eq!(d.portability_class, Portability::HandleReferenceOnly);
    assert!(d.needs_handle_only_note);

    // Export-block takes precedence over session-only.
    let d = resolve_vault_portability(
        Storage::SessionMemoryOnly,
        &[Capability::StoreExportBlocked, Capability::SessionOnly],
        Reveal::HandleOnly,
    );
    assert_eq!(d.portability_class, Portability::ExportBlocked);
}

#[test]
fn health_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .credential_state_rows
        .iter()
        .map(|row| row.health_disclosure().health_class)
        .collect();
    for class in CredentialHealthClass::ALL {
        assert!(covered.contains(&class), "missing health class {class:?}");
    }
}

#[test]
fn target_boundary_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .credential_state_rows
        .iter()
        .map(|row| row.target_boundary)
        .collect();
    for boundary in CredentialTargetBoundary::ALL {
        assert!(covered.contains(&boundary), "missing boundary {boundary:?}");
    }
}

#[test]
fn portability_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .vault_pickers
        .iter()
        .map(|picker| picker.portability_disclosure().portability_class)
        .collect();
    for class in VaultPortabilityClass::ALL {
        assert!(covered.contains(&class), "missing portability {class:?}");
    }
}

#[test]
fn access_scope_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .vault_pickers
        .iter()
        .map(|picker| picker.access_scope)
        .collect();
    for scope in VaultAccessScope::ALL {
        assert!(covered.contains(&scope), "missing scope {scope:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::MissingSourceContracts));
}

#[test]
fn empty_credential_state_rows_fails() {
    let mut packet = packet();
    packet.credential_state_rows.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::CredentialStateRowsMissing));
}

#[test]
fn empty_vault_pickers_fails() {
    let mut packet = packet();
    packet.vault_pickers.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::VaultPickersMissing));
}

#[test]
fn row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].component = M5CredentialComponentFamily::VaultOrKeychainPicker;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::CredentialStateRowWrongComponentClass));
}

#[test]
fn revoked_row_claiming_healthy_fails() {
    let mut packet = packet();
    let row = packet
        .credential_state_rows
        .iter_mut()
        .find(|row| row.health_class == CredentialHealthClass::Revoked)
        .expect("revoked row present");
    row.claims_healthy = true;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::HealthStateMisrepresented));
}

#[test]
fn misdeclared_health_class_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].health_class = CredentialHealthClass::Revoked;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::HealthStateMisrepresented));
}

#[test]
fn missing_revoked_note_fails() {
    let mut packet = packet();
    let row = packet
        .credential_state_rows
        .iter_mut()
        .find(|row| row.health_class == CredentialHealthClass::Revoked)
        .expect("revoked row present");
    row.revoked_note.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::RevokedNoteMissing));
}

#[test]
fn missing_expired_note_fails() {
    let mut packet = packet();
    let row = packet
        .credential_state_rows
        .iter_mut()
        .find(|row| row.health_class == CredentialHealthClass::Expired)
        .expect("expired row present");
    row.expired_note.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::ExpiredNoteMissing));
}

#[test]
fn missing_storage_and_reveal_note_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0]
        .storage_and_reveal_note
        .clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::StorageAndRevealNoteMissing));
}

#[test]
fn missing_audit_note_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].audit_note.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::AuditNoteMissing));
}

#[test]
fn missing_rotate_revoke_test_action_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].default_actions =
        vec![CredentialStateRowAction::CopyHandleReference];
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::StateRowActionsIncomplete));
}

#[test]
fn missing_target_boundary_label_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].target_label.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::TargetBoundaryMissing));
}

#[test]
fn row_masking_storage_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].masks_storage_or_reveal_posture = true;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::StorageOrRevealMasked));
}

#[test]
fn picker_normalizing_raw_secret_fails() {
    let mut packet = packet();
    packet.vault_pickers[0].implies_raw_secret_exportable = true;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::RawSecretHandlingNormalized));
}

#[test]
fn friendly_connected_wording_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].uses_friendly_connected_wording = true;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::FriendlyConnectedWordingUsed));
}

#[test]
fn picker_wrong_component_class_fails() {
    let mut packet = packet();
    packet.vault_pickers[0].component = M5CredentialComponentFamily::CredentialStateRow;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::VaultPickerWrongComponentClass));
}

#[test]
fn export_blocked_picker_claiming_portable_fails() {
    let mut packet = packet();
    let picker = packet
        .vault_pickers
        .iter_mut()
        .find(|picker| picker.portability_class == VaultPortabilityClass::ExportBlocked)
        .expect("export-blocked picker present");
    picker.claims_portable = true;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::PortabilityMisrepresented));
}

#[test]
fn missing_export_blocked_note_fails() {
    let mut packet = packet();
    let picker = packet
        .vault_pickers
        .iter_mut()
        .find(|picker| picker.portability_class == VaultPortabilityClass::ExportBlocked)
        .expect("export-blocked picker present");
    picker.export_blocked_note.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::ExportBlockedNoteMissing));
}

#[test]
fn missing_session_only_note_fails() {
    let mut packet = packet();
    let picker = packet
        .vault_pickers
        .iter_mut()
        .find(|picker| picker.portability_class == VaultPortabilityClass::SessionOnlyNonPortable)
        .expect("session-only picker present");
    picker.session_only_note.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::SessionOnlyNoteMissing));
}

#[test]
fn missing_access_scope_label_fails() {
    let mut packet = packet();
    packet.vault_pickers[0].access_scope_label.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::AvailableSourceOrScopeMissing));
}

#[test]
fn missing_open_source_of_truth_action_fails() {
    let mut packet = packet();
    packet.vault_pickers[0].default_actions = vec![VaultPickerAction::SelectStore];
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::VaultPickerActionsIncomplete));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].required_labels = vec![M5CredentialRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_states_fails() {
    let mut packet = packet();
    packet.vault_pickers[0].degraded_states.clear();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::DegradedStatesMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .revoked_or_expired_never_reads_as_healthy = false;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .storage_mode_clarity_preserved_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.credential_state_rows[0].target_label = "see https://internal.example/creds".to_owned();
    assert!(packet
        .validate()
        .contains(&CredentialStateRowVaultPickerViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Credential-state rows"));
    assert!(summary.contains("## Vault/keychain pickers"));
    assert!(summary.contains("revoked"));
    assert!(summary.contains("export_blocked"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 rows + 5 pickers
    assert_eq!(lines, 1 + 6 + 5);
    assert!(csv.contains("credential_state_row"));
    assert!(csv.contains("vault_or_keychain_picker"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_credential_state_row_vault_picker_export()
        .expect("checked credential state row vault picker export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-credential-state-row-vault-picker-controls/credential_state_row_revoked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-credential-state-row-vault-picker-controls/vault_picker_export_blocked.json"
        )),
    ] {
        let packet: CredentialStateRowVaultPickerControlsPacket = serde_json::from_str(raw)
            .expect("fixture parses as credential state row vault picker packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_credential_state_row_vault_picker_controls_credential_state_row_revoked(),
        seeded_credential_state_row_vault_picker_controls_vault_picker_export_blocked(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
