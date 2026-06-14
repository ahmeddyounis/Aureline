//! Coverage legends and overlays, import / merge review, and snapshot / golden
//! review cards with a raw fallback for the M5 test-intelligence lane.
//!
//! Where [`crate::stability_verdicts_quarantines_and_release_visibility`] governs
//! whether a *test* is trustworthy, this module governs whether the **coverage and
//! snapshot evidence** drawn over the editor and review surfaces is trustworthy.
//! Coverage is no longer a single green percentage and a snapshot change is no
//! longer a blind accept-all: both become inspectable, export-safe records.
//!
//! * a [`CoverageOverlayRecord`] carries a controlled
//!   [`CoverageEvidenceProvenance`] so a verified current run, an imported CI
//!   artifact, a cached local result, and a stale prior result never read as the
//!   same authoritative green. It carries a [`CoverageMetricMode`] and an explicit
//!   line measure plus an *optional* branch measure, so branch-versus-line truth is
//!   structural — a legend can never emphasize branch coverage it does not have. A
//!   [`ChangedLineEmphasis`] block records changed-line coverage distinctly from
//!   whole-scope coverage, and a [`CoverageLegendEntry`] list keeps the gutter
//!   legend honest about imported, stale, and changed cells;
//! * a [`CoverageMergeReview`] keeps a merge sheet honest: every contributing run
//!   is an explicit [`MergedRunEntry`] with an [`CoverageRunDisposition`]
//!   (included or one of the excluded reasons), omitted shards and platforms are
//!   disclosed as [`OmittedScopeEntry`] rows, and duplicate / conflict notes
//!   survive export. A merge that omits or excludes anything may never claim
//!   complete certainty;
//! * a [`SnapshotReviewCard`] preserves the [`SnapshotArtifactKind`], the changed /
//!   total artifact counts, the [`SnapshotBaselineScope`], and a
//!   [`RawFallbackAvailability`] before any [`SnapshotReviewDecision`]. A
//!   binary-only artifact with no raw / text fallback can never be blind-accepted —
//!   it must route through [`SnapshotReviewDecision::NeedsRawInspection`] — and an
//!   imported snapshot is held read-only rather than applied as a local accept.
//!
//! [`CoverageReviewPacket::validate`] refuses a packet that lets an imported or
//! stale overlay present as a verified-current green, emphasizes branch coverage it
//! does not measure, drops changed-line emphasis, implies a merge is complete while
//! omitting shards or excluding runs, blind-accepts a snapshot without a raw
//! fallback, lets an imported snapshot read as a local accept, or collapses a
//! parameterized snapshot template into a concrete invocation.
//!
//! Raw coverage payloads, snapshot bytes, golden-file bodies, baseline diffs, raw
//! provider payloads, provider cursors, credentials, and host names never cross
//! this boundary; the packet carries only typed class tokens, booleans, counts,
//! opaque ids, fingerprint digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json`](../../../../schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json).
//! The contract doc is
//! [`docs/testing/m5/coverage-overlays-and-snapshot-golden-review.md`](../../../../docs/testing/m5/coverage-overlays-and-snapshot-golden-review.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/coverage-overlays-and-snapshot-golden-review/`](../../../../fixtures/testing/m5/coverage-overlays-and-snapshot-golden-review/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

/// Stable record-kind tag carried by [`CoverageReviewPacket`].
pub const COVERAGE_REVIEW_RECORD_KIND: &str = "test_coverage_review_packet";

/// Schema version for the coverage / snapshot-review packet.
pub const COVERAGE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COVERAGE_REVIEW_SCHEMA_REF: &str =
    "schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json";

/// Repo-relative path of the contract doc.
pub const COVERAGE_REVIEW_DOC_REF: &str =
    "docs/testing/m5/coverage-overlays-and-snapshot-golden-review.md";

/// Repo-relative path of the checked support-export artifact.
pub const COVERAGE_REVIEW_ARTIFACT_REF: &str =
    "artifacts/testing/m5/coverage-overlays-and-snapshot-golden-review/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const COVERAGE_REVIEW_SUMMARY_REF: &str =
    "artifacts/testing/m5/coverage-overlays-and-snapshot-golden-review.md";

/// Repo-relative path of the protected fixture directory.
pub const COVERAGE_REVIEW_FIXTURE_DIR: &str =
    "fixtures/testing/m5/coverage-overlays-and-snapshot-golden-review";

/// Provenance of the coverage evidence an overlay or merged run draws. This is the
/// anchor that keeps a verified current run, an imported CI artifact, a cached local
/// result, and a stale prior result from reading as the same authoritative green.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageEvidenceProvenance {
    /// Measured by the verified current local run; the only provenance that may
    /// present as authoritative green.
    VerifiedCurrentRun,
    /// Imported from an external CI artifact; read-only and never a local rerun.
    ImportedCiArtifact,
    /// A cached local result reused from a prior run; correct target but not freshly
    /// re-measured.
    CachedLocalResult,
    /// A stale prior result older than the freshness window; not comparable as
    /// current truth.
    StalePriorResult,
    /// Provenance cannot be classified; treated as non-authoritative.
    UnknownRequiresReview,
}

impl CoverageEvidenceProvenance {
    /// Every provenance, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::VerifiedCurrentRun,
        Self::ImportedCiArtifact,
        Self::CachedLocalResult,
        Self::StalePriorResult,
        Self::UnknownRequiresReview,
    ];

    /// The four provenances coverage surfaces must keep distinguishable.
    pub const REQUIRED: [Self; 4] = [
        Self::VerifiedCurrentRun,
        Self::ImportedCiArtifact,
        Self::CachedLocalResult,
        Self::StalePriorResult,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrentRun => "verified_current_run",
            Self::ImportedCiArtifact => "imported_ci_artifact",
            Self::CachedLocalResult => "cached_local_result",
            Self::StalePriorResult => "stale_prior_result",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether this provenance is imported / provider-backed and read-only.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedCiArtifact)
    }

    /// Whether this provenance is stale prior evidence.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StalePriorResult)
    }

    /// Whether evidence at this provenance may present as authoritative green. Only a
    /// verified current run may; imported, cached, stale, or unknown evidence must
    /// stay visibly non-authoritative.
    pub const fn permits_authoritative_green(self) -> bool {
        matches!(self, Self::VerifiedCurrentRun)
    }
}

/// Whether an overlay or merge emphasizes line or branch coverage. This is the
/// branch-versus-line truth anchor: an overlay carries an explicit line measure and
/// an optional branch measure, and may only emphasize the mode it actually measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageMetricMode {
    /// The overlay / legend emphasizes line coverage.
    LineCoverage,
    /// The overlay / legend emphasizes branch coverage.
    BranchCoverage,
}

impl CoverageMetricMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LineCoverage => "line_coverage",
            Self::BranchCoverage => "branch_coverage",
        }
    }
}

/// One cell class in a coverage legend / gutter overlay. The legend vocabulary keeps
/// the gutter honest about covered, uncovered, partial-branch, changed, imported,
/// and stale cells rather than collapsing everything into "green / red".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageCellClass {
    /// Executable and covered.
    Covered,
    /// Executable and not covered.
    Uncovered,
    /// A branch point only partially covered (some arms taken).
    PartiallyCoveredBranch,
    /// Not executable (blank, comment, or declaration).
    NotExecutable,
    /// A changed line that is covered.
    ChangedCovered,
    /// A changed line that is not covered.
    ChangedUncovered,
    /// Coverage drawn from imported / provider evidence, not locally verified.
    ImportedUnverified,
    /// Coverage that is stale and not comparable as current truth.
    StaleNotComparable,
}

impl CoverageCellClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::Uncovered => "uncovered",
            Self::PartiallyCoveredBranch => "partially_covered_branch",
            Self::NotExecutable => "not_executable",
            Self::ChangedCovered => "changed_covered",
            Self::ChangedUncovered => "changed_uncovered",
            Self::ImportedUnverified => "imported_unverified",
            Self::StaleNotComparable => "stale_not_comparable",
        }
    }
}

/// Scope kind a coverage overlay or merge addresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageScopeKind {
    /// A single file.
    File,
    /// A module or namespace.
    Module,
    /// A package or crate.
    Package,
    /// The changed-set across a diff.
    ChangedSet,
}

impl CoverageScopeKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Module => "module",
            Self::Package => "package",
            Self::ChangedSet => "changed_set",
        }
    }
}

/// Disposition of one run contributing to a coverage merge sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageRunDisposition {
    /// The run was merged into the result.
    Included,
    /// Excluded: the run covered a different scope than the merge target.
    ExcludedScopeMismatch,
    /// Excluded: the run was stale relative to the freshness window.
    ExcludedStale,
    /// Excluded: the run was imported and not comparable to local instrumentation.
    ExcludedImportedIncomparable,
    /// Excluded: the run duplicated another included run.
    ExcludedDuplicate,
    /// Excluded: the run conflicted with another run on overlapping units.
    ExcludedConflict,
}

impl CoverageRunDisposition {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::ExcludedScopeMismatch => "excluded_scope_mismatch",
            Self::ExcludedStale => "excluded_stale",
            Self::ExcludedImportedIncomparable => "excluded_imported_incomparable",
            Self::ExcludedDuplicate => "excluded_duplicate",
            Self::ExcludedConflict => "excluded_conflict",
        }
    }

    /// Whether the run was merged into the result.
    pub const fn is_included(self) -> bool {
        matches!(self, Self::Included)
    }

    /// Whether the run was a duplicate or conflict (which must carry a disclosed
    /// note rather than being silently dropped).
    pub const fn is_duplicate_or_conflict(self) -> bool {
        matches!(self, Self::ExcludedDuplicate | Self::ExcludedConflict)
    }
}

/// Whether an omitted merge scope is a shard or a platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmittedScopeKind {
    /// A test shard whose results are not part of the merge.
    Shard,
    /// A platform / target whose results are not part of the merge.
    Platform,
}

impl OmittedScopeKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shard => "shard",
            Self::Platform => "platform",
        }
    }
}

/// Kind of snapshot / golden artifact under review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotArtifactKind {
    /// A rendered image snapshot.
    ImageSnapshot,
    /// A textual snapshot (serialized text).
    TextSnapshot,
    /// A structured / serialized snapshot (e.g. JSON inline snapshot).
    SerializedSnapshot,
    /// A golden text / data file baseline.
    GoldenFile,
    /// A binary golden artifact with no inherent text form.
    BinaryGolden,
}

impl SnapshotArtifactKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImageSnapshot => "image_snapshot",
            Self::TextSnapshot => "text_snapshot",
            Self::SerializedSnapshot => "serialized_snapshot",
            Self::GoldenFile => "golden_file",
            Self::BinaryGolden => "binary_golden",
        }
    }

    /// Whether the artifact is inherently binary (image or binary golden), so it
    /// cannot be reviewed from a text diff alone.
    pub const fn is_binary(self) -> bool {
        matches!(self, Self::ImageSnapshot | Self::BinaryGolden)
    }
}

/// Scope a snapshot / golden baseline covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotBaselineScope {
    /// Baseline is per concrete test.
    PerTest,
    /// Baseline is per parameter case of a template.
    PerParameterCase,
    /// Baseline is shared across a fixture / suite.
    SharedFixture,
    /// Baseline is platform-specific.
    PlatformSpecific,
}

impl SnapshotBaselineScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerTest => "per_test",
            Self::PerParameterCase => "per_parameter_case",
            Self::SharedFixture => "shared_fixture",
            Self::PlatformSpecific => "platform_specific",
        }
    }
}

/// Whether a raw / text fallback is available for inspecting a snapshot change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawFallbackAvailability {
    /// A text diff of the change is available for review.
    TextDiffAvailable,
    /// A raw artifact reference is available for side-by-side inspection.
    RawArtifactReferenced,
    /// No text or raw fallback is available (binary-only); the change cannot be
    /// blind-accepted and must route through raw inspection.
    UnavailableBinaryOnly,
}

impl RawFallbackAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextDiffAvailable => "text_diff_available",
            Self::RawArtifactReferenced => "raw_artifact_referenced",
            Self::UnavailableBinaryOnly => "unavailable_binary_only",
        }
    }

    /// Whether a fallback that supports a reviewed accept is present.
    pub const fn supports_reviewed_accept(self) -> bool {
        matches!(self, Self::TextDiffAvailable | Self::RawArtifactReferenced)
    }
}

/// Decision recorded on a snapshot / golden review card. A change is preview-first;
/// there is no blind accept-all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotReviewDecision {
    /// Awaiting review; nothing applied.
    PendingReview,
    /// Accepted after preview; the baseline is updated.
    Accepted,
    /// Rejected; the baseline is unchanged.
    Rejected,
    /// Requires raw inspection before any accept (e.g. binary-only with no diff).
    NeedsRawInspection,
}

impl SnapshotReviewDecision {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::NeedsRawInspection => "needs_raw_inspection",
        }
    }

    /// Whether this decision applies (mutates) the baseline.
    pub const fn is_applied(self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// A measured coverage ratio expressed as covered over total units (lines or
/// branches). Counts only; raw payloads never cross the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageMeasure {
    /// Covered executable units.
    pub covered_units: u32,
    /// Total executable units.
    pub total_units: u32,
}

impl CoverageMeasure {
    /// Whether the measure is well-formed (covered never exceeds total).
    pub const fn is_valid(self) -> bool {
        self.covered_units <= self.total_units
    }

    /// Covered percentage rounded down, or 100 when there is nothing to cover.
    pub const fn percent(self) -> u32 {
        if self.total_units == 0 {
            100
        } else {
            (self.covered_units * 100) / self.total_units
        }
    }
}

/// Changed-line emphasis for a coverage overlay: coverage of the lines changed since
/// a diff base, kept distinct from whole-scope coverage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangedLineEmphasis {
    /// Diff base the changed-line set is computed against.
    pub changed_since_ref: String,
    /// Total changed executable lines in scope.
    pub changed_lines_total: u32,
    /// Changed executable lines that are covered.
    pub changed_lines_covered: u32,
}

impl ChangedLineEmphasis {
    /// Whether the changed-line counts are well-formed.
    pub fn is_valid(&self) -> bool {
        !self.changed_since_ref.trim().is_empty()
            && self.changed_lines_covered <= self.changed_lines_total
    }

    /// Coverage measure over the changed lines.
    pub const fn measure(&self) -> CoverageMeasure {
        CoverageMeasure {
            covered_units: self.changed_lines_covered,
            total_units: self.changed_lines_total,
        }
    }
}

/// One legend entry mapping a cell class to a label and a gutter count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageLegendEntry {
    /// Cell class this legend swatch documents.
    pub cell_class: CoverageCellClass,
    /// Reviewable label for the swatch.
    pub label: String,
    /// Count of cells in this class within the overlay scope.
    pub gutter_count: u32,
}

impl CoverageLegendEntry {
    /// Whether the entry carries a non-empty label.
    pub fn is_valid(&self) -> bool {
        !self.label.trim().is_empty()
    }
}

/// Durable scope a coverage overlay / merge addresses, keyed by a non-display
/// fingerprint rather than a label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageScope {
    /// Durable scope id.
    pub scope_id: String,
    /// Scope kind.
    pub scope_kind: CoverageScopeKind,
    /// Non-display fingerprint token. Must differ from
    /// [`scope_id`](CoverageScope::scope_id).
    pub scope_fingerprint_token: String,
}

impl CoverageScope {
    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.scope_fingerprint_token.trim();
        !token.is_empty() && token != self.scope_id.trim()
    }

    /// Whether the scope carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.scope_id.trim().is_empty() && self.fingerprint_independent_of_id()
    }
}

/// A coverage legend + overlay for one scope, backed by stable evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageOverlayRecord {
    /// Stable overlay id.
    pub overlay_id: String,
    /// Durable scope the overlay covers.
    pub scope: CoverageScope,
    /// Provenance of the coverage evidence.
    pub provenance: CoverageEvidenceProvenance,
    /// Metric mode the legend emphasizes.
    pub metric_mode: CoverageMetricMode,
    /// Line coverage measure (always present).
    pub line_measure: CoverageMeasure,
    /// Branch coverage measure, present iff branch coverage is supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_measure: Option<CoverageMeasure>,
    /// Whether branch coverage is supported for this scope.
    pub branch_supported: bool,
    /// Changed-line emphasis, present when the overlay emphasizes a diff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_line_emphasis: Option<ChangedLineEmphasis>,
    /// Gutter legend entries.
    pub legend: Vec<CoverageLegendEntry>,
    /// Run / session ref that produced the overlay (reconstructable elsewhere).
    pub run_ref: String,
    /// Origin provider ref, present iff the overlay is imported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_provider_ref: Option<String>,
    /// Whether the overlay presents as authoritative current green. Only a verified
    /// current run may.
    pub presents_as_authoritative: bool,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe overlay summary.
    pub support_summary: String,
}

impl CoverageOverlayRecord {
    /// Whether the imported markers agree: an imported overlay carries an origin
    /// provider ref and an `imported_unverified` legend cell, and a non-imported
    /// overlay carries neither.
    pub fn imported_markers_consistent(&self) -> bool {
        if self.provenance.is_imported() {
            self.origin_provider_ref.is_some()
                && self.legend_has(CoverageCellClass::ImportedUnverified)
        } else {
            self.origin_provider_ref.is_none()
                && !self.legend_has(CoverageCellClass::ImportedUnverified)
        }
    }

    /// Whether the branch markers agree: a supported branch measure is present iff
    /// `branch_supported`, and a branch-mode overlay actually measures branches.
    pub fn branch_markers_consistent(&self) -> bool {
        if self.branch_supported != self.branch_measure.is_some() {
            return false;
        }
        if self.metric_mode == CoverageMetricMode::BranchCoverage {
            return self.branch_supported;
        }
        true
    }

    /// Whether the authoritative-green claim respects provenance: only a verified
    /// current run may present as authoritative, and a stale overlay always discloses
    /// a `stale_not_comparable` legend cell.
    pub fn authority_consistent(&self) -> bool {
        if self.presents_as_authoritative && !self.provenance.permits_authoritative_green() {
            return false;
        }
        if self.provenance.is_stale() && !self.legend_has(CoverageCellClass::StaleNotComparable) {
            return false;
        }
        true
    }

    /// Whether the changed-line emphasis is reflected in the legend: an overlay that
    /// emphasizes changed lines discloses a changed cell class.
    pub fn changed_line_markers_consistent(&self) -> bool {
        match &self.changed_line_emphasis {
            Some(emphasis) => {
                emphasis.is_valid()
                    && (self.legend_has(CoverageCellClass::ChangedCovered)
                        || self.legend_has(CoverageCellClass::ChangedUncovered))
            }
            None => true,
        }
    }

    /// Whether the legend declares the given cell class.
    pub fn legend_has(&self, cell_class: CoverageCellClass) -> bool {
        self.legend
            .iter()
            .any(|entry| entry.cell_class == cell_class)
    }

    /// Whether every field required to record this overlay is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.overlay_id.trim().is_empty()
            && self.scope.is_valid()
            && self.line_measure.is_valid()
            && self.branch_measure.map_or(true, CoverageMeasure::is_valid)
            && self.branch_markers_consistent()
            && self.imported_markers_consistent()
            && self.authority_consistent()
            && self.changed_line_markers_consistent()
            && !self.legend.is_empty()
            && self.legend.iter().all(CoverageLegendEntry::is_valid)
            && !self.run_ref.trim().is_empty()
            && !self.captured_at.trim().is_empty()
            && !self.support_summary.trim().is_empty()
            && self
                .origin_provider_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// One run contributing (or excluded) from a coverage merge sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedRunEntry {
    /// Run / session ref of the contributing run.
    pub run_ref: String,
    /// Provenance of the run's coverage evidence.
    pub provenance: CoverageEvidenceProvenance,
    /// Disposition of the run in the merge.
    pub disposition: CoverageRunDisposition,
    /// Reviewable note explaining the disposition.
    pub note: String,
}

impl MergedRunEntry {
    /// Whether the entry is well-formed: a duplicate / conflict exclusion must carry
    /// a disclosed note rather than being silently dropped.
    pub fn is_valid(&self) -> bool {
        !self.run_ref.trim().is_empty()
            && (!self.disposition.is_duplicate_or_conflict() || !self.note.trim().is_empty())
    }
}

/// One omitted shard / platform disclosed by a merge sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmittedScopeEntry {
    /// Whether the omission is a shard or a platform.
    pub omission_kind: OmittedScopeKind,
    /// Opaque ref of the omitted shard / platform.
    pub omitted_ref: String,
    /// Reviewable reason for the omission.
    pub reason: String,
}

impl OmittedScopeEntry {
    /// Whether the omission carries a ref and a reason.
    pub fn is_valid(&self) -> bool {
        !self.omitted_ref.trim().is_empty() && !self.reason.trim().is_empty()
    }
}

/// A coverage merge / import review sheet for one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageMergeReview {
    /// Stable merge id.
    pub merge_id: String,
    /// Durable scope the merge addresses.
    pub scope: CoverageScope,
    /// Metric mode the merged result reports.
    pub metric_mode: CoverageMetricMode,
    /// Merged coverage measure across included runs.
    pub merged_measure: CoverageMeasure,
    /// Contributing runs (included and excluded), in disclosure order.
    pub runs: Vec<MergedRunEntry>,
    /// Omitted shards / platforms disclosed by the sheet.
    #[serde(default)]
    pub omitted_scopes: Vec<OmittedScopeEntry>,
    /// Whether the sheet claims complete certainty. Must be false whenever any run is
    /// excluded or any shard / platform is omitted.
    pub implies_complete_certainty: bool,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe merge summary.
    pub support_summary: String,
}

impl CoverageMergeReview {
    /// Count of included runs.
    pub fn included_run_count(&self) -> usize {
        self.runs
            .iter()
            .filter(|r| r.disposition.is_included())
            .count()
    }

    /// Count of excluded runs.
    pub fn excluded_run_count(&self) -> usize {
        self.runs
            .iter()
            .filter(|r| !r.disposition.is_included())
            .count()
    }

    /// Whether the certainty claim is honest: a merge that omits or excludes anything
    /// may not claim complete certainty.
    pub fn certainty_consistent(&self) -> bool {
        if self.implies_complete_certainty {
            self.excluded_run_count() == 0 && self.omitted_scopes.is_empty()
        } else {
            true
        }
    }

    /// Whether every field required to record this merge is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.merge_id.trim().is_empty()
            && self.scope.is_valid()
            && self.merged_measure.is_valid()
            && !self.runs.is_empty()
            && self.included_run_count() >= 1
            && self.runs.iter().all(MergedRunEntry::is_valid)
            && self.omitted_scopes.iter().all(OmittedScopeEntry::is_valid)
            && self.certainty_consistent()
            && !self.captured_at.trim().is_empty()
            && !self.support_summary.trim().is_empty()
    }
}

/// Durable subject a snapshot / golden review card addresses, keyed by a node kind
/// and a non-display fingerprint distinct from its id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotSubject {
    /// Durable node id of the subject.
    pub subject_id: String,
    /// Node kind, reusing the frozen durable-discovery vocabulary so a parameterized
    /// template never collapses into a concrete invocation.
    pub node_kind: DurableTestNodeKind,
    /// Non-display fingerprint token. Must differ from
    /// [`subject_id`](SnapshotSubject::subject_id).
    pub subject_fingerprint_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
}

impl SnapshotSubject {
    /// Whether this subject is imported / provider-owned and read-only.
    pub fn is_imported(&self) -> bool {
        self.identity_class == TestItemIdentityClass::ImportedReadOnly
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.subject_fingerprint_token.trim();
        !token.is_empty() && token != self.subject_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.subject_id.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// A snapshot / golden review card preserving artifact kind, counts, baseline scope,
/// and raw fallback before any accept / reject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotReviewCard {
    /// Stable card id.
    pub card_id: String,
    /// Durable subject the card reviews.
    pub subject: SnapshotSubject,
    /// Artifact kind under review.
    pub artifact_kind: SnapshotArtifactKind,
    /// Count of changed artifacts in the card.
    pub changed_artifact_count: u32,
    /// Total artifacts in the card's baseline scope.
    pub total_artifact_count: u32,
    /// Baseline scope the artifacts belong to.
    pub baseline_scope: SnapshotBaselineScope,
    /// Raw / text fallback availability.
    pub raw_fallback: RawFallbackAvailability,
    /// Decision recorded for the change.
    pub decision: SnapshotReviewDecision,
    /// Whether the change was presented preview-first (never blind accept-all).
    pub preview_first: bool,
    /// Whether the card is imported / provider-backed (held read-only locally).
    pub imported: bool,
    /// Origin provider ref, present iff the card is imported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_provider_ref: Option<String>,
    /// Diff summary ref (reconstructable diff packet, never raw bytes).
    pub diff_summary_ref: String,
    /// Baseline ref the card compares against.
    pub baseline_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe card summary.
    pub support_summary: String,
}

impl SnapshotReviewCard {
    /// Whether the artifact counts are well-formed.
    pub fn counts_consistent(&self) -> bool {
        self.total_artifact_count >= 1 && self.changed_artifact_count <= self.total_artifact_count
    }

    /// Whether the raw-fallback gate is respected: a binary-only artifact with no
    /// fallback can never be accepted; it must route through raw inspection.
    pub fn raw_fallback_consistent(&self) -> bool {
        if self.decision.is_applied() {
            self.raw_fallback.supports_reviewed_accept()
        } else {
            true
        }
    }

    /// Whether the imported markers agree, and an imported card is never applied as a
    /// local accept.
    pub fn imported_markers_consistent(&self) -> bool {
        let markers = self.imported == self.origin_provider_ref.is_some()
            && self.imported == self.subject.is_imported();
        if self.imported {
            markers && !self.decision.is_applied()
        } else {
            markers
        }
    }

    /// Whether every field required to record this card is present and its invariants
    /// hold.
    pub fn is_valid(&self) -> bool {
        !self.card_id.trim().is_empty()
            && self.subject.is_valid()
            && self.counts_consistent()
            && self.preview_first
            && self.raw_fallback_consistent()
            && self.imported_markers_consistent()
            && !self.diff_summary_ref.trim().is_empty()
            && !self.baseline_ref.trim().is_empty()
            && !self.captured_at.trim().is_empty()
            && !self.support_summary.trim().is_empty()
            && self
                .origin_provider_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageReviewGuardrails {
    /// Verified-current, imported, cached, and stale provenance stay distinguished.
    pub provenance_distinguished: bool,
    /// Branch-versus-line truth is preserved; branch is only shown when measured.
    pub branch_versus_line_truthful: bool,
    /// Changed-line emphasis is preserved distinctly from whole-scope coverage.
    pub changed_line_emphasis_preserved: bool,
    /// Imported / provider-backed evidence never reads as a local verified result.
    pub imported_never_reads_as_local: bool,
    /// Merge omissions (excluded runs, omitted shards / platforms) are disclosed.
    pub merge_omissions_disclosed: bool,
    /// Snapshot / golden changes are preview-first with no blind accept-all.
    pub snapshot_preview_first: bool,
    /// No stale or imported coverage hides behind a generic green state.
    pub no_green_over_stale_coverage: bool,
}

impl CoverageReviewGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.provenance_distinguished
            && self.branch_versus_line_truthful
            && self.changed_line_emphasis_preserved
            && self.imported_never_reads_as_local
            && self.merge_omissions_disclosed
            && self.snapshot_preview_first
            && self.no_green_over_stale_coverage
    }
}

/// Consumer projection block: the surfaces that read this packet without re-deriving
/// coverage or snapshot truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageReviewConsumerProjection {
    /// The editor coverage gutter normalizes onto these overlays.
    pub coverage_gutter_normalized: bool,
    /// The coverage legend normalizes onto these overlays.
    pub coverage_legend_normalized: bool,
    /// The merge / import review sheet normalizes onto these merges.
    pub merge_sheet_normalized: bool,
    /// The snapshot / golden review UI normalizes onto these cards.
    pub snapshot_review_normalized: bool,
    /// Release and support exports read the same records.
    pub release_support_export_normalized: bool,
}

impl CoverageReviewConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.coverage_gutter_normalized
            && self.coverage_legend_normalized
            && self.merge_sheet_normalized
            && self.snapshot_review_normalized
            && self.release_support_export_normalized
    }
}

/// Constructor input for [`CoverageReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Coverage legend / overlay records.
    pub overlays: Vec<CoverageOverlayRecord>,
    /// Coverage merge / import review sheets.
    pub merges: Vec<CoverageMergeReview>,
    /// Snapshot / golden review cards.
    pub snapshot_cards: Vec<SnapshotReviewCard>,
    /// Guardrail invariants block.
    pub guardrails: CoverageReviewGuardrails,
    /// Consumer projection block.
    pub consumer_projection: CoverageReviewConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe coverage / snapshot-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageReviewPacket {
    /// Record kind; must equal [`COVERAGE_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COVERAGE_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Coverage legend / overlay records.
    pub overlays: Vec<CoverageOverlayRecord>,
    /// Coverage merge / import review sheets.
    pub merges: Vec<CoverageMergeReview>,
    /// Snapshot / golden review cards.
    pub snapshot_cards: Vec<SnapshotReviewCard>,
    /// Guardrail invariants block.
    pub guardrails: CoverageReviewGuardrails,
    /// Consumer projection block.
    pub consumer_projection: CoverageReviewConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CoverageReviewPacket {
    /// Builds a coverage / snapshot-review packet.
    pub fn new(input: CoverageReviewPacketInput) -> Self {
        Self {
            record_kind: COVERAGE_REVIEW_RECORD_KIND.to_owned(),
            schema_version: COVERAGE_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            overlays: input.overlays,
            merges: input.merges,
            snapshot_cards: input.snapshot_cards,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Coverage provenances represented across overlays.
    pub fn represented_provenances(&self) -> BTreeSet<CoverageEvidenceProvenance> {
        self.overlays.iter().map(|o| o.provenance).collect()
    }

    /// Metric modes represented across overlays.
    pub fn represented_metric_modes(&self) -> BTreeSet<CoverageMetricMode> {
        self.overlays.iter().map(|o| o.metric_mode).collect()
    }

    /// Snapshot artifact kinds represented across cards.
    pub fn represented_artifact_kinds(&self) -> BTreeSet<SnapshotArtifactKind> {
        self.snapshot_cards
            .iter()
            .map(|c| c.artifact_kind)
            .collect()
    }

    /// Snapshot subject node kinds represented across cards.
    pub fn represented_snapshot_subject_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.snapshot_cards
            .iter()
            .map(|c| c.subject.node_kind)
            .collect()
    }

    /// Resolves an overlay by its id.
    pub fn overlay(&self, overlay_id: &str) -> Option<&CoverageOverlayRecord> {
        self.overlays.iter().find(|o| o.overlay_id == overlay_id)
    }

    /// Resolves a merge by its id.
    pub fn merge(&self, merge_id: &str) -> Option<&CoverageMergeReview> {
        self.merges.iter().find(|m| m.merge_id == merge_id)
    }

    /// Resolves a snapshot card by its id.
    pub fn card(&self, card_id: &str) -> Option<&SnapshotReviewCard> {
        self.snapshot_cards.iter().find(|c| c.card_id == card_id)
    }

    /// Count of imported overlays.
    pub fn imported_overlay_count(&self) -> usize {
        self.overlays
            .iter()
            .filter(|o| o.provenance.is_imported())
            .count()
    }

    /// Count of stale overlays.
    pub fn stale_overlay_count(&self) -> usize {
        self.overlays
            .iter()
            .filter(|o| o.provenance.is_stale())
            .count()
    }

    /// Count of snapshot cards still awaiting a decision (pending or needs-raw).
    pub fn open_snapshot_card_count(&self) -> usize {
        self.snapshot_cards
            .iter()
            .filter(|c| {
                matches!(
                    c.decision,
                    SnapshotReviewDecision::PendingReview
                        | SnapshotReviewDecision::NeedsRawInspection
                )
            })
            .count()
    }

    /// Validates the coverage / snapshot-review invariants.
    pub fn validate(&self) -> Vec<CoverageReviewViolation> {
        let mut violations = Vec::new();

        if self.record_kind != COVERAGE_REVIEW_RECORD_KIND {
            violations.push(CoverageReviewViolation::WrongRecordKind);
        }
        if self.schema_version != COVERAGE_REVIEW_SCHEMA_VERSION {
            violations.push(CoverageReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CoverageReviewViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage_matrix(self, &mut violations);
        validate_overlays(self, &mut violations);
        validate_merges(self, &mut violations);
        validate_cards(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(CoverageReviewViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(CoverageReviewViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("coverage review packet serializes"),
        ) {
            violations.push(CoverageReviewViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("coverage review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Coverage Overlays And Snapshot / Golden Review\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Overlays: {} ({} imported, {} stale) across {} / {} provenance class(es)\n",
            self.overlays.len(),
            self.imported_overlay_count(),
            self.stale_overlay_count(),
            self.represented_provenances().len(),
            CoverageEvidenceProvenance::REQUIRED.len()
        ));
        out.push_str(&format!("- Merge sheets: {}\n", self.merges.len()));
        out.push_str(&format!(
            "- Snapshot cards: {} ({} open for review)\n",
            self.snapshot_cards.len(),
            self.open_snapshot_card_count()
        ));
        out.push_str("\n## Coverage overlays\n\n");
        for overlay in &self.overlays {
            out.push_str(&format!(
                "- **{}** [{}] {} `{}%` line",
                overlay.overlay_id,
                overlay.provenance.as_str(),
                overlay.metric_mode.as_str(),
                overlay.line_measure.percent()
            ));
            if let Some(branch) = &overlay.branch_measure {
                out.push_str(&format!(", `{}%` branch", branch.percent()));
            }
            out.push('\n');
            out.push_str(&format!(
                "  - scope `{}` ({}), authoritative {}\n",
                overlay.scope.scope_id,
                overlay.scope.scope_kind.as_str(),
                overlay.presents_as_authoritative
            ));
            if let Some(changed) = &overlay.changed_line_emphasis {
                out.push_str(&format!(
                    "  - changed since `{}`: {}/{} covered\n",
                    changed.changed_since_ref,
                    changed.changed_lines_covered,
                    changed.changed_lines_total
                ));
            }
        }
        out.push_str("\n## Coverage merge sheets\n\n");
        for merge in &self.merges {
            out.push_str(&format!(
                "- **{}** {} included / {} excluded, {} omitted (complete certainty: {})\n",
                merge.merge_id,
                merge.included_run_count(),
                merge.excluded_run_count(),
                merge.omitted_scopes.len(),
                merge.implies_complete_certainty
            ));
        }
        out.push_str("\n## Snapshot / golden review cards\n\n");
        for card in &self.snapshot_cards {
            out.push_str(&format!(
                "- **{}** [{}] {} / {} changed, scope `{}`\n",
                card.card_id,
                card.artifact_kind.as_str(),
                card.changed_artifact_count,
                card.total_artifact_count,
                card.baseline_scope.as_str()
            ));
            out.push_str(&format!(
                "  - fallback `{}` → decision `{}`{}\n",
                card.raw_fallback.as_str(),
                card.decision.as_str(),
                if card.imported { " (imported)" } else { "" }
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum CoverageReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CoverageReviewViolation>),
}

impl fmt::Display for CoverageReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "coverage review export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "coverage review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CoverageReviewArtifactError {}

/// Validation failures emitted by [`CoverageReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoverageReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// Coverage overlays do not distinguish all four required provenance classes.
    ProvenanceCoverageMissing,
    /// Both line and branch metric modes are not represented.
    MetricModeCoverageMissing,
    /// No overlay carries changed-line emphasis.
    ChangedLineCaseMissing,
    /// No merge sheet exercises an included-and-excluded run distinction.
    MergeDistinctionMissing,
    /// No merge sheet discloses an omitted shard / platform.
    OmittedScopeCaseMissing,
    /// No snapshot card exercises a binary-only raw-inspection gate.
    RawFallbackCaseMissing,
    /// No snapshot card holds an imported baseline read-only.
    ImportedSnapshotCaseMissing,
    /// Snapshot subjects collapse a parameterized template into its concrete
    /// invocation.
    TemplateCollapsedWithInvocation,
    /// An overlay is incomplete.
    OverlayInvalid,
    /// An overlay's scope fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// An imported or stale overlay presents as a verified-current green.
    GreenOverStaleOrImported,
    /// An overlay emphasizes branch coverage it does not measure.
    BranchWithoutMeasure,
    /// An imported overlay or card reads as a local verified result.
    ImportedReadsAsLocal,
    /// A merge is incomplete.
    MergeInvalid,
    /// A merge claims complete certainty while omitting or excluding runs.
    MergeImpliesFalseCertainty,
    /// A snapshot card is incomplete.
    SnapshotCardInvalid,
    /// A snapshot change was accepted without a raw / text fallback.
    SnapshotBlindAccept,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CoverageReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ProvenanceCoverageMissing => "provenance_coverage_missing",
            Self::MetricModeCoverageMissing => "metric_mode_coverage_missing",
            Self::ChangedLineCaseMissing => "changed_line_case_missing",
            Self::MergeDistinctionMissing => "merge_distinction_missing",
            Self::OmittedScopeCaseMissing => "omitted_scope_case_missing",
            Self::RawFallbackCaseMissing => "raw_fallback_case_missing",
            Self::ImportedSnapshotCaseMissing => "imported_snapshot_case_missing",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::OverlayInvalid => "overlay_invalid",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::GreenOverStaleOrImported => "green_over_stale_or_imported",
            Self::BranchWithoutMeasure => "branch_without_measure",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::MergeInvalid => "merge_invalid",
            Self::MergeImpliesFalseCertainty => "merge_implies_false_certainty",
            Self::SnapshotCardInvalid => "snapshot_card_invalid",
            Self::SnapshotBlindAccept => "snapshot_blind_accept",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_coverage_review_export() -> Result<CoverageReviewPacket, CoverageReviewArtifactError>
{
    let packet: CoverageReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/coverage-overlays-and-snapshot-golden-review/support_export.json"
    )))
    .map_err(CoverageReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CoverageReviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &CoverageReviewPacket,
    violations: &mut Vec<CoverageReviewViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        COVERAGE_REVIEW_SCHEMA_REF,
        COVERAGE_REVIEW_DOC_REF,
        COVERAGE_REVIEW_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CoverageReviewViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage_matrix(
    packet: &CoverageReviewPacket,
    violations: &mut Vec<CoverageReviewViolation>,
) {
    let provenances = packet.represented_provenances();
    if !CoverageEvidenceProvenance::REQUIRED
        .iter()
        .all(|p| provenances.contains(p))
    {
        violations.push(CoverageReviewViolation::ProvenanceCoverageMissing);
    }

    let modes = packet.represented_metric_modes();
    if !(modes.contains(&CoverageMetricMode::LineCoverage)
        && modes.contains(&CoverageMetricMode::BranchCoverage))
    {
        violations.push(CoverageReviewViolation::MetricModeCoverageMissing);
    }

    if !packet
        .overlays
        .iter()
        .any(|o| o.changed_line_emphasis.is_some())
    {
        violations.push(CoverageReviewViolation::ChangedLineCaseMissing);
    }

    if !packet
        .merges
        .iter()
        .any(|m| m.included_run_count() >= 1 && m.excluded_run_count() >= 1)
    {
        violations.push(CoverageReviewViolation::MergeDistinctionMissing);
    }

    if !packet.merges.iter().any(|m| !m.omitted_scopes.is_empty()) {
        violations.push(CoverageReviewViolation::OmittedScopeCaseMissing);
    }

    if !packet.snapshot_cards.iter().any(|c| {
        c.raw_fallback == RawFallbackAvailability::UnavailableBinaryOnly
            && c.decision == SnapshotReviewDecision::NeedsRawInspection
    }) {
        violations.push(CoverageReviewViolation::RawFallbackCaseMissing);
    }

    if !packet.snapshot_cards.iter().any(|c| c.imported) {
        violations.push(CoverageReviewViolation::ImportedSnapshotCaseMissing);
    }

    let subject_kinds = packet.represented_snapshot_subject_kinds();
    if !(subject_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && subject_kinds.contains(&DurableTestNodeKind::ConcreteInvocation))
    {
        violations.push(CoverageReviewViolation::TemplateCollapsedWithInvocation);
    }
}

fn validate_overlays(packet: &CoverageReviewPacket, violations: &mut Vec<CoverageReviewViolation>) {
    for overlay in &packet.overlays {
        if !overlay.is_valid() {
            violations.push(CoverageReviewViolation::OverlayInvalid);
        }
        if !overlay.scope.fingerprint_independent_of_id() {
            violations.push(CoverageReviewViolation::FingerprintSubstitutesIdentity);
        }
        if !overlay.authority_consistent() {
            violations.push(CoverageReviewViolation::GreenOverStaleOrImported);
        }
        if !overlay.branch_markers_consistent() {
            violations.push(CoverageReviewViolation::BranchWithoutMeasure);
        }
        if !overlay.imported_markers_consistent() {
            violations.push(CoverageReviewViolation::ImportedReadsAsLocal);
        }
    }
}

fn validate_merges(packet: &CoverageReviewPacket, violations: &mut Vec<CoverageReviewViolation>) {
    for merge in &packet.merges {
        if !merge.is_valid() {
            violations.push(CoverageReviewViolation::MergeInvalid);
        }
        if !merge.certainty_consistent() {
            violations.push(CoverageReviewViolation::MergeImpliesFalseCertainty);
        }
    }
}

fn validate_cards(packet: &CoverageReviewPacket, violations: &mut Vec<CoverageReviewViolation>) {
    for card in &packet.snapshot_cards {
        if !card.is_valid() {
            violations.push(CoverageReviewViolation::SnapshotCardInvalid);
        }
        if !card.raw_fallback_consistent() {
            violations.push(CoverageReviewViolation::SnapshotBlindAccept);
        }
        if !card.imported_markers_consistent() {
            violations.push(CoverageReviewViolation::ImportedReadsAsLocal);
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
