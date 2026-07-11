//! Keyboard / screen-reader / reduced-motion / high-contrast / CLI / export /
//! support-packet parity and honest auto-narrowing for the M5 adaptive-efficiency
//! components.
//!
//! This module is the M05-1065 accessibility-and-auto-narrowing capstone over the
//! frozen M5 adaptive-efficiency component matrix
//! ([`crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix`]).
//! Where the freeze matrix defines the reusable power-state indicator, throttled-subsystem
//! row, background-work row / banner, per-workspace override sheet, override-policy note
//! row, resume-summary card, and stale-result continuity note primitives, and the
//! 1061-1064 implementation lanes resolve their per-surface truth, this lane certifies —
//! per component family — that adaptive-efficiency claims stay **keyboard-complete,
//! screen-reader-reachable, reduced-motion safe, high-contrast legible, CLI/export-safe,
//! and self-narrowing** rather than presenting a stale, partial, deferred, or
//! policy-blocked efficiency state as still `full-truth`:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path into
//!   the same source-of-change, active efficiency state, slowed-versus-paused work,
//!   override availability, policy owner, resumed-work backlog, and stale-result
//!   continuity the rich surface shows — never a hover-only or toast-only card that
//!   strands assistive-tech or headless users. Hierarchy-heavy families (the
//!   per-workspace override sheet's nested current-mode / allowed-ceiling tree)
//!   additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the same
//!   pressure sources, work dispositions, override states, and stale-result continuity
//!   shown in-product.
//! - **Honest auto-narrowing.** When the pressure source, work disposition, override
//!   availability, policy owner, resumed-work backlog, or stale-result continuity
//!   becomes partial, deferred, stale-shown, or policy-blocked, the component's
//!   efficiency-support claim auto-narrows from `full-truth` / `resolved-truth` to
//!   degraded / deferred / stale-shown / policy-blocked, discloses the narrowing with a
//!   precise frozen trigger and binding dimension, and preserves the canonical
//!   source / subsystem / override / continuity identity rather than silently dropping
//!   it. A component with every dimension intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in shell status
//!   chrome, the activity center, docs/help, headless CLI, and support/admin exports so
//!   claim publication and field triage stay aligned on adaptive-efficiency downgrade
//!   behavior.
//!
//! Each [`EfficiencyAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::M5EfficiencyComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5EfficiencyRequiredLabel`] and [`M5EfficiencyDowngradeTrigger`] and the shared
//! [`M5EfficiencyConsumerSurface`] consumer surfaces rather than minting parallel
//! synonyms, so the certified labels stay byte-identical to the matrix and the sibling
//! primitive packets.
//!
//! The packet is metadata-only: raw battery/thermal telemetry, workspace secrets, and
//! scheduler cursors never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, and redacted labels so support and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have
//! shown without leaking runtime state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-efficiency-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-efficiency-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/help/m5_efficiency_component_accessibility_parity.md`](../../../../docs/help/m5_efficiency_component_accessibility_parity.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, and downgrade triggers rather than mint parallel ones.
use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix::{
    M5EfficiencyComponentFamily, M5EfficiencyConsumerSurface, M5EfficiencyDowngradeTrigger,
    M5EfficiencyRequiredLabel, M5_EFFICIENCY_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1065 efficiency-component accessibility parity
/// packet.
pub const EFFICIENCY_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EfficiencyAccessibilityPacket`].
pub const EFFICIENCY_A11Y_RECORD_KIND: &str = "m5_efficiency_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`EfficiencyAccessibilityRow`].
pub const EFFICIENCY_A11Y_ROW_RECORD_KIND: &str =
    "m5_efficiency_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const EFFICIENCY_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-efficiency-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const EFFICIENCY_A11Y_DOC_REF: &str =
    "docs/help/m5_efficiency_component_accessibility_parity.md";

/// Repo-relative path of the frozen adaptive-efficiency component matrix this lane
/// certifies.
pub const EFFICIENCY_A11Y_COMPONENT_MATRIX_REF: &str = M5_EFFICIENCY_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const EFFICIENCY_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-efficiency-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EFFICIENCY_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-efficiency-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EFFICIENCY_A11Y_CSV_REF: &str =
    "artifacts/release/m5-efficiency-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EFFICIENCY_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-efficiency-component-accessibility-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the per-workspace
/// override sheet's nested current-mode / allowed-ceiling tree) and therefore MUST bind
/// their tree to an equivalent flat list / textual path so the hierarchy is navigable
/// non-visually.
const fn family_is_hierarchy_heavy(family: M5EfficiencyComponentFamily) -> bool {
    matches!(
        family,
        M5EfficiencyComponentFamily::PerWorkspaceOverrideSheet
    )
}

/// The adaptive-efficiency dimension whose weakening a family primarily discloses. Every
/// row must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5EfficiencyComponentFamily,
) -> M5EfficiencyClaimDimension {
    match family {
        M5EfficiencyComponentFamily::PowerStateIndicator => {
            M5EfficiencyClaimDimension::PressureSourceTruth
        }
        M5EfficiencyComponentFamily::ThrottledSubsystemRow
        | M5EfficiencyComponentFamily::BackgroundWorkRow
        | M5EfficiencyComponentFamily::BackgroundWorkBanner => {
            M5EfficiencyClaimDimension::WorkDispositionTruth
        }
        M5EfficiencyComponentFamily::PerWorkspaceOverrideSheet => {
            M5EfficiencyClaimDimension::OverrideAvailabilityTruth
        }
        M5EfficiencyComponentFamily::OverridePolicyNoteRow => {
            M5EfficiencyClaimDimension::PolicyOwnerTruth
        }
        M5EfficiencyComponentFamily::ResumeSummaryCard => {
            M5EfficiencyClaimDimension::ResumeBacklogTruth
        }
        M5EfficiencyComponentFamily::StaleResultContinuityNote => {
            M5EfficiencyClaimDimension::StaleResultContinuityTruth
        }
    }
}

/// A rendered fallback modality for an adaptive-efficiency component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyFallbackModality {
    /// A rich, structured (override ceiling tree / grouped inventory) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5EfficiencyFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the
/// same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyRenderingSurface {
    /// The full-capability desktop shell surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin export.
    SupportExport,
}

impl M5EfficiencyRenderingSurface {
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
pub enum EfficiencyNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / toast-only surface that traps keyboard / assistive-tech
    /// / headless users (red).
    ViewOnlyTrap,
}

impl EfficiencyNonVisualReachState {
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
pub enum EfficiencyExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl EfficiencyExportSummaryState {
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
pub enum EfficiencyNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl EfficiencyNarrowingDisclosureState {
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

/// The efficiency-support claim ceiling a component asserts: how strong an
/// adaptive-efficiency posture it lets a surface present. Auto-narrowing lowers this
/// ceiling when an efficiency dimension weakens so a stale, partial, deferred, or
/// policy-blocked efficiency state can never keep an old `full-truth` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyAccessClaim {
    /// Full-truth: the component's live source-of-change / state / disposition truth is
    /// fully reachable and current — the strongest claim.
    FullTruth,
    /// Resolved-truth: a resolved, self-sufficient efficiency posture that is not itself
    /// a live-adapting stream (e.g. a fully-shown aggregate banner or policy note).
    ResolvedTruth,
    /// Degraded: usable, but with a disclosed reduction in scope or confidence.
    Degraded,
    /// Deferred: the work is paused / deferred and shown from its last-known backlog,
    /// not live.
    Deferred,
    /// Stale-shown: a stale result is deliberately kept visible pending refresh, not a
    /// live current value.
    StaleShown,
    /// Policy-blocked: a required entitlement / policy dependency blocks the override or
    /// adaptation.
    PolicyBlocked,
}

impl M5EfficiencyAccessClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::FullTruth,
        Self::ResolvedTruth,
        Self::Degraded,
        Self::Deferred,
        Self::StaleShown,
        Self::PolicyBlocked,
    ];

    /// Capability rank; a higher rank asserts a stronger efficiency posture. Narrowing
    /// lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullTruth => 5,
            Self::ResolvedTruth => 4,
            Self::Degraded => 3,
            Self::Deferred => 2,
            Self::StaleShown => 1,
            Self::PolicyBlocked => 0,
        }
    }

    /// Returns true when this claim asserts live, current, actively-adapting truth.
    pub const fn asserts_live_truth(self) -> bool {
        matches!(self, Self::FullTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (live or resolved /
    /// current) posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::FullTruth | Self::ResolvedTruth)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTruth => "full_truth",
            Self::ResolvedTruth => "resolved_truth",
            Self::Degraded => "degraded",
            Self::Deferred => "deferred",
            Self::StaleShown => "stale_shown",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The adaptive-efficiency dimension whose state governs how far a component may claim
/// full, current efficiency truth. These are exactly the six axes the spec requires
/// auto-narrowing on: source-of-change / pressure, work disposition, override
/// availability, policy owner, resumed-work backlog, and stale-result continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyClaimDimension {
    /// Pressure-source truth: is the source of change and active efficiency state
    /// resolved and current?
    PressureSourceTruth,
    /// Work-disposition truth: is the subsystem / job's slowed-versus-paused state and
    /// what-still-works resolved without ambiguity?
    WorkDispositionTruth,
    /// Override-availability truth: is the override genuinely available, or blocked by
    /// policy?
    OverrideAvailabilityTruth,
    /// Policy-owner truth: is the policy owner behind the adaptation named and current?
    PolicyOwnerTruth,
    /// Resume-backlog truth: is the resumed-work backlog size known, or hidden?
    ResumeBacklogTruth,
    /// Stale-result-continuity truth: is the stale-result continuity preserved after
    /// resume, or cleared?
    StaleResultContinuityTruth,
}

impl M5EfficiencyClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PressureSourceTruth,
        Self::WorkDispositionTruth,
        Self::OverrideAvailabilityTruth,
        Self::PolicyOwnerTruth,
        Self::ResumeBacklogTruth,
        Self::StaleResultContinuityTruth,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5EfficiencyDowngradeTrigger {
        match self {
            Self::PressureSourceTruth => M5EfficiencyDowngradeTrigger::SourceOfChangeUnstated,
            Self::WorkDispositionTruth => M5EfficiencyDowngradeTrigger::SlowedVersusPausedAmbiguous,
            Self::OverrideAvailabilityTruth => {
                M5EfficiencyDowngradeTrigger::OverrideAvailabilityUnstated
            }
            Self::PolicyOwnerTruth => M5EfficiencyDowngradeTrigger::PolicyOwnerUnstated,
            Self::ResumeBacklogTruth => M5EfficiencyDowngradeTrigger::ResumeBacklogHidden,
            Self::StaleResultContinuityTruth => {
                M5EfficiencyDowngradeTrigger::StaleResultContinuityCleared
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PressureSourceTruth => "pressure_source_truth",
            Self::WorkDispositionTruth => "work_disposition_truth",
            Self::OverrideAvailabilityTruth => "override_availability_truth",
            Self::PolicyOwnerTruth => "policy_owner_truth",
            Self::ResumeBacklogTruth => "resume_backlog_truth",
            Self::StaleResultContinuityTruth => "stale_result_continuity_truth",
        }
    }
}

/// The observed condition of one adaptive-efficiency dimension. Anything weaker than
/// [`Self::Intact`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyConditionState {
    /// Fully verified / current / live — imposes no ceiling.
    Intact,
    /// Partially resolved — scope or confidence is reduced; support drops to degraded.
    Partial,
    /// Deferred — the work is paused and shown from its last-known backlog; support drops
    /// to deferred.
    Deferred,
    /// Stale-shown — a stale result is deliberately kept visible pending refresh; support
    /// drops to stale-shown.
    StaleShown,
    /// Policy-blocked — a required entitlement / policy dependency is unmet; support drops
    /// to policy-blocked.
    PolicyBlocked,
}

impl M5EfficiencyConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Intact,
        Self::Partial,
        Self::Deferred,
        Self::StaleShown,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest efficiency-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5EfficiencyAccessClaim {
        match self {
            Self::Intact => M5EfficiencyAccessClaim::FullTruth,
            Self::Partial => M5EfficiencyAccessClaim::Degraded,
            Self::Deferred => M5EfficiencyAccessClaim::Deferred,
            Self::StaleShown => M5EfficiencyAccessClaim::StaleShown,
            Self::PolicyBlocked => M5EfficiencyAccessClaim::PolicyBlocked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Partial => "partial",
            Self::Deferred => "deferred",
            Self::StaleShown => "stale_shown",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One adaptive-efficiency dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5EfficiencyClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5EfficiencyConditionState,
}

/// An honest efficiency-support-claim auto-narrow block. When an efficiency dimension
/// weakens, the component's support claim lowers to the permitted ceiling, names the
/// binding dimension and frozen trigger, and preserves the canonical source / subsystem
/// / override / continuity identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5EfficiencyAccessClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5EfficiencyClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5EfficiencyDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical source of change, affected subsystem, override state, and
    /// stale-result continuity are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl EfficiencyClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl EfficiencyCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as the
    /// sole export.
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
pub struct EfficiencyRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5EfficiencyRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: EfficiencyNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an efficiency-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims support, or drops state
    /// silently (red).
    Stranded,
}

impl EfficiencyAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one adaptive-efficiency component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyAccessibilityRow {
    /// Record kind; must equal [`EFFICIENCY_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EFFICIENCY_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5EfficiencyComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the workspace / subsystem / job context this component acts on;
    /// stays visible on every surface, so this is never empty.
    pub efficiency_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5EfficiencyFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical source of change, active
    /// state, work disposition, override state, and continuity as the rich surface; must
    /// hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: EfficiencyNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: EfficiencyNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: EfficiencyNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: EfficiencyExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: EfficiencyCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5EfficiencyAccessClaim,
    /// The observed condition of each modeled efficiency dimension.
    #[serde(default)]
    pub claim_conditions: Vec<EfficiencyClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<EfficiencyClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5EfficiencyRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<EfficiencyRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5EfficiencyRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5EfficiencyConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl EfficiencyAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a
    /// flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality
    /// is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5EfficiencyClaimDimension,
    ) -> M5EfficiencyConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5EfficiencyConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's
    /// ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5EfficiencyAccessClaim {
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
    pub fn binding_dimension(&self) -> Option<M5EfficiencyClaimDimension> {
        let mut binding: Option<(M5EfficiencyClaimDimension, u8)> = None;
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
    pub fn effective_claim(&self) -> M5EfficiencyAccessClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC1 / auto-narrowing honesty: a stale, partial, deferred, or policy-blocked
    /// efficiency state can no longer keep an old `full-truth` / `resolved-truth` label.
    /// The effective claim never exceeds the permitted ceiling; when a dimension narrows
    /// below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity. When nothing narrows, no spurious
    /// narrow block is present.
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

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same
    /// canonical truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family
    /// offers a non-visual fallback, and the export reconstructs meaning without a
    /// screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.efficiency_context_ref.trim().is_empty()
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

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its
    /// reduced interactivity and keeps its labels, so claim publication and field triage
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
        M5EfficiencyRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> EfficiencyAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return EfficiencyAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            EfficiencyAccessibilityStatus::NarrowedDisclosed
        } else {
            EfficiencyAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EFFICIENCY_A11Y_ROW_RECORD_KIND
            && self.schema_version == EFFICIENCY_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.efficiency_context_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-1065 efficiency-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyAccessibilitySummary {
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

/// Constructor input for [`EfficiencyAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EfficiencyAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<EfficiencyAccessibilityRow>,
}

/// Checked-in M05-1065 efficiency-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<EfficiencyAccessibilityRow>,
    pub summary: EfficiencyAccessibilitySummary,
}

impl EfficiencyAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: EfficiencyAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            record_kind: EFFICIENCY_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: EfficiencyAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5EfficiencyComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5EfficiencyClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5EfficiencyAccessClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> HashSet<M5EfficiencyConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EfficiencyAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: HashSet<M5EfficiencyConsumerSurface> = HashSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&EfficiencyAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                EfficiencyAccessibilityStatus::Parity => green += 1,
                EfficiencyAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                EfficiencyAccessibilityStatus::Stranded => red += 1,
            }
        }

        EfficiencyAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(EfficiencyAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(EfficiencyAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(EfficiencyAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(EfficiencyAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EfficiencyAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EFFICIENCY_A11Y_SCHEMA_VERSION {
            violations.push(EfficiencyAccessibilityViolation::SchemaVersion {
                expected: EFFICIENCY_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EFFICIENCY_A11Y_RECORD_KIND {
            violations.push(EfficiencyAccessibilityViolation::RecordKind {
                expected: EFFICIENCY_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EfficiencyAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EfficiencyAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(EfficiencyAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(EfficiencyAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory efficiency-component label.
            if !row.preserves_mandatory_labels() {
                violations.push(EfficiencyAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5EfficiencyFallbackModality::Structured)
            {
                violations.push(
                    EfficiencyAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts full efficiency truth for a weakened one.
            if !row.claim_is_honest() {
                violations.push(EfficiencyAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(EfficiencyAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(EfficiencyAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    EfficiencyAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(EfficiencyAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == EfficiencyAccessibilityStatus::Stranded {
                violations.push(EfficiencyAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5EfficiencyComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(EfficiencyAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5EfficiencyClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(EfficiencyAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (full-truth → … → policy-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5EfficiencyAccessClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(EfficiencyAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Cross-surface: the same narrowed state must reach shell status chrome, the
        // activity center, docs/help, and support/admin exports — so every consumer
        // surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5_EFFICIENCY_A11Y_CONSUMER_SURFACES {
            if !consumers.contains(&surface) {
                violations.push(
                    EfficiencyAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(EfficiencyAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("efficiency accessibility parity packet serializes"),
        ) {
            violations.push(EfficiencyAccessibilityViolation::RawEfficiencyMaterialInExport);
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
            .expect("efficiency accessibility parity packet serializes")
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
        out.push_str("# M5 Adaptive-Efficiency Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5EfficiencyComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in efficiency-component accessibility parity export.
pub fn current_m5_efficiency_a11y_export(
) -> Result<EfficiencyAccessibilityPacket, EfficiencyAccessibilityArtifactError> {
    let packet: EfficiencyAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-efficiency-component-accessibility-proof/support_export.json"
    )))
    .map_err(EfficiencyAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EfficiencyAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in efficiency-component accessibility parity
/// export.
#[derive(Debug)]
pub enum EfficiencyAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EfficiencyAccessibilityViolation>),
}

impl fmt::Display for EfficiencyAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "efficiency accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "efficiency accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for EfficiencyAccessibilityArtifactError {}

/// The consumer surfaces this lane requires the packet to exercise. The full
/// [`M5EfficiencyConsumerSurface`] set — the support export and diagnostics field-triage
/// surfaces plus the shell-status / activity-center / background-work / override-settings
/// / help / product surfaces where an adaptive-efficiency component is embedded.
pub const M5_EFFICIENCY_A11Y_CONSUMER_SURFACES: [M5EfficiencyConsumerSurface; 8] =
    M5EfficiencyConsumerSurface::ALL;

/// Validation failure for M05-1065 efficiency-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EfficiencyAccessibilityViolation {
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
        dimension: M5EfficiencyClaimDimension,
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
        family: M5EfficiencyComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5EfficiencyClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5EfficiencyAccessClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5EfficiencyConsumerSurface,
    },
    SummaryMismatch,
    RawEfficiencyMaterialInExport,
}

impl fmt::Display for EfficiencyAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory efficiency-component label")
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
                    "row {id} over-asserts full efficiency truth for a weakened one, or narrows spuriously"
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
            Self::RawEfficiencyMaterialInExport => {
                write!(f, "export contains raw efficiency material")
            }
        }
    }
}

impl Error for EfficiencyAccessibilityViolation {}

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
            | "low power"
            | "power saver"
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

/// Builds the canonical, checked-in efficiency-component accessibility parity packet. This
/// is the one source of truth shared by the tests, the artifact writer, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_efficiency_a11y_packet() -> EfficiencyAccessibilityPacket {
    EfficiencyAccessibilityPacket::new(EfficiencyAccessibilityPacketInput {
        packet_id: "m5-efficiency-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:efficiency-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5EfficiencyRequiredLabel> {
    M5EfficiencyRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> EfficiencyCopyExportParity {
    EfficiencyCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5EfficiencyClaimDimension,
    state: M5EfficiencyConditionState,
) -> EfficiencyClaimConditionEntry {
    EfficiencyClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support/export replay and
/// the diagnostics UI — so the narrowed state always reaches field triage.
fn base_consumers(extra: &[M5EfficiencyConsumerSurface]) -> Vec<M5EfficiencyConsumerSurface> {
    let mut out = vec![
        M5EfficiencyConsumerSurface::SupportExport,
        M5EfficiencyConsumerSurface::DiagnosticsUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: EfficiencyNarrowingDisclosureState,
) -> Vec<EfficiencyRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        EfficiencyRenderingNarrowingDisclosure {
            rendering_surface: M5EfficiencyRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        EfficiencyRenderingNarrowingDisclosure {
            rendering_surface: M5EfficiencyRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label
/// and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<EfficiencyRenderingNarrowingDisclosure> {
    surface_disclosures(labels, EfficiencyNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<EfficiencyRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        EfficiencyNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5EfficiencyRenderingSurface> {
    vec![
        M5EfficiencyRenderingSurface::DesktopFull,
        M5EfficiencyRenderingSurface::CliHeadless,
        M5EfficiencyRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<EfficiencyAccessibilityRow> {
    vec![
        // Power-state indicator — the source of change (battery saver) and active
        // efficiency state are resolved and current; the indicator offers a fully live,
        // authoritative power-state truth reachable on every surface (green).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:power-state-indicator".to_owned(),
            component_family: M5EfficiencyComponentFamily::PowerStateIndicator,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:power-state:0001".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:power-state-indicator:a11y".to_owned(),
            copy_export: copy_export(&[
                "source_of_change",
                "efficiency_state",
                "what_still_works",
                "inspect_path",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::PressureSourceTruth,
                M5EfficiencyConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "source_of_change",
                "efficiency_state",
                "what_still_works",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EfficiencyConsumerSurface::ShellStatusUi,
                M5EfficiencyConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §9.13 power/thermal/battery-adaptive behavior".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("power-state-indicator"),
        },
        // Background-work banner — the aggregate paused/slowed work is fully resolved and
        // shown explicitly (never toast-only); the banner reports a ready,
        // self-sufficient aggregate disposition (green).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:background-work-banner".to_owned(),
            component_family: M5EfficiencyComponentFamily::BackgroundWorkBanner,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:background-banner:0002".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:background-work-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "affected_work_class",
                "slowed_versus_paused",
                "what_still_works",
                "resume_condition",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::WorkDispositionTruth,
                M5EfficiencyConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "affected_work_class",
                "slowed_versus_paused",
                "what_still_works",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EfficiencyConsumerSurface::BackgroundWorkUi,
                M5EfficiencyConsumerSurface::ActivityCenterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §5.7 background-work honesty".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("background-work-banner"),
        },
        // Throttled-subsystem row — the subsystem's slowed-versus-paused work is only
        // partially resolved (indexing throttle scope still settling), so the disposition
        // claim auto-narrows to degraded rather than reading as full truth (yellow).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:throttled-subsystem-row".to_owned(),
            component_family: M5EfficiencyComponentFamily::ThrottledSubsystemRow,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:throttled-subsystem:0003".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:throttled-subsystem-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "affected_subsystem",
                "slowed_versus_paused",
                "what_still_works",
                "inspect_path",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::WorkDispositionTruth,
                M5EfficiencyConditionState::Partial,
            )],
            claim_narrow: Some(EfficiencyClaimAutoNarrow {
                narrowed_to: M5EfficiencyAccessClaim::Degraded,
                binding_dimension: M5EfficiencyClaimDimension::WorkDispositionTruth,
                trigger: M5EfficiencyDowngradeTrigger::SlowedVersusPausedAmbiguous,
                narrowed_label:
                    "Throttle scope partially resolved — indexing shown degraded until the slowed-versus-paused split settles"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "affected_subsystem",
                "slowed_versus_paused",
                "what_still_works",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EfficiencyConsumerSurface::ShellStatusUi]),
            source_refs: vec![
                "TAD §8.7 power/thermal/battery-efficiency architecture".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("throttled-subsystem-row"),
        },
        // Background-work row — the job is paused / deferred and shown from its last-known
        // backlog, so the disposition claim auto-narrows to deferred rather than reading
        // as live progressing work (yellow).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:background-work-row".to_owned(),
            component_family: M5EfficiencyComponentFamily::BackgroundWorkRow,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:background-row:0004".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:background-work-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "affected_work_class",
                "slowed_versus_paused",
                "resume_condition",
                "override_availability",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::WorkDispositionTruth,
                M5EfficiencyConditionState::Deferred,
            )],
            claim_narrow: Some(EfficiencyClaimAutoNarrow {
                narrowed_to: M5EfficiencyAccessClaim::Deferred,
                binding_dimension: M5EfficiencyClaimDimension::WorkDispositionTruth,
                trigger: M5EfficiencyDowngradeTrigger::SlowedVersusPausedAmbiguous,
                narrowed_label:
                    "Job paused — shown from last-known backlog, not live progress, until pressure clears"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "affected_work_class",
                "slowed_versus_paused",
                "resume_condition",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EfficiencyConsumerSurface::BackgroundWorkUi]),
            source_refs: vec![
                "TAD §8.8 background-work queue/fairness".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("background-work-row"),
        },
        // Per-workspace override sheet — hierarchy-heavy (nested current-mode / allowed
        // ceiling tree); the override is blocked by an admin policy cap, so the sheet
        // auto-narrows to policy-blocked rather than presenting a live "Override now" and
        // binds its tree to a flat list / textual path (yellow).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:per-workspace-override-sheet".to_owned(),
            component_family: M5EfficiencyComponentFamily::PerWorkspaceOverrideSheet,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:override-sheet:0005".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::Structured,
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:per-workspace-override-sheet:a11y".to_owned(),
            copy_export: copy_export(&[
                "current_mode",
                "allowed_ceilings",
                "expected_effect",
                "reset_path",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::OverrideAvailabilityTruth,
                M5EfficiencyConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(EfficiencyClaimAutoNarrow {
                narrowed_to: M5EfficiencyAccessClaim::PolicyBlocked,
                binding_dimension: M5EfficiencyClaimDimension::OverrideAvailabilityTruth,
                trigger: M5EfficiencyDowngradeTrigger::OverrideAvailabilityUnstated,
                narrowed_label:
                    "Override blocked by policy — shown as blocked-by-policy, not available, until the admin cap lifts"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "current_mode",
                "allowed_ceilings",
                "reset_path",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EfficiencyConsumerSurface::OverrideSettingsUi]),
            source_refs: vec![
                "UI/UX Spec §5.7 power-thermal adaptation".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("per-workspace-override-sheet"),
        },
        // Override-policy note row — the policy owner behind the adaptation is only
        // partially resolved (owner attribution still resolving), so the note auto-narrows
        // to degraded rather than reading as a fully-attributed policy owner (yellow).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:override-policy-note-row".to_owned(),
            component_family: M5EfficiencyComponentFamily::OverridePolicyNoteRow,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:policy-note:0006".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:override-policy-note-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "policy_owner",
                "block_reason",
                "local_changeability",
                "expected_effect",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::PolicyOwnerTruth,
                M5EfficiencyConditionState::Partial,
            )],
            claim_narrow: Some(EfficiencyClaimAutoNarrow {
                narrowed_to: M5EfficiencyAccessClaim::Degraded,
                binding_dimension: M5EfficiencyClaimDimension::PolicyOwnerTruth,
                trigger: M5EfficiencyDowngradeTrigger::PolicyOwnerUnstated,
                narrowed_label:
                    "Policy owner partially resolved — attribution shown degraded until the owning policy resolves"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "policy_owner",
                "block_reason",
                "local_changeability",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EfficiencyConsumerSurface::OverrideSettingsUi,
                M5EfficiencyConsumerSurface::HelpAboutUi,
            ]),
            source_refs: vec![
                "UX Design System §16.18 job rows and activity centers".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("override-policy-note-row"),
        },
        // Resume-summary card — the resumed-work backlog is being replayed from its
        // deferred queue, so the card auto-narrows to deferred rather than reading as a
        // fully-current resumed workload (yellow).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:resume-summary-card".to_owned(),
            component_family: M5EfficiencyComponentFamily::ResumeSummaryCard,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:resume-summary:0007".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:resume-summary-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "resumed_work",
                "remaining_backlog",
                "stale_results_visible",
                "next_safe_action",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::ResumeBacklogTruth,
                M5EfficiencyConditionState::Deferred,
            )],
            claim_narrow: Some(EfficiencyClaimAutoNarrow {
                narrowed_to: M5EfficiencyAccessClaim::Deferred,
                binding_dimension: M5EfficiencyClaimDimension::ResumeBacklogTruth,
                trigger: M5EfficiencyDowngradeTrigger::ResumeBacklogHidden,
                narrowed_label:
                    "Resume in progress — remaining backlog shown as still-deferred, not yet fully caught up"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "resumed_work",
                "remaining_backlog",
                "next_safe_action",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5EfficiencyConsumerSurface::ActivityCenterUi]),
            source_refs: vec![
                "Milestones v3.1 durable progress/activity-center truth".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("resume-summary-card"),
        },
        // Stale-result continuity note — a stale result is deliberately kept visible
        // pending refresh, so the note auto-narrows to stale-shown rather than reading as
        // a live current value, and never clears the stale-result context on resume
        // (yellow).
        EfficiencyAccessibilityRow {
            record_kind: EFFICIENCY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:stale-result-continuity-note".to_owned(),
            component_family: M5EfficiencyComponentFamily::StaleResultContinuityNote,
            source_family_schema_ref: EFFICIENCY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            efficiency_context_ref: "efficiency:stale-continuity:0008".to_owned(),
            fallback_modalities: vec![
                M5EfficiencyFallbackModality::List,
                M5EfficiencyFallbackModality::Textual,
                M5EfficiencyFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            cli_reach: EfficiencyNonVisualReachState::ReachableAndLabeled,
            export_summary: EfficiencyExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:stale-result-continuity-note:a11y".to_owned(),
            copy_export: copy_export(&[
                "stale_result_state",
                "based_on_prior_state",
                "refresh_condition",
                "next_safe_action",
            ]),
            full_support_claim: M5EfficiencyAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5EfficiencyClaimDimension::StaleResultContinuityTruth,
                M5EfficiencyConditionState::StaleShown,
            )],
            claim_narrow: Some(EfficiencyClaimAutoNarrow {
                narrowed_to: M5EfficiencyAccessClaim::StaleShown,
                binding_dimension: M5EfficiencyClaimDimension::StaleResultContinuityTruth,
                trigger: M5EfficiencyDowngradeTrigger::StaleResultContinuityCleared,
                narrowed_label:
                    "Stale result kept visible — based on a prior constrained state, not cleared on resume, pending refresh"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "stale_result_state",
                "based_on_prior_state",
                "next_safe_action",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EfficiencyConsumerSurface::ActivityCenterUi,
                M5EfficiencyConsumerSurface::HelpAboutUi,
            ]),
            source_refs: vec![
                "TDD §7.1.12 notification, attention, and activity-center architecture".to_owned(),
                EFFICIENCY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-10T00:00:00Z".to_owned(),
            evidence_refs: ev("stale-result-continuity-note"),
        },
    ]
}
