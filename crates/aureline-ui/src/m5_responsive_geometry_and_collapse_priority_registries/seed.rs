//! Canonical seed builders for the M5 responsive-geometry / collapse-priority registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can
//! only carry projections the resolvers actually produce. Clean window-class and collapse-step entries are
//! built so the canonical Compact / Standard / Expanded desktop window classes and their logical-pixel width
//! bounds, the title / rail / sidebar / workspace / inspector / panel / status shell-zone coexistence, the
//! declared collapse priority order, and the identity-stable docked / sheet / overlay / temporary-panel
//! transitions are proven across the shell, editor, review, notebook, data, and support surfaces without any
//! private breakpoint, dropped recovery state, hover-only action, unusable pane, protected-target collapse,
//! or overlay-only fallback.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_PACKET_ID: &str =
    "m5-responsive-geometry-and-collapse-priority-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn window(input: M5WindowClassEntryResolutionInput) -> M5ResolvedWindowClassEntry {
    resolve_window_class_entry(input).expect("seed window-class entry resolves")
}

fn collapse(input: M5CollapseStepEntryResolutionInput) -> M5ResolvedCollapseStepEntry {
    resolve_collapse_step_entry(input).expect("seed collapse-step entry resolves")
}

fn all_zones() -> Vec<M5ResponsiveShellZone> {
    M5ResponsiveShellZone::ALL.to_vec()
}

// -- Clean window-class entries (canonical bounds bound to the shared registry) --------------------

fn clean_window_base(
    entry_id: &str,
    token_name: &str,
    class: M5WindowClass,
    class_role: M5ResponsiveGeometryRole,
    surface_context: M5ResponsiveSurfaceContext,
) -> M5WindowClassEntryResolutionInput {
    let bounds = class.canonical_bounds();
    M5WindowClassEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5ShellGeometryRole::Responsive,
        responsive_geometry_role: class_role,
        window_class: class,
        surface_context,
        min_width_px: bounds.min_width_px,
        max_width_px: bounds.max_width_px,
        coexisting_zones: all_zones(),
        preserves_task_identity: true,
        preserves_recovery_critical_state: true,
        makes_essential_action_hover_only: false,
        narrows_editor_group_into_unusable_pane: false,
        proof_fresh: true,
    }
}

fn window_compact_shell() -> M5ResolvedWindowClassEntry {
    window(clean_window_base(
        "window:shell:compact",
        "shell.responsive.compact.class",
        M5WindowClass::CompactDesktop,
        M5ResponsiveGeometryRole::CompactClass,
        M5ResponsiveSurfaceContext::Shell,
    ))
}

fn window_standard_editor() -> M5ResolvedWindowClassEntry {
    window(clean_window_base(
        "window:editor:standard",
        "shell.responsive.standard.class",
        M5WindowClass::StandardDesktop,
        M5ResponsiveGeometryRole::StandardClass,
        M5ResponsiveSurfaceContext::Editor,
    ))
}

fn window_expanded_review() -> M5ResolvedWindowClassEntry {
    window(clean_window_base(
        "window:review:expanded",
        "shell.responsive.expanded.class",
        M5WindowClass::ExpandedDesktop,
        M5ResponsiveGeometryRole::ExpandedClass,
        M5ResponsiveSurfaceContext::Review,
    ))
}

fn window_standard_notebook() -> M5ResolvedWindowClassEntry {
    window(clean_window_base(
        "window:notebook:standard",
        "shell.responsive.standard.class",
        M5WindowClass::StandardDesktop,
        M5ResponsiveGeometryRole::PreservesTaskIdentity,
        M5ResponsiveSurfaceContext::Notebook,
    ))
}

fn window_compact_data() -> M5ResolvedWindowClassEntry {
    window(clean_window_base(
        "window:data:compact",
        "shell.responsive.compact.class",
        M5WindowClass::CompactDesktop,
        M5ResponsiveGeometryRole::CompactClass,
        M5ResponsiveSurfaceContext::Data,
    ))
}

fn window_expanded_support() -> M5ResolvedWindowClassEntry {
    window(clean_window_base(
        "window:support:expanded",
        "shell.responsive.expanded.class",
        M5WindowClass::ExpandedDesktop,
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5ResponsiveSurfaceContext::Shell,
    ))
}

// -- Degraded window-class entries ---------------------------------------------------------------

/// Degraded window-class entry: the declared bounds drift from the canonical class bounds (a private
/// breakpoint an extension invented instead of resolving the shared tokens).
fn window_bounds_drift() -> M5ResolvedWindowClassEntry {
    let mut input = clean_window_base(
        "window:shell:private-bounds",
        "shell.responsive.compact.class",
        M5WindowClass::CompactDesktop,
        M5ResponsiveGeometryRole::CompactClass,
        M5ResponsiveSurfaceContext::Shell,
    );
    // A private breakpoint that stays a plausible width but does not match the canonical Compact bounds.
    input.min_width_px = 1024;
    input.max_width_px = 1300;
    window(input)
}

/// Degraded window-class entry: an essential action became hover-only at this class.
fn window_hover_only() -> M5ResolvedWindowClassEntry {
    let mut input = clean_window_base(
        "window:editor:hover-only",
        "shell.responsive.standard.class",
        M5WindowClass::StandardDesktop,
        M5ResponsiveGeometryRole::StandardClass,
        M5ResponsiveSurfaceContext::Editor,
    );
    input.makes_essential_action_hover_only = true;
    window(input)
}

/// Degraded window-class entry: a compare / editor group narrowed into an unusable pane at this class.
fn window_unusable_pane() -> M5ResolvedWindowClassEntry {
    let mut input = clean_window_base(
        "window:review:unusable-pane",
        "shell.responsive.expanded.class",
        M5WindowClass::ExpandedDesktop,
        M5ResponsiveGeometryRole::ExpandedClass,
        M5ResponsiveSurfaceContext::Review,
    );
    input.narrows_editor_group_into_unusable_pane = true;
    window(input)
}

/// Degraded window-class entry: the responsive change drops recovery-critical state.
fn window_drops_recovery() -> M5ResolvedWindowClassEntry {
    let mut input = clean_window_base(
        "window:data:drops-recovery",
        "shell.responsive.compact.class",
        M5WindowClass::CompactDesktop,
        M5ResponsiveGeometryRole::CompactClass,
        M5ResponsiveSurfaceContext::Data,
    );
    input.preserves_recovery_critical_state = false;
    window(input)
}

/// Degraded window-class entry: the shell-zone coexistence is incomplete.
fn window_zone_incomplete() -> M5ResolvedWindowClassEntry {
    let mut input = clean_window_base(
        "window:notebook:zone-incomplete",
        "shell.responsive.standard.class",
        M5WindowClass::StandardDesktop,
        M5ResponsiveGeometryRole::StandardClass,
        M5ResponsiveSurfaceContext::Notebook,
    );
    input.coexisting_zones = vec![
        M5ResponsiveShellZone::TitleContextBar,
        M5ResponsiveShellZone::MainWorkspace,
        M5ResponsiveShellZone::StatusBar,
    ];
    window(input)
}

/// Degraded window-class entry: the canonical registry token name is unstated.
fn window_token_unstated() -> M5ResolvedWindowClassEntry {
    let mut input = clean_window_base(
        "window:support:token-unstated",
        "  ",
        M5WindowClass::ExpandedDesktop,
        M5ResponsiveGeometryRole::ExpandedClass,
        M5ResponsiveSurfaceContext::Shell,
    );
    input.token_name = "  ".to_owned();
    window(input)
}

// -- Clean collapse-step entries -----------------------------------------------------------------

fn clean_collapse_base(
    entry_id: &str,
    token_name: &str,
    collapse_role: M5CollapsePriorityRole,
    target: M5CollapseTarget,
    transition_form: M5IdentityTransitionForm,
    surface_context: M5ResponsiveSurfaceContext,
    collapses: bool,
) -> M5CollapseStepEntryResolutionInput {
    let declared_collapse_rank = target.canonical_collapse_rank().unwrap_or(0);
    M5CollapseStepEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5ShellGeometryRole::Collapse,
        collapse_priority_role: collapse_role,
        collapse_target: target,
        transition_form,
        surface_context,
        collapses,
        declared_collapse_rank,
        preserves_identity_state_and_keyboard_route: true,
        starves_main_workspace: false,
        uses_private_fracturing_width: false,
        proof_fresh: true,
    }
}

fn collapse_inspector_detail_shell() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:shell:inspector-detail",
        "shell.collapse.inspector_detail.sheet",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::OptionalRightInspectorDetail,
        M5IdentityTransitionForm::Sheet,
        M5ResponsiveSurfaceContext::Shell,
        true,
    ))
}

fn collapse_bottom_tabs_editor() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:editor:bottom-tabs",
        "shell.collapse.bottom_tabs.overflow",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::SecondaryBottomPanelTabs,
        M5IdentityTransitionForm::Overflow,
        M5ResponsiveSurfaceContext::Editor,
        true,
    ))
}

fn collapse_editor_workspace_dominant() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:editor:workspace-dominant",
        "shell.collapse.editor_workspace.docked",
        M5CollapsePriorityRole::MainWorkspaceStaysDominant,
        M5CollapseTarget::EditorWorkspace,
        M5IdentityTransitionForm::Docked,
        M5ResponsiveSurfaceContext::Editor,
        false,
    ))
}

fn collapse_side_tools_review() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:review:side-tools",
        "shell.collapse.side_tools.overflow",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::LowFrequencySideTools,
        M5IdentityTransitionForm::Overflow,
        M5ResponsiveSurfaceContext::Review,
        true,
    ))
}

fn collapse_primary_nav_notebook() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:notebook:primary-nav",
        "shell.collapse.primary_nav.temporary_panel",
        M5CollapsePriorityRole::RestoreOnReexpand,
        M5CollapseTarget::PrimaryNavigation,
        M5IdentityTransitionForm::TemporaryPanel,
        M5ResponsiveSurfaceContext::Notebook,
        true,
    ))
}

fn collapse_identity_protected_settings() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:settings:identity-protected",
        "shell.collapse.path_branch_trust_identity.docked",
        M5CollapsePriorityRole::NoFractureGeometry,
        M5CollapseTarget::PathBranchTrustTargetIdentity,
        M5IdentityTransitionForm::Docked,
        M5ResponsiveSurfaceContext::Notebook,
        false,
    ))
}

fn collapse_inspector_detail_support() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:support:inspector-detail",
        "shell.collapse.inspector_detail.inline_disclosure",
        M5CollapsePriorityRole::OverlayOnlyFallbackAvoided,
        M5CollapseTarget::OptionalRightInspectorDetail,
        M5IdentityTransitionForm::InlineDisclosure,
        M5ResponsiveSurfaceContext::Shell,
        true,
    ))
}

// -- Degraded collapse-step entries --------------------------------------------------------------

/// Degraded collapse-step entry: a docked / sheet / overlay / temporary-panel transition dropped the
/// surface's identity, state, history, or keyboard route.
fn collapse_drops_identity() -> M5ResolvedCollapseStepEntry {
    let mut input = clean_collapse_base(
        "collapse:shell:drops-identity",
        "shell.collapse.inspector_detail.sheet",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::OptionalRightInspectorDetail,
        M5IdentityTransitionForm::Sheet,
        M5ResponsiveSurfaceContext::Shell,
        true,
    );
    input.preserves_identity_state_and_keyboard_route = false;
    collapse(input)
}

/// Degraded collapse-step entry: the collapse starved the main workspace below its minimum.
fn collapse_starves_workspace() -> M5ResolvedCollapseStepEntry {
    let mut input = clean_collapse_base(
        "collapse:data:starves-workspace",
        "shell.collapse.primary_nav.temporary_panel",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::PrimaryNavigation,
        M5IdentityTransitionForm::TemporaryPanel,
        M5ResponsiveSurfaceContext::Data,
        true,
    );
    input.starves_main_workspace = true;
    collapse(input)
}

/// Degraded collapse-step entry: a protected target (the editor workspace) collapsed.
fn collapse_collapses_protected() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:review:collapses-protected",
        "shell.collapse.editor_workspace.docked",
        M5CollapsePriorityRole::MainWorkspaceStaysDominant,
        M5CollapseTarget::EditorWorkspace,
        M5IdentityTransitionForm::Docked,
        M5ResponsiveSurfaceContext::Review,
        true,
    ))
}

/// Degraded collapse-step entry: a primary workflow was hidden behind an overlay-only fallback.
fn collapse_overlay_only_fallback() -> M5ResolvedCollapseStepEntry {
    collapse(clean_collapse_base(
        "collapse:settings:overlay-only",
        "shell.collapse.primary_nav.overlay",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::PrimaryNavigation,
        M5IdentityTransitionForm::Overlay,
        M5ResponsiveSurfaceContext::Data,
        true,
    ))
}

/// Degraded collapse-step entry: the canonical registry token name is unstated.
fn collapse_token_unstated() -> M5ResolvedCollapseStepEntry {
    let mut input = clean_collapse_base(
        "collapse:support:token-unstated",
        "  ",
        M5CollapsePriorityRole::CollapseOrderDeclared,
        M5CollapseTarget::OptionalRightInspectorDetail,
        M5IdentityTransitionForm::Sheet,
        M5ResponsiveSurfaceContext::Shell,
        true,
    );
    input.token_name = "  ".to_owned();
    collapse(input)
}

// -- Row builders --------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ResponsiveGeometryRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    window_class_entries: Vec<M5ResolvedWindowClassEntry>,
    collapse_step_entries: Vec<M5ResolvedCollapseStepEntry>,
) -> M5ResponsiveGeometryAndCollapsePriorityRegistriesRow {
    M5ResponsiveGeometryAndCollapsePriorityRegistriesRow {
        consumer_surface,
        qualification: M5ShellGeometryQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5ShellGeometryDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5ShellGeometryRequiredLabel::Identity,
            M5ShellGeometryRequiredLabel::SemanticRole,
            M5ShellGeometryRequiredLabel::RegistryReference,
            M5ShellGeometryRequiredLabel::ResponsiveClass,
        ],
        accessibility_routes: M5ShellGeometryAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ResponsiveRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5ResponsiveRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        window_class_entries,
        collapse_step_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_REF,
            M5_DENSITY_MODE_SCHEMA_REF,
        ]),
        responsive_or_collapse_alters_command_focus_or_trust: false,
        extension_sets_private_fracturing_width: false,
        lets_zone_starve_main_workspace_below_minimum: false,
        hides_primary_workflow_behind_overlay_only_fallback: false,
    }
}

fn registry_rows() -> Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesRow> {
    use M5ShellGeometryConsumerSurface as C;
    use M5ShellGeometryDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the Compact desktop window class from the shared registry and keeps optional right-inspector detail identity-stable when it moves into a sheet; a private breakpoint and an identity-dropping transition degrade honestly instead of reading as a clean pass",
            "evidence:m5-responsive-geometry-shell-ui:001",
            vec![
                D::ExtensionSetPrivateFracturingWidth,
                D::ResponsiveCollapseDroppedRecoveryState,
                D::ProofStale,
            ],
            vec![window_compact_shell(), window_bounds_drift()],
            vec![collapse_inspector_detail_shell(), collapse_drops_identity()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor resolves the Standard desktop window class and keeps the dominant editor workspace docked while secondary bottom-panel tabs move to overflow; an essential action that would become hover-only degrades honestly",
            "evidence:m5-responsive-geometry-editor-ui:001",
            vec![
                D::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
                D::ResponsiveClassUnstated,
                D::ProofStale,
            ],
            vec![window_standard_editor(), window_hover_only()],
            vec![
                collapse_bottom_tabs_editor(),
                collapse_editor_workspace_dominant(),
            ],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface resolves the Expanded desktop window class and converts low-frequency side tools to overflow; a compare / editor group that would narrow into an unusable pane and a collapse of the protected editor workspace both degrade honestly",
            "evidence:m5-responsive-geometry-review-ui:001",
            vec![
                D::ZoneStarvedMainWorkspace,
                D::ResponsiveCollapseDroppedRecoveryState,
                D::ProofStale,
            ],
            vec![window_expanded_review(), window_unusable_pane()],
            vec![collapse_side_tools_review(), collapse_collapses_protected()],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The data surface resolves the Compact desktop window class and keeps primary navigation identity-stable when it moves into a temporary panel; a responsive change that would drop recovery-critical state and a collapse that would starve the main workspace both degrade honestly",
            "evidence:m5-responsive-geometry-data-ui:001",
            vec![
                D::ResponsiveCollapseDroppedRecoveryState,
                D::ZoneStarvedMainWorkspace,
                D::ProofStale,
            ],
            vec![window_compact_data(), window_drops_recovery()],
            vec![collapse_primary_nav_notebook(), collapse_starves_workspace()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface resolves the Standard desktop window class across every shell zone and keeps path / branch / trust / target identity docked and protected; a window class that omits shell zones and a primary navigation hidden behind an overlay-only fallback both degrade honestly",
            "evidence:m5-responsive-geometry-settings-ui:001",
            vec![
                D::ResponsiveClassUnstated,
                D::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
                D::ProofStale,
            ],
            vec![window_standard_notebook(), window_zone_incomplete()],
            vec![
                collapse_identity_protected_settings(),
                collapse_overlay_only_fallback(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved window-class and collapse-step truth, so a private breakpoint or an unstated registry token is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-responsive-geometry-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ExtensionSetPrivateFracturingWidth,
                D::ProofStale,
            ],
            vec![window_expanded_support(), window_token_unstated()],
            vec![collapse_inspector_detail_support(), collapse_token_unstated()],
        ),
    ]
}

fn governance_review() -> M5ResponsiveGeometryAndCollapsePriorityRegistriesGovernanceReview {
    M5ResponsiveGeometryAndCollapsePriorityRegistriesGovernanceReview {
        registry_names_token_role_and_class: true,
        window_class_bounds_encoded_as_logical_pixel_tokens: true,
        every_surface_resolves_from_shared_registry: true,
        responsive_preserves_task_identity_and_recovery_state: true,
        no_hover_only_action_or_unusable_pane: true,
        declared_collapse_priority_order_honored: true,
        transitions_stay_identity_stable: true,
        main_workspace_stays_dominant: true,
        no_primary_workflow_hidden_behind_overlay_only_fallback: true,
        extension_cannot_invent_private_fracturing_width: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ResponsiveGeometryAndCollapsePriorityRegistriesConsumerProjection {
    M5ResponsiveGeometryAndCollapsePriorityRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        editor_consumes_shared_registries: true,
        review_consumes_shared_registries: true,
        notebook_and_data_consume_shared_registries: true,
        geometry_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ResponsiveGeometryAndCollapsePriorityRegistriesProofFreshness {
    M5ResponsiveGeometryAndCollapsePriorityRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ResponsiveGeometryAndCollapsePriorityRegistriesReleasePosture {
    M5ResponsiveGeometryAndCollapsePriorityRegistriesReleasePosture {
        proof_packet_ref: M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        geometry_audit_ref: M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_REF,
        M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 responsive-geometry / collapse-priority registries packet.
pub fn seeded_m5_responsive_geometry_and_collapse_priority_registries(
) -> M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket {
    M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket::new(
        M5ResponsiveGeometryAndCollapsePriorityRegistriesPacketInput {
            packet_id: M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 responsive-geometry and collapse-priority registries with canonical Compact 1024-1279 / Standard 1280-1599 / Expanded 1600+ desktop window-class width bounds, title / rail / sidebar / workspace / inspector / panel / status shell-zone coexistence, declared adaptive-collapse priority order, identity-stable docked / sheet / overlay / temporary-panel transitions, and registry-bound tracing across shell, editor, review, notebook, data, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5ResponsiveGeometryAndCollapsePriorityRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the editor-UI row is held at Beta pending hover-only and unusable-pane proof at 400%
/// zoom on every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_responsive_geometry_and_collapse_priority_registries_editor_ui_beta_narrowed(
) -> M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.packet_id =
        "m5-responsive-geometry-and-collapse-priority-registries:editor-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the settings-UI row is narrowed to Preview pending shell-zone coexistence parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_responsive_geometry_and_collapse_priority_registries_settings_ui_preview_narrowed(
) -> M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.packet_id =
        "m5-responsive-geometry-and-collapse-priority-registries:settings-ui-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::SettingsUi)
        .expect("settings-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Preview;
    packet
}
