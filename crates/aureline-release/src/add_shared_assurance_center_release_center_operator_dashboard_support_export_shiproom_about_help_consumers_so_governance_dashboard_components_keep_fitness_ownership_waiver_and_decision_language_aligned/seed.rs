//! Canonical seed builders for the M5 governance-dashboard-component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The gated artifact generator and the inline tests both call
//! them so the in-code matrix, the artifact, the worked bindings, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical governance-dashboard-component-consumer packet.
pub const M5_GOVERNANCE_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-governance-dashboard-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5GovernanceDashboardConsumer,
    component_family: M5GovernanceDashboardComponentFamily,
    evidence_state: M5GovernanceEvidenceState,
    note: &str,
) -> M5GovernanceBindingCase {
    M5GovernanceBindingCase::resolved(M5GovernanceBindingInput {
        consumer,
        component_family,
        descriptor_families: M5GovernanceDescriptor::ALL.to_vec(),
        evidence_state,
        readiness_vocab: M5GovernanceReadinessState::ALL.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5GovernanceDashboardComponentFamily,
    example_bindings: Vec<M5GovernanceBindingCase>,
) -> M5GovernanceComponentBinding {
    M5GovernanceComponentBinding {
        component_family,
        canonical_schema_ref: component_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: component_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5GovernanceDashboardConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5GovernanceComponentBinding>,
) -> M5GovernanceConsumerRow {
    M5GovernanceConsumerRow {
        consumer,
        qualification: M5GovernanceQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5GovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5GovernanceConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5GovernanceDescriptor::ALL.to_vec(),
        readiness_vocab: M5GovernanceReadinessState::ALL.to_vec(),
        evidence_states: M5GovernanceEvidenceState::ALL.to_vec(),
        projection_modes: M5GovernanceProjectionMode::ALL.to_vec(),
        descriptor_parity_states: M5GovernanceDescriptorParityState::ALL.to_vec(),
        narrow_reasons: M5GovernanceNarrowReason::ALL.to_vec(),
        next_actions: M5GovernanceNextAction::ALL.to_vec(),
        export_fields: M5GovernanceConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5GovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5GovernanceConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5GovernanceDowngradeTrigger::EvidenceStaleHidden,
            M5GovernanceDowngradeTrigger::OwnerCoverageOverstated,
            M5GovernanceDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_GOVERNANCE_CONSUMER_SCHEMA_REF,
            M5_GOVERNANCE_CONSUMER_MATRIX_SCHEMA_REF,
        ]),
        renders_waived_or_stale_as_clean_pass: false,
        lets_ownerless_or_forumless_blocker_read_resolved: false,
        hides_mitigation_behind_internal_jargon: false,
        rewords_governance_vocabulary_per_surface: false,
        invents_new_dashboard_local_status: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5GovernanceConsumerRow> {
    use M5GovernanceDashboardComponentFamily as Family;
    use M5GovernanceDashboardConsumer as Consumer;
    use M5GovernanceEvidenceState as Evidence;

    let mut rows = Vec::new();

    // 1. Assurance center — the authoritative full-truth rendering of the fitness
    //    dashboard tile and the governance report row.
    rows.push(base_row(
        Consumer::AssuranceCenter,
        "Assurance-center surface owner",
        "The assurance center adopts the fitness dashboard tile and governance report row primitives at full truth, pointing at their canonical controls schemas so readiness, evidence-freshness, waiver, owner-coverage, and decision-forum descriptors stay identical to what the release center, operator dashboard, shiproom, support export, and docs read",
        "evidence:m5-governance-consumer-assurance-center:001",
        vec![
            binding(
                Family::FitnessDashboardTile,
                vec![case(
                    Consumer::AssuranceCenter,
                    Family::FitnessDashboardTile,
                    Evidence::FullTruthFresh,
                    "assurance-center fitness tile at full truth",
                )],
            ),
            binding(
                Family::GovernanceReportRow,
                vec![case(
                    Consumer::AssuranceCenter,
                    Family::GovernanceReportRow,
                    Evidence::FullTruthFresh,
                    "assurance-center governance report row at full truth",
                )],
            ),
        ],
    ));

    // 2. Release center — the release-gate banner, decision-right card, and
    //    milestone dashboard row, all at full truth.
    rows.push(base_row(
        Consumer::ReleaseCenter,
        "Release-center surface owner",
        "The release center adopts the release-gate banner, decision-right card, and milestone dashboard row at full truth, reading the same readiness and decision-forum vocabulary so a ship/no-ship decision never diverges from the shiproom or the support export",
        "evidence:m5-governance-consumer-release-center:001",
        vec![
            binding(
                Family::ReleaseGateBanner,
                vec![case(
                    Consumer::ReleaseCenter,
                    Family::ReleaseGateBanner,
                    Evidence::FullTruthFresh,
                    "release-center gate banner at full truth",
                )],
            ),
            binding(
                Family::DecisionRightCard,
                vec![case(
                    Consumer::ReleaseCenter,
                    Family::DecisionRightCard,
                    Evidence::FullTruthFresh,
                    "release-center decision-right card at full truth",
                )],
            ),
            binding(
                Family::MilestoneDashboardRow,
                vec![case(
                    Consumer::ReleaseCenter,
                    Family::MilestoneDashboardRow,
                    Evidence::FullTruthFresh,
                    "release-center milestone row at full truth",
                )],
            ),
        ],
    ));

    // 3. Operator dashboard — the service-ownership card under missing owner
    //    coverage, and the on-call strip at full truth.
    rows.push(base_row(
        Consumer::OperatorDashboard,
        "Operator-dashboard surface owner",
        "The operator dashboard adopts the service-ownership card under missing owner coverage and the on-call strip at full truth, disclosing the ownership narrowing with a self-contained banner while keeping the same descriptor vocabulary the assurance center uses so an ownerless lane never reads as resolved",
        "evidence:m5-governance-consumer-operator:001",
        vec![
            binding(
                Family::ServiceOwnershipCard,
                vec![case(
                    Consumer::OperatorDashboard,
                    Family::ServiceOwnershipCard,
                    Evidence::OwnerCoverageMissing,
                    "operator ownership card with missing owner coverage",
                )],
            ),
            binding(
                Family::OnCallStrip,
                vec![case(
                    Consumer::OperatorDashboard,
                    Family::OnCallStrip,
                    Evidence::FullTruthFresh,
                    "operator on-call strip at full truth",
                )],
            ),
        ],
    ));

    // 4. Shiproom summary — the milestone dashboard row and release-gate banner at
    //    full truth, and the decision-right card under an unresolved forum.
    rows.push(base_row(
        Consumer::ShiproomSummary,
        "Shiproom-summary owner",
        "The shiproom summary adopts the milestone dashboard row and release-gate banner at full truth and the decision-right card under an unresolved forum, disclosing the forum narrowing with a self-contained banner so a forumless blocker never appears resolved in a ship packet",
        "evidence:m5-governance-consumer-shiproom:001",
        vec![
            binding(
                Family::MilestoneDashboardRow,
                vec![case(
                    Consumer::ShiproomSummary,
                    Family::MilestoneDashboardRow,
                    Evidence::FullTruthFresh,
                    "shiproom milestone row at full truth",
                )],
            ),
            binding(
                Family::DecisionRightCard,
                vec![case(
                    Consumer::ShiproomSummary,
                    Family::DecisionRightCard,
                    Evidence::ForumUnresolved,
                    "shiproom decision-right card with unresolved forum",
                )],
            ),
            binding(
                Family::ReleaseGateBanner,
                vec![case(
                    Consumer::ShiproomSummary,
                    Family::ReleaseGateBanner,
                    Evidence::FullTruthFresh,
                    "shiproom gate banner at full truth",
                )],
            ),
        ],
    ));

    // 5. Support export — the waiver-expiry queue item, mitigation note card, and
    //    service-ownership card, all at full truth.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support-export owner",
        "The support export adopts the waiver-expiry queue item, mitigation note card, and service-ownership card at full truth, reconstructing consumer parity from the shared model so a support reviewer reads the same plain-language mitigation and waiver vocabulary that every product surface shows",
        "evidence:m5-governance-consumer-support:001",
        vec![
            binding(
                Family::WaiverExpiryQueueItem,
                vec![case(
                    Consumer::SupportExport,
                    Family::WaiverExpiryQueueItem,
                    Evidence::FullTruthFresh,
                    "support waiver-expiry queue item at full truth",
                )],
            ),
            binding(
                Family::MitigationNoteCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::MitigationNoteCard,
                    Evidence::FullTruthFresh,
                    "support mitigation note card at full truth",
                )],
            ),
            binding(
                Family::ServiceOwnershipCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::ServiceOwnershipCard,
                    Evidence::FullTruthFresh,
                    "support ownership card at full truth",
                )],
            ),
        ],
    ));

    // 6. About / help — the fitness dashboard tile under a not-evaluated-here
    //    context, and the mitigation note card at full truth.
    rows.push(base_row(
        Consumer::AboutHelp,
        "About / help surface owner",
        "The About/help surface adopts the fitness dashboard tile under a not-evaluated-here context and the mitigation note card at full truth, referencing the canonical component schemas so its prose can never drift from the product truth and disclosing the not-evaluated narrowing rather than implying a clean pass",
        "evidence:m5-governance-consumer-about-help:001",
        vec![
            binding(
                Family::FitnessDashboardTile,
                vec![case(
                    Consumer::AboutHelp,
                    Family::FitnessDashboardTile,
                    Evidence::NotEvaluatedHere,
                    "about/help fitness tile not evaluated on this build",
                )],
            ),
            binding(
                Family::MitigationNoteCard,
                vec![case(
                    Consumer::AboutHelp,
                    Family::MitigationNoteCard,
                    Evidence::FullTruthFresh,
                    "about/help mitigation note card at full truth",
                )],
            ),
        ],
    ));

    // 7. Docs portal — the governance report row from a stale-evidence snapshot, and
    //    the on-call strip at full truth.
    rows.push(base_row(
        Consumer::DocsPortal,
        "Docs-portal surface owner",
        "The docs portal adopts the governance report row from a stale-evidence snapshot and the on-call strip at full truth, referencing the canonical component schemas and disclosing the stale-evidence narrowing with a self-contained banner so a docs reader never mistakes stale evidence for a fresh clean pass",
        "evidence:m5-governance-consumer-docs:001",
        vec![
            binding(
                Family::GovernanceReportRow,
                vec![case(
                    Consumer::DocsPortal,
                    Family::GovernanceReportRow,
                    Evidence::EvidenceStale,
                    "docs governance report row from a stale snapshot",
                )],
            ),
            binding(
                Family::OnCallStrip,
                vec![case(
                    Consumer::DocsPortal,
                    Family::OnCallStrip,
                    Evidence::FullTruthFresh,
                    "docs on-call strip at full truth",
                )],
            ),
        ],
    ));

    // 8. CLI inspect — the waiver-expiry queue item under an expiring waiver, and the
    //    fitness dashboard tile at full truth.
    rows.push(base_row(
        Consumer::CliInspect,
        "CLI inspect / headless owner",
        "The CLI inspect surface adopts the waiver-expiry queue item under an expiring waiver and the fitness dashboard tile at full truth, reading the same readiness and waiver vocabulary in headless output so a scripted reader sees identical blocker, waiver, owner, and forum truth to the GUI",
        "evidence:m5-governance-consumer-cli:001",
        vec![
            binding(
                Family::WaiverExpiryQueueItem,
                vec![case(
                    Consumer::CliInspect,
                    Family::WaiverExpiryQueueItem,
                    Evidence::WaiverExpiringOrExpired,
                    "cli waiver-expiry queue item with an expiring waiver",
                )],
            ),
            binding(
                Family::FitnessDashboardTile,
                vec![case(
                    Consumer::CliInspect,
                    Family::FitnessDashboardTile,
                    Evidence::FullTruthFresh,
                    "cli fitness tile at full truth",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5GovernanceConsumerGovernanceReview {
    M5GovernanceConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        governance_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_status: true,
        descriptors_explicit_on_every_surface: true,
        waived_or_stale_never_reads_clean: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_next_action: true,
        support_export_reconstructs_consumer_parity: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5GovernanceConsumerProjection {
    M5GovernanceConsumerProjection {
        all_consumers_adopt_shared_components: true,
        readiness_reads_single_source: true,
        evidence_freshness_reads_single_source: true,
        waiver_state_reads_single_source: true,
        owner_coverage_reads_single_source: true,
        decision_forum_reads_single_source: true,
    }
}

fn proof_freshness() -> M5GovernanceConsumerProofFreshness {
    M5GovernanceConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5GovernanceConsumerReleasePosture {
    M5GovernanceConsumerReleasePosture {
        release_packet_ref: M5_GOVERNANCE_CONSUMER_ARTIFACT_REF.to_owned(),
        consumer_audit_ref: M5_GOVERNANCE_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_GOVERNANCE_CONSUMER_SCHEMA_REF,
        M5_GOVERNANCE_CONSUMER_DOC_REF,
        M5_GOVERNANCE_CONSUMER_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_CONSUMER_MATRIX_DOC_REF,
        M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF,
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF,
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF,
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 governance-dashboard-component-consumer packet.
pub fn seeded_m5_governance_component_consumer_packet() -> M5GovernanceComponentConsumerPacket {
    M5GovernanceComponentConsumerPacket::new(M5GovernanceComponentConsumerPacketInput {
        packet_id: M5_GOVERNANCE_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 governance-dashboard component consumers: assurance center, release center, operator dashboard, shiproom, support export, About/help, docs, and CLI keep readiness, evidence-freshness, waiver, owner-coverage, and decision-forum parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5GovernanceConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the operator dashboard is held at Beta because a slice of
/// operator renderings do not yet expose the ownership narrow banner on every
/// missing-coverage path; every consumer stays visible.
pub fn seeded_m5_governance_component_consumer_operator_ownership_narrowed(
) -> M5GovernanceComponentConsumerPacket {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.packet_id = "m5-governance-dashboard-component-consumer:operator-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5GovernanceDashboardConsumer::OperatorDashboard)
        .expect("operator-dashboard row present");
    row.qualification = M5GovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the docs portal is narrowed to Preview pending stale-evidence
/// caveat-parity proof across every snapshot path; every consumer stays visible.
pub fn seeded_m5_governance_component_consumer_docs_stale_narrowed(
) -> M5GovernanceComponentConsumerPacket {
    let mut packet = seeded_m5_governance_component_consumer_packet();
    packet.packet_id = "m5-governance-dashboard-component-consumer:docs-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5GovernanceDashboardConsumer::DocsPortal)
        .expect("docs-portal row present");
    row.qualification = M5GovernanceQualificationClass::Preview;
    packet
}
