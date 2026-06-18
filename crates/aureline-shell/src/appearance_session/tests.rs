//! Unit tests for the appearance-session runtime audit.

use super::*;

fn checkpoint_refs(report: &AppearanceSessionRuntimeReport) -> Vec<String> {
    report
        .checkpoints
        .iter()
        .map(|checkpoint| checkpoint.checkpoint_ref.clone())
        .collect()
}

#[test]
fn seeded_runtime_is_clean_and_validates() {
    let report = seeded_appearance_session_runtime();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    assert!(report.blocking_findings.is_empty());
    validate_appearance_session_runtime(&report).expect("seeded runtime must validate");
}

#[test]
fn seeded_runtime_envelope_is_stable() {
    let report = seeded_appearance_session_runtime();
    assert_eq!(report.record_kind, APPEARANCE_SESSION_REPORT_RECORD_KIND);
    assert_eq!(report.schema_version, APPEARANCE_SESSION_SCHEMA_VERSION);
    assert_eq!(
        report.shared_contract_ref,
        APPEARANCE_SESSION_SHARED_CONTRACT_REF
    );
    assert_eq!(report.report_id, APPEARANCE_SESSION_REPORT_ID);
    assert_eq!(
        report.source_schema_ref,
        APPEARANCE_SESSION_SOURCE_SCHEMA_REF
    );
    assert_eq!(
        report.canonical_record_schema_ref,
        APPEARANCE_SESSION_CANONICAL_RECORD_SCHEMA_REF
    );
    assert_eq!(
        report.published_report_ref,
        APPEARANCE_SESSION_PUBLISHED_REPORT_REF
    );
    assert_eq!(
        report.published_doc_ref,
        APPEARANCE_SESSION_PUBLISHED_DOC_REF
    );
}

#[test]
fn seeded_runtime_is_deterministic() {
    assert_eq!(
        seeded_appearance_session_runtime(),
        seeded_appearance_session_runtime()
    );
}

#[test]
fn seeded_runtime_covers_every_surface_family() {
    let report = seeded_appearance_session_runtime();
    let families: std::collections::BTreeSet<_> =
        report.surfaces.iter().map(|s| s.surface_family).collect();
    for family in [
        SurfaceFamily::Notebook,
        SurfaceFamily::DataResultSurface,
        SurfaceFamily::PreviewBrowserPane,
        SurfaceFamily::DocsHelpPane,
        SurfaceFamily::CompanionSurface,
        SurfaceFamily::ExtensionHostedSurface,
    ] {
        assert!(
            families.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
}

#[test]
fn seeded_runtime_exercises_every_state_machine_op() {
    let report = seeded_appearance_session_runtime();
    let ops: std::collections::BTreeSet<_> = report.transitions.iter().map(|t| t.op).collect();
    for op in [
        TransitionOp::OpenPreview,
        TransitionOp::PreflightPassed,
        TransitionOp::CommitPreview,
        TransitionOp::CancelPreview,
        TransitionOp::ValidationFailed,
        TransitionOp::RevertCommitted,
        TransitionOp::OsSignalApplied,
    ] {
        assert!(
            ops.contains(&op),
            "missing state-machine op {}",
            op.as_str()
        );
    }
}

#[test]
fn collections_are_sorted() {
    let report = seeded_appearance_session_runtime();
    let checkpoint_ids: Vec<_> = report
        .checkpoints
        .iter()
        .map(|c| c.checkpoint_ref.clone())
        .collect();
    let mut sorted = checkpoint_ids.clone();
    sorted.sort();
    assert_eq!(checkpoint_ids, sorted);

    let seq: Vec<_> = report
        .transitions
        .iter()
        .map(|t| t.sequence_index)
        .collect();
    let mut sorted_seq = seq.clone();
    sorted_seq.sort();
    assert_eq!(seq, sorted_seq);

    let surface_ids: Vec<_> = report
        .surfaces
        .iter()
        .map(|s| s.surface_id.clone())
        .collect();
    let mut sorted_surfaces = surface_ids.clone();
    sorted_surfaces.sort();
    assert_eq!(surface_ids, sorted_surfaces);
}

#[test]
fn every_transition_resolves_its_checkpoint() {
    let report = seeded_appearance_session_runtime();
    assert!(report.every_transition_checkpoint_resolved());
}

#[test]
fn live_change_is_demonstrated_and_session_is_live() {
    let report = seeded_appearance_session_runtime();
    assert!(report.live_change_demonstrated);
    assert_eq!(report.session.preview_state, PreviewState::PreviewLive);
    assert_eq!(
        report.session.current_checkpoint_ref.as_deref(),
        Some("appearance-checkpoint:preview-light")
    );
}

#[test]
fn restart_or_reload_surface_count_is_disclosed() {
    let report = seeded_appearance_session_runtime();
    // preview/browser pane, companion sidecar, extension panel.
    assert_eq!(report.restart_or_reload_surface_count, 3);
    for surface in &report.surfaces {
        if !surface.live_apply_capability.applies_live() {
            assert!(
                surface.restart_or_reload_disclosed,
                "{} hides its restart/reload requirement",
                surface.surface_id
            );
        }
    }
}

#[test]
fn transition_with_missing_checkpoint_is_blocking() {
    let findings = compute_transition_findings(
        &seed_transition(
            "transition:test.no-checkpoint",
            1,
            TransitionOp::OpenPreview,
            TransitionTrigger::UserAction,
            "appearance-checkpoint:does-not-exist",
            PreviewState::NotPreviewing,
            vec![AppearanceAxis::ThemePackage],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "test",
        ),
        &["appearance-checkpoint:preview-light".to_owned()],
    );
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "transition_unknown_checkpoint"));
}

#[test]
fn illegal_state_edge_is_blocking() {
    // CommitPreview is only legal from preview_live.
    let mut transition = seed_transition(
        "transition:test.illegal",
        1,
        TransitionOp::CommitPreview,
        TransitionTrigger::UserAction,
        "appearance-checkpoint:preview-light",
        PreviewState::NotPreviewing,
        vec![AppearanceAxis::ThemePackage],
        AtomicityClass::SingleCheckpointAtomic,
        false,
        "test",
    );
    transition.from_preview_state = PreviewState::NotPreviewing;
    let findings = compute_transition_findings(
        &transition,
        &["appearance-checkpoint:preview-light".to_owned()],
    );
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "transition_illegal_state"));
}

#[test]
fn validation_failure_not_reverting_is_blocking() {
    let mut transition = seed_transition(
        "transition:test.failed",
        1,
        TransitionOp::ValidationFailed,
        TransitionTrigger::UserAction,
        "appearance-checkpoint:preview-light",
        PreviewState::PreviewPendingValidation,
        vec![AppearanceAxis::ThemePackage],
        AtomicityClass::SingleCheckpointAtomic,
        false,
        "test",
    );
    // Force a half-updated state: a validation failure that stays committed.
    transition.to_preview_state = PreviewState::PreviewCommitted;
    let findings = compute_transition_findings(
        &transition,
        &["appearance-checkpoint:preview-light".to_owned()],
    );
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "validation_failure_not_reverted"));
}

#[test]
fn silent_restart_requirement_is_blocking() {
    // requires_restart_or_reload but atomicity claims a live change.
    let transition = seed_transition(
        "transition:test.silent-restart",
        1,
        TransitionOp::RevertCommitted,
        TransitionTrigger::SyncImport,
        "appearance-checkpoint:import-dusk",
        PreviewState::PreviewCommitted,
        vec![AppearanceAxis::ThemePackage],
        AtomicityClass::SingleCheckpointAtomic,
        true,
        "test",
    );
    let findings = compute_transition_findings(
        &transition,
        &["appearance-checkpoint:import-dusk".to_owned()],
    );
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "transition_restart_reload_undisclosed"));
}

#[test]
fn atomicity_mismatch_without_restart_flag_is_blocking() {
    // No restart-or-reload flag, but the atomicity class needs a reload.
    let transition = seed_transition(
        "transition:test.atomicity",
        1,
        TransitionOp::OsSignalApplied,
        TransitionTrigger::OsSignal,
        "appearance-checkpoint:os-contrast",
        PreviewState::NotPreviewing,
        vec![AppearanceAxis::Contrast],
        AtomicityClass::SurfaceReloadFromSingleCheckpoint,
        false,
        "test",
    );
    let findings = compute_transition_findings(
        &transition,
        &["appearance-checkpoint:os-contrast".to_owned()],
    );
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "transition_atomicity_mismatch"));
}

#[test]
fn surface_off_session_is_blocking() {
    let report = seeded_appearance_session_runtime();
    let binding = build_appearance_surface_binding(
        "surface:test.off-session",
        SurfaceFamily::Notebook,
        "rev:test.off-session:1",
        "anchor:test.off-session",
        "note",
        "appearance-session:primary",
        false,
        LiveApplyCapability::AppliesLive,
        false,
        None,
        true,
        "appearance-session:primary",
        &checkpoint_refs(&report),
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "surface_not_on_session"));
}

#[test]
fn surface_session_ref_mismatch_is_blocking() {
    let report = seeded_appearance_session_runtime();
    let binding = build_appearance_surface_binding(
        "surface:test.mismatch",
        SurfaceFamily::Notebook,
        "rev:test.mismatch:1",
        "anchor:test.mismatch",
        "note",
        "appearance-session:other",
        true,
        LiveApplyCapability::AppliesLive,
        false,
        None,
        true,
        "appearance-session:primary",
        &checkpoint_refs(&report),
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "surface_session_ref_mismatch"));
}

#[test]
fn surface_undisclosed_reload_is_blocking() {
    let report = seeded_appearance_session_runtime();
    let binding = build_appearance_surface_binding(
        "surface:test.undisclosed",
        SurfaceFamily::PreviewBrowserPane,
        "rev:test.undisclosed:1",
        "anchor:test.undisclosed",
        "note",
        "appearance-session:primary",
        true,
        LiveApplyCapability::RequiresSurfaceReload,
        false,
        None,
        true,
        "appearance-session:primary",
        &checkpoint_refs(&report),
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "surface_restart_reload_undisclosed"));
}

#[test]
fn surface_unknown_last_checkpoint_is_blocking() {
    let report = seeded_appearance_session_runtime();
    let binding = build_appearance_surface_binding(
        "surface:test.unknown-checkpoint",
        SurfaceFamily::Notebook,
        "rev:test.unknown-checkpoint:1",
        "anchor:test.unknown-checkpoint",
        "note",
        "appearance-session:primary",
        true,
        LiveApplyCapability::AppliesLive,
        false,
        Some("appearance-checkpoint:does-not-exist".to_owned()),
        true,
        "appearance-session:primary",
        &checkpoint_refs(&report),
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "surface_unknown_checkpoint"));
}

#[test]
fn session_preview_without_checkpoint_is_blocking() {
    let mut session = seeded_appearance_session_runtime().session;
    session.preview_state = PreviewState::PreviewLive;
    session.current_checkpoint_ref = None;
    let findings = compute_session_findings(&session, &[]);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "session_preview_without_checkpoint"));
}

#[test]
fn session_rollback_without_ref_is_blocking() {
    let mut session = seeded_appearance_session_runtime().session;
    session.preview_state = PreviewState::RollbackApplied;
    session.rollback_ref = None;
    session.current_checkpoint_ref = None;
    let findings = compute_session_findings(&session, &[]);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "session_rollback_without_ref"));
}

#[test]
fn checkpoint_missing_rollback_path_is_blocking() {
    let mut checkpoint = seeded_appearance_session_runtime().checkpoints[0].clone();
    checkpoint.rollback_path.rollback_ref = String::new();
    let findings = compute_checkpoint_findings(&checkpoint);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "checkpoint_missing_rollback_path"));
}

#[test]
fn checkpoint_reload_with_live_rollback_path_is_blocking() {
    let mut checkpoint = seeded_appearance_session_runtime()
        .checkpoints
        .into_iter()
        .find(|c| c.checkpoint_ref == "appearance-checkpoint:import-dusk")
        .expect("import checkpoint");
    // A surface-reload change with a live single-checkpoint rollback path
    // hides the requirement.
    checkpoint.rollback_path.rollback_path_class = RollbackPathClass::SingleCheckpointRevert;
    let findings = compute_checkpoint_findings(&checkpoint);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "checkpoint_restart_reload_undisclosed"));
}

#[test]
fn support_export_quotes_session_checkpoints_transitions_and_surfaces() {
    let report = seeded_appearance_session_runtime();
    let export = AppearanceSessionSupportExport::from_report(
        APPEARANCE_SESSION_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert_eq!(
        export.record_kind,
        APPEARANCE_SESSION_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.session.session_ref));
    for checkpoint in &report.checkpoints {
        assert!(export.case_ids.contains(&checkpoint.checkpoint_ref));
    }
    for transition in &report.transitions {
        assert!(export.case_ids.contains(&transition.transition_ref));
    }
    for surface in &report.surfaces {
        assert!(export.case_ids.contains(&surface.surface_id));
        assert!(export.case_ids.contains(&surface.descriptor_revision_ref));
    }
}

#[test]
fn markdown_and_compact_are_deterministic() {
    let report = seeded_appearance_session_runtime();
    assert_eq!(report.render_markdown(), report.render_markdown());
    assert_eq!(report.compact_lines(), report.compact_lines());
    assert!(report
        .render_markdown()
        .contains("M5 appearance-session runtime audit"));
}

#[test]
fn json_round_trips() {
    let report = seeded_appearance_session_runtime();
    let json = serde_json::to_string(&report).expect("serialize");
    let back: AppearanceSessionRuntimeReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(report, back);
}
