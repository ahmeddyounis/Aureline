use super::*;

use crate::efficiency::governance::EfficiencyRecoveryState;

fn clean_card_input() -> M5ResumeSummaryCardResolutionInput {
    M5ResumeSummaryCardResolutionInput {
        card_id: "resume-card:test".to_owned(),
        recovery_state: EfficiencyRecoveryState::StagedResume,
        resumed_workloads: vec![WorkloadFamily::IndexingRefresh, WorkloadFamily::AiWarmup],
        backlog_workloads: vec![WorkloadFamily::PreviewRefresh],
        backlog_known: true,
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        stale_results_visible: true,
        durable_summary_present: true,
        next_safe_action_stated: true,
        proof_fresh: true,
    }
}

fn clean_note_input() -> M5StaleResultNoteResolutionInput {
    M5StaleResultNoteResolutionInput {
        note_id: "stale-note:test".to_owned(),
        stale_result_state: M5EfficiencyStaleResultState::StaleResultRetained,
        returned_to_nominal: true,
        stale_results_visible: true,
        based_on_constrained_state_stated: true,
        refresh_path_stated: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_resume_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RESUME_CONTROLS_PACKET_ID);
}

#[test]
fn card_clean_lists_resumed_backlog_and_keeps_stale_visible() {
    let resolved = resolve_resume_summary_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.stale_evidence_preserved);
    assert!(!resolved.silently_dropped_stale_evidence);
    assert_eq!(resolved.recovery_state, "staged_resume");
    assert_eq!(
        resolved.resumed_workloads,
        vec!["indexing_refresh", "ai_warmup"]
    );
    assert_eq!(resolved.backlog_workloads, vec!["preview_refresh"]);
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::StaleResultShown
    );
    assert!(resolved.stale_results_visible);
    assert!(resolved.next_safe_action_stated);
    assert_eq!(resolved.next_action, M5ResumeNextAction::NoActionNeeded);
}

#[test]
fn card_recovered_with_no_backlog_is_running_full() {
    let mut input = clean_card_input();
    input.recovery_state = EfficiencyRecoveryState::Recovered;
    input.resumed_workloads = vec![WorkloadFamily::IndexingRefresh];
    input.backlog_workloads = vec![];
    input.stale_result_state = M5EfficiencyStaleResultState::FreshResult;
    input.stale_results_visible = false;
    let resolved = resolve_resume_summary_card(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::RunningFull
    );
}

#[test]
fn card_stale_dropped_degrades_ac1() {
    let mut input = clean_card_input();
    input.stale_results_visible = false;
    let resolved = resolve_resume_summary_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ResumeSummaryCardDegradeReason::StaleResultEvidenceDropped)
    );
    assert!(resolved.silently_dropped_stale_evidence);
    assert!(!resolved.stale_evidence_preserved);
    assert_eq!(resolved.next_action, M5ResumeNextAction::ReviewStaleResults);
}

#[test]
fn card_not_durable_degrades_ac2() {
    let mut input = clean_card_input();
    input.stale_result_state = M5EfficiencyStaleResultState::FreshResult;
    input.stale_results_visible = false;
    input.durable_summary_present = false;
    let resolved = resolve_resume_summary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ResumeSummaryCardDegradeReason::RecoverySummaryNotDurable)
    );
}

#[test]
fn card_backlog_hidden_degrades_ac2() {
    let mut input = clean_card_input();
    input.stale_result_state = M5EfficiencyStaleResultState::FreshResult;
    input.stale_results_visible = false;
    input.backlog_known = false;
    let resolved = resolve_resume_summary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ResumeSummaryCardDegradeReason::ResumeBacklogHidden)
    );
}

#[test]
fn card_resumed_unnamed_degrades_first_and_is_not_evaluated() {
    let mut input = clean_card_input();
    input.resumed_workloads = vec![];
    let resolved = resolve_resume_summary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ResumeSummaryCardDegradeReason::ResumedWorkUnnamed)
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::NotEvaluated
    );
}

#[test]
fn card_next_action_unstated_degrades() {
    let mut input = clean_card_input();
    input.stale_result_state = M5EfficiencyStaleResultState::FreshResult;
    input.stale_results_visible = false;
    input.next_safe_action_stated = false;
    assert_eq!(
        resolve_resume_summary_card(input).unwrap().degrade_reason,
        Some(M5ResumeSummaryCardDegradeReason::NextSafeActionUnstated)
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "  ".to_owned();
    assert_eq!(
        resolve_resume_summary_card(input).unwrap_err(),
        M5ResumeResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.card_id = "resume-card:https://relay.internal/resume".to_owned();
    assert_eq!(
        resolve_resume_summary_card(input).unwrap_err(),
        M5ResumeResolutionError::ForbiddenMaterial
    );
}

#[test]
fn note_clean_keeps_stale_visible_and_states_prior_state() {
    let resolved = resolve_stale_result_continuity_note(clean_note_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.stale_evidence_preserved);
    assert_eq!(resolved.stale_result_state, "stale_result_retained");
    assert!(resolved.stale_results_visible);
    assert!(resolved.based_on_constrained_state_stated);
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::StaleResultShown
    );
}

#[test]
fn note_superseded_is_clean_and_running_full() {
    let mut input = clean_note_input();
    input.stale_result_state = M5EfficiencyStaleResultState::StaleResultSuperseded;
    input.stale_results_visible = false;
    let resolved = resolve_stale_result_continuity_note(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::RunningFull
    );
}

#[test]
fn note_silently_removed_degrades_ac1() {
    let mut input = clean_note_input();
    input.stale_results_visible = false;
    let resolved = resolve_stale_result_continuity_note(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5StaleResultNoteDegradeReason::StaleEvidenceSilentlyRemoved)
    );
    assert!(resolved.silently_removed_stale_evidence);
}

#[test]
fn note_prior_unstated_and_refresh_unstated_degrade() {
    let mut input = clean_note_input();
    input.based_on_constrained_state_stated = false;
    assert_eq!(
        resolve_stale_result_continuity_note(input)
            .unwrap()
            .degrade_reason,
        Some(M5StaleResultNoteDegradeReason::PriorConstrainedStateUnstated)
    );

    let mut input = clean_note_input();
    input.stale_result_state = M5EfficiencyStaleResultState::StaleResultRefreshing;
    input.refresh_path_stated = false;
    assert_eq!(
        resolve_stale_result_continuity_note(input)
            .unwrap()
            .degrade_reason,
        Some(M5StaleResultNoteDegradeReason::RefreshPathUnstated)
    );
}

#[test]
fn note_continuity_unknown_degrades_first() {
    let mut input = clean_note_input();
    input.stale_result_state = M5EfficiencyStaleResultState::ContinuityUnknown;
    let resolved = resolve_stale_result_continuity_note(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5StaleResultNoteDegradeReason::ContinuityUnknown)
    );
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::NotEvaluated
    );
}

#[test]
fn note_empty_id_and_forbidden_material_error() {
    let mut input = clean_note_input();
    input.note_id = "".to_owned();
    assert_eq!(
        resolve_stale_result_continuity_note(input).unwrap_err(),
        M5ResumeResolutionError::EmptyNoteId
    );

    let mut input = clean_note_input();
    input.note_id = "stale-note:-----begin key".to_owned();
    assert_eq!(
        resolve_stale_result_continuity_note(input).unwrap_err(),
        M5ResumeResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_resume_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.vocabulary_set.work_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RESUME_SUMMARY_CARD_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ResumeAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ResumeExportField::WorkDispositions);
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.controls_rows[0].stale_result_note_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_card_example_fails() {
    let mut packet = seeded_m5_resume_controls();
    // Force a clean card to also drop stale evidence — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.resume_summary_examples[0].degrade_reason = None;
    row.resume_summary_examples[0].silently_dropped_stale_evidence = true;
    row.resume_summary_examples[0].stale_evidence_preserved = false;
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_note_example_fails() {
    let mut packet = seeded_m5_resume_controls();
    let row = &mut packet.controls_rows[0];
    row.stale_result_note_examples[0].degrade_reason = None;
    row.stale_result_note_examples[0].silently_removed_stale_evidence = true;
    row.stale_result_note_examples[0].stale_evidence_preserved = false;
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_resume_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.clears_stale_result_context_on_resume = true,
            1 => row.requires_inferring_recovery_from_transient_banners = true,
            2 => row.hides_resumed_work_backlog = true,
            _ => row.collapses_pressure_sources_into_generic_warning = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ResumeControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_dropped_example_removed() {
    let mut packet = seeded_m5_resume_controls();
    // Drop every stale-dropped example so no AC1-negative example remains.
    for row in &mut packet.controls_rows {
        row.resume_summary_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ResumeSummaryCardDegradeReason::StaleResultEvidenceDropped)
        });
        row.stale_result_note_examples.retain(|ex| {
            ex.degrade_reason != Some(M5StaleResultNoteDegradeReason::StaleEvidenceSilentlyRemoved)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_backlog_hidden_example_removed() {
    let mut packet = seeded_m5_resume_controls();
    for row in &mut packet.controls_rows {
        row.resume_summary_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ResumeSummaryCardDegradeReason::ResumeBacklogHidden)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet
        .governance_review
        .no_stale_result_context_cleared_on_resume = false;
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_resume_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_resume_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ResumeControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_resume_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_resume_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_resume_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_resume_controls_export()
        .expect("checked M5 resume controls export validates");
    assert_eq!(from_disk.packet_id, M5_RESUME_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_resume_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_resume_controls_activity_center_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Beta);

    let preview = seeded_m5_resume_controls_background_work_preview_narrowed();
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
    let beta: M5ResumeControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-resume-summary-stale-note-controls/activity_center_beta_narrowed.json"
    )))
    .expect("activity-center fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_resume_controls_activity_center_beta_narrowed()
    );

    let preview: M5ResumeControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-resume-summary-stale-note-controls/background_work_preview_narrowed.json"
    )))
    .expect("background-work fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_resume_controls_background_work_preview_narrowed()
    );
}
