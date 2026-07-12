//! Canonical seed builders for the M5 table / grid and panel-header controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean grids and
//! panel headers are built so the shared selection / sort-filter / count-scope grammar is proven
//! across surfaces without any hover-only discovery, count-scope collapse, faked-canonical value,
//! lost pinned column, or re-encoded / overloaded header.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_TABLE_GRID_PANEL_HEADER_CONTROLS_PACKET_ID: &str =
    "m5-table-grid-panel-header-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn grid(input: M5TableGridResolutionInput) -> M5ResolvedTableGrid {
    resolve_table_grid(input).expect("seed table grid input resolves")
}

fn header(input: M5PanelHeaderResolutionInput) -> M5ResolvedPanelHeader {
    resolve_panel_header(input).expect("seed panel header input resolves")
}

// -- Clean grid examples (shared selection / sort-filter / count grammar across surfaces) --------

#[allow(clippy::too_many_arguments)]
fn clean_grid_base(
    grid_id: &str,
    label: &str,
    selection: M5SelectionState,
    item_state: M5ItemStateFlag,
    provenance: M5SortFilterProvenance,
    pinned_column: M5PinnedColumnState,
    value_qualification: M5ValueQualification,
    count_scope: M5TablePanelScopeKind,
    density: M5DensityVariant,
) -> M5TableGridResolutionInput {
    M5TableGridResolutionInput {
        grid_id: grid_id.to_owned(),
        grid_label: label.to_owned(),
        selection,
        selection_versus_current_distinct: true,
        row_focus_visible: true,
        current_selection_hover_only: false,
        item_state,
        has_blocked_row: false,
        blocked_state_hover_only: false,
        sort_filter_provenance: provenance,
        pinned_column,
        pinned_column_identity_lost: false,
        value_qualification,
        qualified_value_shown_as_canonical: false,
        count_scope,
        count_scopes_distinct: true,
        loaded_shown_as_exact: false,
        density,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        backend_stale_or_partial: false,
        presents_stale_or_partial_as_complete: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean grid naming an exact, user-sorted count with a pinned identity column.
fn grid_exact_clean(grid_id: &str, label: &str) -> M5ResolvedTableGrid {
    grid(clean_grid_base(
        grid_id,
        label,
        M5SelectionState::SelectedAndCurrent,
        M5ItemStateFlag::Pinned,
        M5SortFilterProvenance::UserSorted,
        M5PinnedColumnState::IdentityColumnPinned,
        M5ValueQualification::ExactCanonical,
        M5TablePanelScopeKind::ExactCount,
        M5DensityVariant::Comfortable,
    ))
}

/// Clean grid keeping an identity column pinned while honestly qualifying imported values under a
/// partially-loaded backend (never claimed complete). Anchors the pinned-identity acceptance
/// criterion.
fn grid_pinned_qualified_clean(grid_id: &str, label: &str) -> M5ResolvedTableGrid {
    let mut input = clean_grid_base(
        grid_id,
        label,
        M5SelectionState::SingleSelected,
        M5ItemStateFlag::ReadOnly,
        M5SortFilterProvenance::ImportedOrder,
        M5PinnedColumnState::IdentityColumnPinned,
        M5ValueQualification::Imported,
        M5TablePanelScopeKind::AllMatchingCount,
        M5DensityVariant::Dense,
    );
    input.backend_stale_or_partial = true;
    grid(input)
}

/// Clean grid naming a hidden-by-filter count with an estimated value qualification and a leading
/// pinned column.
fn grid_hidden_filter_clean(grid_id: &str, label: &str) -> M5ResolvedTableGrid {
    grid(clean_grid_base(
        grid_id,
        label,
        M5SelectionState::MultiSelected,
        M5ItemStateFlag::Modified,
        M5SortFilterProvenance::FilterApplied,
        M5PinnedColumnState::LeadingPinned,
        M5ValueQualification::Estimated,
        M5TablePanelScopeKind::HiddenByFilter,
        M5DensityVariant::Compact,
    ))
}

/// Clean grid naming an outside-current-scope count under a single-line default sort.
fn grid_outside_clean(grid_id: &str, label: &str) -> M5ResolvedTableGrid {
    grid(clean_grid_base(
        grid_id,
        label,
        M5SelectionState::NoneSelected,
        M5ItemStateFlag::Preview,
        M5SortFilterProvenance::DefaultSort,
        M5PinnedColumnState::Unpinned,
        M5ValueQualification::ExactCanonical,
        M5TablePanelScopeKind::OutsideCurrentScope,
        M5DensityVariant::SingleLine,
    ))
}

/// Clean grid honestly naming a loaded / virtualized subset of relevance-ranked partial values (never
/// shown as the exact total).
fn grid_loaded_clean(grid_id: &str, label: &str) -> M5ResolvedTableGrid {
    let mut input = clean_grid_base(
        grid_id,
        label,
        M5SelectionState::CurrentNotSelected,
        M5ItemStateFlag::ReadOnly,
        M5SortFilterProvenance::RelevanceRanked,
        M5PinnedColumnState::TrailingPinned,
        M5ValueQualification::Partial,
        M5TablePanelScopeKind::LoadedCount,
        M5DensityVariant::CondensedOverflow,
    );
    input.backend_stale_or_partial = true;
    grid(input)
}

// -- Degraded grid examples --------------------------------------------------------------------

/// A fully-honest grid input used as the mutation base for the degraded examples below.
fn grid_exact_input(grid_id: &str, label: &str) -> M5TableGridResolutionInput {
    clean_grid_base(
        grid_id,
        label,
        M5SelectionState::SelectedAndCurrent,
        M5ItemStateFlag::Pinned,
        M5SortFilterProvenance::UserSorted,
        M5PinnedColumnState::IdentityColumnPinned,
        M5ValueQualification::ExactCanonical,
        M5TablePanelScopeKind::ExactCount,
        M5DensityVariant::Comfortable,
    )
}

/// Degraded grid: the current grid / row identity is unstated.
fn grid_identity_unstated() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:shell:no-id", "  ");
    input.grid_label = "  ".to_owned();
    grid(input)
}

/// Degraded grid: the selection is collapsed into the current / focused row.
fn grid_selection_collapsed() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:product:selection-collapsed", "rows");
    input.selection_versus_current_distinct = false;
    grid(input)
}

/// Degraded grid: the current selection can only be discovered by pointer hover.
fn grid_current_hover() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:search:current-hover", "match 12");
    input.current_selection_hover_only = true;
    grid(input)
}

/// Degraded grid: a blocked row's state can only be discovered by pointer hover.
fn grid_blocked_hover() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:review:blocked-hover", "queue item 3");
    input.item_state = M5ItemStateFlag::Blocked;
    input.blocked_state_hover_only = true;
    grid(input)
}

/// Degraded grid: the sort / filter provenance cannot be resolved.
fn grid_provenance_unstated() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:data:provenance-unstated", "provider rows");
    input.sort_filter_provenance = M5SortFilterProvenance::ProvenanceUnknown;
    grid(input)
}

/// Degraded grid: a pinned identity column lost its identity under virtualization / overflow.
fn grid_pinned_lost() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:shell:pin-lost", "records");
    input.pinned_column_identity_lost = true;
    grid(input)
}

/// Degraded grid: the pinned-column posture cannot be resolved.
fn grid_pin_unresolved() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:product:pin-unresolved", "columns");
    input.pinned_column = M5PinnedColumnState::PinUnresolved;
    grid(input)
}

/// Degraded grid: an estimated value is presented as exact canonical truth.
fn grid_qualified_as_canonical() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:shell:qualified-as-canonical", "totals");
    input.value_qualification = M5ValueQualification::Estimated;
    input.qualified_value_shown_as_canonical = true;
    grid(input)
}

/// Degraded grid: the value qualification cannot be resolved.
fn grid_value_qual_unresolved() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:support:value-qual-unresolved", "values");
    input.value_qualification = M5ValueQualification::QualificationUnknown;
    grid(input)
}

/// Degraded grid: the exact / loaded / all-matching count scopes collapse into one total.
fn grid_scope_collapsed() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:data:scope-collapsed", "results");
    input.count_scopes_distinct = false;
    grid(input)
}

/// Degraded grid: the count scope cannot be resolved.
fn grid_scope_unresolved() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:support:scope-unresolved", "records");
    input.count_scope = M5TablePanelScopeKind::ScopeUnresolved;
    grid(input)
}

/// Degraded grid: a loaded / virtualized subset is presented as the exact total.
fn grid_loaded_as_exact() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:review:loaded-as-exact", "queue item 40");
    input.count_scope = M5TablePanelScopeKind::LoadedCount;
    input.loaded_shown_as_exact = true;
    grid(input)
}

/// Degraded grid: a stale / partial backend is presented as a complete grid.
fn grid_stale_complete() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:support:stale-complete", "stale rows");
    input.backend_stale_or_partial = true;
    input.presents_stale_or_partial_as_complete = true;
    grid(input)
}

/// Degraded grid: no command-backed path to trace the collection scope is reachable.
fn grid_trace_missing() -> M5ResolvedTableGrid {
    let mut input = grid_exact_input("grid:product:trace-missing", "workspace rows");
    input.detail_command_available = false;
    grid(input)
}

// -- Clean panel-header examples ---------------------------------------------------------------

fn clean_header_base(
    header_id: &str,
    label: &str,
    active_context: M5ActiveContextState,
    budget: M5LocalActionBudget,
) -> M5PanelHeaderResolutionInput {
    M5PanelHeaderResolutionInput {
        header_id: header_id.to_owned(),
        header_label: label.to_owned(),
        active_context,
        background_context_shown_as_active: false,
        local_action_budget: budget,
        local_actions_hover_only: false,
        becomes_secondary_toolbar: false,
        overflowed_action_dropped: false,
        references_canonical_model: true,
        re_encodes_canonical_counts_locally: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean header naming the active, current context with a within-budget action set.
fn header_active_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActiveCurrent,
        M5LocalActionBudget::WithinBudget,
    ))
}

/// Clean header naming a pinned active context with a primary-plus-overflow action budget.
fn header_pinned_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActivePinned,
        M5LocalActionBudget::PrimaryPlusOverflow,
    ))
}

/// Clean header honestly naming a background-open context (never presented as active) with no local
/// actions.
fn header_background_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::BackgroundOpen,
        M5LocalActionBudget::NoLocalActions,
    ))
}

// -- Degraded panel-header examples ------------------------------------------------------------

/// A fully-honest header input used as the mutation base for the degraded examples below.
fn header_active_input(header_id: &str, label: &str) -> M5PanelHeaderResolutionInput {
    clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActiveCurrent,
        M5LocalActionBudget::WithinBudget,
    )
}

/// Degraded header: the header identity is unstated.
fn header_identity_unstated() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:product:no-id", "  ");
    input.header_label = "  ".to_owned();
    header(input)
}

/// Degraded header: the active context cannot be resolved.
fn header_context_unresolved() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:search:context-unresolved", "Results");
    input.active_context = M5ActiveContextState::ContextUnresolved;
    header(input)
}

/// Degraded header: a background context is presented as the active one.
fn header_background_as_active() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:shell:background-as-active", "Panel");
    input.active_context = M5ActiveContextState::BackgroundOpen;
    input.background_context_shown_as_active = true;
    header(input)
}

/// Degraded header: the local actions can only be discovered by pointer hover.
fn header_actions_hover() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:support:actions-hover", "Export");
    input.local_actions_hover_only = true;
    header(input)
}

/// Degraded header: the header overloaded into a cluttered secondary toolbar.
fn header_overloaded() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:review:overloaded", "Review");
    input.becomes_secondary_toolbar = true;
    header(input)
}

/// Degraded header: an overflowed local action was silently dropped.
fn header_overflow_dropped() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:support:overflow-dropped", "Diagnostics");
    input.overflowed_action_dropped = true;
    header(input)
}

/// Degraded header: the header re-encodes the canonical count / selection model in surface-local
/// copy.
fn header_re_encodes() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:data:re-encodes", "Providers");
    input.re_encodes_canonical_counts_locally = true;
    header(input)
}

/// Degraded header: no command-backed path to trace the collection scope is reachable.
fn header_trace_missing() -> M5ResolvedPanelHeader {
    let mut input = header_active_input("header:product:trace-missing", "Workspace");
    input.detail_command_available = false;
    header(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5TablePanelConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    table_grid_examples: Vec<M5ResolvedTableGrid>,
    panel_header_examples: Vec<M5ResolvedPanelHeader>,
) -> M5TablePanelControlsRow {
    M5TablePanelControlsRow {
        consumer_surface,
        qualification: M5NavigationContentQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5NavigationContentDeploymentLine::ALL.to_vec(),
        required_labels: M5NavigationContentRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5NavigationContentAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5TablePanelAnatomyPart::ALL.to_vec(),
        export_fields: M5TablePanelExportField::ALL.to_vec(),
        downgrade_triggers,
        table_grid_examples,
        panel_header_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_REF,
            M5_TABLE_GRID_SCHEMA_REF,
            M5_PANEL_HEADER_SCHEMA_REF,
        ]),
        hides_current_selection_blocked_or_actions_behind_hover_only: false,
        collapses_selection_versus_current_or_count_scopes: false,
        presents_qualified_stale_or_partial_grid_as_canonical: false,
        panel_header_overloads_or_re_encodes_counts: false,
    }
}

fn controls_rows() -> Vec<M5TablePanelControlsRow> {
    use M5NavigationContentConsumerSurface as C;
    use M5NavigationContentDowngradeTrigger as D;

    vec![
        base_row(
            C::DataUi,
            "Request / data grid owner",
            "The request/data grid names selection-versus-current, sort/filter provenance, pinned identity columns, per-value qualification, and exact/loaded/all-matching/hidden/outside-scope counts, degrading when the provenance is unstated or a count scope collapses",
            "evidence:m5-table-panel-data-ui:001",
            vec![
                D::SelectionVersusCurrentCollapsed,
                D::CountScopeCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                grid_exact_clean("grid:data:providers", "provider row 1"),
                grid_pinned_qualified_clean("grid:data:imported", "imported row 7"),
                grid_provenance_unstated(),
                grid_scope_collapsed(),
            ],
            vec![
                header_active_clean("header:data:providers", "Providers"),
                header_re_encodes(),
            ],
        ),
        base_row(
            C::ReviewUi,
            "Review-queue grid owner",
            "The review queue reuses the shared grid selection and scope grammar for queued items, keeping blocked state keyboard-discoverable and degrading when a loaded subset is shown as the exact total or a header overloads into a toolbar",
            "evidence:m5-table-panel-review-ui:001",
            vec![
                D::CountScopeCollapsed,
                D::BlockedRowsHiddenBehindEllipsis,
                D::PanelHeaderOverloaded,
                D::ProofStale,
            ],
            vec![
                grid_hidden_filter_clean("grid:review:filtered", "review item 2"),
                grid_blocked_hover(),
                grid_loaded_as_exact(),
            ],
            vec![
                header_pinned_clean("header:review:queue", "Review"),
                header_overloaded(),
            ],
        ),
        base_row(
            C::SearchUi,
            "Search results grid owner",
            "The search surface reuses the same grid row semantics for result collections, honestly naming a relevance-ranked loaded subset and degrading when the current selection is hover-only or the header's active context is unresolved",
            "evidence:m5-table-panel-search-ui:001",
            vec![
                D::SelectionVersusCurrentCollapsed,
                D::ActiveContextUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                grid_loaded_clean("grid:search:hits", "search hit 3"),
                grid_current_hover(),
            ],
            vec![
                header_active_clean("header:search:results", "Search"),
                header_context_unresolved(),
            ],
        ),
        base_row(
            C::ShellUi,
            "Governance / shell grid owner",
            "Governance surfaces reuse the same grid grammar, keeping pinned identity columns stable under virtualization and never presenting an estimated value as canonical, degrading when a pinned column is lost or a background context reads as active",
            "evidence:m5-table-panel-shell-ui:001",
            vec![
                D::CountScopeCollapsed,
                D::ActiveContextUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                grid_outside_clean("grid:shell:governed", "governed record 5"),
                grid_pinned_lost(),
                grid_qualified_as_canonical(),
            ],
            vec![
                header_background_clean("header:shell:background", "Background panel"),
                header_background_as_active(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export grid owner",
            "The support export carries the same resolved grid and header truth, so a stale-shown-complete grid, an unresolved count scope, an unresolved value qualification, hover-only header actions, or a dropped overflow action is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-table-panel-support-export:001",
            vec![
                D::LocalActionsHoverOnly,
                D::PanelHeaderOverloaded,
                D::CountScopeCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                grid_exact_clean("grid:support:exact", "support row 1"),
                grid_stale_complete(),
                grid_scope_unresolved(),
                grid_value_qual_unresolved(),
            ],
            vec![
                header_active_clean("header:support:export", "Support export"),
                header_actions_hover(),
                header_overflow_dropped(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product grid owner",
            "In-product surfaces reuse the same selection, sort/filter, and scope grammar a user sees in the request/data and review grids, always offering the command-backed scope detail and degrading honestly when the trace path is missing or the pinned column is unresolved",
            "evidence:m5-table-panel-product-ui:001",
            vec![
                D::SelectionVersusCurrentCollapsed,
                D::ActiveContextUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                grid_exact_clean("grid:product:recent", "recent row 1"),
                grid_identity_unstated(),
                grid_selection_collapsed(),
                grid_pin_unresolved(),
                grid_trace_missing(),
            ],
            vec![
                header_active_clean("header:product:workspace", "Workspace"),
                header_identity_unstated(),
                header_trace_missing(),
            ],
        ),
    ]
}

fn governance_review() -> M5TablePanelGovernanceReview {
    M5TablePanelGovernanceReview {
        table_names_selection_sort_filter_and_counts: true,
        table_value_qualification_honest_never_canonical_overclaim: true,
        table_pinned_column_identity_stable_under_virtualization: true,
        table_loaded_never_shown_as_exact: true,
        panel_header_names_active_context_and_bounded_actions: true,
        panel_header_never_becomes_secondary_toolbar: true,
        panel_header_references_canonical_count_and_selection_model: true,
        selection_versus_current_always_distinct: true,
        current_selection_blocked_and_actions_never_hover_only: true,
        count_scopes_never_collapsed: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5TablePanelConsumerProjection {
    M5TablePanelConsumerProjection {
        data_surface_consumes_table_sort_filter_and_scope_vocabulary: true,
        review_queue_consumes_table_selection_and_scope_vocabulary: true,
        governance_and_support_consume_shared_grid_semantics: true,
        grid_facts_trace_to_single_header_and_selection_model: true,
        support_export_reads_single_grid_source: true,
    }
}

fn proof_freshness() -> M5TablePanelProofFreshness {
    M5TablePanelProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TablePanelReleasePosture {
    M5TablePanelReleasePosture {
        proof_packet_ref: M5_TABLE_GRID_PANEL_HEADER_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_TABLE_GRID_PANEL_HEADER_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TABLE_GRID_PANEL_HEADER_CONTROLS_SCHEMA_REF,
        M5_TABLE_GRID_PANEL_HEADER_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TABLE_GRID_SCHEMA_REF,
        M5_PANEL_HEADER_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 table / grid and panel-header controls packet.
pub fn seeded_m5_table_grid_panel_header_controls() -> M5TablePanelControlsPacket {
    M5TablePanelControlsPacket::new(M5TablePanelControlsPacketInput {
        packet_id: M5_TABLE_GRID_PANEL_HEADER_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 table/grid and panel-header controls with sort/filter provenance, selection bars, pinned-column identity, per-value qualification, bounded panel-header action budgets, and exact/loaded/all-matching/hidden/outside-scope count truth aligned across request/data, review, search, governance, and support surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5TablePanelVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the request/data-UI row is held at Beta pending grid parity on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_table_grid_panel_header_controls_data_ui_beta_narrowed(
) -> M5TablePanelControlsPacket {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.packet_id = "m5-table-grid-panel-header-controls:data-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::DataUi)
        .expect("data-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review-UI row is narrowed to Preview pending queue-grid parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_table_grid_panel_header_controls_review_ui_preview_narrowed(
) -> M5TablePanelControlsPacket {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.packet_id = "m5-table-grid-panel-header-controls:review-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Preview;
    packet
}
