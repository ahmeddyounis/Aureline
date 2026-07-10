use super::*;

fn clean_sheet_input() -> M5OverrideSheetResolutionInput {
    M5OverrideSheetResolutionInput {
        sheet_id: "override-sheet:test".to_owned(),
        current_mode: EfficiencyState::EfficiencyAware,
        expected_effect_workloads: vec![WorkloadFamily::IndexingRefresh, WorkloadFamily::AiWarmup],
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        allowed_ceiling_stated: true,
        performance_freshness_tradeoff_stated: true,
        uses_generic_efficiency_language: false,
        reset_path: Some("settings/efficiency/override/reset".to_owned()),
        proof_fresh: true,
    }
}

fn clean_note_input() -> M5PolicyNoteResolutionInput {
    M5PolicyNoteResolutionInput {
        note_id: "policy-note:test".to_owned(),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_actionable: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        block_reason_explained: true,
        locally_changeable: vec![WorkloadFamily::PreviewRefresh],
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_override_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_OVERRIDE_CONTROLS_PACKET_ID);
}

#[test]
fn sheet_clean_previews_mode_ceiling_effect_and_reset() {
    let resolved = resolve_override_sheet(clean_sheet_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.override_control_honest);
    assert!(!resolved.presented_dead_override_control);
    assert_eq!(resolved.current_mode, "EfficiencyAware");
    assert_eq!(
        resolved.expected_effect_workloads,
        vec!["indexing_refresh", "ai_warmup"]
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::OverrideAvailable
    );
    assert!(resolved.override_available);
    assert!(resolved.performance_freshness_tradeoff_stated);
    assert!(resolved.reset_path.is_some());
    assert_eq!(resolved.next_action, M5OverrideNextAction::NoActionNeeded);
}

#[test]
fn sheet_blocked_shown_as_blocked_is_clean() {
    let mut input = clean_sheet_input();
    input.override_posture = OverridePosture::PolicyBlocked;
    input.override_presented_actionable = false;
    let resolved = resolve_override_sheet(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::OverrideBlocked
    );
    assert!(!resolved.override_available);
    assert!(!resolved.presented_dead_override_control);
}

#[test]
fn sheet_dead_control_degrades_ac1() {
    let mut input = clean_sheet_input();
    input.override_posture = OverridePosture::PolicyBlocked;
    input.override_presented_actionable = true;
    let resolved = resolve_override_sheet(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OverrideSheetDegradeReason::DeadOverrideControlOffered)
    );
    assert!(resolved.presented_dead_override_control);
    assert!(!resolved.override_control_honest);
    assert_eq!(resolved.next_action, M5OverrideNextAction::OpenPolicyNote);
}

#[test]
fn sheet_tradeoff_unstated_degrades_ac2() {
    let mut input = clean_sheet_input();
    input.performance_freshness_tradeoff_stated = false;
    let resolved = resolve_override_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OverrideSheetDegradeReason::PerformanceFreshnessTradeoffUnstated)
    );
}

#[test]
fn sheet_generic_language_degrades_ac2() {
    let mut input = clean_sheet_input();
    input.uses_generic_efficiency_language = true;
    let resolved = resolve_override_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OverrideSheetDegradeReason::SideEffectsHiddenByGenericLanguage)
    );
    assert!(resolved.hides_side_effects_generic_language);
}

#[test]
fn sheet_effect_unstated_degrades_first_and_is_not_evaluated() {
    let mut input = clean_sheet_input();
    input.expected_effect_workloads = vec![];
    let resolved = resolve_override_sheet(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OverrideSheetDegradeReason::ExpectedEffectUnstated)
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::NotEvaluated
    );
}

#[test]
fn sheet_ceiling_and_reset_degrade() {
    let mut input = clean_sheet_input();
    input.allowed_ceiling_stated = false;
    assert_eq!(
        resolve_override_sheet(input).unwrap().degrade_reason,
        Some(M5OverrideSheetDegradeReason::AllowedCeilingUnstated)
    );

    let mut input = clean_sheet_input();
    input.reset_path = None;
    assert_eq!(
        resolve_override_sheet(input).unwrap().degrade_reason,
        Some(M5OverrideSheetDegradeReason::ResetPathUnstated)
    );
}

#[test]
fn sheet_empty_id_and_forbidden_material_error() {
    let mut input = clean_sheet_input();
    input.sheet_id = "  ".to_owned();
    assert_eq!(
        resolve_override_sheet(input).unwrap_err(),
        M5OverrideResolutionError::EmptySheetId
    );

    let mut input = clean_sheet_input();
    input.reset_path = Some("https://relay.internal/reset".to_owned());
    assert_eq!(
        resolve_override_sheet(input).unwrap_err(),
        M5OverrideResolutionError::ForbiddenMaterial
    );
}

#[test]
fn note_clean_names_owner_and_local_changeability() {
    let resolved = resolve_policy_note_row(clean_note_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.override_control_honest);
    assert_eq!(resolved.policy_owner, "user_controlled");
    assert_eq!(resolved.locally_changeable, vec!["preview_refresh"]);
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::OverrideAvailable
    );
}

#[test]
fn note_blocked_explained_is_clean_and_policy_blocked() {
    let mut input = clean_note_input();
    input.override_posture = OverridePosture::AdminControlled;
    input.override_presented_actionable = false;
    input.policy_owner = M5EfficiencyPolicyOwner::AdminPolicy;
    let resolved = resolve_policy_note_row(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::PolicyBlocked
    );
    assert!(!resolved.override_available);
}

#[test]
fn note_dead_control_degrades_ac1() {
    let mut input = clean_note_input();
    input.override_posture = OverridePosture::PolicyBlocked;
    input.override_presented_actionable = true;
    input.policy_owner = M5EfficiencyPolicyOwner::AdminPolicy;
    let resolved = resolve_policy_note_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PolicyNoteDegradeReason::DeadOverrideControlOffered)
    );
    assert!(resolved.presented_dead_override_control);
}

#[test]
fn note_owner_unresolved_degrades_first() {
    let mut input = clean_note_input();
    input.policy_owner = M5EfficiencyPolicyOwner::NoOwnerResolved;
    let resolved = resolve_policy_note_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PolicyNoteDegradeReason::PolicyOwnerUnresolved)
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::NotEvaluated
    );
}

#[test]
fn note_block_unexplained_and_locally_unstated_degrade() {
    let mut input = clean_note_input();
    input.override_posture = OverridePosture::AdminControlled;
    input.override_presented_actionable = false;
    input.policy_owner = M5EfficiencyPolicyOwner::AdminPolicy;
    input.block_reason_explained = false;
    assert_eq!(
        resolve_policy_note_row(input).unwrap().degrade_reason,
        Some(M5PolicyNoteDegradeReason::BlockReasonUnexplained)
    );

    let mut input = clean_note_input();
    input.locally_changeable = vec![];
    assert_eq!(
        resolve_policy_note_row(input).unwrap().degrade_reason,
        Some(M5PolicyNoteDegradeReason::LocalChangeabilityUnstated)
    );
}

#[test]
fn note_empty_id_and_forbidden_material_error() {
    let mut input = clean_note_input();
    input.note_id = "".to_owned();
    assert_eq!(
        resolve_policy_note_row(input).unwrap_err(),
        M5OverrideResolutionError::EmptyNoteId
    );

    let mut input = clean_note_input();
    input.note_id = "policy-note:-----begin key".to_owned();
    assert_eq!(
        resolve_policy_note_row(input).unwrap_err(),
        M5OverrideResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_override_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.vocabulary_set.work_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_EFFICIENCY_OVERRIDE_SHEET_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5OverrideAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5OverrideExportField::WorkDispositions);
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.controls_rows[0].policy_note_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_sheet_example_fails() {
    let mut packet = seeded_m5_override_controls();
    // Force a clean sheet to also present a dead override control — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.override_sheet_examples[0].degrade_reason = None;
    row.override_sheet_examples[0].presented_dead_override_control = true;
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_note_example_fails() {
    let mut packet = seeded_m5_override_controls();
    let row = &mut packet.controls_rows[0];
    row.policy_note_examples[0].degrade_reason = None;
    row.policy_note_examples[0].presented_dead_override_control = true;
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_override_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.presents_override_available_when_policy_blocks = true,
            1 => row.hides_side_effects_behind_generic_efficiency_language = true,
            2 => row.collapses_pressure_sources_into_generic_warning = true,
            _ => row.hides_what_remains_changeable_locally = true,
        }
        assert!(packet
            .validate()
            .contains(&M5OverrideControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_dead_control_example_removed() {
    let mut packet = seeded_m5_override_controls();
    // Drop every dead-control example so no AC1-negative example remains.
    for row in &mut packet.controls_rows {
        row.override_sheet_examples.retain(|ex| {
            ex.degrade_reason != Some(M5OverrideSheetDegradeReason::DeadOverrideControlOffered)
        });
        row.policy_note_examples.retain(|ex| {
            ex.degrade_reason != Some(M5PolicyNoteDegradeReason::DeadOverrideControlOffered)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_tradeoff_example_removed() {
    let mut packet = seeded_m5_override_controls();
    for row in &mut packet.controls_rows {
        row.override_sheet_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5OverrideSheetDegradeReason::PerformanceFreshnessTradeoffUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_override_controls();
    packet
        .governance_review
        .no_dead_override_control_when_policy_blocks = false;
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_override_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_override_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_override_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5OverrideControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_override_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_override_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_override_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_override_controls_export()
        .expect("checked M5 override controls export validates");
    assert_eq!(from_disk.packet_id, M5_OVERRIDE_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_override_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_override_controls_override_settings_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::OverrideSettingsUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Beta);

    let preview = seeded_m5_override_controls_activity_center_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5OverrideControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-override-sheet-policy-note-controls/override_settings_beta_narrowed.json"
    )))
    .expect("override-settings fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_override_controls_override_settings_beta_narrowed()
    );

    let preview: M5OverrideControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-override-sheet-policy-note-controls/activity_center_preview_narrowed.json"
    )))
    .expect("activity-center fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_override_controls_activity_center_preview_narrowed()
    );
}
