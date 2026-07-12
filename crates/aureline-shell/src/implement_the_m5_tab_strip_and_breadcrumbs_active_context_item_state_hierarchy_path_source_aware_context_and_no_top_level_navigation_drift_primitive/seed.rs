//! Canonical seed builders for the M5 tab-strip / breadcrumbs controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean tab
//! strips and breadcrumb trails are built so the shared active-context / hierarchy grammar is proven
//! across surfaces without any surface-local badge or top-level-navigation drift.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_TAB_STRIP_BREADCRUMBS_CONTROLS_PACKET_ID: &str =
    "m5-tab-strip-breadcrumbs-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn tab(input: M5TabStripResolutionInput) -> M5ResolvedTabStrip {
    resolve_tab_strip(input).expect("seed tab strip input resolves")
}

fn trail(input: M5BreadcrumbsResolutionInput) -> M5ResolvedBreadcrumbs {
    resolve_breadcrumbs(input).expect("seed breadcrumbs input resolves")
}

// -- Clean tab examples (shared item-state grammar across surfaces) ----------------------------

fn clean_tab_base(
    strip_id: &str,
    label: &str,
    context: M5ActiveContextState,
    item_state: M5TabItemState,
    budget: M5LocalActionBudget,
) -> M5TabStripResolutionInput {
    M5TabStripResolutionInput {
        strip_id: strip_id.to_owned(),
        active_context_label: label.to_owned(),
        active_context: context,
        item_state,
        item_state_stated: true,
        local_action_budget: budget,
        has_blocked_tab: false,
        blocked_tab_stated: true,
        reads_as_top_level_workflow_navigation: false,
        invents_surface_local_badge: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean pinned tab in the shell workspace.
fn tab_pinned_clean() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:shell:main-rs",
        "main.rs",
        M5ActiveContextState::ActivePinned,
        M5TabItemState::Pinned,
        M5LocalActionBudget::WithinBudget,
    ))
}

/// Clean modified tab (active, unsaved edits).
fn tab_modified_clean() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:shell:lib-rs",
        "lib.rs",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Modified,
        M5LocalActionBudget::PrimaryPlusOverflow,
    ))
}

/// Clean read-only tab.
fn tab_read_only_clean() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:explorer:cargo-lock",
        "Cargo.lock",
        M5ActiveContextState::BackgroundOpen,
        M5TabItemState::ReadOnly,
        M5LocalActionBudget::NoLocalActions,
    ))
}

/// Clean shared / co-edited tab.
fn tab_shared_clean() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:review:design-md",
        "design.md",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Shared,
        M5LocalActionBudget::WithinBudget,
    ))
}

/// Clean reopened / recovered tab.
fn tab_reopened_clean() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:shell:notes-md",
        "notes.md",
        M5ActiveContextState::BackgroundModified,
        M5TabItemState::Reopened,
        M5LocalActionBudget::WithinBudget,
    ))
}

/// Clean preview tab (single-click, not yet pinned).
fn tab_preview_clean() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:search:result-preview",
        "search_result.rs",
        M5ActiveContextState::ActivePreview,
        M5TabItemState::Preview,
        M5LocalActionBudget::WithinBudget,
    ))
}

// -- Degraded tab examples ---------------------------------------------------------------------

/// Degraded tab: the active context label is unstated.
fn tab_active_context_unstated() -> M5ResolvedTabStrip {
    let mut input = clean_tab_base(
        "tab:shell:no-label",
        "   ",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Pinned,
        M5LocalActionBudget::WithinBudget,
    );
    input.active_context_label = "   ".to_owned();
    tab(input)
}

/// Degraded tab: the tab strip reads as top-level workflow navigation.
fn tab_masquerade() -> M5ResolvedTabStrip {
    let mut input = clean_tab_base(
        "tab:shell:masquerade",
        "Build",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Pinned,
        M5LocalActionBudget::WithinBudget,
    );
    input.reads_as_top_level_workflow_navigation = true;
    tab(input)
}

/// Degraded tab: a surface-local badge is invented for a shared context.
fn tab_badge_invented() -> M5ResolvedTabStrip {
    let mut input = clean_tab_base(
        "tab:review:badge",
        "shared.rs",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Shared,
        M5LocalActionBudget::WithinBudget,
    );
    input.invents_surface_local_badge = true;
    tab(input)
}

/// Degraded tab: the item state cannot be resolved.
fn tab_item_state_unknown() -> M5ResolvedTabStrip {
    tab(clean_tab_base(
        "tab:shell:unknown-state",
        "unknown.rs",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::StateUnknown,
        M5LocalActionBudget::WithinBudget,
    ))
}

/// Degraded tab: the item state is encoded by color / hover alone.
fn tab_color_only() -> M5ResolvedTabStrip {
    let mut input = clean_tab_base(
        "tab:shell:color-only",
        "colored.rs",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Modified,
        M5LocalActionBudget::WithinBudget,
    );
    input.item_state_stated = false;
    tab(input)
}

/// Degraded tab: a blocked tab is hidden behind an ambiguous ellipsis.
fn tab_blocked_hidden() -> M5ResolvedTabStrip {
    let mut input = clean_tab_base(
        "tab:shell:blocked-hidden",
        "blocked.rs",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Blocked,
        M5LocalActionBudget::WithinBudget,
    );
    input.blocked_tab_stated = false;
    tab(input)
}

/// Degraded tab: no command-backed path to trace the active context is reachable.
fn tab_trace_missing() -> M5ResolvedTabStrip {
    let mut input = clean_tab_base(
        "tab:product:trace-missing",
        "traceless.rs",
        M5ActiveContextState::ActiveCurrent,
        M5TabItemState::Pinned,
        M5LocalActionBudget::WithinBudget,
    );
    input.detail_command_available = false;
    tab(input)
}

// -- Clean breadcrumb examples -----------------------------------------------------------------

fn clean_trail_base(
    trail_id: &str,
    leaf: &str,
    ancestry: M5BreadcrumbAncestryKind,
    path: M5HierarchyPathState,
) -> M5BreadcrumbsResolutionInput {
    M5BreadcrumbsResolutionInput {
        trail_id: trail_id.to_owned(),
        leaf_label: leaf.to_owned(),
        ancestry_kind: ancestry,
        hierarchy_path: path,
        path_explicit_in_compact_and_expanded: true,
        collapses_missing_scope_into_ellipsis: false,
        presents_partial_or_stale_as_complete: false,
        reads_as_top_level_workflow_navigation: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean file-path breadcrumb showing the full path from root.
fn trail_file_path_clean() -> M5ResolvedBreadcrumbs {
    trail(clean_trail_base(
        "trail:explorer:main-rs",
        "main.rs",
        M5BreadcrumbAncestryKind::FilePath,
        M5HierarchyPathState::FullPathShown,
    ))
}

/// Clean symbol-ancestry breadcrumb (module → type → member), root-relative.
fn trail_symbol_clean() -> M5ResolvedBreadcrumbs {
    trail(clean_trail_base(
        "trail:review:resolve-fn",
        "resolve_tab_strip",
        M5BreadcrumbAncestryKind::SymbolAncestry,
        M5HierarchyPathState::RootRelative,
    ))
}

/// Clean search-scope breadcrumb with an honestly truncated middle.
fn trail_search_scope_clean() -> M5ResolvedBreadcrumbs {
    trail(clean_trail_base(
        "trail:search:scope",
        "matches in crates/aureline-shell",
        M5BreadcrumbAncestryKind::SearchScope,
        M5HierarchyPathState::TruncatedMiddle,
    ))
}

/// Clean breadcrumb that shows a partial hierarchy honestly (never presented as complete).
fn trail_partial_honest_clean() -> M5ResolvedBreadcrumbs {
    trail(clean_trail_base(
        "trail:explorer:partial-honest",
        "lazy_child.rs",
        M5BreadcrumbAncestryKind::LogicalRoot,
        M5HierarchyPathState::PartialHierarchy,
    ))
}

// -- Degraded breadcrumb examples --------------------------------------------------------------

/// Degraded breadcrumb: the leaf / current-object identity is unstated.
fn trail_leaf_unstated() -> M5ResolvedBreadcrumbs {
    let mut input = clean_trail_base(
        "trail:explorer:no-leaf",
        "  ",
        M5BreadcrumbAncestryKind::FilePath,
        M5HierarchyPathState::FullPathShown,
    );
    input.leaf_label = "  ".to_owned();
    trail(input)
}

/// Degraded breadcrumb: the ancestry kind cannot be resolved.
fn trail_ancestry_unknown() -> M5ResolvedBreadcrumbs {
    trail(clean_trail_base(
        "trail:explorer:ancestry-unknown",
        "orphan.rs",
        M5BreadcrumbAncestryKind::AncestryUnknown,
        M5HierarchyPathState::FullPathShown,
    ))
}

/// Degraded breadcrumb: the trail reads as top-level workflow navigation.
fn trail_masquerade() -> M5ResolvedBreadcrumbs {
    let mut input = clean_trail_base(
        "trail:shell:masquerade",
        "Settings",
        M5BreadcrumbAncestryKind::LogicalRoot,
        M5HierarchyPathState::FullPathShown,
    );
    input.reads_as_top_level_workflow_navigation = true;
    trail(input)
}

/// Degraded breadcrumb: missing scope is collapsed into an ambiguous ellipsis.
fn trail_ellipsis_collapse() -> M5ResolvedBreadcrumbs {
    let mut input = clean_trail_base(
        "trail:explorer:ellipsis",
        "deep_child.rs",
        M5BreadcrumbAncestryKind::FilePath,
        M5HierarchyPathState::TruncatedMiddle,
    );
    input.collapses_missing_scope_into_ellipsis = true;
    trail(input)
}

/// Degraded breadcrumb: a partial / stale hierarchy is presented as a complete path.
fn trail_partial_shown_complete() -> M5ResolvedBreadcrumbs {
    let mut input = clean_trail_base(
        "trail:explorer:partial-complete",
        "stale_child.rs",
        M5BreadcrumbAncestryKind::FilePath,
        M5HierarchyPathState::StaleHierarchy,
    );
    input.presents_partial_or_stale_as_complete = true;
    trail(input)
}

/// Degraded breadcrumb: the path is not explicit across compact and expanded views.
fn trail_not_explicit() -> M5ResolvedBreadcrumbs {
    let mut input = clean_trail_base(
        "trail:search:not-explicit",
        "hidden_scope.rs",
        M5BreadcrumbAncestryKind::SearchScope,
        M5HierarchyPathState::RootRelative,
    );
    input.path_explicit_in_compact_and_expanded = false;
    trail(input)
}

/// Degraded breadcrumb: no command-backed path to trace the ancestry is reachable.
fn trail_trace_missing() -> M5ResolvedBreadcrumbs {
    let mut input = clean_trail_base(
        "trail:product:trace-missing",
        "traceless.rs",
        M5BreadcrumbAncestryKind::FilePath,
        M5HierarchyPathState::FullPathShown,
    );
    input.detail_command_available = false;
    trail(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5TabBreadcrumbsConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    tab_strip_examples: Vec<M5ResolvedTabStrip>,
    breadcrumbs_examples: Vec<M5ResolvedBreadcrumbs>,
) -> M5TabBreadcrumbsControlsRow {
    M5TabBreadcrumbsControlsRow {
        consumer_surface,
        qualification: M5NavigationContentQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5NavigationContentDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5NavigationContentRequiredLabel::Identity,
            M5NavigationContentRequiredLabel::State,
            M5NavigationContentRequiredLabel::KeyboardRoute,
            M5NavigationContentRequiredLabel::ActiveContextAndHierarchy,
            M5NavigationContentRequiredLabel::SelectionAndItemState,
        ],
        accessibility_routes: M5NavigationContentAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5TabBreadcrumbsAnatomyPart::ALL.to_vec(),
        export_fields: M5TabBreadcrumbsExportField::ALL.to_vec(),
        downgrade_triggers,
        tab_strip_examples,
        breadcrumbs_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_REF,
            M5_TAB_STRIP_SCHEMA_REF,
            M5_BREADCRUMBS_SCHEMA_REF,
        ]),
        tabs_masquerade_as_top_level_workflow_navigation: false,
        breadcrumbs_masquerade_as_top_level_workflow_navigation: false,
        invents_surface_local_badges_for_shared_context: false,
        collapses_missing_scope_or_hides_blocked_behind_ellipsis: false,
    }
}

fn controls_rows() -> Vec<M5TabBreadcrumbsControlsRow> {
    use M5NavigationContentConsumerSurface as C;
    use M5NavigationContentDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell workspace owner",
            "The shell tab strip names the active context and per-tab pinned/preview/modified/read-only/blocked/shared/reopened state with no-color-only semantics, and degrades honestly when the active context is unstated or a blocked tab hides behind an ambiguous ellipsis",
            "evidence:m5-tab-strip-breadcrumbs-shell-ui:001",
            vec![
                D::ActiveContextUnstated,
                D::TabsMasqueradeAsWorkflowNav,
                D::BlockedRowsHiddenBehindEllipsis,
                D::ProofStale,
            ],
            vec![
                tab_pinned_clean(),
                tab_modified_clean(),
                tab_active_context_unstated(),
                tab_blocked_hidden(),
            ],
            vec![trail_file_path_clean(), trail_masquerade()],
        ),
        base_row(
            C::ExplorerUi,
            "Explorer tree owner",
            "The explorer breadcrumb trail names its file-path or logical-root ancestry and stays explicit across compact and expanded views, showing a partial hierarchy honestly and degrading when missing scope collapses into an ambiguous ellipsis",
            "evidence:m5-tab-strip-breadcrumbs-explorer-ui:001",
            vec![
                D::HierarchyPathUnstated,
                D::BlockedRowsHiddenBehindEllipsis,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![tab_read_only_clean(), tab_reopened_clean()],
            vec![
                trail_file_path_clean(),
                trail_partial_honest_clean(),
                trail_ellipsis_collapse(),
                trail_leaf_unstated(),
            ],
        ),
        base_row(
            C::SearchUi,
            "Search results owner",
            "The search surface reuses the same tab and breadcrumb grammar for preview contexts and search-scope ancestry, and degrades honestly when a path is not explicit across views or an item state is encoded by color alone",
            "evidence:m5-tab-strip-breadcrumbs-search-ui:001",
            vec![
                D::HierarchyPathUnstated,
                D::GenericChromeWordingUsed,
                D::ActiveContextUnstated,
                D::ProofStale,
            ],
            vec![tab_preview_clean(), tab_color_only(), tab_item_state_unknown()],
            vec![trail_search_scope_clean(), trail_not_explicit()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved tab and breadcrumb truth, so a masquerading tab, an invented surface-local badge, a partial-hierarchy-shown-complete trail, or an unresolved ancestry is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-tab-strip-breadcrumbs-support-export:001",
            vec![
                D::TabsMasqueradeAsWorkflowNav,
                D::BlockedRowsHiddenBehindEllipsis,
                D::HierarchyPathUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![tab_shared_clean(), tab_masquerade(), tab_badge_invented()],
            vec![
                trail_symbol_clean(),
                trail_partial_shown_complete(),
                trail_ancestry_unknown(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product navigation owner",
            "In-product surfaces reuse the same active-context and ancestry grammar a user sees in the shell, always offering the command-backed detail path and degrading honestly when the trace path is missing",
            "evidence:m5-tab-strip-breadcrumbs-product-ui:001",
            vec![
                D::ActiveContextUnstated,
                D::HierarchyPathUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![tab_pinned_clean(), tab_trace_missing()],
            vec![trail_file_path_clean(), trail_trace_missing()],
        ),
    ]
}

fn governance_review() -> M5TabBreadcrumbsGovernanceReview {
    M5TabBreadcrumbsGovernanceReview {
        tab_names_active_context_and_item_state: true,
        tab_item_state_no_color_only: true,
        breadcrumbs_name_ancestry_and_path: true,
        breadcrumbs_explicit_across_views: true,
        tabs_never_masquerade_as_workflow_navigation: true,
        breadcrumbs_never_masquerade_as_workflow_navigation: true,
        no_surface_local_badges_for_shared_context: true,
        missing_scope_and_blocked_never_hidden_behind_ellipsis: true,
        partial_or_stale_hierarchy_never_shown_as_complete: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5TabBreadcrumbsConsumerProjection {
    M5TabBreadcrumbsConsumerProjection {
        shell_surfaces_consume_active_context_vocabulary: true,
        explorer_consumes_hierarchy_and_ancestry_vocabulary: true,
        search_consumes_active_context_and_hierarchy_vocabulary: true,
        navigation_facts_trace_to_single_component_contract: true,
        support_export_reads_single_navigation_source: true,
    }
}

fn proof_freshness() -> M5TabBreadcrumbsProofFreshness {
    M5TabBreadcrumbsProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TabBreadcrumbsReleasePosture {
    M5TabBreadcrumbsReleasePosture {
        proof_packet_ref: M5_TAB_STRIP_BREADCRUMBS_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_TAB_STRIP_BREADCRUMBS_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TAB_STRIP_BREADCRUMBS_CONTROLS_SCHEMA_REF,
        M5_TAB_STRIP_BREADCRUMBS_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TAB_STRIP_SCHEMA_REF,
        M5_BREADCRUMBS_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 tab-strip / breadcrumbs controls packet.
pub fn seeded_m5_tab_strip_breadcrumbs_controls() -> M5TabBreadcrumbsControlsPacket {
    M5TabBreadcrumbsControlsPacket::new(M5TabBreadcrumbsControlsPacketInput {
        packet_id: M5_TAB_STRIP_BREADCRUMBS_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 tab-strip and breadcrumbs controls with active-context, per-tab item state, source-aware hierarchy ancestry, and no top-level navigation drift aligned across shell, explorer, search, review, help, and support surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5TabBreadcrumbsVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending active-context parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed(
) -> M5TabBreadcrumbsControlsPacket {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.packet_id = "m5-tab-strip-breadcrumbs-controls:shell-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the search-UI row is narrowed to Preview pending breadcrumb parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed(
) -> M5TabBreadcrumbsControlsPacket {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.packet_id = "m5-tab-strip-breadcrumbs-controls:search-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::SearchUi)
        .expect("search-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Preview;
    packet
}
