//! Canonical seed builders for the M5 shell-metric and minimum-size registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean shell-metric and minimum-size entries
//! are built so the canonical logical-pixel envelopes for the title / context bar, activity rail, sidebar,
//! main editor group, right inspector, bottom panel, and status bar, the tab / resize-handle / icon-only
//! control hit targets, and the comfortable / standard / compact density coverage are proven across the
//! shell, editor, review, notebook, data, and support surfaces without any hand-copied constant,
//! out-of-envelope drift, workspace starvation, below-minimum hit target, or density gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SHELL_METRIC_REGISTRIES_PACKET_ID: &str =
    "m5-shell-metric-and-minimum-size-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn metric(input: M5ShellMetricEntryResolutionInput) -> M5ResolvedShellMetricEntry {
    resolve_shell_metric_entry(input).expect("seed shell-metric entry resolves")
}

fn minimum(input: M5MinimumSizeEntryResolutionInput) -> M5ResolvedMinimumSizeEntry {
    resolve_minimum_size_entry(input).expect("seed minimum-size entry resolves")
}

fn all_density() -> Vec<M5ShellDensityMode> {
    M5ShellDensityMode::ALL.to_vec()
}

// -- Clean shell-metric entries (zone envelopes bound to the shared registry) ---------------------

fn clean_metric_base(
    entry_id: &str,
    token_name: &str,
    semantic_role: M5ShellGeometryRole,
    zone: M5ShellZone,
    surface_context: M5ShellSurfaceContext,
) -> M5ShellMetricEntryResolutionInput {
    let bounds = zone.canonical_bounds();
    M5ShellMetricEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        metric_role: M5ShellMetricRole::BoundToRegistry,
        zone,
        surface_context,
        density_coverage: all_density(),
        minimum_px: bounds.minimum_px,
        default_px: bounds.default_px,
        recommended_px: bounds.recommended_px,
        maximum_px: bounds.maximum_px,
        bound_to_registry: true,
        starves_main_workspace: false,
        preserves_task_identity_under_snapped_width: true,
        proof_fresh: true,
    }
}

fn metric_title_bar_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:shell:title-context-bar",
        "shell.metric.title_context_bar.default",
        M5ShellGeometryRole::Zone,
        M5ShellZone::TitleContextBar,
        M5ShellSurfaceContext::Shell,
    ))
}

fn metric_rail_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:shell:activity-rail",
        "shell.metric.activity_rail.default",
        M5ShellGeometryRole::Zone,
        M5ShellZone::ActivityRail,
        M5ShellSurfaceContext::Shell,
    ))
}

fn metric_sidebar_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:editor:sidebar",
        "shell.metric.sidebar.default",
        M5ShellGeometryRole::Zone,
        M5ShellZone::Sidebar,
        M5ShellSurfaceContext::Editor,
    ))
}

fn metric_editor_group_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:editor:main-editor-group",
        "shell.metric.main_editor_group.minimum",
        M5ShellGeometryRole::WorkspaceDominance,
        M5ShellZone::MainEditorGroup,
        M5ShellSurfaceContext::Editor,
    ))
}

fn metric_inspector_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:review:right-inspector",
        "shell.metric.right_inspector.default",
        M5ShellGeometryRole::Metric,
        M5ShellZone::RightInspector,
        M5ShellSurfaceContext::Review,
    ))
}

fn metric_bottom_panel_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:data:bottom-panel",
        "shell.metric.bottom_panel.default",
        M5ShellGeometryRole::Metric,
        M5ShellZone::BottomPanel,
        M5ShellSurfaceContext::Data,
    ))
}

fn metric_status_bar_clean() -> M5ResolvedShellMetricEntry {
    metric(clean_metric_base(
        "metric:notebook:status-bar",
        "shell.metric.status_bar.default",
        M5ShellGeometryRole::Metric,
        M5ShellZone::StatusBar,
        M5ShellSurfaceContext::Notebook,
    ))
}

// -- Degraded shell-metric entries --------------------------------------------------------------

/// Degraded shell-metric entry: the metric is a hand-copied constant instead of tracing to the registry.
fn metric_hand_copied() -> M5ResolvedShellMetricEntry {
    let mut input = clean_metric_base(
        "metric:shell:hand-copied",
        "shell.metric.sidebar.default",
        M5ShellGeometryRole::Zone,
        M5ShellZone::Sidebar,
        M5ShellSurfaceContext::Shell,
    );
    input.metric_role = M5ShellMetricRole::HandCopiedConstantDisallowed;
    input.bound_to_registry = false;
    metric(input)
}

/// Degraded shell-metric entry: the declared bounds fall outside the zone's canonical envelope.
fn metric_outside_envelope() -> M5ResolvedShellMetricEntry {
    let mut input = clean_metric_base(
        "metric:editor:outside-envelope",
        "shell.metric.sidebar.minimum",
        M5ShellGeometryRole::Zone,
        M5ShellZone::Sidebar,
        M5ShellSurfaceContext::Editor,
    );
    // A sidebar minimum below the canonical 220 px floor drifts outside the envelope.
    input.minimum_px = 180;
    metric(input)
}

/// Degraded shell-metric entry: the metric starves the main workspace below its minimum.
fn metric_starves_workspace() -> M5ResolvedShellMetricEntry {
    let mut input = clean_metric_base(
        "metric:review:starves-workspace",
        "shell.metric.main_editor_group.minimum",
        M5ShellGeometryRole::WorkspaceDominance,
        M5ShellZone::MainEditorGroup,
        M5ShellSurfaceContext::Review,
    );
    input.starves_main_workspace = true;
    metric(input)
}

/// Degraded shell-metric entry: the comfortable / standard / compact density coverage is incomplete.
fn metric_density_incomplete() -> M5ResolvedShellMetricEntry {
    let mut input = clean_metric_base(
        "metric:data:density-incomplete",
        "shell.metric.sidebar.default",
        M5ShellGeometryRole::Zone,
        M5ShellZone::Sidebar,
        M5ShellSurfaceContext::Data,
    );
    input.density_coverage = vec![
        M5ShellDensityMode::Comfortable,
        M5ShellDensityMode::Standard,
    ];
    metric(input)
}

/// Degraded shell-metric entry: the metric does not preserve task identity under snapped widths.
fn metric_snapped_unsafe() -> M5ResolvedShellMetricEntry {
    let mut input = clean_metric_base(
        "metric:settings:snapped-unsafe",
        "shell.metric.title_context_bar.default",
        M5ShellGeometryRole::Zone,
        M5ShellZone::TitleContextBar,
        M5ShellSurfaceContext::Data,
    );
    input.preserves_task_identity_under_snapped_width = false;
    metric(input)
}

/// Degraded shell-metric entry: the canonical registry token name is unstated.
fn metric_token_unstated() -> M5ResolvedShellMetricEntry {
    let mut input = clean_metric_base(
        "metric:support:token-unstated",
        "  ",
        M5ShellGeometryRole::Zone,
        M5ShellZone::StatusBar,
        M5ShellSurfaceContext::Shell,
    );
    input.token_name = "  ".to_owned();
    metric(input)
}

// -- Clean minimum-size entries -----------------------------------------------------------------

fn clean_minimum_base(
    entry_id: &str,
    token_name: &str,
    minimum_size_role: M5MinimumSizeRole,
    control: M5ShellControlClass,
    surface_context: M5ShellSurfaceContext,
    declared_minimum_px: u32,
) -> M5MinimumSizeEntryResolutionInput {
    M5MinimumSizeEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        minimum_size_role,
        semantic_role: M5ShellGeometryRole::HitTarget,
        control,
        surface_context,
        density_coverage: all_density(),
        declared_minimum_px,
        pointer_and_keyboard_reachable: true,
        proof_fresh: true,
    }
}

fn minimum_tab_clean() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:shell:tab",
        "shell.minimum.tab.width",
        M5MinimumSizeRole::TabMinimumWidth,
        M5ShellControlClass::Tab,
        M5ShellSurfaceContext::Shell,
        96,
    ))
}

fn minimum_resize_handle_clean() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:editor:resize-handle",
        "shell.minimum.resize_handle.hit_area",
        M5MinimumSizeRole::ResizeHandleHitArea,
        M5ShellControlClass::ResizeHandle,
        M5ShellSurfaceContext::Editor,
        6,
    ))
}

fn minimum_icon_control_clean() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:review:icon-only-control",
        "shell.minimum.icon_only_control.hit_target",
        M5MinimumSizeRole::IconOnlyHitTarget,
        M5ShellControlClass::IconOnlyControl,
        M5ShellSurfaceContext::Review,
        32,
    ))
}

fn minimum_tab_clean_data() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:data:tab",
        "shell.minimum.tab.width",
        M5MinimumSizeRole::TabMinimumWidth,
        M5ShellControlClass::Tab,
        M5ShellSurfaceContext::Data,
        128,
    ))
}

fn minimum_icon_control_clean_settings() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:settings:icon-only-control",
        "shell.minimum.icon_only_control.hit_target",
        M5MinimumSizeRole::IconOnlyHitTarget,
        M5ShellControlClass::IconOnlyControl,
        M5ShellSurfaceContext::Notebook,
        36,
    ))
}

fn minimum_resize_handle_clean_support() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:support:resize-handle",
        "shell.minimum.resize_handle.hit_area",
        M5MinimumSizeRole::ResizeHandleHitArea,
        M5ShellControlClass::ResizeHandle,
        M5ShellSurfaceContext::Shell,
        8,
    ))
}

// -- Degraded minimum-size entries --------------------------------------------------------------

/// Degraded minimum-size entry: the hit target shrinks below its supported minimum.
fn minimum_below_minimum() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:shell:below-minimum",
        "shell.minimum.tab.width",
        M5MinimumSizeRole::TabMinimumWidth,
        M5ShellControlClass::Tab,
        M5ShellSurfaceContext::Shell,
        72,
    ))
}

/// Degraded minimum-size entry: the comfortable / standard / compact density coverage is incomplete.
fn minimum_density_incomplete() -> M5ResolvedMinimumSizeEntry {
    let mut input = clean_minimum_base(
        "minimum:data:density-incomplete",
        "shell.minimum.icon_only_control.hit_target",
        M5MinimumSizeRole::IconOnlyHitTarget,
        M5ShellControlClass::IconOnlyControl,
        M5ShellSurfaceContext::Data,
        32,
    );
    input.density_coverage = vec![M5ShellDensityMode::Standard];
    minimum(input)
}

/// Degraded minimum-size entry: the control class is unclassified.
fn minimum_control_unclassified() -> M5ResolvedMinimumSizeEntry {
    minimum(clean_minimum_base(
        "minimum:review:control-unclassified",
        "shell.minimum.unknown.hit_target",
        M5MinimumSizeRole::MeetsSupportedMinimum,
        M5ShellControlClass::ControlUnclassified,
        M5ShellSurfaceContext::Review,
        40,
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ShellMetricRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    shell_metric_entries: Vec<M5ResolvedShellMetricEntry>,
    minimum_size_entries: Vec<M5ResolvedMinimumSizeEntry>,
) -> M5ShellMetricRegistriesRow {
    M5ShellMetricRegistriesRow {
        consumer_surface,
        qualification: M5ShellGeometryQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5ShellGeometryDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5ShellGeometryRequiredLabel::Identity,
            M5ShellGeometryRequiredLabel::SemanticRole,
            M5ShellGeometryRequiredLabel::RegistryReference,
            M5ShellGeometryRequiredLabel::SizeMetric,
        ],
        accessibility_routes: M5ShellGeometryAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ShellRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5ShellRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        shell_metric_entries,
        minimum_size_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SHELL_METRIC_REGISTRIES_SCHEMA_REF,
            M5_SHELL_METRICS_SCHEMA_REF,
        ]),
        lets_zone_starve_main_workspace_below_minimum: false,
        shrinks_hit_target_below_supported_minimum: false,
        extension_or_embedded_sets_private_fracturing_width: false,
        metric_hand_copied_instead_of_registry: false,
    }
}

fn registry_rows() -> Vec<M5ShellMetricRegistriesRow> {
    use M5ShellGeometryConsumerSurface as C;
    use M5ShellGeometryDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the title / context bar and activity-rail geometry from the shared metric registry and keeps every tab above its minimum width; a hand-copied sidebar constant and a below-minimum tab degrade honestly instead of reading as a clean pass",
            "evidence:m5-shell-metric-shell-ui:001",
            vec![
                D::MetricCopiedByHandAcrossPackages,
                D::HitTargetShrankBelowMinimum,
                D::ProofStale,
            ],
            vec![
                metric_title_bar_clean(),
                metric_rail_clean(),
                metric_hand_copied(),
            ],
            vec![minimum_tab_clean(), minimum_below_minimum()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor resolves the sidebar and keeps the main editor group dominant above its 420 px minimum while binding resize-handle hit areas to the registry; a sidebar minimum below the canonical envelope degrades honestly",
            "evidence:m5-shell-metric-editor-ui:001",
            vec![
                D::ZoneStarvedMainWorkspace,
                D::MetricCopiedByHandAcrossPackages,
                D::ProofStale,
            ],
            vec![
                metric_sidebar_clean(),
                metric_editor_group_clean(),
                metric_outside_envelope(),
            ],
            vec![minimum_resize_handle_clean()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface resolves the right-inspector geometry and keeps icon-only control hit targets above 28 px; a metric that would starve the main workspace and an unclassified control both degrade honestly",
            "evidence:m5-shell-metric-review-ui:001",
            vec![
                D::ZoneStarvedMainWorkspace,
                D::HitTargetShrankBelowMinimum,
                D::ProofStale,
            ],
            vec![metric_inspector_clean(), metric_starves_workspace()],
            vec![minimum_icon_control_clean(), minimum_control_unclassified()],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The data surface resolves the bottom-panel geometry across every density mode and keeps tab minimum widths above their floor; a density-incomplete metric and a density-incomplete hit target degrade honestly",
            "evidence:m5-shell-metric-data-ui:001",
            vec![
                D::DensityModeUnstated,
                D::HitTargetShrankBelowMinimum,
                D::ProofStale,
            ],
            vec![metric_bottom_panel_clean(), metric_density_incomplete()],
            vec![minimum_tab_clean_data(), minimum_density_incomplete()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface resolves the status-bar geometry and keeps icon-only control hit targets above their minimum; a metric that fails under a snapped window width degrades honestly instead of fracturing the layout",
            "evidence:m5-shell-metric-settings-ui:001",
            vec![
                D::ResponsiveClassUnstated,
                D::HitTargetShrankBelowMinimum,
                D::ProofStale,
            ],
            vec![metric_status_bar_clean(), metric_snapped_unsafe()],
            vec![minimum_icon_control_clean_settings()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved shell-metric and minimum-size truth, so a hand-copied constant or an unstated registry token is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-shell-metric-support-export:001",
            vec![
                D::SizeMetricUnstated,
                D::MetricCopiedByHandAcrossPackages,
                D::ProofStale,
            ],
            vec![metric_title_bar_clean(), metric_token_unstated()],
            vec![minimum_resize_handle_clean_support()],
        ),
    ]
}

fn governance_review() -> M5ShellMetricRegistriesGovernanceReview {
    M5ShellMetricRegistriesGovernanceReview {
        shell_metric_registry_names_token_role_and_zone: true,
        reference_metrics_encoded_as_logical_pixel_contracts: true,
        every_surface_resolves_from_shared_registry: true,
        main_editor_group_stays_dominant: true,
        hit_targets_never_shrink_below_supported_minimum: true,
        every_entry_covers_all_density_modes: true,
        metrics_bound_to_single_registry_not_hand_copied: true,
        metric_drift_caught_before_release: true,
        first_consumers_use_canonical_metric_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ShellMetricRegistriesConsumerProjection {
    M5ShellMetricRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        editor_consumes_shared_registries: true,
        review_consumes_shared_registries: true,
        notebook_and_data_consume_shared_registries: true,
        shell_geometry_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ShellMetricRegistriesProofFreshness {
    M5ShellMetricRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ShellMetricRegistriesReleasePosture {
    M5ShellMetricRegistriesReleasePosture {
        proof_packet_ref: M5_SHELL_METRIC_REGISTRIES_ARTIFACT_REF.to_owned(),
        geometry_audit_ref: M5_SHELL_METRIC_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SHELL_METRIC_REGISTRIES_SCHEMA_REF,
        M5_SHELL_METRIC_REGISTRIES_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_SHELL_METRICS_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 shell-metric and minimum-size registries packet.
pub fn seeded_m5_shell_metric_minimum_size_registries() -> M5ShellMetricRegistriesPacket {
    M5ShellMetricRegistriesPacket::new(M5ShellMetricRegistriesPacketInput {
        packet_id: M5_SHELL_METRIC_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 shell-metric and minimum-size registries with canonical logical-pixel envelopes for the title / context bar, activity rail, sidebar, dominant main editor group, right inspector, bottom panel, and status bar, tab / resize-handle / icon-only control hit-target minima, comfortable / standard / compact density coverage, and registry-bound tracing across shell, editor, review, notebook, data, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5ShellMetricRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the editor-UI row is held at Beta pending main-editor-group-dominance proof on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed(
) -> M5ShellMetricRegistriesPacket {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.packet_id = "m5-shell-metric-and-minimum-size-registries:editor-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the data-UI row is narrowed to Preview pending density-mode parity on every surface;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed(
) -> M5ShellMetricRegistriesPacket {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.packet_id =
        "m5-shell-metric-and-minimum-size-registries:data-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::DataUi)
        .expect("data-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Preview;
    packet
}
