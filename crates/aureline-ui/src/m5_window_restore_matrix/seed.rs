//! Canonical seed builders for the frozen M5 window-restore matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical window-restore matrix.
pub const M5_WINDOW_RESTORE_MATRIX_PACKET_ID: &str = "m5-window-restore:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5WindowRestoreRequiredLabel> {
    M5WindowRestoreRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(extra: &[M5WindowRestoreRequiredLabel]) -> Vec<M5WindowRestoreRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    window_restore_family: M5WindowRestoreFamily,
    qualification: M5WindowRestoreQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5WindowRestoreRow {
    M5WindowRestoreRow {
        window_restore_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5WindowRestoreSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WindowRestoreDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        shared_workspace_authority_roles: vec![],
        window_local_topology_roles: vec![],
        skeleton_first_restore_roles: vec![],
        no_rerun_session_hydration_roles: vec![],
        display_topology_recovery_roles: vec![],
        degraded_reasons: M5WindowRestoreDegradedReason::ALL.to_vec(),
        accessibility_routes: M5WindowRestoreAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5WindowRestoreConsumerSurface::SupportExport,
            M5WindowRestoreConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5WindowRestoreDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore: false,
        deletes_layout_structure_silently_on_missing_extension_or_remote_target: false,
        leaves_windows_or_dialogs_unreachable_after_display_topology_remap: false,
        merges_workspace_authority_and_window_topology_into_one_opaque_blob: false,
        overclaims_restore_fidelity_when_only_context_or_evidence_reopened: false,
    }
}

fn window_restore_rows() -> Vec<M5WindowRestoreRow> {
    use M5WindowRestoreConsumerSurface as C;
    use M5WindowRestoreDowngradeTrigger as D;
    use M5WindowRestoreFamily as F;
    use M5WindowRestoreQualificationClass as Q;
    use M5WindowRestoreRequiredLabel as L;
    use M5WindowRestoreRole as R;

    let mut rows = Vec::new();

    // 1. Shared workspace authority.
    let mut row = base_row(
        F::SharedWorkspaceAuthority,
        Q::Stable,
        "Workspace-authority owner",
        "One shared-workspace-authority profile naming the single authority that backs multiple windows, window-local selection and focus, versioned and attributable pane trees, and the explicit authority-to-window binding so workspace authority and window topology stay separately inspectable and never merge into one opaque blob",
        "evidence:m5-shared-workspace-authority-parity:001",
        &[
            M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_MULTI_WINDOW_PARITY_SCHEMA_REF,
        ],
    );
    row.shared_workspace_authority_roles = M5SharedWorkspaceAuthorityRole::ALL.to_vec();
    row.semantic_roles = vec![R::WorkspaceAuthority, R::WindowTopology, R::PaneRole];
    row.required_labels = labels_with(&[L::WorkspaceAuthority]);
    row.consumer_surfaces = vec![
        C::RestoreCoordinator,
        C::ShellUi,
        C::WorkspaceService,
        C::Diagnostics,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
        D::WorkspaceAuthorityUnstated,
        D::WindowTopologyBoundaryDriftedBySurface,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Window-local topology.
    let mut row = base_row(
        F::WindowLocalTopology,
        Q::Stable,
        "Window-topology owner",
        "One window-local-topology profile naming the window-scoped pane tree, versioned pane topology, attributable pane roles, and pane-role placeholder so pane trees stay versioned and attributable and no window collapses into an opaque, unattributable topology",
        "evidence:m5-window-local-topology-parity:001",
        &[
            M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_MULTI_WINDOW_PARITY_SCHEMA_REF,
        ],
    );
    row.window_local_topology_roles = M5WindowLocalTopologyRole::ALL.to_vec();
    row.semantic_roles = vec![R::WindowTopology, R::PaneRole, R::DisplayAffinity];
    row.required_labels = labels_with(&[L::WorkspaceAuthority, L::DisplayAffinity]);
    row.consumer_surfaces = vec![
        C::RestoreCoordinator,
        C::ShellUi,
        C::WorkspaceService,
        C::Diagnostics,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::WindowTopologyBoundaryDriftedBySurface,
        D::WorkspaceAuthorityUnstated,
        D::DisplayAffinityUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Skeleton-first restore.
    let mut row = base_row(
        F::SkeletonFirstRestore,
        Q::Stable,
        "Restore-coordinator owner",
        "One skeleton-first-restore profile naming the layout skeleton rebuilt first, heavy dependency hydrated second, pane-role placeholder shown while hydrating, and disclosed restore-fidelity class so restore rebuilds the layout skeleton before hydrating heavy dependencies and a missing extension or remote target never deletes layout structure silently",
        "evidence:m5-skeleton-first-restore-parity:001",
        &[
            M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
            M5_RESTORE_FIDELITY_SCHEMA_REF,
            M5_MONITOR_GEOMETRY_REMAP_SCHEMA_REF,
        ],
    );
    row.skeleton_first_restore_roles = M5SkeletonFirstRestoreRole::ALL.to_vec();
    row.semantic_roles = vec![R::LayoutSkeleton, R::PaneRole, R::RestoreFidelity];
    row.required_labels = labels_with(&[L::RestoreFidelityClass]);
    row.consumer_surfaces = vec![
        C::RestoreCoordinator,
        C::ShellUi,
        C::SessionService,
        C::Diagnostics,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
        D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
        D::RestoreFidelityClassUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. No-rerun session hydration.
    let mut row = base_row(
        F::NoRerunSessionHydration,
        Q::Stable,
        "Session-service owner",
        "One no-rerun-session-hydration profile naming the session-scoped tool that never silently reruns, the privileged session that is never implicitly reattached, the explicit user action required to reacquire broader authority, and the disclosed reopened-versus-rerun context so terminals, debug sessions, notebooks, previews, remote shells, and collaboration surfaces never silently rerun or reacquire broader authority during restore",
        "evidence:m5-no-rerun-session-hydration-parity:001",
        &[
            M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
            M5_RESTORE_FIDELITY_SCHEMA_REF,
            M5_MULTI_WINDOW_PARITY_SCHEMA_REF,
        ],
    );
    row.no_rerun_session_hydration_roles = M5NoRerunSessionHydrationRole::ALL.to_vec();
    row.semantic_roles = vec![R::SessionHydration, R::RestoreFidelity];
    row.required_labels = labels_with(&[L::RestoreFidelityClass]);
    row.consumer_surfaces = vec![
        C::RestoreCoordinator,
        C::SessionService,
        C::WorkspaceService,
        C::Diagnostics,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore,
        D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
        D::SessionHydrationRuleUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Display-topology recovery.
    let mut row = base_row(
        F::DisplayTopologyRecovery,
        Q::Stable,
        "Display-topology recovery owner",
        "One display-topology-recovery profile naming the preserved monitor-affinity hint, windows staying visible after remap, dialogs staying reachable after remap, and preserved follow / presentation intent so a display-topology change keeps every window and dialog reachable and never strands a window off-screen",
        "evidence:m5-display-topology-recovery-parity:001",
        &[
            M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_MONITOR_GEOMETRY_REMAP_SCHEMA_REF,
        ],
    );
    row.display_topology_recovery_roles = M5DisplayTopologyRecoveryRole::ALL.to_vec();
    row.semantic_roles = vec![
        R::DisplayAffinity,
        R::WorkspaceAuthority,
        R::RestoreFidelity,
    ];
    row.required_labels = labels_with(&[L::DisplayAffinity]);
    row.consumer_surfaces = vec![
        C::RestoreCoordinator,
        C::ShellUi,
        C::WorkspaceService,
        C::Diagnostics,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
        D::DisplayAffinityUnstated,
        D::WindowTopologyBoundaryDriftedBySurface,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5WindowRestoreGovernanceReview {
    M5WindowRestoreGovernanceReview {
        workspace_authority_and_window_topology_stay_separately_inspectable: true,
        session_scoped_tools_never_silently_rerun_or_reattach: true,
        shared_authority_never_clobbers_window_local_selection_or_focus: true,
        restore_rebuilds_layout_skeleton_before_hydrating_heavy_dependencies: true,
        missing_extensions_or_remote_targets_never_delete_layout_structure_silently: true,
        display_topology_changes_keep_windows_and_dialogs_reachable: true,
        pane_trees_stay_versioned_and_attributable: true,
        reacquiring_broader_authority_requires_explicit_user_action: true,
        restore_fidelity_claims_never_outpace_exact_compatible_or_layout_only_evidence: true,
        every_family_declares_restore_contexts: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_window_restore_source: true,
        shell_recovery_diagnostics_admin_bind_to_single_window_restore_source: true,
        later_rows_cannot_invent_parallel_window_restore_vocabulary: true,
        restore_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5WindowRestoreConsumerProjection {
    M5WindowRestoreConsumerProjection {
        shell_and_recovery_consume_shared_window_restore_truth: true,
        diagnostics_and_admin_consume_shared_restore_fidelity_boundaries: true,
        session_and_workspace_services_consume_shared_window_topology: true,
        docs_help_and_screenshots_read_single_window_restore_source: true,
        terminal_debug_notebook_collab_bind_to_shared_no_rerun_rule: true,
        support_export_reads_single_window_restore_source: true,
    }
}

fn proof_freshness() -> M5WindowRestoreProofFreshness {
    M5WindowRestoreProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WindowRestoreReleasePosture {
    M5WindowRestoreReleasePosture {
        proof_packet_ref: M5_WINDOW_RESTORE_ARTIFACT_REF.to_owned(),
        window_restore_audit_ref: M5_WINDOW_RESTORE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_MULTI_WINDOW_PARITY_SCHEMA_REF,
        M5_MONITOR_GEOMETRY_REMAP_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 window-restore matrix packet.
pub fn seeded_m5_window_restore_matrix() -> M5WindowRestoreMatrixPacket {
    M5WindowRestoreMatrixPacket::new(M5WindowRestoreMatrixPacketInput {
        packet_id: M5_WINDOW_RESTORE_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 workspace-window, shared-authority, skeleton-restore, and no-rerun session-hydration matrix"
                .to_owned(),
        window_restore_rows: window_restore_rows(),
        vocabulary_set: M5WindowRestoreVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: no-rerun session hydration is held at Beta because the no-rerun guarantee is not yet
/// proven across every restore context; every family stays visible.
pub fn seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed(
) -> M5WindowRestoreMatrixPacket {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.packet_id = "m5-window-restore:no-rerun-session-hydration-beta:0001".to_owned();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::NoRerunSessionHydration)
        .expect("no-rerun session-hydration row present");
    row.qualification = M5WindowRestoreQualificationClass::Beta;
    packet
}

/// Narrowed variant: display-topology recovery is narrowed to Preview pending complete multi-monitor
/// remap-and-reachability evidence across every restore context; every family stays visible.
pub fn seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed(
) -> M5WindowRestoreMatrixPacket {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.packet_id = "m5-window-restore:display-topology-recovery-preview:0001".to_owned();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::DisplayTopologyRecovery)
        .expect("display-topology-recovery row present");
    row.qualification = M5WindowRestoreQualificationClass::Preview;
    packet
}
