//! Unit tests for the open/save/reveal path-truth builder and validator.

use super::*;

fn clean_descriptor(flow_id: &str, kind: DialogFlowKind) -> SystemDialogFlow {
    let (reveal_side_effect, reveal_action_label_ref) = match kind {
        DialogFlowKind::RevealInSystemShell => (
            RevealSideEffect::SelectsTargetInFileManager,
            Some(format!("{flow_id}:reveal_label")),
        ),
        DialogFlowKind::OpenInDefaultBrowser => (
            RevealSideEffect::OpensDefaultBrowser,
            Some(format!("{flow_id}:reveal_label")),
        ),
        _ => (RevealSideEffect::NoExternalSideEffect, None),
    };
    let (write_posture, checkpoint_availability, checkpoint_ref) = match kind {
        DialogFlowKind::Save => (
            OverwritePosture::OverwriteWithCheckpoint,
            CheckpointAvailability::Pinned,
            Some(format!("{flow_id}:checkpoint")),
        ),
        DialogFlowKind::SaveAs => (
            OverwritePosture::CreateNewFile,
            CheckpointAvailability::NotApplicable,
            None,
        ),
        _ => (
            OverwritePosture::NoWriteAction,
            CheckpointAvailability::NotApplicable,
            None,
        ),
    };
    SystemDialogFlow {
        flow_id: flow_id.to_owned(),
        flow_kind: kind,
        descriptor_revision_ref: format!("{flow_id}:rev"),
        primary_label_ref: format!("{flow_id}:label"),
        literal_target_ref: format!("{flow_id}:literal"),
        literal_format: PathLiteralFormat::PosixPath,
        canonical_target_ref: format!("{flow_id}:canonical"),
        path_truth_class: PathTruthClass::LiteralIsCanonical,
        detected_target_kind: TargetKind::LocalFile,
        boundary_label: BoundaryLabel::LocalWritable,
        boundary_label_ref: format!("{flow_id}:boundary"),
        write_posture,
        checkpoint_availability,
        checkpoint_ref,
        overwrite_review_ref: "save:overwrite_review:checkpoint_aware:v1".to_owned(),
        reveal_side_effect,
        reveal_action_label_ref,
        filesystem_identity_ref: format!("{flow_id}:fs_identity"),
        save_coordination_ref: format!("{flow_id}:save_coord"),
        active_profile_owner_ref: format!("{flow_id}:profile"),
        trust_checkpoint_ref: format!("{flow_id}:trust"),
        canonical_command_ref: "cmd:workspace.save.target".to_owned(),
        path_condition: PathConditionClass::ExactAvailable,
        recovery_actions: vec![],
        continuity_note: "preserves canonical-path truth".to_owned(),
        degraded_state_vocabulary: vec!["Open this file".to_owned()],
        claimed_platforms: Platform::all().to_vec(),
        evidence_freshness: EvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        marketed: true,
        registered_on_path_truth_harness: true,
    }
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_open_save_reveal_report();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_open_save_reveal_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_covers_every_required_flow_kind() {
    let report = seeded_open_save_reveal_report();
    assert!(report.every_kind_present());
    for kind in DialogFlowKind::required_kinds() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.flow_kind == kind),
            "no registered flow for required kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn seeded_entries_are_sorted_by_flow_id() {
    let report = seeded_open_save_reveal_report();
    let ids: Vec<&str> = report
        .entries
        .iter()
        .map(|entry| entry.descriptor.flow_id.as_str())
        .collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    assert_eq!(ids, sorted, "entries must be sorted by flow id");
}

#[test]
fn degraded_flows_offer_recovery_actions() {
    let report = seeded_open_save_reveal_report();
    let mut degraded = 0usize;
    for entry in &report.entries {
        if entry.descriptor.path_condition.requires_recovery() {
            degraded += 1;
            assert!(
                !entry.descriptor.recovery_actions.is_empty(),
                "{} is degraded but offers no recovery",
                entry.descriptor.flow_id
            );
        }
    }
    assert_eq!(
        degraded, 4,
        "the four required failure-path cases must be present"
    );
}

#[test]
fn every_boundary_label_is_covered() {
    let report = seeded_open_save_reveal_report();
    for boundary in BoundaryLabel::all() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.boundary_label == boundary),
            "no flow exercises boundary {}",
            boundary.as_str()
        );
    }
}

#[test]
fn overwrite_without_checkpoint_is_caught() {
    let mut descriptor = clean_descriptor("flow:test.overwrite", DialogFlowKind::Save);
    descriptor.write_posture = OverwritePosture::OverwriteWithCheckpoint;
    descriptor.checkpoint_availability = CheckpointAvailability::Unavailable;
    descriptor.checkpoint_ref = None;
    let row = build_open_save_reveal_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        FlowBlockingFinding::OverwriteWithoutCheckpointReview { .. }
    )));
}

#[test]
fn read_only_write_attempt_is_caught() {
    let mut descriptor = clean_descriptor("flow:test.read_only", DialogFlowKind::Save);
    descriptor.boundary_label = BoundaryLabel::ReadOnly;
    descriptor.path_condition = PathConditionClass::ReadOnlyDestination;
    descriptor.write_posture = OverwritePosture::OverwriteWithCheckpoint;
    descriptor.recovery_actions = vec![RecoveryAction::SaveWritableCopyElsewhere];
    let row = build_open_save_reveal_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, FlowBlockingFinding::ReadOnlyWriteAttempt { .. })));
}

#[test]
fn generated_in_place_save_is_a_distinct_finding() {
    let mut descriptor = clean_descriptor("flow:test.generated", DialogFlowKind::Save);
    descriptor.boundary_label = BoundaryLabel::Generated;
    descriptor.path_condition = PathConditionClass::GeneratedOutput;
    descriptor.write_posture = OverwritePosture::OverwriteWithCheckpoint;
    descriptor.recovery_actions = vec![RecoveryAction::ExportInsteadOfSave];
    let row = build_open_save_reveal_row(descriptor);
    assert!(
        row.blocking_findings.iter().any(|finding| matches!(
            finding,
            FlowBlockingFinding::GeneratedTreatedAsInPlaceSave { .. }
        )),
        "a generated in-place save must be its own finding"
    );
    assert!(
        !row.blocking_findings
            .iter()
            .any(|finding| matches!(finding, FlowBlockingFinding::ReadOnlyWriteAttempt { .. })),
        "the generated and read-only failures must stay distinct"
    );
}

#[test]
fn hidden_reveal_side_effect_is_caught() {
    let mut descriptor = clean_descriptor("flow:test.reveal", DialogFlowKind::RevealInSystemShell);
    descriptor.reveal_side_effect = RevealSideEffect::NoExternalSideEffect;
    descriptor.reveal_action_label_ref = None;
    let row = build_open_save_reveal_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, FlowBlockingFinding::RevealSideEffectHidden { .. })));
}

#[test]
fn missing_reveal_label_is_caught() {
    let mut descriptor =
        clean_descriptor("flow:test.browser", DialogFlowKind::OpenInDefaultBrowser);
    // Correct side effect, but no disclosed action label.
    descriptor.reveal_side_effect = RevealSideEffect::OpensDefaultBrowser;
    descriptor.reveal_action_label_ref = None;
    let row = build_open_save_reveal_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, FlowBlockingFinding::RevealSideEffectHidden { .. })));
}

#[test]
fn distinct_recovery_failures_per_condition() {
    let cases = [
        (
            PathConditionClass::MissingCanonicalTarget,
            "flow:test.missing",
        ),
        (PathConditionClass::NetworkShareAlias, "flow:test.alias"),
        (
            PathConditionClass::GeneratedOutput,
            "flow:test.generated_nr",
        ),
        (
            PathConditionClass::ReadOnlyDestination,
            "flow:test.read_only_nr",
        ),
    ];
    for (condition, flow_id) in cases {
        let mut descriptor = clean_descriptor(flow_id, DialogFlowKind::SaveAs);
        descriptor.path_condition = condition;
        // Keep postures safe so only the recovery-missing finding fires.
        descriptor.write_posture = OverwritePosture::OverwriteReviewRequired;
        if condition == PathConditionClass::GeneratedOutput {
            descriptor.boundary_label = BoundaryLabel::Generated;
            descriptor.write_posture = OverwritePosture::ExportNotInPlaceSave;
        }
        if condition == PathConditionClass::ReadOnlyDestination {
            descriptor.boundary_label = BoundaryLabel::ReadOnly;
            descriptor.write_posture = OverwritePosture::WriteBlockedReadOnly;
        }
        descriptor.recovery_actions = vec![];
        let row = build_open_save_reveal_row(descriptor);
        let expected = condition.missing_recovery_failure_mode();
        assert_eq!(
            row.blocking_findings
                .iter()
                .filter_map(FlowBlockingFinding::failure_mode)
                .find(|mode| Some(*mode) == expected),
            expected,
            "{flow_id} did not raise the distinct recovery failure for {}",
            condition.as_str()
        );
    }
}

#[test]
fn bypassed_trust_and_hidden_canonical_are_caught() {
    let mut descriptor = clean_descriptor("flow:test.trust", DialogFlowKind::Open);
    descriptor.trust_checkpoint_ref = String::new();
    descriptor.canonical_target_ref = "   ".to_owned();
    let row = build_open_save_reveal_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, FlowBlockingFinding::TrustEvaluationBypassed { .. })));
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, FlowBlockingFinding::CanonicalPathHidden { .. })));
}

#[test]
fn missing_filesystem_and_save_refs_are_caught() {
    let mut descriptor = clean_descriptor("flow:test.refs", DialogFlowKind::Save);
    descriptor.filesystem_identity_ref = String::new();
    descriptor.save_coordination_ref = "  ".to_owned();
    let row = build_open_save_reveal_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        FlowBlockingFinding::MissingFilesystemIdentityRef { .. }
    )));
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        FlowBlockingFinding::MissingSaveCoordinationRef { .. }
    )));
}

#[test]
fn divergent_overwrite_review_vocabulary_is_caught() {
    let mut descriptor = clean_descriptor("flow:test.vocab", DialogFlowKind::SaveAs);
    descriptor.overwrite_review_ref = String::new();
    let row = build_open_save_reveal_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        FlowBlockingFinding::CheckpointVocabularyDivergence { .. }
    )));
}

#[test]
fn stale_evidence_on_marketed_flow_is_a_blocker() {
    let mut descriptor = clean_descriptor("flow:test.stale", DialogFlowKind::Open);
    descriptor.evidence_freshness = EvidenceFreshness::Stale;
    let row = build_open_save_reveal_row(descriptor);
    assert!(row.blocking_findings.iter().any(|finding| matches!(
        finding,
        FlowBlockingFinding::StaleEvidenceOnMarketedFlow { .. }
    )));
}

#[test]
fn support_export_quotes_every_flow() {
    let report = seeded_open_save_reveal_report();
    let export = OpenSaveRevealSupportExport::from_report(
        OPEN_SAVE_REVEAL_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert_eq!(export.support_export_id, OPEN_SAVE_REVEAL_SUPPORT_EXPORT_ID);
    for entry in &report.entries {
        assert!(export.case_ids.contains(&entry.descriptor.flow_id));
        assert!(export
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn case_exports_cover_the_four_incidents() {
    let exports = seeded_open_save_reveal_case_exports();
    assert_eq!(exports.len(), 4);
    let labels: Vec<&str> = exports.iter().map(|e| e.case_label.as_str()).collect();
    assert_eq!(
        labels,
        vec![
            "missing_canonical_target",
            "network_share_alias",
            "generated_output",
            "read_only_destination"
        ]
    );
    for export in &exports {
        assert_ne!(export.path_condition, PathConditionClass::ExactAvailable);
        assert!(!export.recovery_actions.is_empty());
        assert_eq!(export.record_kind, OPEN_SAVE_REVEAL_CASE_EXPORT_RECORD_KIND);
    }
}

#[test]
fn validator_flags_a_blocking_finding() {
    let mut report = seeded_open_save_reveal_report();
    if let Some(entry) = report.entries.first_mut() {
        let mut descriptor = entry.descriptor.clone();
        descriptor.active_profile_owner_ref = String::new();
        *entry = build_open_save_reveal_row(descriptor);
    }
    let errors = validate_open_save_reveal_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        OpenSaveRevealValidationError::BlockingFindingPresent { .. }
    )));
}
