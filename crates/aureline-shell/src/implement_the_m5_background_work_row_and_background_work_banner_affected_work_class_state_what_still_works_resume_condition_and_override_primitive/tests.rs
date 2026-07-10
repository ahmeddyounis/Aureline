use super::*;

fn clean_row_input() -> M5BackgroundWorkRowResolutionInput {
    M5BackgroundWorkRowResolutionInput {
        row_id: "bg-row:test".to_owned(),
        affected_work_class: Some(WorkloadFamily::IndexingRefresh),
        paused: true,
        slowed: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        preserved_protected_tasks: vec!["save".to_owned()],
        adaptive_change_user_visible: true,
        durable_surface_present: true,
        proof_fresh: true,
    }
}

fn clean_banner_input() -> M5BackgroundWorkBannerResolutionInput {
    M5BackgroundWorkBannerResolutionInput {
        banner_id: "bg-banner:test".to_owned(),
        slowed_workloads: vec![WorkloadFamily::AiWarmup],
        paused_workloads: vec![WorkloadFamily::IndexingRefresh],
        preserved_protected_tasks: vec!["save".to_owned()],
        pressure_event_count: 4,
        coalesced_into_single_banner: true,
        shows_paused_work_explicitly: true,
        uses_generic_service_failure_copy: false,
        resume_condition: Some(EfficiencyRecoveryState::StagedResume),
        override_posture: OverridePosture::UserOverrideSessionOnly,
        override_presented_available: true,
        policy_owner: M5EfficiencyPolicyOwner::UserControlled,
        durable_surface_present: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_background_work_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BACKGROUND_WORK_CONTROLS_PACKET_ID);
}

#[test]
fn row_clean_names_class_state_preserved_and_resume() {
    let resolved = resolve_background_work_row(clean_row_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.reviewable_after_looking_away);
    assert!(!resolved.presented_override_when_blocked);
    assert_eq!(
        resolved.affected_work_class.as_deref(),
        Some("indexing_refresh")
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::Paused
    );
    assert_eq!(resolved.resume_condition.as_deref(), Some("staged_resume"));
    assert!(resolved.override_available);
}

#[test]
fn row_running_full_needs_no_action() {
    let mut input = clean_row_input();
    input.affected_work_class = Some(WorkloadFamily::PreviewRefresh);
    input.paused = false;
    input.slowed = false;
    input.resume_condition = None;
    input.adaptive_change_user_visible = false;
    let resolved = resolve_background_work_row(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::RunningFull
    );
    assert_eq!(
        resolved.next_action,
        M5BackgroundWorkNextAction::NoActionNeeded
    );
}

#[test]
fn row_toast_only_degrades_ac1() {
    let mut input = clean_row_input();
    input.durable_surface_present = false;
    let resolved = resolve_background_work_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkRowDegradeReason::ToastOnlyNotDurable)
    );
    assert!(!resolved.reviewable_after_looking_away);
}

#[test]
fn row_toast_only_not_visible_is_not_ac1_violation() {
    let mut input = clean_row_input();
    input.durable_surface_present = false;
    input.adaptive_change_user_visible = false;
    let resolved = resolve_background_work_row(input).unwrap();
    assert_ne!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkRowDegradeReason::ToastOnlyNotDurable)
    );
}

#[test]
fn row_unnamed_class_degrades_first() {
    let mut input = clean_row_input();
    input.affected_work_class = None;
    let resolved = resolve_background_work_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkRowDegradeReason::AffectedWorkClassUnnamed)
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::NotEvaluated
    );
}

#[test]
fn row_override_presented_when_blocked_degrades() {
    let mut input = clean_row_input();
    input.override_posture = OverridePosture::PolicyBlocked;
    input.override_presented_available = true;
    let resolved = resolve_background_work_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkRowDegradeReason::OverridePresentedWhenBlocked)
    );
    assert!(resolved.presented_override_when_blocked);
    assert!(!resolved.override_available);
}

#[test]
fn row_missing_resume_condition_degrades() {
    let mut input = clean_row_input();
    input.resume_condition = None;
    let resolved = resolve_background_work_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkRowDegradeReason::ResumeConditionUnstated)
    );
}

#[test]
fn row_missing_preserved_degrades() {
    let mut input = clean_row_input();
    input.preserved_protected_tasks = vec![];
    let resolved = resolve_background_work_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkRowDegradeReason::WhatStillWorksUnstated)
    );
}

#[test]
fn row_empty_id_and_forbidden_material_error() {
    let mut input = clean_row_input();
    input.row_id = "  ".to_owned();
    assert_eq!(
        resolve_background_work_row(input).unwrap_err(),
        M5BackgroundWorkResolutionError::EmptyRowId
    );

    let mut input = clean_row_input();
    input.preserved_protected_tasks = vec!["bearer abc123".to_owned()];
    assert_eq!(
        resolve_background_work_row(input).unwrap_err(),
        M5BackgroundWorkResolutionError::ForbiddenMaterial
    );
}

#[test]
fn banner_clean_coalesces_and_names_work() {
    let resolved = resolve_background_work_banner(clean_banner_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.affected_work_named);
    assert!(!resolved.duplicate_toast_spam);
    assert_eq!(resolved.slowed_workloads, vec!["ai_warmup"]);
    assert_eq!(resolved.paused_workloads, vec!["indexing_refresh"]);
    assert!(resolved
        .work_dispositions
        .contains(&M5EfficiencyWorkDisposition::Paused));
    assert!(resolved
        .work_dispositions
        .contains(&M5EfficiencyWorkDisposition::Resuming));
}

#[test]
fn banner_duplicate_toast_degrades_ac2() {
    let mut input = clean_banner_input();
    input.coalesced_into_single_banner = false;
    let resolved = resolve_background_work_banner(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkBannerDegradeReason::DuplicateToastSpam)
    );
    assert!(resolved.duplicate_toast_spam);
}

#[test]
fn banner_single_event_not_coalesced_is_not_spam() {
    let mut input = clean_banner_input();
    input.pressure_event_count = 1;
    input.coalesced_into_single_banner = false;
    let resolved = resolve_background_work_banner(input).unwrap();
    assert!(!resolved.duplicate_toast_spam);
    assert_ne!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkBannerDegradeReason::DuplicateToastSpam)
    );
}

#[test]
fn banner_generic_copy_degrades_ac2() {
    let mut input = clean_banner_input();
    input.uses_generic_service_failure_copy = true;
    let resolved = resolve_background_work_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkBannerDegradeReason::GenericServiceFailureCopy)
    );
    assert!(resolved.generic_service_failure_copy);
}

#[test]
fn banner_paused_hidden_degrades() {
    let mut input = clean_banner_input();
    input.shows_paused_work_explicitly = false;
    let resolved = resolve_background_work_banner(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkBannerDegradeReason::PausedWorkNotExplicit)
    );
}

#[test]
fn banner_none_named_degrades_first() {
    let mut input = clean_banner_input();
    input.slowed_workloads = vec![];
    input.paused_workloads = vec![];
    let resolved = resolve_background_work_banner(input).unwrap();
    assert!(!resolved.affected_work_named);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BackgroundWorkBannerDegradeReason::NoAffectedWorkNamed)
    );
}

#[test]
fn banner_empty_id_and_forbidden_material_error() {
    let mut input = clean_banner_input();
    input.banner_id = "".to_owned();
    assert_eq!(
        resolve_background_work_banner(input).unwrap_err(),
        M5BackgroundWorkResolutionError::EmptyBannerId
    );

    let mut input = clean_banner_input();
    input.preserved_protected_tasks = vec!["https://relay.internal/leak".to_owned()];
    assert_eq!(
        resolve_background_work_banner(input).unwrap_err(),
        M5BackgroundWorkResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_background_work_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.vocabulary_set.work_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BACKGROUND_WORK_ROW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5BackgroundWorkAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5BackgroundWorkExportField::WorkDispositions);
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.controls_rows[0]
        .background_work_banner_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_row_example_fails() {
    let mut packet = seeded_m5_background_work_controls();
    // Force a clean row to also read as toast-only — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.background_work_row_examples[0].degrade_reason = None;
    row.background_work_row_examples[0].reviewable_after_looking_away = false;
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_banner_example_fails() {
    let mut packet = seeded_m5_background_work_controls();
    let row = &mut packet.controls_rows[1];
    row.background_work_banner_examples[0].degrade_reason = None;
    row.background_work_banner_examples[0].duplicate_toast_spam = true;
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_background_work_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.collapses_pressure_into_generic_service_failure = true,
            1 => row.hides_paused_work_behind_toast_only = true,
            2 => row.presents_override_available_when_policy_blocks = true,
            _ => row.drops_background_work_after_toast_dismissal = true,
        }
        assert!(packet
            .validate()
            .contains(&M5BackgroundWorkControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_toast_only_example_removed() {
    let mut packet = seeded_m5_background_work_controls();
    // Drop every toast-only row so no AC1-negative example remains.
    for row in &mut packet.controls_rows {
        row.background_work_row_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BackgroundWorkRowDegradeReason::ToastOnlyNotDurable)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_spam_example_removed() {
    let mut packet = seeded_m5_background_work_controls();
    for row in &mut packet.controls_rows {
        row.background_work_banner_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BackgroundWorkBannerDegradeReason::DuplicateToastSpam)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.governance_review.banner_coalesces_repeated_pressure = false;
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_background_work_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_background_work_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5BackgroundWorkControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_background_work_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_background_work_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_background_work_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_background_work_controls_export()
        .expect("checked M5 background-work controls export validates");
    assert_eq!(from_disk.packet_id, M5_BACKGROUND_WORK_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_background_work_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_background_work_controls_activity_center_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Beta);

    let preview = seeded_m5_background_work_controls_background_work_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::BackgroundWorkUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5BackgroundWorkControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-background-work-row-banner-controls/activity_center_beta_narrowed.json"
    )))
    .expect("activity-center fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_background_work_controls_activity_center_beta_narrowed()
    );

    let preview: M5BackgroundWorkControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-background-work-row-banner-controls/background_work_preview_narrowed.json"
    )))
    .expect("background-work fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_background_work_controls_background_work_preview_narrowed()
    );
}
