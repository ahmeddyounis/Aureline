//! Tests for the M5 selected-node primitive: the resolver, the parity matrix, and
//! the checked-in support export.

use super::*;

fn editable_literal_input() -> M5VisualSelectionInput {
    M5VisualSelectionInput {
        selection_id: "selection:test:0001".to_owned(),
        node_label: "TestNode".to_owned(),
        node_kind: M5StructureNodeKind::SourceElement,
        canvas_state: M5CanvasState::SourceBoundEditable,
        support_state: M5VisualSupportState::FullySupported,
        viewport: M5DevicePreviewClass::DesktopViewport,
        source_span_ref: Some("span:test:0001".to_owned()),
        visibility_hidden: false,
        locked: false,
        search_query: Some("test".to_owned()),
        properties: vec![prop(
            "font_size",
            M5PropertyValueState::Literal,
            M5PropertyWriteScope::SingleLiteralSpan,
            ProtectedPathPosture::Unprotected,
            "18px",
        )],
    }
}

// --- resolver: AC1 identity preservation ---

#[test]
fn resolver_preserves_selection_identity_across_surfaces() {
    let input = editable_literal_input();
    let resolved = resolve_visual_selection(&input).expect("resolves");
    assert_eq!(resolved.selection_id, input.selection_id);
    assert_eq!(resolved.canvas_frame.selection_id, input.selection_id);
    assert_eq!(resolved.tree_row.selection_id, input.selection_id);
    assert!(resolved.identity_consistent());
}

// --- resolver: AC2 distinct value states never flattened ---

#[test]
fn resolver_renders_distinct_editor_per_value_state() {
    for value in VALUE_STATE_ALL {
        assert_eq!(
            M5PropertyEditorKind::for_value_state(value),
            match value {
                M5PropertyValueState::Literal => M5PropertyEditorKind::LiteralField,
                M5PropertyValueState::DesignToken => M5PropertyEditorKind::TokenBoundPicker,
                M5PropertyValueState::BoundExpression =>
                    M5PropertyEditorKind::BoundExpressionInspector,
                M5PropertyValueState::Inherited => M5PropertyEditorKind::InheritedValueTrace,
                M5PropertyValueState::Mixed => M5PropertyEditorKind::MixedMultiValue,
                M5PropertyValueState::Unset => M5PropertyEditorKind::UnsetPlaceholder,
            }
        );
    }
    // Distinct value states resolve to distinct editor kinds.
    let kinds: BTreeSet<M5PropertyEditorKind> = VALUE_STATE_ALL
        .iter()
        .map(|v| M5PropertyEditorKind::for_value_state(*v))
        .collect();
    assert_eq!(kinds.len(), VALUE_STATE_ALL.len());
}

#[test]
fn resolver_distinguishes_multiple_value_states_in_one_selection() {
    let input = M5VisualSelectionInput {
        properties: vec![
            prop(
                "font_size",
                M5PropertyValueState::Literal,
                M5PropertyWriteScope::SingleLiteralSpan,
                ProtectedPathPosture::Unprotected,
                "18px",
            ),
            prop(
                "color",
                M5PropertyValueState::DesignToken,
                M5PropertyWriteScope::TokenDefinitionShared,
                ProtectedPathPosture::ProtectedReviewRequired,
                "token.color.brand",
            ),
            prop(
                "margin",
                M5PropertyValueState::Inherited,
                M5PropertyWriteScope::NoWriteInspectOnly,
                ProtectedPathPosture::Unprotected,
                "inherited",
            ),
        ],
        ..editable_literal_input()
    };
    let resolved = resolve_visual_selection(&input).expect("resolves");
    assert!(resolved.value_states_distinguished());
    assert_eq!(resolved.distinct_value_states().len(), 3);
    // The token edit carries the shared-token write scope, requires review.
    let token_row = resolved
        .inspector_rows
        .iter()
        .find(|r| r.value_state == M5PropertyValueState::DesignToken)
        .expect("token row");
    assert_eq!(
        token_row.editor_kind,
        M5PropertyEditorKind::TokenBoundPicker
    );
    assert!(token_row.writable);
    assert!(token_row.requires_review);
}

// --- resolver: AC3 source ownership / support gate ---

#[test]
fn resolver_refuses_write_without_source_ownership() {
    let input = M5VisualSelectionInput {
        canvas_state: M5CanvasState::RuntimeMirrored,
        support_state: M5VisualSupportState::InspectOnly,
        properties: vec![prop(
            "color",
            M5PropertyValueState::Literal,
            M5PropertyWriteScope::SingleLiteralSpan,
            ProtectedPathPosture::Unprotected,
            "red",
        )],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::MutationWithoutSourceOwnership)
    );
}

#[test]
fn resolver_refuses_write_on_blocked_protected_path() {
    let input = M5VisualSelectionInput {
        properties: vec![prop(
            "color",
            M5PropertyValueState::Literal,
            M5PropertyWriteScope::SingleLiteralSpan,
            ProtectedPathPosture::ProtectedBlocked,
            "red",
        )],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::MutationWithoutSourceOwnership)
    );
}

#[test]
fn resolver_inspect_only_support_yields_no_write_path() {
    let input = M5VisualSelectionInput {
        canvas_state: M5CanvasState::RuntimeMirrored,
        support_state: M5VisualSupportState::InspectOnly,
        properties: vec![prop(
            "label",
            M5PropertyValueState::BoundExpression,
            M5PropertyWriteScope::NoWriteInspectOnly,
            ProtectedPathPosture::Unprotected,
            "bound to x",
        )],
        ..editable_literal_input()
    };
    let resolved = resolve_visual_selection(&input).expect("resolves");
    assert!(!resolved.any_writable);
    assert!(!resolved.canvas_frame.editable);
    assert!(resolved.writes_gated_by_disclosure());
    assert!(resolved.source_ownership_disclosed && resolved.support_state_disclosed);
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_span_mismatch_for_unmapped_node() {
    let input = M5VisualSelectionInput {
        node_kind: M5StructureNodeKind::UnmappedNode,
        source_span_ref: Some("span:should-not-exist".to_owned()),
        properties: vec![],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::SourceSpanMismatch)
    );
}

#[test]
fn resolver_rejects_missing_span_for_mapped_node() {
    let input = M5VisualSelectionInput {
        node_kind: M5StructureNodeKind::SourceElement,
        source_span_ref: None,
        properties: vec![],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::SourceSpanMismatch)
    );
}

#[test]
fn resolver_unmapped_node_is_not_selection_synced() {
    let input = M5VisualSelectionInput {
        node_kind: M5StructureNodeKind::GeneratedNode,
        support_state: M5VisualSupportState::UnmappedNode,
        canvas_state: M5CanvasState::SnapshotStatic,
        source_span_ref: None,
        properties: vec![],
        ..editable_literal_input()
    };
    let resolved = resolve_visual_selection(&input).expect("resolves");
    assert!(!resolved.tree_row.selection_synced);
    assert!(!resolved.tree_row.open_source_action_available);
    assert!(!resolved.canvas_frame.open_source_action_available);
}

#[test]
fn resolver_search_match_highlighting() {
    let matched = resolve_visual_selection(&M5VisualSelectionInput {
        node_label: "PrimaryButton".to_owned(),
        search_query: Some("button".to_owned()),
        properties: vec![],
        ..editable_literal_input()
    })
    .expect("resolves");
    assert!(matched.tree_row.search_match_highlighted);

    let unmatched = resolve_visual_selection(&M5VisualSelectionInput {
        node_label: "PrimaryButton".to_owned(),
        search_query: Some("zzz".to_owned()),
        properties: vec![],
        ..editable_literal_input()
    })
    .expect("resolves");
    assert!(!unmatched.tree_row.search_match_highlighted);
}

#[test]
fn resolver_rejects_inconsistent_write_scope() {
    let input = M5VisualSelectionInput {
        properties: vec![prop(
            "color",
            M5PropertyValueState::DesignToken,
            // A design token can never be recorded as a single literal span.
            M5PropertyWriteScope::SingleLiteralSpan,
            ProtectedPathPosture::Unprotected,
            "token.color.brand",
        )],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::WriteScopeInconsistentWithValue)
    );
}

#[test]
fn resolver_rejects_duplicate_property_key() {
    let input = M5VisualSelectionInput {
        properties: vec![
            prop(
                "color",
                M5PropertyValueState::Literal,
                M5PropertyWriteScope::SingleLiteralSpan,
                ProtectedPathPosture::Unprotected,
                "red",
            ),
            prop(
                "color",
                M5PropertyValueState::Literal,
                M5PropertyWriteScope::SingleLiteralSpan,
                ProtectedPathPosture::Unprotected,
                "blue",
            ),
        ],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::DuplicatePropertyKey(
            "color".to_owned()
        ))
    );
}

#[test]
fn resolver_rejects_empty_selection_id() {
    let input = M5VisualSelectionInput {
        selection_id: "   ".to_owned(),
        properties: vec![],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::EmptySelectionId)
    );
}

#[test]
fn resolver_rejects_forbidden_value_material() {
    let input = M5VisualSelectionInput {
        properties: vec![prop(
            "href",
            M5PropertyValueState::Literal,
            M5PropertyWriteScope::SingleLiteralSpan,
            ProtectedPathPosture::Unprotected,
            "https://example.com",
        )],
        ..editable_literal_input()
    };
    assert_eq!(
        resolve_visual_selection(&input),
        Err(M5VisualSelectionResolutionError::ForbiddenValueMaterial)
    );
}

#[test]
fn resolver_reset_action_availability() {
    let resolved = resolve_visual_selection(&M5VisualSelectionInput {
        properties: vec![
            prop(
                "font_size",
                M5PropertyValueState::Literal,
                M5PropertyWriteScope::SingleLiteralSpan,
                ProtectedPathPosture::Unprotected,
                "18px",
            ),
            prop(
                "elevation",
                M5PropertyValueState::Unset,
                M5PropertyWriteScope::NoWriteInspectOnly,
                ProtectedPathPosture::Unprotected,
                "unset",
            ),
        ],
        ..editable_literal_input()
    })
    .expect("resolves");
    let literal_row = &resolved.inspector_rows[0];
    let unset_row = &resolved.inspector_rows[1];
    assert!(literal_row.reset_action_available);
    assert!(!unset_row.reset_action_available);
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_selected_node_primitive_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_selected_node_primitive_packet();
    let present: BTreeSet<M5VisualDesignSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5VisualDesignSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_example_selections_are_self_consistent() {
    let packet = seeded_m5_selected_node_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_selections {
            assert!(
                case.is_self_consistent(),
                "case drifted on {:?}",
                row.surface_family
            );
        }
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5SelectedNodeVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_selected_node_primitive_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_selected_node_primitive_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5SelectedNodePrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_selected_node_primitive_packet();
    packet.surface_rows[0].flattens_property_value_states = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5SelectedNodePrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_example_is_flagged() {
    let mut packet = seeded_m5_selected_node_primitive_packet();
    // Corrupt the stored resolution so it no longer matches a fresh resolve.
    packet.surface_rows[0].example_selections[0]
        .resolved
        .canvas_frame
        .editable = !packet.surface_rows[0].example_selections[0]
        .resolved
        .canvas_frame
        .editable;
    let violations = packet.validate();
    assert!(violations.contains(&M5SelectedNodePrimitiveViolation::ExampleSelectionDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_selected_node_primitive_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_selected_node_primitive_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_selected_node_primitive_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_selected_node_primitive_packet();
    assert_eq!(packet.record_kind, M5_SELECTED_NODE_PRIMITIVE_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_SELECTED_NODE_PRIMITIVE_SCHEMA_VERSION
    );
}
