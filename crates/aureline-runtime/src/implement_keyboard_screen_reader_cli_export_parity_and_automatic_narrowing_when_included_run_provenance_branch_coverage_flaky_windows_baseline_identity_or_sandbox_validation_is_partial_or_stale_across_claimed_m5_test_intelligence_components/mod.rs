//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 test-intelligence components.
//!
//! This module is the M05-1034 accessibility-and-auto-narrowing capstone over the frozen
//! M5 coverage-summary-bar / coverage-overlay-marker / flaky-state-badge / retry-history-row /
//! snapshot-review-card / coverage-import-merge-sheet / test-generation-suggestion-card component
//! matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`]).
//! Where the freeze matrix defines the reusable coverage bar, coverage overlay marker,
//! flaky-state badge, retry-history row, snapshot-review card, coverage-import / merge sheet, and
//! test-generation suggestion card primitives, and the 1029-1033 implementation / consumer lanes
//! resolve their per-surface truth, this lane certifies — per component family — that
//! test-intelligence claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe,
//! and self-narrowing** rather than presenting imported or stale included-run provenance, partial
//! branch / condition coverage, an insufficient flaky evidence window, an unverified snapshot
//! baseline, or an unproven sandbox validation as a still verified-current signal:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same evidence identity,
//!   provenance / freshness class, included-run scope, line-versus-branch metric, classifier
//!   confidence, artifact baseline identity, raw / text fallback, and generated-test assumption
//!   boundary the rich component shows — never a pointer-only or hover-only chip that strands
//!   assistive-tech or headless users. Hierarchy-heavy families (the coverage-import / merge
//!   sheet's nested per-shard legs and the snapshot-review card's nested per-artifact diffs)
//!   additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning
//!   from typed tokens and opaque refs without a screenshot, preserving the same stable evidence
//!   IDs, provenance class, included-run scope, baseline identity, and assumption-boundary
//!   vocabulary shown in-product so coverage / flaky / snapshot / generated-test truth can be
//!   reconstructed without screenshots or private team memory.
//! - **Honest auto-narrowing.** When included-run provenance is imported or stale, branch /
//!   condition coverage is partial, a flaky evidence window is insufficient, a snapshot baseline
//!   is unverified, or a sandbox validation is unproven, the component's evidence claim
//!   auto-narrows from `VerifiedCurrentEvidence` / `ReviewableEvidence` to an imported-or-stale /
//!   partial-condition / unconfirmed-flaky / unverified-baseline / unvalidated-generated evidence
//!   claim, discloses the narrowing with a precise trigger and binding dimension, and preserves
//!   the canonical identity / provenance / baseline / assumption lineage — the underlying evidence
//!   lineage is never dropped opaquely. A component with every dimension intact must NOT carry a
//!   spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the coverage-report UI,
//!   editor-overlay, flaky-dashboard, retry-history, snapshot-review, coverage-import, and
//!   test-generation surfaces, the headless CLI, and support / release exports so product, docs,
//!   and release publication stay aligned on test-intelligence downgrade behavior rather than
//!   drifting in copy — a green percentage, a confident flaky verdict, or a generated test can
//!   never outrun the provenance / scope / baseline / assumption proof it is being viewed away
//!   from.
//!
//! Each [`IntelComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::M5TestIntelligenceComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5TestIntelligenceRequiredLabel`] and
//! [`M5TestIntelligenceDowngradeTrigger`] and the shared [`M5TestIntelligenceConsumerSurface`]
//! consumer surfaces rather than minting parallel synonyms, so the certified labels stay
//! byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw logs, assertion bodies, coverage payloads, snapshot bytes, and
//! credential-bearing material never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels so support, release, and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have shown
//! without leaking test material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    M5TestIntelligenceComponentFamily, M5TestIntelligenceConsumerSurface,
    M5TestIntelligenceDowngradeTrigger, M5TestIntelligenceRequiredLabel,
};

/// Schema version stamped on the M05-1034 test-intelligence component accessibility fallback
/// packet.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`IntelComponentAccessibilityPacket`].
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_test_intelligence_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`IntelComponentAccessibilityRow`].
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_test_intelligence_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-test-intelligence-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/testing/m5_test_intelligence_component_accessibility_fallback.md";

/// Repo-relative path of the frozen test-intelligence component matrix this lane certifies.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-intelligence-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-test-intelligence-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-test-intelligence-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEST_INTEL_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the coverage-import /
/// merge sheet's nested per-shard legs and the snapshot-review card's nested per-artifact diffs)
/// and therefore MUST bind their tree to an equivalent flat list / textual path so the hierarchy
/// is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5TestIntelligenceComponentFamily) -> bool {
    matches!(
        family,
        M5TestIntelligenceComponentFamily::CoverageImportMergeSheet
            | M5TestIntelligenceComponentFamily::SnapshotReviewCard
    )
}

/// The test-intelligence dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5TestIntelligenceComponentFamily,
) -> M5IntelComponentClaimDimension {
    match family {
        M5TestIntelligenceComponentFamily::CoverageSummaryBar => {
            M5IntelComponentClaimDimension::IncludedRunProvenance
        }
        M5TestIntelligenceComponentFamily::CoverageOverlayMarker => {
            M5IntelComponentClaimDimension::BranchConditionCoverage
        }
        M5TestIntelligenceComponentFamily::FlakyStateBadge => {
            M5IntelComponentClaimDimension::FlakyEvidenceWindow
        }
        M5TestIntelligenceComponentFamily::RetryHistoryRow => {
            M5IntelComponentClaimDimension::FlakyEvidenceWindow
        }
        M5TestIntelligenceComponentFamily::SnapshotReviewCard => {
            M5IntelComponentClaimDimension::BaselineScopeIdentity
        }
        M5TestIntelligenceComponentFamily::CoverageImportMergeSheet => {
            M5IntelComponentClaimDimension::BaselineScopeIdentity
        }
        M5TestIntelligenceComponentFamily::TestGenerationSuggestionCard => {
            M5IntelComponentClaimDimension::SandboxValidation
        }
    }
}

/// A rendered fallback modality for a test-intelligence component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IntelComponentFallbackModality {
    /// A rich, structured (nested per-shard / per-artifact tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5IntelComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / headless path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IntelComponentRenderingSurface {
    /// The full-capability desktop test surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5IntelComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntelComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl IntelComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntelComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl IntelComponentExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntelComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl IntelComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The evidence claim ceiling a component asserts: how strong a test-intelligence posture it lets
/// a surface present. Auto-narrowing lowers this ceiling when a test dimension weakens so imported
/// or stale provenance, partial branch / condition coverage, an insufficient flaky window, an
/// unverified baseline, or an unproven sandbox validation can never keep an old
/// `VerifiedCurrentEvidence` or `ReviewableEvidence` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IntelComponentClaim {
    /// Verified current evidence: a verified current-run, exact-scope, fresh signal — the
    /// strongest claim, a coverage number or verdict a user can act on as-is.
    VerifiedCurrentEvidence,
    /// Reviewable evidence: a self-consistent, review-first signal (evidence a reviewer reads and
    /// accepts) that is not itself a certified verified-current signal — the honest baseline for a
    /// snapshot-review card or a generated-test suggestion.
    ReviewableEvidence,
    /// Partial-condition evidence: usable, but branch / condition coverage is partial — the number
    /// cannot be read as full line-and-branch coverage.
    PartialConditionEvidence,
    /// Unconfirmed-flaky evidence: the signal stands, but the flaky evidence window is
    /// insufficient; the verdict cannot be read as confirmed flakiness.
    UnconfirmedFlakyEvidence,
    /// Unverified-baseline evidence: the snapshot / merge baseline identity or shard scope is
    /// unverified; the card cannot claim a trusted baseline.
    UnverifiedBaselineEvidence,
    /// Imported-or-stale evidence: the included-run provenance is imported or stale; the mark is
    /// attributable but cannot be read as a fresh current-run result.
    ImportedOrStaleEvidence,
    /// Unvalidated-generated evidence: the generated test's sandbox validation is unproven; the
    /// suggestion stays review-first and cannot claim a validated generated test.
    UnvalidatedGeneratedEvidence,
}

impl M5IntelComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 7] = [
        Self::VerifiedCurrentEvidence,
        Self::ReviewableEvidence,
        Self::PartialConditionEvidence,
        Self::UnconfirmedFlakyEvidence,
        Self::UnverifiedBaselineEvidence,
        Self::ImportedOrStaleEvidence,
        Self::UnvalidatedGeneratedEvidence,
    ];

    /// Capability rank; a higher rank asserts a stronger evidence posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::VerifiedCurrentEvidence => 6,
            Self::ReviewableEvidence => 5,
            Self::PartialConditionEvidence => 4,
            Self::UnconfirmedFlakyEvidence => 3,
            Self::UnverifiedBaselineEvidence => 2,
            Self::ImportedOrStaleEvidence => 1,
            Self::UnvalidatedGeneratedEvidence => 0,
        }
    }

    /// Returns true when this claim asserts a fully verified-current signal.
    pub const fn asserts_verified_current(self) -> bool {
        matches!(self, Self::VerifiedCurrentEvidence)
    }

    /// Returns true when this claim asserts a fully self-sufficient (verified-current or
    /// reviewable) signal.
    pub const fn asserts_full_evidence(self) -> bool {
        matches!(
            self,
            Self::VerifiedCurrentEvidence | Self::ReviewableEvidence
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrentEvidence => "verified_current_evidence",
            Self::ReviewableEvidence => "reviewable_evidence",
            Self::PartialConditionEvidence => "partial_condition_evidence",
            Self::UnconfirmedFlakyEvidence => "unconfirmed_flaky_evidence",
            Self::UnverifiedBaselineEvidence => "unverified_baseline_evidence",
            Self::ImportedOrStaleEvidence => "imported_or_stale_evidence",
            Self::UnvalidatedGeneratedEvidence => "unvalidated_generated_evidence",
        }
    }
}

/// The test-intelligence dimension whose state governs how far a component may claim to be a
/// verified-current signal. The five spec axes the lane must auto-narrow on — imported / stale
/// included-run provenance, partial branch / condition coverage, an insufficient flaky evidence
/// window, an unverified baseline identity / scope, and an unproven sandbox validation — are
/// [`Self::IncludedRunProvenance`], [`Self::BranchConditionCoverage`],
/// [`Self::FlakyEvidenceWindow`], [`Self::BaselineScopeIdentity`], and [`Self::SandboxValidation`];
/// every frozen family maps onto one of these axes so each carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IntelComponentClaimDimension {
    /// Included-run provenance: is the coverage / result verified current-run, or imported /
    /// stale?
    IncludedRunProvenance,
    /// Branch / condition coverage: is the overlay full line-and-branch, or is branch / condition
    /// coverage partial?
    BranchConditionCoverage,
    /// Flaky evidence window: has flakiness been reproduced across a sufficient window, or is the
    /// window insufficient?
    FlakyEvidenceWindow,
    /// Baseline / scope identity: is the snapshot / merge baseline identity and shard scope
    /// verified, or unverified?
    BaselineScopeIdentity,
    /// Sandbox validation: has the generated test been validated in a sandbox, or is validation
    /// unproven?
    SandboxValidation,
}

impl M5IntelComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::IncludedRunProvenance,
        Self::BranchConditionCoverage,
        Self::FlakyEvidenceWindow,
        Self::BaselineScopeIdentity,
        Self::SandboxValidation,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a narrowing.
    /// Each dimension maps to the on-topic frozen trigger the freeze matrix already governs, so
    /// the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5TestIntelligenceDowngradeTrigger {
        match self {
            Self::IncludedRunProvenance => {
                M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated
            }
            Self::BranchConditionCoverage => {
                M5TestIntelligenceDowngradeTrigger::LineVersusBranchUnstated
            }
            Self::FlakyEvidenceWindow => {
                M5TestIntelligenceDowngradeTrigger::FlakyConfidenceOverstated
            }
            Self::BaselineScopeIdentity => {
                M5TestIntelligenceDowngradeTrigger::SnapshotBaselineUnstated
            }
            Self::SandboxValidation => {
                M5TestIntelligenceDowngradeTrigger::GeneratedAssumptionHidden
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedRunProvenance => "included_run_provenance",
            Self::BranchConditionCoverage => "branch_condition_coverage",
            Self::FlakyEvidenceWindow => "flaky_evidence_window",
            Self::BaselineScopeIdentity => "baseline_scope_identity",
            Self::SandboxValidation => "sandbox_validation",
        }
    }
}

/// The observed condition of one test dimension. Anything weaker than
/// [`Self::EvidenceCurrentExact`] imposes a narrowing ceiling on the component's evidence claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IntelComponentConditionState {
    /// Verified current-run, full-coverage, exact-scope, verified baseline, validated sandbox —
    /// imposes no ceiling.
    EvidenceCurrentExact,
    /// Included-run provenance is imported or stale — the mark drops to imported-or-stale.
    ProvenanceImportedOrStale,
    /// Branch / condition coverage is partial — the number drops to partial-condition.
    BranchConditionPartial,
    /// The flaky evidence window is insufficient — the verdict drops to unconfirmed-flaky.
    FlakyWindowInsufficient,
    /// The snapshot / merge baseline identity or shard scope is unverified — the card drops to
    /// unverified-baseline.
    BaselineIdentityUnverified,
    /// The generated test's sandbox validation is unproven — the suggestion drops to
    /// unvalidated-generated.
    SandboxValidationUnproven,
}

impl M5IntelComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceCurrentExact,
        Self::ProvenanceImportedOrStale,
        Self::BranchConditionPartial,
        Self::FlakyWindowInsufficient,
        Self::BaselineIdentityUnverified,
        Self::SandboxValidationUnproven,
    ];

    /// Returns true when the dimension is weaker than current-exact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::EvidenceCurrentExact)
    }

    /// The strongest evidence claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5IntelComponentClaim {
        match self {
            Self::EvidenceCurrentExact => M5IntelComponentClaim::VerifiedCurrentEvidence,
            Self::ProvenanceImportedOrStale => M5IntelComponentClaim::ImportedOrStaleEvidence,
            Self::BranchConditionPartial => M5IntelComponentClaim::PartialConditionEvidence,
            Self::FlakyWindowInsufficient => M5IntelComponentClaim::UnconfirmedFlakyEvidence,
            Self::BaselineIdentityUnverified => M5IntelComponentClaim::UnverifiedBaselineEvidence,
            Self::SandboxValidationUnproven => M5IntelComponentClaim::UnvalidatedGeneratedEvidence,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceCurrentExact => "evidence_current_exact",
            Self::ProvenanceImportedOrStale => "provenance_imported_or_stale",
            Self::BranchConditionPartial => "branch_condition_partial",
            Self::FlakyWindowInsufficient => "flaky_window_insufficient",
            Self::BaselineIdentityUnverified => "baseline_identity_unverified",
            Self::SandboxValidationUnproven => "sandbox_validation_unproven",
        }
    }
}

/// One test dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5IntelComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5IntelComponentConditionState,
}

/// An honest evidence-claim auto-narrow block. When a test dimension weakens, the component's
/// evidence claim lowers to the permitted ceiling, names the binding dimension and frozen trigger,
/// and preserves the canonical identity / provenance / baseline / assumption lineage rather than
/// silently dropping it — the underlying evidence lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentClaimAutoNarrow {
    /// The evidence claim the component is narrowed to.
    pub narrowed_to: M5IntelComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5IntelComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5TestIntelligenceDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical evidence identity, provenance class, baseline identity, and assumption
    /// boundary are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying provenance / baseline / assumption lineage is preserved (never dropped)
    /// across the narrowing; must hold so imported, stale, partial, unconfirmed, unverified, and
    /// unvalidated states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl IntelComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and evidence
    /// lineage and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_lineage_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable
/// as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl IntelComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5IntelComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: IntelComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a test-intelligence component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntelComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims a current signal, or drops state
    /// silently (red).
    Stranded,
}

impl IntelComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one test-intelligence component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentAccessibilityRow {
    /// Record kind; must equal [`TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5TestIntelligenceComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the coverage / flaky / snapshot / generated-test evidence object this
    /// component acts on; stays visible on every surface, so this is never empty.
    pub test_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5IntelComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, provenance, scope, baseline,
    /// and assumption truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: IntelComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: IntelComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: IntelComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: IntelComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: IntelComponentCopyExportParity,
    /// The full evidence claim this family asserts when every dimension is intact.
    pub full_test_claim: M5IntelComponentClaim,
    /// The observed condition of each modeled test dimension.
    #[serde(default)]
    pub claim_conditions: Vec<IntelComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<IntelComponentClaimAutoNarrow>,
    /// Whether the underlying provenance / baseline / assumption lineage is preserved on this
    /// component regardless of narrowing; must hold so imported, stale, partial, unconfirmed,
    /// unverified, and unvalidated states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5IntelComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<IntelComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5TestIntelligenceRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5TestIntelligenceConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl IntelComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `EvidenceCurrentExact` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5IntelComponentClaimDimension,
    ) -> M5IntelComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5IntelComponentConditionState::EvidenceCurrentExact)
    }

    /// Whether any modeled dimension is weaker than current-exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest evidence claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5IntelComponentClaim {
        let mut permitted = self.full_test_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5IntelComponentClaimDimension> {
        let mut binding: Option<(M5IntelComponentClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_test_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The evidence claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5IntelComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_test_claim,
        }
    }

    /// AC / auto-narrowing honesty: imported or stale provenance, partial branch / condition
    /// coverage, an insufficient flaky window, an unverified baseline, or an unproven sandbox
    /// validation can no longer keep an old `VerifiedCurrentEvidence` / `ReviewableEvidence`
    /// label. The effective claim never exceeds the permitted ceiling; when a dimension narrows
    /// below the full claim, an honest narrow block is present, narrows to exactly the permitted
    /// ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and evidence lineage. When nothing narrows, no spurious narrow block is
    /// present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.test_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: imported, stale, partial, unconfirmed, unverified, and unvalidated states
    /// preserve the underlying provenance / baseline / assumption lineage. The row must assert
    /// `lineage_preserved`, and any narrow block must preserve lineage continuity too.
    pub fn preserves_lineage_continuity(&self) -> bool {
        self.lineage_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_lineage_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned
    /// on the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5TestIntelligenceRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> IntelComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return IntelComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            IntelComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            IntelComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.test_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_test_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1034 test-intelligence component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_lineage_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`IntelComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntelComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<IntelComponentAccessibilityRow>,
}

/// Checked-in M05-1034 test-intelligence component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntelComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<IntelComponentAccessibilityRow>,
    pub summary: IntelComponentAccessibilitySummary,
}

impl IntelComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: IntelComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: IntelComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
                all_lineage_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5TestIntelligenceComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5IntelComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5IntelComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Evidence claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5IntelComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5TestIntelligenceConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> IntelComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5TestIntelligenceConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&IntelComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                IntelComponentAccessibilityStatus::Parity => green += 1,
                IntelComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                IntelComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        IntelComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(IntelComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(IntelComponentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(IntelComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(IntelComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(IntelComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<IntelComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(IntelComponentAccessibilityViolation::SchemaVersion {
                expected: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEST_INTEL_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(IntelComponentAccessibilityViolation::RecordKind {
                expected: TEST_INTEL_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(IntelComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(IntelComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(IntelComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    IntelComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory test label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    IntelComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5IntelComponentFallbackModality::Structured)
            {
                violations.push(
                    IntelComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a verified-current / reviewable signal for a weakened
            // one.
            if !row.claim_is_honest() {
                violations.push(IntelComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    IntelComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    IntelComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: imported, stale, partial, unconfirmed, unverified, and unvalidated states
            // preserve evidence lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(IntelComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    IntelComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    IntelComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == IntelComponentAccessibilityStatus::Stranded {
                violations.push(IntelComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5TestIntelligenceComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(IntelComponentAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5IntelComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    IntelComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the current-exact baseline plus each spec narrowing
        // axis) is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5IntelComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    IntelComponentAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every evidence claim tier appears as an effective claim, so the full narrowing
        // spectrum (verified-current → … → unvalidated-generated) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5IntelComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(IntelComponentAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach every consumer surface at least once
        // across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5TestIntelligenceConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    IntelComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(IntelComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("test-intelligence accessibility fallback packet serializes"),
        ) {
            violations.push(IntelComponentAccessibilityViolation::RawTestMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("test-intelligence accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_test_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Test-Intelligence Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5TestIntelligenceComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_test_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in test-intelligence component accessibility fallback export.
pub fn current_m5_test_intel_component_a11y_fallback_export(
) -> Result<IntelComponentAccessibilityPacket, IntelComponentAccessibilityArtifactError> {
    let packet: IntelComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-intelligence-component-accessibility-fallback/support_export.json"
    )))
    .map_err(IntelComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(IntelComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in test-intelligence component accessibility fallback
/// export.
#[derive(Debug)]
pub enum IntelComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<IntelComponentAccessibilityViolation>),
}

impl fmt::Display for IntelComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "test-intelligence accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "test-intelligence accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for IntelComponentAccessibilityArtifactError {}

/// Validation failure for M05-1034 test-intelligence component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntelComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5IntelComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    LineageDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5TestIntelligenceComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5IntelComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5IntelComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5IntelComponentClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5TestIntelligenceConsumerSurface,
    },
    SummaryMismatch,
    RawTestMaterialInExport,
}

impl fmt::Display for IntelComponentAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory test label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a verified-current / reviewable signal for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve provenance / baseline / assumption lineage across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "evidence claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawTestMaterialInExport => {
                write!(f, "export contains raw test material")
            }
        }
    }
}

impl Error for IntelComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "imported"
            | "stale"
            | "imported or stale"
            | "partial"
            | "unconfirmed"
            | "unverified"
            | "unvalidated"
            | "flaky"
            | "generated"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in test-intelligence component accessibility fallback packet.
/// This is the one source of truth shared by the tests and the on-disk support export so both
/// stay byte-aligned.
pub fn seeded_m5_test_intel_component_a11y_fallback_packet() -> IntelComponentAccessibilityPacket {
    IntelComponentAccessibilityPacket::new(IntelComponentAccessibilityPacketInput {
        packet_id: "m5-test-intelligence-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:test-intel-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5TestIntelligenceRequiredLabel> {
    M5TestIntelligenceRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> IntelComponentCopyExportParity {
    IntelComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5IntelComponentClaimDimension,
    state: M5IntelComponentConditionState,
) -> IntelComponentClaimConditionEntry {
    IntelComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5TestIntelligenceConsumerSurface],
) -> Vec<M5TestIntelligenceConsumerSurface> {
    let mut out = vec![
        M5TestIntelligenceConsumerSurface::SupportExport,
        M5TestIntelligenceConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: IntelComponentNarrowingDisclosureState,
) -> Vec<IntelComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        IntelComponentRenderingNarrowingDisclosure {
            rendering_surface: M5IntelComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        IntelComponentRenderingNarrowingDisclosure {
            rendering_surface: M5IntelComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_rerun".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<IntelComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        IntelComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<IntelComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        IntelComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5IntelComponentRenderingSurface> {
    vec![
        M5IntelComponentRenderingSurface::DesktopFull,
        M5IntelComponentRenderingSurface::CliHeadless,
        M5IntelComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<IntelComponentAccessibilityRow> {
    vec![
        // Coverage-summary bar — the included-run provenance is imported CI / stale rather than a
        // fresh current run, so the bar auto-narrows to an imported-or-stale result rather than
        // implying an exact current-run percentage, while keeping its included-run scope and
        // line/branch metric visible (yellow).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:coverage-summary-bar-imported".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::CoverageSummaryBar,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:coverage-summary-bar:0001".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:coverage-summary-bar-imported:a11y".to_owned(),
            copy_export: copy_export(&[
                "coverage_percentage",
                "included_run_scope",
                "line_versus_branch_metric",
                "provenance_class",
            ]),
            full_test_claim: M5IntelComponentClaim::VerifiedCurrentEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::IncludedRunProvenance,
                M5IntelComponentConditionState::ProvenanceImportedOrStale,
            )],
            claim_narrow: Some(IntelComponentClaimAutoNarrow {
                narrowed_to: M5IntelComponentClaim::ImportedOrStaleEvidence,
                binding_dimension: M5IntelComponentClaimDimension::IncludedRunProvenance,
                trigger: M5TestIntelligenceDowngradeTrigger::ProvenanceClassUnstated,
                narrowed_label:
                    "Coverage number is built from imported CI runs, not a fresh current run — shown as an imported-or-stale result that names its included-run scope and provenance, never as an exact current-run percentage"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "coverage_percentage",
                "included_run_scope",
                "provenance_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::CoverageReportUi,
                M5TestIntelligenceConsumerSurface::EditorOverlayUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.16 coverage visualization".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("coverage-summary-bar-imported"),
        },
        // Coverage-overlay marker — branch / condition coverage is only partially known, so the
        // overlay auto-narrows to a partial-condition result rather than implying full
        // line-and-branch coverage, while keeping its changed-line emphasis and per-line state
        // visible (yellow).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:coverage-overlay-marker-partial".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::CoverageOverlayMarker,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:coverage-overlay-marker:0002".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:coverage-overlay-marker-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "overlay_state",
                "line_versus_branch_metric",
                "changed_line_emphasis",
                "provenance_class",
            ]),
            full_test_claim: M5IntelComponentClaim::VerifiedCurrentEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::BranchConditionCoverage,
                M5IntelComponentConditionState::BranchConditionPartial,
            )],
            claim_narrow: Some(IntelComponentClaimAutoNarrow {
                narrowed_to: M5IntelComponentClaim::PartialConditionEvidence,
                binding_dimension: M5IntelComponentClaimDimension::BranchConditionCoverage,
                trigger: M5TestIntelligenceDowngradeTrigger::LineVersusBranchUnstated,
                narrowed_label:
                    "Only line coverage is measured on this overlay; branch / condition coverage is partial — shown as a partial-condition result that keeps its changed-line emphasis, never as full line-and-branch coverage"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "overlay_state",
                "line_versus_branch_metric",
                "changed_line_emphasis",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::EditorOverlayUi,
                M5TestIntelligenceConsumerSurface::CoverageReportUi,
            ]),
            source_refs: vec![
                "UX Design System §16.47 coverage overlays".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("coverage-overlay-marker-partial"),
        },
        // Flaky-state badge — the flaky evidence window is insufficient (a single or unconfirmed
        // occurrence), so the badge auto-narrows to an unconfirmed-flaky result rather than a
        // confirmed-flaky verdict, while keeping its classifier confidence and evidence window
        // visible (yellow).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:flaky-state-badge-unconfirmed".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::FlakyStateBadge,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:flaky-state-badge:0003".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:flaky-state-badge-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "flaky_classification",
                "classifier_confidence",
                "evidence_window",
                "provenance_class",
            ]),
            full_test_claim: M5IntelComponentClaim::VerifiedCurrentEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::FlakyEvidenceWindow,
                M5IntelComponentConditionState::FlakyWindowInsufficient,
            )],
            claim_narrow: Some(IntelComponentClaimAutoNarrow {
                narrowed_to: M5IntelComponentClaim::UnconfirmedFlakyEvidence,
                binding_dimension: M5IntelComponentClaimDimension::FlakyEvidenceWindow,
                trigger: M5TestIntelligenceDowngradeTrigger::FlakyConfidenceOverstated,
                narrowed_label:
                    "Flakiness is seen once and not yet reproduced across the evidence window — shown as an unconfirmed-flaky result that names its classifier confidence and window size, never as confirmed flakiness"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "flaky_classification",
                "classifier_confidence",
                "evidence_window",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::FlakyDashboardUi,
                M5TestIntelligenceConsumerSurface::EditorOverlayUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.16 flaky classification".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("flaky-state-badge-unconfirmed"),
        },
        // Retry-history row — every attempt is a verified current-run outcome with its rerun scope
        // and env / build / runtime deltas intact, so the row is a verified-current signal
        // reachable on every surface (green).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:retry-history-row-current".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::RetryHistoryRow,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:retry-history-row:0004".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:retry-history-row-current:a11y".to_owned(),
            copy_export: copy_export(&[
                "attempt_outcome",
                "rerun_scope",
                "env_build_runtime_delta",
                "provenance_class",
            ]),
            full_test_claim: M5IntelComponentClaim::VerifiedCurrentEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::FlakyEvidenceWindow,
                M5IntelComponentConditionState::EvidenceCurrentExact,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "attempt_outcome",
                "rerun_scope",
                "env_build_runtime_delta",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::RetryHistoryUi,
                M5TestIntelligenceConsumerSurface::FlakyDashboardUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.16 retry history".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("retry-history-row-current"),
        },
        // Snapshot / golden review card — hierarchy-heavy (nested per-artifact diffs); the card is
        // a self-consistent, review-first acceptance surface built on a verified current baseline
        // (not itself a certified verified-current signal), reachable on every surface (green).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:snapshot-review-card-reviewable".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::SnapshotReviewCard,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:snapshot-review-card:0005".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::Structured,
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:snapshot-review-card-reviewable:a11y".to_owned(),
            copy_export: copy_export(&[
                "baseline_identity",
                "diff_state",
                "artifact_count",
                "raw_or_text_fallback",
            ]),
            full_test_claim: M5IntelComponentClaim::ReviewableEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::BaselineScopeIdentity,
                M5IntelComponentConditionState::EvidenceCurrentExact,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "baseline_identity",
                "diff_state",
                "artifact_count",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::SnapshotReviewUi,
                M5TestIntelligenceConsumerSurface::TestGenerationUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.16 snapshot/golden review UX".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("snapshot-review-card-reviewable"),
        },
        // Coverage-import / merge sheet — hierarchy-heavy (nested per-shard legs); the merged
        // baseline identity / shard scope is unverified (a shard is omitted), so the sheet
        // auto-narrows to an unverified-baseline result rather than presenting the merged number as
        // exact current truth, while keeping the omission and import source visible (yellow).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:coverage-import-merge-sheet-unverified".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::CoverageImportMergeSheet,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:coverage-import-merge-sheet:0006".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::Structured,
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:coverage-import-merge-sheet-unverified:a11y".to_owned(),
            copy_export: copy_export(&[
                "import_source",
                "merge_resolution",
                "shard_scope",
                "baseline_identity",
            ]),
            full_test_claim: M5IntelComponentClaim::VerifiedCurrentEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::BaselineScopeIdentity,
                M5IntelComponentConditionState::BaselineIdentityUnverified,
            )],
            claim_narrow: Some(IntelComponentClaimAutoNarrow {
                narrowed_to: M5IntelComponentClaim::UnverifiedBaselineEvidence,
                binding_dimension: M5IntelComponentClaimDimension::BaselineScopeIdentity,
                trigger: M5TestIntelligenceDowngradeTrigger::SnapshotBaselineUnstated,
                narrowed_label:
                    "A shard is omitted from the merge, so the baseline identity and scope are unverified — shown as an unverified-baseline result that names the omission and import source, never as an exact merged current number"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "import_source",
                "merge_resolution",
                "shard_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::CoverageImportUi,
                M5TestIntelligenceConsumerSurface::CoverageReportUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §17.16 coverage import / merge".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("coverage-import-merge-sheet-unverified"),
        },
        // Test-generation suggestion card — the generated test's sandbox validation is unproven,
        // so the card auto-narrows to an unvalidated-generated result that stays review-first,
        // separating assertion / helper-fixture / snapshot churn and keeping its assumption
        // summary visible rather than bundling a validated apply (yellow).
        IntelComponentAccessibilityRow {
            record_kind: TEST_INTEL_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_INTEL_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:test-generation-suggestion-card-unvalidated".to_owned(),
            component_family: M5TestIntelligenceComponentFamily::TestGenerationSuggestionCard,
            source_family_schema_ref: TEST_INTEL_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            test_context_ref: "test:test-generation-suggestion-card:0007".to_owned(),
            fallback_modalities: vec![
                M5IntelComponentFallbackModality::List,
                M5IntelComponentFallbackModality::Textual,
                M5IntelComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: IntelComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: IntelComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:test-generation-suggestion-card-unvalidated:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "assumption_summary",
                "apply_scope",
                "sandbox_validation_state",
                "diff_first_preview",
            ]),
            full_test_claim: M5IntelComponentClaim::ReviewableEvidence,
            claim_conditions: vec![condition(
                M5IntelComponentClaimDimension::SandboxValidation,
                M5IntelComponentConditionState::SandboxValidationUnproven,
            )],
            claim_narrow: Some(IntelComponentClaimAutoNarrow {
                narrowed_to: M5IntelComponentClaim::UnvalidatedGeneratedEvidence,
                binding_dimension: M5IntelComponentClaimDimension::SandboxValidation,
                trigger: M5TestIntelligenceDowngradeTrigger::GeneratedAssumptionHidden,
                narrowed_label:
                    "The generated test has not been validated in a sandbox, so it stays review-first — shown as an unvalidated-generated result that keeps its assumption summary and diff-first preview, never as a validated one-click apply"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "assumption_summary",
                "apply_scope",
                "sandbox_validation_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestIntelligenceConsumerSurface::TestGenerationUi,
                M5TestIntelligenceConsumerSurface::SnapshotReviewUi,
            ]),
            source_refs: vec![
                "UX Design System §16.47 test-generation suggestion cards".to_owned(),
                TEST_INTEL_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-09T00:00:00Z".to_owned(),
            evidence_refs: ev("test-generation-suggestion-card-unvalidated"),
        },
    ]
}
