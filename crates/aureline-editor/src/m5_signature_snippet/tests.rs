//! Unit tests for the canonical signature-help and snippet-session model.

use super::*;

#[test]
fn model_builds_and_every_invariant_holds() {
    let model = signature_snippet_model();
    assert_eq!(model.record_kind, M5_SIGNATURE_SNIPPET_RECORD_KIND);
    assert_eq!(model.schema_ref, M5_SIGNATURE_SNIPPET_SCHEMA_REF);
    assert_eq!(model.model_id, M5_SIGNATURE_SNIPPET_MODEL_ID);
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
    let model = signature_snippet_model();
    let json = serde_json::to_string(&model).expect("model serializes");
    let restored: SignatureSnippetModel = serde_json::from_str(&json).expect("model round-trips");
    assert_eq!(model, restored);
}

#[test]
fn every_claimed_surface_is_present() {
    let model = signature_snippet_model();
    for surface in EditorSurfaceClass::ALL {
        assert!(
            model.snapshot(surface).is_some(),
            "missing snapshot for {}",
            surface.as_str()
        );
    }
}

#[test]
fn overloaded_card_exposes_active_overload_and_parameter() {
    let model = signature_snippet_model();
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let card = code.signature_card.as_ref().expect("code signature card");
    assert_eq!(card.state_class, SignatureHelpStateClass::VisibleOverloaded);
    assert!(card.exposes_active_overload());
    assert!(card.exposes_active_parameter());
    assert!(card.signature_count >= 2);
    assert!(
        card.active_signature_index >= 1 && card.active_signature_index <= card.signature_count
    );
}

#[test]
fn signature_card_mirrors_its_canonical_record() {
    let model = signature_snippet_model();
    for card in model.all_cards() {
        let record = &card.canonical_record;
        assert_eq!(record.active_parameter_index, card.active_parameter_index);
        assert_eq!(record.parameter_count, card.parameter_count);
        assert_eq!(record.active_signature_index, card.active_signature_index);
        assert_eq!(record.signature_count, card.signature_count);
        assert_eq!(record.placement_class, card.placement_class);
        assert_eq!(record.source, card.source);
        assert!(!card.obscures_active_line);
    }
}

#[test]
fn snippet_strip_mirrors_its_canonical_record() {
    let model = signature_snippet_model();
    for strip in model.all_strips() {
        let record = &strip.canonical_record;
        assert_eq!(record.state_class, strip.state_class);
        assert_eq!(
            record.active_placeholder_index,
            strip.active_placeholder_index
        );
        assert_eq!(record.placeholder_count, strip.placeholder_count);
        assert_eq!(record.ime_posture_class, strip.ime_posture_class);
        assert_eq!(record.cursor_posture_class, strip.cursor_posture_class);
        assert_eq!(record.source, strip.source);
    }
}

#[test]
fn active_tab_capturing_strip_discloses_capture() {
    let model = signature_snippet_model();
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let strip = code.snippet_strip.as_ref().expect("code snippet strip");
    assert!(
        strip.captures_tab(),
        "code snippet should own Tab while active"
    );
    assert!(strip.visible_strip_required);
    assert!(strip.tab_capture_disclosed);
    assert!(strip.does_not_hijack_tab());
    assert!(strip.exposes_exit_path());
    assert!(strip.exit_path.is_visible());
}

#[test]
fn multi_cursor_ime_degrades_to_one_disclosed_target() {
    let model = signature_snippet_model();
    let notebook = model
        .snapshot(EditorSurfaceClass::NotebookCell)
        .expect("notebook");
    let strip = notebook.snippet_strip.as_ref().expect("notebook strip");
    assert_eq!(
        strip.ime_posture_class,
        SnippetImePostureClass::CompositionPrimaryCaretOnly
    );
    assert!(strip.selection_count > 1);
    assert!(strip.composition_disclosure_required);
    assert!(strip
        .primary_caret_ref
        .as_ref()
        .is_some_and(|caret| !caret.trim().is_empty()));
    assert!(strip.ime_and_multicursor_coherent_or_degraded());
    assert!(strip.accessibility_label.contains("narrowed"));
    // While composing, snippet navigation does not own Tab.
    assert!(!strip.captures_tab());
}

#[test]
fn stale_signature_discloses_limited_cue() {
    let model = signature_snippet_model();
    let docs = model
        .snapshot(EditorSurfaceClass::DocsCodeBlock)
        .expect("docs");
    let card = docs.signature_card.as_ref().expect("docs card");
    assert_eq!(
        card.state_class,
        SignatureHelpStateClass::StalePendingRefresh
    );
    assert!(card.stale_disclosed);
    assert!(!card.non_color_differentiator.trim().is_empty());
    assert!(card.exposes_active_parameter());
    assert_eq!(
        card.blocked_reason,
        Some(AssistBlockReason::StaleAwaitingRefresh)
    );
}

#[test]
fn import_snippet_discloses_side_effect_before_commit() {
    let model = signature_snippet_model();
    let code = model.snapshot(EditorSurfaceClass::CodeFile).expect("code");
    let strip = code.snippet_strip.as_ref().expect("code strip");
    assert_eq!(strip.accept_side_effect, AcceptSideEffectClass::AddsImport);
    assert!(strip.commit_disclosure_required);
    assert!(strip.side_effect_summary.is_some());
    assert!(!strip.preview_required);
}

#[test]
fn generated_and_dependency_effects_require_preview() {
    let model = signature_snippet_model();
    let generated = model
        .snapshot(EditorSurfaceClass::GeneratedFile)
        .expect("generated");
    let gen_strip = generated.snippet_strip.as_ref().expect("generated strip");
    assert_eq!(
        gen_strip.accept_side_effect,
        AcceptSideEffectClass::AddsGeneratedScaffolding
    );
    assert!(gen_strip.preview_required);
    assert!(gen_strip.commit_disclosure_required);

    let config = model
        .snapshot(EditorSurfaceClass::ConfigFile)
        .expect("config");
    let cfg_strip = config.snippet_strip.as_ref().expect("config strip");
    assert_eq!(
        cfg_strip.accept_side_effect,
        AcceptSideEffectClass::AddsDependency
    );
    assert!(cfg_strip.preview_required);
}

#[test]
fn blocked_surfaces_carry_reason_and_disclose() {
    let model = signature_snippet_model();
    for surface in [
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
    ] {
        let snapshot = model.snapshot(surface).expect("snapshot");
        assert!(
            snapshot.disclosure_required,
            "{} must flag disclosure",
            surface.as_str()
        );
        assert_ne!(snapshot.degrade_class, AssistDegradeClass::FullFidelity);
        assert!(!snapshot.degrade_label.trim().is_empty());
    }
}

#[test]
fn large_file_suppresses_signature_and_snippet() {
    let model = signature_snippet_model();
    let large = model
        .snapshot(EditorSurfaceClass::LargeFileRestricted)
        .expect("large");
    let card = large.signature_card.as_ref().expect("large card");
    assert_eq!(card.state_class, SignatureHelpStateClass::Unavailable);
    assert!(!card.is_visible());
    assert_eq!(
        card.blocked_reason,
        Some(AssistBlockReason::LargeFileSuppressed)
    );
    assert!(large.snippet_strip.is_none());
    // Even an unavailable card stays keyboard reachable and screen-reader labeled.
    assert!(card.keyboard_reachable);
    assert!(!card.accessibility_label.trim().is_empty());
}

#[test]
fn class_catalogs_have_unique_tokens() {
    let model = signature_snippet_model();
    for catalog in [
        &model.signature_state_classes,
        &model.snippet_state_classes,
        &model.ime_posture_classes,
        &model.cursor_posture_classes,
        &model.accept_side_effect_classes,
        &model.block_reason_classes,
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
        model.accept_side_effect_classes.len(),
        AcceptSideEffectClass::ALL.len()
    );
    assert_eq!(
        model.block_reason_classes.len(),
        AssistBlockReason::ALL.len()
    );
}

#[test]
fn lines_projection_renders_every_section() {
    let model = signature_snippet_model();
    let lines = signature_snippet_model_lines(&model);
    assert!(lines
        .iter()
        .any(|line| line.contains("Signature-snippet model")));
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
