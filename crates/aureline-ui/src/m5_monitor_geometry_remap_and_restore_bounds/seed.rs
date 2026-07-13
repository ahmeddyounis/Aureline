//! Canonical seed builders for the M5 monitor-topology geometry-remap / restore-bounds registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures. The
//! headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean restore-bounds and remap-provenance entries are
//! built so the canonical monitor-attach / detach / undock / DPI-change / fullscreen / snapped-layout topology
//! changes, the window / approval-sheet / dialog / docked-panel / split-layout restore-surface kinds, the
//! visible-bounds clamp, the persisted layout intent and monitor-affinity hints, the exact / proportional /
//! affinity-fallback / recenter fidelity outcomes, and the diagnosable remap provenance are proven across the
//! shell, editor, review, notebook, data, and support surfaces without any off-screen restore, focus trap,
//! stale-coordinate replay, silent workspace drop, or unrecorded fidelity change.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_PACKET_ID: &str =
    "m5-monitor-geometry-remap-and-restore-bounds:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn restore(input: M5RestoreBoundsEntryResolutionInput) -> M5ResolvedRestoreBoundsEntry {
    resolve_restore_bounds_entry(input).expect("seed restore-bounds entry resolves")
}

fn provenance(
    input: M5GeometryRemapProvenanceEntryResolutionInput,
) -> M5ResolvedGeometryRemapProvenanceEntry {
    resolve_geometry_remap_provenance_entry(input).expect("seed remap-provenance entry resolves")
}

fn all_provenance_fields() -> Vec<M5RemapProvenanceField> {
    M5RemapProvenanceField::ALL.to_vec()
}

// -- Clean restore-bounds entries (visible-bounds clamp bound to the shared registry) --------------

#[allow(clippy::too_many_arguments)]
fn clean_restore_base(
    entry_id: &str,
    token_name: &str,
    class_role: M5ResponsiveGeometryRole,
    kind: M5RestoreSurfaceKind,
    change: M5TopologyChange,
    surface_context: M5RemapSurfaceContext,
    fidelity_outcome: M5RemapFidelityOutcome,
) -> M5RestoreBoundsEntryResolutionInput {
    let reduced = fidelity_outcome.is_reduced_fidelity();
    M5RestoreBoundsEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5ShellGeometryRole::Responsive,
        responsive_geometry_role: class_role,
        restore_surface_kind: kind,
        topology_change: change,
        surface_context,
        fidelity_outcome,
        reopens_fully_off_screen: false,
        traps_focus_after_remap: false,
        clamped_into_visible_bounds: true,
        preserves_usable_geometry: true,
        uses_absolute_coordinates_instead_of_intent: false,
        // A reduced-fidelity restore always surfaces a recenter / reset affordance as recoverable truth.
        offers_recenter_reset_affordance: reduced,
        proof_fresh: true,
    }
}

fn restore_window_shell() -> M5ResolvedRestoreBoundsEntry {
    restore(clean_restore_base(
        "restore:shell:window-detach",
        "shell.restore.window.bounds",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::RestorableWindow,
        M5TopologyChange::MonitorDetach,
        M5RemapSurfaceContext::Shell,
        M5RemapFidelityOutcome::ExactBoundsRestored,
    ))
}

fn restore_panel_editor_dpi() -> M5ResolvedRestoreBoundsEntry {
    restore(clean_restore_base(
        "restore:editor:panel-dpi",
        "editor.restore.docked_panel.bounds",
        M5ResponsiveGeometryRole::PreservesTaskIdentity,
        M5RestoreSurfaceKind::DockedPanel,
        M5TopologyChange::DpiChange,
        M5RemapSurfaceContext::Editor,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
    ))
}

fn restore_split_review() -> M5ResolvedRestoreBoundsEntry {
    restore(clean_restore_base(
        "restore:review:split-fullscreen",
        "review.restore.split_layout.bounds",
        M5ResponsiveGeometryRole::PreservesTaskIdentity,
        M5RestoreSurfaceKind::SplitLayout,
        M5TopologyChange::FullscreenTransition,
        M5RemapSurfaceContext::Review,
        M5RemapFidelityOutcome::ExactBoundsRestored,
    ))
}

fn restore_dialog_data_undock() -> M5ResolvedRestoreBoundsEntry {
    restore(clean_restore_base(
        "restore:data:dialog-undock",
        "data.restore.dialog.bounds",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::Dialog,
        M5TopologyChange::Undock,
        M5RemapSurfaceContext::Data,
        M5RemapFidelityOutcome::MonitorAffinityFallback,
    ))
}

fn restore_sheet_notebook_snap() -> M5ResolvedRestoreBoundsEntry {
    restore(clean_restore_base(
        "restore:notebook:sheet-snap",
        "notebook.restore.approval_sheet.bounds",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::ApprovalSheet,
        M5TopologyChange::SnappedLayoutRecovery,
        M5RemapSurfaceContext::Notebook,
        M5RemapFidelityOutcome::ExactBoundsRestored,
    ))
}

fn restore_window_support_attach() -> M5ResolvedRestoreBoundsEntry {
    restore(clean_restore_base(
        "restore:support:window-attach",
        "shell.restore.window.recenter",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::RestorableWindow,
        M5TopologyChange::MonitorAttach,
        M5RemapSurfaceContext::Shell,
        M5RemapFidelityOutcome::RecenterReset,
    ))
}

// -- Degraded restore-bounds entries -------------------------------------------------------------

/// Degraded restore-bounds entry: the restored surface reopened fully off-screen after a monitor detach.
fn restore_off_screen() -> M5ResolvedRestoreBoundsEntry {
    let mut input = clean_restore_base(
        "restore:shell:off-screen",
        "shell.restore.window.bounds",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::RestorableWindow,
        M5TopologyChange::MonitorDetach,
        M5RemapSurfaceContext::Shell,
        M5RemapFidelityOutcome::ExactBoundsRestored,
    );
    input.reopens_fully_off_screen = true;
    restore(input)
}

/// Degraded restore-bounds entry: the restored surface trapped focus after a DPI change.
fn restore_traps_focus() -> M5ResolvedRestoreBoundsEntry {
    let mut input = clean_restore_base(
        "restore:editor:traps-focus",
        "editor.restore.docked_panel.bounds",
        M5ResponsiveGeometryRole::PreservesTaskIdentity,
        M5RestoreSurfaceKind::DockedPanel,
        M5TopologyChange::DpiChange,
        M5RemapSurfaceContext::Editor,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
    );
    input.traps_focus_after_remap = true;
    restore(input)
}

/// Degraded restore-bounds entry: the restore lost usable editor / panel / inspector geometry.
fn restore_loses_geometry() -> M5ResolvedRestoreBoundsEntry {
    let mut input = clean_restore_base(
        "restore:review:loses-geometry",
        "review.restore.split_layout.bounds",
        M5ResponsiveGeometryRole::PreservesTaskIdentity,
        M5RestoreSurfaceKind::SplitLayout,
        M5TopologyChange::FullscreenTransition,
        M5RemapSurfaceContext::Review,
        M5RemapFidelityOutcome::ExactBoundsRestored,
    );
    input.preserves_usable_geometry = false;
    restore(input)
}

/// Degraded restore-bounds entry: the restore replayed stale absolute coordinates instead of persisted intent.
fn restore_stale_coordinates() -> M5ResolvedRestoreBoundsEntry {
    let mut input = clean_restore_base(
        "restore:data:stale-coordinates",
        "data.restore.dialog.bounds",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::Dialog,
        M5TopologyChange::Undock,
        M5RemapSurfaceContext::Data,
        M5RemapFidelityOutcome::ExactBoundsRestored,
    );
    input.uses_absolute_coordinates_instead_of_intent = true;
    restore(input)
}

/// Degraded restore-bounds entry: fidelity was reduced but no recenter / reset affordance was offered.
fn restore_no_affordance() -> M5ResolvedRestoreBoundsEntry {
    let mut input = clean_restore_base(
        "restore:notebook:no-affordance",
        "notebook.restore.approval_sheet.bounds",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::ApprovalSheet,
        M5TopologyChange::SnappedLayoutRecovery,
        M5RemapSurfaceContext::Notebook,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
    );
    input.offers_recenter_reset_affordance = false;
    restore(input)
}

/// Degraded restore-bounds entry: the canonical registry token name is unstated.
fn restore_token_unstated() -> M5ResolvedRestoreBoundsEntry {
    let mut input = clean_restore_base(
        "restore:support:token-unstated",
        "  ",
        M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        M5RestoreSurfaceKind::RestorableWindow,
        M5TopologyChange::MonitorAttach,
        M5RemapSurfaceContext::Shell,
        M5RemapFidelityOutcome::RecenterReset,
    );
    input.token_name = "  ".to_owned();
    restore(input)
}

// -- Clean remap-provenance entries --------------------------------------------------------------

fn clean_provenance_base(
    entry_id: &str,
    token_name: &str,
    change: M5TopologyChange,
    fidelity_outcome: M5RemapFidelityOutcome,
    surface_context: M5RemapSurfaceContext,
) -> M5GeometryRemapProvenanceEntryResolutionInput {
    M5GeometryRemapProvenanceEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5ShellGeometryRole::WorkspaceDominance,
        responsive_geometry_role: M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        topology_change: change,
        fidelity_outcome,
        surface_context,
        recorded_provenance_fields: all_provenance_fields(),
        preserves_workspace_focus_and_critical_state: true,
        records_remap_reason: true,
        silently_drops_workspace_or_state: false,
        proof_fresh: true,
    }
}

fn provenance_exact_shell() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:shell:exact",
        "shell.remap.provenance.exact",
        M5TopologyChange::MonitorDetach,
        M5RemapFidelityOutcome::ExactBoundsRestored,
        M5RemapSurfaceContext::Shell,
    ))
}

fn provenance_proportional_editor() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:editor:proportional",
        "editor.remap.provenance.proportional",
        M5TopologyChange::DpiChange,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
        M5RemapSurfaceContext::Editor,
    ))
}

fn provenance_affinity_review() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:review:affinity",
        "review.remap.provenance.affinity_fallback",
        M5TopologyChange::FullscreenTransition,
        M5RemapFidelityOutcome::MonitorAffinityFallback,
        M5RemapSurfaceContext::Review,
    ))
}

fn provenance_recenter_data() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:data:recenter",
        "data.remap.provenance.recenter_reset",
        M5TopologyChange::Undock,
        M5RemapFidelityOutcome::RecenterReset,
        M5RemapSurfaceContext::Data,
    ))
}

fn provenance_exact_notebook() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:notebook:exact",
        "notebook.remap.provenance.exact",
        M5TopologyChange::SnappedLayoutRecovery,
        M5RemapFidelityOutcome::ExactBoundsRestored,
        M5RemapSurfaceContext::Notebook,
    ))
}

fn provenance_proportional_support() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:support:proportional",
        "shell.remap.provenance.proportional",
        M5TopologyChange::MonitorAttach,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
        M5RemapSurfaceContext::Shell,
    ))
}

// -- Degraded remap-provenance entries -----------------------------------------------------------

/// Degraded remap-provenance entry: the remap silently dropped the workspace, focus chain, or critical state.
fn provenance_silent_drop() -> M5ResolvedGeometryRemapProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:shell:silent-drop",
        "shell.remap.provenance.exact",
        M5TopologyChange::MonitorDetach,
        M5RemapFidelityOutcome::ExactBoundsRestored,
        M5RemapSurfaceContext::Shell,
    );
    input.silently_drops_workspace_or_state = true;
    provenance(input)
}

/// Degraded remap-provenance entry: the remap dropped the workspace / focus / critical state.
fn provenance_drops_state() -> M5ResolvedGeometryRemapProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:editor:drops-state",
        "editor.remap.provenance.proportional",
        M5TopologyChange::DpiChange,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
        M5RemapSurfaceContext::Editor,
    );
    input.preserves_workspace_focus_and_critical_state = false;
    provenance(input)
}

/// Degraded remap-provenance entry: the remap reason was not recorded in provenance.
fn provenance_reason_unrecorded() -> M5ResolvedGeometryRemapProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:review:reason-unrecorded",
        "review.remap.provenance.affinity_fallback",
        M5TopologyChange::FullscreenTransition,
        M5RemapFidelityOutcome::MonitorAffinityFallback,
        M5RemapSurfaceContext::Review,
    );
    input.records_remap_reason = false;
    provenance(input)
}

/// Degraded remap-provenance entry: the recorded provenance omits a mandatory field.
fn provenance_detail_incomplete() -> M5ResolvedGeometryRemapProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:data:detail-incomplete",
        "data.remap.provenance.recenter_reset",
        M5TopologyChange::Undock,
        M5RemapFidelityOutcome::RecenterReset,
        M5RemapSurfaceContext::Data,
    );
    // Records the trigger and fidelity outcome but omits the after-topology, after-DPI, and preserved-state
    // fields, so a diagnostician cannot explain why fidelity changed.
    input.recorded_provenance_fields = vec![
        M5RemapProvenanceField::RemapTrigger,
        M5RemapProvenanceField::FidelityOutcome,
    ];
    provenance(input)
}

/// Degraded remap-provenance entry: the fidelity outcome is unclassified.
fn provenance_fidelity_unclassified() -> M5ResolvedGeometryRemapProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:notebook:fidelity-unclassified",
        "notebook.remap.provenance.unclassified",
        M5TopologyChange::SnappedLayoutRecovery,
        M5RemapFidelityOutcome::OutcomeUnclassified,
        M5RemapSurfaceContext::Notebook,
    ))
}

/// Degraded remap-provenance entry: the canonical registry token name is unstated.
fn provenance_token_unstated() -> M5ResolvedGeometryRemapProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:support:token-unstated",
        "  ",
        M5TopologyChange::MonitorAttach,
        M5RemapFidelityOutcome::ProportionalIntentRemap,
        M5RemapSurfaceContext::Shell,
    );
    input.token_name = "  ".to_owned();
    provenance(input)
}

// -- Row builders --------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5MonitorGeometryRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    restore_bounds_entries: Vec<M5ResolvedRestoreBoundsEntry>,
    remap_provenance_entries: Vec<M5ResolvedGeometryRemapProvenanceEntry>,
) -> M5MonitorGeometryRemapAndRestoreBoundsRow {
    M5MonitorGeometryRemapAndRestoreBoundsRow {
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
        anatomy_parts: M5RestoreRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5RestoreRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        restore_bounds_entries,
        remap_provenance_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_REF,
            M5_DENSITY_MODE_SCHEMA_REF,
        ]),
        restore_reopens_off_screen_or_traps_focus: false,
        remap_replays_stale_absolute_coordinates_without_clamp: false,
        remap_silently_drops_workspace_focus_or_critical_state: false,
        reduced_fidelity_without_recenter_or_provenance: false,
    }
}

fn registry_rows() -> Vec<M5MonitorGeometryRemapAndRestoreBoundsRow> {
    use M5ShellGeometryConsumerSurface as C;
    use M5ShellGeometryDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the restorable-window bounds from the shared registry and clamps them into visible bounds after a monitor detach, recording the remap in provenance; a window that would reopen fully off-screen and a remap that would silently drop the workspace both degrade honestly instead of reading as a clean pass",
            "evidence:m5-monitor-geometry-shell-ui:001",
            vec![
                D::ResponsiveCollapseDroppedRecoveryState,
                D::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
                D::ProofStale,
            ],
            vec![restore_window_shell(), restore_off_screen()],
            vec![provenance_exact_shell(), provenance_silent_drop()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor remaps a docked panel through a mixed-DPI change with a proportional-intent restore and a recenter affordance, keeping usable geometry and recording the remap; a restore that would trap focus and a remap that would drop the focus chain both degrade honestly",
            "evidence:m5-monitor-geometry-editor-ui:001",
            vec![
                D::ResponsiveCollapseDroppedRecoveryState,
                D::ZoneStarvedMainWorkspace,
                D::ProofStale,
            ],
            vec![restore_panel_editor_dpi(), restore_traps_focus()],
            vec![provenance_proportional_editor(), provenance_drops_state()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface restores a split layout across a fullscreen transition with exact bounds and records a monitor-affinity fallback in provenance; a restore that would lose usable compare geometry and a remap whose reason is unrecorded both degrade honestly",
            "evidence:m5-monitor-geometry-review-ui:001",
            vec![
                D::ZoneStarvedMainWorkspace,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![restore_split_review(), restore_loses_geometry()],
            vec![provenance_affinity_review(), provenance_reason_unrecorded()],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The data surface restores a dialog after an undock via a monitor-affinity fallback with a recenter affordance and records a recenter-reset in provenance; a restore that would replay stale absolute coordinates and a remap whose provenance omits detail both degrade honestly",
            "evidence:m5-monitor-geometry-data-ui:001",
            vec![
                D::MetricCopiedByHandAcrossPackages,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![restore_dialog_data_undock(), restore_stale_coordinates()],
            vec![provenance_recenter_data(), provenance_detail_incomplete()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface restores an approval sheet after a snapped-layout recovery with exact bounds and records the remap; a reduced-fidelity restore that would omit its recenter affordance and a remap whose fidelity outcome is unclassified both degrade honestly",
            "evidence:m5-monitor-geometry-settings-ui:001",
            vec![
                D::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![restore_sheet_notebook_snap(), restore_no_affordance()],
            vec![provenance_exact_notebook(), provenance_fidelity_unclassified()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved restore-bounds and remap-provenance truth, so an off-screen restore or an unstated registry token is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-monitor-geometry-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ResponsiveCollapseDroppedRecoveryState,
                D::ProofStale,
            ],
            vec![restore_window_support_attach(), restore_token_unstated()],
            vec![provenance_proportional_support(), provenance_token_unstated()],
        ),
    ]
}

fn governance_review() -> M5MonitorGeometryRemapAndRestoreBoundsGovernanceReview {
    M5MonitorGeometryRemapAndRestoreBoundsGovernanceReview {
        registry_names_token_role_and_topology_change: true,
        restore_clamps_every_surface_into_visible_bounds: true,
        no_restored_window_or_sheet_reopens_off_screen_or_traps_focus: true,
        persists_layout_intent_and_monitor_affinity_not_absolute_coordinates: true,
        mixed_dpi_and_topology_drills_preserve_usable_geometry: true,
        reduced_fidelity_offers_recenter_or_reset_affordance: true,
        geometry_continuity_distinct_from_surface_continuity: true,
        workspace_focus_and_critical_state_preserved_across_remap: true,
        remap_provenance_records_enough_detail_to_diagnose: true,
        every_surface_resolves_from_shared_registry: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5MonitorGeometryRemapAndRestoreBoundsConsumerProjection {
    M5MonitorGeometryRemapAndRestoreBoundsConsumerProjection {
        shell_consumes_shared_registries: true,
        editor_consumes_shared_registries: true,
        review_consumes_shared_registries: true,
        notebook_and_data_consume_shared_registries: true,
        geometry_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5MonitorGeometryRemapAndRestoreBoundsProofFreshness {
    M5MonitorGeometryRemapAndRestoreBoundsProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MonitorGeometryRemapAndRestoreBoundsReleasePosture {
    M5MonitorGeometryRemapAndRestoreBoundsReleasePosture {
        proof_packet_ref: M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_ARTIFACT_REF.to_owned(),
        geometry_audit_ref: M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_REF,
        M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 monitor-topology geometry-remap / restore-bounds registries packet.
pub fn seeded_m5_monitor_geometry_remap_and_restore_bounds(
) -> M5MonitorGeometryRemapAndRestoreBoundsPacket {
    M5MonitorGeometryRemapAndRestoreBoundsPacket::new(
        M5MonitorGeometryRemapAndRestoreBoundsPacketInput {
            packet_id: M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_PACKET_ID.to_owned(),
            registries_label:
                "M5 monitor-topology geometry-remap and restore-bounds registries with monitor-affinity restore across monitor attach / detach, undock, DPI change, fullscreen, and snapped-layout recovery, visible-bounds clamping with no off-screen or focus-trapped restore, persisted layout intent instead of stale absolute coordinates, recenter / reset affordances under reduced fidelity, and diagnosable geometry-remap provenance across shell, editor, review, notebook, data, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5MonitorGeometryRemapAndRestoreBoundsVocabularySet::canonical(),
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

/// Narrowed variant: the editor-UI row is held at Beta pending mixed-DPI focus-trap proof at 400% zoom on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_monitor_geometry_remap_and_restore_bounds_editor_ui_beta_narrowed(
) -> M5MonitorGeometryRemapAndRestoreBoundsPacket {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.packet_id =
        "m5-monitor-geometry-remap-and-restore-bounds:editor-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the settings-UI row is narrowed to Preview pending recenter-affordance parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_monitor_geometry_remap_and_restore_bounds_settings_ui_preview_narrowed(
) -> M5MonitorGeometryRemapAndRestoreBoundsPacket {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.packet_id =
        "m5-monitor-geometry-remap-and-restore-bounds:settings-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::SettingsUi)
        .expect("settings-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Preview;
    packet
}
