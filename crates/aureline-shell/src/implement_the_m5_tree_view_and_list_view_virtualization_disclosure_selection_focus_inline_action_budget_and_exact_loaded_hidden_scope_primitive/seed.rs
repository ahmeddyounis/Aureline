//! Canonical seed builders for the M5 tree-view / list-view controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean tree
//! views and list views are built so the shared disclosure / selection / count-scope grammar is
//! proven across surfaces without any hover-only discovery, count-scope collapse, faked-complete
//! tree, or overclaimed drag / continuity.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_TREE_VIEW_LIST_VIEW_CONTROLS_PACKET_ID: &str =
    "m5-tree-view-list-view-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn tree(input: M5TreeViewResolutionInput) -> M5ResolvedTreeView {
    resolve_tree_view(input).expect("seed tree view input resolves")
}

fn list(input: M5ListViewResolutionInput) -> M5ResolvedListView {
    resolve_list_view(input).expect("seed list view input resolves")
}

// -- Clean tree examples (shared disclosure / selection / count grammar across surfaces) --------

#[allow(clippy::too_many_arguments)]
fn clean_tree_base(
    tree_id: &str,
    label: &str,
    disclosure: M5DisclosureState,
    selection: M5SelectionState,
    item_state: M5ItemStateFlag,
    count_scope: M5TreeListScopeKind,
    density: M5DensityVariant,
    drag_reorder: M5DragReorderPosture,
    continuity: M5CrossSurfaceContinuity,
) -> M5TreeViewResolutionInput {
    M5TreeViewResolutionInput {
        tree_id: tree_id.to_owned(),
        node_label: label.to_owned(),
        disclosure,
        lazy_subtree_shown_as_leaf: false,
        selection,
        selection_versus_current_distinct: true,
        row_focus_visible: true,
        current_selection_hover_only: false,
        item_state,
        has_blocked_row: false,
        blocked_state_hover_only: false,
        count_scope,
        count_scopes_distinct: true,
        density,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        drag_reorder,
        overclaims_drag_reorder: false,
        cross_surface_continuity: continuity,
        overclaims_cross_surface_continuity: false,
        backend_stale_or_partial: false,
        presents_stale_or_partial_as_complete: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean expanded explorer tree naming an exact count.
fn tree_expanded_clean(tree_id: &str, label: &str) -> M5ResolvedTreeView {
    tree(clean_tree_base(
        tree_id,
        label,
        M5DisclosureState::Expanded,
        M5SelectionState::SingleSelected,
        M5ItemStateFlag::Pinned,
        M5TreeListScopeKind::ExactCount,
        M5DensityVariant::Comfortable,
        M5DragReorderPosture::ReorderEnabled,
        M5CrossSurfaceContinuity::SinglePaneOnly,
    ))
}

/// Clean tree that honestly discloses a lazily-unloaded / partial backend (never claimed complete).
fn tree_partial_honest_clean(tree_id: &str, label: &str) -> M5ResolvedTreeView {
    let mut input = clean_tree_base(
        tree_id,
        label,
        M5DisclosureState::LazyUnloaded,
        M5SelectionState::CurrentNotSelected,
        M5ItemStateFlag::ReadOnly,
        M5TreeListScopeKind::LoadedCount,
        M5DensityVariant::Dense,
        M5DragReorderPosture::ReorderWithinScopeOnly,
        M5CrossSurfaceContinuity::CrossPaneMirrored,
    );
    input.backend_stale_or_partial = true;
    tree(input)
}

/// Clean tree naming a hidden-by-filter count, reorder scoped to the current parent.
fn tree_hidden_filter_clean(tree_id: &str, label: &str) -> M5ResolvedTreeView {
    tree(clean_tree_base(
        tree_id,
        label,
        M5DisclosureState::PartiallyExpanded,
        M5SelectionState::MultiSelected,
        M5ItemStateFlag::Modified,
        M5TreeListScopeKind::HiddenByFilter,
        M5DensityVariant::Compact,
        M5DragReorderPosture::ReorderWithinScopeOnly,
        M5CrossSurfaceContinuity::CrossWindowMirrored,
    ))
}

/// Clean tree naming an outside-current-scope count on a read-only, single-pane surface.
fn tree_outside_scope_clean(tree_id: &str, label: &str) -> M5ResolvedTreeView {
    tree(clean_tree_base(
        tree_id,
        label,
        M5DisclosureState::Collapsed,
        M5SelectionState::SelectedAndCurrent,
        M5ItemStateFlag::ReadOnly,
        M5TreeListScopeKind::OutsideCurrentScope,
        M5DensityVariant::CondensedOverflow,
        M5DragReorderPosture::ReorderReadOnly,
        M5CrossSurfaceContinuity::ContinuityNotSupported,
    ))
}

// -- Degraded tree examples --------------------------------------------------------------------

/// Degraded tree: the current node identity is unstated.
fn tree_node_unstated() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:shell:no-node", "   ");
    input.node_label = "   ".to_owned();
    tree(input)
}

/// Degraded tree: the disclosure state cannot be resolved.
fn tree_disclosure_unknown() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:explorer:disclosure-unknown", "src");
    input.disclosure = M5DisclosureState::DisclosureUnknown;
    tree(input)
}

/// Degraded tree: a lazily-unloaded subtree is drawn as an empty leaf.
fn tree_lazy_leaf() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:explorer:lazy-leaf", "vendor");
    input.disclosure = M5DisclosureState::LazyUnloaded;
    input.lazy_subtree_shown_as_leaf = true;
    tree(input)
}

/// Degraded tree: the selection is collapsed into the current / focused item.
fn tree_selection_collapsed() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:explorer:selection-collapsed", "models");
    input.selection_versus_current_distinct = false;
    tree(input)
}

/// Degraded tree: the current selection can only be discovered by pointer hover.
fn tree_current_hover() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:data:current-hover", "providers");
    input.current_selection_hover_only = true;
    tree(input)
}

/// Degraded tree: a blocked row's state can only be discovered by pointer hover.
fn tree_blocked_hover() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:review:blocked-hover", "conflicts");
    input.item_state = M5ItemStateFlag::Blocked;
    input.blocked_state_hover_only = true;
    tree(input)
}

/// Degraded tree: the local actions can only be discovered by pointer hover.
fn tree_actions_hover() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:support:actions-hover", "artifacts");
    input.local_actions_hover_only = true;
    tree(input)
}

/// Degraded tree: the exact / loaded / all-matching count scopes collapse into one total.
fn tree_scope_collapsed() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:search:scope-collapsed", "results");
    input.count_scopes_distinct = false;
    tree(input)
}

/// Degraded tree: a stale / partial backend is presented as a complete tree.
fn tree_stale_complete() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:support:stale-complete", "stale_root");
    input.backend_stale_or_partial = true;
    input.presents_stale_or_partial_as_complete = true;
    tree(input)
}

/// Degraded tree: no command-backed path to trace the collection scope is reachable.
fn tree_trace_missing() -> M5ResolvedTreeView {
    let mut input = tree_expanded_input("tree:product:trace-missing", "workspace");
    input.detail_command_available = false;
    tree(input)
}

/// A fully-honest tree input used as the mutation base for the degraded examples above.
fn tree_expanded_input(tree_id: &str, label: &str) -> M5TreeViewResolutionInput {
    clean_tree_base(
        tree_id,
        label,
        M5DisclosureState::Expanded,
        M5SelectionState::SingleSelected,
        M5ItemStateFlag::Pinned,
        M5TreeListScopeKind::ExactCount,
        M5DensityVariant::Comfortable,
        M5DragReorderPosture::ReorderEnabled,
        M5CrossSurfaceContinuity::SinglePaneOnly,
    )
}

// -- Clean list examples -----------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_list_base(
    list_id: &str,
    label: &str,
    selection: M5SelectionState,
    item_state: M5ItemStateFlag,
    count_scope: M5TreeListScopeKind,
    density: M5DensityVariant,
    drag_reorder: M5DragReorderPosture,
    continuity: M5CrossSurfaceContinuity,
) -> M5ListViewResolutionInput {
    M5ListViewResolutionInput {
        list_id: list_id.to_owned(),
        row_label: label.to_owned(),
        selection,
        selection_versus_current_distinct: true,
        row_focus_visible: true,
        current_selection_hover_only: false,
        item_state,
        has_blocked_row: false,
        blocked_state_hover_only: false,
        count_scope,
        count_scopes_distinct: true,
        loaded_shown_as_exact: false,
        density,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        drag_reorder,
        overclaims_drag_reorder: false,
        cross_surface_continuity: continuity,
        overclaims_cross_surface_continuity: false,
        backend_stale_or_partial: false,
        presents_stale_or_partial_as_complete: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean list naming an exact count with selection distinct from the current row.
fn list_exact_clean(list_id: &str, label: &str) -> M5ResolvedListView {
    list(clean_list_base(
        list_id,
        label,
        M5SelectionState::SelectedAndCurrent,
        M5ItemStateFlag::Pinned,
        M5TreeListScopeKind::ExactCount,
        M5DensityVariant::Comfortable,
        M5DragReorderPosture::ReorderEnabled,
        M5CrossSurfaceContinuity::CrossWindowMirrored,
    ))
}

/// Clean review-queue list with a multi-selection and a hidden-by-policy count.
fn list_hidden_policy_clean(list_id: &str, label: &str) -> M5ResolvedListView {
    list(clean_list_base(
        list_id,
        label,
        M5SelectionState::MultiSelected,
        M5ItemStateFlag::Modified,
        M5TreeListScopeKind::HiddenByPolicy,
        M5DensityVariant::Dense,
        M5DragReorderPosture::ReorderDisabledByPolicy,
        M5CrossSurfaceContinuity::CrossPaneMirrored,
    ))
}

/// Clean provider list honestly naming a loaded / virtualized subset (never shown as the total).
fn list_loaded_clean(list_id: &str, label: &str) -> M5ResolvedListView {
    let mut input = clean_list_base(
        list_id,
        label,
        M5SelectionState::CurrentNotSelected,
        M5ItemStateFlag::ReadOnly,
        M5TreeListScopeKind::LoadedCount,
        M5DensityVariant::Compact,
        M5DragReorderPosture::ReorderNotSupported,
        M5CrossSurfaceContinuity::SinglePaneOnly,
    );
    input.backend_stale_or_partial = true;
    list(input)
}

/// Clean list naming an outside-current-scope count.
fn list_outside_clean(list_id: &str, label: &str) -> M5ResolvedListView {
    list(clean_list_base(
        list_id,
        label,
        M5SelectionState::NoneSelected,
        M5ItemStateFlag::Preview,
        M5TreeListScopeKind::OutsideCurrentScope,
        M5DensityVariant::SingleLine,
        M5DragReorderPosture::ReorderReadOnly,
        M5CrossSurfaceContinuity::ContinuityNotSupported,
    ))
}

// -- Degraded list examples --------------------------------------------------------------------

/// A fully-honest list input used as the mutation base for the degraded examples below.
fn list_exact_input(list_id: &str, label: &str) -> M5ListViewResolutionInput {
    clean_list_base(
        list_id,
        label,
        M5SelectionState::SelectedAndCurrent,
        M5ItemStateFlag::Pinned,
        M5TreeListScopeKind::ExactCount,
        M5DensityVariant::Comfortable,
        M5DragReorderPosture::ReorderEnabled,
        M5CrossSurfaceContinuity::CrossPaneMirrored,
    )
}

/// Degraded list: the current row identity is unstated.
fn list_row_unstated() -> M5ResolvedListView {
    let mut input = list_exact_input("list:shell:no-row", "  ");
    input.row_label = "  ".to_owned();
    list(input)
}

/// Degraded list: the current selection can only be discovered by pointer hover.
fn list_current_hover() -> M5ResolvedListView {
    let mut input = list_exact_input("list:search:current-hover", "match 12");
    input.current_selection_hover_only = true;
    list(input)
}

/// Degraded list: a blocked row's state can only be discovered by pointer hover.
fn list_blocked_hover() -> M5ResolvedListView {
    let mut input = list_exact_input("list:review:blocked-hover", "review item 3");
    input.item_state = M5ItemStateFlag::Blocked;
    input.blocked_state_hover_only = true;
    list(input)
}

/// Degraded list: the local actions can only be discovered by pointer hover.
fn list_actions_hover() -> M5ResolvedListView {
    let mut input = list_exact_input("list:product:actions-hover", "recent item");
    input.local_actions_hover_only = true;
    list(input)
}

/// Degraded list: the exact / loaded / all-matching count scopes collapse into one total.
fn list_scope_collapsed() -> M5ResolvedListView {
    let mut input = list_exact_input("list:explorer:scope-collapsed", "files");
    input.count_scopes_distinct = false;
    list(input)
}

/// Degraded list: the count scope cannot be resolved.
fn list_scope_unresolved() -> M5ResolvedListView {
    let mut input = list_exact_input("list:support:scope-unresolved", "records");
    input.count_scope = M5TreeListScopeKind::ScopeUnresolved;
    list(input)
}

/// Degraded list: a loaded / virtualized subset is presented as the exact total.
fn list_loaded_as_exact() -> M5ResolvedListView {
    let mut input = list_exact_input("list:review:loaded-as-exact", "queue item 40");
    input.count_scope = M5TreeListScopeKind::LoadedCount;
    input.loaded_shown_as_exact = true;
    list(input)
}

/// Degraded list: a stale / partial backend is presented as a complete list.
fn list_stale_complete() -> M5ResolvedListView {
    let mut input = list_exact_input("list:data:stale-complete", "provider row");
    input.backend_stale_or_partial = true;
    input.presents_stale_or_partial_as_complete = true;
    list(input)
}

/// Degraded list: no command-backed path to trace the collection scope is reachable.
fn list_trace_missing() -> M5ResolvedListView {
    let mut input = list_exact_input("list:product:trace-missing", "item");
    input.detail_command_available = false;
    list(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5TreeListConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    tree_view_examples: Vec<M5ResolvedTreeView>,
    list_view_examples: Vec<M5ResolvedListView>,
) -> M5TreeListControlsRow {
    M5TreeListControlsRow {
        consumer_surface,
        qualification: M5NavigationContentQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5NavigationContentDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5NavigationContentRequiredLabel::Identity,
            M5NavigationContentRequiredLabel::State,
            M5NavigationContentRequiredLabel::KeyboardRoute,
            M5NavigationContentRequiredLabel::CountAndScope,
            M5NavigationContentRequiredLabel::SelectionAndItemState,
        ],
        accessibility_routes: M5NavigationContentAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5TreeListAnatomyPart::ALL.to_vec(),
        export_fields: M5TreeListExportField::ALL.to_vec(),
        downgrade_triggers,
        tree_view_examples,
        list_view_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_REF,
            M5_TREE_VIEW_SCHEMA_REF,
            M5_LIST_VIEW_SCHEMA_REF,
        ]),
        hides_current_selection_blocked_or_actions_behind_hover_only: false,
        collapses_selection_versus_current_or_count_scopes: false,
        presents_stale_partial_or_lazy_collection_as_complete: false,
        overclaims_drag_reorder_or_cross_surface_continuity: false,
    }
}

fn controls_rows() -> Vec<M5TreeListControlsRow> {
    use M5NavigationContentConsumerSurface as C;
    use M5NavigationContentDowngradeTrigger as D;

    vec![
        base_row(
            C::ExplorerUi,
            "Explorer tree owner",
            "The explorer tree names disclosure, selection-versus-current, per-row item state, and exact/loaded/hidden/outside-scope counts with virtualization honest under deep nesting, degrading when a lazily-unloaded subtree is drawn as an empty leaf or a count scope collapses",
            "evidence:m5-tree-list-explorer-ui:001",
            vec![
                D::DisclosureStateHidden,
                D::SelectionVersusCurrentCollapsed,
                D::CountScopeCollapsed,
                D::ProofStale,
            ],
            vec![
                tree_expanded_clean("tree:explorer:src-main", "main.rs"),
                tree_partial_honest_clean("tree:explorer:lazy-root", "lazy_dir"),
                tree_lazy_leaf(),
                tree_disclosure_unknown(),
                tree_node_unstated(),
                tree_selection_collapsed(),
            ],
            vec![
                list_exact_clean("list:explorer:files", "Cargo.toml"),
                list_scope_collapsed(),
                list_row_unstated(),
            ],
        ),
        base_row(
            C::SearchUi,
            "Search results owner",
            "The search surface reuses the same tree and list row semantics for result collections, keeping exact-versus-loaded-versus-all-matching scopes distinct and degrading when a count scope collapses or the current selection is hover-only",
            "evidence:m5-tree-list-search-ui:001",
            vec![
                D::CountScopeCollapsed,
                D::SelectionVersusCurrentCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                tree_hidden_filter_clean("tree:search:scoped", "matches"),
                tree_scope_collapsed(),
            ],
            vec![
                list_hidden_policy_clean("list:search:hits", "search hit 7"),
                list_current_hover(),
            ],
        ),
        base_row(
            C::ReviewUi,
            "Review-queue owner",
            "The review queue reuses the shared list selection and scope grammar for queued items, keeping blocked state and local actions keyboard-discoverable and degrading when a loaded subset is shown as the exact total or a blocked row hides its state behind hover",
            "evidence:m5-tree-list-review-ui:001",
            vec![
                D::CountScopeCollapsed,
                D::BlockedRowsHiddenBehindEllipsis,
                D::LocalActionsHoverOnly,
                D::ProofStale,
            ],
            vec![
                tree_outside_scope_clean("tree:review:scope", "other_branch"),
                tree_blocked_hover(),
            ],
            vec![
                list_exact_clean("list:review:queue", "review item 1"),
                list_hidden_policy_clean("list:review:restricted", "review item 2"),
                list_loaded_as_exact(),
                list_blocked_hover(),
            ],
        ),
        base_row(
            C::DataUi,
            "Provider / request-data owner",
            "The provider surface reuses the shared tree and list semantics for request/data collections, keeping loaded subsets honest and degrading when the current selection is hover-only or a stale backend is presented as complete",
            "evidence:m5-tree-list-data-ui:001",
            vec![
                D::SelectionVersusCurrentCollapsed,
                D::HierarchyPathUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                tree_expanded_clean("tree:data:providers", "providers"),
                tree_current_hover(),
            ],
            vec![
                list_loaded_clean("list:data:rows", "provider row 3"),
                list_stale_complete(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved tree and list truth, so a faked-complete tree, an unresolved count scope, hover-only local actions, or an overclaimed continuity is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-tree-list-support-export:001",
            vec![
                D::LocalActionsHoverOnly,
                D::HierarchyPathUnstated,
                D::CountScopeCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                tree_partial_honest_clean("tree:support:partial", "partial_root"),
                tree_stale_complete(),
                tree_actions_hover(),
            ],
            vec![
                list_outside_clean("list:support:outside", "out-of-scope record"),
                list_scope_unresolved(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product collection owner",
            "In-product surfaces reuse the same disclosure, selection, and scope grammar a user sees in the explorer and review queue, always offering the command-backed scope detail and degrading honestly when the trace path is missing",
            "evidence:m5-tree-list-product-ui:001",
            vec![
                D::SelectionVersusCurrentCollapsed,
                D::LocalActionsHoverOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                tree_expanded_clean("tree:product:workspace", "workspace"),
                tree_trace_missing(),
            ],
            vec![
                list_exact_clean("list:product:recent", "recent item 1"),
                list_actions_hover(),
                list_trace_missing(),
            ],
        ),
    ]
}

fn governance_review() -> M5TreeListGovernanceReview {
    M5TreeListGovernanceReview {
        tree_names_disclosure_selection_and_counts: true,
        tree_virtualization_honest_never_fakes_complete: true,
        list_names_selection_counts_and_density: true,
        list_loaded_never_shown_as_exact: true,
        selection_versus_current_always_distinct: true,
        current_selection_blocked_and_actions_never_hover_only: true,
        count_scopes_never_collapsed: true,
        drag_reorder_posture_honest_where_allowed: true,
        cross_surface_continuity_never_overclaimed: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5TreeListConsumerProjection {
    M5TreeListConsumerProjection {
        explorer_consumes_tree_disclosure_and_scope_vocabulary: true,
        review_queue_consumes_list_selection_and_scope_vocabulary: true,
        search_and_provider_consume_shared_row_semantics: true,
        collection_facts_trace_to_single_component_contract: true,
        support_export_reads_single_collection_source: true,
    }
}

fn proof_freshness() -> M5TreeListProofFreshness {
    M5TreeListProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TreeListReleasePosture {
    M5TreeListReleasePosture {
        proof_packet_ref: M5_TREE_VIEW_LIST_VIEW_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_TREE_VIEW_LIST_VIEW_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TREE_VIEW_LIST_VIEW_CONTROLS_SCHEMA_REF,
        M5_TREE_VIEW_LIST_VIEW_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TREE_VIEW_SCHEMA_REF,
        M5_LIST_VIEW_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 tree-view / list-view controls packet.
pub fn seeded_m5_tree_view_list_view_controls() -> M5TreeListControlsPacket {
    M5TreeListControlsPacket::new(M5TreeListControlsPacketInput {
        packet_id: M5_TREE_VIEW_LIST_VIEW_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 tree-view and list-view controls with virtualization, keyboard-complete disclosure, selection-versus-current distinction, capped inline-action budgets, and exact/loaded/hidden/outside-scope count truth aligned across explorer, search, review-queue, provider, help, and support surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5TreeListVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the explorer-UI row is held at Beta pending disclosure parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed() -> M5TreeListControlsPacket
{
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.packet_id = "m5-tree-view-list-view-controls:explorer-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::ExplorerUi)
        .expect("explorer-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review-UI row is narrowed to Preview pending list-queue parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed(
) -> M5TreeListControlsPacket {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.packet_id = "m5-tree-view-list-view-controls:review-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::ReviewUi)
        .expect("review-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Preview;
    packet
}
