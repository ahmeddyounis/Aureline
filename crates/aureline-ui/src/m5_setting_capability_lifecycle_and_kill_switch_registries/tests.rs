use super::*;

fn clean_capability_input() -> M5CapabilityRecordEntryResolutionInput {
    M5CapabilityRecordEntryResolutionInput {
        entry_id: "capability:test".to_owned(),
        capability_ref: "capability.acme.ai.inline-assist@labs".to_owned(),
        token_name: "capability.ai.inline_assist".to_owned(),
        semantic_role: M5SettingsGovernanceRole::CapabilityLifecycle,
        lifecycle_class: M5CapabilityLifecycleClass::Labs,
        surface_context: M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        resolution_form_coverage: M5ConfigCapabilityResolutionForm::ALL.to_vec(),
        owner: "owner.ai-platform-team".to_owned(),
        scope: "scope.user".to_owned(),
        review_or_expiry: "review.2026-10-01-labs-review".to_owned(),
        enabled_posture: "posture.opt-in-off-by-default".to_owned(),
        dependency_marker: "dependency.marker-ai-runtime-v3".to_owned(),
        fallback: "fallback.classic-completion".to_owned(),
        rollback_note: "rollback.disable-restores-classic".to_owned(),
        bound_to_registry: true,
        dependency_marker_published: true,
        requires_dependency_marker_and_fallback: true,
        fallback_published: true,
        proof_fresh: true,
    }
}

fn clean_kill_switch_input() -> M5KillSwitchRecordEntryResolutionInput {
    M5KillSwitchRecordEntryResolutionInput {
        entry_id: "kill-switch:test".to_owned(),
        capability_ref: "capability.acme.ai.inline-assist@labs".to_owned(),
        token_name: "kill_switch.ai.inline_assist".to_owned(),
        semantic_role: M5SettingsGovernanceRole::CapabilityLifecycle,
        kill_switch_class: M5KillSwitchClass::KillSwitch,
        surface_context: M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        resolution_form_coverage: M5ConfigCapabilityResolutionForm::ALL.to_vec(),
        disabling_source: "source.remote-kill-switch-signal".to_owned(),
        disabled_timestamp: "ts.2026-07-15T00-00-00Z".to_owned(),
        preserved_data_reference: "preserved.user-prompts-retained".to_owned(),
        explanation_reference: "explain.disabled-by-remote-kill-switch".to_owned(),
        capability_dependency: "capability.ai.inline_assist".to_owned(),
        fallback_reference: "fallback.classic-completion".to_owned(),
        last_ledger_revision: "revision.0007".to_owned(),
        keeps_disabling_source_visible: true,
        ledger_is_truthful: true,
        policy_disable_present: false,
        disable_cause_disclosed: false,
        user_data_present: false,
        user_data_preservation_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_PACKET_ID
    );
}

#[test]
fn capability_clean_names_meaning_and_is_bound() {
    let resolved = resolve_capability_record_entry(clean_capability_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.capability_resolves_across_routes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.capability_record_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.lifecycle_class_is_classified);
    assert!(resolved.dependency_marker_published);
    assert_eq!(resolved.semantic_role, "capability_lifecycle");
    assert_eq!(resolved.lifecycle_class, "labs");
    assert_eq!(resolved.canonical_state_mode, "labs_capability");
    assert_eq!(resolved.surface_context, "settings_surface_flow");
    assert_eq!(
        resolved.next_action,
        M5ConfigCapabilityNextAction::ExpandCapabilityMeaning
    );
}

#[test]
fn capability_token_unstated_degrades() {
    let mut input = clean_capability_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityTokenUnstated)
    );
}

#[test]
fn capability_unbound_and_unclassified_degrade() {
    let mut input = clean_capability_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityNotBoundToRegistry)
    );

    let mut input = clean_capability_input();
    input.lifecycle_class = M5CapabilityLifecycleClass::LifecycleClassUnclassified;
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityLifecycleClassUnclassified)
    );
}

#[test]
fn capability_record_incomplete_and_hide_and_form_degrade() {
    // An unstated rollback note leaves the resolved record incomplete.
    let mut input = clean_capability_input();
    input.rollback_note = "  ".to_owned();
    let resolved = resolve_capability_record_entry(input).unwrap();
    assert!(!resolved.capability_record_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityRecordIncomplete)
    );

    // A hidden dependency marker degrades.
    let mut input = clean_capability_input();
    input.dependency_marker_published = false;
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityHidesDependencyOrLacksFallback)
    );

    let mut input = clean_capability_input();
    input.resolution_form_coverage = vec![M5ConfigCapabilityResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn capability_missing_fallback_and_surface_and_proof_degrade() {
    let mut input = clean_capability_input();
    input.lifecycle_class = M5CapabilityLifecycleClass::Beta;
    input.requires_dependency_marker_and_fallback = true;
    input.fallback_published = false;
    // A protected capability without a fallback first fails the dependency-marker fold.
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityHidesDependencyOrLacksFallback)
    );

    let mut input = clean_capability_input();
    input.surface_context = M5ConfigCapabilitySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_capability_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_capability_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CapabilityRecordEntryDegradeReason::ProofStale)
    );
}

#[test]
fn capability_empty_id_and_forbidden_material_error() {
    let mut input = clean_capability_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_capability_record_entry(input).unwrap_err(),
        M5ConfigCapabilityResolutionError::EmptyCapabilityEntryId
    );

    let mut input = clean_capability_input();
    input.fallback = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_capability_record_entry(input).unwrap_err(),
        M5ConfigCapabilityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn capability_does_not_hide_dependency_rejects_protected_without_fallback() {
    assert!(capability_does_not_hide_dependency(
        M5CapabilityLifecycleClass::GenerallyAvailable,
        true,
        false,
        true
    ));
    assert!(!capability_does_not_hide_dependency(
        M5CapabilityLifecycleClass::GenerallyAvailable,
        false,
        false,
        true
    ));
    assert!(capability_does_not_hide_dependency(
        M5CapabilityLifecycleClass::Beta,
        true,
        true,
        true
    ));
    assert!(!capability_does_not_hide_dependency(
        M5CapabilityLifecycleClass::Beta,
        true,
        true,
        false
    ));
    assert!(!capability_does_not_hide_dependency(
        M5CapabilityLifecycleClass::LifecycleClassUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn capability_record_is_complete_requires_all_fields() {
    assert!(capability_record_is_complete(
        M5CapabilityLifecycleClass::Labs,
        "owner.ai-platform-team",
        "scope.user",
        "review.2026-10-01-labs-review",
        "posture.opt-in-off-by-default",
        "dependency.marker-ai-runtime-v3",
        "fallback.classic-completion",
        "rollback.disable-restores-classic",
    ));
    assert!(!capability_record_is_complete(
        M5CapabilityLifecycleClass::Labs,
        "owner.ai-platform-team",
        "  ",
        "review.2026-10-01-labs-review",
        "posture.opt-in-off-by-default",
        "dependency.marker-ai-runtime-v3",
        "fallback.classic-completion",
        "rollback.disable-restores-classic",
    ));
    assert!(!capability_record_is_complete(
        M5CapabilityLifecycleClass::LifecycleClassUnclassified,
        "owner.ai-platform-team",
        "scope.user",
        "review.2026-10-01-labs-review",
        "posture.opt-in-off-by-default",
        "dependency.marker-ai-runtime-v3",
        "fallback.classic-completion",
        "rollback.disable-restores-classic",
    ));
}

#[test]
fn kill_switch_clean_preserves_data() {
    let resolved = resolve_kill_switch_record_entry(clean_kill_switch_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.ledger_safe_on_every_route);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_kill_switch_ledger);
    assert!(resolved.kill_switch_record_preserves_data_and_explains);
    assert_eq!(resolved.kill_switch_class, "kill_switch");
    assert_eq!(resolved.surface_context, "settings_surface_flow");
}

#[test]
fn kill_switch_hidden_and_unclassified_degrade() {
    // A kill switch that hides its cause is a hidden ledger.
    let mut input = clean_kill_switch_input();
    input.policy_disable_present = true;
    input.disable_cause_disclosed = false;
    let resolved = resolve_kill_switch_record_entry(input).unwrap();
    assert!(!resolved.provides_complete_kill_switch_ledger);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation)
    );

    // A record that drops the disabling-source visibility is also a hidden ledger.
    let mut input = clean_kill_switch_input();
    input.keeps_disabling_source_visible = false;
    assert_eq!(
        resolve_kill_switch_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation)
    );

    // A disable that held user data but hides its preservation is also a hidden ledger.
    let mut input = clean_kill_switch_input();
    input.user_data_present = true;
    input.user_data_preservation_disclosed = false;
    assert_eq!(
        resolve_kill_switch_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation)
    );

    let mut input = clean_kill_switch_input();
    input.kill_switch_class = M5KillSwitchClass::KillSwitchClassUnclassified;
    assert_eq!(
        resolve_kill_switch_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchClassUnclassified)
    );
}

#[test]
fn kill_switch_form_and_surface_and_id_and_material() {
    let mut input = clean_kill_switch_input();
    input.resolution_form_coverage = vec![M5ConfigCapabilityResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_kill_switch_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5KillSwitchRecordEntryDegradeReason::LedgerFormCoverageIncomplete)
    );

    let mut input = clean_kill_switch_input();
    input.surface_context = M5ConfigCapabilitySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_kill_switch_record_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5KillSwitchRecordEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_kill_switch_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_kill_switch_record_entry(input).unwrap_err(),
        M5ConfigCapabilityResolutionError::EmptyKillSwitchEntryId
    );

    let mut input = clean_kill_switch_input();
    input.disabling_source = "see internal://notes".to_owned();
    assert_eq!(
        resolve_kill_switch_record_entry(input).unwrap_err(),
        M5ConfigCapabilityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn kill_switch_disclosed_cause_and_disclosed_preservation_stay_clean() {
    // A kill switch that discloses its cause stays reconstructable.
    let mut input = clean_kill_switch_input();
    input.policy_disable_present = true;
    input.disable_cause_disclosed = true;
    assert!(resolve_kill_switch_record_entry(input).unwrap().is_clean());

    // A disable that discloses that user data stays preserved stays reconstructable.
    let mut input = clean_kill_switch_input();
    input.user_data_present = true;
    input.user_data_preservation_disclosed = true;
    assert!(resolve_kill_switch_record_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.vocabulary_set.capability_lifecycle_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ConfigCapabilityAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ConfigCapabilityExportField::CapabilityLifecycleClasses);
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.registry_rows[0].kill_switch_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    // Force a clean capability entry to also read as record-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.capability_entries[0].degrade_reason = None;
    row.capability_entries[0].capability_record_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers = true,
            1 => {
                row.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy = true
            }
            2 => row.lets_a_stable_surface_depend_on_a_hidden_labs_or_preview_capability = true,
            _ => row.loses_user_authored_data_when_a_kill_switch_or_policy_disable_fires = true,
        }
        assert!(packet.validate().contains(
            &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn capability_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    for row in &mut packet.registry_rows {
        row.capability_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CapabilityRecordEntryDegradeReason::CapabilityRecordIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::CapabilityLifecycleResolutionNotProven
    ));
}

#[test]
fn capability_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    // Drop every clean bundle-flow capability so the first-consumer flows no longer include it.
    for row in &mut packet.registry_rows {
        row.capability_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "bundle_flow"));
    }
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::CapabilityLifecycleResolutionNotProven
    ));
}

#[test]
fn dependency_honesty_not_proven_when_hide_example_removed() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    for row in &mut packet.registry_rows {
        row.capability_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5CapabilityRecordEntryDegradeReason::CapabilityHidesDependencyOrLacksFallback,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DependencyMarkerHonestyNotProven
    ));
}

#[test]
fn dependency_honesty_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    for row in &mut packet.registry_rows {
        row.capability_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CapabilityRecordEntryDegradeReason::CapabilityNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DependencyMarkerHonestyNotProven
    ));
}

#[test]
fn kill_switch_integrity_not_proven_when_hidden_example_removed() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    for row in &mut packet.registry_rows {
        row.kill_switch_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::KillSwitchDataPreservationNotProven
    ));
}

#[test]
fn kill_switch_integrity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    // Drop every clean manual-opt-out record so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.kill_switch_entries
            .retain(|ex| !(ex.is_clean() && ex.kill_switch_class == "manual_opt_out"));
    }
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::KillSwitchDataPreservationNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet
        .governance_review
        .no_stable_surface_depends_on_a_hidden_labs_or_preview_capability = false;
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://capabilities.example/scope leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn capability_table_lists_only_clean_capabilities() {
    let packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    let table = packet.render_capability_table();
    // The clean Labs and Preview capabilities are rendered from the registry.
    assert!(table.contains("labs_capability"));
    assert!(table.contains("preview_capability"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_setting_capability_lifecycle_and_kill_switch_registries_export()
            .expect("checked M5 capability / kill-switch registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_capability_lifecycle_beta_narrowed();
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
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_kill_switch_preview_narrowed();
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
    let beta: M5SettingCapabilityLifecycleKillSwitchRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-capability-lifecycle-and-kill-switch-registries/capability_lifecycle_beta_narrowed.json"
    )))
    .expect("capability-lifecycle fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_capability_lifecycle_beta_narrowed()
    );

    let preview: M5SettingCapabilityLifecycleKillSwitchRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-capability-lifecycle-and-kill-switch-registries/kill_switch_preview_narrowed.json"
    )))
    .expect("kill-switch fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_kill_switch_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_rollout_capability() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5SettingsGovernanceFamily::RolloutCapability]
    );
}
