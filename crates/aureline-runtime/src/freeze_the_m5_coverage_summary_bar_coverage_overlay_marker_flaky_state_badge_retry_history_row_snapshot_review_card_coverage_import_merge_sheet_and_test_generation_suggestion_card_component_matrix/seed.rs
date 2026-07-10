//! Canonical seed builders for the frozen M5 test-intelligence component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical test-intelligence component matrix.
pub const M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-test-intelligence-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5TestIntelligenceRequiredLabel> {
    M5TestIntelligenceRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5TestIntelligenceRequiredLabel]) -> Vec<M5TestIntelligenceRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate. Every component
/// binds the one controlled provenance vocabulary.
fn base_row(
    component_family: M5TestIntelligenceComponentFamily,
    qualification: M5TestIntelligenceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5TestIntelligenceComponentRow {
    M5TestIntelligenceComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestIntelligenceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestIntelligenceDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        provenance_classes: M5TestIntelligenceProvenanceClass::ALL.to_vec(),
        coverage_scope_classes: vec![],
        coverage_metric_kinds: vec![],
        overlay_states: vec![],
        overlay_emphasis_classes: vec![],
        flaky_classifications: vec![],
        flaky_confidence_classes: vec![],
        retry_attempt_outcomes: vec![],
        retry_scope_classes: vec![],
        snapshot_baseline_identities: vec![],
        snapshot_diff_states: vec![],
        coverage_import_sources: vec![],
        merge_resolution_states: vec![],
        generated_assumption_classes: vec![],
        generated_apply_scopes: vec![],
        accessibility_routes: M5TestIntelligenceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5TestIntelligenceConsumerSurface::CoverageReportUi,
            M5TestIntelligenceConsumerSurface::SupportExport,
            M5TestIntelligenceConsumerSurface::CliInspect,
        ],
        downgrade_triggers: vec![M5TestIntelligenceDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_provenance_or_freshness_class: false,
        hides_shard_omission_behind_single_percentage: false,
        labels_intermittent_failure_as_confirmed_flaky: false,
        bundles_generated_changes_into_opaque_apply: false,
        invents_alternate_state_label: false,
    }
}

fn component_rows() -> Vec<M5TestIntelligenceComponentRow> {
    use M5CoverageImportSource as CI;
    use M5CoverageMetricKind as CM;
    use M5CoverageOverlayState as OS;
    use M5CoverageScopeClass as CS;
    use M5FlakyClassification as FCl;
    use M5FlakyConfidenceClass as FCo;
    use M5GeneratedApplyScope as GA;
    use M5GeneratedAssumptionClass as GAsm;
    use M5MergeResolutionState as MR;
    use M5OverlayEmphasisClass as OE;
    use M5RetryAttemptOutcome as RO;
    use M5RetryScopeClass as RS;
    use M5SnapshotBaselineIdentity as SB;
    use M5SnapshotDiffState as SD;
    use M5TestIntelligenceComponentFamily as F;
    use M5TestIntelligenceConsumerSurface as C;
    use M5TestIntelligenceDowngradeTrigger as D;
    use M5TestIntelligenceQualificationClass as Q;
    use M5TestIntelligenceRequiredLabel as L;

    let mut rows = Vec::new();

    // 1. Coverage-summary bar.
    let mut row = base_row(
        F::CoverageSummaryBar,
        Q::Stable,
        "Coverage-summary bar owner",
        "One coverage-summary-bar model naming the included run set behind a coverage number — full suite, changed files only, a single shard, a merged multi-shard run, an imported report, or a partial incomplete scope — and which measure it summarizes so a single percentage never hides a shard omission or conflates line, branch, function, statement, region, and combined measures",
        "evidence:m5-coverage-summary-bar-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_MERGE_REF,
        ],
    );
    row.coverage_scope_classes = CS::ALL.to_vec();
    row.coverage_metric_kinds = CM::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndFreshness, L::BaselineOrScopeIdentity]);
    row.consumer_surfaces = vec![
        C::CoverageReportUi,
        C::EditorOverlayUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::LineVersusBranchUnstated,
        D::ShardOmissionHidden,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Coverage-overlay marker.
    let mut row = base_row(
        F::CoverageOverlayMarker,
        Q::Stable,
        "Coverage-overlay marker owner",
        "One coverage-overlay-marker model naming what a per-line gutter glyph asserts — covered, uncovered, partially covered, branch missed, excluded, or no data — with changed-file emphasis and its provenance so an editor overlay never shows a stale or imported measurement as if it were freshly produced here and a regression on a changed line is never lost",
        "evidence:m5-coverage-overlay-marker-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_OVERLAY_REF,
        ],
    );
    row.overlay_states = OS::ALL.to_vec();
    row.overlay_emphasis_classes = OE::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndFreshness]);
    row.consumer_surfaces = vec![
        C::EditorOverlayUi,
        C::CoverageReportUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::FreshnessClassUndisclosed,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Flaky-state badge.
    let mut row = base_row(
        F::FlakyStateBadge,
        Q::Stable,
        "Flaky-state badge owner",
        "One flaky-state-badge model naming the classification a badge asserts — stable, suspected flaky, reproduced flaky, stable again, manually muted, or unknown — and the classifier confidence behind it so a single intermittent failure is never labelled as confirmed flakiness and a suspicion never presents with the authority of a reproduced verdict",
        "evidence:m5-flaky-state-badge-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_FLAKY_VERDICT_REF,
        ],
    );
    row.flaky_classifications = FCl::ALL.to_vec();
    row.flaky_confidence_classes = FCo::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndFreshness]);
    row.consumer_surfaces = vec![
        C::FlakyDashboardUi,
        C::RetryHistoryUi,
        C::CoverageReportUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::FlakyConfidenceOverstated,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Retry-history row.
    let mut row = base_row(
        F::RetryHistoryRow,
        Q::Stable,
        "Retry-history row owner",
        "One retry-history-row model naming what a single attempt resulted in — passed first try, passed on retry, failed all retries, errored, skipped, or aborted — and how the rerun behind it was scoped so a pass-on-retry is never shown as a clean first-try pass and a widened rerun is never presented as the same selection",
        "evidence:m5-retry-history-row-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_TEST_ATTEMPT_REF,
        ],
    );
    row.retry_attempt_outcomes = RO::ALL.to_vec();
    row.retry_scope_classes = RS::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndFreshness]);
    row.consumer_surfaces = vec![
        C::RetryHistoryUi,
        C::FlakyDashboardUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::RetryScopeWidened,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Snapshot / golden review card.
    let mut row = base_row(
        F::SnapshotReviewCard,
        Q::Stable,
        "Snapshot-review card owner",
        "One snapshot-review-card model naming which baseline a snapshot or golden compares against — committed, pending new, updated, imported, missing, or ambiguous — and its diff state with a raw or text fallback so a binary-only change is never blind-accepted and an imported baseline never reads as a local accept",
        "evidence:m5-snapshot-review-card-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_SNAPSHOT_REVIEW_REF,
        ],
    );
    row.snapshot_baseline_identities = SB::ALL.to_vec();
    row.snapshot_diff_states = SD::ALL.to_vec();
    row.required_labels = labels_with(&[L::BaselineOrScopeIdentity, L::ProvenanceAndFreshness]);
    row.consumer_surfaces = vec![
        C::SnapshotReviewUi,
        C::CoverageReportUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::SnapshotBaselineUnstated,
        D::RawTextFallbackMissing,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Coverage-import / merge sheet.
    let mut row = base_row(
        F::CoverageImportMergeSheet,
        Q::Stable,
        "Coverage-import merge sheet owner",
        "One coverage-import-merge-sheet model naming where a report was drawn from — a local run, an imported CI artifact, a cached local report, a stale prior report, an uploaded report, or an unknown source — and how overlapping reports resolved so a shard omission is never hidden behind a merged total and a stale or imported report never reads as a fresh local run",
        "evidence:m5-coverage-import-merge-sheet-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_MERGE_REF,
        ],
    );
    row.coverage_import_sources = CI::ALL.to_vec();
    row.merge_resolution_states = MR::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProvenanceAndFreshness, L::BaselineOrScopeIdentity]);
    row.consumer_surfaces = vec![
        C::CoverageImportUi,
        C::CoverageReportUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::ShardOmissionHidden,
        D::FreshnessClassUndisclosed,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Test-generation suggestion card.
    let mut row = base_row(
        F::TestGenerationSuggestionCard,
        Q::Stable,
        "Test-generation suggestion card owner",
        "One test-generation-suggestion-card model naming what an AI-generated test assumed — a fixture, an inferred assertion, a generated snapshot, a synthesized mock, an assumed dependency, or an unverified behavior — and what it would apply so assertion, fixture, and snapshot changes are never silently bundled into one opaque apply path and a generated test always discloses its assumptions and recovery boundary",
        "evidence:m5-test-generation-suggestion-card-parity:001",
        &[
            M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
            M5_TEST_INTELLIGENCE_COMPONENT_TEST_GENERATION_REF,
        ],
    );
    row.generated_assumption_classes = GAsm::ALL.to_vec();
    row.generated_apply_scopes = GA::ALL.to_vec();
    row.required_labels =
        labels_with(&[L::AssumptionAndRecoveryBoundary, L::ProvenanceAndFreshness]);
    row.consumer_surfaces = vec![
        C::TestGenerationUi,
        C::CoverageReportUi,
        C::SupportExport,
        C::CliInspect,
    ];
    row.downgrade_triggers = vec![
        D::GeneratedAssumptionHidden,
        D::OpaqueApplyBundle,
        D::ProvenanceClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5TestIntelligenceComponentGovernanceReview {
    M5TestIntelligenceComponentGovernanceReview {
        coverage_summary_shows_scope_and_metric: true,
        coverage_overlay_shows_state_and_emphasis: true,
        flaky_badge_shows_classification_and_confidence: true,
        retry_history_shows_outcome_and_scope: true,
        snapshot_card_shows_baseline_and_diff: true,
        coverage_import_shows_source_and_merge_resolution: true,
        test_generation_shows_assumptions_and_apply_scope: true,
        no_surface_invents_alternate_state_label: true,
        provenance_vocabulary_named_once: true,
        single_percentage_never_hides_shard_omission: true,
        intermittent_never_labeled_confirmed_flaky: true,
        generated_changes_never_bundled_opaquely: true,
        raw_text_fallback_always_available: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5TestIntelligenceComponentConsumerProjection {
    M5TestIntelligenceComponentConsumerProjection {
        coverage_surfaces_consume_scope_and_metric_vocabulary: true,
        overlay_surfaces_consume_freshness_and_provenance_vocabulary: true,
        flaky_surfaces_consume_classification_and_confidence_vocabulary: true,
        snapshot_surfaces_consume_baseline_identity_vocabulary: true,
        generation_surfaces_consume_assumption_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5TestIntelligenceComponentProofFreshness {
    M5TestIntelligenceComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TestIntelligenceComponentReleasePosture {
    M5TestIntelligenceComponentReleasePosture {
        proof_packet_ref: M5_TEST_INTELLIGENCE_COMPONENT_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_TEST_INTELLIGENCE_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_DOC_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_MERGE_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_COVERAGE_OVERLAY_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_FLAKY_VERDICT_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_TEST_ATTEMPT_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_SNAPSHOT_REVIEW_REF,
        M5_TEST_INTELLIGENCE_COMPONENT_TEST_GENERATION_REF,
    ])
}

/// Builds the canonical frozen M5 test-intelligence component matrix packet.
pub fn seeded_m5_test_intelligence_component_matrix() -> M5TestIntelligenceComponentMatrixPacket {
    M5TestIntelligenceComponentMatrixPacket::new(M5TestIntelligenceComponentMatrixPacketInput {
        packet_id: M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 coverage-summary-bar, coverage-overlay-marker, flaky-state-badge, retry-history-row, snapshot-review-card, coverage-import-merge-sheet, and test-generation-suggestion-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5TestIntelligenceComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the flaky-state badge is held at Beta because a slice of the
/// reproduced-flaky confidence state does not yet round-trip across every test
/// surface; every component stays visible.
pub fn seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed(
) -> M5TestIntelligenceComponentMatrixPacket {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.packet_id = "m5-test-intelligence-components:flaky-state-badge-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TestIntelligenceComponentFamily::FlakyStateBadge)
        .expect("flaky-state-badge row present");
    row.qualification = M5TestIntelligenceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the coverage-import / merge sheet is narrowed to Preview pending
/// shard-omission parity proof across every surface; every component stays visible.
pub fn seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed(
) -> M5TestIntelligenceComponentMatrixPacket {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.packet_id =
        "m5-test-intelligence-components:coverage-import-merge-sheet-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5TestIntelligenceComponentFamily::CoverageImportMergeSheet
        })
        .expect("coverage-import-merge-sheet row present");
    row.qualification = M5TestIntelligenceQualificationClass::Preview;
    packet
}
