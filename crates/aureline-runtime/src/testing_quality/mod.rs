//! Beta finalize layer for test-quality truth packets.
//!
//! This module pins the closed set of beta test-quality dimensions — coverage,
//! flaky verdict, snapshot review, and per-case baseline — and republishes the
//! tokens already carried on the alpha [`TestAttemptRecord`] into typed truth
//! packets that quote exact provenance, freshness, and support-class instead
//! of letting consumers infer those properties from log text.
//!
//! Every claimed beta test-case row references one packet per dimension. A
//! packet records:
//!
//! - the producing test attempt ref, the typed structured artifact identity
//!   ref (when applicable), and the producing run ref;
//! - the canonical test-item ref, selector ref, and test session ref the
//!   packet pertains to, so support exports and review flows can name the same
//!   identity the tree row, the inline editor row, and the rerun-last command
//!   already quote;
//! - the typed [`TestQualityFreshness`] freshness label and the typed
//!   [`TestQualitySupportClass`] support class, where rows without current
//!   evidence downgrade to `limited_imported_or_partial` or
//!   `retest_pending_no_current_packet` instead of implying stable support.
//!
//! The [`TestQualityRowTruth`] roll-up binds the four packets to a single
//! canonical test-item ref so support exports and review flows can point to
//! the same per-row truth referenced in-product.
//!
//! The machine-readable boundary lives at
//! [`/schemas/testing/test_quality_truth_beta.schema.json`](../../../../schemas/testing/test_quality_truth_beta.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/test_quality_truth_beta.md`](../../../../docs/runtime/m3/test_quality_truth_beta.md).

use serde::{Deserialize, Serialize};

use crate::testing::{
    TestArtifactIdentity, TestArtifactKind, TestRunnerBetaFramework, TestRunnerBetaProjection,
    TEST_RUNNER_BETA_SCHEMA_VERSION,
};
use crate::tests::{
    CoverageMergeClass, FlakyVerdictState, ImportedSignalAuthority, TestAttemptAlphaPacket,
    TestAttemptRecord, TestAttemptResultState, TestSourceDriftState,
};

/// Schema version the beta quality lane republishes; pinned to the canonical
/// alpha test-attempt schema so downstream consumers can correlate truth
/// without reading the alpha module first.
pub const TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION: u32 = TEST_RUNNER_BETA_SCHEMA_VERSION;

/// Stable record-kind tag for the beta coverage-truth packet.
pub const TEST_QUALITY_COVERAGE_PACKET_RECORD_KIND: &str =
    "test_quality_coverage_packet_record";

/// Stable record-kind tag for the beta flaky-truth packet.
pub const TEST_QUALITY_FLAKY_PACKET_RECORD_KIND: &str = "test_quality_flaky_packet_record";

/// Stable record-kind tag for the beta snapshot-truth packet.
pub const TEST_QUALITY_SNAPSHOT_PACKET_RECORD_KIND: &str =
    "test_quality_snapshot_packet_record";

/// Stable record-kind tag for the beta baseline-truth packet.
pub const TEST_QUALITY_BASELINE_PACKET_RECORD_KIND: &str =
    "test_quality_baseline_packet_record";

/// Stable record-kind tag for the per-row truth roll-up.
pub const TEST_QUALITY_ROW_TRUTH_RECORD_KIND: &str = "test_quality_row_truth_record";

/// Stable record-kind tag for the beta coverage manifest.
pub const TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "test_quality_beta_coverage_manifest_record";

/// Stable record-kind tag for the projection bundling every claimed packet.
pub const TEST_QUALITY_BETA_PROJECTION_RECORD_KIND: &str =
    "test_quality_beta_projection_record";

/// Stable record-kind tag for the beta support-export packet.
pub const TEST_QUALITY_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "test_quality_beta_support_export_record";

/// Closed kind vocabulary for the four quality dimensions a beta row claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQualityKind {
    /// Coverage truth packet.
    Coverage,
    /// Flaky-verdict truth packet.
    Flaky,
    /// Snapshot review truth packet.
    Snapshot,
    /// Per-case baseline truth packet.
    Baseline,
}

impl TestQualityKind {
    /// Every dimension claimed by the beta lane.
    pub const ALL: [Self; 4] = [Self::Coverage, Self::Flaky, Self::Snapshot, Self::Baseline];

    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Coverage => "coverage",
            Self::Flaky => "flaky",
            Self::Snapshot => "snapshot",
            Self::Baseline => "baseline",
        }
    }
}

/// Closed freshness vocabulary for a quality packet.
///
/// The label MUST be derived from explicit attempt provenance (the producing
/// attempt's result state, source-drift state, and imported-CI projection
/// class). It MUST NOT be inferred from log text or from the presence of an
/// artifact ref alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQualityFreshness {
    /// A current local attempt produced the packet.
    CurrentLocalEvidence,
    /// Authoritative imported provider evidence; read-only locally.
    AuthoritativeImportedReadOnly,
    /// Evidence is stale or outside its comparability window; a rerun is
    /// required before the packet can claim current support.
    StaleRequiresRetest,
    /// No packet has been produced yet for this row; the row downgrades to
    /// retest-pending.
    NoEvidenceRetestPending,
    /// Freshness cannot be classified; mutating actions and "stable support"
    /// claims fail closed.
    UnknownRequiresReview,
}

impl TestQualityFreshness {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLocalEvidence => "current_local_evidence",
            Self::AuthoritativeImportedReadOnly => "authoritative_imported_read_only",
            Self::StaleRequiresRetest => "stale_requires_retest",
            Self::NoEvidenceRetestPending => "no_evidence_retest_pending",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Closed support-class vocabulary the row roll-up downgrades to when a
/// packet's freshness regresses.
///
/// A row may only claim `stable_supported` when every claimed packet is
/// labelled `current_local_evidence` or `authoritative_imported_read_only`
/// AND the underlying attempt did not declare source drift. Otherwise the row
/// downgrades to `limited_imported_or_partial` (imported / partial scope) or
/// `retest_pending_no_current_packet` (missing / stale / drifted), so the
/// product never implies stable support for a row that does not have it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQualitySupportClass {
    /// Quality dimension is out of scope for this row.
    OutOfScope,
    /// A packet exists, is current local evidence, and binds the canonical
    /// identity; the row may claim stable support.
    StableSupported,
    /// A packet exists but is partial, imported-only, or read-only.
    LimitedImportedOrPartial,
    /// No current packet exists; the row downgrades to retest-pending.
    RetestPendingNoCurrentPacket,
    /// Support-class cannot be classified; the row fails closed.
    UnknownRequiresReview,
}

impl TestQualitySupportClass {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutOfScope => "out_of_scope",
            Self::StableSupported => "stable_supported",
            Self::LimitedImportedOrPartial => "limited_imported_or_partial",
            Self::RetestPendingNoCurrentPacket => "retest_pending_no_current_packet",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Order used by [`TestQualityRowTruth`] to roll the row-level support
    /// class down to the weakest packet support class.
    const fn severity(self) -> u8 {
        match self {
            Self::OutOfScope => 0,
            Self::StableSupported => 1,
            Self::LimitedImportedOrPartial => 2,
            Self::RetestPendingNoCurrentPacket => 3,
            Self::UnknownRequiresReview => 4,
        }
    }
}

/// Closed provenance-source vocabulary for a quality packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQualityProvenanceSource {
    /// No packet has been produced yet.
    NotEstablished,
    /// A local attempt produced the packet.
    LocalAttempt,
    /// Imported provider CI evidence produced the packet.
    ImportedProviderCi,
    /// A local rerun reconfirmed imported evidence.
    MergedLocalAndImported,
}

impl TestQualityProvenanceSource {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotEstablished => "not_established",
            Self::LocalAttempt => "local_attempt",
            Self::ImportedProviderCi => "imported_provider_ci",
            Self::MergedLocalAndImported => "merged_local_and_imported",
        }
    }
}

/// Identity row every quality packet quotes verbatim.
///
/// The fields mirror the identity the test-tree row, the inline editor row,
/// and the rerun-last command already use, so a support reviewer can join the
/// packet to the in-product row without parsing any free-form summary text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualityPacketIdentity {
    /// Canonical test-item ref the packet pertains to.
    pub canonical_test_item_ref: String,
    /// Selector ref the packet was produced for.
    pub selector_ref: String,
    /// Test session ref the packet belongs to.
    pub test_session_ref: String,
    /// Beta framework that owns the packet.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Execution-context ref used by the producing attempt, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    /// Resolved canonical target id used by the producing attempt, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// Coverage-truth packet for one beta test row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the alpha test-attempt model.
    pub schema_version: u32,
    /// Stable packet id.
    pub coverage_packet_id: String,
    /// Quality kind token; always `coverage` for this packet.
    pub quality_kind_token: String,
    /// Identity the packet pertains to.
    pub identity: TestQualityPacketIdentity,
    /// Producing test-attempt ref, when a packet has been established.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_test_attempt_ref: Option<String>,
    /// Producing execution-rail attempt ref, when a packet has been established.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_execution_attempt_ref: Option<String>,
    /// Coverage-report artifact identity ref, when one was published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_artifact_identity_ref: Option<String>,
    /// Coverage-merge class copied from the producing attempt.
    pub coverage_merge_token: String,
    /// Provenance-source label.
    pub provenance_source: TestQualityProvenanceSource,
    /// Stable provenance-source token.
    pub provenance_source_token: String,
    /// Imported signal authority class.
    pub imported_signal_authority: ImportedSignalAuthority,
    /// Stable imported-signal-authority token.
    pub imported_signal_authority_token: String,
    /// Freshness label.
    pub freshness: TestQualityFreshness,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Row support class derived from this packet alone.
    pub support_class: TestQualitySupportClass,
    /// Stable support-class token.
    pub support_class_token: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Flaky/stability-truth packet for one beta test row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlakyTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the alpha test-attempt model.
    pub schema_version: u32,
    /// Stable packet id.
    pub flaky_packet_id: String,
    /// Quality kind token; always `flaky` for this packet.
    pub quality_kind_token: String,
    /// Identity the packet pertains to.
    pub identity: TestQualityPacketIdentity,
    /// Producing test-attempt ref (latest attempt contributing to the verdict).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_test_attempt_ref: Option<String>,
    /// Attempt refs that contributed to the verdict.
    pub evidence_attempt_refs: Vec<String>,
    /// Number of attempts considered when computing the verdict.
    pub observation_window_attempts: u32,
    /// Verdict state copied from the producing attempt.
    pub flaky_verdict_state: FlakyVerdictState,
    /// Stable flaky-verdict token.
    pub flaky_verdict_token: String,
    /// Provenance-source label.
    pub provenance_source: TestQualityProvenanceSource,
    /// Stable provenance-source token.
    pub provenance_source_token: String,
    /// Imported signal authority class.
    pub imported_signal_authority: ImportedSignalAuthority,
    /// Stable imported-signal-authority token.
    pub imported_signal_authority_token: String,
    /// Freshness label.
    pub freshness: TestQualityFreshness,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Row support class derived from this packet alone.
    pub support_class: TestQualitySupportClass,
    /// Stable support-class token.
    pub support_class_token: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Snapshot-truth packet for one beta test row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the alpha test-attempt model.
    pub schema_version: u32,
    /// Stable packet id.
    pub snapshot_packet_id: String,
    /// Quality kind token; always `snapshot` for this packet.
    pub quality_kind_token: String,
    /// Identity the packet pertains to.
    pub identity: TestQualityPacketIdentity,
    /// Producing test-attempt ref, when a packet has been established.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_test_attempt_ref: Option<String>,
    /// Snapshot-diff artifact identity ref, when one was published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_diff_artifact_identity_ref: Option<String>,
    /// Snapshot review token copied from the producing attempt.
    pub snapshot_review_token: String,
    /// Provenance-source label.
    pub provenance_source: TestQualityProvenanceSource,
    /// Stable provenance-source token.
    pub provenance_source_token: String,
    /// Imported signal authority class.
    pub imported_signal_authority: ImportedSignalAuthority,
    /// Stable imported-signal-authority token.
    pub imported_signal_authority_token: String,
    /// Freshness label.
    pub freshness: TestQualityFreshness,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Row support class derived from this packet alone.
    pub support_class: TestQualitySupportClass,
    /// Stable support-class token.
    pub support_class_token: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Baseline-truth packet for one beta test row.
///
/// The baseline is the most recent attempt that produced a `passed` result on
/// the same canonical identity, in the same session. A row that has never
/// passed downgrades to `no_baseline_established_yet`; a row whose last
/// success is older than the latest source-drift event is reported as
/// `baseline_stale_requires_refresh`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the alpha test-attempt model.
    pub schema_version: u32,
    /// Stable packet id.
    pub baseline_packet_id: String,
    /// Quality kind token; always `baseline` for this packet.
    pub quality_kind_token: String,
    /// Identity the packet pertains to.
    pub identity: TestQualityPacketIdentity,
    /// Latest attempt ref considered when classifying the baseline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Baseline attempt ref, when one has been established.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_attempt_ref: Option<String>,
    /// Baseline-state token; one of `established_baseline_matches`,
    /// `regression_against_baseline`, `baseline_stale_requires_refresh`,
    /// `no_baseline_established_yet`, `baseline_unknown_requires_review`.
    pub baseline_state_token: String,
    /// True when the latest attempt regresses against the baseline.
    pub regression_against_baseline: bool,
    /// Provenance-source label.
    pub provenance_source: TestQualityProvenanceSource,
    /// Stable provenance-source token.
    pub provenance_source_token: String,
    /// Freshness label.
    pub freshness: TestQualityFreshness,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Row support class derived from this packet alone.
    pub support_class: TestQualitySupportClass,
    /// Stable support-class token.
    pub support_class_token: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Per-row truth roll-up binding the four packets to one canonical identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualityRowTruth {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable row-truth id.
    pub row_truth_id: String,
    /// Identity the row truth pertains to.
    pub identity: TestQualityPacketIdentity,
    /// Tree-row ref the roll-up mirrors.
    pub tree_row_ref: String,
    /// Inline-row ref the roll-up mirrors.
    pub inline_row_ref: String,
    /// Latest test-attempt ref considered for the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Coverage packet ref the row points to.
    pub coverage_packet_ref: String,
    /// Flaky packet ref the row points to.
    pub flaky_packet_ref: String,
    /// Snapshot packet ref the row points to.
    pub snapshot_packet_ref: String,
    /// Baseline packet ref the row points to.
    pub baseline_packet_ref: String,
    /// Row-level support class, rolled down to the weakest packet support
    /// class so the row never implies stable support that one packet does
    /// not back.
    pub row_support_class: TestQualitySupportClass,
    /// Stable row-support-class token.
    pub row_support_class_token: String,
    /// Reviewer-facing summary listing each packet's freshness label.
    pub summary: String,
}

/// One row of the [`TestQualityBetaCoverageManifest`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualityBetaCoverageRow {
    /// Beta framework the row applies to.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Quality kinds the framework's beta rows claim a packet for.
    pub claimed_quality_kinds: Vec<TestQualityKind>,
    /// Stable quality-kind tokens.
    pub claimed_quality_kind_tokens: Vec<String>,
    /// Artifact kinds that may back a packet.
    pub backing_artifact_kinds: Vec<TestArtifactKind>,
    /// Stable backing artifact-kind tokens.
    pub backing_artifact_kind_tokens: Vec<String>,
}

impl TestQualityBetaCoverageRow {
    /// Builds the canonical coverage row for one framework.
    pub fn canonical(framework: TestRunnerBetaFramework) -> Self {
        let claimed_quality_kinds = TestQualityKind::ALL.to_vec();
        let claimed_quality_kind_tokens = claimed_quality_kinds
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect();
        let backing_artifact_kinds = vec![
            TestArtifactKind::CoverageReport,
            TestArtifactKind::SnapshotDiff,
            TestArtifactKind::RunReport,
            TestArtifactKind::RawEventEnvelope,
        ];
        let backing_artifact_kind_tokens = backing_artifact_kinds
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect();
        Self {
            framework,
            framework_token: framework.as_str().to_owned(),
            claimed_quality_kinds,
            claimed_quality_kind_tokens,
            backing_artifact_kinds,
            backing_artifact_kind_tokens,
        }
    }
}

/// Beta coverage manifest pinning the closed quality-packet vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualityBetaCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Per-framework coverage rows.
    pub frameworks: Vec<TestQualityBetaCoverageRow>,
}

impl TestQualityBetaCoverageManifest {
    /// Builds the canonical beta coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            frameworks: TestRunnerBetaFramework::ALL
                .into_iter()
                .map(TestQualityBetaCoverageRow::canonical)
                .collect(),
        }
    }

    /// Returns the canonical row for one framework, if present.
    pub fn row_for_framework(
        &self,
        framework: TestRunnerBetaFramework,
    ) -> Option<&TestQualityBetaCoverageRow> {
        self.frameworks
            .iter()
            .find(|row| row.framework == framework)
    }
}

/// Beta projection bundling per-row packets and roll-ups.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualityProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Workspace id copied from the discovery context.
    pub workspace_id: String,
    /// Beta framework that owns the projection.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Tree-projection ref the row truths mirror.
    pub tree_projection_ref: String,
    /// Inline-projection ref the row truths mirror.
    pub inline_projection_ref: String,
    /// Coverage packets indexed in deterministic case order.
    pub coverage_packets: Vec<CoverageTruthPacket>,
    /// Flaky packets indexed in deterministic case order.
    pub flaky_packets: Vec<FlakyTruthPacket>,
    /// Snapshot packets indexed in deterministic case order.
    pub snapshot_packets: Vec<SnapshotTruthPacket>,
    /// Baseline packets indexed in deterministic case order.
    pub baseline_packets: Vec<BaselineTruthPacket>,
    /// Per-row roll-ups in deterministic case order.
    pub row_truths: Vec<TestQualityRowTruth>,
}

impl TestQualityProjection {
    /// Builds a quality projection from a beta test-runner projection.
    ///
    /// The producing tree projection, inline projection, and alpha test-
    /// attempt packets are read verbatim; this method does not invent
    /// truth — it republishes the tokens already on the attempt ledger into
    /// typed packets with explicit freshness and support class so consumers
    /// can join them to the in-product row.
    pub fn from_beta_projection(projection: &TestRunnerBetaProjection) -> Self {
        let framework = projection.framework;
        let workspace_id = projection.workspace_id.clone();
        let projection_id = format!(
            "test-quality-projection:{}:{}",
            stable_token(&workspace_id),
            stable_token(&projection.tree.tree_projection_id)
        );

        let mut coverage_packets = Vec::new();
        let mut flaky_packets = Vec::new();
        let mut snapshot_packets = Vec::new();
        let mut baseline_packets = Vec::new();
        let mut row_truths = Vec::new();

        let canonical_refs = canonical_refs_in_order(projection);
        for canonical in canonical_refs {
            let Some(tree_row) = projection.tree.case_row_for_test_item(&canonical) else {
                continue;
            };
            let Some(inline_row) = projection.inline.row_for_test_item(&canonical) else {
                continue;
            };

            let packet = packet_session_for_canonical(&projection.attempt_packets, &canonical);
            let attempts: Vec<&TestAttemptRecord> = packet
                .map(|packet| packet.attempts.iter().collect())
                .unwrap_or_default();
            let latest_attempt = attempts.last().copied();

            let identity = TestQualityPacketIdentity {
                canonical_test_item_ref: canonical.clone(),
                selector_ref: inline_row.selector_ref.clone(),
                test_session_ref: inline_row.test_session_ref.clone(),
                framework,
                framework_token: framework.as_str().to_owned(),
                execution_context_ref: latest_attempt
                    .map(|att| att.execution_context_ref.clone()),
                target_id: latest_attempt.map(|att| att.target_id.clone()),
            };

            let coverage = build_coverage_packet(
                &identity,
                latest_attempt,
                &projection.artifact_identities,
            );
            let flaky = build_flaky_packet(&identity, &attempts);
            let snapshot = build_snapshot_packet(
                &identity,
                latest_attempt,
                &projection.artifact_identities,
            );
            let baseline = build_baseline_packet(&identity, &attempts);

            let row_truth = TestQualityRowTruth {
                record_kind: TEST_QUALITY_ROW_TRUTH_RECORD_KIND.to_owned(),
                row_truth_id: format!(
                    "test-quality-row:{}:{}",
                    stable_token(&identity.test_session_ref),
                    stable_token(&canonical)
                ),
                identity: identity.clone(),
                tree_row_ref: tree_row.tree_row_id.clone(),
                inline_row_ref: inline_row.inline_row_id.clone(),
                latest_attempt_ref: latest_attempt.map(|att| att.test_attempt_id.clone()),
                coverage_packet_ref: coverage.coverage_packet_id.clone(),
                flaky_packet_ref: flaky.flaky_packet_id.clone(),
                snapshot_packet_ref: snapshot.snapshot_packet_id.clone(),
                baseline_packet_ref: baseline.baseline_packet_id.clone(),
                row_support_class: roll_up_support_class(&[
                    coverage.support_class,
                    flaky.support_class,
                    snapshot.support_class,
                    baseline.support_class,
                ]),
                row_support_class_token: String::new(),
                summary: format!(
                    "coverage={} flaky={} snapshot={} baseline={}",
                    coverage.freshness_token,
                    flaky.freshness_token,
                    snapshot.freshness_token,
                    baseline.freshness_token
                ),
            };
            let row_support_token = row_truth.row_support_class.as_str().to_owned();
            let row_truth = TestQualityRowTruth {
                row_support_class_token: row_support_token,
                ..row_truth
            };

            coverage_packets.push(coverage);
            flaky_packets.push(flaky);
            snapshot_packets.push(snapshot);
            baseline_packets.push(baseline);
            row_truths.push(row_truth);
        }

        Self {
            record_kind: TEST_QUALITY_BETA_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
            projection_id,
            workspace_id,
            framework,
            framework_token: framework.as_str().to_owned(),
            tree_projection_ref: projection.tree.tree_projection_id.clone(),
            inline_projection_ref: projection.inline.inline_projection_id.clone(),
            coverage_packets,
            flaky_packets,
            snapshot_packets,
            baseline_packets,
            row_truths,
        }
    }

    /// Returns the per-row truth for one canonical test-item ref.
    pub fn row_truth_for(&self, canonical_test_item_ref: &str) -> Option<&TestQualityRowTruth> {
        self.row_truths
            .iter()
            .find(|row| row.identity.canonical_test_item_ref == canonical_test_item_ref)
    }

    /// Returns true when every row in the projection rolls up to
    /// `stable_supported`.
    pub fn every_row_stable_supported(&self) -> bool {
        self.row_truths
            .iter()
            .all(|row| row.row_support_class == TestQualitySupportClass::StableSupported)
    }

    /// Returns true when at least one row downgrades away from
    /// `stable_supported`.
    pub fn any_row_downgraded(&self) -> bool {
        self.row_truths
            .iter()
            .any(|row| row.row_support_class != TestQualitySupportClass::StableSupported)
    }
}

/// Support-export packet binding the projection's manifest, projection ref,
/// every packet, and every row truth so a reviewer can join the in-product
/// row to the same packet referenced by the support flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualityBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Coverage manifest at export time.
    pub coverage_manifest: TestQualityBetaCoverageManifest,
    /// Quality-projection ref the export bundles.
    pub projection_ref: String,
    /// Underlying beta test-runner projection refs.
    pub tree_projection_ref: String,
    /// Underlying inline-projection ref.
    pub inline_projection_ref: String,
    /// Coverage packets included in the export.
    pub coverage_packets: Vec<CoverageTruthPacket>,
    /// Flaky packets included in the export.
    pub flaky_packets: Vec<FlakyTruthPacket>,
    /// Snapshot packets included in the export.
    pub snapshot_packets: Vec<SnapshotTruthPacket>,
    /// Baseline packets included in the export.
    pub baseline_packets: Vec<BaselineTruthPacket>,
    /// Per-row roll-ups included in the export.
    pub row_truths: Vec<TestQualityRowTruth>,
    /// Reviewer-facing summary lines.
    pub summary_lines: Vec<String>,
}

impl TestQualityBetaSupportExport {
    /// Builds a support-export packet bundling the projection and the manifest.
    pub fn from_projection(
        projection: &TestQualityProjection,
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> Self {
        let generated_at = generated_at.into();
        let coverage_manifest =
            TestQualityBetaCoverageManifest::canonical(manifest_id, generated_at.clone());
        let summary_lines = projection
            .row_truths
            .iter()
            .map(|row| {
                format!(
                    "framework={} session={} test={} row_support={} summary={}",
                    projection.framework_token,
                    row.identity.test_session_ref,
                    row.identity.canonical_test_item_ref,
                    row.row_support_class_token,
                    row.summary
                )
            })
            .collect();
        Self {
            record_kind: TEST_QUALITY_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
            support_export_id: format!(
                "test-quality-beta-support:{}:{}",
                stable_token(&projection.workspace_id),
                stable_token(&generated_at)
            ),
            generated_at,
            workspace_id: projection.workspace_id.clone(),
            coverage_manifest,
            projection_ref: projection.projection_id.clone(),
            tree_projection_ref: projection.tree_projection_ref.clone(),
            inline_projection_ref: projection.inline_projection_ref.clone(),
            coverage_packets: projection.coverage_packets.clone(),
            flaky_packets: projection.flaky_packets.clone(),
            snapshot_packets: projection.snapshot_packets.clone(),
            baseline_packets: projection.baseline_packets.clone(),
            row_truths: projection.row_truths.clone(),
            summary_lines,
        }
    }

    /// Renders deterministic plaintext lines for support / CLI consumers.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Test quality support export: {}\n",
            self.support_export_id
        );
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!("Projection: {}\n", self.projection_ref));
        out.push_str(&format!(
            "Quality kinds: {}\n",
            TestQualityKind::ALL
                .iter()
                .map(|kind| kind.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ));
        for line in &self.summary_lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

fn canonical_refs_in_order(projection: &TestRunnerBetaProjection) -> Vec<String> {
    let mut refs: Vec<String> = Vec::new();
    for row in &projection.inline.rows {
        if !refs.iter().any(|existing| existing == &row.canonical_test_item_ref) {
            refs.push(row.canonical_test_item_ref.clone());
        }
    }
    refs
}

fn packet_session_for_canonical<'a>(
    attempt_packets: &'a [TestAttemptAlphaPacket],
    canonical_test_item_ref: &str,
) -> Option<&'a TestAttemptAlphaPacket> {
    attempt_packets.iter().find(|packet| {
        packet
            .session_plan
            .canonical_test_item_refs
            .iter()
            .any(|item| item == canonical_test_item_ref)
            || packet
                .identity_projection
                .canonical_test_item_ref
                .as_deref()
                == Some(canonical_test_item_ref)
    })
}

fn artifact_identity_for_kind<'a>(
    artifact_identities: &'a [TestArtifactIdentity],
    canonical_test_item_ref: &str,
    kind: TestArtifactKind,
) -> Option<&'a TestArtifactIdentity> {
    artifact_identities.iter().find(|identity| {
        identity.artifact_kind == kind
            && identity
                .canonical_test_item_refs
                .iter()
                .any(|item| item == canonical_test_item_ref)
    })
}

fn build_coverage_packet(
    identity: &TestQualityPacketIdentity,
    latest_attempt: Option<&TestAttemptRecord>,
    artifact_identities: &[TestArtifactIdentity],
) -> CoverageTruthPacket {
    let coverage_artifact = artifact_identity_for_kind(
        artifact_identities,
        &identity.canonical_test_item_ref,
        TestArtifactKind::CoverageReport,
    );
    let (
        provenance_source,
        imported_authority,
        freshness,
        support_class,
        coverage_token,
        summary,
        producing_attempt_ref,
        producing_execution_attempt_ref,
    ) = match latest_attempt {
        None => (
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            CoverageMergeClass::CoverageUnknownRequiresReview
                .as_str()
                .to_owned(),
            "No attempt has produced a coverage packet yet.".to_owned(),
            None,
            None,
        ),
        Some(attempt) => {
            let coverage_token = attempt.coverage_merge_token.clone();
            let (provenance, authority, freshness, support_class, summary) =
                classify_coverage(attempt, coverage_artifact);
            (
                provenance,
                authority,
                freshness,
                support_class,
                coverage_token,
                summary,
                Some(attempt.test_attempt_id.clone()),
                attempt.execution_attempt_ref.clone(),
            )
        }
    };
    CoverageTruthPacket {
        record_kind: TEST_QUALITY_COVERAGE_PACKET_RECORD_KIND.to_owned(),
        schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
        coverage_packet_id: format!(
            "test-quality-coverage:{}:{}",
            stable_token(&identity.test_session_ref),
            stable_token(&identity.canonical_test_item_ref)
        ),
        quality_kind_token: TestQualityKind::Coverage.as_str().to_owned(),
        identity: identity.clone(),
        producing_test_attempt_ref: producing_attempt_ref,
        producing_execution_attempt_ref,
        coverage_artifact_identity_ref: coverage_artifact
            .map(|artifact| artifact.artifact_identity_id.clone()),
        coverage_merge_token: coverage_token,
        provenance_source,
        provenance_source_token: provenance_source.as_str().to_owned(),
        imported_signal_authority: imported_authority,
        imported_signal_authority_token: imported_authority.as_str().to_owned(),
        freshness,
        freshness_token: freshness.as_str().to_owned(),
        support_class,
        support_class_token: support_class.as_str().to_owned(),
        summary,
    }
}

fn classify_coverage(
    attempt: &TestAttemptRecord,
    coverage_artifact: Option<&TestArtifactIdentity>,
) -> (
    TestQualityProvenanceSource,
    ImportedSignalAuthority,
    TestQualityFreshness,
    TestQualitySupportClass,
    String,
) {
    match attempt.coverage_merge_class {
        CoverageMergeClass::NotRequested => (
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::OutOfScope,
            "Coverage was not requested for this attempt.".to_owned(),
        ),
        CoverageMergeClass::CoverageMerged => {
            let _coverage_artifact = coverage_artifact;
            let provenance = TestQualityProvenanceSource::LocalAttempt;
            let authority = ImportedSignalAuthority::None;
            let (freshness, support_class) = if attempt.source_drift_state
                == TestSourceDriftState::SourceChangedRequiresCurrentContextRerun
            {
                (
                    TestQualityFreshness::StaleRequiresRetest,
                    TestQualitySupportClass::RetestPendingNoCurrentPacket,
                )
            } else {
                (
                    TestQualityFreshness::CurrentLocalEvidence,
                    TestQualitySupportClass::StableSupported,
                )
            };
            (
                provenance,
                authority,
                freshness,
                support_class,
                "Coverage merged for the declared scope.".to_owned(),
            )
        }
        CoverageMergeClass::CoveragePartial => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::CurrentLocalEvidence,
            TestQualitySupportClass::LimitedImportedOrPartial,
            "Coverage is partial for the declared scope.".to_owned(),
        ),
        CoverageMergeClass::AuthoritativeImportedEvidence => (
            TestQualityProvenanceSource::ImportedProviderCi,
            ImportedSignalAuthority::AuthoritativeImportedEvidence,
            TestQualityFreshness::AuthoritativeImportedReadOnly,
            TestQualitySupportClass::LimitedImportedOrPartial,
            "Coverage is authoritative imported evidence; local rerun required for current truth."
                .to_owned(),
        ),
        CoverageMergeClass::StaleImportedEvidence => (
            TestQualityProvenanceSource::ImportedProviderCi,
            ImportedSignalAuthority::StaleImportedEvidence,
            TestQualityFreshness::StaleRequiresRetest,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            "Coverage evidence is stale; retest required before claiming support.".to_owned(),
        ),
        CoverageMergeClass::CoverageUnknownRequiresReview => (
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::UnknownRequiresReview,
            TestQualitySupportClass::UnknownRequiresReview,
            "Coverage cannot be classified; review required.".to_owned(),
        ),
    }
}

fn build_flaky_packet(
    identity: &TestQualityPacketIdentity,
    attempts: &[&TestAttemptRecord],
) -> FlakyTruthPacket {
    let latest = attempts.last().copied();
    let observation_window_attempts = attempts.len() as u32;
    let evidence_attempt_refs: Vec<String> = attempts
        .iter()
        .map(|att| att.test_attempt_id.clone())
        .collect();
    let (
        verdict_state,
        verdict_token,
        provenance,
        authority,
        freshness,
        support_class,
        summary,
        producing_attempt_ref,
    ) = match latest {
        None => (
            FlakyVerdictState::Unknown,
            FlakyVerdictState::Unknown.as_str().to_owned(),
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            "No attempt has produced a flaky verdict yet.".to_owned(),
            None,
        ),
        Some(attempt) => {
            let verdict_token = attempt.flaky_verdict_token.clone();
            let (provenance, authority, freshness, support_class, summary) =
                classify_flaky(attempt);
            (
                attempt.flaky_verdict_state,
                verdict_token,
                provenance,
                authority,
                freshness,
                support_class,
                summary,
                Some(attempt.test_attempt_id.clone()),
            )
        }
    };

    FlakyTruthPacket {
        record_kind: TEST_QUALITY_FLAKY_PACKET_RECORD_KIND.to_owned(),
        schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
        flaky_packet_id: format!(
            "test-quality-flaky:{}:{}",
            stable_token(&identity.test_session_ref),
            stable_token(&identity.canonical_test_item_ref)
        ),
        quality_kind_token: TestQualityKind::Flaky.as_str().to_owned(),
        identity: identity.clone(),
        producing_test_attempt_ref: producing_attempt_ref,
        evidence_attempt_refs,
        observation_window_attempts,
        flaky_verdict_state: verdict_state,
        flaky_verdict_token: verdict_token,
        provenance_source: provenance,
        provenance_source_token: provenance.as_str().to_owned(),
        imported_signal_authority: authority,
        imported_signal_authority_token: authority.as_str().to_owned(),
        freshness,
        freshness_token: freshness.as_str().to_owned(),
        support_class,
        support_class_token: support_class.as_str().to_owned(),
        summary,
    }
}

fn classify_flaky(
    attempt: &TestAttemptRecord,
) -> (
    TestQualityProvenanceSource,
    ImportedSignalAuthority,
    TestQualityFreshness,
    TestQualitySupportClass,
    String,
) {
    match attempt.flaky_verdict_state {
        FlakyVerdictState::StableAgain => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::CurrentLocalEvidence,
            TestQualitySupportClass::StableSupported,
            "Stability cleared through the evidence window.".to_owned(),
        ),
        FlakyVerdictState::SuspectedFlaky => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::CurrentLocalEvidence,
            TestQualitySupportClass::LimitedImportedOrPartial,
            "Flaky behaviour is suspected; reproduce before claiming support.".to_owned(),
        ),
        FlakyVerdictState::ReproducedFlaky => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::CurrentLocalEvidence,
            TestQualitySupportClass::LimitedImportedOrPartial,
            "Comparable attempts reproduced divergent outcomes.".to_owned(),
        ),
        FlakyVerdictState::Muted => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::CurrentLocalEvidence,
            TestQualitySupportClass::LimitedImportedOrPartial,
            "Delivery or execution is muted.".to_owned(),
        ),
        FlakyVerdictState::Unknown => (
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::UnknownRequiresReview,
            TestQualitySupportClass::UnknownRequiresReview,
            "Flaky verdict cannot be classified; review required.".to_owned(),
        ),
    }
}

fn build_snapshot_packet(
    identity: &TestQualityPacketIdentity,
    latest_attempt: Option<&TestAttemptRecord>,
    artifact_identities: &[TestArtifactIdentity],
) -> SnapshotTruthPacket {
    let snapshot_artifact = artifact_identity_for_kind(
        artifact_identities,
        &identity.canonical_test_item_ref,
        TestArtifactKind::SnapshotDiff,
    );
    let (
        review_token,
        provenance,
        authority,
        freshness,
        support_class,
        summary,
        producing_attempt_ref,
    ) = match latest_attempt {
        None => (
            "snapshot_review_unknown_requires_review".to_owned(),
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            "No attempt has produced a snapshot packet yet.".to_owned(),
            None,
        ),
        Some(attempt) => {
            let token = attempt.snapshot_review_token.clone();
            let (provenance, authority, freshness, support_class, summary) =
                classify_snapshot(attempt, snapshot_artifact);
            (
                token,
                provenance,
                authority,
                freshness,
                support_class,
                summary,
                Some(attempt.test_attempt_id.clone()),
            )
        }
    };
    SnapshotTruthPacket {
        record_kind: TEST_QUALITY_SNAPSHOT_PACKET_RECORD_KIND.to_owned(),
        schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
        snapshot_packet_id: format!(
            "test-quality-snapshot:{}:{}",
            stable_token(&identity.test_session_ref),
            stable_token(&identity.canonical_test_item_ref)
        ),
        quality_kind_token: TestQualityKind::Snapshot.as_str().to_owned(),
        identity: identity.clone(),
        producing_test_attempt_ref: producing_attempt_ref,
        snapshot_diff_artifact_identity_ref: snapshot_artifact
            .map(|artifact| artifact.artifact_identity_id.clone()),
        snapshot_review_token: review_token,
        provenance_source: provenance,
        provenance_source_token: provenance.as_str().to_owned(),
        imported_signal_authority: authority,
        imported_signal_authority_token: authority.as_str().to_owned(),
        freshness,
        freshness_token: freshness.as_str().to_owned(),
        support_class,
        support_class_token: support_class.as_str().to_owned(),
        summary,
    }
}

fn classify_snapshot(
    attempt: &TestAttemptRecord,
    snapshot_artifact: Option<&TestArtifactIdentity>,
) -> (
    TestQualityProvenanceSource,
    ImportedSignalAuthority,
    TestQualityFreshness,
    TestQualitySupportClass,
    String,
) {
    match attempt.snapshot_review_token.as_str() {
        "not_required" => (
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::OutOfScope,
            "Snapshot review is not in scope for this attempt.".to_owned(),
        ),
        "snapshot_review_required" => {
            let _artifact = snapshot_artifact;
            (
                TestQualityProvenanceSource::LocalAttempt,
                ImportedSignalAuthority::None,
                TestQualityFreshness::CurrentLocalEvidence,
                TestQualitySupportClass::LimitedImportedOrPartial,
                "Snapshot diff requires review.".to_owned(),
            )
        }
        "snapshot_review_blocked" => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::StaleRequiresRetest,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            "Snapshot review is blocked by policy or missing preview.".to_owned(),
        ),
        "snapshot_review_completed" => (
            TestQualityProvenanceSource::LocalAttempt,
            ImportedSignalAuthority::None,
            TestQualityFreshness::CurrentLocalEvidence,
            TestQualitySupportClass::StableSupported,
            "Snapshot review completed for the declared scope.".to_owned(),
        ),
        _ => (
            TestQualityProvenanceSource::NotEstablished,
            ImportedSignalAuthority::None,
            TestQualityFreshness::UnknownRequiresReview,
            TestQualitySupportClass::UnknownRequiresReview,
            "Snapshot review state cannot be classified; review required.".to_owned(),
        ),
    }
}

fn build_baseline_packet(
    identity: &TestQualityPacketIdentity,
    attempts: &[&TestAttemptRecord],
) -> BaselineTruthPacket {
    let latest = attempts.last().copied();
    let baseline_attempt = attempts
        .iter()
        .rev()
        .copied()
        .find(|attempt| attempt.result_state == TestAttemptResultState::Passed);

    let (
        baseline_state_token,
        regression,
        provenance,
        freshness,
        support_class,
        summary,
        baseline_attempt_ref,
    ) = match (latest, baseline_attempt) {
        (None, _) => (
            "no_baseline_established_yet".to_owned(),
            false,
            TestQualityProvenanceSource::NotEstablished,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            "No attempt has been recorded; baseline pending first run.".to_owned(),
            None,
        ),
        (Some(_latest), None) => (
            "no_baseline_established_yet".to_owned(),
            false,
            TestQualityProvenanceSource::NotEstablished,
            TestQualityFreshness::NoEvidenceRetestPending,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            "No passing attempt has been recorded; baseline pending.".to_owned(),
            None,
        ),
        (Some(latest), Some(baseline)) => {
            let stale = latest.source_drift_state
                == TestSourceDriftState::SourceChangedRequiresCurrentContextRerun
                || latest.source_drift_state
                    == TestSourceDriftState::SourceDriftUnknownRequiresReview;
            let regression = matches!(
                latest.result_state,
                TestAttemptResultState::Failed | TestAttemptResultState::ImportedFailed
            );
            let token = if stale {
                "baseline_stale_requires_refresh".to_owned()
            } else if regression {
                "regression_against_baseline".to_owned()
            } else {
                "established_baseline_matches".to_owned()
            };
            let (freshness, support_class, summary) = if stale {
                (
                    TestQualityFreshness::StaleRequiresRetest,
                    TestQualitySupportClass::RetestPendingNoCurrentPacket,
                    "Baseline is stale; source changed since the last passing attempt.".to_owned(),
                )
            } else if regression {
                (
                    TestQualityFreshness::CurrentLocalEvidence,
                    TestQualitySupportClass::LimitedImportedOrPartial,
                    "Latest attempt regressed against the established baseline.".to_owned(),
                )
            } else {
                (
                    TestQualityFreshness::CurrentLocalEvidence,
                    TestQualitySupportClass::StableSupported,
                    "Latest attempt matches the established baseline.".to_owned(),
                )
            };
            (
                token,
                regression,
                TestQualityProvenanceSource::LocalAttempt,
                freshness,
                support_class,
                summary,
                Some(baseline.test_attempt_id.clone()),
            )
        }
    };

    BaselineTruthPacket {
        record_kind: TEST_QUALITY_BASELINE_PACKET_RECORD_KIND.to_owned(),
        schema_version: TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
        baseline_packet_id: format!(
            "test-quality-baseline:{}:{}",
            stable_token(&identity.test_session_ref),
            stable_token(&identity.canonical_test_item_ref)
        ),
        quality_kind_token: TestQualityKind::Baseline.as_str().to_owned(),
        identity: identity.clone(),
        latest_attempt_ref: latest.map(|att| att.test_attempt_id.clone()),
        baseline_attempt_ref,
        baseline_state_token,
        regression_against_baseline: regression,
        provenance_source: provenance,
        provenance_source_token: provenance.as_str().to_owned(),
        freshness,
        freshness_token: freshness.as_str().to_owned(),
        support_class,
        support_class_token: support_class.as_str().to_owned(),
        summary,
    }
}

fn roll_up_support_class(classes: &[TestQualitySupportClass]) -> TestQualitySupportClass {
    let non_scope: Vec<TestQualitySupportClass> = classes
        .iter()
        .copied()
        .filter(|class| *class != TestQualitySupportClass::OutOfScope)
        .collect();
    if non_scope.is_empty() {
        return TestQualitySupportClass::OutOfScope;
    }
    non_scope
        .into_iter()
        .max_by_key(|class| class.severity())
        .unwrap_or(TestQualitySupportClass::UnknownRequiresReview)
}

fn stable_token(raw: &str) -> String {
    let mut token = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('_') {
            token.push('_');
        }
    }
    let token = token.trim_matches('_').to_owned();
    if token.is_empty() {
        "unnamed".to_owned()
    } else {
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_manifest_pins_all_four_quality_kinds_for_pytest() {
        let manifest = TestQualityBetaCoverageManifest::canonical(
            "test-quality-truth-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert_eq!(
            manifest.record_kind,
            TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND
        );
        let row = manifest
            .row_for_framework(TestRunnerBetaFramework::Pytest)
            .expect("pytest row");
        for kind in TestQualityKind::ALL {
            assert!(row.claimed_quality_kind_tokens.contains(&kind.as_str().to_owned()));
        }
        assert!(row
            .backing_artifact_kind_tokens
            .contains(&"coverage_report".to_owned()));
        assert!(row
            .backing_artifact_kind_tokens
            .contains(&"snapshot_diff".to_owned()));
    }

    #[test]
    fn roll_up_support_class_picks_the_weakest_non_out_of_scope_class() {
        let class = roll_up_support_class(&[
            TestQualitySupportClass::OutOfScope,
            TestQualitySupportClass::StableSupported,
            TestQualitySupportClass::LimitedImportedOrPartial,
            TestQualitySupportClass::StableSupported,
        ]);
        assert_eq!(class, TestQualitySupportClass::LimitedImportedOrPartial);

        let class = roll_up_support_class(&[
            TestQualitySupportClass::StableSupported,
            TestQualitySupportClass::RetestPendingNoCurrentPacket,
            TestQualitySupportClass::LimitedImportedOrPartial,
        ]);
        assert_eq!(class, TestQualitySupportClass::RetestPendingNoCurrentPacket);

        let class = roll_up_support_class(&[
            TestQualitySupportClass::OutOfScope,
            TestQualitySupportClass::OutOfScope,
        ]);
        assert_eq!(class, TestQualitySupportClass::OutOfScope);
    }
}
