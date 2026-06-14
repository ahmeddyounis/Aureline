//! Conformance dump for the M5 coverage / snapshot-review packet.
//!
//! Prints the canonical support export (default), the Markdown summary
//! (`summary` argument), or the raw-fallback drill fixture (`fixture` argument) so
//! the checked-in artifacts stay byte-aligned with the in-crate builder.

use aureline_runtime::coverage_overlays_and_snapshot_golden_review::*;
use aureline_runtime::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use aureline_runtime::testing_identity::TestItemIdentityClass;

const PACKET_ID: &str = "coverage-review:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

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
        overlay_id: "overlay:verified:auth".to_owned(),
        scope: scope("crates/aureline-auth/src/login.rs", CoverageScopeKind::File),
        provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(184, 200),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "Covered line", 184),
            (CoverageCellClass::Uncovered, "Uncovered line", 12),
            (CoverageCellClass::NotExecutable, "Not executable", 4),
        ]),
        run_ref: "session:local:auth".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: true,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Verified current run; 92% line coverage on the login module.".to_owned(),
    }
}

fn verified_branch_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:verified:checkout-branch".to_owned(),
        scope: scope(
            "crates/aureline-commerce/src/checkout.rs",
            CoverageScopeKind::File,
        ),
        provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
        metric_mode: CoverageMetricMode::BranchCoverage,
        line_measure: measure(120, 140),
        branch_measure: Some(measure(34, 48)),
        branch_supported: true,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "Covered line", 120),
            (
                CoverageCellClass::PartiallyCoveredBranch,
                "Partially covered branch",
                14,
            ),
            (CoverageCellClass::Uncovered, "Uncovered line", 20),
        ]),
        run_ref: "session:local:checkout".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: true,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Verified branch coverage; 14 of 48 branches only partially taken."
            .to_owned(),
    }
}

fn changed_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:changed:diff".to_owned(),
        scope: scope("changed-set:pr-4821", CoverageScopeKind::ChangedSet),
        provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(58, 72),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: Some(ChangedLineEmphasis {
            changed_since_ref: "git:origin/main".to_owned(),
            changed_lines_total: 26,
            changed_lines_covered: 22,
        }),
        legend: legend(&[
            (CoverageCellClass::ChangedCovered, "Changed and covered", 22),
            (
                CoverageCellClass::ChangedUncovered,
                "Changed and uncovered",
                4,
            ),
            (CoverageCellClass::Covered, "Covered line", 36),
        ]),
        run_ref: "session:local:pr".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: true,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Changed-line emphasis over the diff; 22 of 26 changed lines covered."
            .to_owned(),
    }
}

fn imported_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:imported:nightly".to_owned(),
        scope: scope(
            "crates/aureline-render/src/pipeline.rs",
            CoverageScopeKind::File,
        ),
        provenance: CoverageEvidenceProvenance::ImportedCiArtifact,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(150, 240),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "Covered line", 150),
            (
                CoverageCellClass::ImportedUnverified,
                "Imported (not locally verified)",
                240,
            ),
        ]),
        run_ref: "import:ci:nightly-1042".to_owned(),
        origin_provider_ref: Some("provider:ci-nightly".to_owned()),
        presents_as_authoritative: false,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Imported CI coverage; held read-only and never presented as current."
            .to_owned(),
    }
}

fn cached_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:cached:editor".to_owned(),
        scope: scope(
            "crates/aureline-editor/src/buffer.rs",
            CoverageScopeKind::File,
        ),
        provenance: CoverageEvidenceProvenance::CachedLocalResult,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(210, 260),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[
            (CoverageCellClass::Covered, "Covered line (cached)", 210),
            (CoverageCellClass::Uncovered, "Uncovered line (cached)", 50),
        ]),
        run_ref: "session:local:cached-2026-06-10".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: false,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Cached local result reused from a prior run; not freshly re-measured."
            .to_owned(),
    }
}

fn stale_overlay() -> CoverageOverlayRecord {
    CoverageOverlayRecord {
        overlay_id: "overlay:stale:legacy".to_owned(),
        scope: scope(
            "crates/aureline-legacy/src/migrate.rs",
            CoverageScopeKind::File,
        ),
        provenance: CoverageEvidenceProvenance::StalePriorResult,
        metric_mode: CoverageMetricMode::LineCoverage,
        line_measure: measure(88, 90),
        branch_measure: None,
        branch_supported: false,
        changed_line_emphasis: None,
        legend: legend(&[(
            CoverageCellClass::StaleNotComparable,
            "Stale (not comparable as current)",
            90,
        )]),
        run_ref: "session:local:stale-2026-04-01".to_owned(),
        origin_provider_ref: None,
        presents_as_authoritative: false,
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Stale prior result beyond the freshness window; not current truth."
            .to_owned(),
    }
}

fn merge_review() -> CoverageMergeReview {
    CoverageMergeReview {
        merge_id: "merge:commerce:full".to_owned(),
        scope: scope("package:aureline-commerce", CoverageScopeKind::Package),
        metric_mode: CoverageMetricMode::LineCoverage,
        merged_measure: measure(1_420, 1_800),
        runs: vec![
            MergedRunEntry {
                run_ref: "session:local:shard-1".to_owned(),
                provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
                disposition: CoverageRunDisposition::Included,
                note: "Shard 1 merged.".to_owned(),
            },
            MergedRunEntry {
                run_ref: "session:local:shard-2".to_owned(),
                provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
                disposition: CoverageRunDisposition::Included,
                note: "Shard 2 merged.".to_owned(),
            },
            MergedRunEntry {
                run_ref: "import:ci:shard-2".to_owned(),
                provenance: CoverageEvidenceProvenance::ImportedCiArtifact,
                disposition: CoverageRunDisposition::ExcludedImportedIncomparable,
                note: "Imported CI shard not comparable to local instrumentation.".to_owned(),
            },
            MergedRunEntry {
                run_ref: "session:local:shard-2-rerun".to_owned(),
                provenance: CoverageEvidenceProvenance::VerifiedCurrentRun,
                disposition: CoverageRunDisposition::ExcludedDuplicate,
                note: "Duplicate of shard 2; excluded to avoid double counting.".to_owned(),
            },
        ],
        omitted_scopes: vec![
            OmittedScopeEntry {
                omission_kind: OmittedScopeKind::Shard,
                omitted_ref: "shard:3".to_owned(),
                reason: "Shard 3 did not report coverage in this run.".to_owned(),
            },
            OmittedScopeEntry {
                omission_kind: OmittedScopeKind::Platform,
                omitted_ref: "platform:windows-msvc".to_owned(),
                reason: "Windows platform was not part of this merge.".to_owned(),
            },
        ],
        implies_complete_certainty: false,
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Merge of two shards; one imported and one duplicate excluded, shard 3 and Windows omitted."
                .to_owned(),
    }
}

fn binary_snapshot_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:image:home".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:ui::home_renders".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:ui-home".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        artifact_kind: SnapshotArtifactKind::ImageSnapshot,
        changed_artifact_count: 3,
        total_artifact_count: 8,
        baseline_scope: SnapshotBaselineScope::PlatformSpecific,
        raw_fallback: RawFallbackAvailability::UnavailableBinaryOnly,
        decision: SnapshotReviewDecision::NeedsRawInspection,
        preview_first: true,
        imported: false,
        origin_provider_ref: None,
        diff_summary_ref: "diff:image:home".to_owned(),
        baseline_ref: "baseline:image:home".to_owned(),
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Binary image snapshot with no text fallback; routed to raw inspection, not blind accept."
                .to_owned(),
    }
}

fn text_snapshot_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:text:serialize".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:serialize::packet[*]".to_owned(),
            node_kind: DurableTestNodeKind::ParameterizedTemplate,
            subject_fingerprint_token: "fingerprint:serialize-packet".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        artifact_kind: SnapshotArtifactKind::TextSnapshot,
        changed_artifact_count: 2,
        total_artifact_count: 6,
        baseline_scope: SnapshotBaselineScope::PerParameterCase,
        raw_fallback: RawFallbackAvailability::TextDiffAvailable,
        decision: SnapshotReviewDecision::Accepted,
        preview_first: true,
        imported: false,
        origin_provider_ref: None,
        diff_summary_ref: "diff:text:serialize".to_owned(),
        baseline_ref: "baseline:text:serialize".to_owned(),
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Text snapshot reviewed against its diff and accepted preview-first."
            .to_owned(),
    }
}

fn golden_invocation_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:golden:report".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:report::render[case-2]".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:report-case-2".to_owned(),
            identity_class: TestItemIdentityClass::Stable,
        },
        artifact_kind: SnapshotArtifactKind::GoldenFile,
        changed_artifact_count: 1,
        total_artifact_count: 1,
        baseline_scope: SnapshotBaselineScope::PerTest,
        raw_fallback: RawFallbackAvailability::RawArtifactReferenced,
        decision: SnapshotReviewDecision::Rejected,
        preview_first: true,
        imported: false,
        origin_provider_ref: None,
        diff_summary_ref: "diff:golden:report".to_owned(),
        baseline_ref: "baseline:golden:report".to_owned(),
        captured_at: MINTED_AT.to_owned(),
        support_summary: "Golden file change rejected after raw-artifact inspection.".to_owned(),
    }
}

fn imported_snapshot_card() -> SnapshotReviewCard {
    SnapshotReviewCard {
        card_id: "card:imported:smoke".to_owned(),
        subject: SnapshotSubject {
            subject_id: "test:imported::smoke_golden".to_owned(),
            node_kind: DurableTestNodeKind::ConcreteInvocation,
            subject_fingerprint_token: "fingerprint:imported-smoke".to_owned(),
            identity_class: TestItemIdentityClass::ImportedReadOnly,
        },
        artifact_kind: SnapshotArtifactKind::SerializedSnapshot,
        changed_artifact_count: 1,
        total_artifact_count: 4,
        baseline_scope: SnapshotBaselineScope::SharedFixture,
        raw_fallback: RawFallbackAvailability::RawArtifactReferenced,
        decision: SnapshotReviewDecision::PendingReview,
        preview_first: true,
        imported: true,
        origin_provider_ref: Some("provider:ci-smoke".to_owned()),
        diff_summary_ref: "diff:imported:smoke".to_owned(),
        baseline_ref: "baseline:imported:smoke".to_owned(),
        captured_at: MINTED_AT.to_owned(),
        support_summary:
            "Imported snapshot held read-only; cannot be accepted as a local baseline.".to_owned(),
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
        COVERAGE_REVIEW_SUMMARY_REF,
    ])
}

fn packet() -> CoverageReviewPacket {
    CoverageReviewPacket::new(CoverageReviewPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Coverage Overlays And Snapshot / Golden Review".to_owned(),
        overlays: vec![
            verified_line_overlay(),
            verified_branch_overlay(),
            changed_overlay(),
            imported_overlay(),
            cached_overlay(),
            stale_overlay(),
        ],
        merges: vec![merge_review()],
        snapshot_cards: vec![
            binary_snapshot_card(),
            text_snapshot_card(),
            golden_invocation_card(),
            imported_snapshot_card(),
        ],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

/// Builds the raw-fallback drill fixture: a binary-only snapshot whose change cannot
/// be reviewed from a text diff routes through `needs_raw_inspection` instead of a
/// blind accept, alongside the imported card held read-only.
fn raw_fallback_drill_fixture() -> CoverageReviewPacket {
    CoverageReviewPacket::new(CoverageReviewPacketInput {
        packet_id: "coverage-review:fixture:raw-fallback-drill".to_owned(),
        label: "Binary snapshot requires raw inspection".to_owned(),
        overlays: vec![
            verified_line_overlay(),
            verified_branch_overlay(),
            changed_overlay(),
            imported_overlay(),
            cached_overlay(),
            stale_overlay(),
        ],
        merges: vec![merge_review()],
        snapshot_cards: vec![
            binary_snapshot_card(),
            text_snapshot_card(),
            imported_snapshot_card(),
        ],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());

    let packet = if which == "fixture" {
        raw_fallback_drill_fixture()
    } else {
        packet()
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
