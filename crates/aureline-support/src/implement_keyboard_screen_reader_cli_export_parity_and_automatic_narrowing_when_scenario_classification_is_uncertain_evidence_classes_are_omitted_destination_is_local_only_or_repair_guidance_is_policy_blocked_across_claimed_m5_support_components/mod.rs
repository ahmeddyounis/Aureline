//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 support-intake / escalation components.
//!
//! This module is the M05-906 accessibility-and-auto-narrowing capstone over the frozen
//! M5 support-intake / escalation component matrix
//! ([`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`]).
//! Where the freeze matrix defines the reusable support-scenario picker row, issue-report
//! builder step, escalation-packet summary, handoff-timeline row, and unsafe-fix blocked-note
//! primitives, and the 901-905 implementation / consumer lanes resolve their per-surface
//! truth, this lane certifies — per component family — that support-intake and escalation
//! claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
//! self-narrowing** rather than presenting an uncertain scenario classification, an
//! evidence-omitted report, a local-only destination, or a policy-blocked repair as a
//! still ready-to-escalate case:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same scenario family,
//!   incident scope, selected and omitted evidence classes, Doctor finding lineage, approved
//!   repair class, packet destination, redaction state, and next human step the rich component
//!   shows — never a hover-only chip that strands assistive-tech or headless users. Hierarchy-
//!   heavy families (the escalation-packet summary's nested finding / repair / evidence
//!   lineage) additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot, preserving the
//!   same stable scenario codes, data-class labels, packet IDs, redaction state, and narrowing
//!   reasons shown in-product so scenario / evidence / packet truth can be reconstructed
//!   without screenshots or private team memory.
//! - **Honest auto-narrowing.** When scenario classification is uncertain, evidence classes
//!   are omitted, a destination is local-only, or repair guidance is policy-blocked, the
//!   component's support claim auto-narrows from `ReadyToEscalate` / `ReviewableCase` to an
//!   unclassified-scenario / evidence-incomplete / local-only-diagnosis / policy-blocked-repair
//!   case, discloses the narrowing with a precise trigger and binding dimension, and preserves
//!   the canonical scenario / finding / packet / redaction / repair lineage — the underlying
//!   case lineage is never dropped opaquely. A component with every dimension intact must NOT
//!   carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the Doctor UI, support
//!   center, report builder, escalation desk, recovery center, Help center, headless CLI, and
//!   support / release exports so product, docs, and release publication stay aligned on
//!   support-intake / escalation downgrade behavior rather than drifting in copy — a
//!   ready-looking case can never outrun the scenario / evidence / destination / repair proof
//!   it is being viewed away from.
//!
//! Each [`SupportIntakeComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::M5SupportIntakeEscalationComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5SupportRequiredLabel`] and
//! [`M5SupportDowngradeTrigger`] and the shared [`M5SupportConsumerSurface`] consumer surfaces
//! rather than minting parallel synonyms, so the certified labels stay byte-identical to the
//! matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw logs, transcripts, attachment bytes, and credential-bearing
//! material never cross this boundary; the packet carries only typed class tokens, opaque
//! summary / evidence refs, booleans, and redacted labels so support, release, and diagnostics
//! exports can reconstruct exactly what an accessible fallback would have shown without leaking
//! support material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5SupportConsumerSurface, M5SupportDowngradeTrigger, M5SupportIntakeEscalationComponentFamily,
    M5SupportRequiredLabel,
};

/// Schema version stamped on the M05-906 support-intake / escalation component accessibility
/// fallback packet.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`SupportIntakeComponentAccessibilityPacket`].
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_support_intake_escalation_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`SupportIntakeComponentAccessibilityRow`].
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_support_intake_escalation_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/support/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_scenario_classification_is_uncertain_evidence_classes_are_omitted_destination_is_local_only_or_repair_guidance_is_policy_blocked_across_claimed_m5_support_components.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix this lane
/// certifies.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-intake-escalation-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the escalation-packet
/// summary's nested finding / repair / evidence lineage) and therefore MUST bind their tree to
/// an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5SupportIntakeEscalationComponentFamily) -> bool {
    matches!(
        family,
        M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary
    )
}

/// The support dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5SupportIntakeEscalationComponentFamily,
) -> M5SupportIntakeClaimDimension {
    match family {
        M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow => {
            M5SupportIntakeClaimDimension::ScenarioClassification
        }
        M5SupportIntakeEscalationComponentFamily::IssueReportBuilderStep => {
            M5SupportIntakeClaimDimension::EvidenceCompleteness
        }
        M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary => {
            M5SupportIntakeClaimDimension::DestinationReach
        }
        M5SupportIntakeEscalationComponentFamily::HandoffTimelineRow => {
            M5SupportIntakeClaimDimension::HandoffContinuity
        }
        M5SupportIntakeEscalationComponentFamily::UnsafeFixBlockedNote => {
            M5SupportIntakeClaimDimension::RepairGuidance
        }
    }
}

/// A rendered fallback modality for a support-intake / escalation component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeFallbackModality {
    /// A rich, structured (nested packet / evidence tree) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5SupportIntakeFallbackModality {
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
pub enum M5SupportIntakeRenderingSurface {
    /// The full-capability desktop support surface.
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

impl M5SupportIntakeRenderingSurface {
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
pub enum SupportIntakeNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl SupportIntakeNonVisualReachState {
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
pub enum SupportIntakeExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl SupportIntakeExportSummaryState {
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
pub enum SupportIntakeNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl SupportIntakeNarrowingDisclosureState {
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

/// The support claim ceiling a component asserts: how strong a support-intake / escalation
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a support
/// dimension weakens so an uncertain scenario, an evidence-omitted report, a local-only
/// destination, or a policy-blocked repair can never keep an old `ReadyToEscalate` or
/// `ReviewableCase` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeClaim {
    /// Ready to escalate: a classified scenario, complete evidence, a shareable destination,
    /// and an approved repair path — the strongest claim, a case a human can act on as-is.
    ReadyToEscalate,
    /// Reviewable case: a self-sufficient, reviewable case / report (evidence a reviewer can
    /// read) that is not itself a certified send-ready escalation.
    ReviewableCase,
    /// Local-only diagnosis: usable, but the destination is local-only — the case cannot be
    /// shared or uploaded and stays a self-diagnosis start.
    LocalOnlyDiagnosis,
    /// Evidence-incomplete case: one or more evidence classes were omitted; the case is
    /// attributable but cannot carry the full evidence a reviewer needs.
    EvidenceIncompleteCase,
    /// Unclassified scenario: the scenario classification is uncertain / unmapped and must be
    /// narrowed before the case is trusted.
    UnclassifiedScenario,
    /// Policy-blocked repair: the repair guidance is policy-blocked; no approved repair can
    /// proceed and the note stays a blocked-action explanation.
    PolicyBlockedRepair,
}

impl M5SupportIntakeClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ReadyToEscalate,
        Self::ReviewableCase,
        Self::LocalOnlyDiagnosis,
        Self::EvidenceIncompleteCase,
        Self::UnclassifiedScenario,
        Self::PolicyBlockedRepair,
    ];

    /// Capability rank; a higher rank asserts a stronger support posture. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ReadyToEscalate => 5,
            Self::ReviewableCase => 4,
            Self::LocalOnlyDiagnosis => 3,
            Self::EvidenceIncompleteCase => 2,
            Self::UnclassifiedScenario => 1,
            Self::PolicyBlockedRepair => 0,
        }
    }

    /// Returns true when this claim asserts a fully send-ready escalation.
    pub const fn asserts_ready_to_escalate(self) -> bool {
        matches!(self, Self::ReadyToEscalate)
    }

    /// Returns true when this claim asserts a fully self-sufficient (send-ready or reviewable)
    /// case.
    pub const fn asserts_full_case(self) -> bool {
        matches!(self, Self::ReadyToEscalate | Self::ReviewableCase)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToEscalate => "ready_to_escalate",
            Self::ReviewableCase => "reviewable_case",
            Self::LocalOnlyDiagnosis => "local_only_diagnosis",
            Self::EvidenceIncompleteCase => "evidence_incomplete_case",
            Self::UnclassifiedScenario => "unclassified_scenario",
            Self::PolicyBlockedRepair => "policy_blocked_repair",
        }
    }
}

/// The support dimension whose state governs how far a component may claim to be a ready-to-
/// escalate case. The four spec axes the lane must auto-narrow on — uncertain scenario
/// classification, omitted evidence classes, local-only destination, and policy-blocked repair
/// guidance — are [`Self::ScenarioClassification`], [`Self::EvidenceCompleteness`],
/// [`Self::DestinationReach`], and [`Self::RepairGuidance`]; the remaining dimension covers the
/// handoff-timeline family's primary weakening axis so every frozen family carries an honest
/// narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeClaimDimension {
    /// Scenario classification: did the support-scenario picker row map the symptom to a stable
    /// scenario family and scope, or is the classification uncertain?
    ScenarioClassification,
    /// Evidence completeness: does the issue-report builder step include every needed evidence
    /// class, or were some omitted?
    EvidenceCompleteness,
    /// Destination reach: does the escalation-packet summary reach a shareable destination, or
    /// is it local-only?
    DestinationReach,
    /// Handoff continuity: does the handoff-timeline row carry a stated next human step and
    /// owner, or is the continuity unstated?
    HandoffContinuity,
    /// Repair guidance: does the unsafe-fix blocked note name an approved repair path, or is
    /// the repair guidance policy-blocked?
    RepairGuidance,
}

impl M5SupportIntakeClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ScenarioClassification,
        Self::EvidenceCompleteness,
        Self::DestinationReach,
        Self::HandoffContinuity,
        Self::RepairGuidance,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a narrowing.
    /// Each dimension maps to the on-topic frozen trigger the freeze matrix already governs, so
    /// the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5SupportDowngradeTrigger {
        match self {
            Self::ScenarioClassification => M5SupportDowngradeTrigger::ScenarioOrScopeUnstated,
            Self::EvidenceCompleteness => M5SupportDowngradeTrigger::EvidenceClassMasked,
            Self::DestinationReach => M5SupportDowngradeTrigger::PacketDestinationUnstated,
            Self::HandoffContinuity => M5SupportDowngradeTrigger::NextHumanStepUnstated,
            Self::RepairGuidance => M5SupportDowngradeTrigger::ApprovedRepairClassMasked,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioClassification => "scenario_classification",
            Self::EvidenceCompleteness => "evidence_completeness",
            Self::DestinationReach => "destination_reach",
            Self::HandoffContinuity => "handoff_continuity",
            Self::RepairGuidance => "repair_guidance",
        }
    }
}

/// The observed condition of one support dimension. Anything weaker than [`Self::Classified`]
/// imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConditionState {
    /// Fully classified / complete / shareable / approved — imposes no ceiling.
    Classified,
    /// Local-only destination — the case cannot leave the machine; support drops to
    /// local-only-diagnosis.
    LocalOnlyDestination,
    /// Evidence omitted — one or more evidence classes were left out; support drops to
    /// evidence-incomplete.
    EvidenceOmitted,
    /// Scenario uncertain — the scenario classification is uncertain / unmapped; support drops
    /// to unclassified-scenario.
    ScenarioUncertain,
    /// Repair policy-blocked — the repair guidance is policy-blocked; support drops to
    /// policy-blocked-repair.
    RepairPolicyBlocked,
}

impl M5SupportIntakeConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Classified,
        Self::LocalOnlyDestination,
        Self::EvidenceOmitted,
        Self::ScenarioUncertain,
        Self::RepairPolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than classified and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Classified)
    }

    /// The strongest support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5SupportIntakeClaim {
        match self {
            Self::Classified => M5SupportIntakeClaim::ReadyToEscalate,
            Self::LocalOnlyDestination => M5SupportIntakeClaim::LocalOnlyDiagnosis,
            Self::EvidenceOmitted => M5SupportIntakeClaim::EvidenceIncompleteCase,
            Self::ScenarioUncertain => M5SupportIntakeClaim::UnclassifiedScenario,
            Self::RepairPolicyBlocked => M5SupportIntakeClaim::PolicyBlockedRepair,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classified => "classified",
            Self::LocalOnlyDestination => "local_only_destination",
            Self::EvidenceOmitted => "evidence_omitted",
            Self::ScenarioUncertain => "scenario_uncertain",
            Self::RepairPolicyBlocked => "repair_policy_blocked",
        }
    }
}

/// One support dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportIntakeClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5SupportIntakeClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5SupportIntakeConditionState,
}

/// An honest support-claim auto-narrow block. When a support dimension weakens, the component's
/// support claim lowers to the permitted ceiling, names the binding dimension and frozen
/// trigger, and preserves the canonical scenario / finding / packet / redaction / repair
/// lineage rather than silently dropping it — the underlying case lineage is never erased
/// opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportIntakeClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5SupportIntakeClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5SupportIntakeClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5SupportDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical scenario family, incident scope, Doctor finding lineage, packet id, and
    /// destination are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying scenario / finding / packet / redaction / repair lineage is preserved
    /// (never dropped) across the narrowing; must hold so uncertain, evidence-omitted,
    /// local-only, and policy-blocked states never fail opaquely.
    pub preserves_lineage_continuity: bool,
}

impl SupportIntakeClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and case
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
pub struct SupportIntakeCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl SupportIntakeCopyExportParity {
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
pub struct SupportIntakeRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5SupportIntakeRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: SupportIntakeNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a support-intake accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportIntakeComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims escalation, or drops state
    /// silently (red).
    Stranded,
}

impl SupportIntakeComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one support-intake / escalation component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportIntakeComponentAccessibilityRow {
    /// Record kind; must equal [`SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5SupportIntakeEscalationComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the scenario / report / packet / handoff / note object this component acts
    /// on; stays visible on every surface, so this is never empty.
    pub support_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5SupportIntakeFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical scenario, scope, evidence, finding,
    /// packet, redaction, and next-step truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: SupportIntakeNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: SupportIntakeNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: SupportIntakeNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: SupportIntakeExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: SupportIntakeCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5SupportIntakeClaim,
    /// The observed condition of each modeled support dimension.
    #[serde(default)]
    pub claim_conditions: Vec<SupportIntakeClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<SupportIntakeClaimAutoNarrow>,
    /// Whether the underlying case lineage is preserved on this component regardless of
    /// narrowing; must hold so uncertain, evidence-omitted, local-only, and policy-blocked
    /// states never fail opaquely.
    pub lineage_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5SupportIntakeRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<SupportIntakeRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5SupportRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5SupportConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl SupportIntakeComponentAccessibilityRow {
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

    /// The condition state observed for one dimension, or `Classified` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5SupportIntakeClaimDimension,
    ) -> M5SupportIntakeConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5SupportIntakeConditionState::Classified)
    }

    /// Whether any modeled dimension is weaker than classified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5SupportIntakeClaim {
        let mut permitted = self.full_support_claim;
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
    pub fn binding_dimension(&self) -> Option<M5SupportIntakeClaimDimension> {
        let mut binding: Option<(M5SupportIntakeClaimDimension, u8)> = None;
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
    pub fn effective_claim(&self) -> M5SupportIntakeClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: an uncertain scenario, an evidence-omitted report, a
    /// local-only destination, or a policy-blocked repair can no longer keep an old
    /// `ReadyToEscalate` / `ReviewableCase` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block
    /// is present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing
    /// dimension with its frozen trigger, and preserves canonical identity and case lineage.
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
            && !self.support_context_ref.trim().is_empty()
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

    /// AC / no-loss: uncertain, evidence-omitted, local-only, and policy-blocked states preserve
    /// the underlying case lineage. The row must assert `lineage_preserved`, and any narrow
    /// block must preserve lineage continuity too.
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
        M5SupportRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> SupportIntakeComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_lineage_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return SupportIntakeComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            SupportIntakeComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            SupportIntakeComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.support_context_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-906 support-intake / escalation component accessibility fallback
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportIntakeComponentAccessibilitySummary {
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

/// Constructor input for [`SupportIntakeComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportIntakeComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<SupportIntakeComponentAccessibilityRow>,
}

/// Checked-in M05-906 support-intake / escalation component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportIntakeComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<SupportIntakeComponentAccessibilityRow>,
    pub summary: SupportIntakeComponentAccessibilitySummary,
}

impl SupportIntakeComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: SupportIntakeComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: SupportIntakeComponentAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5SupportIntakeEscalationComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5SupportIntakeClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5SupportIntakeConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5SupportIntakeClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5SupportConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> SupportIntakeComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5SupportConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&SupportIntakeComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                SupportIntakeComponentAccessibilityStatus::Parity => green += 1,
                SupportIntakeComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                SupportIntakeComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        SupportIntakeComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(SupportIntakeComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(SupportIntakeComponentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(SupportIntakeComponentAccessibilityRow::export_preserves_meaning),
            all_lineage_preserved: self
                .rows
                .iter()
                .all(SupportIntakeComponentAccessibilityRow::preserves_lineage_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(SupportIntakeComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<SupportIntakeComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(
                SupportIntakeComponentAccessibilityViolation::SchemaVersion {
                    expected: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(SupportIntakeComponentAccessibilityViolation::RecordKind {
                expected: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(SupportIntakeComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(SupportIntakeComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::IncompleteRow {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory support label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5SupportIntakeFallbackModality::Structured)
            {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a ready / reviewable case for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: uncertain, evidence-omitted, local-only, and policy-blocked states preserve
            // case lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::LineageDropped {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == SupportIntakeComponentAccessibilityStatus::Stranded {
                violations.push(SupportIntakeComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5SupportIntakeEscalationComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5SupportIntakeClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the classified baseline plus each spec narrowing
        // axis) is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5SupportIntakeConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (ready-to-escalate → … → policy-blocked-repair) is proven
        // end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5SupportIntakeClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingClaimTierCoverage {
                        claim,
                    },
                );
            }
        }

        // Cross-surface: the same narrowed state must reach the Doctor UI, support center,
        // report builder, escalation desk, recovery center, Help center, CLI, and support /
        // release exports — so every consumer surface is exercised at least once across the
        // packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5SupportConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    SupportIntakeComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(SupportIntakeComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("support-intake / escalation accessibility fallback packet serializes"),
        ) {
            violations
                .push(SupportIntakeComponentAccessibilityViolation::RawSupportMaterialInExport);
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
            .expect("support-intake / escalation accessibility fallback packet serializes")
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
        out.push_str(
            "# M5 Support-Intake / Escalation Component Accessibility & Auto-Narrowing\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5SupportIntakeEscalationComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in support-intake / escalation component accessibility
/// fallback export.
pub fn current_m5_support_intake_component_a11y_fallback_export() -> Result<
    SupportIntakeComponentAccessibilityPacket,
    SupportIntakeComponentAccessibilityArtifactError,
> {
    let packet: SupportIntakeComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/support_export.json"
    )))
    .map_err(SupportIntakeComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SupportIntakeComponentAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in support-intake / escalation component
/// accessibility fallback export.
#[derive(Debug)]
pub enum SupportIntakeComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SupportIntakeComponentAccessibilityViolation>),
}

impl fmt::Display for SupportIntakeComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "support-intake / escalation accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "support-intake / escalation accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for SupportIntakeComponentAccessibilityArtifactError {}

/// Validation failure for M05-906 support-intake / escalation component accessibility fallback
/// packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportIntakeComponentAccessibilityViolation {
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
        dimension: M5SupportIntakeClaimDimension,
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
        family: M5SupportIntakeEscalationComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5SupportIntakeClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5SupportIntakeConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5SupportIntakeClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5SupportConsumerSurface,
    },
    SummaryMismatch,
    RawSupportMaterialInExport,
}

impl fmt::Display for SupportIntakeComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory support label")
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
                    "row {id} over-asserts a ready / reviewable case for a weakened one, or narrows spuriously"
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
                    "row {id} does not preserve case lineage across narrowing"
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
            Self::RawSupportMaterialInExport => {
                write!(f, "export contains raw support material")
            }
        }
    }
}

impl Error for SupportIntakeComponentAccessibilityViolation {}

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
            | "uncertain"
            | "local only"
            | "policy blocked"
            | "unclassified"
            | "omitted"
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

/// Builds the canonical, checked-in support-intake / escalation component accessibility fallback
/// packet. This is the one source of truth shared by the tests and the on-disk support export
/// so both stay byte-aligned.
pub fn seeded_m5_support_intake_component_a11y_fallback_packet(
) -> SupportIntakeComponentAccessibilityPacket {
    SupportIntakeComponentAccessibilityPacket::new(SupportIntakeComponentAccessibilityPacketInput {
        packet_id: "m5-support-intake-escalation-component-accessibility-fallback:stable:0001"
            .to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:support-intake-escalation-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5SupportRequiredLabel> {
    M5SupportRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> SupportIntakeCopyExportParity {
    SupportIntakeCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5SupportIntakeClaimDimension,
    state: M5SupportIntakeConditionState,
) -> SupportIntakeClaimConditionEntry {
    SupportIntakeClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and CLI
/// inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5SupportConsumerSurface]) -> Vec<M5SupportConsumerSurface> {
    let mut out = vec![
        M5SupportConsumerSurface::SupportExport,
        M5SupportConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: SupportIntakeNarrowingDisclosureState,
) -> Vec<SupportIntakeRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        SupportIntakeRenderingNarrowingDisclosure {
            rendering_surface: M5SupportIntakeRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        SupportIntakeRenderingNarrowingDisclosure {
            rendering_surface: M5SupportIntakeRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_escalation".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<SupportIntakeRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        SupportIntakeNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<SupportIntakeRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        SupportIntakeNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5SupportIntakeRenderingSurface> {
    vec![
        M5SupportIntakeRenderingSurface::DesktopFull,
        M5SupportIntakeRenderingSurface::CliHeadless,
        M5SupportIntakeRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<SupportIntakeComponentAccessibilityRow> {
    vec![
        // Support-scenario picker row — the symptom could not be mapped to a stable scenario
        // family with confidence, so the row auto-narrows to an unclassified scenario rather
        // than presenting a classified, ready-to-escalate case, while keeping its scope and
        // bound Doctor finding family visible (yellow).
        SupportIntakeComponentAccessibilityRow {
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:support-scenario-picker-row".to_owned(),
            component_family: M5SupportIntakeEscalationComponentFamily::SupportScenarioPickerRow,
            source_family_schema_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            support_context_ref: "support:scenario-picker-row:0001".to_owned(),
            fallback_modalities: vec![
                M5SupportIntakeFallbackModality::List,
                M5SupportIntakeFallbackModality::Textual,
                M5SupportIntakeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            cli_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            export_summary: SupportIntakeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:support-scenario-picker-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "scenario_family",
                "incident_scope",
                "doctor_finding_family",
                "keyboard_route",
            ]),
            full_support_claim: M5SupportIntakeClaim::ReadyToEscalate,
            claim_conditions: vec![condition(
                M5SupportIntakeClaimDimension::ScenarioClassification,
                M5SupportIntakeConditionState::ScenarioUncertain,
            )],
            claim_narrow: Some(SupportIntakeClaimAutoNarrow {
                narrowed_to: M5SupportIntakeClaim::UnclassifiedScenario,
                binding_dimension: M5SupportIntakeClaimDimension::ScenarioClassification,
                trigger: M5SupportDowngradeTrigger::ScenarioOrScopeUnstated,
                narrowed_label:
                    "Symptom could not be mapped to a stable scenario family with confidence — shown as an unclassified scenario with its scope and bound finding family still preserved, starting a local diagnosis"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "scenario_family",
                "incident_scope",
                "doctor_finding_family",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5SupportConsumerSurface::DoctorUi,
                M5SupportConsumerSurface::SupportCenterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.33 support-scenario picker rows".to_owned(),
                SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("support-scenario-picker-row"),
        },
        // Issue-report builder step (evidence omitted) — one or more evidence classes were left
        // out of the report, so the step auto-narrows to an evidence-incomplete case rather than
        // presenting a full, ready-to-escalate report (yellow).
        SupportIntakeComponentAccessibilityRow {
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:issue-report-builder-step-evidence-omitted".to_owned(),
            component_family: M5SupportIntakeEscalationComponentFamily::IssueReportBuilderStep,
            source_family_schema_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            support_context_ref: "support:issue-report-builder-step:0002".to_owned(),
            fallback_modalities: vec![
                M5SupportIntakeFallbackModality::List,
                M5SupportIntakeFallbackModality::Textual,
                M5SupportIntakeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            cli_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            export_summary: SupportIntakeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:issue-report-builder-step-evidence-omitted:a11y".to_owned(),
            copy_export: copy_export(&[
                "step_kind",
                "included_evidence_class",
                "excluded_evidence_class",
                "redaction_state",
            ]),
            full_support_claim: M5SupportIntakeClaim::ReadyToEscalate,
            claim_conditions: vec![condition(
                M5SupportIntakeClaimDimension::EvidenceCompleteness,
                M5SupportIntakeConditionState::EvidenceOmitted,
            )],
            claim_narrow: Some(SupportIntakeClaimAutoNarrow {
                narrowed_to: M5SupportIntakeClaim::EvidenceIncompleteCase,
                binding_dimension: M5SupportIntakeClaimDimension::EvidenceCompleteness,
                trigger: M5SupportDowngradeTrigger::EvidenceClassMasked,
                narrowed_label:
                    "One or more evidence classes were left out of the report — shown as an evidence-incomplete case that names the included and excluded classes, never as a full report"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "step_kind",
                "included_evidence_class",
                "excluded_evidence_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5SupportConsumerSurface::ReportBuilderUi,
                M5SupportConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix CI issue-report builder steps".to_owned(),
                SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("issue-report-builder-step-evidence-omitted"),
        },
        // Issue-report builder step (reviewable) — the report includes every needed evidence
        // class and is a self-sufficient reviewable case (not itself a certified escalation),
        // reachable on every surface (green).
        SupportIntakeComponentAccessibilityRow {
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:issue-report-builder-step-reviewable".to_owned(),
            component_family: M5SupportIntakeEscalationComponentFamily::IssueReportBuilderStep,
            source_family_schema_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            support_context_ref: "support:issue-report-builder-step:0003".to_owned(),
            fallback_modalities: vec![
                M5SupportIntakeFallbackModality::List,
                M5SupportIntakeFallbackModality::Textual,
                M5SupportIntakeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            cli_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            export_summary: SupportIntakeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:issue-report-builder-step-reviewable:a11y".to_owned(),
            copy_export: copy_export(&[
                "step_kind",
                "included_evidence_class",
                "redaction_state",
                "repro_summary",
            ]),
            full_support_claim: M5SupportIntakeClaim::ReviewableCase,
            claim_conditions: vec![condition(
                M5SupportIntakeClaimDimension::EvidenceCompleteness,
                M5SupportIntakeConditionState::Classified,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "step_kind",
                "included_evidence_class",
                "redaction_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5SupportConsumerSurface::ReportBuilderUi,
                M5SupportConsumerSurface::HelpCenterUi,
            ]),
            source_refs: vec![
                "UX Design System §33.15 support-center / artifact-fidelity layout".to_owned(),
                SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("issue-report-builder-step-reviewable"),
        },
        // Escalation-packet summary — hierarchy-heavy (nested finding / repair / evidence
        // lineage); the only available destination is local-only, so the summary auto-narrows to
        // a local-only diagnosis and binds its nested lineage to a flat list / textual path
        // (yellow).
        SupportIntakeComponentAccessibilityRow {
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:escalation-packet-summary".to_owned(),
            component_family: M5SupportIntakeEscalationComponentFamily::EscalationPacketSummary,
            source_family_schema_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            support_context_ref: "support:escalation-packet-summary:0004".to_owned(),
            fallback_modalities: vec![
                M5SupportIntakeFallbackModality::Structured,
                M5SupportIntakeFallbackModality::List,
                M5SupportIntakeFallbackModality::Textual,
                M5SupportIntakeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: SupportIntakeNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            export_summary: SupportIntakeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:escalation-packet-summary:a11y".to_owned(),
            copy_export: copy_export(&[
                "packet_id",
                "packet_destination",
                "finding_repair_lineage",
                "redaction_state",
            ]),
            full_support_claim: M5SupportIntakeClaim::ReadyToEscalate,
            claim_conditions: vec![condition(
                M5SupportIntakeClaimDimension::DestinationReach,
                M5SupportIntakeConditionState::LocalOnlyDestination,
            )],
            claim_narrow: Some(SupportIntakeClaimAutoNarrow {
                narrowed_to: M5SupportIntakeClaim::LocalOnlyDiagnosis,
                binding_dimension: M5SupportIntakeClaimDimension::DestinationReach,
                trigger: M5SupportDowngradeTrigger::PacketDestinationUnstated,
                narrowed_label:
                    "Packet destination is restricted to a local-only bundle — shown as a local-only diagnosis with its packet id and finding / repair lineage preserved, never as a shared escalation"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "packet_id",
                "packet_destination",
                "finding_repair_lineage",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5SupportConsumerSurface::EscalationDeskUi,
                M5SupportConsumerSurface::RecoveryCenterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.33 escalation-packet summaries".to_owned(),
                SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("escalation-packet-summary"),
        },
        // Handoff-timeline row — the timeline carries a stated owner and next human step and is
        // a fully send-ready case, reachable on every surface (green).
        SupportIntakeComponentAccessibilityRow {
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:handoff-timeline-row".to_owned(),
            component_family: M5SupportIntakeEscalationComponentFamily::HandoffTimelineRow,
            source_family_schema_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            support_context_ref: "support:handoff-timeline-row:0005".to_owned(),
            fallback_modalities: vec![
                M5SupportIntakeFallbackModality::List,
                M5SupportIntakeFallbackModality::Textual,
                M5SupportIntakeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            cli_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            export_summary: SupportIntakeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:handoff-timeline-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "handoff_stage",
                "owner",
                "next_human_step",
                "packet_id",
            ]),
            full_support_claim: M5SupportIntakeClaim::ReadyToEscalate,
            claim_conditions: vec![condition(
                M5SupportIntakeClaimDimension::HandoffContinuity,
                M5SupportIntakeConditionState::Classified,
            )],
            claim_narrow: None,
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "handoff_stage",
                "owner",
                "next_human_step",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5SupportConsumerSurface::EscalationDeskUi,
                M5SupportConsumerSurface::HelpCenterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.33 handoff-timeline rows".to_owned(),
                SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("handoff-timeline-row"),
        },
        // Unsafe-fix blocked note — the suggested repair is policy-blocked, so the note
        // auto-narrows to a policy-blocked repair rather than presenting an approved,
        // ready-to-apply fix, while keeping the block reason and safer-repair guidance visible
        // (yellow).
        SupportIntakeComponentAccessibilityRow {
            record_kind: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:unsafe-fix-blocked-note".to_owned(),
            component_family: M5SupportIntakeEscalationComponentFamily::UnsafeFixBlockedNote,
            source_family_schema_ref: SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            support_context_ref: "support:unsafe-fix-blocked-note:0006".to_owned(),
            fallback_modalities: vec![
                M5SupportIntakeFallbackModality::List,
                M5SupportIntakeFallbackModality::Textual,
                M5SupportIntakeFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            cli_reach: SupportIntakeNonVisualReachState::ReachableAndLabeled,
            export_summary: SupportIntakeExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:unsafe-fix-blocked-note:a11y".to_owned(),
            copy_export: copy_export(&[
                "block_reason",
                "approved_repair_class",
                "safer_repair",
                "blast_radius",
            ]),
            full_support_claim: M5SupportIntakeClaim::ReadyToEscalate,
            claim_conditions: vec![condition(
                M5SupportIntakeClaimDimension::RepairGuidance,
                M5SupportIntakeConditionState::RepairPolicyBlocked,
            )],
            claim_narrow: Some(SupportIntakeClaimAutoNarrow {
                narrowed_to: M5SupportIntakeClaim::PolicyBlockedRepair,
                binding_dimension: M5SupportIntakeClaimDimension::RepairGuidance,
                trigger: M5SupportDowngradeTrigger::ApprovedRepairClassMasked,
                narrowed_label:
                    "Suggested repair is held by policy and cannot be applied — shown as a policy-blocked repair that names the block reason and safer-repair guidance, never an approved fix"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_lineage_continuity: true,
            }),
            lineage_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "block_reason",
                "approved_repair_class",
                "safer_repair",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5SupportConsumerSurface::DoctorUi,
                M5SupportConsumerSurface::RecoveryCenterUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §18.33 unsafe-fix blocked notes".to_owned(),
                SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("unsafe-fix-blocked-note"),
        },
    ]
}
