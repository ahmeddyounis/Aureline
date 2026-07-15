use super::*;

fn clean_write_intent_input() -> M5SettingWriteIntentEntryResolutionInput {
    M5SettingWriteIntentEntryResolutionInput {
        entry_id: "write-intent:test".to_owned(),
        write_target_id: "settings.acme.editor.format-on-save@workspace".to_owned(),
        token_name: "write.intent.editor.format_on_save".to_owned(),
        semantic_role: M5SettingsGovernanceRole::WriteIntent,
        preview_class: M5WriteIntentPreviewClass::NoOpReversible,
        surface_context: M5ConfigWriteSurfaceContext::SettingsSurface,
        resolution_form_coverage: M5ConfigWriteResolutionForm::ALL.to_vec(),
        target_scope: "scope.workspace".to_owned(),
        target_artifact: "artifact.workspace-settings-json".to_owned(),
        intended_value: "value.true".to_owned(),
        actor: "actor.user-edit".to_owned(),
        change_reason: "reason.enable-format-on-save".to_owned(),
        preview_reference: "preview.none-needed".to_owned(),
        recovery_reference: "recovery.checkpoint-and-rollback-0007".to_owned(),
        bound_to_registry: true,
        scope_ownership_preserved: true,
        is_high_risk_write: false,
        evidence_materialized: true,
        proof_fresh: true,
    }
}

fn clean_constraint_input() -> M5PolicyConstraintEntryResolutionInput {
    M5PolicyConstraintEntryResolutionInput {
        entry_id: "constraint:test".to_owned(),
        constraint_ref: "editor.format_on_save".to_owned(),
        token_name: "constraint.editor.format_on_save".to_owned(),
        semantic_role: M5SettingsGovernanceRole::PolicyConstraint,
        lock_class: M5PolicyLockClass::PolicyLocked,
        surface_context: M5ConfigWriteSurfaceContext::SettingsSurface,
        resolution_form_coverage: M5ConfigWriteResolutionForm::ALL.to_vec(),
        lock_source: "lock.org-policy-bundle".to_owned(),
        allowed_override_classes: "override.none".to_owned(),
        expiry_review: "review.expires-2026-12-31".to_owned(),
        validation_status: "validation.ok".to_owned(),
        review_state: "review.current".to_owned(),
        docs_pointer: "docs.settings-policy-locked".to_owned(),
        last_review_revision: "revision.0007".to_owned(),
        keeps_lock_source_visible: true,
        constraint_is_truthful: true,
        lock_present: true,
        lock_source_disclosed: true,
        denial_present: false,
        fallback_guidance_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_PACKET_ID
    );
}

#[test]
fn write_intent_clean_names_meaning_and_is_bound() {
    let resolved = resolve_setting_write_intent_entry(clean_write_intent_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.write_intent_lands_across_routes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.write_intent_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.preview_class_is_classified);
    assert!(resolved.scope_ownership_preserved);
    assert_eq!(resolved.semantic_role, "write_intent");
    assert_eq!(resolved.preview_class, "no_op_reversible");
    assert_eq!(resolved.canonical_class_mode, "no_op_reversible_class");
    assert_eq!(resolved.surface_context, "settings_surface");
    assert_eq!(
        resolved.next_action,
        M5ConfigWriteNextAction::ExpandWriteIntentMeaning
    );
}

#[test]
fn write_intent_token_unstated_degrades() {
    let mut input = clean_write_intent_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentTokenUnstated)
    );
}

#[test]
fn write_intent_unbound_and_unclassified_degrade() {
    let mut input = clean_write_intent_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentNotBoundToRegistry)
    );

    let mut input = clean_write_intent_input();
    input.preview_class = M5WriteIntentPreviewClass::PreviewClassUnclassified;
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::PreviewClassUnclassified)
    );
}

#[test]
fn write_intent_object_incomplete_and_rewrite_and_form_degrade() {
    // An unstated preview reference leaves the resolved object incomplete.
    let mut input = clean_write_intent_input();
    input.preview_reference = "  ".to_owned();
    let resolved = resolve_setting_write_intent_entry(input).unwrap();
    assert!(!resolved.write_intent_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentObjectIncomplete)
    );

    // A rewritten chosen scope / artifact degrades.
    let mut input = clean_write_intent_input();
    input.scope_ownership_preserved = false;
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(
            M5SettingWriteIntentEntryDegradeReason::WriteIntentRewritesScopeOrHidesRecoveryEvidence
        )
    );

    let mut input = clean_write_intent_input();
    input.resolution_form_coverage = vec![M5ConfigWriteResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn write_intent_recovery_and_surface_and_proof_degrade() {
    let mut input = clean_write_intent_input();
    input.preview_class = M5WriteIntentPreviewClass::HighRiskIrreversible;
    input.is_high_risk_write = true;
    input.evidence_materialized = false;
    // A high-risk write hiding its recovery evidence first fails the scope / evidence fold.
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(
            M5SettingWriteIntentEntryDegradeReason::WriteIntentRewritesScopeOrHidesRecoveryEvidence
        )
    );

    let mut input = clean_write_intent_input();
    input.surface_context = M5ConfigWriteSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_write_intent_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_setting_write_intent_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SettingWriteIntentEntryDegradeReason::ProofStale)
    );
}

#[test]
fn write_intent_empty_id_and_forbidden_material_error() {
    let mut input = clean_write_intent_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_setting_write_intent_entry(input).unwrap_err(),
        M5ConfigWriteResolutionError::EmptyWriteIntentEntryId
    );

    let mut input = clean_write_intent_input();
    input.change_reason = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_setting_write_intent_entry(input).unwrap_err(),
        M5ConfigWriteResolutionError::ForbiddenMaterial
    );
}

#[test]
fn write_intent_lands_in_chosen_scope_rejects_rewrite() {
    assert!(write_intent_lands_in_chosen_scope(
        M5WriteIntentPreviewClass::NoOpReversible,
        true,
        false,
        true
    ));
    assert!(!write_intent_lands_in_chosen_scope(
        M5WriteIntentPreviewClass::NoOpReversible,
        false,
        false,
        true
    ));
    assert!(write_intent_lands_in_chosen_scope(
        M5WriteIntentPreviewClass::HighRiskIrreversible,
        true,
        true,
        true
    ));
    assert!(!write_intent_lands_in_chosen_scope(
        M5WriteIntentPreviewClass::HighRiskIrreversible,
        true,
        true,
        false
    ));
    assert!(!write_intent_lands_in_chosen_scope(
        M5WriteIntentPreviewClass::PreviewClassUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn write_intent_object_is_complete_requires_all_fields() {
    assert!(write_intent_object_is_complete(
        M5WriteIntentPreviewClass::NoOpReversible,
        "scope.workspace",
        "artifact.workspace-settings-json",
        "value.true",
        "actor.user-edit",
        "reason.enable-format-on-save",
        "preview.none-needed",
        "recovery.checkpoint-and-rollback-0007",
    ));
    assert!(!write_intent_object_is_complete(
        M5WriteIntentPreviewClass::NoOpReversible,
        "scope.workspace",
        "  ",
        "value.true",
        "actor.user-edit",
        "reason.enable-format-on-save",
        "preview.none-needed",
        "recovery.checkpoint-and-rollback-0007",
    ));
    assert!(!write_intent_object_is_complete(
        M5WriteIntentPreviewClass::PreviewClassUnclassified,
        "scope.workspace",
        "artifact.workspace-settings-json",
        "value.true",
        "actor.user-edit",
        "reason.enable-format-on-save",
        "preview.none-needed",
        "recovery.checkpoint-and-rollback-0007",
    ));
}

#[test]
fn constraint_clean_stays_honest() {
    let resolved = resolve_policy_constraint_entry(clean_constraint_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.constraint_safe_on_every_route);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_policy_constraint);
    assert!(resolved.policy_constraint_stays_honest);
    assert_eq!(resolved.lock_class, "policy_locked");
    assert_eq!(resolved.surface_context, "settings_surface");
}

#[test]
fn constraint_masked_lock_and_unclassified_degrade() {
    // A masked locked value that hides its lock source is a masked lock.
    let mut input = clean_constraint_input();
    input.lock_present = true;
    input.lock_source_disclosed = false;
    let resolved = resolve_policy_constraint_entry(input).unwrap();
    assert!(!resolved.provides_complete_policy_constraint);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback)
    );

    // A record that drops the lock source visibility is also a masked lock.
    let mut input = clean_constraint_input();
    input.keeps_lock_source_visible = false;
    assert_eq!(
        resolve_policy_constraint_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback)
    );

    // A denied write that hides its fallback guidance is also a masked lock / hidden fallback.
    let mut input = clean_constraint_input();
    input.denial_present = true;
    input.fallback_guidance_disclosed = false;
    assert_eq!(
        resolve_policy_constraint_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback)
    );

    let mut input = clean_constraint_input();
    input.lock_class = M5PolicyLockClass::LockClassUnclassified;
    assert_eq!(
        resolve_policy_constraint_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyConstraintEntryDegradeReason::LockClassUnclassified)
    );
}

#[test]
fn constraint_form_and_surface_and_id_and_material() {
    let mut input = clean_constraint_input();
    input.resolution_form_coverage = vec![M5ConfigWriteResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_policy_constraint_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyConstraintEntryDegradeReason::ConstraintFormCoverageIncomplete)
    );

    let mut input = clean_constraint_input();
    input.surface_context = M5ConfigWriteSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_policy_constraint_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5PolicyConstraintEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_constraint_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_policy_constraint_entry(input).unwrap_err(),
        M5ConfigWriteResolutionError::EmptyPolicyConstraintEntryId
    );

    let mut input = clean_constraint_input();
    input.lock_source = "see internal://notes".to_owned();
    assert_eq!(
        resolve_policy_constraint_entry(input).unwrap_err(),
        M5ConfigWriteResolutionError::ForbiddenMaterial
    );
}

#[test]
fn constraint_disclosed_lock_and_disclosed_fallback_stay_clean() {
    // A locked value that discloses its lock source stays honest.
    let mut input = clean_constraint_input();
    input.lock_present = true;
    input.lock_source_disclosed = true;
    assert!(resolve_policy_constraint_entry(input).unwrap().is_clean());

    // A denied write that discloses its fallback guidance stays honest.
    let mut input = clean_constraint_input();
    input.denial_present = true;
    input.fallback_guidance_disclosed = true;
    assert!(resolve_policy_constraint_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_setting_write_intent_and_policy_constraint_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.vocabulary_set.write_intent_preview_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5SettingWriteIntentPolicyConstraintRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_POLICY_CONSTRAINT_LANDED_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ConfigWriteAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ConfigWriteExportField::WriteIntentPreviewClasses);
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.registry_rows[0].policy_constraint_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingWriteIntentPolicyConstraintRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    // Force a clean write-intent entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.write_intent_entries[0].degrade_reason = None;
    row.write_intent_entries[0].write_intent_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SettingWriteIntentPolicyConstraintRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.rewrites_a_scoped_write_into_a_broader_scope = true,
            1 => row.lands_a_write_in_an_unintended_artifact_or_scope = true,
            2 => row.applies_a_high_risk_write_without_preview_checkpoint_or_rollback = true,
            _ => row.hides_a_lock_or_policy_disable_cause_behind_generic_unavailable_copy = true,
        }
        assert!(packet.validate().contains(
            &M5SettingWriteIntentPolicyConstraintRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn write_intent_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    for row in &mut packet.registry_rows {
        row.write_intent_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::WriteIntentResolutionNotProven
    ));
}

#[test]
fn write_intent_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    // Drop every clean admin-surface write intent so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.write_intent_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::WriteIntentResolutionNotProven
    ));
}

#[test]
fn write_scope_ownership_not_proven_when_rewrite_example_removed() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    for row in &mut packet.registry_rows {
        row.write_intent_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SettingWriteIntentEntryDegradeReason::WriteIntentRewritesScopeOrHidesRecoveryEvidence,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::WriteScopeOwnershipPreservationNotProven
    ));
}

#[test]
fn write_scope_ownership_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    for row in &mut packet.registry_rows {
        row.write_intent_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::WriteScopeOwnershipPreservationNotProven
    ));
}

#[test]
fn policy_constraint_integrity_not_proven_when_masked_lock_example_removed() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    for row in &mut packet.registry_rows {
        row.policy_constraint_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::PolicyConstraintIntegrityNotProven
    ));
}

#[test]
fn policy_constraint_integrity_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    // Drop every clean advisory-constraint policy record so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.policy_constraint_entries
            .retain(|ex| !(ex.is_clean() && ex.lock_class == "advisory_constraint"));
    }
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::PolicyConstraintIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet
        .governance_review
        .writes_land_only_in_chosen_scope_and_artifact = false;
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5SettingWriteIntentPolicyConstraintRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://sync.example/scope leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SettingWriteIntentPolicyConstraintRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_setting_write_intent_and_policy_constraint_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn write_intent_table_lists_only_clean_write_intents() {
    let packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    let table = packet.render_write_intent_table();
    // The clean no-op and low-risk write intents are rendered from the registry.
    assert!(table.contains("no_op_reversible_class"));
    assert!(table.contains("low_risk_reversible_class"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_setting_write_intent_and_policy_constraint_registries_export()
            .expect("checked M5 write-intent / policy-constraint registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_setting_write_intent_and_policy_constraint_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed(
        );
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
        seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed();
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
    let beta: M5SettingWriteIntentPolicyConstraintRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-write-intent-and-policy-constraint-registries/write_intent_beta_narrowed.json"
    )))
    .expect("write-intent fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed(
        )
    );

    let preview: M5SettingWriteIntentPolicyConstraintRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-setting-write-intent-and-policy-constraint-registries/policy_constraint_preview_narrowed.json"
    )))
    .expect("policy-constraint fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_write_setting() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5SettingsGovernanceFamily::WriteSetting]
    );
}
