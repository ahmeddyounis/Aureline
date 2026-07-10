//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for the
//! M5 governance-dashboard components.
//!
//! This module is the M05-1058 accessibility-and-auto-narrowing capstone over the
//! frozen M5 governance-dashboard component matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`]).
//! Where the freeze matrix defines the reusable fitness dashboard tile, governance
//! report row, waiver-expiry queue item, release-gate banner, mitigation note card,
//! service-ownership card, on-call strip, decision-right card, and milestone dashboard
//! row primitives, and the 1053-1056 implementation lanes resolve their per-surface
//! truth, this lane certifies — per component family — that governance-dashboard claims
//! stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
//! self-narrowing** rather than presenting stale evidence, an expiring waiver, a
//! partial owner coverage, an unresolved decision forum, or a downgraded support class
//! as a still-clean green pass:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into
//!   the same readiness state, evidence freshness, waiver expiry, owner coverage,
//!   escalation route, decision forum, and blocker/waiver counts the rich surface
//!   shows — never a view-only tile that strands assistive-tech or headless users.
//!   Hierarchy-heavy families (the milestone dashboard row's exit-gate tree with its
//!   per-gate blocker / waiver sub-rows) additionally bind their tree to a flat list /
//!   textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot,
//!   preserving the same readiness states, freshness, owners, forums, and counts shown
//!   in-product.
//! - **Honest auto-narrowing.** When evidence freshness, waiver expiry, owner coverage,
//!   support class, or decision-right truth becomes stale, partial, expiring, or
//!   unresolved, the component's governance-support claim auto-narrows from a clean
//!   governed pass to degraded / provisional / waiver-gated / blocked, discloses the
//!   narrowing with a precise trigger and binding dimension, and preserves the
//!   canonical governance identity rather than silently collapsing it into a generic
//!   warning. A component with every dimension intact must NOT carry a spurious
//!   narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the assurance
//!   dashboard, operator board, release center, shiproom packet, service-health, docs /
//!   help, headless CLI, and support / admin exports so governed truth and field
//!   triage stay aligned on governance-dashboard downgrade behavior — a clean-looking
//!   claim can never outrun the proof it is being viewed away from.
//!
//! Each [`GovernanceAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::M5GovernanceDashboardComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5GovernanceRequiredLabel`] and [`M5GovernanceDowngradeTrigger`] and the shared
//! [`M5GovernanceConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the sibling
//! primitive packets.
//!
//! The packet is metadata-only: raw evidence bodies, waiver text, owner PII, and forum
//! transcripts never cross this boundary; the packet carries only typed class tokens,
//! opaque summary / evidence refs, booleans, and redacted labels so support and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have
//! shown without leaking governance material.
//!
//! The boundary schema is
//! [`schemas/ui/m5-governance-dashboard-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-governance-dashboard-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/help/m5_governance_dashboard_component_accessibility_parity.md`](../../../../docs/help/m5_governance_dashboard_component_accessibility_parity.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, downgrade triggers, and consumer surfaces rather than mint
// parallel ones.
use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::{
    M5GovernanceConsumerSurface, M5GovernanceDashboardComponentFamily,
    M5GovernanceDowngradeTrigger, M5GovernanceRequiredLabel, M5_GOVERNANCE_DASHBOARD_SCHEMA_REF,
};

/// Schema version stamped on the M05-1058 governance-dashboard component accessibility
/// parity packet.
pub const GOVERNANCE_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`GovernanceAccessibilityPacket`].
pub const GOVERNANCE_A11Y_RECORD_KIND: &str =
    "m5_governance_dashboard_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`GovernanceAccessibilityRow`].
pub const GOVERNANCE_A11Y_ROW_RECORD_KIND: &str =
    "m5_governance_dashboard_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const GOVERNANCE_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const GOVERNANCE_A11Y_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_component_accessibility_parity.md";

/// Repo-relative path of the frozen governance-dashboard component matrix this lane
/// certifies.
pub const GOVERNANCE_A11Y_COMPONENT_MATRIX_REF: &str = M5_GOVERNANCE_DASHBOARD_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const GOVERNANCE_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-governance-dashboard-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const GOVERNANCE_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const GOVERNANCE_A11Y_CSV_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const GOVERNANCE_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-accessibility-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the milestone
/// dashboard row's exit-gate tree with its per-gate blocker / waiver sub-rows) and
/// therefore MUST bind their tree to an equivalent flat list / textual path so the
/// hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5GovernanceDashboardComponentFamily) -> bool {
    matches!(
        family,
        M5GovernanceDashboardComponentFamily::MilestoneDashboardRow
    )
}

/// The governance dimension whose weakening a family primarily discloses. Every row
/// must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5GovernanceDashboardComponentFamily,
) -> M5GovernanceClaimDimension {
    match family {
        M5GovernanceDashboardComponentFamily::FitnessDashboardTile => {
            M5GovernanceClaimDimension::EvidenceFreshness
        }
        M5GovernanceDashboardComponentFamily::GovernanceReportRow => {
            M5GovernanceClaimDimension::EvidenceFreshness
        }
        M5GovernanceDashboardComponentFamily::WaiverExpiryQueueItem => {
            M5GovernanceClaimDimension::WaiverExpiry
        }
        M5GovernanceDashboardComponentFamily::ReleaseGateBanner => {
            M5GovernanceClaimDimension::DecisionRightTruth
        }
        M5GovernanceDashboardComponentFamily::MitigationNoteCard => {
            M5GovernanceClaimDimension::SupportClass
        }
        M5GovernanceDashboardComponentFamily::ServiceOwnershipCard => {
            M5GovernanceClaimDimension::OwnerCoverage
        }
        M5GovernanceDashboardComponentFamily::OnCallStrip => {
            M5GovernanceClaimDimension::OwnerCoverage
        }
        M5GovernanceDashboardComponentFamily::DecisionRightCard => {
            M5GovernanceClaimDimension::DecisionRightTruth
        }
        M5GovernanceDashboardComponentFamily::MilestoneDashboardRow => {
            M5GovernanceClaimDimension::DecisionRightTruth
        }
    }
}

/// A rendered fallback modality for a governance-dashboard component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceFallbackModality {
    /// A rich, structured (exit-gate tree / grouped roll-up) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5GovernanceFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich,
    /// structured surface (i.e. a keyboard / screen-reader / headless path).
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface:
/// the same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceRenderingSurface {
    /// The full-capability desktop governance-dashboard surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin / evaluation export.
    SupportExport,
}

impl M5GovernanceRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
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
pub enum GovernanceNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only surface that traps keyboard / assistive-tech / headless users (red).
    ViewOnlyTrap,
}

impl GovernanceNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless
    /// users.
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
pub enum GovernanceExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl GovernanceExportSummaryState {
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
pub enum GovernanceNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl GovernanceNarrowingDisclosureState {
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

/// The governance-support claim ceiling a component asserts: how strong a governance
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a
/// governance dimension weakens so a stale, expiring, uncovered, or unresolved lane can
/// never keep a clean green pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceSupportClaim {
    /// Governed pass: a fully proven, current, owned, waiver-clean, forum-authoritative
    /// reading — the strongest claim, and the only clean green pass.
    GovernedPass,
    /// Governed resolved: a resolved, self-sufficient governance object (a report row
    /// or waiver-queue item) that is not itself a certified pass reading.
    GovernedResolved,
    /// Degraded: usable, but with a disclosed reduction in scope or confidence.
    Degraded,
    /// Provisional: the underlying evidence is stale and being re-established; the state
    /// is last-known, not current.
    Provisional,
    /// Waiver-gated: an active or expiring waiver holds the lane; it may not read as a
    /// clean pass.
    WaiverGated,
    /// Blocked: a required owner or decision forum is unresolved, so the move cannot be
    /// authoritatively approved from here.
    Blocked,
}

impl M5GovernanceSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::GovernedPass,
        Self::GovernedResolved,
        Self::Degraded,
        Self::Provisional,
        Self::WaiverGated,
        Self::Blocked,
    ];

    /// Capability rank; a higher rank asserts a stronger governance posture. Narrowing
    /// lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::GovernedPass => 5,
            Self::GovernedResolved => 4,
            Self::Degraded => 3,
            Self::Provisional => 2,
            Self::WaiverGated => 1,
            Self::Blocked => 0,
        }
    }

    /// Returns true when this claim asserts a clean governed green pass.
    pub const fn asserts_clean_pass(self) -> bool {
        matches!(self, Self::GovernedPass)
    }

    /// Returns true when this claim asserts a fully self-sufficient (clean pass or
    /// resolved) posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::GovernedPass | Self::GovernedResolved)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GovernedPass => "governed_pass",
            Self::GovernedResolved => "governed_resolved",
            Self::Degraded => "degraded",
            Self::Provisional => "provisional",
            Self::WaiverGated => "waiver_gated",
            Self::Blocked => "blocked",
        }
    }
}

/// The governance dimension whose state governs how far a component may claim to be a
/// clean, owned, authoritative pass. These are exactly the five axes the spec requires
/// auto-narrowing on — evidence freshness, waiver expiry, owner coverage, support class,
/// and decision-right truth — so every frozen family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceClaimDimension {
    /// Evidence freshness: is the lane's fitness / governance evidence current, or
    /// stale / pending re-verification?
    EvidenceFreshness,
    /// Waiver expiry: is the exception waiver active and clean, or expiring / expired?
    WaiverExpiry,
    /// Owner coverage: is the service owner covered with a backup and on-call route, or
    /// partial / uncovered?
    OwnerCoverage,
    /// Support class: is the lane's mitigation / supportability stated in reusable
    /// plain language, or hidden behind internal jargon / a partial support class?
    SupportClass,
    /// Decision-right truth: is the forum authorized to approve the next move
    /// authoritative and resolved, or advisory / masked / unresolved?
    DecisionRightTruth,
}

impl M5GovernanceClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EvidenceFreshness,
        Self::WaiverExpiry,
        Self::OwnerCoverage,
        Self::SupportClass,
        Self::DecisionRightTruth,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5GovernanceDowngradeTrigger {
        match self {
            Self::EvidenceFreshness => M5GovernanceDowngradeTrigger::EvidenceStaleHidden,
            Self::WaiverExpiry => M5GovernanceDowngradeTrigger::WaiverExpiryHidden,
            Self::OwnerCoverage => M5GovernanceDowngradeTrigger::OwnerCoverageOverstated,
            Self::SupportClass => M5GovernanceDowngradeTrigger::MitigationHiddenBehindJargon,
            Self::DecisionRightTruth => M5GovernanceDowngradeTrigger::DecisionForumMasked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceFreshness => "evidence_freshness",
            Self::WaiverExpiry => "waiver_expiry",
            Self::OwnerCoverage => "owner_coverage",
            Self::SupportClass => "support_class",
            Self::DecisionRightTruth => "decision_right_truth",
        }
    }
}

/// The observed condition of one governance dimension. Anything weaker than
/// [`Self::Current`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceConditionState {
    /// Fully current / fresh / owned / authoritative — imposes no ceiling.
    Current,
    /// Partially resolved — scope, owner coverage, or support class is reduced; support
    /// drops to degraded.
    Partial,
    /// Stale — the evidence aged out and is re-establishing; support drops to
    /// provisional.
    Stale,
    /// Waived — an active or expiring waiver holds the lane; support drops to
    /// waiver-gated and can no longer read as a clean pass.
    Waived,
    /// Unresolved — a required owner or decision forum is unresolved; support drops to
    /// blocked.
    Unresolved,
}

impl M5GovernanceConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Partial,
        Self::Stale,
        Self::Waived,
        Self::Unresolved,
    ];

    /// Returns true when the dimension is weaker than current and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Current)
    }

    /// The strongest governance-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5GovernanceSupportClaim {
        match self {
            Self::Current => M5GovernanceSupportClaim::GovernedPass,
            Self::Partial => M5GovernanceSupportClaim::Degraded,
            Self::Stale => M5GovernanceSupportClaim::Provisional,
            Self::Waived => M5GovernanceSupportClaim::WaiverGated,
            Self::Unresolved => M5GovernanceSupportClaim::Blocked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Waived => "waived",
            Self::Unresolved => "unresolved",
        }
    }
}

/// One governance dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5GovernanceClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5GovernanceConditionState,
}

/// An honest governance-support-claim auto-narrow block. When a governance dimension
/// weakens, the component's support claim lowers to the permitted ceiling, names the
/// binding dimension and frozen trigger, and preserves the canonical governance
/// identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5GovernanceSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5GovernanceClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5GovernanceDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical readiness / owner / forum / count identity is preserved rather than
    /// dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl GovernanceClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl GovernanceCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as
    /// the sole export.
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
pub struct GovernanceRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5GovernanceRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: GovernanceNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a governance accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims support, or drops state
    /// silently (red).
    Stranded,
}

impl GovernanceAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one governance-dashboard component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceAccessibilityRow {
    /// Record kind; must equal [`GOVERNANCE_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GOVERNANCE_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5GovernanceDashboardComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the governance lane / object this component acts on; stays visible
    /// on every surface, so this is never empty.
    pub governance_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5GovernanceFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical readiness, freshness,
    /// waiver, owner, forum, and count truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: GovernanceNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: GovernanceNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: GovernanceNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: GovernanceExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: GovernanceCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5GovernanceSupportClaim,
    /// The observed condition of each modeled governance dimension.
    #[serde(default)]
    pub claim_conditions: Vec<GovernanceClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<GovernanceClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5GovernanceRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<GovernanceRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5GovernanceRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5GovernanceConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl GovernanceAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a
    /// flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Current` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5GovernanceClaimDimension,
    ) -> M5GovernanceConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5GovernanceConditionState::Current)
    }

    /// Whether any modeled dimension is weaker than current.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's
    /// ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5GovernanceSupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5GovernanceClaimDimension> {
        let mut binding: Option<(M5GovernanceClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
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

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5GovernanceSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC2 / auto-narrowing honesty: a lane with stale evidence, an expiring waiver, or
    /// an unresolved owner / forum can no longer keep a clean green pass. The effective
    /// claim never exceeds the permitted ceiling; when a dimension narrows below the
    /// full claim, an honest narrow block is present, narrows to exactly the permitted
    /// ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and
    /// preserves canonical identity. When nothing narrows, no spurious narrow block is
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

    /// AC1 / assistive-tech reach: accessibility and export surfaces reach the same
    /// canonical truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without
    /// a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.governance_context_ref.trim().is_empty()
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

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component
    /// carries an honest claim narrow.
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

    /// AC3 / cross-surface disclosure: every narrower rendering surface discloses its
    /// reduced interactivity and keeps its labels, so governed truth and field triage
    /// stay aligned on the same narrowed state.
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
        // Every disclosure never silently drops and preserves labels on a narrowed
        // surface.
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
        M5GovernanceRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> GovernanceAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return GovernanceAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            GovernanceAccessibilityStatus::NarrowedDisclosed
        } else {
            GovernanceAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == GOVERNANCE_A11Y_ROW_RECORD_KIND
            && self.schema_version == GOVERNANCE_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.governance_context_ref.trim().is_empty()
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
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1058 governance-dashboard component accessibility parity
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`GovernanceAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<GovernanceAccessibilityRow>,
}

/// Checked-in M05-1058 governance-dashboard component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<GovernanceAccessibilityRow>,
    pub summary: GovernanceAccessibilitySummary,
}

impl GovernanceAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: GovernanceAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            record_kind: GOVERNANCE_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: GovernanceAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5GovernanceDashboardComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5GovernanceClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5GovernanceSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5GovernanceConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> GovernanceAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5GovernanceConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&GovernanceAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                GovernanceAccessibilityStatus::Parity => green += 1,
                GovernanceAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                GovernanceAccessibilityStatus::Stranded => red += 1,
            }
        }

        GovernanceAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(GovernanceAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(GovernanceAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(GovernanceAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(GovernanceAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<GovernanceAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != GOVERNANCE_A11Y_SCHEMA_VERSION {
            violations.push(GovernanceAccessibilityViolation::SchemaVersion {
                expected: GOVERNANCE_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != GOVERNANCE_A11Y_RECORD_KIND {
            violations.push(GovernanceAccessibilityViolation::RecordKind {
                expected: GOVERNANCE_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(GovernanceAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(GovernanceAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(GovernanceAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(GovernanceAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory governance label.
            if !row.preserves_mandatory_labels() {
                violations.push(GovernanceAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5GovernanceFallbackModality::Structured)
            {
                violations.push(
                    GovernanceAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: claim never over-asserts a clean pass for a weakened lane.
            if !row.claim_is_honest() {
                violations.push(GovernanceAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(GovernanceAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(GovernanceAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    GovernanceAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(GovernanceAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == GovernanceAccessibilityStatus::Stranded {
                violations.push(GovernanceAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5GovernanceDashboardComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(GovernanceAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5GovernanceClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(GovernanceAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (governed pass → … → blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5GovernanceSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(GovernanceAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach the assurance dashboard,
        // operator board, release center, shiproom, service-health, docs / help, CLI,
        // and support / admin exports — so every consumer surface is exercised at least
        // once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5GovernanceConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    GovernanceAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(GovernanceAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("governance accessibility parity packet serializes"),
        ) {
            violations.push(GovernanceAccessibilityViolation::RawGovernanceMaterialInExport);
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
            .expect("governance accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
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
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Governance-Dashboard Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5GovernanceDashboardComponentFamily::ALL.len(),
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
                    row.full_support_claim.as_str(),
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

/// Reads and validates the checked-in governance-dashboard component accessibility
/// parity export.
pub fn current_m5_governance_dashboard_a11y_parity_export(
) -> Result<GovernanceAccessibilityPacket, GovernanceAccessibilityArtifactError> {
    let packet: GovernanceAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-governance-dashboard-component-accessibility-proof/support_export.json"
    )))
    .map_err(GovernanceAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GovernanceAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in governance-dashboard component
/// accessibility parity export.
#[derive(Debug)]
pub enum GovernanceAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GovernanceAccessibilityViolation>),
}

impl fmt::Display for GovernanceAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "governance accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "governance accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for GovernanceAccessibilityArtifactError {}

/// Validation failure for M05-1058 governance-dashboard component accessibility parity
/// packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceAccessibilityViolation {
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
        dimension: M5GovernanceClaimDimension,
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
        family: M5GovernanceDashboardComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5GovernanceClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5GovernanceSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5GovernanceConsumerSurface,
    },
    SummaryMismatch,
    RawGovernanceMaterialInExport,
}

impl fmt::Display for GovernanceAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory governance label")
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
                    "row {id} over-asserts a clean governed pass for a weakened lane, or narrows spuriously"
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
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "support claim tier {} does not appear as an effective claim",
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
            Self::RawGovernanceMaterialInExport => {
                write!(f, "export contains raw governance material")
            }
        }
    }
}

impl Error for GovernanceAccessibilityViolation {}

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
            | "warning"
            | "blocked"
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

/// Builds the canonical, checked-in governance-dashboard component accessibility parity
/// packet. This is the one source of truth shared by the tests and the on-disk support
/// export so both stay byte-aligned.
pub fn seeded_m5_governance_dashboard_a11y_parity_packet() -> GovernanceAccessibilityPacket {
    GovernanceAccessibilityPacket::new(GovernanceAccessibilityPacketInput {
        packet_id: "m5-governance-dashboard-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:governance-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5GovernanceRequiredLabel> {
    M5GovernanceRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> GovernanceCopyExportParity {
    GovernanceCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5GovernanceClaimDimension,
    state: M5GovernanceConditionState,
) -> GovernanceClaimConditionEntry {
    GovernanceClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / admin export and
/// CLI inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5GovernanceConsumerSurface]) -> Vec<M5GovernanceConsumerSurface> {
    let mut out = vec![
        M5GovernanceConsumerSurface::SupportExport,
        M5GovernanceConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: GovernanceNarrowingDisclosureState,
) -> Vec<GovernanceRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        GovernanceRenderingNarrowingDisclosure {
            rendering_surface: M5GovernanceRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        GovernanceRenderingNarrowingDisclosure {
            rendering_surface: M5GovernanceRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label
/// and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<GovernanceRenderingNarrowingDisclosure> {
    surface_disclosures(labels, GovernanceNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<GovernanceRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        GovernanceNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5GovernanceRenderingSurface> {
    vec![
        M5GovernanceRenderingSurface::DesktopFull,
        M5GovernanceRenderingSurface::CliHeadless,
        M5GovernanceRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<GovernanceAccessibilityRow> {
    vec![
        // Fitness dashboard tile — the protected-metric evidence is current and the
        // corpus/profile provenance is stated, so the tile carries a clean governed pass
        // and is reachable on every surface (green).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:fitness-dashboard-tile".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::FitnessDashboardTile,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "fitness:tile:0001".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:fitness-dashboard-tile:a11y".to_owned(),
            copy_export: copy_export(&[
                "metric_identity",
                "readiness_state",
                "evidence_freshness",
                "corpus_profile_provenance",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::EvidenceFreshness,
                M5GovernanceConditionState::Current,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "metric_identity",
                "readiness_state",
                "evidence_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::AssuranceDashboard,
                M5GovernanceConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UX Design System §16.53 fitness dashboard tiles".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("fitness-dashboard-tile"),
        },
        // Governance report row — the lane's evidence aged out and is being
        // re-verified, so the row's reading auto-narrows to provisional and reads from
        // last-known evidence rather than a fresh pass (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:governance-report-row".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::GovernanceReportRow,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "report:lane:0002".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:governance-report-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "report_scope",
                "readiness_state",
                "evidence_freshness",
                "owner",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::EvidenceFreshness,
                M5GovernanceConditionState::Stale,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::Provisional,
                binding_dimension: M5GovernanceClaimDimension::EvidenceFreshness,
                trigger: M5GovernanceDowngradeTrigger::EvidenceStaleHidden,
                narrowed_label:
                    "Evidence stale — lane shown from last-known governance evidence until re-verification lands, not a fresh pass"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "report_scope",
                "readiness_state",
                "evidence_freshness",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::DocsPortal,
                M5GovernanceConsumerSurface::HelpAbout,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.41 governance reports".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("governance-report-row"),
        },
        // Waiver-expiry queue item — the exception waiver is expiring, so the item
        // auto-narrows to waiver-gated and can never read as a clean pass while the
        // waiver holds the lane (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:waiver-expiry-queue-item".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::WaiverExpiryQueueItem,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "waiver:item:0003".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:waiver-expiry-queue-item:a11y".to_owned(),
            copy_export: copy_export(&[
                "waiver_identity",
                "expiry_at",
                "readiness_state",
                "owner",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::WaiverExpiry,
                M5GovernanceConditionState::Waived,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::WaiverGated,
                binding_dimension: M5GovernanceClaimDimension::WaiverExpiry,
                trigger: M5GovernanceDowngradeTrigger::WaiverExpiryHidden,
                narrowed_label:
                    "Waiver expiring — lane held by an exception waiver and shown waiver-gated, not a clean pass, until the waiver is renewed or cleared"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "waiver_identity",
                "expiry_at",
                "readiness_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::ShiproomPacket,
                M5GovernanceConsumerSurface::OperatorBoard,
            ]),
            source_refs: vec![
                "UX Design System §16.53 waiver-expiry chips".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("waiver-expiry-queue-item"),
        },
        // Release-gate banner — the forum authorized to approve the next move is
        // unresolved, so the banner auto-narrows to blocked rather than presenting a
        // ready ship decision (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:release-gate-banner".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::ReleaseGateBanner,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "gate:banner:0004".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:release-gate-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "gate_identity",
                "gate_decision",
                "decision_forum",
                "reason",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedResolved,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::DecisionRightTruth,
                M5GovernanceConditionState::Unresolved,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::Blocked,
                binding_dimension: M5GovernanceClaimDimension::DecisionRightTruth,
                trigger: M5GovernanceDowngradeTrigger::DecisionForumMasked,
                narrowed_label:
                    "Decision forum unresolved — no authoritative forum can approve this move yet, so the gate is shown blocked, not ready to ship"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "gate_identity",
                "gate_decision",
                "decision_forum",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::ReleaseCenterUi,
                M5GovernanceConsumerSurface::ShiproomPacket,
            ]),
            source_refs: vec![
                "UX Design System §16.53 release-gate banners".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("release-gate-banner"),
        },
        // Mitigation note card — the support class is only partially stated (the
        // mitigation still leans on internal jargon a support/export consumer cannot
        // reuse), so the card auto-narrows to degraded until the plain-language
        // mitigation lands (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:mitigation-note-card".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::MitigationNoteCard,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "mitigation:card:0005".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:mitigation-note-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "mitigation_identity",
                "support_class",
                "plain_language_mitigation",
                "readiness_state",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::SupportClass,
                M5GovernanceConditionState::Partial,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::Degraded,
                binding_dimension: M5GovernanceClaimDimension::SupportClass,
                trigger: M5GovernanceDowngradeTrigger::MitigationHiddenBehindJargon,
                narrowed_label:
                    "Support class partial — mitigation shown degraded until the plain-language note replaces the internal jargon support consumers cannot reuse"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "mitigation_identity",
                "support_class",
                "plain_language_mitigation",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::HelpAbout,
                M5GovernanceConsumerSurface::DocsPortal,
            ]),
            source_refs: vec![
                "UX Design System §16.53 mitigation notes".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("mitigation-note-card"),
        },
        // Service-ownership card — the owner is covered with a backup and the on-call
        // route is stated, so the card carries a resolved, self-sufficient ownership
        // object reachable on every surface (green).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:service-ownership-card".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::ServiceOwnershipCard,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "ownership:card:0006".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:service-ownership-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "service_identity",
                "owner_coverage",
                "owner_freshness",
                "escalation_route",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedResolved,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::OwnerCoverage,
                M5GovernanceConditionState::Current,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "service_identity",
                "owner_coverage",
                "escalation_route",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::ServiceHealth,
                M5GovernanceConsumerSurface::OperatorBoard,
            ]),
            source_refs: vec![
                "UX Design System §16.54 service ownership cards".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("service-ownership-card"),
        },
        // On-call strip — the on-call owner coverage is only partial (a backup slot is
        // unfilled), so the strip auto-narrows to degraded until the rotation is covered
        // (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:on-call-strip".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::OnCallStrip,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "oncall:strip:0007".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:on-call-strip:a11y".to_owned(),
            copy_export: copy_export(&[
                "rotation_identity",
                "on_call_coverage",
                "escalation_route",
                "readiness_state",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::OwnerCoverage,
                M5GovernanceConditionState::Partial,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::Degraded,
                binding_dimension: M5GovernanceClaimDimension::OwnerCoverage,
                trigger: M5GovernanceDowngradeTrigger::OwnerCoverageOverstated,
                narrowed_label:
                    "On-call coverage partial — rotation shown degraded with an unfilled backup slot, never as a fully covered on-call route"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "rotation_identity",
                "on_call_coverage",
                "escalation_route",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::ServiceHealth,
                M5GovernanceConsumerSurface::OperatorBoard,
            ]),
            source_refs: vec![
                "UX Design System §16.54 on-call strips".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("on-call-strip"),
        },
        // Decision-right card — the authoritative forum record aged out and is being
        // re-confirmed, so the card auto-narrows to provisional and reads from
        // last-known decision-right state rather than a fresh authoritative one
        // (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:decision-right-card".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::DecisionRightCard,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "decision:card:0008".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:decision-right-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "decision_identity",
                "decision_forum",
                "decision_right_state",
                "readiness_state",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::DecisionRightTruth,
                M5GovernanceConditionState::Stale,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::Provisional,
                binding_dimension: M5GovernanceClaimDimension::DecisionRightTruth,
                trigger: M5GovernanceDowngradeTrigger::DecisionForumMasked,
                narrowed_label:
                    "Decision-right record stale — forum authority shown from last-known state until re-confirmation lands, not a fresh authoritative reading"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "decision_identity",
                "decision_forum",
                "decision_right_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::ShiproomPacket,
                M5GovernanceConsumerSurface::ReleaseCenterUi,
            ]),
            source_refs: vec![
                "TDD §10.3 decision forums".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("decision-right-card"),
        },
        // Milestone dashboard row — hierarchy-heavy (exit-gate tree with per-gate
        // blocker / waiver sub-rows); the forum authorized to clear the milestone's next
        // gate is unresolved, so the row auto-narrows to blocked and binds its gate tree
        // to a flat list / textual path (yellow).
        GovernanceAccessibilityRow {
            record_kind: GOVERNANCE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: GOVERNANCE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:milestone-dashboard-row".to_owned(),
            component_family: M5GovernanceDashboardComponentFamily::MilestoneDashboardRow,
            source_family_schema_ref: GOVERNANCE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            governance_context_ref: "milestone:row:0009".to_owned(),
            fallback_modalities: vec![
                M5GovernanceFallbackModality::Structured,
                M5GovernanceFallbackModality::List,
                M5GovernanceFallbackModality::Textual,
                M5GovernanceFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: GovernanceNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: GovernanceNonVisualReachState::ReachableAndLabeled,
            export_summary: GovernanceExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:milestone-dashboard-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "milestone_identity",
                "exit_gate_state",
                "decision_forum",
                "blocker_waiver_counts",
            ]),
            full_support_claim: M5GovernanceSupportClaim::GovernedPass,
            claim_conditions: vec![condition(
                M5GovernanceClaimDimension::DecisionRightTruth,
                M5GovernanceConditionState::Unresolved,
            )],
            claim_narrow: Some(GovernanceClaimAutoNarrow {
                narrowed_to: M5GovernanceSupportClaim::Blocked,
                binding_dimension: M5GovernanceClaimDimension::DecisionRightTruth,
                trigger: M5GovernanceDowngradeTrigger::DecisionForumMasked,
                narrowed_label:
                    "Milestone gate forum unresolved — no authoritative forum can clear the next exit gate, so the milestone is shown blocked, not on-track"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "milestone_identity",
                "exit_gate_state",
                "blocker_waiver_counts",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5GovernanceConsumerSurface::OperatorBoard,
                M5GovernanceConsumerSurface::ReleaseCenterUi,
            ]),
            source_refs: vec![
                "UX Design System §16.54 milestone dashboard rows".to_owned(),
                GOVERNANCE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("milestone-dashboard-row"),
        },
    ]
}
