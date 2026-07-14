//! Canonical seed builders for the M5 workspace-authority and window-topology registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean workspace-authority and window-topology entries are
//! built so the one stable workspace-authority object resolving per workspace, the window-local selection and
//! focus staying window-local while one authority backs multiple windows, the shared dirty-buffer / save /
//! checkpoint state kept distinct from the profile-defaults reference, the canonical / accessible / audit
//! resolution forms, and the window-local pane-tree / focus-history / display-affinity disclosure triple are
//! proven across the shell, recovery, diagnostics, admin, workspace, session, and support surfaces without any
//! hand-copied per-window assumption, window-local overwrite, incomplete object, leaked authority, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_PACKET_ID: &str =
    "m5-workspace-authority-and-window-topology-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn authority(input: M5WorkspaceAuthorityEntryResolutionInput) -> M5ResolvedWorkspaceAuthorityEntry {
    resolve_workspace_authority_entry(input).expect("seed workspace-authority entry resolves")
}

fn topology(input: M5WindowTopologyEntryResolutionInput) -> M5ResolvedWindowTopologyEntry {
    resolve_window_topology_entry(input).expect("seed window-topology entry resolves")
}

fn all_forms() -> Vec<M5WindowStateResolutionForm> {
    M5WindowStateResolutionForm::ALL.to_vec()
}

// -- Clean workspace-authority entries (stable object, window-local isolation, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_authority_base(
    entry_id: &str,
    workspace_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    authority_scope: M5WorkspaceAuthorityScope,
    surface_context: M5WindowRestoreSurfaceContext,
    backing_window_ids: &str,
    stable_pane_tree_ids: &str,
    shared_dirty_buffer_state: &str,
    shared_save_checkpoint_state: &str,
    authority_state_root: &str,
    profile_defaults_ref: &str,
) -> M5WorkspaceAuthorityEntryResolutionInput {
    M5WorkspaceAuthorityEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        workspace_id: workspace_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        authority_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        backing_window_ids: backing_window_ids.to_owned(),
        stable_pane_tree_ids: stable_pane_tree_ids.to_owned(),
        shared_dirty_buffer_state: shared_dirty_buffer_state.to_owned(),
        shared_save_checkpoint_state: shared_save_checkpoint_state.to_owned(),
        authority_state_root: authority_state_root.to_owned(),
        profile_defaults_ref: profile_defaults_ref.to_owned(),
        bound_to_registry: true,
        window_local_state_isolated: true,
        shares_authority_across_windows: false,
        window_local_history_preserved: true,
        proof_fresh: true,
    }
}

fn authority_shell_single_clean() -> M5ResolvedWorkspaceAuthorityEntry {
    authority(clean_authority_base(
        "authority:shell:single",
        "workspace.acme.single",
        "workspace.authority.single_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        M5WindowRestoreSurfaceContext::ShellSurface,
        "window.main",
        "pane-tree.main.v3",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    ))
}

fn authority_recovery_multi_clean() -> M5ResolvedWorkspaceAuthorityEntry {
    // One authority backs two windows; selection and focus stay window-local with preserved history.
    let mut base = clean_authority_base(
        "authority:recovery:multi",
        "workspace.acme.shared",
        "workspace.authority.multi_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        M5WindowRestoreSurfaceContext::RecoverySurface,
        "window.main|window.secondary",
        "pane-tree.main.v4|pane-tree.secondary.v2",
        "dirty-buffer.shared.0011",
        "checkpoint.shared.0011",
        "workspace-authority.acme/shared",
        "profile-defaults.machine-hints",
    );
    base.shares_authority_across_windows = true;
    base.window_local_history_preserved = true;
    authority(base)
}

fn authority_diagnostics_detached_clean() -> M5ResolvedWorkspaceAuthorityEntry {
    // A detached / auxiliary window sharing the workspace authority keeps its window-local history distinct.
    let mut base = clean_authority_base(
        "authority:diagnostics:detached",
        "workspace.acme.shared",
        "workspace.authority.detached_auxiliary",
        M5WindowRestoreRole::PaneRole,
        M5WorkspaceAuthorityScope::DetachedAuxiliaryWindowScope,
        M5WindowRestoreSurfaceContext::DiagnosticsSurface,
        "window.main|window.detached-inspector",
        "pane-tree.main.v4|pane-tree.detached.v1",
        "dirty-buffer.shared.0011",
        "checkpoint.shared.0011",
        "workspace-authority.acme/shared",
        "profile-defaults.machine-hints",
    );
    base.shares_authority_across_windows = true;
    base.window_local_history_preserved = true;
    authority(base)
}

fn authority_admin_multi_clean() -> M5ResolvedWorkspaceAuthorityEntry {
    let mut base = clean_authority_base(
        "authority:admin:multi",
        "workspace.acme.shared",
        "workspace.authority.multi_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        M5WindowRestoreSurfaceContext::AdminSurface,
        "window.main|window.secondary|window.third",
        "pane-tree.main.v4|pane-tree.secondary.v2|pane-tree.third.v1",
        "dirty-buffer.shared.0011",
        "checkpoint.shared.0011",
        "workspace-authority.acme/shared",
        "profile-defaults.machine-hints",
    );
    base.shares_authority_across_windows = true;
    base.window_local_history_preserved = true;
    authority(base)
}

fn authority_support_single_clean() -> M5ResolvedWorkspaceAuthorityEntry {
    authority(clean_authority_base(
        "authority:support:single",
        "workspace.acme.single",
        "workspace.authority.single_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        M5WindowRestoreSurfaceContext::SupportOrExportForm,
        "window.main",
        "pane-tree.main.v3",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    ))
}

// -- Degraded workspace-authority entries -------------------------------------------------------

/// Degraded authority entry: the behavior is a hand-copied per-window assumption instead of tracing to the
/// registry.
fn authority_unbound() -> M5ResolvedWorkspaceAuthorityEntry {
    let mut base = clean_authority_base(
        "authority:admin:unbound",
        "workspace.acme.shared",
        "workspace.authority.multi_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        M5WindowRestoreSurfaceContext::AdminSurface,
        "window.main|window.secondary",
        "pane-tree.main.v4|pane-tree.secondary.v2",
        "dirty-buffer.shared.0011",
        "checkpoint.shared.0011",
        "workspace-authority.acme/shared",
        "profile-defaults.machine-hints",
    );
    base.shares_authority_across_windows = true;
    base.bound_to_registry = false;
    authority(base)
}

/// Degraded authority entry: the resolved authority object is incomplete — the stable pane-tree IDs are
/// unstated.
fn authority_object_incomplete() -> M5ResolvedWorkspaceAuthorityEntry {
    let mut base = clean_authority_base(
        "authority:shell:incomplete",
        "workspace.acme.single",
        "workspace.authority.single_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        M5WindowRestoreSurfaceContext::ShellSurface,
        "window.main",
        "pane-tree.main.v3",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    );
    base.stable_pane_tree_ids = "   ".to_owned();
    authority(base)
}

/// Degraded authority entry: a window-local selection or focus overwrote the shared workspace authority.
fn authority_window_local_overwrite() -> M5ResolvedWorkspaceAuthorityEntry {
    let mut base = clean_authority_base(
        "authority:diagnostics:overwrite",
        "workspace.acme.shared",
        "workspace.authority.multi_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        M5WindowRestoreSurfaceContext::DiagnosticsSurface,
        "window.main|window.secondary",
        "pane-tree.main.v4|pane-tree.secondary.v2",
        "dirty-buffer.shared.0011",
        "checkpoint.shared.0011",
        "workspace-authority.acme/shared",
        "profile-defaults.machine-hints",
    );
    base.shares_authority_across_windows = true;
    base.window_local_state_isolated = false;
    authority(base)
}

/// Degraded authority entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn authority_form_incomplete() -> M5ResolvedWorkspaceAuthorityEntry {
    let mut base = clean_authority_base(
        "authority:recovery:form-incomplete",
        "workspace.acme.shared",
        "workspace.authority.multi_window",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::MultiWindowSharedAuthorityScope,
        M5WindowRestoreSurfaceContext::RecoverySurface,
        "window.main|window.secondary",
        "pane-tree.main.v4|pane-tree.secondary.v2",
        "dirty-buffer.shared.0011",
        "checkpoint.shared.0011",
        "workspace-authority.acme/shared",
        "profile-defaults.machine-hints",
    );
    base.shares_authority_across_windows = true;
    base.window_local_history_preserved = true;
    base.resolution_form_coverage = vec![M5WindowStateResolutionForm::CanonicalObject];
    authority(base)
}

/// Degraded authority entry: the canonical registry token name is unstated.
fn authority_token_unstated() -> M5ResolvedWorkspaceAuthorityEntry {
    let mut base = clean_authority_base(
        "authority:support:token-unstated",
        "workspace.acme.single",
        "  ",
        M5WindowRestoreRole::WorkspaceAuthority,
        M5WorkspaceAuthorityScope::SingleWindowAuthorityScope,
        M5WindowRestoreSurfaceContext::SupportOrExportForm,
        "window.main",
        "pane-tree.main.v3",
        "dirty-buffer.shared.0007",
        "checkpoint.shared.0007",
        "workspace-authority.acme/single",
        "profile-defaults.machine-hints",
    );
    base.token_name = "  ".to_owned();
    authority(base)
}

// -- Clean window-topology entries --------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_topology_base(
    entry_id: &str,
    window_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    topology_surface: M5WindowTopologySurface,
    surface_context: M5WindowRestoreSurfaceContext,
    window_local_pane_tree: &str,
    window_local_focus_history: &str,
    display_affinity_hint: &str,
) -> M5WindowTopologyEntryResolutionInput {
    M5WindowTopologyEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        window_id: window_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        topology_surface,
        surface_context,
        resolution_form_coverage: all_forms(),
        window_local_pane_tree: window_local_pane_tree.to_owned(),
        window_local_focus_history: window_local_focus_history.to_owned(),
        display_affinity_hint: display_affinity_hint.to_owned(),
        keeps_authority_distinct: true,
        topology_is_truthful: true,
        authority_copied_into_window_used: false,
        authority_copy_disclosed: false,
        profile_default_override_asserted: false,
        profile_default_override_explained: false,
        proof_fresh: true,
    }
}

fn topology_primary_shell_clean() -> M5ResolvedWindowTopologyEntry {
    topology(clean_topology_base(
        "topology:primary:shell",
        "window.main",
        "window.topology.primary",
        M5WindowRestoreRole::WindowTopology,
        M5WindowTopologySurface::PrimaryWindowTopology,
        M5WindowRestoreSurfaceContext::ShellSurface,
        "pane-tree.main.v4",
        "focus-history.window.main",
        "display-affinity.monitor-1",
    ))
}

fn topology_auxiliary_recovery_clean() -> M5ResolvedWindowTopologyEntry {
    // A disclosed read-only cache of shared authority state on an auxiliary window stays distinct and clean.
    let mut base = clean_topology_base(
        "topology:auxiliary:recovery",
        "window.secondary",
        "window.topology.auxiliary",
        M5WindowRestoreRole::DisplayAffinity,
        M5WindowTopologySurface::AuxiliaryWindowTopology,
        M5WindowRestoreSurfaceContext::RecoverySurface,
        "pane-tree.secondary.v2",
        "focus-history.window.secondary",
        "display-affinity.monitor-2",
    );
    base.authority_copied_into_window_used = true;
    base.authority_copy_disclosed = true;
    topology(base)
}

fn topology_diagnostics_clean() -> M5ResolvedWindowTopologyEntry {
    // A justified profile-default override stays distinct: it is explained on this diagnostics surface.
    let mut base = clean_topology_base(
        "topology:diagnostics:inspection",
        "window.detached-inspector",
        "window.topology.diagnostics",
        M5WindowRestoreRole::WindowTopology,
        M5WindowTopologySurface::DiagnosticsInspectionTopology,
        M5WindowRestoreSurfaceContext::DiagnosticsSurface,
        "pane-tree.detached.v1",
        "focus-history.window.detached-inspector",
        "display-affinity.monitor-2",
    );
    base.profile_default_override_asserted = true;
    base.profile_default_override_explained = true;
    topology(base)
}

fn topology_auxiliary_admin_clean() -> M5ResolvedWindowTopologyEntry {
    topology(clean_topology_base(
        "topology:auxiliary:admin",
        "window.third",
        "window.topology.auxiliary",
        M5WindowRestoreRole::WindowTopology,
        M5WindowTopologySurface::AuxiliaryWindowTopology,
        M5WindowRestoreSurfaceContext::AdminSurface,
        "pane-tree.third.v1",
        "focus-history.window.third",
        "display-affinity.monitor-3",
    ))
}

fn topology_primary_support_clean() -> M5ResolvedWindowTopologyEntry {
    topology(clean_topology_base(
        "topology:primary:support",
        "window.main",
        "window.topology.primary",
        M5WindowRestoreRole::WindowTopology,
        M5WindowTopologySurface::PrimaryWindowTopology,
        M5WindowRestoreSurfaceContext::SupportOrExportForm,
        "pane-tree.main.v4",
        "focus-history.window.main",
        "display-affinity.monitor-1",
    ))
}

// -- Degraded window-topology entries -----------------------------------------------------------

/// Degraded topology entry: the window privately copied shared authority state without disclosure — the window
/// reads as independent when it has quietly become the workspace authority.
fn topology_leaked() -> M5ResolvedWindowTopologyEntry {
    let mut base = clean_topology_base(
        "topology:primary:leaked",
        "window.main",
        "window.topology.primary",
        M5WindowRestoreRole::WindowTopology,
        M5WindowTopologySurface::PrimaryWindowTopology,
        M5WindowRestoreSurfaceContext::ShellSurface,
        "pane-tree.main.v4",
        "focus-history.window.main",
        "display-affinity.monitor-1",
    );
    base.authority_copied_into_window_used = true;
    base.authority_copy_disclosed = false;
    topology(base)
}

/// Degraded topology entry: the canonical / accessible / audit resolution-form coverage of the topology is
/// incomplete.
fn topology_form_incomplete() -> M5ResolvedWindowTopologyEntry {
    let mut base = clean_topology_base(
        "topology:auxiliary:form-incomplete",
        "window.secondary",
        "window.topology.auxiliary",
        M5WindowRestoreRole::DisplayAffinity,
        M5WindowTopologySurface::AuxiliaryWindowTopology,
        M5WindowRestoreSurfaceContext::RecoverySurface,
        "pane-tree.secondary.v2",
        "focus-history.window.secondary",
        "display-affinity.monitor-2",
    );
    base.resolution_form_coverage = vec![M5WindowStateResolutionForm::CanonicalObject];
    topology(base)
}

/// Degraded topology entry: the window-topology surface is unclassified.
fn topology_surface_unclassified() -> M5ResolvedWindowTopologyEntry {
    topology(clean_topology_base(
        "topology:admin:surface-unclassified",
        "window.third",
        "window.topology.unknown",
        M5WindowRestoreRole::WindowTopology,
        M5WindowTopologySurface::SurfaceUnclassified,
        M5WindowRestoreSurfaceContext::AdminSurface,
        "pane-tree.third.v1",
        "focus-history.window.third",
        "display-affinity.monitor-3",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5WorkspaceAuthorityWindowTopologyRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    workspace_authority_entries: Vec<M5ResolvedWorkspaceAuthorityEntry>,
    window_topology_entries: Vec<M5ResolvedWindowTopologyEntry>,
) -> M5WorkspaceAuthorityWindowTopologyRegistriesRow {
    M5WorkspaceAuthorityWindowTopologyRegistriesRow {
        consumer_surface,
        qualification: M5WindowRestoreQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5WindowRestoreDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5WindowRestoreRequiredLabel::Identity,
            M5WindowRestoreRequiredLabel::SemanticRole,
            M5WindowRestoreRequiredLabel::RegistryReference,
            M5WindowRestoreRequiredLabel::WorkspaceAuthority,
            M5WindowRestoreRequiredLabel::DisplayAffinity,
        ],
        accessibility_routes: M5WindowRestoreAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5WindowStateAnatomyPart::ALL.to_vec(),
        export_fields: M5WindowStateExportField::ALL.to_vec(),
        downgrade_triggers,
        workspace_authority_entries,
        window_topology_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_RESTORE_FIDELITY_SCHEMA_REF,
        ]),
        window_local_state_overwrites_shared_workspace_authority: false,
        shared_workspace_authority_becomes_private_window_state: false,
        merges_workspace_authority_and_window_topology_into_one_opaque_blob: false,
        dirty_buffer_state_drifts_across_windows_sharing_one_authority: false,
    }
}

fn registry_rows() -> Vec<M5WorkspaceAuthorityWindowTopologyRegistriesRow> {
    use M5WindowRestoreConsumerSurface as C;
    use M5WindowRestoreDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the per-window workspace authority to one stable object — backing windows, stable pane-tree IDs, shared dirty-buffer / save / checkpoint state, authoritative state root, and the distinct profile-defaults reference — from the shared registry and renders the primary window topology; an authority object missing a pane-tree ID and a window topology that privately copies shared authority state degrade honestly instead of reading as a clean pass",
            "evidence:m5-workspace-ownership-shell-ui:001",
            vec![
                D::WorkspaceAuthorityUnstated,
                D::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
                D::ProofStale,
            ],
            vec![authority_shell_single_clean(), authority_object_incomplete()],
            vec![topology_primary_shell_clean(), topology_leaked()],
        ),
        base_row(
            C::RestoreCoordinator,
            "Restore-coordinator owner",
            "The restore coordinator resolves one shared workspace authority backing multiple windows while selection and focus stay window-local, and rebuilds the auxiliary window topology; a resolution-form gap on an authority entry and on a window topology is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-workspace-ownership-restore-coordinator:001",
            vec![
                D::RegistryReferenceUnstated,
                D::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
                D::ProofStale,
            ],
            vec![authority_recovery_multi_clean(), authority_form_incomplete()],
            vec![
                topology_auxiliary_recovery_clean(),
                topology_form_incomplete(),
            ],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the detached / auxiliary window sharing the workspace authority and the diagnostics window topology without manual reconstruction; a window-local selection that overwrites the shared authority is caught as a window-local overwrite for its scope",
            "evidence:m5-workspace-ownership-diagnostics:001",
            vec![
                D::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
                D::WindowTopologyBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                authority_diagnostics_detached_clean(),
                authority_window_local_overwrite(),
            ],
            vec![topology_diagnostics_clean()],
        ),
        base_row(
            C::WorkspaceService,
            "Workspace-service owner",
            "The workspace service resolves the multi-window shared authority object while keeping it bound to the registry; an authority that is a hand-copied per-window assumption and a window topology on an unclassified surface degrade honestly",
            "evidence:m5-workspace-ownership-workspace-service:001",
            vec![
                D::WindowTopologyBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![authority_admin_multi_clean(), authority_unbound()],
            vec![
                topology_auxiliary_admin_clean(),
                topology_surface_unclassified(),
            ],
        ),
        base_row(
            C::SessionService,
            "Session-service owner",
            "The session service renders the same resolved workspace-authority and window-topology truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied ownership table",
            "evidence:m5-workspace-ownership-session-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::WindowTopologyBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                authority_diagnostics_detached_clean(),
                authority_form_incomplete(),
            ],
            vec![
                topology_auxiliary_recovery_clean(),
                topology_form_incomplete(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved workspace-authority and window-topology truth, so a hand-copied constant, an unstated registry token, a window-local overwrite, or a privately-copied authority is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-workspace-ownership-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
                D::ProofStale,
            ],
            vec![authority_support_single_clean(), authority_token_unstated()],
            vec![topology_primary_support_clean()],
        ),
    ]
}

fn governance_review() -> M5WorkspaceAuthorityWindowTopologyRegistriesGovernanceReview {
    M5WorkspaceAuthorityWindowTopologyRegistriesGovernanceReview {
        authority_registry_names_token_role_and_scope: true,
        workspace_resolves_to_stable_object_from_shared_registry: true,
        backing_windows_pane_ids_and_shared_state_published: true,
        window_local_selection_and_focus_stay_window_local: true,
        window_topology_keeps_shared_authority_distinct: true,
        shared_authority_never_becomes_private_window_state: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shell_recovery_diagnostics_admin_read_single_source: true,
        authority_or_topology_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5WorkspaceAuthorityWindowTopologyRegistriesConsumerProjection {
    M5WorkspaceAuthorityWindowTopologyRegistriesConsumerProjection {
        shell_and_recovery_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        session_and_workspace_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5WorkspaceAuthorityWindowTopologyRegistriesProofFreshness {
    M5WorkspaceAuthorityWindowTopologyRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WorkspaceAuthorityWindowTopologyRegistriesReleasePosture {
    M5WorkspaceAuthorityWindowTopologyRegistriesReleasePosture {
        proof_packet_ref: M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_ARTIFACT_REF.to_owned(),
        window_restore_audit_ref: M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_REF,
        M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 workspace-authority and window-topology registries packet.
pub fn seeded_m5_workspace_authority_and_window_topology_registries(
) -> M5WorkspaceAuthorityWindowTopologyRegistriesPacket {
    M5WorkspaceAuthorityWindowTopologyRegistriesPacket::new(
        M5WorkspaceAuthorityWindowTopologyRegistriesPacketInput {
            packet_id: M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 workspace-authority and window-topology registries with one stable workspace-authority object resolving per workspace, window-local selection and focus staying window-local while one authority backs multiple windows, shared dirty-buffer / save / checkpoint state kept distinct from the profile-defaults reference, canonical / accessible / audit resolution-form coverage, and the window-local pane-tree / focus-history / display-affinity disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5WorkspaceAuthorityWindowTopologyRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the restore-coordinator row is held at Beta pending multi-window shared-authority parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed(
) -> M5WorkspaceAuthorityWindowTopologyRegistriesPacket {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.packet_id =
        "m5-workspace-authority-and-window-topology-registries:multi-window-shared-authority-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .expect("restore-coordinator row present");
    row.qualification = M5WindowRestoreQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending detached / auxiliary window-topology
/// parity on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed(
) -> M5WorkspaceAuthorityWindowTopologyRegistriesPacket {
    let mut packet = seeded_m5_workspace_authority_and_window_topology_registries();
    packet.packet_id =
        "m5-workspace-authority-and-window-topology-registries:auxiliary-window-topology-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5WindowRestoreQualificationClass::Preview;
    packet
}
