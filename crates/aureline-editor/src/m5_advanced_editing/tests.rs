//! Unit tests for the canonical advanced-editing micro-surface model.

use super::*;

#[test]
fn model_builds_and_every_invariant_holds() {
    let model = advanced_editing_model();
    assert_eq!(model.record_kind, M5_ADVANCED_EDITING_RECORD_KIND);
    assert_eq!(model.schema_ref, M5_ADVANCED_EDITING_SCHEMA_REF);
    assert_eq!(model.model_id, M5_ADVANCED_EDITING_MODEL_ID);
    assert!(
        model.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        model
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(model.is_support_export_safe());
    assert!(model.raw_payload_excluded);
}

#[test]
fn model_serialization_round_trips() {
    let model = advanced_editing_model();
    let json = serde_json::to_string(&model).expect("model serializes");
    let restored: AdvancedEditingModel = serde_json::from_str(&json).expect("model round-trips");
    assert_eq!(model, restored);
}

#[test]
fn every_claimed_surface_is_present_once() {
    let model = advanced_editing_model();
    for surface in EditorSurfaceClass::ALL {
        assert!(
            model.snapshot(surface).is_some(),
            "missing snapshot for {}",
            surface.as_str()
        );
    }
    assert_eq!(model.surface_snapshots.len(), EditorSurfaceClass::ALL.len());
}

#[test]
fn selection_semantics_are_disclosed_for_all_four_classes() {
    let model = advanced_editing_model();
    // Every one of the four semantics classes must be exercised somewhere in the
    // corpus so the model proves the full vocabulary, not just the exact case.
    let mut seen: Vec<SelectionSemanticsClass> = model
        .all_selection_strips()
        .map(|strip| strip.semantics_class)
        .collect();
    seen.sort_unstable();
    seen.dedup();
    for class in SelectionSemanticsClass::ALL {
        assert!(
            seen.contains(&class),
            "selection semantics class {} not exercised",
            class.as_str()
        );
    }
    for strip in model.all_selection_strips() {
        assert!(strip.semantics_disclosed_when_inexact());
        if strip.semantics_class.requires_disclosure() {
            assert!(strip.semantics_disclosed);
        }
    }
}

#[test]
fn multi_caret_strips_show_count_primary_and_undo_group() {
    let model = advanced_editing_model();
    for strip in model.all_selection_strips() {
        assert!(strip.count_and_primary_visible());
        if strip.mode_class.is_multi() {
            assert!(strip.caret_count > 1, "{} carets", strip.caret_count);
            assert!(!strip.primary_caret_label.trim().is_empty());
            assert!(!strip.undo_grouping_class.trim().is_empty());
        }
    }
}

#[test]
fn narrowed_and_blocked_selections_explain_unsupported_ops() {
    let model = advanced_editing_model();
    for strip in model.all_selection_strips() {
        assert!(strip.unsupported_operations_explained());
        if strip.semantics_class.narrows_application() {
            assert!(
                !strip.unsupported_operations.is_empty(),
                "{:?} must explain unsupported ops",
                strip.semantics_class
            );
            for note in &strip.unsupported_operations {
                assert!(!note.operation_label.trim().is_empty());
                assert!(!note.reason.trim().is_empty());
                assert!(!note.fallback_route_ref.trim().is_empty());
            }
        }
    }
}

#[test]
fn column_edit_preview_present_for_column_block_mode() {
    let model = advanced_editing_model();
    let config = model
        .snapshot(EditorSurfaceClass::ConfigFile)
        .expect("config");
    assert_eq!(
        config.selection_strip.mode_class,
        SelectionModeClass::ColumnBlock
    );
    assert!(config.selection_strip.column_edit_preview.is_some());

    let sql = model.snapshot(EditorSurfaceClass::SqlEditor).expect("sql");
    assert_eq!(
        sql.selection_strip.mode_class,
        SelectionModeClass::ColumnBlock
    );
    assert!(sql.selection_strip.column_edit_preview.is_some());
}

#[test]
fn folds_with_hidden_critical_state_never_appear_clean() {
    let model = advanced_editing_model();
    let mut saw_critical = false;
    for fold in model.all_fold_summaries() {
        assert!(fold.risk_class_matches_counts());
        assert!(fold.advertises_critical_state());
        assert!(fold.keyboard_and_label_present());
        if fold.hidden_state_counts.has_critical_state() {
            saw_critical = true;
            assert_eq!(fold.risk_class, FoldRiskClass::HiddenCritical);
            assert!(fold.advertises_hidden_state);
            assert!(fold
                .reveal_detail_route_ref
                .as_ref()
                .is_some_and(|route| !route.trim().is_empty()));
        }
    }
    assert!(
        saw_critical,
        "corpus must include a fold hiding critical state"
    );
}

#[test]
fn fold_reveal_route_matches_the_hidden_kind() {
    let model = advanced_editing_model();
    // The protected-file fold hides a conflict and a trust warning; conflicts route
    // to the review surface.
    let protected = model
        .snapshot(EditorSurfaceClass::ProtectedFile)
        .expect("protected");
    let fold = protected.fold_summaries.first().expect("protected fold");
    assert_eq!(fold.risk_class, FoldRiskClass::HiddenCritical);
    assert_eq!(
        fold.reveal_detail_route_ref.as_deref(),
        Some("route:review.reveal_conflict_in_fold")
    );
    // The generated-file fold hides trust warnings only; it routes to the trust path.
    let generated = model
        .snapshot(EditorSurfaceClass::GeneratedFile)
        .expect("generated");
    let gfold = generated.fold_summaries.first().expect("generated fold");
    assert_eq!(
        gfold.reveal_detail_route_ref.as_deref(),
        Some("route:trust.reveal_fold_policy_state")
    );
}

#[test]
fn overview_aids_are_optional_aligned_and_never_sole_carrier() {
    let model = advanced_editing_model();
    for aid in model.all_overview_aids() {
        assert!(aid.is_optional_accelerator);
        assert!(!aid.is_sole_carrier_of_critical_state);
        assert!(aid.not_sole_carrier());
        assert!(aid.aligned_with_main());
        assert!(aid.degrades_honestly());
        assert!(!aid.non_color_differentiator.trim().is_empty());
    }
}

#[test]
fn degraded_aids_carry_message_and_replacement_routes() {
    let model = advanced_editing_model();
    let mut saw_disabled = false;
    for aid in model.all_overview_aids() {
        if !matches!(aid.availability, OrientationAidAvailability::Available) {
            saw_disabled = true;
            assert!(aid
                .degraded_state_message
                .as_ref()
                .is_some_and(|m| !m.trim().is_empty()));
            assert!(!aid.replacement_route_refs.is_empty());
        }
    }
    assert!(saw_disabled, "corpus must include a degraded overview aid");
}

#[test]
fn large_file_suppresses_multicursor_and_folds_and_disables_aids() {
    let model = advanced_editing_model();
    let large = model
        .snapshot(EditorSurfaceClass::LargeFileRestricted)
        .expect("large");
    assert_eq!(
        large.selection_strip.mode_class,
        SelectionModeClass::SingleCaret
    );
    assert_eq!(
        large.selection_strip.semantics_class,
        SelectionSemanticsClass::PrimaryCaretOnly
    );
    assert!(!large.selection_strip.unsupported_operations.is_empty());
    assert!(large.fold_summaries.is_empty());
    assert!(large.overview_aids.iter().all(|aid| matches!(
        aid.availability,
        OrientationAidAvailability::DisabledLargeFile
    )));
    assert_eq!(large.degrade_class, AssistDegradeClass::SuppressedLargeFile);
    assert!(large.disclosure_required);
}

#[test]
fn notebook_and_request_reuse_shared_record_kinds() {
    let model = advanced_editing_model();
    for surface in [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::RequestEditor,
    ] {
        let snapshot = model.snapshot(surface).expect("snapshot");
        assert_eq!(
            snapshot.selection_strip.record_kind,
            SelectionSummaryStrip::RECORD_KIND
        );
        for fold in &snapshot.fold_summaries {
            assert_eq!(fold.record_kind, FoldRiskSummary::RECORD_KIND);
        }
        for aid in &snapshot.overview_aids {
            assert_eq!(aid.record_kind, OverviewAidParity::RECORD_KIND);
        }
    }
}

#[test]
fn render_awareness_covers_density_zoom_motion_and_preserves_state() {
    let model = advanced_editing_model();
    assert_eq!(model.render_awareness.len(), 3 + 2 + 2);
    for dimension in ["density", "zoom", "motion"] {
        assert!(
            model
                .render_awareness
                .iter()
                .any(|policy| policy.dimension == dimension),
            "render awareness must cover {dimension}"
        );
    }
    for policy in &model.render_awareness {
        assert!(policy.critical_state_preserved);
        assert!(policy.non_color_only_preserved);
        assert!(!policy.compaction_note.trim().is_empty());
    }
}

#[test]
fn degraded_surfaces_label_and_disclose() {
    let model = advanced_editing_model();
    for snapshot in &model.surface_snapshots {
        if snapshot.degrade_class != AssistDegradeClass::FullFidelity {
            assert!(!snapshot.degrade_label.trim().is_empty());
            assert!(snapshot.disclosure_required);
        }
    }
}

#[test]
fn class_catalogs_have_unique_tokens() {
    let model = advanced_editing_model();
    for catalog in [
        &model.selection_mode_classes,
        &model.selection_semantics_classes,
        &model.fold_risk_classes,
        &model.overview_aid_classes,
        &model.aid_availability_classes,
    ] {
        let mut tokens: Vec<&str> = catalog
            .iter()
            .map(|descriptor| descriptor.class_token.as_str())
            .collect();
        let total = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), total, "catalog tokens must be unique");
    }
    assert_eq!(
        model.selection_semantics_classes.len(),
        SelectionSemanticsClass::ALL.len()
    );
    assert_eq!(model.fold_risk_classes.len(), FoldRiskClass::ALL.len());
}

#[test]
fn lines_projection_renders_every_section() {
    let model = advanced_editing_model();
    let lines = advanced_editing_model_lines(&model);
    assert!(lines
        .iter()
        .any(|line| line.contains("Advanced-editing model")));
    assert!(lines.iter().any(|line| line.contains("Surface snapshots:")));
    assert!(lines.iter().any(|line| line.contains("Invariants:")));
    for snapshot in &model.surface_snapshots {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(snapshot.surface_class.as_str())),
            "lines must mention surface {}",
            snapshot.surface_class.as_str()
        );
    }
}
