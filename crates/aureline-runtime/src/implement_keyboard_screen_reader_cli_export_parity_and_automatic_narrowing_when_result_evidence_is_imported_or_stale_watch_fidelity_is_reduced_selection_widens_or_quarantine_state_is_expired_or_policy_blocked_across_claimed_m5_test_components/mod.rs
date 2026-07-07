//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 test-explorer / watch / triage components.
//!
//! This module is the M05-914 accessibility-and-auto-narrowing capstone over the frozen
//! M5 test-tree-row / inline-result-marker / session-summary-bar / watch-mode-banner /
//! failure-triage-panel / quarantine-review-sheet / environment-matrix-card component matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`]).
//! Where the freeze matrix defines the reusable test-tree row, inline result marker,
//! session-summary bar, watch-mode banner, failure-triage panel, quarantine-review sheet, and
//! environment-matrix card primitives, and the 909-913 implementation / consumer lanes resolve
//! their per-surface truth, this lane certifies — per component family — that test-result
//! claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
//! self-narrowing** rather than presenting imported or stale evidence, a reduced-fidelity watch,
//! a widened rerun scope, or an expired / policy-blocked quarantine as a still trusted-live
//! result:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same test identity class,
//!   imported/live result origin, freshness, target class, environment lane, watch fidelity,
//!   retry / attempt lineage, mute / quarantine ownership, and release impact the rich component
//!   shows — never a pointer-only or hover-only chip that strands assistive-tech or headless
//!   users. Keyboard / screen-reader flows cover run, rerun-failed, debug-failed, open-triage,
//!   expand-parameterized-cases, inspect-watch-state, and review-quarantine. Hierarchy-heavy
//!   families (the environment-matrix card's nested target × environment legs and the
//!   failure-triage panel's nested recent attempts) additionally bind their tree to a flat list
//!   / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning
//!   from typed tokens and opaque refs without a screenshot, preserving the same stable test IDs,
//!   target class, freshness, watch-state vocabulary, quarantine ownership, and
//!   widening-selection notes shown in-product so test / watch / quarantine truth can be
//!   reconstructed without screenshots or private team memory.
//! - **Honest auto-narrowing.** When result evidence is imported or stale, watch fidelity is
//!   reduced, the rerun selection widens, or a quarantine is expired / policy-blocked, the
//!   component's test claim auto-narrows from `TrustedLiveResult` / `ReviewableResult` to an
//!   imported-or-stale / reduced-watch / widened-selection / restricted-quarantine result,
//!   discloses the narrowing with a precise trigger and binding dimension, and preserves the
//!   canonical identity / origin / attempt / retry lineage — the underlying result lineage is
//!   never dropped opaquely. A component with every dimension intact must NOT carry a spurious
//!   narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the test-tree UI,
//!   editor-gutter, session-summary, watch-banner, triage-panel, quarantine-sheet, headless CLI,
//!   and support / release exports so product, docs, and release publication stay aligned on
//!   test-component downgrade behavior rather than drifting in copy — a live-looking red or green
//!   mark can never outrun the origin / freshness / watch / selection / quarantine proof it is
//!   being viewed away from.
//!
//! Each [`TestComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::M5TestExplorerWatchTriageComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5TestRequiredLabel`] and
//! [`M5TestDowngradeTrigger`] and the shared [`M5TestConsumerSurface`] consumer surfaces rather
//! than minting parallel synonyms, so the certified labels stay byte-identical to the matrix and
//! the sibling primitive packets.
//!
//! The packet is metadata-only: raw logs, assertion bodies, transcripts, attachment bytes, and
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
use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    M5TestConsumerSurface, M5TestDowngradeTrigger, M5TestExplorerWatchTriageComponentFamily,
    M5TestRequiredLabel,
};

/// Schema version stamped on the M05-914 test-explorer / watch / triage component accessibility
/// fallback packet.
pub const TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TestComponentAccessibilityPacket`].
pub const TEST_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_test_explorer_watch_triage_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`TestComponentAccessibilityRow`].
pub const TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_test_explorer_watch_triage_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/testing/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_result_evidence_is_imported_or_stale_watch_fidelity_is_reduced_selection_widens_or_quarantine_state_is_expired_or_policy_blocked_across_claimed_m5_test_components.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component matrix this lane
/// certifies.
pub const TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const TEST_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-test-explorer-watch-triage-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const TEST_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEST_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEST_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the environment-matrix
/// card's nested target × environment legs and the failure-triage panel's nested recent
/// attempts) and therefore MUST bind their tree to an equivalent flat list / textual path so the
/// hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5TestExplorerWatchTriageComponentFamily) -> bool {
    matches!(
        family,
        M5TestExplorerWatchTriageComponentFamily::EnvironmentMatrixCard
            | M5TestExplorerWatchTriageComponentFamily::FailureTriagePanel
    )
}

/// The test dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5TestExplorerWatchTriageComponentFamily,
) -> M5TestComponentClaimDimension {
    match family {
        M5TestExplorerWatchTriageComponentFamily::TestTreeRow => {
            M5TestComponentClaimDimension::ResultEvidence
        }
        M5TestExplorerWatchTriageComponentFamily::InlineResultMarker => {
            M5TestComponentClaimDimension::ResultEvidence
        }
        M5TestExplorerWatchTriageComponentFamily::SessionSummaryBar => {
            M5TestComponentClaimDimension::SelectionScope
        }
        M5TestExplorerWatchTriageComponentFamily::WatchModeBanner => {
            M5TestComponentClaimDimension::WatchFidelity
        }
        M5TestExplorerWatchTriageComponentFamily::FailureTriagePanel => {
            M5TestComponentClaimDimension::ResultEvidence
        }
        M5TestExplorerWatchTriageComponentFamily::QuarantineReviewSheet => {
            M5TestComponentClaimDimension::QuarantineVisibility
        }
        M5TestExplorerWatchTriageComponentFamily::EnvironmentMatrixCard => {
            M5TestComponentClaimDimension::ResultEvidence
        }
    }
}

/// A rendered fallback modality for a test-explorer / watch / triage component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestComponentFallbackModality {
    /// A rich, structured (nested env-matrix / attempt tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5TestComponentFallbackModality {
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
pub enum M5TestComponentRenderingSurface {
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

impl M5TestComponentRenderingSurface {
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
pub enum TestComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl TestComponentNonVisualReachState {
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
pub enum TestComponentExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl TestComponentExportSummaryState {
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
pub enum TestComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl TestComponentNarrowingDisclosureState {
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

/// The test claim ceiling a component asserts: how strong a test-result posture it lets a surface
/// present. Auto-narrowing lowers this ceiling when a test dimension weakens so imported or stale
/// evidence, a reduced-fidelity watch, a widened rerun scope, or an expired / policy-blocked
/// quarantine can never keep an old `TrustedLiveResult` or `ReviewableResult` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestComponentClaim {
    /// Trusted live result: a live-local, fresh result produced here for the exact selection with
    /// a governed quarantine — the strongest claim, a red or green mark a user can act on as-is.
    TrustedLiveResult,
    /// Reviewable result: a self-consistent, reviewable result (evidence a reviewer can read)
    /// that is not itself a certified trusted-live signal.
    ReviewableResult,
    /// Widened-selection result: usable, but the rerun scope widened beyond the exact selection —
    /// the result cannot be read as covering only what was asked.
    WidenedSelectionResult,
    /// Reduced-watch result: the result stands, but watch fidelity is reduced and live certainty
    /// dropped; the mark cannot claim to be tracking live.
    ReducedWatchResult,
    /// Imported-or-stale result: the evidence is imported or stale; the mark is attributable but
    /// cannot be read as a fresh local result.
    ImportedOrStaleResult,
    /// Restricted-quarantine result: the quarantine is expired or policy-blocked; the mark stays
    /// a restricted-visibility explanation and cannot claim clean release impact.
    RestrictedQuarantineResult,
}

impl M5TestComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::TrustedLiveResult,
        Self::ReviewableResult,
        Self::WidenedSelectionResult,
        Self::ReducedWatchResult,
        Self::ImportedOrStaleResult,
        Self::RestrictedQuarantineResult,
    ];

    /// Capability rank; a higher rank asserts a stronger test posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedLiveResult => 5,
            Self::ReviewableResult => 4,
            Self::WidenedSelectionResult => 3,
            Self::ReducedWatchResult => 2,
            Self::ImportedOrStaleResult => 1,
            Self::RestrictedQuarantineResult => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted-live result.
    pub const fn asserts_trusted_live(self) -> bool {
        matches!(self, Self::TrustedLiveResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted-live or reviewable)
    /// result.
    pub const fn asserts_full_result(self) -> bool {
        matches!(self, Self::TrustedLiveResult | Self::ReviewableResult)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedLiveResult => "trusted_live_result",
            Self::ReviewableResult => "reviewable_result",
            Self::WidenedSelectionResult => "widened_selection_result",
            Self::ReducedWatchResult => "reduced_watch_result",
            Self::ImportedOrStaleResult => "imported_or_stale_result",
            Self::RestrictedQuarantineResult => "restricted_quarantine_result",
        }
    }
}

/// The test dimension whose state governs how far a component may claim to be a trusted-live
/// result. The four spec axes the lane must auto-narrow on — imported / stale result evidence,
/// reduced watch fidelity, a widened rerun selection, and an expired / policy-blocked quarantine —
/// are [`Self::ResultEvidence`], [`Self::WatchFidelity`], [`Self::SelectionScope`], and
/// [`Self::QuarantineVisibility`]; every frozen family maps onto one of these axes so each carries
/// an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestComponentClaimDimension {
    /// Result evidence: is the result live-local and fresh, or is it imported / stale?
    ResultEvidence,
    /// Watch fidelity: is watch mode observing live, or is fidelity reduced?
    WatchFidelity,
    /// Selection scope: does the rerun cover exactly the selection, or has it widened?
    SelectionScope,
    /// Quarantine visibility: is the mute / quarantine governed and current, or expired /
    /// policy-blocked?
    QuarantineVisibility,
}

impl M5TestComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ResultEvidence,
        Self::WatchFidelity,
        Self::SelectionScope,
        Self::QuarantineVisibility,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a narrowing.
    /// Each dimension maps to the on-topic frozen trigger the freeze matrix already governs, so
    /// the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5TestDowngradeTrigger {
        match self {
            Self::ResultEvidence => M5TestDowngradeTrigger::ResultOriginUnstated,
            Self::WatchFidelity => M5TestDowngradeTrigger::WatchFidelityUnstated,
            Self::SelectionScope => M5TestDowngradeTrigger::RerunScopeWidened,
            Self::QuarantineVisibility => M5TestDowngradeTrigger::QuarantineReleaseImpactHidden,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultEvidence => "result_evidence",
            Self::WatchFidelity => "watch_fidelity",
            Self::SelectionScope => "selection_scope",
            Self::QuarantineVisibility => "quarantine_visibility",
        }
    }
}

/// The observed condition of one test dimension. Anything weaker than [`Self::ResultsLiveExact`]
/// imposes a narrowing ceiling on the component's test claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestComponentConditionState {
    /// Live-local, fresh, exact-selection, governed quarantine — imposes no ceiling.
    ResultsLiveExact,
    /// Result evidence is imported or stale — the mark drops to imported-or-stale.
    EvidenceImportedOrStale,
    /// Watch fidelity is reduced — live certainty dropped; the mark drops to reduced-watch.
    WatchFidelityReduced,
    /// The rerun selection widened beyond what was asked — the mark drops to widened-selection.
    SelectionWidened,
    /// The quarantine is expired or policy-blocked — the mark drops to restricted-quarantine.
    QuarantineExpiredOrBlocked,
}

impl M5TestComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ResultsLiveExact,
        Self::EvidenceImportedOrStale,
        Self::WatchFidelityReduced,
        Self::SelectionWidened,
        Self::QuarantineExpiredOrBlocked,
    ];

    /// Returns true when the dimension is weaker than live-exact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::ResultsLiveExact)
    }

    /// The strongest test claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5TestComponentClaim {
        match self {
            Self::ResultsLiveExact => M5TestComponentClaim::TrustedLiveResult,
            Self::EvidenceImportedOrStale => M5TestComponentClaim::ImportedOrStaleResult,
            Self::WatchFidelityReduced => M5TestComponentClaim::ReducedWatchResult,
            Self::SelectionWidened => M5TestComponentClaim::WidenedSelectionResult,
            Self::QuarantineExpiredOrBlocked => M5TestComponentClaim::RestrictedQuarantineResult,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultsLiveExact => "results_live_exact",
            Self::EvidenceImportedOrStale => "evidence_imported_or_stale",
            Self::WatchFidelityReduced => "watch_fidelity_reduced",
            Self::SelectionWidened => "selection_widened",
            Self::QuarantineExpiredOrBlocked => "quarantine_expired_or_blocked",
        }
    }
}

/// One test dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5TestComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5TestComponentConditionState,
}

/// An honest test-claim auto-narrow block. When a test dimension weakens, the component's test
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and
/// preserves the canonical identity / origin / attempt / retry lineage rather than silently
/// dropping it — the underlying result lineage is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestComponentClaimAutoNarrow {
    /// The test claim the component is narrowed to.
    pub narrowed_to: M5TestComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5TestComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5TestDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical test identity, imported/live origin, target class, environment lane, and
    /// quarantine ownership are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying result / attempt / retry lineage is preserved (never dropped) across the
    /// narrowing; must hold so imported, stale, reduced-watch, widened, and restricted-quarantine
    /// states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl TestComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and result
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
pub struct TestComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl TestComponentCopyExportParity {
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
pub struct TestComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5TestComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: TestComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a test-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims a live result, or drops state
    /// silently (red).
    Stranded,
}

impl TestComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one test-explorer / watch / triage component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestComponentAccessibilityRow {
    /// Record kind; must equal [`TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5TestExplorerWatchTriageComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the test item / session / attempt / quarantine object this component acts
    /// on; stays visible on every surface, so this is never empty.
    pub test_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5TestComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, origin, freshness, target,
    /// environment, watch, attempt, and quarantine truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: TestComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: TestComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: TestComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: TestComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: TestComponentCopyExportParity,
    /// The full test claim this family asserts when every dimension is intact.
    pub full_test_claim: M5TestComponentClaim,
    /// The observed condition of each modeled test dimension.
    #[serde(default)]
    pub claim_conditions: Vec<TestComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<TestComponentClaimAutoNarrow>,
    /// Whether the underlying result / attempt / retry lineage is preserved on this component
    /// regardless of narrowing; must hold so imported, stale, reduced-watch, widened, and
    /// restricted-quarantine states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5TestComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<TestComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5TestRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5TestConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl TestComponentAccessibilityRow {
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

    /// The condition state observed for one dimension, or `ResultsLiveExact` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5TestComponentClaimDimension,
    ) -> M5TestComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5TestComponentConditionState::ResultsLiveExact)
    }

    /// Whether any modeled dimension is weaker than live-exact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest test claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5TestComponentClaim {
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
    pub fn binding_dimension(&self) -> Option<M5TestComponentClaimDimension> {
        let mut binding: Option<(M5TestComponentClaimDimension, u8)> = None;
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

    /// The test claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5TestComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_test_claim,
        }
    }

    /// AC / auto-narrowing honesty: imported or stale evidence, a reduced-fidelity watch, a
    /// widened rerun scope, or an expired / policy-blocked quarantine can no longer keep an old
    /// `TrustedLiveResult` / `ReviewableResult` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block
    /// is present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing
    /// dimension with its frozen trigger, and preserves canonical identity and result lineage.
    /// When nothing narrows, no spurious narrow block is present.
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

    /// AC / no-loss: imported, stale, reduced-watch, widened, and restricted-quarantine states
    /// preserve the underlying result / attempt / retry lineage. The row must assert
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
        M5TestRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> TestComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return TestComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            TestComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            TestComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-914 test-explorer / watch / triage component accessibility
/// fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestComponentAccessibilitySummary {
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

/// Constructor input for [`TestComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<TestComponentAccessibilityRow>,
}

/// Checked-in M05-914 test-explorer / watch / triage component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<TestComponentAccessibilityRow>,
    pub summary: TestComponentAccessibilitySummary,
}

impl TestComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TestComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: TestComponentAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5TestExplorerWatchTriageComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5TestComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5TestComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Test claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5TestComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5TestConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TestComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5TestConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&TestComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                TestComponentAccessibilityStatus::Parity => green += 1,
                TestComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                TestComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        TestComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(TestComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(TestComponentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(TestComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(TestComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(TestComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TestComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(TestComponentAccessibilityViolation::SchemaVersion {
                expected: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEST_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(TestComponentAccessibilityViolation::RecordKind {
                expected: TEST_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TestComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TestComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(TestComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    TestComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory test label.
            if !row.preserves_mandatory_labels() {
                violations.push(TestComponentAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5TestComponentFallbackModality::Structured)
            {
                violations.push(
                    TestComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a trusted-live / reviewable result for a weakened one.
            if !row.claim_is_honest() {
                violations.push(TestComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(TestComponentAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    TestComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: imported, stale, reduced-watch, widened, and restricted-quarantine states
            // preserve result lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(TestComponentAccessibilityViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    TestComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(TestComponentAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == TestComponentAccessibilityStatus::Stranded {
                violations.push(TestComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5TestExplorerWatchTriageComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(TestComponentAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5TestComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    TestComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the live-exact baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5TestComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    TestComponentAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every test claim tier appears as an effective claim, so the full narrowing
        // spectrum (trusted-live → … → restricted-quarantine) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5TestComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(TestComponentAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach the test-tree UI, editor-gutter,
        // session-summary, watch-banner, triage-panel, quarantine-sheet, CLI, and support /
        // release exports — so every consumer surface is exercised at least once across the
        // packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5TestConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    TestComponentAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(TestComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("test-explorer / watch / triage accessibility fallback packet serializes"),
        ) {
            violations.push(TestComponentAccessibilityViolation::RawTestMaterialInExport);
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
            .expect("test-explorer / watch / triage accessibility fallback packet serializes")
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
        out.push_str(
            "# M5 Test-Explorer / Watch / Triage Component Accessibility & Auto-Narrowing\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5TestExplorerWatchTriageComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in test-explorer / watch / triage component accessibility
/// fallback export.
pub fn current_m5_test_component_a11y_fallback_export(
) -> Result<TestComponentAccessibilityPacket, TestComponentAccessibilityArtifactError> {
    let packet: TestComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-explorer-watch-triage-component-accessibility-fallback/support_export.json"
    )))
    .map_err(TestComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in test-explorer / watch / triage component
/// accessibility fallback export.
#[derive(Debug)]
pub enum TestComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestComponentAccessibilityViolation>),
}

impl fmt::Display for TestComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "test-explorer / watch / triage accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "test-explorer / watch / triage accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for TestComponentAccessibilityArtifactError {}

/// Validation failure for M05-914 test-explorer / watch / triage component accessibility fallback
/// packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestComponentAccessibilityViolation {
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
        dimension: M5TestComponentClaimDimension,
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
        family: M5TestExplorerWatchTriageComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5TestComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5TestComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5TestComponentClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5TestConsumerSurface,
    },
    SummaryMismatch,
    RawTestMaterialInExport,
}

impl fmt::Display for TestComponentAccessibilityViolation {
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
                    "row {id} over-asserts a trusted-live / reviewable result for a weakened one, or narrows spuriously"
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
                    "row {id} does not preserve result / attempt lineage across narrowing"
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
                    "test claim tier {} does not appear as an effective claim",
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

impl Error for TestComponentAccessibilityViolation {}

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
            | "widened"
            | "widened selection"
            | "restricted"
            | "expired"
            | "policy blocked"
            | "quarantined"
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

/// Builds the canonical, checked-in test-explorer / watch / triage component accessibility
/// fallback packet. This is the one source of truth shared by the tests and the on-disk support
/// export so both stay byte-aligned.
pub fn seeded_m5_test_component_a11y_fallback_packet() -> TestComponentAccessibilityPacket {
    TestComponentAccessibilityPacket::new(TestComponentAccessibilityPacketInput {
        packet_id: "m5-test-explorer-watch-triage-component-accessibility-fallback:stable:0001"
            .to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:test-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5TestRequiredLabel> {
    M5TestRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> TestComponentCopyExportParity {
    TestComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5TestComponentClaimDimension,
    state: M5TestComponentConditionState,
) -> TestComponentClaimConditionEntry {
    TestComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5TestConsumerSurface]) -> Vec<M5TestConsumerSurface> {
    let mut out = vec![
        M5TestConsumerSurface::SupportExport,
        M5TestConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: TestComponentNarrowingDisclosureState,
) -> Vec<TestComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        TestComponentRenderingNarrowingDisclosure {
            rendering_surface: M5TestComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        TestComponentRenderingNarrowingDisclosure {
            rendering_surface: M5TestComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_rerun".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<TestComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        TestComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<TestComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        TestComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5TestComponentRenderingSurface> {
    vec![
        M5TestComponentRenderingSurface::DesktopFull,
        M5TestComponentRenderingSurface::CliHeadless,
        M5TestComponentRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<TestComponentAccessibilityRow> {
    vec![
        // Test-tree row — a live-local, fresh result for the exact selection with a governed
        // quarantine; a trusted-live result reachable on every surface (green).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:test-tree-row".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::TestTreeRow,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:tree-row:0001".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:test-tree-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "test_identity_class",
                "result_origin",
                "target_class",
                "keyboard_route",
            ]),
            full_test_claim: M5TestComponentClaim::TrustedLiveResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::ResultEvidence,
                M5TestComponentConditionState::ResultsLiveExact,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "test_identity_class",
                "result_origin",
                "target_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::TestTreeUi,
                M5TestConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TDD §8.58 test explorer rows".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("test-tree-row"),
        },
        // Inline result marker — the mark is backed by imported / stale evidence rather than a
        // fresh local run, so it auto-narrows to an imported-or-stale result rather than showing a
        // live-local certainty, while keeping its identity and imported origin visible (yellow).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:inline-result-marker-imported".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::InlineResultMarker,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:inline-result-marker:0002".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:inline-result-marker-imported:a11y".to_owned(),
            copy_export: copy_export(&[
                "marker_verdict",
                "result_origin",
                "result_freshness",
                "attempt_lineage",
            ]),
            full_test_claim: M5TestComponentClaim::TrustedLiveResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::ResultEvidence,
                M5TestComponentConditionState::EvidenceImportedOrStale,
            )],
            claim_narrow: Some(TestComponentClaimAutoNarrow {
                narrowed_to: M5TestComponentClaim::ImportedOrStaleResult,
                binding_dimension: M5TestComponentClaimDimension::ResultEvidence,
                trigger: M5TestDowngradeTrigger::ResultOriginUnstated,
                narrowed_label:
                    "Mark is backed by imported CI evidence, not a fresh local run — shown as an imported-or-stale result with its origin and attempt lineage preserved, never as a live-local certainty"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "marker_verdict",
                "result_origin",
                "result_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::EditorGutterUi,
                M5TestConsumerSurface::TestTreeUi,
            ]),
            source_refs: vec![
                "TDD §9.51 inline result markers".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("inline-result-marker-imported"),
        },
        // Session-summary bar — the rerun widened beyond the exact selection, so the bar
        // auto-narrows to a widened-selection result rather than presenting a result covering only
        // what was asked, while keeping the exact original selection and attempt lineage visible
        // (yellow).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:session-summary-bar-widened".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::SessionSummaryBar,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:session-summary-bar:0003".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:session-summary-bar-widened:a11y".to_owned(),
            copy_export: copy_export(&[
                "session_outcome",
                "exact_selection",
                "widened_selection_note",
                "attempt_lineage",
            ]),
            full_test_claim: M5TestComponentClaim::TrustedLiveResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::SelectionScope,
                M5TestComponentConditionState::SelectionWidened,
            )],
            claim_narrow: Some(TestComponentClaimAutoNarrow {
                narrowed_to: M5TestComponentClaim::WidenedSelectionResult,
                binding_dimension: M5TestComponentClaimDimension::SelectionScope,
                trigger: M5TestDowngradeTrigger::RerunScopeWidened,
                narrowed_label:
                    "Rerun covered more than the exact selection — shown as a widened-selection result that names the original selection and what the rerun added, never as an exact-selection run"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "session_outcome",
                "exact_selection",
                "widened_selection_note",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::SessionSummaryUi,
                M5TestConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TDD §9.51 session summaries".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("session-summary-bar-widened"),
        },
        // Watch-mode banner — watch fidelity dropped to reduced, so the banner auto-narrows to a
        // reduced-watch result rather than claiming a live watch, while keeping the degrade reason
        // and last-successful cycle visible (yellow).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:watch-mode-banner-reduced".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::WatchModeBanner,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:watch-mode-banner:0004".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:watch-mode-banner-reduced:a11y".to_owned(),
            copy_export: copy_export(&[
                "watch_fidelity",
                "watch_degrade_reason",
                "last_successful_cycle",
                "keyboard_route",
            ]),
            full_test_claim: M5TestComponentClaim::TrustedLiveResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::WatchFidelity,
                M5TestComponentConditionState::WatchFidelityReduced,
            )],
            claim_narrow: Some(TestComponentClaimAutoNarrow {
                narrowed_to: M5TestComponentClaim::ReducedWatchResult,
                binding_dimension: M5TestComponentClaimDimension::WatchFidelity,
                trigger: M5TestDowngradeTrigger::WatchFidelityUnstated,
                narrowed_label:
                    "Watch fidelity dropped to reduced under resource pressure — shown as a reduced-watch result that names the degrade reason and last successful cycle, never as a live watch"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "watch_fidelity",
                "watch_degrade_reason",
                "last_successful_cycle",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::WatchBannerUi,
                M5TestConsumerSurface::SessionSummaryUi,
            ]),
            source_refs: vec![
                "TDD Appendix L.22 watch-state transitions".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("watch-mode-banner-reduced"),
        },
        // Failure-triage panel — hierarchy-heavy (nested recent attempts + assertion / diff
        // summaries); the panel is a self-consistent reviewable case built from live-local
        // attempts (not itself a certified trusted-live signal), reachable on every surface
        // (green).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:failure-triage-panel-reviewable".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::FailureTriagePanel,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:failure-triage-panel:0005".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::Structured,
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:failure-triage-panel-reviewable:a11y".to_owned(),
            copy_export: copy_export(&[
                "failure_category",
                "triage_disposition",
                "recent_attempts",
                "assertion_diff_summary",
            ]),
            full_test_claim: M5TestComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::ResultEvidence,
                M5TestComponentConditionState::ResultsLiveExact,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "failure_category",
                "triage_disposition",
                "recent_attempts",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::TriagePanelUi,
                M5TestConsumerSurface::EditorGutterUi,
            ]),
            source_refs: vec![
                "TDD §9.52 triage packets".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("failure-triage-panel-reviewable"),
        },
        // Quarantine-review sheet — the quarantine is expired / policy-blocked, so the sheet
        // auto-narrows to a restricted-quarantine result rather than presenting clean release
        // impact, while keeping the ownership, expiry, and release impact visible (yellow).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:quarantine-review-sheet-restricted".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::QuarantineReviewSheet,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:quarantine-review-sheet:0006".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:quarantine-review-sheet-restricted:a11y".to_owned(),
            copy_export: copy_export(&[
                "quarantine_ownership",
                "quarantine_expiry",
                "release_impact",
                "keyboard_route",
            ]),
            full_test_claim: M5TestComponentClaim::TrustedLiveResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::QuarantineVisibility,
                M5TestComponentConditionState::QuarantineExpiredOrBlocked,
            )],
            claim_narrow: Some(TestComponentClaimAutoNarrow {
                narrowed_to: M5TestComponentClaim::RestrictedQuarantineResult,
                binding_dimension: M5TestComponentClaimDimension::QuarantineVisibility,
                trigger: M5TestDowngradeTrigger::QuarantineReleaseImpactHidden,
                narrowed_label:
                    "Quarantine ownership has expired and its visibility is policy-restricted — shown as a restricted-quarantine result that names the owner, expiry, and hidden release impact, never as a clean release signal"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "quarantine_ownership",
                "quarantine_expiry",
                "release_impact",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::QuarantineSheetUi,
                M5TestConsumerSurface::TriagePanelUi,
            ]),
            source_refs: vec![
                "TDD §7.6.18.2 quarantine object model".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("quarantine-review-sheet-restricted"),
        },
        // Environment-matrix card — hierarchy-heavy (nested target × environment legs); every leg
        // is a live-local, fresh result, so the card is a trusted-live result that binds its
        // nested matrix to a flat list / textual path, reachable on every surface (green).
        TestComponentAccessibilityRow {
            record_kind: TEST_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: TEST_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:environment-matrix-card".to_owned(),
            component_family: M5TestExplorerWatchTriageComponentFamily::EnvironmentMatrixCard,
            source_family_schema_ref: TEST_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            test_context_ref: "test:environment-matrix-card:0007".to_owned(),
            fallback_modalities: vec![
                M5TestComponentFallbackModality::Structured,
                M5TestComponentFallbackModality::List,
                M5TestComponentFallbackModality::Textual,
                M5TestComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TestComponentNonVisualReachState::ReachableAndLabeled,
            export_summary: TestComponentExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:environment-matrix-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "target_class",
                "environment_lane",
                "per_leg_result_origin",
                "keyboard_route",
            ]),
            full_test_claim: M5TestComponentClaim::TrustedLiveResult,
            claim_conditions: vec![condition(
                M5TestComponentClaimDimension::ResultEvidence,
                M5TestComponentConditionState::ResultsLiveExact,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "target_class",
                "environment_lane",
                "per_leg_result_origin",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5TestConsumerSurface::ProductUi,
                M5TestConsumerSurface::WatchBannerUi,
            ]),
            source_refs: vec![
                "TDD §9.52 environment-matrix cards".to_owned(),
                TEST_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("environment-matrix-card"),
        },
    ]
}
