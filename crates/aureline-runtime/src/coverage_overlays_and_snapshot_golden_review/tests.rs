use super::*;

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn scope(scope_id: &str, scope_kind: CoverageScopeKind) -> CoverageScope {
    CoverageScope {
        scope_id: scope_id.to_owned(),
        scope_kind,
        scope_fingerprint_token: format!("fingerprint:{scope_id}"),
    }
}

fn measure(covered: u32, total: u32) -> CoverageMeasure {
    CoverageMeasure {
        covered_units: covered,
        total_units: total,
    }
}

fn legend(entries: &[(CoverageCellClass, &str, u32)]) -> Vec<CoverageLegendEntry> {
    entries
        .iter()
        .map(|(cell_class, label, gutter_count)| CoverageLegendEntry {
            cell_class: *cell_class,
            label: (*label).to_owned(),
            gutter_count: *gutter_count,
        })
        .collect()
}

fn verified_line_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:verified:line".to_owned(),
        scope: scope("src/lib.rs", CoverageScopeKind::File),
        provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(80, 100),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "covered", 80),
            (CoverageCellClass::Uncovered, "uncovered", 20),
        ]),
        run_ref: "run:verified".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: true,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "verified line overlay".to_owned(),
    }
}

fn verified_branch_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:verified:branch".to_owned(),
        scope: scope("src/branch.rs", CoverageScopeKind::File),
        provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
        metric_mode: CoverageMetricMode::BranchCoverage,
        line_measure: measure(40, 50),
        branch_measure: Some(measure(12, 20)),
        branch_supported: true,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "covered", 40),
            (
                CoverageCellClass::PartiallyCoveredBranch,
                "partial branch",
                8,
            ),
        ]),
        run_ref: "run:branch".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: true,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "verified branch overlay".to_owned(),
    }
}

fn changed_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:changed".to_owned(),
        scope: scope("changed-set", CoverageScopeKind::ChangedSet),
        provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(30, 40),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: Some(ChangedLineEmphasis {
            changed_since_ref: "git:main".to_owned(),
            changed_lines_total: 10,
            changed_lines_covered: 7,
        }),
        legend: legend(&[
            (CoverageCellClass::ChangedCovered, "changed covered", 7),
            (CoverageCellClass::ChangedUncovered, "changed uncovered", 3),
        ]),
        run_ref: "run:changed".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: true,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "changed-line overlay".to_owned(),
    }
}

fn imported_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:imported".to_owned(),
        scope: scope("src/imported.rs", CoverageScopeKind::File),
        provenance: CoverageEvidenceProvenance::ImportedCiArtifact,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(60, 100),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "covered", 60),
            (CoverageCellClass::ImportedUnverified, "imported", 100),
        ]),
        run_ref: "run:imported".to_owned(),
        origin_provider_ref: Some("provider:ci".to_owned()),
        presents_as_authoritative: false,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "imported overlay".to_owned(),
    }
}

fn cached_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:cached".to_owned(),
        scope: scope("src/cached.rs", CoverageScopeKind::File),
        provenance: CoverageEvidenceProvenance::CachedLocalResult,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(70, 100),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[(CoverageCellClass::Covered, "covered", 70)]),
        run_ref: "run:cached".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: false,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "cached overlay".to_owned(),
    }
}

fn stale_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:stale".to_owned(),
        scope: scope("src/stale.rs", CoverageScopeKind::File),
        provenance: CoverageEvidenceProvenance::StalePriorResult,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(90, 100),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[(CoverageCellClass::StaleNotComparable, "stale", 100)]),
        run_ref: "run:stale".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: false,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "stale overlay".to_owned(),
    }
}

fn merge_review() -> CoverageMergeReview {
    CoverageMergeReview {
        merge_id: "merge:checkout".to_owned(),
        scope: scope("package:checkout", CoverageScopeKind::Package),
        metric_mode: CoverageMetricMode::LineCoverage,
        merged_measure: measure(150, 200),
        runs: vec![
            MergedRunEntry {
                run_ref: "run:shard-a".to_owned(),
                provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
                disposition: CoverageRunDisposition::Included,
                note: "included".to_owned(),
            },
            MergedRunEntry {
                run_ref: "run:shard-b".to_owned(),
                provenance: CoverageEvidenceProvenance::StalePriorResult,
                disposition: CoverageRunDisposition::ExcludedStale,
                note: "stale beyond freshness window".to_owned(),
            },
            MergedRunEntry {
                run_ref: "run:shard-c".to_owned(),
                provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
                disposition: CoverageRunDisposition::ExcludedDuplicate,
                note: "duplicate of shard-a".to_owned(),
            },
        ],
        omitted_scopes: vec![
            OmittedScopeEntry {
                omission_kind: OmittedScopeKind::Shard,
                omitted_ref: "shard:3".to_owned(),
                reason: "shard did not report".to_owned(),
            },
            OmittedScopeEntry {
                omission_kind: OmittedScopeKind::Platform,
                omitted_ref: "platform:windows".to_owned(),
                reason: "platform not run".to_owned(),
            },
        ],
        implies_complete_certainty: false,
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "merge sheet".to_owned(),
    }
}

fn binary_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:image".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:render::home".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:render-home".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        artifact_kind: SnapshotArtifactKind::ImageSnapshot,
        changed_artifact_count: 2,
        total_artifact_count: 5,
        baseline_scope: SnapshotBaselineScope::PerTest,
        raw_fallback: RawFallbackAvailability::UnavailableBinaryOnly,
        decision: SnapshotReviewDecision::NeedsRawInspection,
        preview_first: true,
        imported: false,
        origin_provider_ref: None,
        diff_summary_ref: "diff:image".to_owned(),
        baseline_ref: "baseline:image".to_owned(),
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "binary image card".to_owned(),
    }
}

fn text_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:text".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:serialize::case[*]".to_owned(),
            node_kind: DurableTestNodeKind::ParameterizedTemplate,
            subject_fingerprint_token: "fingerprint:serialize".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        artifact_kind: SnapshotArtifactKind::TextSnapshot,
        changed_artifact_count: 1,
        total_artifact_count: 3,
        baseline_scope: SnapshotBaselineScope::PerParameterCase,
        raw_fallback: RawFallbackAvailability::TextDiffAvailable,
        decision: SnapshotReviewDecision::Accepted,
        preview_first: true,
        imported: false,
        origin_provider_ref: None,
        diff_summary_ref: "diff:text".to_owned(),
        baseline_ref: "baseline:text".to_owned(),
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "text snapshot card".to_owned(),
    }
}

fn imported_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:imported".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:golden::imported".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:golden-imported".to_owned(),
            identity_class: TestItemIdentityClass::ImportedReadOnly,
        },
        artifact_kind: SnapshotArtifactKind::GoldenFile,
        changed_artifact_count: 1,
        total_artifact_count: 1,
        baseline_scope: SnapshotBaselineScope::SharedFixture,
        raw_fallback: RawFallbackAvailability::RawArtifactReferenced,
        decision: SnapshotReviewDecision::PendingReview,
        preview_first: true,
        imported: true,
        origin_provider_ref: Some("provider:ci".to_owned()),
        diff_summary_ref: "diff:imported".to_owned(),
        baseline_ref: "baseline:imported".to_owned(),
        captured_at: "2026-06-13T00:00:00Z".to_owned(),
        support_summary: "imported golden card".to_owned(),
    }
}

fn guardrails() -> CoverageReviewGuardrails {
    CoverageReviewGuardrails {
        provenance_distinguished: true,
        branch_versus_line_truthful: true,
        changed_line_emphasis_preserved: true,
        imported_never_reads_as_local: true,
        merge_omissions_disclosed: true,
        snapshot_preview_first: true,
        no_green_over_stale_coverage: true,
    }
}

fn consumer_projection() -> CoverageReviewConsumerProjection {
    CoverageReviewConsumerProjection {
        coverage_gutter_normalized: true,
        coverage_legend_normalized: true,
        merge_sheet_normalized: true,
        snapshot_review_normalized: true,
        release_support_export_normalized: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        COVERAGE_REVIEW_SCHEMA_REF,
        COVERAGE_REVIEW_DOC_REF,
        COVERAGE_REVIEW_ARTIFACT_REF,
    ])
}

fn valid_packet() -> CoverageReviewPacket {
    CoverageReviewPacket::new(CoverageReviewPacketInput {
        packet_id: "packet:test".to_owned(),
        label: "test".to_owned(),
        overlays: vec![
            verified_line_overlay(),
            verified_branch_overlay(),
            changed_overlay(),
            imported_overlay(),
            cached_overlay(),
            stale_overlay(),
        ],
        merges: vec![merge_review()],
        snapshot_cards: vec![binary_card(), text_card(), imported_card()],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-13T00:00:00Z".to_owned(),
    })
}

#[test]
fn valid_packet_validates() {
    let packet = valid_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn export_round_trips() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: CoverageReviewPacket = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, packet);
}

#[test]
fn coverage_requires_each_provenance() {
    let mut packet = valid_packet();
    packet
        .overlays
        .retain(|o| o.provenance != CoverageEvidenceProvenance::StalePriorResult);
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::ProvenanceCoverageMissing));
}

#[test]
fn coverage_requires_branch_and_line_modes() {
    let mut packet = valid_packet();
    packet
        .overlays
        .retain(|o| o.metric_mode != CoverageMetricMode::BranchCoverage);
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::MetricModeCoverageMissing));
}

#[test]
fn imported_overlay_cannot_present_as_authoritative_green() {
    let mut packet = valid_packet();
    let imported = packet
        .overlays
        .iter_mut()
        .find(|o| o.overlay_id == "overlay:imported")
        .unwrap();
    imported.presents_as_authoritative = true;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::GreenOverStaleOrImported));
}

#[test]
fn stale_overlay_cannot_present_as_authoritative_green() {
    let mut packet = valid_packet();
    let stale = packet
        .overlays
        .iter_mut()
        .find(|o| o.overlay_id == "overlay:stale")
        .unwrap();
    stale.presents_as_authoritative = true;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::GreenOverStaleOrImported));
}

#[test]
fn branch_mode_requires_a_branch_measure() {
    let mut packet = valid_packet();
    let branch = packet
        .overlays
        .iter_mut()
        .find(|o| o.overlay_id == "overlay:verified:branch")
        .unwrap();
    branch.branch_measure = None;
    branch.branch_supported = false;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::BranchWithoutMeasure));
}

#[test]
fn imported_overlay_must_carry_imported_markers() {
    let mut packet = valid_packet();
    let imported = packet
        .overlays
        .iter_mut()
        .find(|o| o.overlay_id == "overlay:imported")
        .unwrap();
    imported.origin_provider_ref = None;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::ImportedReadsAsLocal));
}

#[test]
fn changed_line_emphasis_must_be_represented() {
    let mut packet = valid_packet();
    for overlay in &mut packet.overlays {
        overlay.changed_line_emphasis = None;
    }
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::ChangedLineCaseMissing));
}

#[test]
fn merge_must_distinguish_included_and_excluded() {
    let mut packet = valid_packet();
    for merge in &mut packet.merges {
        merge.runs.retain(|r| r.disposition.is_included());
    }
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::MergeDistinctionMissing));
}

#[test]
fn merge_cannot_claim_false_certainty() {
    let mut packet = valid_packet();
    packet.merges[0].implies_complete_certainty = true;
    let violations = packet.validate();
    assert!(violations.contains(&CoverageReviewViolation::MergeImpliesFalseCertainty));
}

#[test]
fn merge_must_disclose_omitted_scope() {
    let mut packet = valid_packet();
    for merge in &mut packet.merges {
        merge.omitted_scopes.clear();
    }
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::OmittedScopeCaseMissing));
}

#[test]
fn binary_snapshot_cannot_be_blind_accepted() {
    let mut packet = valid_packet();
    let binary = packet
        .snapshot_cards
        .iter_mut()
        .find(|c| c.card_id == "card:image")
        .unwrap();
    binary.decision = SnapshotReviewDecision::Accepted;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::SnapshotBlindAccept));
}

#[test]
fn imported_snapshot_cannot_be_accepted_locally() {
    let mut packet = valid_packet();
    let imported = packet
        .snapshot_cards
        .iter_mut()
        .find(|c| c.card_id == "card:imported")
        .unwrap();
    imported.decision = SnapshotReviewDecision::Accepted;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::ImportedReadsAsLocal));
}

#[test]
fn snapshot_subjects_keep_template_and_invocation_distinct() {
    let mut packet = valid_packet();
    for card in &mut packet.snapshot_cards {
        if card.subject.node_kind == DurableTestNodeKind::ParameterizedTemplate {
            card.subject.node_kind = DurableTestNodeKind::ConcreteInvocation;
        }
    }
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn raw_fallback_case_must_be_present() {
    let mut packet = valid_packet();
    let binary = packet
        .snapshot_cards
        .iter_mut()
        .find(|c| c.card_id == "card:image")
        .unwrap();
    binary.raw_fallback = RawFallbackAvailability::TextDiffAvailable;
    binary.decision = SnapshotReviewDecision::Rejected;
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::RawFallbackCaseMissing));
}

#[test]
fn scope_fingerprint_cannot_substitute_for_id() {
    let mut packet = valid_packet();
    let overlay = &mut packet.overlays[0];
    overlay.scope.scope_fingerprint_token = overlay.scope.scope_id.clone();
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = valid_packet();
    packet.source_contract_refs = refs(&[COVERAGE_REVIEW_SCHEMA_REF]);
    assert!(packet
        .validate()
        .contains(&CoverageReviewViolation::MissingSourceContracts));
}

#[test]
fn counts_and_helpers_reflect_packet() {
    let packet = valid_packet();
    assert_eq!(packet.imported_overlay_count(), 1);
    assert_eq!(packet.stale_overlay_count(), 1);
    assert_eq!(packet.open_snapshot_card_count(), 2);
    assert_eq!(
        packet.merge("merge:checkout").unwrap().excluded_run_count(),
        2
    );
}

#[test]
fn markdown_summary_renders_rows() {
    let packet = valid_packet();
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("Coverage Overlays And Snapshot / Golden Review"));
    assert!(summary.contains("overlay:imported"));
    assert!(summary.contains("merge:checkout"));
    assert!(summary.contains("card:image"));
}
