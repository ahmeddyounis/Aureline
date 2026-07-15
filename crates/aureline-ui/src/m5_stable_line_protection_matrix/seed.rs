//! Canonical seed builders for the frozen M5 stable-line-protection matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical stable-line-protection matrix.
pub const M5_STABLE_LINE_PROTECTION_MATRIX_PACKET_ID: &str =
    "m5-stable-line-protection:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every line must be able to show.
fn mandatory_labels() -> Vec<M5StableLineProtectionRequiredLabel> {
    M5StableLineProtectionRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a line carries.
fn labels_with(
    extra: &[M5StableLineProtectionRequiredLabel],
) -> Vec<M5StableLineProtectionRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every line filled in and every line-specific vocabulary left empty
/// for the caller to populate.
fn base_row(
    line_class: M5StableLineProtectionLine,
    qualification: M5StableLineProtectionQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5StableLineProtectionRow {
    M5StableLineProtectionRow {
        line_class,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5StableLineProtectionSurfaceFamily::ALL.to_vec(),
        widening_stages: M5StableLineProtectionWideningStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        fresh_stable_line_roles: vec![],
        evidence_refresh_line_roles: vec![],
        correction_backport_line_roles: vec![],
        bundle_currentness_line_roles: vec![],
        lts_candidate_line_roles: vec![],
        degraded_reasons: M5StableLineProtectionDegradedReason::ALL.to_vec(),
        accessibility_routes: M5StableLineProtectionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5StableLineProtectionConsumerSurface::SupportExport,
            M5StableLineProtectionConsumerSurface::DocsHelp,
        ],
        downgrade_triggers: vec![M5StableLineProtectionDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        widens_support_language_without_current_refresh_and_correction_evidence: false,
        drifts_a_shipping_line_on_stale_evidence_or_frozen_launch_bundles: false,
        relies_on_tribal_backport_memory_instead_of_a_documented_correction_packet: false,
        claims_lts_eligibility_without_current_rollback_and_support_evidence: false,
        leaves_a_supported_line_defect_unowned_or_unresolved_past_its_sla: false,
    }
}

fn stable_line_protection_rows() -> Vec<M5StableLineProtectionRow> {
    use M5StableLineProtectionConsumerSurface as C;
    use M5StableLineProtectionDowngradeTrigger as D;
    use M5StableLineProtectionLine as F;
    use M5StableLineProtectionQualificationClass as Q;
    use M5StableLineProtectionRequiredLabel as L;
    use M5StableLineProtectionRole as R;

    let mut rows = Vec::new();

    // 1. Fresh stable line (first 30 days after stable).
    let mut row = base_row(
        F::FreshStableLine,
        Q::Stable,
        "Stable-line release owner",
        "One fresh stable line naming the crash/rollback flow protected, the support-export flow protected, the migration flow protected, and the first-thirty-day watch active so the just-shipped stable line never drifts on stale evidence in its first month",
        "evidence:m5-fresh-stable-line-parity:001",
        &[
            M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
            M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF,
            M5_STABLE_CLAIM_MANIFEST_LANDED_SCHEMA_REF,
        ],
    );
    row.fresh_stable_line_roles = M5FreshStableLineRole::ALL.to_vec();
    row.semantic_roles = vec![R::SupportWindow, R::BundleCurrentness];
    row.required_labels = labels_with(&[L::SupportWindow]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
    ];
    row.downgrade_triggers = vec![
        D::WidenedSupportWithoutCurrentRefreshEvidence,
        D::SupportWindowUnstated,
        D::BundleCurrentnessUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Evidence-refresh line.
    let mut row = base_row(
        F::EvidenceRefreshLine,
        Q::Stable,
        "Evidence-refresh cadence owner",
        "One evidence-refresh line naming the certified-archetype evidence refreshed, the compatibility evidence refreshed, the known-limits evidence refreshed, and the refresh cadence kept ordinary release ops so support language never outruns current refresh proof",
        "evidence:m5-evidence-refresh-line-parity:001",
        &[
            M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
            M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF,
            M5_STABLE_CLAIM_MANIFEST_LANDED_SCHEMA_REF,
        ],
    );
    row.evidence_refresh_line_roles = M5EvidenceRefreshLineRole::ALL.to_vec();
    row.semantic_roles = vec![R::SupportWindow, R::DefectLedger];
    row.required_labels = labels_with(&[L::SupportWindow]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::PublicProof,
        C::Diagnostics,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::RanSupportLanguageAheadOfRefreshProof,
        D::LeftASupportedLineDefectUnownedPastSla,
        D::SupportWindowUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Correction / backport line.
    let mut row = base_row(
        F::CorrectionBackportLine,
        Q::Stable,
        "Correction-line owner",
        "One correction/backport line naming the correction path exercised, the backport decision recorded within SLA, the may-slip item shipped or narrowed, and the post-launch correction report published so no backport rests on tribal memory instead of a documented correction packet",
        "evidence:m5-correction-backport-line-parity:001",
        &[
            M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
            M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
            M5_RELEASE_CENTER_LANDED_SCHEMA_REF,
        ],
    );
    row.correction_backport_line_roles = M5CorrectionBackportLineRole::ALL.to_vec();
    row.semantic_roles = vec![R::BackportDecision, R::EvidenceRefresh];
    row.required_labels = labels_with(&[L::RefreshState]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ProgramGovernance,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::ReliedOnTribalBackportMemory,
        D::WidenedSupportWithoutCurrentCorrectionEvidence,
        D::RefreshStateUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Launch-bundle-currentness line.
    let mut row = base_row(
        F::BundleCurrentnessLine,
        Q::Stable,
        "Bundle-currentness owner",
        "One launch-bundle-currentness line naming the launch-bundle freshness rechecked, the bundle-refresh obligation met, the shipping-line bundle audited, and any frozen-bundle drift detected so the shipping line never ships a stale launch bundle without a currentness audit",
        "evidence:m5-bundle-currentness-line-parity:001",
        &[
            M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
            M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
            M5_RELEASE_CENTER_LANDED_SCHEMA_REF,
        ],
    );
    row.bundle_currentness_line_roles = M5BundleCurrentnessLineRole::ALL.to_vec();
    row.semantic_roles = vec![R::CorrectionOwnership, R::EvidenceRefresh];
    row.required_labels = labels_with(&[L::RefreshState]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::PublicProof,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::RanSupportLanguageAheadOfRefreshProof,
        D::WidenedSupportWithoutCurrentCorrectionEvidence,
        D::RefreshStateUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. LTS-candidate line.
    let mut row = base_row(
        F::LtsCandidateLine,
        Q::Stable,
        "LTS-readiness decision owner",
        "One LTS-candidate line naming the backport discipline demonstrated, the rollback discipline demonstrated, the LTS decision packet recorded, and the support-evidence snapshot preserved so LTS is never claimed without current rollback and support evidence and never reads as green while refresh or ledger state is stale",
        "evidence:m5-lts-candidate-line-parity:001",
        &[
            M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
            M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF,
            M5_STABLE_CLAIM_MANIFEST_LANDED_SCHEMA_REF,
        ],
    );
    row.lts_candidate_line_roles = M5LtsCandidateLineRole::ALL.to_vec();
    row.semantic_roles = vec![R::LtsEligibility, R::CorrectionOwnership];
    row.required_labels = labels_with(&[L::LtsPosture]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::ProgramGovernance,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::ImpliedGreenWhileRefreshOrLedgerWasStale,
        D::WidenedSupportWithoutCurrentRefreshEvidence,
        D::LtsPostureUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5StableLineProtectionGovernanceReview {
    M5StableLineProtectionGovernanceReview {
        no_shipping_line_drifts_on_stale_evidence: true,
        every_active_line_names_support_window_correction_owner_and_refresh_cadence: true,
        bundle_currentness_depends_on_current_refresh_audits: true,
        supported_line_defects_stay_owned_and_resolved_within_sla: true,
        evidence_refresh_cadence_is_ordinary_release_ops: true,
        first_correction_and_backport_path_exercised: true,
        lts_decisions_preserve_rollback_and_support_evidence_snapshot: true,
        backport_decisions_are_documented_not_tribal_memory: true,
        every_line_declares_widening_stages: true,
        every_line_declares_accessibility_route: true,
        support_export_reads_single_stable_line_source: true,
        release_help_and_support_bind_to_single_stable_line_source: true,
        later_rows_cannot_invent_parallel_stable_line_vocabulary: true,
        stable_line_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
        support_language_never_outruns_current_refresh_and_correction_proof: true,
    }
}

fn consumer_projection() -> M5StableLineProtectionConsumerProjection {
    M5StableLineProtectionConsumerProjection {
        release_and_help_consume_shared_stable_line_truth: true,
        support_and_public_proof_consume_shared_support_window_and_refresh_truth: true,
        diagnostics_and_cli_export_consume_shared_correction_and_bundle_truth: true,
        docs_help_and_screenshots_read_single_stable_line_source: true,
        lts_and_refresh_proofs_bind_to_shared_evidence_snapshot: true,
        support_export_reads_single_stable_line_source: true,
    }
}

fn proof_freshness() -> M5StableLineProtectionProofFreshness {
    M5StableLineProtectionProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StableLineProtectionReleasePosture {
    M5StableLineProtectionReleasePosture {
        proof_packet_ref: M5_STABLE_LINE_PROTECTION_ARTIFACT_REF.to_owned(),
        stable_line_protection_audit_ref: M5_STABLE_LINE_PROTECTION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STABLE_LINE_PROTECTION_MATRIX_SCHEMA_REF,
        M5_STABLE_LINE_PROTECTION_MATRIX_DOC_REF,
        M5_STABLE_LINE_REFRESH_POLICY_DOMAIN_SCHEMA_REF,
        M5_SUPPORTED_LINE_DEFECT_LEDGER_DOMAIN_SCHEMA_REF,
        M5_LTS_READINESS_DECISION_DOMAIN_SCHEMA_REF,
        M5_STABLE_CLAIM_MANIFEST_LANDED_SCHEMA_REF,
        M5_RELEASE_CENTER_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 stable-line-protection matrix packet.
pub fn seeded_m5_stable_line_protection_matrix() -> M5StableLineProtectionMatrixPacket {
    M5StableLineProtectionMatrixPacket::new(M5StableLineProtectionMatrixPacketInput {
        packet_id: M5_STABLE_LINE_PROTECTION_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 stable-line protection, evidence-refresh, correction-line, and LTS-readiness matrix"
                .to_owned(),
        stable_line_protection_rows: stable_line_protection_rows(),
        vocabulary_set: M5StableLineProtectionVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the launch-bundle-currentness line is held at Beta because a bundle-refresh audit is not
/// yet current on the shipping line; every line stays visible.
pub fn seeded_m5_stable_line_protection_matrix_bundle_currentness_beta_narrowed(
) -> M5StableLineProtectionMatrixPacket {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.packet_id = "m5-stable-line-protection:bundle-currentness-beta:0001".to_owned();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::BundleCurrentnessLine)
        .expect("bundle-currentness row present");
    row.qualification = M5StableLineProtectionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the LTS-candidate line is narrowed to Preview pending an LTS decision packet backed by
/// current rollback and support evidence; every line stays visible.
pub fn seeded_m5_stable_line_protection_matrix_lts_candidate_preview_narrowed(
) -> M5StableLineProtectionMatrixPacket {
    let mut packet = seeded_m5_stable_line_protection_matrix();
    packet.packet_id = "m5-stable-line-protection:lts-candidate-preview:0001".to_owned();
    let row = packet
        .stable_line_protection_rows
        .iter_mut()
        .find(|row| row.line_class == M5StableLineProtectionLine::LtsCandidateLine)
        .expect("lts-candidate row present");
    row.qualification = M5StableLineProtectionQualificationClass::Preview;
    packet
}
