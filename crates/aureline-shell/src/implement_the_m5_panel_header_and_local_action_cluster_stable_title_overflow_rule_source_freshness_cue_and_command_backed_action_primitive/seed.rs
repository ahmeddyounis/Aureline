//! Canonical seed builders for the M5 panel-header and local-action-cluster controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean headers
//! and clusters are built so the shared title / active-context / source-freshness / action / overflow
//! grammar is proven across surfaces without any hover-only discovery, hidden freshness cue,
//! overstated readiness, persistent clutter, dropped overflow action, or surface-swapping compaction.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_PACKET_ID: &str =
    "m5-panel-header-local-action-cluster-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn header(input: M5PanelHeaderResolutionInput) -> M5ResolvedPanelHeader {
    resolve_panel_header(input).expect("seed panel header input resolves")
}

fn cluster(input: M5LocalActionClusterResolutionInput) -> M5ResolvedLocalActionCluster {
    resolve_local_action_cluster(input).expect("seed local action cluster input resolves")
}

// -- Clean panel-header examples (shared title / context / freshness grammar) -------------------

fn clean_header_base(
    header_id: &str,
    label: &str,
    active_context: M5ActiveContextState,
    source_freshness: M5PanelSourceFreshness,
) -> M5PanelHeaderResolutionInput {
    M5PanelHeaderResolutionInput {
        header_id: header_id.to_owned(),
        header_label: label.to_owned(),
        title_slot_stable: true,
        active_context,
        background_context_shown_as_active: false,
        source_freshness,
        // A qualified pane always shows its cue; a current pane needs none.
        freshness_cue_shown: source_freshness.is_qualified(),
        overstates_readiness: false,
        references_canonical_model: true,
        re_encodes_canonical_counts_locally: false,
        refresh_command_available: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean header naming a current, first-party pane in the active, current context.
fn header_current_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActiveCurrent,
        M5PanelSourceFreshness::Current,
    ))
}

/// Clean header labelling a cached pane at the boundary under a pinned active context.
fn header_cached_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActivePinned,
        M5PanelSourceFreshness::Cached,
    ))
}

/// Clean header labelling a remote-owned pane at the boundary.
fn header_remote_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActiveCurrent,
        M5PanelSourceFreshness::Remote,
    ))
}

/// Clean header honestly naming a background-open provider-owned pane (never presented as active).
fn header_provider_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::BackgroundOpen,
        M5PanelSourceFreshness::ProviderOwned,
    ))
}

/// Clean header labelling a partially-loaded pane at the boundary.
fn header_partial_clean(header_id: &str, label: &str) -> M5ResolvedPanelHeader {
    header(clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActivePreview,
        M5PanelSourceFreshness::Partial,
    ))
}

// -- Degraded panel-header examples ------------------------------------------------------------

fn header_current_input(header_id: &str, label: &str) -> M5PanelHeaderResolutionInput {
    clean_header_base(
        header_id,
        label,
        M5ActiveContextState::ActiveCurrent,
        M5PanelSourceFreshness::Current,
    )
}

/// Degraded header: the header title is unstated.
fn header_title_unstated() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:product:no-title", "  ");
    input.header_label = "  ".to_owned();
    header(input)
}

/// Degraded header: the title slot is not stable.
fn header_title_unstable() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:shell:title-unstable", "Panel");
    input.title_slot_stable = false;
    header(input)
}

/// Degraded header: the active context cannot be resolved.
fn header_context_unresolved() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:search:context-unresolved", "Results");
    input.active_context = M5ActiveContextState::ContextUnresolved;
    header(input)
}

/// Degraded header: a background context is presented as the active one.
fn header_background_as_active() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:shell:background-as-active", "Background panel");
    input.active_context = M5ActiveContextState::BackgroundOpen;
    input.background_context_shown_as_active = true;
    header(input)
}

/// Degraded header: the source / freshness cannot be resolved.
fn header_freshness_unresolved() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:support:freshness-unresolved", "Support export");
    input.source_freshness = M5PanelSourceFreshness::FreshnessUnknown;
    header(input)
}

/// Degraded header: a cached pane hides its freshness cue at the boundary.
fn header_cue_missing() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:data:cue-missing", "Providers");
    input.source_freshness = M5PanelSourceFreshness::Cached;
    input.freshness_cue_shown = false;
    header(input)
}

/// Degraded header: a stale pane is presented as current / ready.
fn header_readiness_overstated() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:review:readiness-overstated", "Review");
    input.source_freshness = M5PanelSourceFreshness::Stale;
    input.freshness_cue_shown = true;
    input.overstates_readiness = true;
    header(input)
}

/// Degraded header: the header re-encodes the canonical count / selection model locally.
fn header_re_encodes() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:data:re-encodes", "Provider rows");
    input.re_encodes_canonical_counts_locally = true;
    header(input)
}

/// Degraded header: no command-backed refresh affordance is reachable.
fn header_refresh_missing() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:support:refresh-missing", "Diagnostics");
    input.refresh_command_available = false;
    header(input)
}

/// Degraded header: no command-backed reveal / detail path is reachable.
fn header_trace_missing() -> M5ResolvedPanelHeader {
    let mut input = header_current_input("header:product:trace-missing", "Workspace");
    input.detail_command_available = false;
    header(input)
}

// -- Clean local-action-cluster examples -------------------------------------------------------

fn clean_cluster_base(
    cluster_id: &str,
    label: &str,
    budget: M5LocalActionBudget,
    placement: M5PanelActionPlacement,
    compaction: M5PanelCompactionMode,
) -> M5LocalActionClusterResolutionInput {
    M5LocalActionClusterResolutionInput {
        cluster_id: cluster_id.to_owned(),
        cluster_label: label.to_owned(),
        local_action_budget: budget,
        local_actions_hover_only: false,
        keyboard_reachable: true,
        advanced_actions_persistent_clutter: false,
        action_placement: placement,
        overflowed_action_dropped: false,
        compaction_mode: compaction,
        reinstantiates_different_surface: false,
        compaction_preserves_identity: true,
        compaction_preserves_action_semantics: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean full-header cluster with inline primary actions within budget.
fn cluster_full_clean(cluster_id: &str, label: &str) -> M5ResolvedLocalActionCluster {
    cluster(clean_cluster_base(
        cluster_id,
        label,
        M5LocalActionBudget::WithinBudget,
        M5PanelActionPlacement::InlinePrimary,
        M5PanelCompactionMode::FullHeader,
    ))
}

/// Clean compacted cluster keeping a primary action plus overflow, identity and semantics preserved.
fn cluster_overflow_clean(cluster_id: &str, label: &str) -> M5ResolvedLocalActionCluster {
    cluster(clean_cluster_base(
        cluster_id,
        label,
        M5LocalActionBudget::PrimaryPlusOverflow,
        M5PanelActionPlacement::MixedPrimaryOverflow,
        M5PanelCompactionMode::CompactHeader,
    ))
}

/// Clean responsively-reflowed cluster with advanced actions in a structured menu, identity and
/// semantics preserved.
fn cluster_structured_clean(cluster_id: &str, label: &str) -> M5ResolvedLocalActionCluster {
    cluster(clean_cluster_base(
        cluster_id,
        label,
        M5LocalActionBudget::OverflowedMenu,
        M5PanelActionPlacement::StructuredMenu,
        M5PanelCompactionMode::ResponsiveReflow,
    ))
}

/// Clean minimized-rail cluster with all actions overflowed into a keyboard-reachable menu, identity
/// and semantics preserved.
fn cluster_minimized_clean(cluster_id: &str, label: &str) -> M5ResolvedLocalActionCluster {
    cluster(clean_cluster_base(
        cluster_id,
        label,
        M5LocalActionBudget::AllOverflowed,
        M5PanelActionPlacement::OverflowMenu,
        M5PanelCompactionMode::MinimizedRail,
    ))
}

/// Clean cluster with no local actions on a full header.
fn cluster_none_clean(cluster_id: &str, label: &str) -> M5ResolvedLocalActionCluster {
    cluster(clean_cluster_base(
        cluster_id,
        label,
        M5LocalActionBudget::NoLocalActions,
        M5PanelActionPlacement::NoActions,
        M5PanelCompactionMode::FullHeader,
    ))
}

// -- Degraded local-action-cluster examples ----------------------------------------------------

fn cluster_full_input(cluster_id: &str, label: &str) -> M5LocalActionClusterResolutionInput {
    clean_cluster_base(
        cluster_id,
        label,
        M5LocalActionBudget::WithinBudget,
        M5PanelActionPlacement::InlinePrimary,
        M5PanelCompactionMode::FullHeader,
    )
}

/// Degraded cluster: the cluster identity is unstated.
fn cluster_identity_unstated() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:product:no-id", "  ");
    input.cluster_label = "  ".to_owned();
    cluster(input)
}

/// Degraded cluster: the local-action budget cannot be resolved.
fn cluster_budget_unresolved() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:product:budget-unresolved", "Actions");
    input.local_action_budget = M5LocalActionBudget::BudgetUnknown;
    cluster(input)
}

/// Degraded cluster: the local actions can only be discovered by pointer hover.
fn cluster_actions_hover() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:search:actions-hover", "Result actions");
    input.local_actions_hover_only = true;
    cluster(input)
}

/// Degraded cluster: keyboard access to the actions is lost.
fn cluster_keyboard_missing() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:support:keyboard-missing", "Export actions");
    input.keyboard_reachable = false;
    cluster(input)
}

/// Degraded cluster: advanced actions are kept as persistent clutter.
fn cluster_persistent_clutter() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:data:persistent-clutter", "Grid actions");
    input.advanced_actions_persistent_clutter = true;
    cluster(input)
}

/// Degraded cluster: the action placement cannot be resolved.
fn cluster_placement_unresolved() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:support:placement-unresolved", "Support actions");
    input.action_placement = M5PanelActionPlacement::PlacementUnknown;
    cluster(input)
}

/// Degraded cluster: an overflowed local action was silently dropped.
fn cluster_overflow_dropped() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:review:overflow-dropped", "Queue actions");
    input.action_placement = M5PanelActionPlacement::OverflowMenu;
    input.overflowed_action_dropped = true;
    cluster(input)
}

/// Degraded cluster: compaction re-instantiated a different surface.
fn cluster_reinstantiates() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:review:reinstantiates", "Review actions");
    input.compaction_mode = M5PanelCompactionMode::CompactHeader;
    input.reinstantiates_different_surface = true;
    cluster(input)
}

/// Degraded cluster: compaction lost the panel identity.
fn cluster_loses_identity() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:shell:loses-identity", "Shell actions");
    input.compaction_mode = M5PanelCompactionMode::CompactHeader;
    input.compaction_preserves_identity = false;
    cluster(input)
}

/// Degraded cluster: compaction lost the action semantics.
fn cluster_loses_semantics() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:product:loses-semantics", "Workspace actions");
    input.compaction_mode = M5PanelCompactionMode::ResponsiveReflow;
    input.compaction_preserves_action_semantics = false;
    cluster(input)
}

/// Degraded cluster: no command-backed reveal / detail path is reachable.
fn cluster_trace_missing() -> M5ResolvedLocalActionCluster {
    let mut input = cluster_full_input("cluster:product:trace-missing", "Recent actions");
    input.detail_command_available = false;
    cluster(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5PanelConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5NavigationContentDowngradeTrigger>,
    panel_header_examples: Vec<M5ResolvedPanelHeader>,
    local_action_cluster_examples: Vec<M5ResolvedLocalActionCluster>,
) -> M5PanelControlsRow {
    M5PanelControlsRow {
        consumer_surface,
        qualification: M5NavigationContentQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5NavigationContentDeploymentLine::ALL.to_vec(),
        required_labels: M5NavigationContentRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5NavigationContentAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5PanelAnatomyPart::ALL.to_vec(),
        export_fields: M5PanelExportField::ALL.to_vec(),
        downgrade_triggers,
        panel_header_examples,
        local_action_cluster_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_REF,
            M5_PANEL_HEADER_SCHEMA_REF,
        ]),
        hides_actions_behind_hover_only_or_loses_keyboard_access: false,
        overstates_readiness_or_hides_source_freshness_cue: false,
        overloads_header_or_keeps_advanced_actions_as_persistent_clutter: false,
        compaction_reinstantiates_surface_or_loses_panel_identity: false,
    }
}

fn controls_rows() -> Vec<M5PanelControlsRow> {
    use M5NavigationContentConsumerSurface as C;
    use M5NavigationContentDowngradeTrigger as D;

    vec![
        base_row(
            C::DataUi,
            "Request / data pane-header owner",
            "The request/data pane header names a stable title and active context, labels a cached provider view at the boundary, points back to the canonical count/selection model, and keeps grid actions in structured menus rather than persistent clutter — degrading when the freshness cue is hidden, the header re-encodes counts, or advanced actions become persistent clutter",
            "evidence:m5-panel-controls-data-ui:001",
            vec![
                D::ActiveContextUnstated,
                D::PanelHeaderOverloaded,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                header_current_clean("header:data:providers", "Providers"),
                header_cached_clean("header:data:cached", "Cached providers"),
                header_cue_missing(),
                header_re_encodes(),
            ],
            vec![
                cluster_full_clean("cluster:data:grid", "Grid actions"),
                cluster_persistent_clutter(),
            ],
        ),
        base_row(
            C::ReviewUi,
            "Review-queue pane-header owner",
            "The review queue reuses the shared header grammar for remote-owned queues and keeps its compacted action cluster preserving panel identity and semantics — degrading when readiness is overstated, compaction re-instantiates a different surface, or an overflowed action is silently dropped",
            "evidence:m5-panel-controls-review-ui:001",
            vec![
                D::PanelHeaderOverloaded,
                D::LocalActionsHoverOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                header_remote_clean("header:review:queue", "Review queue"),
                header_readiness_overstated(),
            ],
            vec![
                cluster_overflow_clean("cluster:review:compact", "Review actions"),
                cluster_reinstantiates(),
                cluster_overflow_dropped(),
            ],
        ),
        base_row(
            C::SearchUi,
            "Search results pane-header owner",
            "The search surface reuses the same header semantics for a current results pane and keeps its responsively-reflowed action cluster keyboard-reachable — degrading when the active context is unresolved or the actions are hover-only",
            "evidence:m5-panel-controls-search-ui:001",
            vec![
                D::ActiveContextUnstated,
                D::LocalActionsHoverOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                header_current_clean("header:search:results", "Search results"),
                header_context_unresolved(),
            ],
            vec![
                cluster_structured_clean("cluster:search:reflow", "Result actions"),
                cluster_actions_hover(),
            ],
        ),
        base_row(
            C::ShellUi,
            "Governance / shell pane-header owner",
            "Governance surfaces reuse the same header grammar, honestly naming a background provider-owned pane and keeping a minimized-rail action cluster that preserves panel identity — degrading when a background context reads as active, the title slot is unstable, or compaction loses the panel identity",
            "evidence:m5-panel-controls-shell-ui:001",
            vec![
                D::ActiveContextUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                header_provider_clean("header:shell:provider", "Provider panel"),
                header_background_as_active(),
                header_title_unstable(),
            ],
            vec![
                cluster_minimized_clean("cluster:shell:rail", "Shell actions"),
                cluster_loses_identity(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export pane-header owner",
            "The support export carries the same resolved header and cluster truth, so a partial pane labelled at the boundary, an unresolved source/freshness, a missing refresh command, lost keyboard access, or an unresolved action placement is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-panel-controls-support-export:001",
            vec![
                D::LocalActionsHoverOnly,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                header_partial_clean("header:support:partial", "Partial export"),
                header_freshness_unresolved(),
                header_refresh_missing(),
            ],
            vec![
                cluster_full_clean("cluster:support:export", "Export actions"),
                cluster_keyboard_missing(),
                cluster_placement_unresolved(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product pane-header owner",
            "In-product surfaces reuse the same header and action grammar a user sees in the shell and data panes, always offering the command-backed refresh and reveal/detail affordances — degrading when the title is unstated, the reveal path is missing, the budget is unresolved, or compaction loses the action semantics",
            "evidence:m5-panel-controls-product-ui:001",
            vec![
                D::PanelHeaderOverloaded,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                header_current_clean("header:product:workspace", "Workspace"),
                header_title_unstated(),
                header_trace_missing(),
            ],
            vec![
                cluster_none_clean("cluster:product:none", "Workspace actions"),
                cluster_identity_unstated(),
                cluster_budget_unresolved(),
                cluster_loses_semantics(),
                cluster_trace_missing(),
            ],
        ),
    ]
}

fn governance_review() -> M5PanelGovernanceReview {
    M5PanelGovernanceReview {
        header_names_stable_title_and_active_context: true,
        header_labels_source_freshness_at_boundary: true,
        header_never_overstates_readiness: true,
        header_references_canonical_count_and_selection_model: true,
        header_offers_command_backed_refresh_and_detail: true,
        advanced_actions_in_structured_menu_or_overflow: true,
        local_actions_keyboard_reachable_never_hover_only: true,
        overflowed_action_never_dropped: true,
        compaction_preserves_panel_identity_and_action_semantics: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5PanelConsumerProjection {
    M5PanelConsumerProjection {
        shell_and_explorer_consume_shared_header_grammar: true,
        search_and_review_consume_shared_action_grammar: true,
        data_and_help_consume_shared_panel_header_semantics: true,
        header_facts_trace_to_single_canonical_model: true,
        support_export_reads_single_panel_header_source: true,
    }
}

fn proof_freshness() -> M5PanelProofFreshness {
    M5PanelProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PanelReleasePosture {
    M5PanelReleasePosture {
        proof_packet_ref: M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_SCHEMA_REF,
        M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_DOC_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_SCHEMA_REF,
        M5_NAVIGATION_CONTENT_COMPONENT_DOC_REF,
        M5_PANEL_HEADER_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 panel-header and local-action-cluster controls packet.
pub fn seeded_m5_panel_header_local_action_cluster_controls() -> M5PanelControlsPacket {
    M5PanelControlsPacket::new(M5PanelControlsPacketInput {
        packet_id: M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 panel-header and local-action-cluster controls with stable title slots, active-context truth, cached/partial/stale/remote/provider-owned source-freshness cues at the pane boundary, structured-menu/overflow action placement, keyboard-reachable command-backed refresh and reveal/detail affordances, and compaction that preserves panel identity across shell, request/data, review, search, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5PanelVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the governance/shell row is held at Beta pending pane-header parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed(
) -> M5PanelControlsPacket {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.packet_id =
        "m5-panel-header-local-action-cluster-controls:shell-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5NavigationContentQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support/export row is narrowed to Preview pending export-pane parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed(
) -> M5PanelControlsPacket {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.packet_id =
        "m5-panel-header-local-action-cluster-controls:support-export-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5NavigationContentConsumerSurface::SupportExport)
        .expect("support-export row present");
    row.qualification = M5NavigationContentQualificationClass::Preview;
    packet
}
