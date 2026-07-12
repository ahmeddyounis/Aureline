//! Canonical seed builders for the M5 editor-tab / gutter controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean editor
//! tabs and gutters are built so the shared file/session-state and marker-layering grammar is proven
//! across single-editor, split-editor, diff, notebook, and diagnostics surfaces without any
//! feature-local badge, color-only encoding, continuity loss, or precedence loss.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_PACKET_ID: &str =
    "m5-editor-tab-gutter-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn tab(input: M5EditorTabResolutionInput) -> M5ResolvedEditorTab {
    resolve_editor_tab(input).expect("seed editor tab input resolves")
}

fn gutter(input: M5GutterResolutionInput) -> M5ResolvedGutter {
    resolve_gutter(input).expect("seed gutter input resolves")
}

// -- Clean editor-tab examples (shared item-state grammar across panes) -------------------------

fn clean_tab_base(
    tab_id: &str,
    label: &str,
    context: M5EditorTabState,
    item_state: M5EditorTabItemState,
    pane: M5EditorPaneKind,
) -> M5EditorTabResolutionInput {
    M5EditorTabResolutionInput {
        tab_id: tab_id.to_owned(),
        file_session_label: label.to_owned(),
        tab_context: context,
        item_state,
        item_state_stated: true,
        pane_kind: pane,
        reopen_reveal_continuity_preserved: true,
        has_blocked_tab: false,
        blocked_tab_stated: true,
        invents_feature_local_badge: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean pinned tab in a single editor pane.
fn tab_pinned_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:editor:main-rs",
        "main.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Pinned,
        M5EditorPaneKind::SingleEditor,
    ))
}

/// Clean modified tab (active, unsaved edits) in a split editor pane.
fn tab_modified_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:editor:lib-rs",
        "lib.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Modified,
        M5EditorPaneKind::SplitEditor,
    ))
}

/// Clean read-only tab in a diff pane.
fn tab_read_only_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:diff:cargo-lock",
        "Cargo.lock",
        M5EditorTabState::ReadOnlyLocked,
        M5EditorTabItemState::ReadOnly,
        M5EditorPaneKind::DiffPane,
    ))
}

/// Clean shared / co-edited tab.
fn tab_shared_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:review:design-md",
        "design.md",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Shared,
        M5EditorPaneKind::SingleEditor,
    ))
}

/// Clean generated tab (machine-generated content) in a peek pane.
fn tab_generated_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:editor:generated-rs",
        "generated.rs",
        M5EditorTabState::BackgroundOpen,
        M5EditorTabItemState::Generated,
        M5EditorPaneKind::PeekPane,
    ))
}

/// Clean remote-backed tab.
fn tab_remote_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:editor:remote-rs",
        "remote_module.rs",
        M5EditorTabState::BackgroundOpen,
        M5EditorTabItemState::Remote,
        M5EditorPaneKind::SingleEditor,
    ))
}

/// Clean preview tab (single-click, not yet pinned) in a notebook code cell.
fn tab_preview_clean() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:notebook:cell-preview",
        "cell_3.ipynb",
        M5EditorTabState::PreviewUnpinned,
        M5EditorTabItemState::Preview,
        M5EditorPaneKind::NotebookCodeCell,
    ))
}

// -- Degraded editor-tab examples --------------------------------------------------------------

/// Degraded tab: the file/session identity label is unstated.
fn tab_identity_unstated() -> M5ResolvedEditorTab {
    let mut input = clean_tab_base(
        "tab:editor:no-label",
        "   ",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Pinned,
        M5EditorPaneKind::SingleEditor,
    );
    input.file_session_label = "   ".to_owned();
    tab(input)
}

/// Degraded tab: the tab context (current versus merely open) cannot be resolved.
fn tab_context_unresolved() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:editor:context-unknown",
        "context.rs",
        M5EditorTabState::ContextUnresolved,
        M5EditorTabItemState::Pinned,
        M5EditorPaneKind::SingleEditor,
    ))
}

/// Degraded tab: a feature-local badge is invented for the same file/session state.
fn tab_badge_invented() -> M5ResolvedEditorTab {
    let mut input = clean_tab_base(
        "tab:review:badge",
        "shared.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Shared,
        M5EditorPaneKind::SingleEditor,
    );
    input.invents_feature_local_badge = true;
    tab(input)
}

/// Degraded tab: the item state cannot be resolved.
fn tab_item_state_unknown() -> M5ResolvedEditorTab {
    tab(clean_tab_base(
        "tab:editor:unknown-state",
        "unknown.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::StateUnknown,
        M5EditorPaneKind::SingleEditor,
    ))
}

/// Degraded tab: the item state is encoded by color / hover alone.
fn tab_color_only() -> M5ResolvedEditorTab {
    let mut input = clean_tab_base(
        "tab:editor:color-only",
        "colored.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Modified,
        M5EditorPaneKind::SingleEditor,
    );
    input.item_state_stated = false;
    tab(input)
}

/// Degraded tab: a blocked tab is hidden behind a color / ellipsis cue.
fn tab_blocked_hidden() -> M5ResolvedEditorTab {
    let mut input = clean_tab_base(
        "tab:editor:blocked-hidden",
        "blocked.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Blocked,
        M5EditorPaneKind::SingleEditor,
    );
    input.blocked_tab_stated = false;
    tab(input)
}

/// Degraded tab: reopen/reveal continuity is lost across panes.
fn tab_continuity_lost() -> M5ResolvedEditorTab {
    let mut input = clean_tab_base(
        "tab:diff:continuity-lost",
        "moved.rs",
        M5EditorTabState::BackgroundOpen,
        M5EditorTabItemState::Pinned,
        M5EditorPaneKind::DiffPane,
    );
    input.reopen_reveal_continuity_preserved = false;
    tab(input)
}

/// Degraded tab: no command-backed path to trace the file/session state is reachable.
fn tab_trace_missing() -> M5ResolvedEditorTab {
    let mut input = clean_tab_base(
        "tab:product:trace-missing",
        "traceless.rs",
        M5EditorTabState::ActiveCurrent,
        M5EditorTabItemState::Pinned,
        M5EditorPaneKind::SingleEditor,
    );
    input.detail_command_available = false;
    tab(input)
}

// -- Clean gutter examples ---------------------------------------------------------------------

fn clean_gutter_base(
    gutter_id: &str,
    anchor: &str,
    kind: M5GutterMarkerKind,
    layer: M5GutterMarkerLayer,
) -> M5GutterResolutionInput {
    M5GutterResolutionInput {
        gutter_id: gutter_id.to_owned(),
        anchor_label: anchor.to_owned(),
        marker_kind: kind,
        marker_layer: layer,
        marker_kind_stated: true,
        diagnostic_severity: M5DiagnosticSeverity::Info,
        severity_stated: true,
        layer_precedence_preserved: true,
        readable_in_compact_and_export: true,
        invents_feature_local_badge: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean breakpoint marker on the breakpoint layer.
fn gutter_breakpoint_clean() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:editor:breakpoint-42",
        "main.rs:42",
        M5GutterMarkerKind::Breakpoint,
        M5GutterMarkerLayer::Breakpoint,
    ))
}

/// Clean added-line change marker on the change-marker layer.
fn gutter_change_added_clean() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:diff:added-10",
        "lib.rs:10",
        M5GutterMarkerKind::ChangeAdded,
        M5GutterMarkerLayer::ChangeMarker,
    ))
}

/// Clean modified-line change marker on the change-marker layer.
fn gutter_change_modified_clean() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:diff:modified-11",
        "lib.rs:11",
        M5GutterMarkerKind::ChangeModified,
        M5GutterMarkerLayer::ChangeMarker,
    ))
}

/// Clean error diagnostic on the diagnostic layer, severity stated non-color-only.
fn gutter_diagnostic_error_clean() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:diagnostics:error-77",
        "parser.rs:77",
        M5GutterMarkerKind::ChangeModified,
        M5GutterMarkerLayer::Diagnostic,
    );
    input.diagnostic_severity = M5DiagnosticSeverity::Error;
    gutter(input)
}

/// Clean blame / trust cue on the blame layer, where claimed.
fn gutter_blame_cue_clean() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:editor:blame-5",
        "main.rs:5",
        M5GutterMarkerKind::ChangeModified,
        M5GutterMarkerLayer::BlameOrTrustCue,
    ))
}

/// Clean fold-region affordance on the fold layer.
fn gutter_fold_clean() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:notebook:fold-3",
        "cell_3.ipynb:3",
        M5GutterMarkerKind::FoldRegion,
        M5GutterMarkerLayer::FoldAffordance,
    ))
}

// -- Degraded gutter examples ------------------------------------------------------------------

/// Degraded gutter: the anchor (line / range identity) is unstated.
fn gutter_anchor_unstated() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:editor:no-anchor",
        "  ",
        M5GutterMarkerKind::Breakpoint,
        M5GutterMarkerLayer::Breakpoint,
    );
    input.anchor_label = "  ".to_owned();
    gutter(input)
}

/// Degraded gutter: the marker kind cannot be resolved.
fn gutter_marker_kind_unresolved() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:editor:kind-unknown",
        "main.rs:8",
        M5GutterMarkerKind::MarkerUnresolved,
        M5GutterMarkerLayer::ChangeMarker,
    ))
}

/// Degraded gutter: the marker layer cannot be resolved.
fn gutter_layer_unresolved() -> M5ResolvedGutter {
    gutter(clean_gutter_base(
        "gutter:editor:layer-unknown",
        "main.rs:9",
        M5GutterMarkerKind::Breakpoint,
        M5GutterMarkerLayer::LayerUnresolved,
    ))
}

/// Degraded gutter: a feature-local badge is invented for a marker the shared grammar names.
fn gutter_badge_invented() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:editor:badge",
        "main.rs:12",
        M5GutterMarkerKind::Breakpoint,
        M5GutterMarkerLayer::Breakpoint,
    );
    input.invents_feature_local_badge = true;
    gutter(input)
}

/// Degraded gutter: the marker kind is encoded by color alone.
fn gutter_marker_color_only() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:editor:color-only",
        "main.rs:13",
        M5GutterMarkerKind::ChangeRemoved,
        M5GutterMarkerLayer::ChangeMarker,
    );
    input.marker_kind_stated = false;
    gutter(input)
}

/// Degraded gutter: the diagnostic severity cannot be resolved on the diagnostic layer.
fn gutter_severity_unresolved() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:diagnostics:severity-unknown",
        "parser.rs:80",
        M5GutterMarkerKind::ChangeModified,
        M5GutterMarkerLayer::Diagnostic,
    );
    input.diagnostic_severity = M5DiagnosticSeverity::SeverityUnknown;
    gutter(input)
}

/// Degraded gutter: the diagnostic severity is encoded by color alone on the diagnostic layer.
fn gutter_severity_color_only() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:diagnostics:severity-color-only",
        "parser.rs:81",
        M5GutterMarkerKind::ChangeModified,
        M5GutterMarkerLayer::Diagnostic,
    );
    input.diagnostic_severity = M5DiagnosticSeverity::Warning;
    input.severity_stated = false;
    gutter(input)
}

/// Degraded gutter: layer precedence is lost; layered markers collapse into ambiguity.
fn gutter_precedence_lost() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:diagnostics:precedence-lost",
        "parser.rs:82",
        M5GutterMarkerKind::Breakpoint,
        M5GutterMarkerLayer::Breakpoint,
    );
    input.layer_precedence_preserved = false;
    gutter(input)
}

/// Degraded gutter: the marker layering is not readable in compact / high-zoom / exported views.
fn gutter_unreadable_layering() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:diagnostics:unreadable",
        "parser.rs:83",
        M5GutterMarkerKind::ChangeAdded,
        M5GutterMarkerLayer::ChangeMarker,
    );
    input.readable_in_compact_and_export = false;
    gutter(input)
}

/// Degraded gutter: no command-backed reveal / detail entrypoint is reachable.
fn gutter_reveal_missing() -> M5ResolvedGutter {
    let mut input = clean_gutter_base(
        "gutter:product:reveal-missing",
        "main.rs:14",
        M5GutterMarkerKind::Breakpoint,
        M5GutterMarkerLayer::Breakpoint,
    );
    input.detail_command_available = false;
    gutter(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5EditorTabGutterConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    editor_tab_examples: Vec<M5ResolvedEditorTab>,
    gutter_examples: Vec<M5ResolvedGutter>,
) -> M5EditorTabGutterControlsRow {
    M5EditorTabGutterControlsRow {
        consumer_surface,
        qualification: M5EditorInlineQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EditorInlineDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EditorInlineRequiredLabel::Identity,
            M5EditorInlineRequiredLabel::State,
            M5EditorInlineRequiredLabel::KeyboardRoute,
            M5EditorInlineRequiredLabel::AnchorAndFreshness,
        ],
        accessibility_routes: M5EditorInlineAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5EditorTabGutterAnatomyPart::ALL.to_vec(),
        export_fields: M5EditorTabGutterExportField::ALL.to_vec(),
        downgrade_triggers,
        editor_tab_examples,
        gutter_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_REF,
            M5_EDITOR_TAB_SCHEMA_REF,
            M5_GUTTER_MARKER_SCHEMA_REF,
        ]),
        tabs_invent_feature_local_badges_for_file_session_state: false,
        gutter_markers_encode_state_by_color_alone: false,
        gutter_marker_layering_loses_identity_or_precedence: false,
        reopen_reveal_continuity_breaks_across_panes: false,
    }
}

fn controls_rows() -> Vec<M5EditorTabGutterControlsRow> {
    use M5EditorInlineConsumerSurface as C;
    use M5EditorInlineDowngradeTrigger as D;

    vec![
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor tab strip names the active document context and per-tab pinned/modified/read-only/shared/generated/remote state with no-color-only semantics, and the gutter layers breakpoints and change markers with stable precedence; both degrade honestly when identity is unstated or a marker is encoded by color alone",
            "evidence:m5-editor-tab-gutter-editor-ui:001",
            vec![
                D::TabMarkerDiagnosticColorOnly,
                D::AnchorStateUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                tab_pinned_clean(),
                tab_modified_clean(),
                tab_identity_unstated(),
                tab_color_only(),
            ],
            vec![
                gutter_breakpoint_clean(),
                gutter_blame_cue_clean(),
                gutter_marker_color_only(),
            ],
        ),
        base_row(
            C::DiffUi,
            "Diff / merge surface owner",
            "The diff surface reuses the same tab and gutter grammar for read-only and split panes and added/modified change markers, and degrades honestly when reopen/reveal continuity is lost across panes or a marker kind cannot be resolved",
            "evidence:m5-editor-tab-gutter-diff-ui:001",
            vec![
                D::TabMarkerDiagnosticColorOnly,
                D::DiffChangeKindCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                tab_read_only_clean(),
                tab_continuity_lost(),
                tab_context_unresolved(),
            ],
            vec![
                gutter_change_added_clean(),
                gutter_change_modified_clean(),
                gutter_marker_kind_unresolved(),
            ],
        ),
        base_row(
            C::NotebookUi,
            "Notebook code-pane owner",
            "The notebook code cell reuses the same preview tab grammar and fold-region gutter affordance a user sees in the editor, and degrades honestly when the pane context is unresolved or the marker layer cannot be resolved",
            "evidence:m5-editor-tab-gutter-notebook-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::AnchorStateUnstated,
                D::ProofStale,
            ],
            vec![tab_preview_clean(), tab_generated_clean()],
            vec![gutter_fold_clean(), gutter_layer_unresolved()],
        ),
        base_row(
            C::DiagnosticsUi,
            "Diagnostics gutter owner",
            "The diagnostics surface names problem severity on the diagnostic gutter layer with no-color-only semantics and keeps layer precedence readable in compact, high-zoom, and exported views, degrading honestly when precedence is lost, layering is unreadable, or severity is unresolved",
            "evidence:m5-editor-tab-gutter-diagnostics-ui:001",
            vec![
                D::TabMarkerDiagnosticColorOnly,
                D::DiagnosticFreshnessUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![tab_remote_clean(), tab_item_state_unknown()],
            vec![
                gutter_diagnostic_error_clean(),
                gutter_severity_unresolved(),
                gutter_severity_color_only(),
                gutter_precedence_lost(),
                gutter_unreadable_layering(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved tab and gutter truth, so an invented feature-local badge, a blocked tab hidden behind a color cue, or an unresolved anchor is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-editor-tab-gutter-support-export:001",
            vec![
                D::GenericChromeWordingUsed,
                D::TabMarkerDiagnosticColorOnly,
                D::AnchorStateUnstated,
                D::ProofStale,
            ],
            vec![
                tab_shared_clean(),
                tab_badge_invented(),
                tab_blocked_hidden(),
            ],
            vec![
                gutter_breakpoint_clean(),
                gutter_badge_invented(),
                gutter_anchor_unstated(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product editor owner",
            "In-product surfaces reuse the same file/session and gutter state grammar a user sees in the editor, always offering the command-backed detail/reveal path and degrading honestly when the trace path is missing",
            "evidence:m5-editor-tab-gutter-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::TabMarkerDiagnosticColorOnly,
                D::ProofStale,
            ],
            vec![tab_pinned_clean(), tab_trace_missing()],
            vec![gutter_breakpoint_clean(), gutter_reveal_missing()],
        ),
    ]
}

fn governance_review() -> M5EditorTabGutterGovernanceReview {
    M5EditorTabGutterGovernanceReview {
        tab_names_context_and_item_state: true,
        tab_item_state_no_color_only: true,
        gutter_names_marker_kind_and_layer: true,
        gutter_layering_readable_across_representations: true,
        tabs_never_invent_feature_local_badges: true,
        gutters_never_invent_feature_local_badges: true,
        state_never_encoded_by_color_alone: true,
        blocked_and_hidden_never_behind_color_or_ellipsis: true,
        reopen_reveal_continuity_preserved_across_panes: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5EditorTabGutterConsumerProjection {
    M5EditorTabGutterConsumerProjection {
        editor_surfaces_consume_tab_and_gutter_vocabulary: true,
        diff_surfaces_consume_tab_and_gutter_vocabulary: true,
        notebook_consumes_tab_and_gutter_vocabulary: true,
        diagnostics_consume_marker_and_severity_vocabulary: true,
        state_facts_trace_to_single_component_contract: true,
        support_export_reads_single_editor_source: true,
    }
}

fn proof_freshness() -> M5EditorTabGutterProofFreshness {
    M5EditorTabGutterProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EditorTabGutterReleasePosture {
    M5EditorTabGutterReleasePosture {
        proof_packet_ref: M5_EDITOR_TAB_GUTTER_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_EDITOR_TAB_GUTTER_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_REF,
        M5_EDITOR_TAB_GUTTER_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_EDITOR_TAB_SCHEMA_REF,
        M5_GUTTER_MARKER_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 editor-tab / gutter controls packet.
pub fn seeded_m5_editor_tab_gutter_controls() -> M5EditorTabGutterControlsPacket {
    M5EditorTabGutterControlsPacket::new(M5EditorTabGutterControlsPacketInput {
        packet_id: M5_EDITOR_TAB_GUTTER_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 editor-tab and gutter controls with modified/preview/pinned/read-only/blocked/shared/generated/remote file-session state, breakpoint/change-marker/diagnostic/blame layering, and reopen/reveal continuity aligned across editor, diff, notebook, diagnostics, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5EditorTabGutterVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the editor-UI row is held at Beta pending file/session-state parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed(
) -> M5EditorTabGutterControlsPacket {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.packet_id = "m5-editor-tab-gutter-controls:editor-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics-UI row is narrowed to Preview pending gutter-layering parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed(
) -> M5EditorTabGutterControlsPacket {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.packet_id = "m5-editor-tab-gutter-controls:diagnostics-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::DiagnosticsUi)
        .expect("diagnostics-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Preview;
    packet
}
