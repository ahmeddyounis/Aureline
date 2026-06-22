//! Unit tests for the canonical editor-assist matrix.

use super::*;

#[test]
fn matrix_builds_and_every_invariant_holds() {
    let matrix = editor_assist_matrix();
    assert_eq!(matrix.record_kind, M5_EDITOR_ASSIST_RECORD_KIND);
    assert_eq!(matrix.schema_ref, M5_EDITOR_ASSIST_SCHEMA_REF);
    assert_eq!(matrix.matrix_id, M5_EDITOR_ASSIST_MATRIX_ID);
    assert!(
        matrix.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        matrix
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(matrix.is_support_export_safe());
    assert!(matrix.raw_payload_excluded);
}

#[test]
fn matrix_serialization_round_trips() {
    let matrix = editor_assist_matrix();
    let json = serde_json::to_string(&matrix).expect("matrix serializes");
    let restored: EditorAssistMatrix = serde_json::from_str(&json).expect("matrix round-trips");
    assert_eq!(matrix, restored);
}

#[test]
fn editing_truth_outranks_every_convenience_layer() {
    let matrix = editor_assist_matrix();
    let max_truth = matrix
        .precedence_ladder
        .iter()
        .filter(|layer| layer.truth_tier == TruthTier::EditingTruth)
        .map(|layer| layer.rank)
        .max()
        .expect("at least one editing-truth layer");
    let min_convenience = matrix
        .precedence_ladder
        .iter()
        .filter(|layer| layer.truth_tier == TruthTier::ConvenienceMetadata)
        .map(|layer| layer.rank)
        .min()
        .expect("at least one convenience layer");
    assert!(
        max_truth < min_convenience,
        "editing truth (max rank {max_truth}) must outrank convenience (min rank {min_convenience})"
    );
}

#[test]
fn ladder_ranks_are_dense_and_ordered() {
    let matrix = editor_assist_matrix();
    for (index, layer) in matrix.precedence_ladder.iter().enumerate() {
        assert_eq!(layer.rank as usize, index, "rank must equal ladder index");
    }
    assert_eq!(
        matrix.precedence_ladder.len(),
        EditorLayerClass::ALL.len(),
        "every layer appears once"
    );
}

#[test]
fn every_surface_binds_every_channel_once() {
    let matrix = editor_assist_matrix();
    assert_eq!(matrix.surface_profiles.len(), EditorSurfaceClass::ALL.len());
    for profile in &matrix.surface_profiles {
        assert_eq!(
            profile.cells.len(),
            AssistChannelClass::ALL.len(),
            "{} must bind every channel",
            profile.surface.as_str()
        );
        for channel in AssistChannelClass::ALL {
            assert!(
                profile.cell(channel).is_some(),
                "{} is missing channel {}",
                profile.surface.as_str(),
                channel.as_str()
            );
        }
    }
}

#[test]
fn code_file_is_full_fidelity_everywhere() {
    let matrix = editor_assist_matrix();
    let profile = matrix
        .surface_profile(EditorSurfaceClass::CodeFile)
        .expect("code file profile");
    assert!(!profile.is_constrained);
    for cell in &profile.cells {
        assert_eq!(
            cell.degrade_state,
            AssistDegradeClass::FullFidelity,
            "code file channel {} must be full fidelity",
            cell.channel.as_str()
        );
        assert!(cell.keyboard_reachable);
    }
}

#[test]
fn generated_and_protected_block_apply() {
    let matrix = editor_assist_matrix();
    for surface in [
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
    ] {
        let profile = matrix.surface_profile(surface).expect("profile");
        for channel in [
            AssistChannelClass::Completion,
            AssistChannelClass::SnippetSession,
            AssistChannelClass::InlineAiAssist,
        ] {
            let cell = profile.cell(channel).expect("cell");
            assert_eq!(
                cell.degrade_state,
                AssistDegradeClass::ReadOnlyNoApply,
                "{} channel {} must block apply",
                surface.as_str(),
                channel.as_str()
            );
        }
        // Reading channels stay full on generated / protected surfaces.
        assert_eq!(
            profile
                .cell(AssistChannelClass::Hover)
                .expect("hover")
                .degrade_state,
            AssistDegradeClass::FullFidelity
        );
    }
}

#[test]
fn large_file_suppresses_convenience_and_reduces_decorations() {
    let matrix = editor_assist_matrix();
    let profile = matrix
        .surface_profile(EditorSurfaceClass::LargeFileRestricted)
        .expect("large-file profile");
    for cell in &profile.cells {
        if cell.channel == AssistChannelClass::Decoration {
            assert_eq!(
                cell.degrade_state,
                AssistDegradeClass::SourceLabeledFallback,
                "decorations are reduced, not suppressed"
            );
        } else {
            assert_eq!(
                cell.degrade_state,
                AssistDegradeClass::SuppressedLargeFile,
                "channel {} must be suppressed in large-file mode",
                cell.channel.as_str()
            );
        }
    }
}

#[test]
fn partial_index_pends_semantic_channels() {
    let matrix = editor_assist_matrix();
    let profile = matrix
        .surface_profile(EditorSurfaceClass::PartialIndexState)
        .expect("partial-index profile");
    for cell in &profile.cells {
        if cell.channel.is_semantic() {
            assert_eq!(
                cell.degrade_state,
                AssistDegradeClass::PendingPartialIndex,
                "semantic channel {} must pend",
                cell.channel.as_str()
            );
        }
    }
    // Snippets do not need the index.
    assert_eq!(
        profile
            .cell(AssistChannelClass::SnippetSession)
            .expect("snippet")
            .degrade_state,
        AssistDegradeClass::FullFidelity
    );
}

#[test]
fn docs_code_blocks_drop_lenses_inlay_and_peek() {
    let matrix = editor_assist_matrix();
    let profile = matrix
        .surface_profile(EditorSurfaceClass::DocsCodeBlock)
        .expect("docs-code profile");
    for channel in [
        AssistChannelClass::CodeLens,
        AssistChannelClass::InlayHint,
        AssistChannelClass::Peek,
    ] {
        let cell = profile.cell(channel).expect("cell");
        assert_eq!(cell.degrade_state, AssistDegradeClass::BlockedUnavailable);
        assert!(
            !cell.keyboard_reachable,
            "blocked channel {} is not keyboard reachable",
            channel.as_str()
        );
    }
}

#[test]
fn blocked_cells_are_the_only_unreachable_ones() {
    let matrix = editor_assist_matrix();
    for profile in &matrix.surface_profiles {
        for cell in &profile.cells {
            assert_eq!(
                cell.keyboard_reachable,
                cell.degrade_state.is_offered(),
                "{}::{} reachability must match offered state",
                profile.surface.as_str(),
                cell.channel.as_str()
            );
        }
    }
}

#[test]
fn decorations_map_only_to_editing_truth_layers() {
    for class in DecorationClass::ALL {
        assert_eq!(
            class.owning_layer().truth_tier(),
            TruthTier::EditingTruth,
            "decoration {} must be editing truth",
            class.as_str()
        );
    }
}

#[test]
fn completion_source_catalog_reuses_shared_labels() {
    let matrix = editor_assist_matrix();
    let tokens: Vec<&str> = matrix
        .completion_source_kinds
        .iter()
        .map(|descriptor| descriptor.class_token.as_str())
        .collect();
    // The AI inline assist source must be present and flagged for distinction.
    assert!(tokens.contains(&"ai_inline_assist"));
    assert!(tokens.contains(&"deterministic_language"));
    assert!(tokens.contains(&"cached_fallback"));
    assert!(tokens.contains(&"snippet_origin"));
}

#[test]
fn ai_classes_require_explicit_labels() {
    assert!(InlayHintClass::AiInferred.requires_ai_label());
    assert!(CodeLensClass::AiExplainAction.requires_ai_label());
    assert!(!InlayHintClass::ParameterName.requires_ai_label());
}

#[test]
fn identity_contracts_and_export_minimums_are_complete() {
    let matrix = editor_assist_matrix();
    for kind in MicroSurfaceKind::ALL {
        let contract = matrix
            .identity_contracts
            .iter()
            .find(|contract| contract.kind == kind)
            .unwrap_or_else(|| panic!("missing identity contract for {}", kind.as_str()));
        assert_eq!(contract.id_prefix, kind.id_prefix());
        assert!(!contract.required_lifecycle_fields.is_empty());

        let export = matrix
            .support_export_minimums
            .iter()
            .find(|minimum| minimum.record_kind == kind.export_record_kind())
            .unwrap_or_else(|| panic!("missing export minimum for {}", kind.as_str()));
        assert!(export.raw_payload_excluded);
        assert!(!export.required_fields.is_empty());
    }
    // The matrix itself carries an export minimum.
    assert!(matrix
        .support_export_minimums
        .iter()
        .any(|minimum| minimum.record_kind == M5_EDITOR_ASSIST_RECORD_KIND));
}

#[test]
fn lines_projection_renders_every_section() {
    let matrix = editor_assist_matrix();
    let lines = editor_assist_matrix_lines(&matrix);
    assert!(lines
        .iter()
        .any(|line| line.contains("Editor-assist matrix")));
    assert!(lines.iter().any(|line| line.contains("Precedence ladder")));
    assert!(lines.iter().any(|line| line.contains("Surface matrix:")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Identity contracts:")));
    assert!(lines.iter().any(|line| line.contains("Invariants:")));
    // Every surface token appears.
    for surface in EditorSurfaceClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(surface.as_str())),
            "lines must mention surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn class_tokens_are_unique_within_each_catalog() {
    let matrix = editor_assist_matrix();
    for catalog in [
        &matrix.decoration_classes,
        &matrix.code_lens_classes,
        &matrix.inlay_hint_classes,
        &matrix.completion_source_kinds,
        &matrix.signature_help_states,
        &matrix.snippet_session_states,
        &matrix.hover_peek_modes,
        &matrix.degrade_states,
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
}
