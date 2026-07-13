//! Canonical seed builders for the M5 install-topology and state-root registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean install-topology and state-root-boundary
//! entries are built so the stable install-topology object resolving per delivery profile, the explicit
//! shared-versus-isolated state namespaces across managed / user / side-by-side scopes, the full-graph
//! rollback and disclosed spill, the canonical / accessible / audit resolution forms, and the
//! writable-state-root / policy-root / rollback-target disclosure triple are proven across the About, update,
//! diagnostics, admin, installer, docs, CLI, and support surfaces without any hand-copied per-profile
//! assumption, reused state namespace, incomplete object, hidden spill, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_PACKET_ID: &str =
    "m5-install-topology-and-state-root-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn install(input: M5InstallTopologyEntryResolutionInput) -> M5ResolvedInstallTopologyEntry {
    resolve_install_topology_entry(input).expect("seed install-topology entry resolves")
}

fn boundary(input: M5StateRootBoundaryEntryResolutionInput) -> M5ResolvedStateRootBoundaryEntry {
    resolve_state_root_boundary_entry(input).expect("seed state-root-boundary entry resolves")
}

fn all_forms() -> Vec<M5InstallStateResolutionForm> {
    M5InstallStateResolutionForm::ALL.to_vec()
}

// -- Clean install-topology entries (stable object, isolated namespaces, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_install_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    delivery_scope: M5DeliveryScope,
    surface_context: M5InstallSurfaceContext,
    channel: &str,
    updater_owner: &str,
    binary_root: &str,
    writable_state_roots: &str,
    policy_roots: &str,
    rollback_target: &str,
) -> M5InstallTopologyEntryResolutionInput {
    M5InstallTopologyEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        delivery_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        channel: channel.to_owned(),
        updater_owner: updater_owner.to_owned(),
        binary_root: binary_root.to_owned(),
        writable_state_roots: writable_state_roots.to_owned(),
        policy_roots: policy_roots.to_owned(),
        rollback_target: rollback_target.to_owned(),
        bound_to_registry: true,
        state_namespaces_isolated: true,
        coexists_with_sibling_channel: false,
        coexistence_handoff_explained: true,
        proof_fresh: true,
    }
}

fn install_per_user_about_clean() -> M5ResolvedInstallTopologyEntry {
    install(clean_install_base(
        "install:per-user:about",
        "profile.per_user_managed",
        "install.topology.per_user",
        M5InstallTopologyRole::InstallMode,
        M5DeliveryScope::PerUserManagedScope,
        M5InstallSurfaceContext::AboutSurface,
        "stable",
        "per_user_updater",
        r"%LOCALAPPDATA%\Aureline\app",
        r"%LOCALAPPDATA%\Aureline\state",
        r"%LOCALAPPDATA%\Aureline\policy",
        "artifact-graph:per-user:stable",
    ))
}

fn install_per_machine_update_clean() -> M5ResolvedInstallTopologyEntry {
    install(clean_install_base(
        "install:per-machine:update",
        "profile.per_machine_managed",
        "install.topology.per_machine",
        M5InstallTopologyRole::UpdaterOwner,
        M5DeliveryScope::PerMachineManagedScope,
        M5InstallSurfaceContext::UpdateFlow,
        "stable",
        "admin_owned_updater",
        r"C:\Program Files\Aureline",
        r"C:\ProgramData\Aureline\state",
        r"C:\ProgramData\Aureline\policy",
        "artifact-graph:per-machine:stable",
    ))
}

fn install_side_by_side_diagnostics_clean() -> M5ResolvedInstallTopologyEntry {
    // A side-by-side preview channel that isolates its state namespace and explains its handoff stays clean.
    let mut base = clean_install_base(
        "install:side-by-side:diagnostics",
        "profile.side_by_side_preview",
        "install.topology.side_by_side",
        M5InstallTopologyRole::WritableStateRoots,
        M5DeliveryScope::SideBySideChannelScope,
        M5InstallSurfaceContext::DiagnosticsSurface,
        "preview",
        "per_user_updater",
        "~/Applications/Aureline Preview.app",
        "~/Library/Application Support/Aureline Preview",
        "~/Library/Application Support/Aureline Preview/policy",
        "artifact-graph:side-by-side:preview",
    );
    base.coexists_with_sibling_channel = true;
    base.coexistence_handoff_explained = true;
    install(base)
}

fn install_per_machine_admin_clean() -> M5ResolvedInstallTopologyEntry {
    install(clean_install_base(
        "install:per-machine:admin",
        "profile.per_machine_managed",
        "install.topology.per_machine",
        M5InstallTopologyRole::PolicyRoots,
        M5DeliveryScope::PerMachineManagedScope,
        M5InstallSurfaceContext::AdminSurface,
        "stable",
        "admin_owned_updater",
        "/opt/aureline",
        "/var/lib/aureline/state",
        "/etc/aureline/policy",
        "artifact-graph:per-machine:managed",
    ))
}

fn install_per_user_support_clean() -> M5ResolvedInstallTopologyEntry {
    install(clean_install_base(
        "install:per-user:support",
        "profile.per_user_managed",
        "install.topology.per_user",
        M5InstallTopologyRole::RollbackTarget,
        M5DeliveryScope::PerUserManagedScope,
        M5InstallSurfaceContext::SupportOrExportForm,
        "stable",
        "per_user_updater",
        "~/.local/share/aureline/app",
        "~/.config/aureline",
        "~/.config/aureline/policy",
        "artifact-graph:per-user:full",
    ))
}

// -- Degraded install-topology entries ----------------------------------------------------------

/// Degraded install entry: the behavior is a hand-copied per-profile assumption instead of tracing to the
/// registry (or hides updater ownership in a managed flow).
fn install_unbound() -> M5ResolvedInstallTopologyEntry {
    let mut base = clean_install_base(
        "install:per-machine:unbound",
        "profile.per_machine_managed",
        "install.topology.per_machine",
        M5InstallTopologyRole::UpdaterOwner,
        M5DeliveryScope::PerMachineManagedScope,
        M5InstallSurfaceContext::AdminSurface,
        "stable",
        "admin_owned_updater",
        r"C:\Program Files\Aureline",
        r"C:\ProgramData\Aureline\state",
        r"C:\ProgramData\Aureline\policy",
        "artifact-graph:per-machine:stable",
    );
    base.bound_to_registry = false;
    install(base)
}

/// Degraded install entry: the resolved install-topology object is incomplete — the updater owner is unstated.
fn install_object_incomplete() -> M5ResolvedInstallTopologyEntry {
    let mut base = clean_install_base(
        "install:per-user:incomplete",
        "profile.per_user_managed",
        "install.topology.per_user",
        M5InstallTopologyRole::InstallMode,
        M5DeliveryScope::PerUserManagedScope,
        M5InstallSurfaceContext::AboutSurface,
        "stable",
        "per_user_updater",
        r"%LOCALAPPDATA%\Aureline\app",
        r"%LOCALAPPDATA%\Aureline\state",
        r"%LOCALAPPDATA%\Aureline\policy",
        "artifact-graph:per-user:stable",
    );
    base.updater_owner = "   ".to_owned();
    install(base)
}

/// Degraded install entry: a side-by-side preview channel reused the stable state namespace without an
/// explicit handoff.
fn install_namespace_reused() -> M5ResolvedInstallTopologyEntry {
    let mut base = clean_install_base(
        "install:side-by-side:namespace-reused",
        "profile.side_by_side_preview",
        "install.topology.side_by_side",
        M5InstallTopologyRole::WritableStateRoots,
        M5DeliveryScope::SideBySideChannelScope,
        M5InstallSurfaceContext::DiagnosticsSurface,
        "preview",
        "per_user_updater",
        "~/Applications/Aureline Preview.app",
        "~/Library/Application Support/Aureline",
        "~/Library/Application Support/Aureline/policy",
        "artifact-graph:side-by-side:preview",
    );
    base.state_namespaces_isolated = false;
    install(base)
}

/// Degraded install entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn install_form_incomplete() -> M5ResolvedInstallTopologyEntry {
    let mut base = clean_install_base(
        "install:per-user:form-incomplete",
        "profile.per_user_managed",
        "install.topology.per_user",
        M5InstallTopologyRole::InstallMode,
        M5DeliveryScope::PerUserManagedScope,
        M5InstallSurfaceContext::UpdateFlow,
        "stable",
        "per_user_updater",
        r"%LOCALAPPDATA%\Aureline\app",
        r"%LOCALAPPDATA%\Aureline\state",
        r"%LOCALAPPDATA%\Aureline\policy",
        "artifact-graph:per-user:stable",
    );
    base.resolution_form_coverage = vec![M5InstallStateResolutionForm::CanonicalObject];
    install(base)
}

/// Degraded install entry: the canonical registry token name is unstated.
fn install_token_unstated() -> M5ResolvedInstallTopologyEntry {
    let mut base = clean_install_base(
        "install:support:token-unstated",
        "profile.per_user_managed",
        "  ",
        M5InstallTopologyRole::InstallMode,
        M5DeliveryScope::PerUserManagedScope,
        M5InstallSurfaceContext::SupportOrExportForm,
        "stable",
        "per_user_updater",
        "~/.local/share/aureline/app",
        "~/.config/aureline",
        "~/.config/aureline/policy",
        "artifact-graph:per-user:full",
    );
    base.token_name = "  ".to_owned();
    install(base)
}

// -- Clean state-root-boundary entries ----------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_boundary_base(
    entry_id: &str,
    profile_id: &str,
    token_name: &str,
    semantic_role: M5InstallTopologyRole,
    state_root_surface: M5StateRootSurface,
    surface_context: M5InstallSurfaceContext,
    writable_state_roots: &str,
    policy_roots: &str,
    rollback_target: &str,
) -> M5StateRootBoundaryEntryResolutionInput {
    M5StateRootBoundaryEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        profile_id: profile_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        state_root_surface,
        surface_context,
        resolution_form_coverage: all_forms(),
        writable_state_roots: writable_state_roots.to_owned(),
        policy_roots: policy_roots.to_owned(),
        rollback_target: rollback_target.to_owned(),
        rollback_targets_full_graph: true,
        boundary_is_truthful: true,
        machine_global_spill_used: false,
        machine_global_spill_disclosed: false,
        narrower_scope_asserted: false,
        narrower_scope_explained: false,
        proof_fresh: true,
    }
}

fn boundary_portable_clean() -> M5ResolvedStateRootBoundaryEntry {
    boundary(clean_boundary_base(
        "boundary:portable:colocated",
        "profile.portable_mode",
        "state.root.portable",
        M5InstallTopologyRole::WritableStateRoots,
        M5StateRootSurface::PortableModeBoundary,
        M5InstallSurfaceContext::AboutSurface,
        r".\AurelinePortable\state",
        r".\AurelinePortable\policy",
        "artifact-graph:portable:full",
    ))
}

fn boundary_offline_clean() -> M5ResolvedStateRootBoundaryEntry {
    // A disclosed machine-global write on an offline bundle stays clean: it is surfaced honestly.
    let mut base = clean_boundary_base(
        "boundary:offline:bundled",
        "profile.offline_airgap",
        "state.root.offline",
        M5InstallTopologyRole::PolicyRoots,
        M5StateRootSurface::OfflineAirgapBoundary,
        M5InstallSurfaceContext::UpdateFlow,
        "/opt/aureline-offline/state",
        "/opt/aureline-offline/policy",
        "artifact-graph:offline:bundled-full",
    );
    base.machine_global_spill_used = true;
    base.machine_global_spill_disclosed = true;
    boundary(base)
}

fn boundary_diagnostics_clean() -> M5ResolvedStateRootBoundaryEntry {
    // A justified narrower-scope assertion stays clean: it is explained on this diagnostics surface.
    let mut base = clean_boundary_base(
        "boundary:diagnostics:inspection",
        "profile.portable_mode",
        "state.root.diagnostics",
        M5InstallTopologyRole::RollbackTarget,
        M5StateRootSurface::DiagnosticsInspectionBoundary,
        M5InstallSurfaceContext::DiagnosticsSurface,
        r".\AurelinePortable\state",
        r".\AurelinePortable\policy",
        "artifact-graph:portable:full",
    );
    base.narrower_scope_asserted = true;
    base.narrower_scope_explained = true;
    boundary(base)
}

fn boundary_admin_clean() -> M5ResolvedStateRootBoundaryEntry {
    boundary(clean_boundary_base(
        "boundary:offline:admin",
        "profile.offline_airgap",
        "state.root.offline",
        M5InstallTopologyRole::WritableStateRoots,
        M5StateRootSurface::OfflineAirgapBoundary,
        M5InstallSurfaceContext::AdminSurface,
        "/opt/aureline-offline/state",
        "/opt/aureline-offline/policy",
        "artifact-graph:offline:bundled-full",
    ))
}

fn boundary_support_clean() -> M5ResolvedStateRootBoundaryEntry {
    boundary(clean_boundary_base(
        "boundary:portable:support",
        "profile.portable_mode",
        "state.root.portable",
        M5InstallTopologyRole::RollbackTarget,
        M5StateRootSurface::PortableModeBoundary,
        M5InstallSurfaceContext::SupportOrExportForm,
        r".\AurelinePortable\state",
        r".\AurelinePortable\policy",
        "artifact-graph:portable:full",
    ))
}

// -- Degraded state-root-boundary entries -------------------------------------------------------

/// Degraded boundary entry: portable mode wrote hidden machine-global durable state (spill undisclosed) — the
/// boundary reads as isolated when it is not.
fn boundary_untruthful() -> M5ResolvedStateRootBoundaryEntry {
    let mut base = clean_boundary_base(
        "boundary:portable:hidden-spill",
        "profile.portable_mode",
        "state.root.portable",
        M5InstallTopologyRole::WritableStateRoots,
        M5StateRootSurface::PortableModeBoundary,
        M5InstallSurfaceContext::AboutSurface,
        r".\AurelinePortable\state",
        r".\AurelinePortable\policy",
        "artifact-graph:portable:full",
    );
    base.machine_global_spill_used = true;
    base.machine_global_spill_disclosed = false;
    boundary(base)
}

/// Degraded boundary entry: the canonical / accessible / audit resolution-form coverage of the boundary is
/// incomplete.
fn boundary_form_incomplete() -> M5ResolvedStateRootBoundaryEntry {
    let mut base = clean_boundary_base(
        "boundary:diagnostics:form-incomplete",
        "profile.offline_airgap",
        "state.root.offline",
        M5InstallTopologyRole::PolicyRoots,
        M5StateRootSurface::DiagnosticsInspectionBoundary,
        M5InstallSurfaceContext::DiagnosticsSurface,
        "/opt/aureline-offline/state",
        "/opt/aureline-offline/policy",
        "artifact-graph:offline:bundled-full",
    );
    base.resolution_form_coverage = vec![M5InstallStateResolutionForm::CanonicalObject];
    boundary(base)
}

/// Degraded boundary entry: the state-root surface is unclassified.
fn boundary_surface_unclassified() -> M5ResolvedStateRootBoundaryEntry {
    boundary(clean_boundary_base(
        "boundary:offline:surface-unclassified",
        "profile.offline_airgap",
        "state.root.unknown",
        M5InstallTopologyRole::WritableStateRoots,
        M5StateRootSurface::SurfaceUnclassified,
        M5InstallSurfaceContext::AdminSurface,
        "/opt/aureline-offline/state",
        "/opt/aureline-offline/policy",
        "artifact-graph:offline:bundled-full",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5InstallTopologyStateRootRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    install_topology_entries: Vec<M5ResolvedInstallTopologyEntry>,
    state_root_boundary_entries: Vec<M5ResolvedStateRootBoundaryEntry>,
) -> M5InstallTopologyStateRootRegistriesRow {
    M5InstallTopologyStateRootRegistriesRow {
        consumer_surface,
        qualification: M5InstallTopologyQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5InstallTopologyDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5InstallTopologyRequiredLabel::Identity,
            M5InstallTopologyRequiredLabel::SemanticRole,
            M5InstallTopologyRequiredLabel::RegistryReference,
            M5InstallTopologyRequiredLabel::InstallMode,
            M5InstallTopologyRequiredLabel::StateRoot,
        ],
        accessibility_routes: M5InstallTopologyAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5InstallStateAnatomyPart::ALL.to_vec(),
        export_fields: M5InstallStateExportField::ALL.to_vec(),
        downgrade_triggers,
        install_topology_entries,
        state_root_boundary_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
        ]),
        portable_mode_writes_hidden_machine_global_durable_state: false,
        preview_channel_reuses_stable_state_namespace_without_handoff: false,
        rollback_targets_primary_executable_while_sidecars_drift: false,
        hides_updater_ownership_or_admin_control_in_managed_flow: false,
    }
}

fn registry_rows() -> Vec<M5InstallTopologyStateRootRegistriesRow> {
    use M5InstallTopologyConsumerSurface as C;
    use M5InstallTopologyDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell/About surface owner",
            "About resolves the per-user managed install to one stable topology object — install mode, channel, updater owner, binary root, state roots, policy roots, rollback target — from the shared registry and inspects the portable state-root boundary; a hand-copied per-profile assumption and a portable boundary that hides a machine-global spill degrade honestly instead of reading as a clean pass",
            "evidence:m5-install-state-shell-ui:001",
            vec![
                D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
                D::PortableModeWroteHiddenMachineGlobalDurableState,
                D::ProofStale,
            ],
            vec![install_per_user_about_clean(), install_object_incomplete()],
            vec![boundary_portable_clean(), boundary_untruthful()],
        ),
        base_row(
            C::UpdaterService,
            "Updater/update-flow owner",
            "The update flow resolves the per-machine managed install object and the offline / air-gap state-root boundary; a resolution-form gap on an install entry and on a state-root boundary is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-install-state-updater:001",
            vec![
                D::RegistryReferenceUnstated,
                D::PortableModeWroteHiddenMachineGlobalDurableState,
                D::ProofStale,
            ],
            vec![install_per_machine_update_clean(), install_form_incomplete()],
            vec![boundary_offline_clean(), boundary_form_incomplete()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the side-by-side stable-plus-preview install topology and the portable state-root boundary without manual reconstruction; a preview channel that reuses the stable state namespace without an explicit handoff is caught as unisolated for its scope",
            "evidence:m5-install-state-diagnostics:001",
            vec![
                D::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
                D::StateRootBoundaryDriftedByTopology,
                D::ProofStale,
            ],
            vec![
                install_side_by_side_diagnostics_clean(),
                install_namespace_reused(),
            ],
            vec![boundary_diagnostics_clean()],
        ),
        base_row(
            C::Admin,
            "Admin surface owner",
            "Admin resolves the per-machine managed install object while preserving updater ownership and admin control; a topology that hides updater ownership in a managed flow and a state-root boundary on an unclassified surface degrade honestly",
            "evidence:m5-install-state-admin:001",
            vec![
                D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![install_per_machine_admin_clean(), install_unbound()],
            vec![boundary_admin_clean(), boundary_surface_unclassified()],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the same resolved install-topology and state-root boundary truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied path table",
            "evidence:m5-install-state-docs-help:001",
            vec![
                D::RegistryReferenceUnstated,
                D::StateRootBoundaryDriftedByTopology,
                D::ProofStale,
            ],
            vec![install_side_by_side_diagnostics_clean(), install_form_incomplete()],
            vec![boundary_offline_clean(), boundary_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved install-topology and state-root boundary truth, so a hand-copied constant, an unstated registry token, a reused state namespace, or a hidden machine-global spill is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-install-state-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
                D::ProofStale,
            ],
            vec![install_per_user_support_clean(), install_token_unstated()],
            vec![boundary_support_clean()],
        ),
    ]
}

fn governance_review() -> M5InstallTopologyStateRootRegistriesGovernanceReview {
    M5InstallTopologyStateRootRegistriesGovernanceReview {
        topology_registry_names_token_role_and_scope: true,
        profile_resolves_to_stable_object_from_shared_registry: true,
        install_mode_owner_roots_and_rollback_published: true,
        managed_and_user_scopes_and_channels_isolated: true,
        state_root_boundaries_truthful_and_complete: true,
        portable_mode_never_spills_and_rollback_full_graph: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        about_update_diagnostics_admin_read_single_source: true,
        topology_or_boundary_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5InstallTopologyStateRootRegistriesConsumerProjection {
    M5InstallTopologyStateRootRegistriesConsumerProjection {
        about_and_update_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        installers_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5InstallTopologyStateRootRegistriesProofFreshness {
    M5InstallTopologyStateRootRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5InstallTopologyStateRootRegistriesReleasePosture {
    M5InstallTopologyStateRootRegistriesReleasePosture {
        proof_packet_ref: M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_ARTIFACT_REF.to_owned(),
        install_topology_audit_ref: M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 install-topology and state-root registries packet.
pub fn seeded_m5_install_topology_and_state_root_registries(
) -> M5InstallTopologyStateRootRegistriesPacket {
    M5InstallTopologyStateRootRegistriesPacket::new(M5InstallTopologyStateRootRegistriesPacketInput {
        packet_id: M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 install-topology and state-root registries with one stable install-topology object resolving per delivery profile, explicit shared-versus-isolated state namespaces across managed / user / side-by-side scopes, full-graph rollback and disclosed spill, canonical / accessible / audit resolution-form coverage, and the writable-state-root / policy-root / rollback-target disclosure triple across About, update, diagnostics, admin, docs, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5InstallTopologyStateRootRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the diagnostics row is held at Beta pending side-by-side channel state-namespace parity
/// on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_install_topology_and_state_root_registries_side_by_side_channel_beta_narrowed(
) -> M5InstallTopologyStateRootRegistriesPacket {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.packet_id =
        "m5-install-topology-and-state-root-registries:side-by-side-channel-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5InstallTopologyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the updater row is narrowed to Preview pending offline / air-gap state-root parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_install_topology_and_state_root_registries_offline_airgap_bundle_preview_narrowed(
) -> M5InstallTopologyStateRootRegistriesPacket {
    let mut packet = seeded_m5_install_topology_and_state_root_registries();
    packet.packet_id =
        "m5-install-topology-and-state-root-registries:offline-airgap-bundle-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5InstallTopologyConsumerSurface::UpdaterService)
        .expect("updater-service row present");
    row.qualification = M5InstallTopologyQualificationClass::Preview;
    packet
}
