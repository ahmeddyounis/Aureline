//! Canonical seed builders for the frozen M5 install-topology matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical install-topology matrix.
pub const M5_INSTALL_TOPOLOGY_MATRIX_PACKET_ID: &str = "m5-install-topology:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5InstallTopologyRequiredLabel> {
    M5InstallTopologyRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(extra: &[M5InstallTopologyRequiredLabel]) -> Vec<M5InstallTopologyRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    install_topology_family: M5InstallTopologyFamily,
    qualification: M5InstallTopologyQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5InstallTopologyRow {
    M5InstallTopologyRow {
        install_topology_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5InstallTopologySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5InstallTopologyDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        per_user_managed_install_roles: vec![],
        per_machine_managed_install_roles: vec![],
        side_by_side_channel_roles: vec![],
        portable_mode_roles: vec![],
        offline_airgap_bundle_roles: vec![],
        degraded_reasons: M5InstallTopologyDegradedReason::ALL.to_vec(),
        accessibility_routes: M5InstallTopologyAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5InstallTopologyConsumerSurface::SupportExport,
            M5InstallTopologyConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5InstallTopologyDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        portable_mode_writes_hidden_machine_global_durable_state: false,
        preview_channel_reuses_stable_state_namespace_without_handoff: false,
        rollback_targets_primary_executable_while_sidecars_drift: false,
        hides_updater_ownership_or_admin_control_in_managed_flow: false,
        publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence: false,
    }
}

fn install_topology_rows() -> Vec<M5InstallTopologyRow> {
    use M5InstallTopologyConsumerSurface as C;
    use M5InstallTopologyDowngradeTrigger as D;
    use M5InstallTopologyFamily as F;
    use M5InstallTopologyQualificationClass as Q;
    use M5InstallTopologyRequiredLabel as L;
    use M5InstallTopologyRole as R;

    let mut rows = Vec::new();

    // 1. Per-user managed install.
    let mut row = base_row(
        F::PerUserManaged,
        Q::Stable,
        "Updater and install-topology owner",
        "One per-user managed install profile naming the user-scoped binary root, per-user updater ownership, user-writable state root, and user-scoped policy root so binary placement and updater ownership stay inspectable and durable state never spills into hidden machine-global paths",
        "evidence:m5-per-user-managed-topology-parity:001",
        &[
            M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.per_user_managed_install_roles = M5PerUserManagedInstallRole::ALL.to_vec();
    row.semantic_roles = vec![R::InstallMode, R::UpdaterOwner, R::BinaryRoot];
    row.required_labels = labels_with(&[L::InstallMode]);
    row.consumer_surfaces = vec![
        C::UpdaterService,
        C::ShellUi,
        C::Diagnostics,
        C::Installer,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
        D::InstallModeUnstated,
        D::UpdaterOwnerUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Per-machine managed install.
    let mut row = base_row(
        F::PerMachineManaged,
        Q::Stable,
        "Managed-deployment and admin owner",
        "One per-machine managed install profile naming the machine-scoped binary root, admin-owned updater, shared machine state root, and machine policy root so updater ownership and admin control are never hidden in managed flows and every writable state root is inspectable",
        "evidence:m5-per-machine-managed-topology-parity:001",
        &[
            M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.per_machine_managed_install_roles = M5PerMachineManagedInstallRole::ALL.to_vec();
    row.semantic_roles = vec![R::InstallMode, R::UpdaterOwner, R::PolicyRoots];
    row.required_labels = labels_with(&[L::InstallMode, L::StateRoot]);
    row.consumer_surfaces = vec![
        C::UpdaterService,
        C::Admin,
        C::Diagnostics,
        C::Installer,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
        D::InstallModeUnstated,
        D::UpdaterOwnerUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Side-by-side stable plus preview.
    let mut row = base_row(
        F::SideBySideStablePreview,
        Q::Stable,
        "Channel-coexistence owner",
        "One side-by-side channel profile naming the isolated channel binary root, isolated channel state namespace, explicit cross-channel handoff, and per-channel rollback target so stable and preview channels never corrupt one another and no preview channel reuses a stable state namespace without an explicit import or handoff",
        "evidence:m5-side-by-side-channel-parity:001",
        &[
            M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
            M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
            M5_COEXISTENCE_AND_FLEET_ROLLOUT_SCHEMA_REF,
        ],
    );
    row.side_by_side_channel_roles = M5SideBySideChannelRole::ALL.to_vec();
    row.semantic_roles = vec![R::InstallMode, R::WritableStateRoots, R::RolloutRing];
    row.required_labels = labels_with(&[L::InstallMode, L::StateRoot]);
    row.consumer_surfaces = vec![
        C::UpdaterService,
        C::ShellUi,
        C::Admin,
        C::Diagnostics,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
        D::StateRootBoundaryDriftedByTopology,
        D::StateRootUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Portable mode.
    let mut row = base_row(
        F::PortableMode,
        Q::Stable,
        "Portable-mode owner",
        "One portable-mode profile naming the self-contained binary root, colocated writable state root, no-machine-global-spill guarantee, and disclosed portable limitations so portable mode never writes hidden machine-global durable settings, secrets, or services and every limitation is disclosed",
        "evidence:m5-portable-mode-parity:001",
        &[
            M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
            M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
            M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
        ],
    );
    row.portable_mode_roles = M5PortableModeRole::ALL.to_vec();
    row.semantic_roles = vec![R::BinaryRoot, R::WritableStateRoots];
    row.required_labels = labels_with(&[L::InstallMode, L::StateRoot]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::Diagnostics,
        C::Installer,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PortableModeWroteHiddenMachineGlobalDurableState,
        D::StateRootBoundaryDriftedByTopology,
        D::StateRootUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Offline / air-gap bundle.
    let mut row = base_row(
        F::OfflineAirgapBundle,
        Q::Stable,
        "Offline / air-gap delivery owner",
        "One offline / air-gap bundle profile naming the bundled artifact root, offline updater ownership, bundled policy root, and complete rollback-target set so a rollback restores the full artifact graph rather than only the primary executable and no undisclosed network dependency hides in an air-gapped deployment",
        "evidence:m5-offline-airgap-bundle-parity:001",
        &[
            M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
            M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
            M5_COEXISTENCE_AND_FLEET_ROLLOUT_SCHEMA_REF,
        ],
    );
    row.offline_airgap_bundle_roles = M5OfflineAirgapBundleRole::ALL.to_vec();
    row.semantic_roles = vec![R::PolicyRoots, R::RollbackTarget, R::RolloutRing];
    row.required_labels = labels_with(&[L::InstallMode, L::RollbackTarget]);
    row.consumer_surfaces = vec![
        C::UpdaterService,
        C::Admin,
        C::Diagnostics,
        C::Installer,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
        D::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
        D::RollbackTargetUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5InstallTopologyGovernanceReview {
    M5InstallTopologyGovernanceReview {
        binary_placement_and_updater_ownership_inspectable: true,
        portable_mode_never_spills_machine_global_durable_state: true,
        stable_and_preview_channels_never_corrupt_one_another: true,
        silent_and_managed_flows_preserve_diagnostics_and_repair_verify: true,
        rollback_targets_full_artifact_graph_not_just_primary_executable: true,
        rollout_rings_keep_promotion_and_rollback_evidence: true,
        updater_ownership_never_hidden_in_managed_flow: true,
        preview_channel_requires_explicit_import_or_handoff: true,
        deployment_claims_never_outpace_ring_or_repair_verify_evidence: true,
        every_family_declares_deployment_lines: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_install_topology_source: true,
        about_update_diagnostics_admin_bind_to_single_install_topology_source: true,
        later_rows_cannot_invent_parallel_install_vocabulary: true,
        install_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5InstallTopologyConsumerProjection {
    M5InstallTopologyConsumerProjection {
        about_and_update_consume_shared_install_topology_truth: true,
        diagnostics_and_admin_consume_shared_state_root_boundaries: true,
        installers_consume_shared_binary_and_state_roots: true,
        docs_help_and_screenshots_read_single_install_topology_source: true,
        rollout_tooling_binds_to_shared_ring_evidence: true,
        support_export_reads_single_install_topology_source: true,
    }
}

fn proof_freshness() -> M5InstallTopologyProofFreshness {
    M5InstallTopologyProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5InstallTopologyReleasePosture {
    M5InstallTopologyReleasePosture {
        proof_packet_ref: M5_INSTALL_TOPOLOGY_ARTIFACT_REF.to_owned(),
        install_topology_audit_ref: M5_INSTALL_TOPOLOGY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
        M5_COEXISTENCE_AND_FLEET_ROLLOUT_SCHEMA_REF,
        M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 install-topology matrix packet.
pub fn seeded_m5_install_topology_matrix() -> M5InstallTopologyMatrixPacket {
    M5InstallTopologyMatrixPacket::new(M5InstallTopologyMatrixPacketInput {
        packet_id: M5_INSTALL_TOPOLOGY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 install-topology, mutable-state-boundary, portable-update, and fleet-rollout execution matrix"
                .to_owned(),
        install_topology_rows: install_topology_rows(),
        vocabulary_set: M5InstallTopologyVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: side-by-side stable-plus-preview is held at Beta because channel coexistence is not
/// yet proven across every deployment line; every family stays visible.
pub fn seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed(
) -> M5InstallTopologyMatrixPacket {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.packet_id = "m5-install-topology:side-by-side-channel-beta:0001".to_owned();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::SideBySideStablePreview)
        .expect("side-by-side row present");
    row.qualification = M5InstallTopologyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the offline / air-gap bundle is narrowed to Preview pending complete rollback and
/// rollout-ring evidence across every deployment line; every family stays visible.
pub fn seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed(
) -> M5InstallTopologyMatrixPacket {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.packet_id = "m5-install-topology:offline-airgap-bundle-preview:0001".to_owned();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::OfflineAirgapBundle)
        .expect("offline / air-gap row present");
    row.qualification = M5InstallTopologyQualificationClass::Preview;
    packet
}
