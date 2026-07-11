//! Canonical seed builders for the frozen M5 workspace-trust-repair component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical workspace-trust-repair component matrix.
pub const M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-workspace-trust-repair-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5WorkspaceTrustRepairRequiredLabel> {
    M5WorkspaceTrustRepairRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(
    extra: &[M5WorkspaceTrustRepairRequiredLabel],
) -> Vec<M5WorkspaceTrustRepairRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5WorkspaceTrustRepairComponentFamily,
    qualification: M5WorkspaceTrustRepairQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5WorkspaceTrustRepairComponentRow {
    M5WorkspaceTrustRepairComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5WorkspaceTrustRepairSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkspaceTrustRepairDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: M5WorkspaceTrustRepairDisposition::ALL.to_vec(),
        grant_source_classes: vec![],
        trust_scope_states: vec![],
        capability_narrow_states: vec![],
        root_trust_states: vec![],
        reversal_classes: vec![],
        checkpoint_states: vec![],
        repair_outcomes: vec![],
        preview_states: vec![],
        degraded_reasons: M5WorkspaceTrustRepairDegradedReason::ALL.to_vec(),
        accessibility_routes: M5WorkspaceTrustRepairAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5WorkspaceTrustRepairConsumerSurface::SupportExport,
            M5WorkspaceTrustRepairConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5WorkspaceTrustRepairDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        implies_blanket_trust_across_roots_or_routes: false,
        hides_checkpoint_absence_or_reversal_limits: false,
        collapses_reversal_outcomes_into_generic_success: false,
        presents_partial_success_as_complete: false,
    }
}

fn component_rows() -> Vec<M5WorkspaceTrustRepairComponentRow> {
    use M5CapabilityNarrowState as CN;
    use M5RepairCheckpointState as CK;
    use M5RepairOutcomeClass as RO;
    use M5RepairPreviewState as PV;
    use M5RepairReversalClass as RC;
    use M5RootTrustState as RT;
    use M5TrustGrantSourceClass as GS;
    use M5TrustScopeState as TS;
    use M5WorkspaceTrustRepairComponentFamily as F;
    use M5WorkspaceTrustRepairConsumerSurface as C;
    use M5WorkspaceTrustRepairDisposition as BD;
    use M5WorkspaceTrustRepairDowngradeTrigger as D;
    use M5WorkspaceTrustRepairQualificationClass as Q;
    use M5WorkspaceTrustRepairRequiredLabel as L;

    let mut rows = Vec::new();

    // 1. Workspace-trust banner.
    let mut row = base_row(
        F::WorkspaceTrustBanner,
        Q::Stable,
        "Workspace trust owner",
        "One workspace-trust-banner model naming whether the workspace is trusted, restricted, or mixed-root, who granted the trust (user, inherited parent, policy, workspace config, or first-party default), and what capability is narrowed, so a restricted or mixed-root workspace never reads as blanket trust across roots",
        "evidence:m5-workspace-trust-banner-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
        ],
    );
    row.grant_source_classes = GS::ALL.to_vec();
    row.trust_scope_states = vec![
        TS::TrustedWorkspace,
        TS::RestrictedWorkspace,
        TS::MixedRoot,
        TS::PolicyBlocked,
    ];
    row.capability_narrow_states = vec![CN::FullCapability, CN::ReducedMode, CN::TaskBlocked];
    row.dispositions = vec![
        BD::Trusted,
        BD::Restricted,
        BD::MixedRoot,
        BD::PolicyBlocked,
        BD::ReducedMode,
    ];
    row.required_labels = labels_with(&[L::GrantSourceAndScope, L::CapabilityAndRootScope]);
    row.consumer_surfaces = vec![
        C::WorkspaceTrustUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::GrantSourceUnstated,
        D::PolicyEpochUnstated,
        D::RootScopeCollapsedIntoBlanketTrust,
        D::MixedRootShownAsUniformTrust,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Trust-fact grid.
    let mut row = base_row(
        F::TrustFactGrid,
        Q::Stable,
        "Workspace trust owner",
        "One trust-fact-grid model naming grant source and policy epoch, trusted object and root scope, narrowed capability, and per-root trust together in one place, so a user can read every trust fact about a workspace without hunting through menus",
        "evidence:m5-trust-fact-grid-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_TRUST_FACT_GRID_SCHEMA_REF,
        ],
    );
    row.grant_source_classes = GS::ALL.to_vec();
    row.trust_scope_states = TS::ALL.to_vec();
    row.capability_narrow_states = CN::ALL.to_vec();
    row.root_trust_states = vec![
        RT::RootTrusted,
        RT::RootRestricted,
        RT::RootInherited,
        RT::RootMixedChildren,
    ];
    row.dispositions = vec![
        BD::Trusted,
        BD::Restricted,
        BD::MixedRoot,
        BD::PolicyBlocked,
        BD::ReducedMode,
    ];
    row.required_labels = labels_with(&[L::GrantSourceAndScope, L::CapabilityAndRootScope]);
    row.consumer_surfaces = vec![
        C::WorkspaceTrustUi,
        C::SettingsUi,
        C::DoctorUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::GrantSourceUnstated,
        D::PolicyEpochUnstated,
        D::NarrowedCapabilityUnstated,
        D::MixedRootShownAsUniformTrust,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Trust-elevation sheet.
    let mut row = base_row(
        F::TrustElevationSheet,
        Q::Stable,
        "Workspace trust owner",
        "One trust-elevation-sheet model naming exactly what a trust elevation grants, its grant source and the scope it changes (workspace or single root), and that the elevation never implies blanket approval across every root or route",
        "evidence:m5-trust-elevation-sheet-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
        ],
    );
    row.grant_source_classes = vec![GS::UserExplicit, GS::PolicyManaged, GS::WorkspaceConfig];
    row.trust_scope_states = vec![
        TS::TrustedWorkspace,
        TS::TrustedRoot,
        TS::RestrictedWorkspace,
        TS::PolicyBlocked,
    ];
    row.dispositions = vec![BD::Trusted, BD::Restricted, BD::PolicyBlocked];
    row.required_labels = labels_with(&[L::GrantSourceAndScope]);
    row.consumer_surfaces = vec![
        C::WorkspaceTrustUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::GrantSourceUnstated,
        D::PolicyEpochUnstated,
        D::RootScopeCollapsedIntoBlanketTrust,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Restricted-capability row.
    let mut row = base_row(
        F::RestrictedCapabilityRow,
        Q::Stable,
        "Restricted mode owner",
        "One restricted-capability-row model naming exactly which capability is narrowed (reduced mode, a blocked task, blocked execution, or a blocked extension) and the trust scope that narrowed it, so a narrowed capability is always named rather than left as a vague reduced experience",
        "evidence:m5-restricted-capability-row-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
        ],
    );
    row.trust_scope_states = vec![
        TS::RestrictedWorkspace,
        TS::MixedRoot,
        TS::PolicyBlocked,
        TS::TrustedRoot,
    ];
    row.capability_narrow_states = CN::ALL.to_vec();
    row.dispositions = vec![BD::Restricted, BD::ReducedMode, BD::PolicyBlocked];
    row.required_labels = labels_with(&[L::CapabilityAndRootScope]);
    row.consumer_surfaces = vec![
        C::SafeModeUi,
        C::ExtensionsUi,
        C::DoctorUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::NarrowedCapabilityUnstated,
        D::RootScopeCollapsedIntoBlanketTrust,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Root-trust strip.
    let mut row = base_row(
        F::RootTrustStrip,
        Q::Stable,
        "Workspace trust owner",
        "One root-trust-strip model naming the trust of each root in a multi-root workspace (trusted, restricted, inherited, policy-blocked, or mixed children) plus the grant source, so mixed-root trust never collapses into one uniform trust badge",
        "evidence:m5-root-trust-strip-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_ROOT_TRUST_STRIP_SCHEMA_REF,
        ],
    );
    row.grant_source_classes = vec![
        GS::UserExplicit,
        GS::InheritedParent,
        GS::PolicyManaged,
        GS::WorkspaceConfig,
    ];
    row.root_trust_states = RT::ALL.to_vec();
    row.dispositions = vec![
        BD::Trusted,
        BD::Restricted,
        BD::MixedRoot,
        BD::PolicyBlocked,
    ];
    row.required_labels = labels_with(&[L::GrantSourceAndScope, L::CapabilityAndRootScope]);
    row.consumer_surfaces = vec![
        C::WorkspaceTrustUi,
        C::SafeModeUi,
        C::RemoteUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::MixedRootShownAsUniformTrust,
        D::RootScopeCollapsedIntoBlanketTrust,
        D::GrantSourceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Repair-transaction preview card.
    let mut row = base_row(
        F::RepairTransactionPreviewCard,
        Q::Stable,
        "Guided repair owner",
        "One repair-transaction-preview-card model naming the repair candidate ids that will be mutated, checkpoint availability, and the reversal class before anything is applied, so a repair preview never hides checkpoint absence and a user always reviews what a repair will mutate and how it reverses",
        "evidence:m5-repair-transaction-preview-card-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
        ],
    );
    row.reversal_classes = vec![
        RC::ExactReversal,
        RC::CompensatingReversal,
        RC::RegenerateReversal,
        RC::ManualFollowUp,
        RC::AuditOnly,
    ];
    row.checkpoint_states = vec![
        CK::CheckpointAvailable,
        CK::CheckpointPartial,
        CK::CheckpointMissing,
        CK::CheckpointExpired,
    ];
    row.preview_states = PV::ALL.to_vec();
    row.dispositions = vec![
        BD::PreviewReady,
        BD::CheckpointMissing,
        BD::ExactReversal,
        BD::Compensate,
        BD::Regenerate,
        BD::ManualFollowUp,
    ];
    row.required_labels = labels_with(&[L::ReversalAndCheckpoint]);
    row.consumer_surfaces = vec![
        C::DoctorUi,
        C::AiContextUi,
        C::RemoteUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CheckpointAbsenceHidden,
        D::ReversalLimitHidden,
        D::RepairTargetIdsUnstated,
        D::ReversalClassCollapsedIntoGenericSuccess,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Rollback-class strip.
    let mut row = base_row(
        F::RollbackClassStrip,
        Q::Stable,
        "Guided repair owner",
        "One rollback-class-strip model naming the reversal class (exact, compensate, regenerate, manual follow-up, or audit-only) and checkpoint availability, so exact and compensating and regenerating and manual and audit-only reversals never collapse into one generic undo",
        "evidence:m5-rollback-class-strip-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
        ],
    );
    row.reversal_classes = RC::ALL.to_vec();
    row.checkpoint_states = CK::ALL.to_vec();
    row.dispositions = vec![
        BD::ExactReversal,
        BD::Compensate,
        BD::Regenerate,
        BD::ManualFollowUp,
        BD::AuditOnly,
        BD::CheckpointMissing,
    ];
    row.required_labels = labels_with(&[L::ReversalAndCheckpoint]);
    row.consumer_surfaces = vec![C::DoctorUi, C::SafeModeUi, C::SupportExport, C::ProductUi];
    row.downgrade_triggers = vec![
        D::ReversalClassCollapsedIntoGenericSuccess,
        D::ReversalLimitHidden,
        D::CheckpointAbsenceHidden,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Repair-result receipt row.
    let mut row = base_row(
        F::RepairResultReceiptRow,
        Q::Stable,
        "Guided repair owner",
        "One repair-result-receipt-row model naming the applied outcome (applied exact, compensated, regenerated, partial success, manual required, or failed), the reversal class, and any manual follow-up, so a partial success is never shown as a complete success",
        "evidence:m5-repair-result-receipt-row-parity:001",
        &[
            M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
            M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
        ],
    );
    row.reversal_classes = vec![
        RC::ExactReversal,
        RC::CompensatingReversal,
        RC::RegenerateReversal,
        RC::ManualFollowUp,
        RC::AuditOnly,
    ];
    row.repair_outcomes = RO::ALL.to_vec();
    row.dispositions = vec![
        BD::ExactReversal,
        BD::Compensate,
        BD::Regenerate,
        BD::ManualFollowUp,
        BD::AuditOnly,
    ];
    row.required_labels = labels_with(&[L::ReversalAndCheckpoint]);
    row.consumer_surfaces = vec![C::DoctorUi, C::AiContextUi, C::SupportExport, C::ProductUi];
    row.downgrade_triggers = vec![
        D::PartialSuccessShownAsComplete,
        D::ReversalClassCollapsedIntoGenericSuccess,
        D::RepairTargetIdsUnstated,
        D::GenericChromeWordingUsed,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5WorkspaceTrustRepairGovernanceReview {
    M5WorkspaceTrustRepairGovernanceReview {
        workspace_trust_banner_shows_grant_source_and_scope: true,
        trust_fact_grid_shows_grant_scope_capability_root_together: true,
        trust_elevation_sheet_shows_grant_source_and_scope_change: true,
        restricted_capability_row_shows_narrowed_capability: true,
        root_trust_strip_shows_per_root_trust: true,
        repair_transaction_preview_card_shows_targets_checkpoint_reversal: true,
        rollback_class_strip_shows_reversal_class_and_checkpoint: true,
        repair_result_receipt_row_shows_outcome_and_followup: true,
        no_trust_surface_implies_blanket_approval: true,
        grant_source_and_policy_epoch_always_explicit: true,
        checkpoint_absence_never_hidden: true,
        reversal_outcomes_never_collapsed_into_generic_success: true,
        partial_success_never_shown_as_complete: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_trust_repair_vocabulary: true,
    }
}

fn consumer_projection() -> M5WorkspaceTrustRepairConsumerProjection {
    M5WorkspaceTrustRepairConsumerProjection {
        trust_surfaces_consume_grant_source_vocabulary: true,
        settings_and_doctor_consume_capability_narrow_vocabulary: true,
        safe_mode_consumes_root_trust_vocabulary: true,
        repair_surfaces_consume_reversal_class_vocabulary: true,
        guided_repair_consumes_checkpoint_vocabulary: true,
        support_export_reads_single_trust_repair_source: true,
    }
}

fn proof_freshness() -> M5WorkspaceTrustRepairProofFreshness {
    M5WorkspaceTrustRepairProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WorkspaceTrustRepairReleasePosture {
    M5WorkspaceTrustRepairReleasePosture {
        proof_packet_ref: M5_WORKSPACE_TRUST_REPAIR_COMPONENT_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_WORKSPACE_TRUST_REPAIR_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
        M5_TRUST_FACT_GRID_SCHEMA_REF,
        M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
        M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
        M5_ROOT_TRUST_STRIP_SCHEMA_REF,
        M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
        M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
        M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 workspace-trust-repair component matrix packet.
pub fn seeded_m5_workspace_trust_repair_component_matrix(
) -> M5WorkspaceTrustRepairComponentMatrixPacket {
    M5WorkspaceTrustRepairComponentMatrixPacket::new(M5WorkspaceTrustRepairComponentMatrixPacketInput {
        packet_id: M5_WORKSPACE_TRUST_REPAIR_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 workspace-trust-banner, trust-fact-grid, trust-elevation-sheet, restricted-capability-row, root-trust-strip, repair-transaction-preview-card, rollback-class-strip, and repair-result-receipt-row component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5WorkspaceTrustRepairVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the trust-elevation sheet is held at Beta because policy-managed elevation
/// scope round-trips are not yet proven across every deployment line; every component stays visible.
pub fn seeded_m5_workspace_trust_repair_component_matrix_trust_elevation_sheet_beta_narrowed(
) -> M5WorkspaceTrustRepairComponentMatrixPacket {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.packet_id =
        "m5-workspace-trust-repair-components:trust-elevation-sheet-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5WorkspaceTrustRepairComponentFamily::TrustElevationSheet
        })
        .expect("trust-elevation-sheet row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Beta;
    packet
}

/// Narrowed variant: the repair-transaction preview card is narrowed to Preview pending
/// checkpoint-availability parity across every deployment line; every component stays visible.
pub fn seeded_m5_workspace_trust_repair_component_matrix_repair_transaction_preview_card_preview_narrowed(
) -> M5WorkspaceTrustRepairComponentMatrixPacket {
    let mut packet = seeded_m5_workspace_trust_repair_component_matrix();
    packet.packet_id =
        "m5-workspace-trust-repair-components:repair-transaction-preview-card-preview:0001"
            .to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family
                == M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard
        })
        .expect("repair-transaction-preview-card row present");
    row.qualification = M5WorkspaceTrustRepairQualificationClass::Preview;
    packet
}
