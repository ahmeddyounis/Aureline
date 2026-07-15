use super::*;

fn clean_conflict_input() -> M5SyncConflictPacketEntryResolutionInput {
    M5SyncConflictPacketEntryResolutionInput {
        entry_id: "conflict:test".to_owned(),
        conflict_ref: "settings.acme.editor.font-size@device-42".to_owned(),
        token_name: "conflict.editor.font_size".to_owned(),
        semantic_role: M5SettingsGovernanceRole::SyncConflict,
        conflict_class: M5SyncConflictClass::SameKeyDivergent,
        surface_context: M5ConfigSyncSurfaceContext::SyncSessionFlow,
        resolution_form_coverage: M5ConfigSyncResolutionForm::ALL.to_vec(),
        field_path: "editor.font-size".to_owned(),
        local_revision: "local.rev-0007".to_owned(),
        remote_revision: "remote.rev-0009".to_owned(),
        keep_local_option: "keep-local.font-size-14".to_owned(),
        keep_synced_option: "keep-synced.font-size-16".to_owned(),
        compare_reference: "compare.field-diff-0007".to_owned(),
        blocked_state_reason: "blocked.none-review-and-choose".to_owned(),
        bound_to_registry: true,
        resolution_is_field_aware: true,
        requires_local_authoritative: false,
        local_authority_preserved: true,
        proof_fresh: true,
    }
}

fn clean_device_input() -> M5DeviceActionRecordEntryResolutionInput {
    M5DeviceActionRecordEntryResolutionInput {
        entry_id: "device:test".to_owned(),
        device_ref: "device-42".to_owned(),
        token_name: "device_action.pause".to_owned(),
        semantic_role: M5SettingsGovernanceRole::SyncConflict,
        device_action_class: M5DeviceActionClass::PauseSync,
        surface_context: M5ConfigSyncSurfaceContext::SyncSessionFlow,
        resolution_form_coverage: M5ConfigSyncResolutionForm::ALL.to_vec(),
        actor: "actor.owner-user".to_owned(),
        action_timestamp: "ts.2026-07-15T00-00-00Z".to_owned(),
        transport_state: "transport.online-encrypted".to_owned(),
        policy_state: "policy.allow-sync".to_owned(),
        capability_dependency: "capability.sync-core".to_owned(),
        attribution_reference: "attribution.ledger-0007".to_owned(),
        last_ledger_revision: "revision.0007".to_owned(),
        keeps_attribution_visible: true,
        ledger_is_truthful: true,
        revocation_present: false,
        revocation_reason_disclosed: false,
        degraded_transport_present: false,
        local_authority_preserved_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_PACKET_ID
    );
}

#[test]
fn conflict_clean_names_meaning_and_is_bound() {
    let resolved = resolve_sync_conflict_packet_entry(clean_conflict_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.conflict_resolves_across_routes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.sync_conflict_packet_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.conflict_class_is_classified);
    assert!(resolved.resolution_is_field_aware);
    assert_eq!(resolved.semantic_role, "sync_conflict");
    assert_eq!(resolved.conflict_class, "same_key_divergent");
    assert_eq!(resolved.canonical_class_mode, "same_key_divergent_conflict");
    assert_eq!(resolved.surface_context, "sync_session_flow");
    assert_eq!(
        resolved.next_action,
        M5ConfigSyncNextAction::ExpandConflictMeaning
    );
}

#[test]
fn conflict_token_unstated_degrades() {
    let mut input = clean_conflict_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictTokenUnstated)
    );
}

#[test]
fn conflict_unbound_and_unclassified_degrade() {
    let mut input = clean_conflict_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictNotBoundToRegistry)
    );

    let mut input = clean_conflict_input();
    input.conflict_class = M5SyncConflictClass::ConflictClassUnclassified;
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::ConflictClassUnclassified)
    );
}

#[test]
fn conflict_packet_incomplete_and_collapse_and_form_degrade() {
    // An unstated blocked-state reason leaves the resolved packet incomplete.
    let mut input = clean_conflict_input();
    input.blocked_state_reason = "  ".to_owned();
    let resolved = resolve_sync_conflict_packet_entry(input).unwrap();
    assert!(!resolved.sync_conflict_packet_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictPacketIncomplete)
    );

    // A resolution collapsing into last-writer-wins degrades.
    let mut input = clean_conflict_input();
    input.resolution_is_field_aware = false;
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::ConflictSilentlyOverwritesOrHidesFieldResolution)
    );

    let mut input = clean_conflict_input();
    input.resolution_form_coverage = vec![M5ConfigSyncResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn conflict_overwrite_and_surface_and_proof_degrade() {
    let mut input = clean_conflict_input();
    input.conflict_class = M5SyncConflictClass::MachineOnly;
    input.requires_local_authoritative = true;
    input.local_authority_preserved = false;
    // A protected conflict silently overwriting local state first fails the field-aware / overwrite fold.
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::ConflictSilentlyOverwritesOrHidesFieldResolution)
    );

    let mut input = clean_conflict_input();
    input.surface_context = M5ConfigSyncSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_conflict_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_sync_conflict_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SyncConflictPacketEntryDegradeReason::ProofStale)
    );
}

#[test]
fn conflict_empty_id_and_forbidden_material_error() {
    let mut input = clean_conflict_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_sync_conflict_packet_entry(input).unwrap_err(),
        M5ConfigSyncResolutionError::EmptySyncConflictEntryId
    );

    let mut input = clean_conflict_input();
    input.blocked_state_reason = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_sync_conflict_packet_entry(input).unwrap_err(),
        M5ConfigSyncResolutionError::ForbiddenMaterial
    );
}

#[test]
fn conflict_does_not_silently_overwrite_rejects_protected_without_local_authority() {
    assert!(conflict_does_not_silently_overwrite(
        M5SyncConflictClass::SameKeyDivergent,
        true,
        false,
        true
    ));
    assert!(!conflict_does_not_silently_overwrite(
        M5SyncConflictClass::SameKeyDivergent,
        false,
        false,
        true
    ));
    assert!(conflict_does_not_silently_overwrite(
        M5SyncConflictClass::MachineOnly,
        true,
        true,
        true
    ));
    assert!(!conflict_does_not_silently_overwrite(
        M5SyncConflictClass::MachineOnly,
        true,
        true,
        false
    ));
    assert!(!conflict_does_not_silently_overwrite(
        M5SyncConflictClass::ConflictClassUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn sync_conflict_packet_is_complete_requires_all_fields() {
    assert!(sync_conflict_packet_is_complete(
        M5SyncConflictClass::SameKeyDivergent,
        "editor.font-size",
        "local.rev-0007",
        "remote.rev-0009",
        "keep-local.font-size-14",
        "keep-synced.font-size-16",
        "compare.field-diff-0007",
        "blocked.none-review-and-choose",
    ));
    assert!(!sync_conflict_packet_is_complete(
        M5SyncConflictClass::SameKeyDivergent,
        "editor.font-size",
        "  ",
        "remote.rev-0009",
        "keep-local.font-size-14",
        "keep-synced.font-size-16",
        "compare.field-diff-0007",
        "blocked.none-review-and-choose",
    ));
    assert!(!sync_conflict_packet_is_complete(
        M5SyncConflictClass::ConflictClassUnclassified,
        "editor.font-size",
        "local.rev-0007",
        "remote.rev-0009",
        "keep-local.font-size-14",
        "keep-synced.font-size-16",
        "compare.field-diff-0007",
        "blocked.none-review-and-choose",
    ));
}

#[test]
fn device_clean_stays_reconstructable() {
    let resolved = resolve_device_action_record_entry(clean_device_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.ledger_safe_on_every_route);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_device_action_ledger);
    assert!(resolved.device_action_ledger_stays_reconstructable);
    assert_eq!(resolved.device_action_class, "pause_sync");
    assert_eq!(resolved.surface_context, "sync_session_flow");
}

#[test]
fn device_hidden_and_unclassified_degrade() {
    // A revoke action that hides its cause is a hidden ledger.
    let mut input = clean_device_input();
    input.device_action_class = M5DeviceActionClass::RevokeDevice;
    input.revocation_present = true;
    input.revocation_reason_disclosed = false;
    let resolved = resolve_device_action_record_entry(input).unwrap();
    assert!(!resolved.provides_complete_device_action_ledger);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction)
    );

    // A record that drops the attribution visibility is also a hidden ledger.
    let mut input = clean_device_input();
    input.keeps_attribution_visible = false;
    assert_eq!(
        resolve_device_action_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction)
    );

    // A degraded-transport action that hides its local-authority posture is also a hidden ledger.
    let mut input = clean_device_input();
    input.degraded_transport_present = true;
    input.local_authority_preserved_disclosed = false;
    assert_eq!(
        resolve_device_action_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction)
    );

    let mut input = clean_device_input();
    input.device_action_class = M5DeviceActionClass::DeviceActionClassUnclassified;
    assert_eq!(
        resolve_device_action_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionClassUnclassified)
    );
}

#[test]
fn device_form_and_surface_and_id_and_material() {
    let mut input = clean_device_input();
    input.resolution_form_coverage = vec![M5ConfigSyncResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_device_action_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DeviceActionRecordEntryDegradeReason::LedgerFormCoverageIncomplete)
    );

    let mut input = clean_device_input();
    input.surface_context = M5ConfigSyncSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_device_action_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DeviceActionRecordEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_device_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_device_action_record_entry(input).unwrap_err(),
        M5ConfigSyncResolutionError::EmptyDeviceActionEntryId
    );

    let mut input = clean_device_input();
    input.actor = "see internal://notes".to_owned();
    assert_eq!(
        resolve_device_action_record_entry(input).unwrap_err(),
        M5ConfigSyncResolutionError::ForbiddenMaterial
    );
}

#[test]
fn device_disclosed_revocation_and_disclosed_authority_stay_clean() {
    // A revoke action that discloses its cause stays reconstructable.
    let mut input = clean_device_input();
    input.device_action_class = M5DeviceActionClass::RevokeDevice;
    input.revocation_present = true;
    input.revocation_reason_disclosed = true;
    assert!(resolve_device_action_record_entry(input)
        .unwrap()
        .is_clean());

    // A degraded-transport action that discloses its local-authority posture stays reconstructable.
    let mut input = clean_device_input();
    input.degraded_transport_present = true;
    input.local_authority_preserved_disclosed = true;
    assert!(resolve_device_action_record_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_setting_sync_conflict_and_device_action_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.vocabulary_set.sync_conflict_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SYNC_DEVICE_RECORD_LANDED_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ConfigSyncAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ConfigSyncExportField::SyncConflictClasses);
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.registry_rows[0].device_action_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    // Force a clean conflict entry to also read as packet-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.sync_conflict_entries[0].degrade_reason = None;
    row.sync_conflict_entries[0].sync_conflict_packet_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.silently_overwrites_locked_or_machine_only_state_during_sync = true,
            1 => row.collapses_conflict_classes_into_last_writer_wins = true,
            2 => row.resolves_a_conflict_without_a_field_level_keep_local_or_blocked_reason = true,
            _ => row.loses_device_action_lineage_in_diagnostics_or_support = true,
        }
        assert!(packet
            .validate()
            .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn sync_conflict_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    for row in &mut packet.registry_rows {
        row.sync_conflict_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictPacketIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::SyncConflictResolutionNotProven
    ));
}

#[test]
fn sync_conflict_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    // Drop every clean device-review-flow conflict so the first-consumer flows no longer include it.
    for row in &mut packet.registry_rows {
        row.sync_conflict_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "device_review_flow"));
    }
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::SyncConflictResolutionNotProven
    ));
}

#[test]
fn conflict_overwrite_not_proven_when_overwrite_example_removed() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    for row in &mut packet.registry_rows {
        row.sync_conflict_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SyncConflictPacketEntryDegradeReason::ConflictSilentlyOverwritesOrHidesFieldResolution,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::ConflictOverwriteHonestyNotProven
    ));
}

#[test]
fn conflict_overwrite_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    for row in &mut packet.registry_rows {
        row.sync_conflict_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::ConflictOverwriteHonestyNotProven
    ));
}

#[test]
fn device_action_integrity_not_proven_when_hidden_ledger_example_removed() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    for row in &mut packet.registry_rows {
        row.device_action_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::DeviceActionLedgerIntegrityNotProven
    ));
}

#[test]
fn device_action_integrity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    // Drop every clean rotate-token record so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.device_action_entries
            .retain(|ex| !(ex.is_clean() && ex.device_action_class == "rotate_token"));
    }
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::DeviceActionLedgerIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet
        .governance_review
        .conflicts_never_collapse_into_last_writer_wins = false;
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5SettingSyncConflictDeviceActionRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://sync.example/scope leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SettingSyncConflictDeviceActionRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_setting_sync_conflict_and_device_action_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn conflict_table_lists_only_clean_conflicts() {
    let packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    let table = packet.render_conflict_table();
    // The clean same-key and policy-locked conflicts are rendered from the registry.
    assert!(table.contains("same_key_divergent_conflict"));
    assert!(table.contains("policy_locked_conflict"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_setting_sync_conflict_and_device_action_registries_export()
        .expect("checked M5 sync-conflict / device-action registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_setting_sync_conflict_and_device_action_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5SettingsGovernanceQualificationClass::Beta
    );

    let preview =
        seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed(
        );
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5SettingsGovernanceQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5SettingSyncConflictDeviceActionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-sync-conflict-and-device-action-registries/sync_conflict_beta_narrowed.json"
    )))
    .expect("sync-conflict fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed()
    );

    let preview: M5SettingSyncConflictDeviceActionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-sync-conflict-and-device-action-registries/device_action_preview_narrowed.json"
    )))
    .expect("device-action fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed(
        )
    );
}

#[test]
fn implemented_families_is_sync_scope() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5SettingsGovernanceFamily::SyncScope]
    );
}
