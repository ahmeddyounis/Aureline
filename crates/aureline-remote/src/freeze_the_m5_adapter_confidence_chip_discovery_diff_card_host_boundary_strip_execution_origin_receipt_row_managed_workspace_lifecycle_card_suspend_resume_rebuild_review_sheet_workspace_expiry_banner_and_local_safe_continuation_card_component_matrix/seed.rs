//! Canonical seed builders for the frozen M5 build/remote-boundary component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical build/remote-boundary component matrix.
pub const M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-build-remote-boundary-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5BuildRemoteRequiredLabel> {
    M5BuildRemoteRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5BuildRemoteRequiredLabel]) -> Vec<M5BuildRemoteRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5BuildRemoteBoundaryComponentFamily,
    qualification: M5BuildRemoteQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5BuildRemoteBoundaryComponentRow {
    M5BuildRemoteBoundaryComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BuildRemoteSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5BuildRemoteDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        boundary_dispositions: M5BuildRemoteBoundaryDisposition::ALL.to_vec(),
        adapter_confidences: vec![],
        discovery_confidences: vec![],
        host_kinds: vec![],
        origin_loci: vec![],
        lifecycle_states: vec![],
        persistence_classes: vec![],
        continuity_classes: vec![],
        expiry_classes: vec![],
        degraded_reasons: M5BuildRemoteDegradedReason::ALL.to_vec(),
        accessibility_routes: M5BuildRemoteAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5BuildRemoteConsumerSurface::SupportExport,
            M5BuildRemoteConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5BuildRemoteDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        implies_exact_continuity_after_material_change: false,
        hides_local_safe_or_companion_handoff_in_overflow_only: false,
        lower_confidence_overwrites_resolved_target_without_review: false,
    }
}

fn component_rows() -> Vec<M5BuildRemoteBoundaryComponentRow> {
    use AdapterConfidence as AC;
    use DiscoveryConfidence as DC;
    use HostKind as HK;
    use M5BuildRemoteBoundaryComponentFamily as F;
    use M5BuildRemoteBoundaryDisposition as BD;
    use M5BuildRemoteConsumerSurface as C;
    use M5BuildRemoteDowngradeTrigger as D;
    use M5BuildRemoteQualificationClass as Q;
    use M5BuildRemoteRequiredLabel as L;
    use OriginLocus as OL;

    let mut rows = Vec::new();

    // 1. Adapter-confidence chip.
    let mut row = base_row(
        F::AdapterConfidenceChip,
        Q::Stable,
        "Build intelligence owner",
        "One adapter-confidence-chip model naming the build/runtime adapter's confidence in the resolved target (verified, high, heuristic, or unverified) and the claim ceiling it permits, so a heuristic guess is never presented with the certainty of a verified target",
        "evidence:m5-adapter-confidence-chip-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
            M5_BUILD_AND_HOST_GOVERNANCE_PATH,
        ],
    );
    row.adapter_confidences = AC::ALL.to_vec();
    row.boundary_dispositions = vec![
        BD::LocalExecution,
        BD::SshExecution,
        BD::ContainerExecution,
        BD::DevcontainerExecution,
        BD::ManagedWorkspace,
        BD::ServicePlane,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::ConfidenceAndDiscovery]);
    row.consumer_surfaces = vec![
        C::RunTestDebugUi,
        C::NotebookUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::AdapterConfidenceUnstated,
        D::LowerConfidenceOverwroteResolvedTarget,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Discovery-diff card.
    let mut row = base_row(
        F::DiscoveryDiffCard,
        Q::Stable,
        "Target discovery owner",
        "One discovery-diff-card model naming the discovery confidence (exact, structured, imported, heuristic, or unresolved) and any heuristic-vs-resolved drift, so a lower-confidence rediscovery never silently overwrites a higher-confidence resolved target without an explicit review state",
        "evidence:m5-discovery-diff-card-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
            M5_TARGET_DISCOVERY_PATH,
        ],
    );
    row.discovery_confidences = DC::ALL.to_vec();
    row.boundary_dispositions = vec![
        BD::LocalExecution,
        BD::SshExecution,
        BD::ContainerExecution,
        BD::DevcontainerExecution,
        BD::ManagedWorkspace,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::ConfidenceAndDiscovery]);
    row.consumer_surfaces = vec![
        C::RunTestDebugUi,
        C::NotebookUi,
        C::PreviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DiscoveryDriftHidden,
        D::LowerConfidenceOverwroteResolvedTarget,
        D::AdapterConfidenceUnstated,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Host-boundary strip.
    let mut row = base_row(
        F::HostBoundaryStrip,
        Q::Stable,
        "Host boundary owner",
        "One host-boundary-strip model naming which host kind the work ran on (local, SSH, container, managed workspace, browser bridge, or service plane), so a remote, managed, bridged, or service-plane host is never mistaken for local execution",
        "evidence:m5-host-boundary-strip-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
            M5_HOST_BOUNDARY_PATH,
        ],
    );
    row.host_kinds = HK::ALL.to_vec();
    row.boundary_dispositions = vec![
        BD::LocalExecution,
        BD::SshExecution,
        BD::ContainerExecution,
        BD::DevcontainerExecution,
        BD::ManagedWorkspace,
        BD::BrowserBridge,
        BD::ServicePlane,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::HostAndExecutionOrigin]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::RunTestDebugUi,
        C::PreviewUi,
        C::IncidentUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HostBoundaryUnstated,
        D::ExecutionOriginUnstated,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Execution-origin receipt row.
    let mut row = base_row(
        F::ExecutionOriginReceiptRow,
        Q::Stable,
        "Execution context owner",
        "One execution-origin-receipt-row model naming the origin locus where the work actually ran (local, remote, managed, bridged, or service plane) as a receipt-backed fact, so an exported receipt's locus never disagrees with the host the work ran on",
        "evidence:m5-execution-origin-receipt-row-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
            M5_HOST_BOUNDARY_PATH,
        ],
    );
    row.origin_loci = OL::ALL.to_vec();
    row.boundary_dispositions = vec![
        BD::LocalExecution,
        BD::SshExecution,
        BD::ContainerExecution,
        BD::ManagedWorkspace,
        BD::BrowserBridge,
        BD::ServicePlane,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::HostAndExecutionOrigin]);
    row.consumer_surfaces = vec![
        C::RunTestDebugUi,
        C::NotebookUi,
        C::IncidentUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExecutionOriginUnstated,
        D::HostBoundaryUnstated,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Managed-workspace lifecycle card.
    let mut row = base_row(
        F::ManagedWorkspaceLifecycleCard,
        Q::Stable,
        "Managed workspace owner",
        "One managed-workspace-lifecycle-card model naming the workspace lifecycle state (provision, warm, ready, suspended, resumed, reconnecting, rebuild-required, recreate-required, expired, or local-safe continuation), so a provisioned workspace's lifecycle is a first-class reviewed fact rather than feature-local status copy",
        "evidence:m5-managed-workspace-lifecycle-card-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
            MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
        ],
    );
    row.lifecycle_states = BOUND_LIFECYCLE_STATES.to_vec();
    row.boundary_dispositions = vec![
        BD::ManagedWorkspace,
        BD::Suspended,
        BD::Rebuilt,
        BD::Recreated,
        BD::Expired,
        BD::LocalSafeContinuation,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::LifecycleAndContinuity]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::PreviewUi,
        C::CompanionUi,
        C::IncidentUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LifecycleStateUnstated,
        D::ExactContinuityOverclaimed,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Suspend / resume / rebuild review sheet.
    let mut row = base_row(
        F::SuspendResumeRebuildReviewSheet,
        Q::Stable,
        "Managed workspace owner",
        "One suspend-resume-rebuild-review-sheet model naming the lifecycle transition, the changed persistence class, and the claimed continuity relative to the prior runtime, so a resume that lands on a rebuilt, recreated, or snapshot-restored runtime never implies exact continuity over a material change",
        "evidence:m5-suspend-resume-rebuild-review-sheet-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
            MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
        ],
    );
    row.lifecycle_states = vec![
        LifecycleStateClass::Suspended,
        LifecycleStateClass::Resumed,
        LifecycleStateClass::RebuildRequired,
        LifecycleStateClass::RecreateRequired,
        LifecycleStateClass::Expired,
        LifecycleStateClass::LocalSafeContinuation,
    ];
    row.persistence_classes = BOUND_PERSISTENCE_CLASSES.to_vec();
    row.continuity_classes = BOUND_CONTINUITY_CLASSES.to_vec();
    row.boundary_dispositions = vec![
        BD::ManagedWorkspace,
        BD::Suspended,
        BD::Rebuilt,
        BD::Recreated,
        BD::Expired,
        BD::LocalSafeContinuation,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::LifecycleAndContinuity]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::PreviewUi,
        C::CompanionUi,
        C::IncidentUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::PersistenceChangeHidden,
        D::ExactContinuityOverclaimed,
        D::LifecycleStateUnstated,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Workspace-expiry banner.
    let mut row = base_row(
        F::WorkspaceExpiryBanner,
        Q::Stable,
        "Managed workspace owner",
        "One workspace-expiry-banner model naming the expiry timing that governs the workspace (no window, idle window, hibernation window, hard deadline, or control-plane outage clock), so a user learns a workspace is about to expire before context is lost",
        "evidence:m5-workspace-expiry-banner-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
            MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
        ],
    );
    row.expiry_classes = BOUND_EXPIRY_CLASSES.to_vec();
    row.boundary_dispositions = vec![
        BD::ManagedWorkspace,
        BD::Suspended,
        BD::Expired,
        BD::LocalSafeContinuation,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::LifecycleAndContinuity]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::PreviewUi,
        C::CompanionUi,
        C::IncidentUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ExpiryTimingUnstated,
        D::LifecycleStateUnstated,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Local-safe continuation card.
    let mut row = base_row(
        F::LocalSafeContinuationCard,
        Q::Stable,
        "Continuity owner",
        "One local-safe-continuation-card model naming the continuity class offered when managed continuity is unavailable (exact, material change, fresh no-continuity, or local-safe only) and keeping local-safe continuation and companion handoff first-class, so the escape hatch is never hidden behind overflow-only affordances",
        "evidence:m5-local-safe-continuation-card-parity:001",
        &[
            M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
            M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
            MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
        ],
    );
    row.continuity_classes = BOUND_CONTINUITY_CLASSES.to_vec();
    row.boundary_dispositions = vec![
        BD::LocalSafeContinuation,
        BD::Rebuilt,
        BD::Recreated,
        BD::Expired,
        BD::ManagedWorkspace,
        BD::NotEvaluated,
    ];
    row.required_labels = labels_with(&[L::LifecycleAndContinuity]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::PreviewUi,
        C::CompanionUi,
        C::IncidentUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LocalSafeOrCompanionHandoffOverflowOnly,
        D::ExactContinuityOverclaimed,
        D::GenericStatusWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5BuildRemoteBoundaryComponentGovernanceReview {
    M5BuildRemoteBoundaryComponentGovernanceReview {
        adapter_confidence_chip_names_confidence_and_ceiling: true,
        discovery_diff_card_shows_drift_and_review_state: true,
        host_boundary_strip_names_host_kind: true,
        execution_origin_receipt_row_names_origin_locus: true,
        managed_workspace_lifecycle_card_names_lifecycle_state: true,
        suspend_resume_rebuild_review_sheet_names_continuity_and_persistence: true,
        workspace_expiry_banner_names_expiry_timing: true,
        local_safe_continuation_card_names_local_safe_continuation: true,
        no_card_implies_exact_continuity_after_material_change: true,
        host_ownership_and_execution_origin_always_explicit: true,
        discovery_confidence_and_drift_always_explicit: true,
        local_safe_and_companion_handoff_never_overflow_only: true,
        lower_confidence_never_overwrites_resolved_without_review: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_build_remote_vocabulary: true,
    }
}

fn consumer_projection() -> M5BuildRemoteBoundaryComponentConsumerProjection {
    M5BuildRemoteBoundaryComponentConsumerProjection {
        run_test_debug_surfaces_consume_confidence_vocabulary: true,
        remote_and_preview_surfaces_consume_host_and_origin_vocabulary: true,
        managed_workspace_surfaces_consume_lifecycle_vocabulary: true,
        companion_surfaces_consume_continuity_vocabulary: true,
        incident_surfaces_consume_expiry_vocabulary: true,
        support_export_reads_single_build_remote_source: true,
    }
}

fn proof_freshness() -> M5BuildRemoteBoundaryComponentProofFreshness {
    M5BuildRemoteBoundaryComponentProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BuildRemoteBoundaryComponentReleasePosture {
    M5BuildRemoteBoundaryComponentReleasePosture {
        proof_packet_ref: M5_BUILD_REMOTE_BOUNDARY_COMPONENT_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_BUILD_REMOTE_BOUNDARY_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_ADAPTER_CONFIDENCE_CHIP_SCHEMA_REF,
        M5_DISCOVERY_DIFF_CARD_SCHEMA_REF,
        M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
        M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
        M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
        M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
        M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
        M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
        M5_BUILD_AND_HOST_GOVERNANCE_PATH,
        M5_HOST_BOUNDARY_PATH,
        M5_TARGET_DISCOVERY_PATH,
        MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
    ])
}

/// Builds the canonical frozen M5 build/remote-boundary component matrix packet.
pub fn seeded_m5_build_remote_boundary_component_matrix(
) -> M5BuildRemoteBoundaryComponentMatrixPacket {
    M5BuildRemoteBoundaryComponentMatrixPacket::new(M5BuildRemoteBoundaryComponentMatrixPacketInput {
        packet_id: M5_BUILD_REMOTE_BOUNDARY_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 adapter-confidence-chip, discovery-diff-card, host-boundary-strip, execution-origin-receipt-row, managed-workspace-lifecycle-card, suspend-resume-rebuild-review-sheet, workspace-expiry-banner, and local-safe-continuation-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5BuildRemoteBoundaryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the adapter-confidence chip is held at Beta because verified-vs-heuristic
/// confidence round-trips are not yet proven across every deployment line; every component stays
/// visible.
pub fn seeded_m5_build_remote_boundary_component_matrix_adapter_confidence_chip_beta_narrowed(
) -> M5BuildRemoteBoundaryComponentMatrixPacket {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.packet_id =
        "m5-build-remote-boundary-components:adapter-confidence-chip-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip
        })
        .expect("adapter-confidence-chip row present");
    row.qualification = M5BuildRemoteQualificationClass::Beta;
    packet
}

/// Narrowed variant: the suspend/resume/rebuild review sheet is narrowed to Preview pending
/// continuity-vs-persistence parity on every surface; every component stays visible.
pub fn seeded_m5_build_remote_boundary_component_matrix_suspend_resume_rebuild_review_sheet_preview_narrowed(
) -> M5BuildRemoteBoundaryComponentMatrixPacket {
    let mut packet = seeded_m5_build_remote_boundary_component_matrix();
    packet.packet_id =
        "m5-build-remote-boundary-components:suspend-resume-rebuild-review-sheet-preview:0001"
            .to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5BuildRemoteBoundaryComponentFamily::SuspendResumeRebuildReviewSheet
        })
        .expect("suspend-resume-rebuild-review-sheet row present");
    row.qualification = M5BuildRemoteQualificationClass::Preview;
    packet
}
