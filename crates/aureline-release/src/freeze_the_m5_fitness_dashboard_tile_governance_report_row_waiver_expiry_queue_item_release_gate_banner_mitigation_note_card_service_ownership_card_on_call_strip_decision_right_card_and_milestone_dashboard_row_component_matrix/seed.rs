//! Canonical seed builders for the frozen M5 fitness-dashboard-tile,
//! governance-report-row, waiver-expiry-queue-item, release-gate-banner,
//! mitigation-note-card, service-ownership-card, on-call-strip, decision-right-card,
//! and milestone-dashboard-row component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The gated artifact generator and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical governance-dashboard-component matrix.
pub const M5_GOVERNANCE_DASHBOARD_MATRIX_PACKET_ID: &str =
    "m5-governance-dashboard-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5GovernanceRequiredLabel> {
    M5GovernanceRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5GovernanceRequiredLabel]) -> Vec<M5GovernanceRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5GovernanceDashboardComponentFamily,
    qualification: M5GovernanceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_schema_ref: &str,
) -> M5GovernanceDashboardComponentRow {
    M5GovernanceDashboardComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5GovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        readiness_states: M5GovernanceReadinessState::ALL.to_vec(),
        fitness_provenance_classes: vec![],
        report_scopes: vec![],
        waiver_expiry_states: vec![],
        release_gate_decisions: vec![],
        mitigation_postures: vec![],
        ownership_coverage_states: vec![],
        on_call_coverage_states: vec![],
        escalation_route_classes: vec![],
        decision_forum_classes: vec![],
        decision_right_states: vec![],
        milestone_gate_states: vec![],
        accessibility_routes: M5GovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5GovernanceConsumerSurface::AssuranceDashboard,
            M5GovernanceConsumerSurface::SupportExport,
            M5GovernanceConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5GovernanceDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[M5_GOVERNANCE_DASHBOARD_SCHEMA_REF, component_schema_ref]),
        renders_waived_or_stale_as_clean_pass: false,
        lets_ownerless_or_forumless_blocker_read_resolved: false,
        hides_mitigation_behind_internal_jargon: false,
        invents_private_governance_status_grammar: false,
    }
}

fn component_rows() -> Vec<M5GovernanceDashboardComponentRow> {
    use M5DecisionForumClass as DF;
    use M5DecisionRightState as DR;
    use M5EscalationRouteClass as ER;
    use M5FitnessProvenanceClass as FP;
    use M5GovernanceConsumerSurface as C;
    use M5GovernanceDashboardComponentFamily as F;
    use M5GovernanceDowngradeTrigger as D;
    use M5GovernanceQualificationClass as Q;
    use M5GovernanceReportScope as RSc;
    use M5GovernanceRequiredLabel as L;
    use M5MilestoneGateState as MG;
    use M5MitigationPosture as MP;
    use M5OnCallCoverageState as OC;
    use M5OwnershipCoverageState as OW;
    use M5ReleaseGateDecision as RG;
    use M5WaiverExpiryState as WE;

    let mut rows = Vec::new();

    // 1. Fitness dashboard tile.
    let mut row = base_row(
        F::FitnessDashboardTile,
        Q::Stable,
        "Fitness-dashboard component owner",
        "One fitness-dashboard-tile model carrying a fitness-function reading and the corpus/profile provenance behind it — canonical corpus, pinned profile, sampled, synthetic, or unknown — so a passing tile is never shown while its evidence is stale or its provenance is unknown",
        "evidence:m5-fitness-dashboard-tile-parity:001",
        M5_FITNESS_DASHBOARD_TILE_SCHEMA_REF,
    );
    row.fitness_provenance_classes = FP::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::AssuranceDashboard,
        C::OperatorBoard,
        C::ServiceHealth,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::FitnessProvenanceUnstated,
        D::EvidenceStaleHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Governance report row.
    let mut row = base_row(
        F::GovernanceReportRow,
        Q::Stable,
        "Governance-report component owner",
        "One governance-report-row model naming the report scope — service, family, train, fleet, or waiver ledger — and the lane's readiness with its evidence, so a blocked or evidence-stale lane is never summarized as clear",
        "evidence:m5-governance-report-row-parity:001",
        M5_GOVERNANCE_REPORT_ROW_SCHEMA_REF,
    );
    row.report_scopes = RSc::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::AssuranceDashboard,
        C::ShiproomPacket,
        C::DocsPortal,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::EvidenceStaleHidden, D::ProofStale];
    rows.push(row);

    // 3. Waiver-expiry queue item.
    let mut row = base_row(
        F::WaiverExpiryQueueItem,
        Q::Stable,
        "Waiver-ledger component owner",
        "One waiver-expiry-queue-item model naming the waiver's lifecycle — active, expiring soon, expired, revoked, or none — and when it lapses, so an expired or revoked waiver is never shown as still holding a blocker",
        "evidence:m5-waiver-expiry-queue-item-parity:001",
        M5_WAIVER_EXPIRY_QUEUE_ITEM_SCHEMA_REF,
    );
    row.waiver_expiry_states = WE::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::AssuranceDashboard,
        C::ShiproomPacket,
        C::OperatorBoard,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::WaiverExpiryHidden, D::ProofStale];
    rows.push(row);

    // 4. Release-gate banner.
    let mut row = base_row(
        F::ReleaseGateBanner,
        Q::Stable,
        "Release-gate component owner",
        "One release-gate-banner model naming the ship/no-ship decision — go, no-go, conditional, held pending evidence, or blocked by an unresolved owner or forum — with a specific reason, so a held or blocked gate is never shown as a generic go",
        "evidence:m5-release-gate-banner-parity:001",
        M5_RELEASE_GATE_BANNER_SCHEMA_REF,
    );
    row.release_gate_decisions = RG::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness, L::DecisionForum]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::ShiproomPacket,
        C::AssuranceDashboard,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::ReleaseGateReasonGeneric, D::ProofStale];
    rows.push(row);

    // 5. Mitigation note card.
    let mut row = base_row(
        F::MitigationNoteCard,
        Q::Stable,
        "Mitigation-note component owner",
        "One mitigation-note-card model carrying user-facing, jargon-free mitigation language and its posture — mitigated, partially mitigated, unmitigated, risk accepted, or unknown — so an unmitigated or merely accepted risk is never shown as resolved and support/export can reuse the text",
        "evidence:m5-mitigation-note-card-parity:001",
        M5_MITIGATION_NOTE_CARD_SCHEMA_REF,
    );
    row.mitigation_postures = MP::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::AssuranceDashboard,
        C::HelpAbout,
        C::DocsPortal,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::MitigationHiddenBehindJargon, D::ProofStale];
    rows.push(row);

    // 6. Service-ownership card.
    let mut row = base_row(
        F::ServiceOwnershipCard,
        Q::Stable,
        "Service-ownership component owner",
        "One service-ownership-card model naming owner coverage — owned with backup, primary only, unresolved, stale, or policy-hidden — and its freshness, so a backup-missing or unresolved owner is never shown as covered",
        "evidence:m5-service-ownership-card-parity:001",
        M5_SERVICE_OWNERSHIP_CARD_SCHEMA_REF,
    );
    row.ownership_coverage_states = OW::ALL.to_vec();
    row.required_labels = labels_with(&[L::OwnerAndEscalation, L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::OperatorBoard,
        C::ServiceHealth,
        C::AssuranceDashboard,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::OwnerCoverageOverstated, D::ProofStale];
    rows.push(row);

    // 7. On-call strip.
    let mut row = base_row(
        F::OnCallStrip,
        Q::Stable,
        "On-call component owner",
        "One on-call-strip model naming on-call coverage — covered, gap, escalation-only, follow-the-sun, or unknown — and the escalation route, so an on-call gap or a missing escalation path is never shown as covered",
        "evidence:m5-on-call-strip-parity:001",
        M5_ON_CALL_STRIP_SCHEMA_REF,
    );
    row.on_call_coverage_states = OC::ALL.to_vec();
    row.escalation_route_classes = ER::ALL.to_vec();
    row.required_labels = labels_with(&[L::OwnerAndEscalation, L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::OperatorBoard,
        C::ServiceHealth,
        C::AssuranceDashboard,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OnCallGapHidden,
        D::EscalationRouteUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Decision-right card.
    let mut row = base_row(
        F::DecisionRightCard,
        Q::Stable,
        "Decision-right component owner",
        "One decision-right-card model naming the forum authorized to approve the next move — release council, service owner, security review board, architecture forum, or none — and its state, so an advisory or unresolved forum is never shown as authoritative",
        "evidence:m5-decision-right-card-parity:001",
        M5_DECISION_RIGHT_CARD_SCHEMA_REF,
    );
    row.decision_forum_classes = DF::ALL.to_vec();
    row.decision_right_states = DR::ALL.to_vec();
    row.required_labels = labels_with(&[L::DecisionForum, L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ShiproomPacket,
        C::ReleaseCenterUi,
        C::AssuranceDashboard,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::DecisionForumMasked,
        D::AdvisoryForumReadsAuthoritative,
        D::ProofStale,
    ];
    rows.push(row);

    // 9. Milestone dashboard row.
    let mut row = base_row(
        F::MilestoneDashboardRow,
        Q::Stable,
        "Milestone-dashboard component owner",
        "One milestone-dashboard-row model naming the milestone's exit-gate state — met, pending, blocked, waived, or stale — so a blocked, waived, or stale exit gate is never shown as met",
        "evidence:m5-milestone-dashboard-row-parity:001",
        M5_MILESTONE_DASHBOARD_ROW_SCHEMA_REF,
    );
    row.milestone_gate_states = MG::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ShiproomPacket,
        C::AssuranceDashboard,
        C::OperatorBoard,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::MilestoneGateOverstated, D::ProofStale];
    rows.push(row);

    rows
}

fn governance_review() -> M5GovernanceDashboardGovernanceReview {
    M5GovernanceDashboardGovernanceReview {
        fitness_tile_shows_provenance_and_freshness: true,
        report_row_states_readiness_with_evidence: true,
        waiver_item_states_expiry: true,
        release_gate_banner_states_specific_reason: true,
        mitigation_card_carries_reusable_language: true,
        ownership_card_states_coverage_and_freshness: true,
        on_call_strip_states_coverage_and_route: true,
        decision_right_card_names_authorized_forum: true,
        waived_or_stale_never_clean_pass: true,
        no_component_invents_second_status_grammar: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5GovernanceDashboardConsumerProjection {
    M5GovernanceDashboardConsumerProjection {
        assurance_and_operator_surfaces_consume_readiness_vocabulary: true,
        waiver_and_mitigation_surfaces_consume_matrix: true,
        ownership_and_on_call_surfaces_consume_coverage_vocabulary: true,
        release_gate_and_decision_right_surfaces_consume_forum_vocabulary: true,
        support_export_reads_single_source: true,
        shiproom_and_milestone_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5GovernanceDashboardProofFreshness {
    M5GovernanceDashboardProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5GovernanceDashboardReleasePosture {
    M5GovernanceDashboardReleasePosture {
        governance_packet_ref:
            "artifacts/release/m5-governance-dashboard-proof/support_export.json".to_owned(),
        assurance_audit_ref: "artifacts/design/m5-governance-dashboard-component-matrix.md"
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_GOVERNANCE_DASHBOARD_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_DOC_REF,
        M5_FITNESS_DASHBOARD_TILE_SCHEMA_REF,
        M5_GOVERNANCE_REPORT_ROW_SCHEMA_REF,
        M5_WAIVER_EXPIRY_QUEUE_ITEM_SCHEMA_REF,
        M5_RELEASE_GATE_BANNER_SCHEMA_REF,
        M5_MITIGATION_NOTE_CARD_SCHEMA_REF,
        M5_SERVICE_OWNERSHIP_CARD_SCHEMA_REF,
        M5_ON_CALL_STRIP_SCHEMA_REF,
        M5_DECISION_RIGHT_CARD_SCHEMA_REF,
        M5_MILESTONE_DASHBOARD_ROW_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 governance-dashboard-component matrix packet.
pub fn seeded_m5_governance_dashboard_component_matrix() -> M5GovernanceDashboardMatrixPacket {
    M5GovernanceDashboardMatrixPacket::new(M5GovernanceDashboardMatrixPacketInput {
        packet_id: M5_GOVERNANCE_DASHBOARD_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 fitness-dashboard-tile, governance-report-row, waiver-expiry-queue-item, release-gate-banner, mitigation-note-card, service-ownership-card, on-call-strip, decision-right-card, and milestone-dashboard-row component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5GovernanceDashboardVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the service-ownership card is held at Beta because a slice of
/// owner-coverage transitions do not yet round-trip across every export path; every
/// component stays visible and an unresolved owner never reads as covered.
pub fn seeded_m5_governance_dashboard_component_matrix_service_ownership_card_beta_narrowed(
) -> M5GovernanceDashboardMatrixPacket {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.packet_id =
        "m5-governance-dashboard-components:service-ownership-card-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5GovernanceDashboardComponentFamily::ServiceOwnershipCard
        })
        .expect("service-ownership-card row present");
    row.qualification = M5GovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-gate banner is narrowed to Preview pending
/// ship/no-ship reason parity across every gate feed; every component stays visible
/// and a held gate never reads as a generic go.
pub fn seeded_m5_governance_dashboard_component_matrix_release_gate_banner_preview_narrowed(
) -> M5GovernanceDashboardMatrixPacket {
    let mut packet = seeded_m5_governance_dashboard_component_matrix();
    packet.packet_id =
        "m5-governance-dashboard-components:release-gate-banner-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5GovernanceDashboardComponentFamily::ReleaseGateBanner)
        .expect("release-gate-banner row present");
    row.qualification = M5GovernanceQualificationClass::Preview;
    packet
}
