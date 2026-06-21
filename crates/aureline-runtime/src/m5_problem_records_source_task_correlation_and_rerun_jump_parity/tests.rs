use super::*;

/// The canonical problem ids exercised by the perturbation tests.
const ROW_STRUCTURED_DIAGNOSTIC: &str = "problem:local-structured-diagnostic:0001";
const ROW_TEST_NORMALIZED: &str = "problem:local-test-normalized-event:0001";
const ROW_HEURISTIC_PARSE: &str = "problem:local-heuristic-parse:0001";
const ROW_IMPORTED_ANNOTATION: &str = "problem:imported-provider-annotation:0001";
const ROW_PIPELINE_RUN: &str = "problem:pipeline-provider-run:0001";
const ROW_NOTEBOOK_SUPERSEDED: &str = "problem:notebook-superseded:0001";
const ROW_HEADLESS_STALE: &str = "problem:headless-stale-run:0001";
const ROW_DOWNGRADED_MAPPING: &str = "problem:local-downgraded-mapping:0001";
const ROW_GATED_RERUN: &str = "problem:extension-gated-rerun:0001";
const ROW_FLOORED: &str = "problem:local-lineage-lost-floored:0001";
const ROW_LABS: &str = "problem:labs-cross-run-correlation:0001";

fn canonical() -> M5ProblemRecordSetPacket {
    current_m5_problem_record_set().expect("canonical problem-record set loads and validates")
}

fn record<'a>(packet: &'a M5ProblemRecordSetPacket, problem_id: &str) -> &'a ProblemRecord {
    packet
        .records
        .iter()
        .find(|record| record.problem_id == problem_id)
        .unwrap_or_else(|| panic!("missing row {problem_id}"))
}

fn cloned(packet: &M5ProblemRecordSetPacket, problem_id: &str) -> ProblemRecord {
    record(packet, problem_id).clone()
}

// --------------------------------------------------------------------------- //
// Canonical packet.
// --------------------------------------------------------------------------- //

#[test]
fn checked_in_artifact_matches_the_in_crate_builder() {
    // The dump example regenerates the support export from this builder; the two
    // must stay byte-aligned so the artifact never drifts away from Rust.
    assert_eq!(canonical(), seeded_problem_record_set());
}

#[test]
fn canonical_export_loads_and_validates_clean() {
    let packet = canonical();
    assert_eq!(packet.record_kind, M5_PROBLEM_RECORDS_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_PROBLEM_RECORDS_SCHEMA_VERSION);
    assert_eq!(packet.taxonomy_version, M5_PROBLEM_RECORDS_TAXONOMY_VERSION);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.records.len(), 11);
}

#[test]
fn canonical_status_distribution_matches_report() {
    let dist = canonical().status_distribution();
    assert_eq!(dist.actionable, 4);
    assert_eq!(dist.narrowed, 3);
    assert_eq!(dist.read_only_imported, 2);
    assert_eq!(dist.raw_evidence_only, 1);
    assert_eq!(dist.labs, 1);
}

#[test]
fn canonical_packet_covers_every_claimed_source_kind() {
    let kinds = canonical().represented_source_kinds();
    for kind in [
        ProblemSourceKind::StructuredLanguageDiagnostic,
        ProblemSourceKind::NormalizedTaskEvent,
        ProblemSourceKind::HeuristicOutputParse,
        ProblemSourceKind::ImportedProviderAnnotation,
    ] {
        assert!(kinds.contains(&kind), "missing source kind {kind:?}");
    }
}

#[test]
fn canonical_export_carries_no_forbidden_material() {
    let packet = canonical();
    let value = serde_json::to_value(&packet).expect("serializes");
    assert!(!json_contains_forbidden_boundary_material(&value));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = canonical();
    let json = packet.export_safe_json();
    let reparsed: M5ProblemRecordSetPacket = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(reparsed, packet);
    assert!(reparsed.validate().is_empty());
}

#[test]
fn markdown_summary_lists_rows_and_counts() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("# M5 Problem Records"));
    assert!(summary.contains("4 actionable, 3 narrowed, 2 read-only imported"));
    assert!(summary.contains(ROW_FLOORED));
}

// --------------------------------------------------------------------------- //
// Acceptance criterion 1: the four origins stay inspectable.
// --------------------------------------------------------------------------- //

#[test]
fn each_origin_kind_is_inspectable_on_its_row() {
    let packet = canonical();
    assert_eq!(
        record(&packet, ROW_STRUCTURED_DIAGNOSTIC).parse_class,
        ProblemSourceKind::StructuredLanguageDiagnostic
    );
    assert_eq!(
        record(&packet, ROW_TEST_NORMALIZED).parse_class,
        ProblemSourceKind::NormalizedTaskEvent
    );
    assert_eq!(
        record(&packet, ROW_HEURISTIC_PARSE).parse_class,
        ProblemSourceKind::HeuristicOutputParse
    );
    assert_eq!(
        record(&packet, ROW_IMPORTED_ANNOTATION).parse_class,
        ProblemSourceKind::ImportedProviderAnnotation
    );
}

// --------------------------------------------------------------------------- //
// Per-record derivation (mirrors the perturbation corpus).
// --------------------------------------------------------------------------- //

#[test]
fn clean_structured_row_stays_actionable() {
    let packet = canonical();
    let decision = record(&packet, ROW_STRUCTURED_DIAGNOSTIC).narrow(false);
    assert_eq!(decision.effective_status, ProblemRecordStatus::Actionable);
    assert!(!decision.narrowed);
    assert!(decision.active_downgrade_reasons.is_empty());
}

#[test]
fn origin_flattened_floors() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_HEURISTIC_PARSE);
    row.evidence.structured_vs_heuristic_distinct = false;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::OriginFlattened));
}

#[test]
fn heuristic_without_backlink_floors_and_keeps_fallback_label() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_HEURISTIC_PARSE);
    row.evidence.raw_output_backlink_present = false;
    let decision = row.narrow(false);
    assert_eq!(decision.claimed_status, ProblemRecordStatus::Actionable);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert!(decision.narrowed);
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::RawBacklinkMissing]
    );
    assert_eq!(
        row.effective_confidence(decision.effective_status),
        ConfidenceTier::UnmappedRequiresReview
    );
    let label = row.narrowed_label(&decision).expect("floored label");
    assert!(!label_is_generic(&label));
    assert!(label.contains("reopenable"));
}

#[test]
fn heuristic_with_structured_tier_narrows_confidence() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_HEURISTIC_PARSE);
    row.declared_confidence_tier = ConfidenceTier::StructuredFull;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::ConfidenceUnlabeled]
    );
}

#[test]
fn source_ref_missing_floors() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.source.source_tool_ref = None;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::SourceRefMissing]
    );
}

#[test]
fn anchor_missing_narrows_and_disables_jump() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.anchor.file_ref = None;
    row.anchor.start_line = None;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::AnchorMissing));
    assert_eq!(
        decision.actions.jump_to_source,
        ActionAvailability::Unavailable
    );
}

#[test]
fn owning_channel_missing_narrows_and_disables_open_output() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.correlations.owning_output_channel_ref = None;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::OwningChannelMissing));
    assert_eq!(
        decision.actions.open_owning_output,
        ActionAvailability::Unavailable
    );
}

#[test]
fn source_task_uncorrelated_narrows_and_disables_rerun() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.correlations.source_task_ref = None;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::SourceTaskUncorrelated));
    assert_eq!(
        decision.actions.rerun_or_inspect_originator,
        ActionAvailability::Unavailable
    );
}

#[test]
fn editor_decoration_uncorrelated_narrows() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.correlations.editor_decoration_ref = None;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::EditorDecorationUncorrelated));
}

#[test]
fn timeline_uncorrelated_narrows() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.correlations.timeline_entry_ref = None;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::TimelineUncorrelated));
}

#[test]
fn superseded_marked_stays_visibly_classified() {
    // Acceptance criterion 3: a superseded retry stays visibly classified, not
    // silently dropped and not silently upgraded back to fresh certainty.
    let packet = canonical();
    let decision = record(&packet, ROW_NOTEBOOK_SUPERSEDED).narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::Superseded]
    );
}

#[test]
fn superseded_without_marker_floors() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_NOTEBOOK_SUPERSEDED);
    row.evidence.superseded_state_marked = false;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::SupersededNotMarked]
    );
}

#[test]
fn stale_run_stays_visibly_classified() {
    let packet = canonical();
    let decision = record(&packet, ROW_HEADLESS_STALE).narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::StaleRun]
    );
}

#[test]
fn downgraded_mapping_stays_visibly_classified() {
    let packet = canonical();
    let decision = record(&packet, ROW_DOWNGRADED_MAPPING).narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::DowngradedMapping]
    );
}

#[test]
fn evidence_missing_floors() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.declared_freshness_state = FreshnessState::Missing;
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert!(decision
        .active_downgrade_reasons
        .contains(&ProblemDowngradeReason::EvidenceMissing));
    assert_eq!(
        decision.actions.jump_to_source,
        ActionAvailability::Unavailable
    );
}

#[test]
fn canonical_floored_row_keeps_raw_fallback() {
    let packet = canonical();
    let row = record(&packet, ROW_FLOORED);
    let decision = row.narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert!(row.floored_row_keeps_fallback(decision.effective_status));
    assert!(ev_backlink_present(row));
}

// --------------------------------------------------------------------------- //
// Acceptance criterion 2: jump / open-output / rerun-or-inspect parity.
// --------------------------------------------------------------------------- //

#[test]
fn local_row_offers_all_three_actions() {
    let packet = canonical();
    let decision = record(&packet, ROW_TEST_NORMALIZED).narrow(false);
    assert_eq!(
        decision.actions.jump_to_source,
        ActionAvailability::Available
    );
    assert_eq!(
        decision.actions.open_owning_output,
        ActionAvailability::Available
    );
    assert_eq!(
        decision.actions.rerun_or_inspect_originator,
        ActionAvailability::Available
    );
}

#[test]
fn structured_diagnostic_without_a_channel_marks_open_output_not_applicable() {
    let packet = canonical();
    let decision = record(&packet, ROW_STRUCTURED_DIAGNOSTIC).narrow(false);
    assert_eq!(
        decision.actions.open_owning_output,
        ActionAvailability::NotApplicable
    );
}

#[test]
fn authority_gated_rerun_is_surfaced_not_dropped() {
    let packet = canonical();
    let decision = record(&packet, ROW_GATED_RERUN).narrow(false);
    // The row stays actionable; only the rerun action is gated.
    assert_eq!(decision.effective_status, ProblemRecordStatus::Actionable);
    assert_eq!(
        decision.actions.rerun_or_inspect_originator,
        ActionAvailability::GatedRequiresAuthority
    );
}

#[test]
fn imported_row_inspects_read_only_and_never_reruns_locally() {
    let packet = canonical();
    let decision = record(&packet, ROW_IMPORTED_ANNOTATION).narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::ReadOnlyImported
    );
    assert_eq!(
        decision.actions.rerun_or_inspect_originator,
        ActionAvailability::ReadOnlyInspectOnly
    );
    // The imported row still jumps to source and opens its provider output.
    assert_eq!(
        decision.actions.jump_to_source,
        ActionAvailability::Available
    );
    assert_eq!(
        decision.actions.open_owning_output,
        ActionAvailability::Available
    );
}

#[test]
fn rerun_denied_by_policy_is_unavailable() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_TEST_NORMALIZED);
    row.correlations.rerun_authority = RerunAuthority::DeniedPolicy;
    let decision = row.narrow(false);
    assert_eq!(
        decision.actions.rerun_or_inspect_originator,
        ActionAvailability::Unavailable
    );
}

// --------------------------------------------------------------------------- //
// Overlay / proof / labs.
// --------------------------------------------------------------------------- //

#[test]
fn overlay_dropping_read_only_marker_floors() {
    let packet = canonical();
    let mut row = cloned(&packet, ROW_IMPORTED_ANNOTATION);
    row.evidence.imported_overlay_read_only = false;
    let decision = row.narrow(false);
    assert_eq!(
        decision.claimed_status,
        ProblemRecordStatus::ReadOnlyImported
    );
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::RawEvidenceOnly
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::ImportedOverlayClaimsLive]
    );
}

#[test]
fn imported_overlay_with_cached_snapshot_stays_overlay() {
    let packet = canonical();
    let decision = record(&packet, ROW_PIPELINE_RUN).narrow(false);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::ReadOnlyImported
    );
    assert!(!decision.narrowed);
    assert!(decision.active_downgrade_reasons.is_empty());
}

#[test]
fn elapsed_verification_window_narrows_current_proof() {
    let packet = canonical();
    let decision = record(&packet, ROW_TEST_NORMALIZED).narrow(true);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::NarrowedActionable
    );
    assert_eq!(
        decision.active_downgrade_reasons,
        vec![ProblemDowngradeReason::StaleProof]
    );
}

#[test]
fn labs_row_makes_no_claim_and_never_narrows() {
    let packet = canonical();
    let decision = record(&packet, ROW_LABS).narrow(false);
    assert_eq!(decision.claimed_status, ProblemRecordStatus::LabsNotClaimed);
    assert_eq!(
        decision.effective_status,
        ProblemRecordStatus::LabsNotClaimed
    );
    assert!(!decision.narrowed);
    assert!(decision.active_downgrade_reasons.is_empty());
}

// --------------------------------------------------------------------------- //
// Projection guard.
// --------------------------------------------------------------------------- //

#[test]
fn projection_guard_refuses_imported_rendered_as_actionable() {
    let packet = canonical();
    let decision = record(&packet, ROW_IMPORTED_ANNOTATION).narrow(false);
    assert!(decision.surface_overclaims(ProblemRecordStatus::Actionable));
    assert!(!decision.surface_overclaims(ProblemRecordStatus::ReadOnlyImported));
    assert!(!decision.surface_overclaims(ProblemRecordStatus::RawEvidenceOnly));
}

#[test]
fn labs_projection_only_renders_as_labs() {
    assert!(ProblemRecordStatus::LabsNotClaimed.overclaims_as(ProblemRecordStatus::Actionable));
    assert!(!ProblemRecordStatus::LabsNotClaimed.overclaims_as(ProblemRecordStatus::LabsNotClaimed));
    assert!(ProblemRecordStatus::Actionable.overclaims_as(ProblemRecordStatus::LabsNotClaimed));
}

// --------------------------------------------------------------------------- //
// Validation negatives.
// --------------------------------------------------------------------------- //

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = canonical();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::WrongRecordKind));
}

#[test]
fn invalid_redaction_class_is_rejected() {
    let mut packet = canonical();
    packet.redaction_class_token = "raw_dump".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::InvalidRedactionClass));
}

#[test]
fn overlay_without_provider_ref_is_rejected() {
    let mut packet = canonical();
    for row in &mut packet.records {
        if row.problem_id == ROW_IMPORTED_ANNOTATION {
            row.source.provider_ref = None;
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::OverlayMissingProviderRef));
}

#[test]
fn floored_row_without_fallback_is_rejected() {
    let mut packet = canonical();
    for row in &mut packet.records {
        if row.problem_id == ROW_FLOORED {
            row.evidence.raw_output_backlink_present = false;
            row.source.raw_output_backlink_ref = None;
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::FlooredRowLosesFallback));
}

#[test]
fn missing_source_kind_is_rejected() {
    let mut packet = canonical();
    packet
        .records
        .retain(|row| row.parse_class != ProblemSourceKind::ImportedProviderAnnotation);
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::ProblemSourceKindMissing));
}

#[test]
fn duplicate_problem_id_is_rejected() {
    let mut packet = canonical();
    let dup = packet.records[0].clone();
    packet.records.push(dup);
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::DuplicateProblemId));
}

#[test]
fn packet_with_no_narrowing_demonstration_is_rejected() {
    let mut packet = canonical();
    // Keep only the clean actionable rows plus enough source-kind coverage.
    packet.records.retain(|row| {
        matches!(
            row.problem_id.as_str(),
            ROW_STRUCTURED_DIAGNOSTIC
                | ROW_TEST_NORMALIZED
                | ROW_HEURISTIC_PARSE
                | ROW_IMPORTED_ANNOTATION
        )
    });
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::DowngradedRowCaseMissing));
}

#[test]
fn forbidden_material_in_label_is_rejected() {
    let mut packet = canonical();
    packet.records[0].label_summary = "token bearer abcdef".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ProblemRecordsViolation::RawBoundaryMaterialInExport));
}

// --------------------------------------------------------------------------- //
// Builder and freshness.
// --------------------------------------------------------------------------- //

#[test]
fn builder_seals_record_constants() {
    let packet = canonical();
    let built = M5ProblemRecordSetPacket::new(M5ProblemRecordSetInput {
        packet_id: packet.packet_id.clone(),
        label: packet.label.clone(),
        as_of: packet.as_of.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        verification_freshness: packet.verification_freshness.clone(),
        records: packet.records.clone(),
    });
    assert_eq!(built, packet);
    assert!(built.validate().is_empty());
}

#[test]
fn freshness_window_uses_the_slo() {
    let packet = canonical();
    assert!(!packet.stale_window());
    assert!(!packet.freshness_stale_at("2026-06-21T12:00:00Z"));
    assert!(packet.freshness_stale_at("2026-06-30T00:00:00Z"));
}

// --------------------------------------------------------------------------- //
// Token stability.
// --------------------------------------------------------------------------- //

#[test]
fn enum_tokens_round_trip_through_serde() {
    for action in ProblemAction::ALL {
        let json = serde_json::to_string(&action).expect("serializes");
        assert_eq!(json, format!("\"{}\"", action.as_str()));
    }
    for status in [
        ProblemRecordStatus::RawEvidenceOnly,
        ProblemRecordStatus::ReadOnlyImported,
        ProblemRecordStatus::NarrowedActionable,
        ProblemRecordStatus::Actionable,
        ProblemRecordStatus::LabsNotClaimed,
    ] {
        let json = serde_json::to_string(&status).expect("serializes");
        assert_eq!(json, format!("\"{}\"", status.as_str()));
    }
    for severity in [
        ProblemSeverity::Info,
        ProblemSeverity::Warning,
        ProblemSeverity::Error,
        ProblemSeverity::Fatal,
    ] {
        let json = serde_json::to_string(&severity).expect("serializes");
        assert_eq!(json, format!("\"{}\"", severity.as_str()));
    }
}
