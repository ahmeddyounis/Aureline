//! Canonical seed builders for the frozen M5 navigation-content component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical navigation-content component matrix.
pub const M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-navigation-content-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5NavigationContentRequiredLabel> {
    M5NavigationContentRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(
    extra: &[M5NavigationContentRequiredLabel],
) -> Vec<M5NavigationContentRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5NavigationContentComponentFamily,
    qualification: M5NavigationContentQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5NavigationContentComponentRow {
    M5NavigationContentComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5NavigationContentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5NavigationContentDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: M5NavigationContentDisposition::ALL.to_vec(),
        active_context_states: vec![],
        hierarchy_path_states: vec![],
        disclosure_states: vec![],
        selection_states: vec![],
        count_scopes: vec![],
        item_state_flags: vec![],
        density_variants: vec![],
        local_action_budgets: vec![],
        degraded_reasons: M5NavigationContentDegradedReason::ALL.to_vec(),
        accessibility_routes: M5NavigationContentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5NavigationContentConsumerSurface::SupportExport,
            M5NavigationContentConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5NavigationContentDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        tabs_masquerade_as_top_level_workflow_navigation: false,
        hides_counts_or_blocked_rows_behind_ambiguous_ellipsis: false,
        makes_tree_list_or_table_actions_hover_only: false,
        panel_header_becomes_cluttered_secondary_toolbar: false,
        collapses_exact_loaded_and_all_matching_scopes_into_one_total: false,
    }
}

fn component_rows() -> Vec<M5NavigationContentComponentRow> {
    use M5ActiveContextState as AC;
    use M5CountScope as CS;
    use M5DensityVariant as DV;
    use M5DisclosureState as DI;
    use M5HierarchyPathState as HP;
    use M5ItemStateFlag as IS;
    use M5LocalActionBudget as LA;
    use M5NavigationContentComponentFamily as F;
    use M5NavigationContentConsumerSurface as C;
    use M5NavigationContentDisposition as BD;
    use M5NavigationContentDowngradeTrigger as D;
    use M5NavigationContentQualificationClass as Q;
    use M5NavigationContentRequiredLabel as L;
    use M5SelectionState as SE;

    let mut rows = Vec::new();

    // 1. Tab strip.
    let mut row = base_row(
        F::TabStrip,
        Q::Stable,
        "Shell navigation owner",
        "One tab-strip model naming the active context (current, pinned, preview, or background), the per-tab item state (pinned, preview, modified, read-only, blocked), and the overflow budget, so a background or preview tab never reads as the active context and a tab set never masquerades as top-level workflow navigation",
        "evidence:m5-tab-strip-parity:001",
        &[M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_TAB_STRIP_SCHEMA_REF],
    );
    row.active_context_states = AC::ALL.to_vec();
    row.item_state_flags = vec![
        IS::Pinned,
        IS::Preview,
        IS::Modified,
        IS::ReadOnly,
        IS::Blocked,
    ];
    row.local_action_budgets = vec![
        LA::WithinBudget,
        LA::PrimaryPlusOverflow,
        LA::OverflowedMenu,
    ];
    row.dispositions = vec![
        BD::Preview,
        BD::Pinned,
        BD::Modified,
        BD::ReadOnly,
        BD::Blocked,
        BD::OverflowedLocalAction,
    ];
    row.required_labels = labels_with(&[L::ActiveContextAndHierarchy, L::SelectionAndItemState]);
    row.consumer_surfaces = vec![C::ShellUi, C::ReviewUi, C::SupportExport, C::ProductUi];
    row.downgrade_triggers = vec![
        D::TabsMasqueradeAsWorkflowNav,
        D::ActiveContextUnstated,
        D::LocalActionsHoverOnly,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Breadcrumbs.
    let mut row = base_row(
        F::Breadcrumbs,
        Q::Stable,
        "Explorer navigation owner",
        "One breadcrumbs model naming the hierarchy / path to the current object — full path, root-relative, truncated-middle, stale, or partial — so a truncated, stale, or partial hierarchy is never presented as a complete path",
        "evidence:m5-breadcrumbs-parity:001",
        &[M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_BREADCRUMBS_SCHEMA_REF],
    );
    row.hierarchy_path_states = HP::ALL.to_vec();
    row.dispositions = vec![
        BD::StaleOrPartialHierarchy,
        BD::OverflowedLocalAction,
        BD::ReadOnly,
    ];
    row.required_labels = labels_with(&[L::ActiveContextAndHierarchy]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ExplorerUi,
        C::HelpUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HierarchyPathUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Tree view.
    let mut row = base_row(
        F::TreeView,
        Q::Stable,
        "Explorer navigation owner",
        "One tree-view model naming hierarchy, disclosure state (expanded, collapsed, partially expanded, leaf, or lazily unloaded), selection versus the current item, item state, exact / loaded / all-matching / hidden counts, density, and a bounded local-action budget, so a lazily-unloaded subtree is never presented as an empty leaf and tree actions are never hover-only",
        "evidence:m5-tree-view-parity:001",
        &[M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_TREE_VIEW_SCHEMA_REF],
    );
    row.hierarchy_path_states = vec![
        HP::FullPathShown,
        HP::RootRelative,
        HP::PartialHierarchy,
        HP::StaleHierarchy,
    ];
    row.disclosure_states = DI::ALL.to_vec();
    row.selection_states = vec![
        SE::SingleSelected,
        SE::MultiSelected,
        SE::CurrentNotSelected,
        SE::SelectedAndCurrent,
        SE::NoneSelected,
    ];
    row.count_scopes = vec![
        CS::ExactCount,
        CS::LoadedCount,
        CS::AllMatchingCount,
        CS::HiddenByFilterCount,
    ];
    row.item_state_flags = vec![IS::Pinned, IS::Modified, IS::ReadOnly, IS::Blocked];
    row.density_variants = vec![DV::Comfortable, DV::Compact, DV::Dense];
    row.local_action_budgets = vec![
        LA::WithinBudget,
        LA::PrimaryPlusOverflow,
        LA::OverflowedMenu,
    ];
    row.dispositions = BD::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::ActiveContextAndHierarchy,
        L::CountAndScope,
        L::SelectionAndItemState,
    ]);
    row.consumer_surfaces = vec![
        C::ExplorerUi,
        C::ShellUi,
        C::ReviewUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HierarchyPathUnstated,
        D::DisclosureStateHidden,
        D::SelectionVersusCurrentCollapsed,
        D::LocalActionsHoverOnly,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. List view.
    let mut row = base_row(
        F::ListView,
        Q::Stable,
        "Collection surface owner",
        "One list-view model naming selection versus the current item, per-row item state, the exact / loaded / all-matching / hidden-by-filter / hidden-by-policy counts, density, and a bounded local-action budget, so exact, loaded, and all-matching scopes never collapse into one vague total and blocked rows never hide behind an ambiguous ellipsis",
        "evidence:m5-list-view-parity:001",
        &[M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_LIST_VIEW_SCHEMA_REF],
    );
    row.selection_states = SE::ALL.to_vec();
    row.count_scopes = CS::ALL.to_vec();
    row.item_state_flags = vec![IS::Modified, IS::ReadOnly, IS::Blocked, IS::Preview];
    row.density_variants = vec![
        DV::Comfortable,
        DV::Compact,
        DV::Dense,
        DV::CondensedOverflow,
    ];
    row.local_action_budgets = vec![
        LA::WithinBudget,
        LA::PrimaryPlusOverflow,
        LA::OverflowedMenu,
        LA::AllOverflowed,
    ];
    row.dispositions = vec![
        BD::ExactCount,
        BD::LoadedCount,
        BD::AllMatchingCount,
        BD::HiddenByFilter,
        BD::HiddenByPolicy,
        BD::Blocked,
        BD::ReadOnly,
        BD::Modified,
    ];
    row.required_labels = labels_with(&[L::CountAndScope, L::SelectionAndItemState]);
    row.consumer_surfaces = vec![
        C::SearchUi,
        C::DataUi,
        C::ReviewUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CountScopeCollapsed,
        D::BlockedRowsHiddenBehindEllipsis,
        D::HiddenByPolicyCountOmitted,
        D::LocalActionsHoverOnly,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Table / grid.
    let mut row = base_row(
        F::TableGrid,
        Q::Stable,
        "Data surface owner",
        "One table/grid model naming selection, the exact / loaded / all-matching / hidden-by-policy counts, every density variant, per-cell item state, and a bounded local-action budget across a dense structure, so a condensed or overflowed layout is never mistaken for a complete comfortable one and counts stay scoped",
        "evidence:m5-table-grid-parity:001",
        &[M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_TABLE_GRID_SCHEMA_REF],
    );
    row.selection_states = vec![
        SE::SingleSelected,
        SE::MultiSelected,
        SE::NoneSelected,
        SE::SelectionUnknown,
    ];
    row.count_scopes = vec![
        CS::ExactCount,
        CS::LoadedCount,
        CS::AllMatchingCount,
        CS::HiddenByPolicyCount,
        CS::CountUnresolved,
    ];
    row.item_state_flags = IS::ALL.to_vec();
    row.density_variants = DV::ALL.to_vec();
    row.local_action_budgets = vec![
        LA::WithinBudget,
        LA::PrimaryPlusOverflow,
        LA::OverflowedMenu,
    ];
    row.dispositions = vec![
        BD::ExactCount,
        BD::LoadedCount,
        BD::AllMatchingCount,
        BD::HiddenByFilter,
        BD::HiddenByPolicy,
        BD::Blocked,
        BD::OverflowedLocalAction,
    ];
    row.required_labels = labels_with(&[L::CountAndScope, L::SelectionAndItemState]);
    row.consumer_surfaces = vec![
        C::DataUi,
        C::SearchUi,
        C::ReviewUi,
        C::AiContextUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CountScopeCollapsed,
        D::BlockedRowsHiddenBehindEllipsis,
        D::SelectionVersusCurrentCollapsed,
        D::LocalActionsHoverOnly,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Panel header.
    let mut row = base_row(
        F::PanelHeader,
        Q::Stable,
        "Shell navigation owner",
        "One panel-header model naming the active context and a bounded local-action budget (within budget, primary-plus-overflow, or fully overflowed), so a panel header never becomes a cluttered secondary toolbar and an overflowed action is never silently dropped",
        "evidence:m5-panel-header-parity:001",
        &[M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF, M5_PANEL_HEADER_SCHEMA_REF],
    );
    row.active_context_states = vec![
        AC::ActiveCurrent,
        AC::ActivePinned,
        AC::BackgroundModified,
        AC::ContextUnresolved,
    ];
    row.local_action_budgets = LA::ALL.to_vec();
    row.dispositions = vec![
        BD::Modified,
        BD::ReadOnly,
        BD::OverflowedLocalAction,
        BD::Pinned,
    ];
    row.required_labels = labels_with(&[L::ActiveContextAndHierarchy]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::ReviewUi,
        C::DataUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ActiveContextUnstated,
        D::PanelHeaderOverloaded,
        D::LocalActionsHoverOnly,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5NavigationContentGovernanceReview {
    M5NavigationContentGovernanceReview {
        tab_strip_shows_active_context_and_overflow: true,
        breadcrumbs_show_full_or_truncated_hierarchy: true,
        tree_view_shows_disclosure_and_selection: true,
        list_view_shows_counts_and_hidden_scopes: true,
        table_grid_shows_counts_and_density: true,
        panel_header_shows_context_and_bounded_actions: true,
        tabs_never_masquerade_as_workflow_navigation: true,
        counts_never_collapsed_into_one_total: true,
        blocked_rows_never_hidden_behind_ellipsis: true,
        hidden_by_filter_and_policy_always_distinct: true,
        local_actions_never_hover_only: true,
        panel_headers_never_become_secondary_toolbars: true,
        stale_or_partial_hierarchy_always_named: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_navigation_vocabulary: true,
    }
}

fn consumer_projection() -> M5NavigationContentConsumerProjection {
    M5NavigationContentConsumerProjection {
        shell_surfaces_consume_active_context_vocabulary: true,
        explorer_consumes_hierarchy_and_disclosure_vocabulary: true,
        search_consumes_count_scope_vocabulary: true,
        review_consumes_selection_and_item_state_vocabulary: true,
        help_consumes_navigation_vocabulary: true,
        support_export_reads_single_navigation_source: true,
    }
}

fn proof_freshness() -> M5NavigationContentProofFreshness {
    M5NavigationContentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5NavigationContentReleasePosture {
    M5NavigationContentReleasePosture {
        proof_packet_ref: M5_NAVIGATION_CONTENT_COMPONENT_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_NAVIGATION_CONTENT_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_TAB_STRIP_SCHEMA_REF,
        M5_BREADCRUMBS_SCHEMA_REF,
        M5_TREE_VIEW_SCHEMA_REF,
        M5_LIST_VIEW_SCHEMA_REF,
        M5_TABLE_GRID_SCHEMA_REF,
        M5_PANEL_HEADER_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 navigation-content component matrix packet.
pub fn seeded_m5_navigation_content_component_matrix() -> M5NavigationContentComponentMatrixPacket {
    M5NavigationContentComponentMatrixPacket::new(M5NavigationContentComponentMatrixPacketInput {
        packet_id: M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 tab-strip, breadcrumbs, tree-view, list-view, table/grid, and panel-header component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5NavigationContentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the table/grid is held at Beta because density and count-scope parity
/// round-trips are not yet proven across every deployment line; every component stays visible.
pub fn seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed(
) -> M5NavigationContentComponentMatrixPacket {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.packet_id = "m5-navigation-content-components:table-grid-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NavigationContentComponentFamily::TableGrid)
        .expect("table-grid row present");
    row.qualification = M5NavigationContentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the tree view is narrowed to Preview pending disclosure and lazy-load parity
/// across every deployment line; every component stays visible.
pub fn seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed(
) -> M5NavigationContentComponentMatrixPacket {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.packet_id = "m5-navigation-content-components:tree-view-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NavigationContentComponentFamily::TreeView)
        .expect("tree-view row present");
    row.qualification = M5NavigationContentQualificationClass::Preview;
    packet
}
