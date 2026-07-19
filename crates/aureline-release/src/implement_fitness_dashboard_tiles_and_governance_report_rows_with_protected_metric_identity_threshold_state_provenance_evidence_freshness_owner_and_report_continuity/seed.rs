//! Canonical seed builders for the M5 fitness-tile / governance-report controls
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The gated artifact generator and the inline tests both call
//! them so the in-code matrix, the artifact, the worked resolutions, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical fitness/governance controls packet.
pub const M5_FITNESS_GOVERNANCE_CONTROLS_PACKET_ID: &str =
    "m5-fitness-governance-report-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked fitness-tile case from a full tile state.
#[allow(clippy::too_many_arguments)]
fn ftile(
    fitness_id: &str,
    fitness_family: &str,
    declared_state: M5FitnessDeclaredState,
    threshold_state: M5ThresholdState,
    provenance_class: M5FitnessProvenanceClass,
    evidence_freshness: M5EvidenceFreshness,
    profile_match: M5ProfileMatchState,
    owner_alias: &str,
    linked_evidence_refs: &[&str],
) -> M5FitnessTileCase {
    M5FitnessTileCase::resolved(M5FitnessTileResolutionInput {
        fitness_id_repr: fitness_id.to_owned(),
        fitness_family_repr: fitness_family.to_owned(),
        declared_state,
        threshold_state,
        provenance_class,
        evidence_freshness,
        profile_match,
        owner_alias: owner_alias.to_owned(),
        linked_evidence_refs: linked_evidence_refs
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
    })
}

/// Builds a worked governance-report case from a full report state.
#[allow(clippy::too_many_arguments)]
fn rrep(
    report_id: &str,
    report_type: M5GovernanceReportType,
    report_scope: M5GovernanceReportScope,
    provenance_class: M5FitnessProvenanceClass,
    timestamp: &str,
    declared_outcome: M5ReportOutcome,
    evidence_freshness: M5EvidenceFreshness,
    support_class_bounded: bool,
) -> M5GovernanceReportCase {
    M5GovernanceReportCase::resolved(M5GovernanceReportResolutionInput {
        report_id_repr: report_id.to_owned(),
        report_type,
        report_scope,
        provenance_class,
        timestamp_repr: timestamp.to_owned(),
        declared_outcome,
        evidence_freshness,
        support_class_bounded,
    })
}

/// A base row with the shared fields filled in and the full anatomy, label,
/// readiness, declared-state, threshold, provenance, evidence, profile, degrade,
/// report-type, scope, outcome, disclosure, action, next-action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5FitnessGovernanceConsumerSurface,
    qualification: M5GovernanceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    fitness_tile_examples: Vec<M5FitnessTileCase>,
    report_row_examples: Vec<M5GovernanceReportCase>,
) -> M5FitnessGovernanceRow {
    M5FitnessGovernanceRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5GovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5FitnessGovernanceAnatomyPart::ALL.to_vec(),
        required_labels: M5GovernanceRequiredLabel::ALL.to_vec(),
        readiness_states: M5GovernanceReadinessState::ALL.to_vec(),
        fitness_declared_states: M5FitnessDeclaredState::ALL.to_vec(),
        threshold_states: M5ThresholdState::ALL.to_vec(),
        provenance_classes: M5FitnessProvenanceClass::ALL.to_vec(),
        evidence_freshness_states: M5EvidenceFreshness::ALL.to_vec(),
        profile_match_states: M5ProfileMatchState::ALL.to_vec(),
        fitness_degrade_reasons: M5FitnessDegradeReason::ALL.to_vec(),
        report_types: M5GovernanceReportType::ALL.to_vec(),
        report_scopes: M5GovernanceReportScope::ALL.to_vec(),
        report_outcomes: M5ReportOutcome::ALL.to_vec(),
        provenance_disclosures: M5ProvenanceDisclosure::ALL.to_vec(),
        report_degrade_reasons: M5ReportDegradeReason::ALL.to_vec(),
        report_actions: M5ReportAction::ALL.to_vec(),
        next_actions: M5GovernanceNextAction::ALL.to_vec(),
        export_fields: M5FitnessGovernanceExportField::ALL.to_vec(),
        accessibility_routes: M5GovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5GovernanceConsumerSurface::AssuranceDashboard,
            M5GovernanceConsumerSurface::OperatorBoard,
            M5GovernanceConsumerSurface::ShiproomPacket,
            M5GovernanceConsumerSurface::ServiceHealth,
            M5GovernanceConsumerSurface::SupportExport,
            M5GovernanceConsumerSurface::CliInspect,
            M5GovernanceConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5GovernanceDowngradeTrigger::FitnessProvenanceUnstated,
            M5GovernanceDowngradeTrigger::EvidenceStaleHidden,
            M5GovernanceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF,
            M5_FITNESS_DASHBOARD_TILE_CONTRACT_REF,
            M5_GOVERNANCE_REPORT_ROW_CONTRACT_REF,
        ]),
        fitness_tile_examples,
        report_row_examples,
        renders_stale_or_wrong_profile_as_clean_pass: false,
        hides_corpus_or_profile_provenance: false,
        hides_owner_or_evidence_freshness: false,
        invents_dashboard_local_status_grammar: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn controls_rows() -> Vec<M5FitnessGovernanceRow> {
    use M5EvidenceFreshness as Fresh;
    use M5FitnessDeclaredState as Metric;
    use M5FitnessProvenanceClass as Prov;
    use M5GovernanceReportScope as Scope;
    use M5GovernanceReportType as Report;
    use M5ProfileMatchState as Profile;
    use M5ReportOutcome as Outcome;
    use M5ThresholdState as Thresh;

    let mut rows = Vec::new();

    // 1. Assurance dashboard — a green metric that degrades on stale evidence (the AC-1
    //    example), a green metric that degrades on a wrong profile, a sampled-corpus
    //    report disclosed as not trustable outside its support class (the AC-2
    //    example), and a clean canonical report.
    rows.push(base_row(
        M5FitnessGovernanceConsumerSurface::AssuranceDashboard,
        M5GovernanceQualificationClass::Stable,
        "Assurance-dashboard owner",
        "The assurance dashboard renders the shared fitness tile so a green metric whose evidence has gone stale reads as evidence_stale — not passing — and a green metric whose evidence came from a wrong profile reads as warning, while a sampled-corpus governance report is disclosed as not trustable outside its support class before it is trusted",
        "evidence:m5-fitness-governance-assurance:001",
        vec![
            ftile(
                "fitness:api-p99-latency",
                "performance",
                Metric::MetricPass,
                Thresh::WithinThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceStale,
                Profile::ProfileMatched,
                "role:performance-guild",
                &["evidence:bench-run-4821"],
            ),
            ftile(
                "fitness:cold-start-budget",
                "performance",
                Metric::MetricPass,
                Thresh::WithinThreshold,
                Prov::ProfilePinned,
                Fresh::EvidenceFresh,
                Profile::WrongProfile,
                "role:performance-guild",
                &["evidence:bench-run-4822"],
            ),
        ],
        vec![
            rrep(
                "report:fitness-rollup-fleet",
                Report::FitnessRollupReport,
                Scope::FleetScope,
                Prov::SampledCorpus,
                "2026-07-09T12:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceFresh,
                true,
            ),
            rrep(
                "report:release-readiness-train",
                Report::ReleaseReadinessReport,
                Scope::TrainScope,
                Prov::CanonicalCorpus,
                "2026-07-09T13:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    // 2. Operator board — a clean passing metric, a failed/breached metric that blocks,
    //    a synthetic-corpus report disclosed with a caveat, and an undisclosed-provenance
    //    report that degrades to warning.
    rows.push(base_row(
        M5FitnessGovernanceConsumerSurface::OperatorBoard,
        M5GovernanceQualificationClass::Stable,
        "Operator-board owner",
        "The operator board renders the shared fitness tile so a passing metric with fresh, profile-matched evidence reads as passing, a failed metric that breached its threshold reads as blocked, and a governance report whose corpus/profile provenance is undisclosed degrades to warning rather than reading like a clean pass",
        "evidence:m5-fitness-governance-operator:001",
        vec![
            ftile(
                "fitness:error-budget-burn",
                "reliability",
                Metric::MetricPass,
                Thresh::WithinThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceFresh,
                Profile::ProfileMatched,
                "role:reliability-guild",
                &["evidence:slo-run-3310"],
            ),
            ftile(
                "fitness:memory-ceiling",
                "reliability",
                Metric::MetricFail,
                Thresh::BreachedThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceFresh,
                Profile::ProfileMatched,
                "role:reliability-guild",
                &["evidence:slo-run-3311"],
            ),
        ],
        vec![
            rrep(
                "report:ownership-coverage-family",
                Report::OwnershipCoverageReport,
                Scope::FamilyScope,
                Prov::SyntheticCorpus,
                "2026-07-09T14:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceFresh,
                true,
            ),
            rrep(
                "report:milestone-exit-service",
                Report::MilestoneExitReport,
                Scope::ServiceScope,
                Prov::ProvenanceUnknown,
                "2026-07-09T15:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    // 3. Shiproom packet — a waived metric, a not-run metric, a canonical report read
    //    outside its support class (degrades), and a pinned-profile partial report.
    rows.push(base_row(
        M5FitnessGovernanceConsumerSurface::ShiproomPacket,
        M5GovernanceQualificationClass::Stable,
        "Shiproom-packet owner",
        "The shiproom packet renders the shared fitness tile so a waived metric reads as waived rather than passing and a not-run metric reads as not_evaluated, while a canonical governance result read outside its support class, and a pinned-profile partial result, each disclose their corpus/profile before they are trusted",
        "evidence:m5-fitness-governance-shiproom:001",
        vec![
            ftile(
                "fitness:startup-crash-rate",
                "quality",
                Metric::MetricWaived,
                Thresh::WithinThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceFresh,
                Profile::ProfileMatched,
                "role:quality-guild",
                &["evidence:crash-run-2201", "waiver:w-3391"],
            ),
            ftile(
                "fitness:accessibility-audit",
                "quality",
                Metric::MetricNotRun,
                Thresh::ThresholdUnknown,
                Prov::ProvenanceUnknown,
                Fresh::EvidenceUnknown,
                Profile::ProfileMatchUnknown,
                "role:quality-guild",
                &[],
            ),
        ],
        vec![
            rrep(
                "report:fitness-rollup-service-oob",
                Report::FitnessRollupReport,
                Scope::ServiceScope,
                Prov::CanonicalCorpus,
                "2026-07-09T16:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceFresh,
                false,
            ),
            rrep(
                "report:waiver-ledger-partial",
                Report::WaiverLedgerReport,
                Scope::WaiverLedgerScope,
                Prov::ProfilePinned,
                "2026-07-09T17:00:00Z",
                Outcome::ReportPartial,
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    // 4. CLI inspect — a metric blocked on missing evidence, a metric with an
    //    unresolved owner, a failed sampled report, and a not-run report.
    rows.push(base_row(
        M5FitnessGovernanceConsumerSurface::CliInspect,
        M5GovernanceQualificationClass::Stable,
        "CLI-inspect owner",
        "The CLI inspect surface renders the shared fitness tile so a metric whose evidence is missing reads as blocked, a metric with no resolved owner reads as owner_unresolved, a failed sampled-corpus report reads as blocked, and a not-run report reads as not_evaluated — the same fitness/governance vocabulary a headless reviewer reads elsewhere",
        "evidence:m5-fitness-governance-cli:001",
        vec![
            ftile(
                "fitness:build-reproducibility",
                "supply-chain",
                Metric::MetricPass,
                Thresh::WithinThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceMissing,
                Profile::ProfileMatched,
                "role:supply-chain-guild",
                &["evidence:repro-run-7781"],
            ),
            ftile(
                "fitness:license-compliance",
                "supply-chain",
                Metric::MetricPass,
                Thresh::WithinThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceFresh,
                Profile::ProfileMatched,
                "",
                &["evidence:license-run-7782"],
            ),
        ],
        vec![
            rrep(
                "report:fitness-rollup-sampled-fail",
                Report::FitnessRollupReport,
                Scope::FamilyScope,
                Prov::SampledCorpus,
                "2026-07-09T18:00:00Z",
                Outcome::ReportFail,
                Fresh::EvidenceFresh,
                true,
            ),
            rrep(
                "report:release-readiness-not-run",
                Report::ReleaseReadinessReport,
                Scope::TrainScope,
                Prov::CanonicalCorpus,
                "2026-07-09T19:00:00Z",
                Outcome::ReportNotRun,
                Fresh::EvidenceUnknown,
                true,
            ),
        ],
    ));

    // 5. Support / export — a metric at warning, a green metric with aging evidence,
    //    a synthetic aging report, and a clean canonical report.
    rows.push(base_row(
        M5FitnessGovernanceConsumerSurface::SupportExport,
        M5GovernanceQualificationClass::Stable,
        "Support / export owner",
        "The support / export packet renders the shared fitness tile so a metric at warning and a green metric with aging evidence both read as warning rather than clean, and a synthetic-corpus report with aging evidence discloses its corpus/profile — the same fitness/governance vocabulary a support or evaluation reviewer reads elsewhere",
        "evidence:m5-fitness-governance-support:001",
        vec![
            ftile(
                "fitness:query-throughput",
                "performance",
                Metric::MetricWarn,
                Thresh::AtThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceFresh,
                Profile::ProfileMatched,
                "role:performance-guild",
                &["evidence:bench-run-4830"],
            ),
            ftile(
                "fitness:index-freshness",
                "performance",
                Metric::MetricPass,
                Thresh::WithinThreshold,
                Prov::CanonicalCorpus,
                Fresh::EvidenceAging,
                Profile::ProfileMatched,
                "role:performance-guild",
                &["evidence:bench-run-4831"],
            ),
        ],
        vec![
            rrep(
                "report:fitness-rollup-synthetic-aging",
                Report::FitnessRollupReport,
                Scope::FleetScope,
                Prov::SyntheticCorpus,
                "2026-07-09T20:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceAging,
                true,
            ),
            rrep(
                "report:ownership-coverage-clean",
                Report::OwnershipCoverageReport,
                Scope::FleetScope,
                Prov::CanonicalCorpus,
                "2026-07-09T21:00:00Z",
                Outcome::ReportPass,
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5FitnessGovernanceReview {
    M5FitnessGovernanceReview {
        one_packet_carries_fitness_and_governance_truth: true,
        identity_and_report_type_always_shown: true,
        stale_or_wrong_profile_never_reads_clean_pass: true,
        corpus_or_profile_provenance_always_disclosed: true,
        out_of_support_class_never_presented_trustable: true,
        owner_and_evidence_freshness_always_shown: true,
        compare_and_open_report_always_offered: true,
        readiness_state_drawn_from_frozen_vocabulary: true,
        support_export_reconstructs_truth: true,
        no_surface_invents_second_grammar: true,
        every_row_declares_accessibility_route: true,
        owner_alias_is_role_not_person: true,
    }
}

fn consumer_projection() -> M5FitnessGovernanceConsumerProjection {
    M5FitnessGovernanceConsumerProjection {
        surfaces_consume_shared_packet: true,
        readiness_resolver_reads_single_source: true,
        provenance_disclosure_reads_single_source: true,
        evidence_freshness_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5FitnessGovernanceProofFreshness {
    M5FitnessGovernanceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5FitnessGovernanceReleasePosture {
    M5FitnessGovernanceReleasePosture {
        governance_packet_ref: M5_FITNESS_GOVERNANCE_CONTROLS_ARTIFACT_REF.to_owned(),
        assurance_audit_ref: M5_FITNESS_GOVERNANCE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_REF,
        M5_FITNESS_GOVERNANCE_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_FITNESS_DASHBOARD_TILE_CONTRACT_REF,
        M5_GOVERNANCE_REPORT_ROW_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 fitness/governance controls packet.
pub fn seeded_m5_fitness_governance_controls_packet() -> M5FitnessGovernanceControlsPacket {
    M5FitnessGovernanceControlsPacket::new(M5FitnessGovernanceControlsPacketInput {
        packet_id: M5_FITNESS_GOVERNANCE_CONTROLS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 fitness dashboard tile and governance report row controls: protected-metric identity, threshold state, corpus/profile provenance, evidence freshness, owner, and compare-or-open-report continuity"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5FitnessGovernanceVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the assurance dashboard is held at Beta because a slice of
/// assurance-dashboard tiles do not yet render the linked-evidence list on every
/// profile; every consumer stays visible.
pub fn seeded_m5_fitness_governance_controls_assurance_dashboard_beta_narrowed(
) -> M5FitnessGovernanceControlsPacket {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.packet_id =
        "m5-fitness-governance-report-controls:assurance-dashboard-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5FitnessGovernanceConsumerSurface::AssuranceDashboard)
        .expect("assurance-dashboard row present");
    row.qualification = M5GovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the shiproom packet is narrowed to Preview pending
/// provenance-disclosure parity proof across every export path; every consumer stays
/// visible.
pub fn seeded_m5_fitness_governance_controls_shiproom_packet_preview_narrowed(
) -> M5FitnessGovernanceControlsPacket {
    let mut packet = seeded_m5_fitness_governance_controls_packet();
    packet.packet_id =
        "m5-fitness-governance-report-controls:shiproom-packet-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5FitnessGovernanceConsumerSurface::ShiproomPacket)
        .expect("shiproom-packet row present");
    row.qualification = M5GovernanceQualificationClass::Preview;
    packet
}
