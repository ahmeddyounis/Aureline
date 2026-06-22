//! Unit tests for the canonical completion-row model.

use super::*;
use crate::assist::CompletionSideEffectClass;

#[test]
fn model_builds_and_every_invariant_holds() {
    let model = completion_row_model();
    assert_eq!(model.record_kind, M5_COMPLETION_ROWS_RECORD_KIND);
    assert_eq!(model.schema_ref, M5_COMPLETION_ROWS_SCHEMA_REF);
    assert_eq!(model.model_id, M5_COMPLETION_ROWS_MODEL_ID);
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
    let model = completion_row_model();
    let json = serde_json::to_string(&model).expect("model serializes");
    let restored: CompletionRowModel = serde_json::from_str(&json).expect("model round-trips");
    assert_eq!(model, restored);
}

#[test]
fn every_claimed_surface_is_present() {
    let model = completion_row_model();
    for surface in [
        EditorSurfaceClass::CodeFile,
        EditorSurfaceClass::ConfigFile,
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
    ] {
        assert!(
            model.snapshot(surface).is_some(),
            "missing snapshot for {}",
            surface.as_str()
        );
    }
}

#[test]
fn assist_class_derive_splits_local_word_from_cached() {
    assert_eq!(
        CompletionAssistClass::derive(
            AssistSourceLabelClass::CachedFallback,
            CompletionItemKindClass::LocalWord
        ),
        CompletionAssistClass::LocalWord
    );
    assert_eq!(
        CompletionAssistClass::derive(
            AssistSourceLabelClass::CachedFallback,
            CompletionItemKindClass::Function
        ),
        CompletionAssistClass::CachedFallback
    );
    assert_eq!(
        CompletionAssistClass::derive(
            AssistSourceLabelClass::AiInlineAssist,
            CompletionItemKindClass::Function
        ),
        CompletionAssistClass::AiBacked
    );
}

#[test]
fn ai_and_local_word_never_inherit_full_semantic_trust() {
    let model = completion_row_model();
    for row in model.all_rows() {
        if matches!(
            row.assist_class,
            CompletionAssistClass::AiBacked | CompletionAssistClass::LocalWord
        ) {
            assert!(
                !row.trust_weight.is_full_semantic(),
                "{} must not carry full-semantic trust",
                row.row_id
            );
        }
    }
    // Deterministic semantic rows do carry full-semantic weight.
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let det = code.row("code-fn").expect("deterministic row");
    assert_eq!(det.trust_weight, TrustWeightClass::FullSemantic);
    assert!(det.assist_class.is_deterministic());
}

#[test]
fn deterministic_and_ai_rows_are_distinct_on_a_surface() {
    let model = completion_row_model();
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let det = code.row("code-fn").expect("deterministic");
    let ai = code.row("code-ai").expect("ai");
    assert_ne!(det.assist_class, ai.assist_class);
    assert_ne!(det.trust_weight, ai.trust_weight);
    assert!(ai.requires_visual_distinction);
    assert!(ai.non_color_differentiator.contains("AI"));
}

#[test]
fn import_row_discloses_additional_edit_before_commit() {
    let model = completion_row_model();
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let import_row = code.row("code-type-import").expect("import row");
    assert_eq!(
        import_row.additional_edit_cue,
        AdditionalEditCue::AddsImport
    );
    assert!(import_row.commit_disclosure_required);
    assert!(import_row.additional_edit_summary.is_some());
    assert_eq!(
        import_row.side_effect_class(),
        CompletionSideEffectClass::CurrentFileAdditionalEditsNoted
    );
}

#[test]
fn generated_output_row_requires_preview() {
    let model = completion_row_model();
    let generated = model
        .snapshot(EditorSurfaceClass::GeneratedFile)
        .expect("generated");
    let row = generated.row("gen-symbol").expect("generated symbol");
    assert_eq!(
        row.additional_edit_cue,
        AdditionalEditCue::GeneratedOutputEffect
    );
    assert!(row.preview_required);
    assert!(row.commit_disclosure_required);
    assert_eq!(
        row.side_effect_class(),
        CompletionSideEffectClass::PreviewRequiredBeforeAdditionalEdits
    );
}

#[test]
fn degraded_postures_carry_visible_fallback_label() {
    let model = completion_row_model();
    for surface in [
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::ProtectedFile,
    ] {
        let snapshot = model.snapshot(surface).expect("snapshot");
        assert!(
            snapshot.provider_posture.is_degraded(),
            "{} should be degraded",
            surface.as_str()
        );
        assert!(snapshot.fallback_label_required);
        assert!(!snapshot.provider_posture_label.trim().is_empty());
        assert!(snapshot.disclosure_required);
    }
    // Full-semantic surfaces do not force a fallback label.
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    assert_eq!(
        code.provider_posture,
        CompletionProviderPosture::FullSemantic
    );
    assert!(!code.fallback_label_required);
}

#[test]
fn unavailable_rows_are_inspect_only_and_marked() {
    let model = completion_row_model();
    let large = model
        .snapshot(EditorSurfaceClass::LargeFileRestricted)
        .expect("large file");
    let row = large
        .row("lf-semantic-unavailable")
        .expect("unavailable row");
    assert_eq!(row.availability, CompletionAvailabilityClass::Unavailable);
    assert_eq!(
        row.side_effect_class(),
        CompletionSideEffectClass::InspectOnlyNoApply
    );
    assert!(!row.commit_disclosure_required);
    assert!(row.requires_visual_distinction);
    assert!(row.non_color_differentiator.contains("unavailable"));
}

#[test]
fn deprecated_row_is_marked_but_acceptable() {
    let model = completion_row_model();
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let row = code.row("code-deprecated").expect("deprecated row");
    assert_eq!(row.availability, CompletionAvailabilityClass::Deprecated);
    assert!(row.availability.is_acceptable());
    assert!(row.non_color_differentiator.contains("strikethrough"));
}

#[test]
fn canonical_snapshot_mirrors_rows() {
    let model = completion_row_model();
    for snapshot in &model.surface_snapshots {
        assert_eq!(snapshot.canonical_snapshot.items.len(), snapshot.rows.len());
        for row in &snapshot.rows {
            let item = snapshot
                .canonical_snapshot
                .items
                .iter()
                .find(|item| item.completion_item_id == row.row_id)
                .expect("canonical item for row");
            assert_eq!(item.source, row.source);
            assert_eq!(item.label, row.primary_label);
            assert_eq!(item.kind_class, row.kind_class);
        }
    }
}

#[test]
fn every_row_carries_a_source_label_and_icon() {
    let model = completion_row_model();
    for row in model.all_rows() {
        assert!(!row.source.source_label.trim().is_empty());
        assert!(row.kind_icon_token.starts_with("icon.completion."));
        assert!(!row.accessibility_label.trim().is_empty());
    }
}

#[test]
fn class_catalogs_have_unique_tokens() {
    let model = completion_row_model();
    for catalog in [
        &model.assist_classes,
        &model.trust_weights,
        &model.additional_edit_cues,
        &model.availability_classes,
        &model.provider_postures,
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
    assert_eq!(model.assist_classes.len(), CompletionAssistClass::ALL.len());
}

#[test]
fn lines_projection_renders_every_section() {
    let model = completion_row_model();
    let lines = completion_row_model_lines(&model);
    assert!(lines
        .iter()
        .any(|line| line.contains("Completion-row model")));
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
