use super::*;

fn packet() -> DocsValidationReportPacket {
    DocsValidationReportPacket::materialize(seeded_stable_docs_validation_report_input())
}

#[test]
fn seeded_report_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, DOCS_VALIDATION_REPORT_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_VALIDATION_REPORT_SCHEMA_VERSION);
}

#[test]
fn report_covers_required_subject_kinds_and_modes() {
    let packet = packet();
    let kinds: BTreeSet<ValidationSubjectKind> =
        packet.rows.iter().map(|r| r.subject.subject_kind).collect();
    for required in ValidationSubjectKind::REQUIRED {
        assert!(
            kinds.contains(&required),
            "missing subject kind {required:?}"
        );
    }
    let modes: BTreeSet<ValidationMode> = packet.rows.iter().map(|r| r.mode).collect();
    for required in ValidationMode::REQUIRED {
        assert!(modes.contains(&required), "missing mode {required:?}");
    }
}

#[test]
fn seed_demonstrates_all_eight_modes() {
    let modes: BTreeSet<&str> = packet().rows.iter().map(|r| r.mode.as_str()).collect();
    for expected in [
        "rendered",
        "syntax_checked",
        "executed_local",
        "executed_remote",
        "skipped",
        "stale",
        "unsupported",
        "broken_link",
    ] {
        assert!(
            modes.contains(expected),
            "seed is missing mode `{expected}`"
        );
    }
}

#[test]
fn every_row_names_mode_scope_producer_and_actions() {
    for row in packet().rows {
        // Concrete subject.
        assert!(row.subject.names_concrete_subject());
        assert!(!row.detail.trim().is_empty());
        // Explicit last-checked and scope.
        assert!(!row.last_checked_at.trim().is_empty());
        assert!(row.scope.is_complete());
        // Producer attribution.
        assert!(row.produced_by.names_concrete_context());
        assert!(row.mode.permits_validator(row.produced_by.validator));
        assert!(row.actions.preserves_producer);
        // Mode/outcome consistency: a non-executed row never claims execution.
        if row.outcome.claims_execution() {
            assert!(row.mode.is_executed());
        }
        assert!(row.mode.outcome_is_consistent(row.outcome));
        // Provenance disclosure present.
        assert!(!row.provenance_disclosure_note.trim().is_empty());
        // Actionable findings carry a trace.
        if row.requires_source_trace() {
            assert!(!row.source_trace_ref.trim().is_empty());
        }
        // Action parity.
        assert!(row.actions.parity_complete());
        // Suppressed rows stay attributable and reopenable.
        assert!(row.suppression.is_attributable());
        assert!(row.suppression.is_reopenable());
        // Touch each token so it stays stable across refactors.
        let _ = (
            row.subject.subject_kind.as_str(),
            row.mode.as_str(),
            row.outcome.as_str(),
            row.produced_by.validator.as_str(),
            row.provenance.as_str(),
            row.suppression.state.as_str(),
            row.chips.freshness.as_str(),
            row.chips.version_match.as_str(),
            row.chips.locality.as_str(),
            row.scope.version_match.as_str(),
        );
    }
}

#[test]
fn rendered_row_is_distinguished_from_executed_row() {
    let packet = packet();
    let rendered = packet
        .rows
        .iter()
        .find(|r| r.mode == ValidationMode::Rendered)
        .expect("rendered row present");
    assert!(!rendered.outcome.claims_execution());
    assert_eq!(rendered.outcome, ValidationOutcome::RenderedPreviewOnly);

    let executed = packet
        .rows
        .iter()
        .find(|r| r.mode == ValidationMode::ExecutedLocal)
        .expect("executed row present");
    assert!(executed.outcome.claims_execution());
}

#[test]
fn broken_link_and_stale_findings_are_traced() {
    let packet = packet();
    for mode in [ValidationMode::BrokenLink, ValidationMode::Stale] {
        let row = packet
            .rows
            .iter()
            .find(|r| r.mode == mode)
            .unwrap_or_else(|| panic!("{mode:?} row present"));
        assert!(row.requires_source_trace());
        assert!(!row.source_trace_ref.trim().is_empty());
        assert!(row.actions.parity_complete());
    }
}

#[test]
fn report_demonstrates_cached_and_imported_visibility() {
    // At least one row carries non-authoritative provenance, stays cited, and is
    // not presented as an authoritative live executed pass.
    let row = packet()
        .rows
        .into_iter()
        .find(|r| !r.provenance.is_authoritative())
        .expect("a non-authoritative row is present");
    assert!(row.cited);
    assert!(!row.outcome.claims_execution_pass() || !row.chips.freshness.is_authoritative_live());
}

#[test]
fn missing_required_subject_kind_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    // Drop every link row, keeping a broken-link finding via mode coverage check.
    let removed: Vec<String> = input
        .rows
        .iter()
        .filter(|r| r.subject.subject_kind == ValidationSubjectKind::Link)
        .map(|r| r.row_id.clone())
        .collect();
    input
        .rows
        .retain(|r| r.subject.subject_kind != ValidationSubjectKind::Link);
    input
        .export
        .rows
        .retain(|r| !removed.contains(&r.row_id_ref));
    let packet = DocsValidationReportPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ValidationPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::RequiredSubjectKindMissing));
}

#[test]
fn missing_required_mode_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    let removed: Vec<String> = input
        .rows
        .iter()
        .filter(|r| r.mode == ValidationMode::Stale)
        .map(|r| r.row_id.clone())
        .collect();
    input.rows.retain(|r| r.mode != ValidationMode::Stale);
    input
        .export
        .rows
        .retain(|r| !removed.contains(&r.row_id_ref));
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::RequiredModeCoverageMissing));
}

#[test]
fn missing_subject_identity_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].subject.doc_ref = "  ".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::SubjectIdentityMissing));
}

#[test]
fn empty_snippet_anchor_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].subject.snippet_anchor = Some("   ".to_owned());
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::SubjectIdentityMissing));
}

#[test]
fn missing_last_checked_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].last_checked_at = "  ".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::LastCheckedMissing));
}

#[test]
fn missing_environment_scope_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].scope.toolchain_ref = "  ".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::EnvironmentScopeMissing));
}

#[test]
fn missing_producer_context_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].produced_by.execution_context_ref = "  ".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ProducerContextMissing));
}

#[test]
fn producer_validator_mode_mismatch_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    // An executed-local row claimed to be produced by the link checker.
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::ExecutedLocal)
        .expect("executed-local row present");
    row.produced_by.validator = ValidatorKind::LinkChecker;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.produced_by = ValidatorKind::LinkChecker;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ProducerValidatorModeMismatch));
}

#[test]
fn producer_not_preserved_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].actions.preserves_producer = false;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ProducerNotPreserved));
}

#[test]
fn rendered_row_claiming_execution_collapses_the_distinction() {
    let mut input = seeded_stable_docs_validation_report_input();
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::Rendered)
        .expect("rendered row present");
    row.outcome = ValidationOutcome::ExecutedPass;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.outcome = ValidationOutcome::ExecutedPass;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ValidationPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExecutionClaimWithoutRun));
}

#[test]
fn mode_outcome_inconsistency_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    // A syntax-checked row reporting not_run is inconsistent without claiming
    // execution, so it lands as mode_outcome_inconsistent.
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::SyntaxChecked)
        .expect("syntax-checked row present");
    row.outcome = ValidationOutcome::NotRun;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.outcome = ValidationOutcome::NotRun;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ModeOutcomeInconsistent));
}

#[test]
fn missing_provenance_disclosure_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].provenance_disclosure_note = "  ".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ProvenanceDisclosureMissing));
}

#[test]
fn unverified_executed_pass_at_live_freshness_collapses_result_truth() {
    let mut input = seeded_stable_docs_validation_report_input();
    // Make the imported broken-link row claim an authoritative live executed pass.
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.provenance == ValidationEvidenceProvenance::Imported)
        .expect("imported row present");
    row.mode = ValidationMode::ExecutedRemote;
    row.outcome = ValidationOutcome::ExecutedPass;
    row.chips.freshness = ValidationFreshness::AuthoritativeLive;
    row.produced_by.validator = ValidatorKind::RemoteExampleRunner;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.mode = ValidationMode::ExecutedRemote;
            export.outcome = ValidationOutcome::ExecutedPass;
            export.freshness = ValidationFreshness::AuthoritativeLive;
            export.produced_by = ValidatorKind::RemoteExampleRunner;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ResultTruthCollapsed));
}

#[test]
fn uncited_unverified_row_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.provenance.needs_citation())
        .expect("unverified row present");
    row.cited = false;
    row.citation_ref = None;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.cited = false;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::RowNotCited));
}

#[test]
fn drifted_version_presented_as_confident_live_pass_collapses_version_truth() {
    let mut input = seeded_stable_docs_validation_report_input();
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::ExecutedLocal)
        .expect("executed-local row present");
    row.chips.version_match = ValidationVersionMatch::IncompatibleDriftDetected;
    let id = row.row_id.clone();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == id {
            export.version_match = ValidationVersionMatch::IncompatibleDriftDetected;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::VersionTruthCollapsed));
}

#[test]
fn untraced_finding_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.mode == ValidationMode::BrokenLink)
        .expect("broken-link row present");
    row.source_trace_ref = "  ".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ValidationPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::FindingNotTraced));
}

#[test]
fn incomplete_action_parity_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].actions.rerun_available = false;
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == input.rows[0].row_id {
            export.action_parity_complete = false;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ActionParityIncomplete));
}

#[test]
fn missing_compare_action_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].actions.compare_current_source_ref = "  ".to_owned();
    for export in input.export.rows.iter_mut() {
        if export.row_id_ref == input.rows[0].row_id {
            export.action_parity_complete = false;
        }
    }
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ActionParityIncomplete));
}

#[test]
fn suppressed_row_without_attribution_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.suppression.state == ValidationSuppressionState::Suppressed)
        .expect("suppressed row present");
    row.suppression.attributed_to_ref = None;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::SuppressionNotAttributable));
}

#[test]
fn suppressed_row_not_reopenable_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    let row = input
        .rows
        .iter_mut()
        .find(|r| r.suppression.state == ValidationSuppressionState::Suppressed)
        .expect("suppressed row present");
    row.suppression.reopenable = false;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::SuppressionNotReopenable));
}

#[test]
fn export_dropping_scope_preservation_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.preserves_scope = false;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportDropsPreservation));
}

#[test]
fn export_mode_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].mode = ValidationMode::Skipped;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportModeMismatch));
}

#[test]
fn export_outcome_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].outcome = ValidationOutcome::NotRun;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportOutcomeMismatch));
}

#[test]
fn export_last_checked_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].last_checked_at = "2020-01-01T00:00:00Z".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportLastCheckedMismatch));
}

#[test]
fn export_scope_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].environment_label = "some other environment".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportScopeMismatch));
}

#[test]
fn export_freshness_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].freshness = ValidationFreshness::Stale;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportFreshnessMismatch));
}

#[test]
fn export_provenance_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].provenance = ValidationEvidenceProvenance::DerivedHeuristic;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportProvenanceMismatch));
}

#[test]
fn export_producer_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].produced_by = ValidatorKind::ManualReviewer;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportProducerMismatch));
}

#[test]
fn export_suppression_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].suppression_state = ValidationSuppressionState::Suppressed;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportSuppressionMismatch));
}

#[test]
fn export_cited_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].cited = false;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportCitedMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows.pop();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.export.rows[0].row_id_ref = "row:does-not-exist".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.report_degradations.push(ValidationDegradation {
        degradation_class: ValidationDegradationClass::ReportNarrowed,
        severity: ValidationFindingSeverity::Narrowing,
        summary: "the report was narrowed to the qualified release docs after a scope change"
            .to_owned(),
        row_id_ref: None,
        evidence_ref: None,
    });
    let packet = DocsValidationReportPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ValidationPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.report_degradations.push(ValidationDegradation {
        degradation_class: ValidationDegradationClass::QuarantinedSource,
        severity: ValidationFindingSeverity::Blocking,
        summary: "a docs source is quarantined and must not present as validated".to_owned(),
        row_id_ref: Some("row:readme:config_example_executed_local".to_owned()),
        evidence_ref: None,
    });
    let packet = DocsValidationReportPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ValidationPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_row_is_orphan() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.report_degradations[0].row_id_ref = Some("row:does-not-exist".to_owned());
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_scope_drifts() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.consumer_projections[0].preserves_scope = false;
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_docs_validation_report_input();
    input
        .consumer_projections
        .retain(|p| p.surface != ValidationConsumerSurface::ReleaseCenter);
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut input = seeded_stable_docs_validation_report_input();
    let clone = input.rows[0].clone();
    input.rows.push(clone);
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::DuplicateRowId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_docs_validation_report_input();
    input.rows[0].detail = "matched on bearer abc123 token in the source".to_owned();
    let packet = DocsValidationReportPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ValidationFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_rows_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(summary.contains(&row.row_id));
    }
    assert!(summary.contains("Mode/outcome"));
    assert!(summary.contains("Scope"));
    assert!(summary.contains("Produced by"));
    assert!(summary.contains("Actions"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-12T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsValidationReportSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        DOCS_VALIDATION_REPORT_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_docs_validation_report_export()
        .expect("checked docs-validation-report export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:docs_validation_report:retry_backoff_release"
    );
    assert_eq!(
        export.packet.promotion_state,
        ValidationPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/example-link-validation/mirror_offline_narrows.json"
            )),
            ValidationPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/example-link-validation/rendered_claims_execution_blocks_stable.json"
            )),
            ValidationPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/example-link-validation/untraced_broken_link_blocks_stable.json"
            )),
            ValidationPromotionState::BlocksStable,
        ),
    ] {
        let fixture: DocsValidationReportFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = DocsValidationReportPacket::materialize(fixture.input);
        assert_eq!(
            packet.promotion_state, expected,
            "fixture `{}` expected {:?}, findings: {:?}",
            fixture.case_name, expected, packet.validation_findings
        );
        for expected_kind in fixture.expect.expected_finding_kinds {
            assert!(
                packet
                    .validation_findings
                    .iter()
                    .any(|f| f.finding_kind.as_str() == expected_kind),
                "fixture `{}` expected finding `{}`",
                fixture.case_name,
                expected_kind
            );
        }
    }
}

#[derive(Debug, Deserialize)]
struct DocsValidationReportFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: DocsValidationReportPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
