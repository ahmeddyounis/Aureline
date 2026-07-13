//! Canonical seed builders for the frozen M5 shell-metric / density matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical shell-geometry matrix.
pub const M5_SHELL_METRIC_DENSITY_MATRIX_PACKET_ID: &str = "m5-shell-metric-density:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5ShellGeometryRequiredLabel> {
    M5ShellGeometryRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(extra: &[M5ShellGeometryRequiredLabel]) -> Vec<M5ShellGeometryRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    geometry_family: M5ShellGeometryFamily,
    qualification: M5ShellGeometryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5ShellGeometryRow {
    M5ShellGeometryRow {
        geometry_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ShellGeometrySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ShellGeometryDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        shell_metric_roles: vec![],
        minimum_size_roles: vec![],
        density_mode_roles: vec![],
        responsive_geometry_roles: vec![],
        collapse_priority_roles: vec![],
        degraded_reasons: M5ShellGeometryDegradedReason::ALL.to_vec(),
        accessibility_routes: M5ShellGeometryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellGeometryConsumerSurface::SupportExport,
            M5ShellGeometryConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ShellGeometryDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        density_or_collapse_changes_command_focus_or_trust: false,
        extension_or_embedded_sets_private_fracturing_width: false,
        shrinks_hit_target_below_supported_minimum: false,
        hides_primary_workflow_behind_overlay_only_fallback: false,
        lets_zone_starve_main_workspace_below_minimum: false,
    }
}

fn geometry_rows() -> Vec<M5ShellGeometryRow> {
    use M5CollapsePriorityRole as CP;
    use M5DensityModeRole as DM;
    use M5MinimumSizeRole as MS;
    use M5ResponsiveGeometryRole as RG;
    use M5ShellGeometryConsumerSurface as C;
    use M5ShellGeometryDowngradeTrigger as D;
    use M5ShellGeometryFamily as F;
    use M5ShellGeometryQualificationClass as Q;
    use M5ShellGeometryRequiredLabel as L;
    use M5ShellGeometryRole as R;
    use M5ShellMetricRole as SM;

    let mut rows = Vec::new();

    // 1. Shell metrics.
    let mut row = base_row(
        F::ShellMetric,
        Q::Stable,
        "Shell layout owner",
        "One shell-metric table naming default, minimum, recommended, and maximum sizes for the title / context bar, rail, sidebar, main editor group, right inspector, bottom panel, and status bar so every zone honors one registry-bound size rather than a scattered local constant",
        "evidence:m5-shell-metric-parity:001",
        &[
            M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
            M5_SHELL_METRICS_SCHEMA_REF,
            M5_SHELL_ZONE_SCHEMA_REF,
        ],
    );
    row.shell_metric_roles = SM::ALL.to_vec();
    row.semantic_roles = vec![R::Zone, R::Metric, R::WorkspaceDominance];
    row.required_labels = labels_with(&[L::SizeMetric]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::ReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ZoneStarvedMainWorkspace,
        D::MetricCopiedByHandAcrossPackages,
        D::SizeMetricUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Minimum sizes / hit targets.
    let mut row = base_row(
        F::MinimumSize,
        Q::Stable,
        "Shell layout owner",
        "One minimum-size contract naming the tab minimum width, resize-handle hit area, and icon-only hit targets so every control stays reachable by pointer and keyboard and never shrinks below the supported minimum under zoom or snapped widths",
        "evidence:m5-minimum-size-parity:001",
        &[
            M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
            M5_SHELL_METRICS_SCHEMA_REF,
            M5_SHELL_PRIMITIVES_SCHEMA_REF,
        ],
    );
    row.minimum_size_roles = MS::ALL.to_vec();
    row.semantic_roles = vec![R::HitTarget, R::Metric];
    row.required_labels = labels_with(&[L::SizeMetric]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::DataUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HitTargetShrankBelowMinimum,
        D::SizeMetricUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Density modes.
    let mut row = base_row(
        F::DensityMode,
        Q::Stable,
        "Design-token fidelity owner",
        "One density-mode contract naming the comfortable, standard, and compact modes as presentation-only changes that preserve the information architecture so command meaning, focus order, and trust visibility never move when density changes",
        "evidence:m5-density-mode-parity:001",
        &[
            M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
            M5_DENSITY_MODE_SCHEMA_REF,
            M5_SHELL_ZONE_SCHEMA_REF,
        ],
    );
    row.density_mode_roles = DM::ALL.to_vec();
    row.semantic_roles = vec![R::Density, R::WorkspaceDominance];
    row.required_labels = labels_with(&[L::DensityMode]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::DataUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DensityChangedCommandOrFocusOrTrust,
        D::DensityModeUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Responsive geometry.
    let mut row = base_row(
        F::ResponsiveGeometry,
        Q::Stable,
        "Adaptive-layout owner",
        "One responsive-geometry contract naming the compact, standard, and expanded window classes so snapped or narrow widths preserve task identity and recovery-critical state rather than dropping in-progress work",
        "evidence:m5-responsive-geometry-parity:001",
        &[
            M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
            M5_DENSITY_MODE_SCHEMA_REF,
            M5_SHELL_PRIMITIVES_SCHEMA_REF,
        ],
    );
    row.responsive_geometry_roles = RG::ALL.to_vec();
    row.semantic_roles = vec![R::Responsive, R::WorkspaceDominance];
    row.required_labels = labels_with(&[L::ResponsiveClass]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::ReviewUi,
        C::NotebookUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ResponsiveCollapseDroppedRecoveryState,
        D::ResponsiveClassUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Collapse priority.
    let mut row = base_row(
        F::CollapsePriority,
        Q::Stable,
        "Adaptive-layout owner",
        "One collapse-priority contract naming the declared collapse order and no-fracture geometry so the main workspace stays dominant, collapsed zones restore on re-expand, and extension or embedded surfaces never set a private width that fractures the shell",
        "evidence:m5-collapse-priority-parity:001",
        &[
            M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
            M5_DENSITY_MODE_SCHEMA_REF,
            M5_SHELL_ZONE_SCHEMA_REF,
        ],
    );
    row.collapse_priority_roles = CP::ALL.to_vec();
    row.semantic_roles = vec![R::Collapse, R::Responsive, R::WorkspaceDominance];
    row.required_labels = labels_with(&[L::ResponsiveClass]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::NotebookUi,
        C::DataUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExtensionSetPrivateFracturingWidth,
        D::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
        D::ResponsiveClassUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ShellGeometryGovernanceReview {
    M5ShellGeometryGovernanceReview {
        main_workspace_remains_dominant: true,
        zones_honor_declared_minimum_and_recommended_sizes: true,
        density_changes_presentation_not_information_architecture: true,
        responsive_preserves_task_identity: true,
        responsive_preserves_recovery_critical_state: true,
        hit_targets_meet_supported_minimums: true,
        resize_handles_meet_hit_area_minimum: true,
        tab_minimum_width_enforced: true,
        extension_or_embedded_cannot_invent_private_widths: true,
        no_primary_workflow_hidden_behind_overlay_only_fallback: true,
        metrics_bound_to_single_registry_not_hand_copied: true,
        every_family_declares_deployment_lines: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_shell_geometry_source: true,
        later_rows_cannot_invent_parallel_metric_or_density_vocabulary: true,
        geometry_survives_zoom_and_snapped_widths: true,
    }
}

fn consumer_projection() -> M5ShellGeometryConsumerProjection {
    M5ShellGeometryConsumerProjection {
        shell_and_editor_consume_shared_metric_and_density_grammar: true,
        review_and_notebook_consume_shared_responsive_geometry: true,
        data_and_embedded_surfaces_consume_shared_collapse_model: true,
        metric_density_consumers_read_single_registry_source: true,
        appearance_and_layout_bind_to_shared_shell_metrics: true,
        support_export_reads_single_shell_geometry_source: true,
    }
}

fn proof_freshness() -> M5ShellGeometryProofFreshness {
    M5ShellGeometryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ShellGeometryReleasePosture {
    M5ShellGeometryReleasePosture {
        proof_packet_ref: M5_SHELL_METRIC_DENSITY_ARTIFACT_REF.to_owned(),
        geometry_audit_ref: M5_SHELL_METRIC_DENSITY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_SHELL_METRICS_SCHEMA_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
        M5_SHELL_ZONE_SCHEMA_REF,
        M5_SHELL_PRIMITIVES_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 shell-metric / density matrix packet.
pub fn seeded_m5_shell_metric_density_matrix() -> M5ShellMetricDensityMatrixPacket {
    M5ShellMetricDensityMatrixPacket::new(M5ShellMetricDensityMatrixPacketInput {
        packet_id: M5_SHELL_METRIC_DENSITY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 shell-metric, minimum-size, density-mode, responsive-geometry, and collapse-priority shell-geometry matrix"
                .to_owned(),
        geometry_rows: geometry_rows(),
        vocabulary_set: M5ShellGeometryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: responsive geometry is held at Beta because compact / expanded window-class parity is
/// not yet proven across every deployment line; every family stays visible.
pub fn seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed(
) -> M5ShellMetricDensityMatrixPacket {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.packet_id = "m5-shell-metric-density:responsive-geometry-beta:0001".to_owned();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::ResponsiveGeometry)
        .expect("responsive-geometry row present");
    row.qualification = M5ShellGeometryQualificationClass::Beta;
    packet
}

/// Narrowed variant: collapse priority is narrowed to Preview pending no-fracture parity across every
/// deployment line; every family stays visible.
pub fn seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed(
) -> M5ShellMetricDensityMatrixPacket {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.packet_id = "m5-shell-metric-density:collapse-priority-preview:0001".to_owned();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::CollapsePriority)
        .expect("collapse-priority row present");
    row.qualification = M5ShellGeometryQualificationClass::Preview;
    packet
}
