//! One reusable M5 support-intake primitive — the issue-report builder step — so a
//! support packet's scope and omissions are made explicit *before* anything is shared:
//! the human-readable summary, the ordered reproduction steps, the selected evidence
//! classes, the excluded evidence classes, the redaction posture, and the same-weight
//! local-only preview all stay legible and never collapse into one opaque "report draft"
//! blob.
//!
//! Aureline's frozen support-intake / escalation component matrix
//! ([`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`])
//! names the issue-report builder step as one governed component family and freezes its
//! controlled vocabulary — the builder step kinds, the evidence classes it selects or
//! omits, and the redaction states an export can carry — plus the surface families, the
//! deployment lines, the consumer surfaces, the accessibility routes, the qualification
//! classes, and the downgrade triggers. This module *implements* that contract as one
//! reusable resolver so a user can tell — from the builder step alone — which evidence
//! classes will leave the local boundary and which stay excluded, at what data-risk
//! class, under which redaction posture, without ever losing a same-weight local-only
//! preview.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_issue_report_builder_step`] — takes one builder step's kind, its
//!    human-readable summary, its ordered reproduction steps, the selected and excluded
//!    evidence classes, the redaction posture, a share-requested signal, and a stable step
//!    identity, and produces one [`M5ResolvedIssueReportBuilderStep`] carrying the derived
//!    step posture (share-blocked, no-evidence-selected, redaction-review-required,
//!    local-only-preview, or ready-to-share), one boundary disposition per decided
//!    evidence class (its data-risk class, whether it is selected, and whether it will
//!    cross the local boundary), and the bounded reveal-boundary / preview-local-only /
//!    edit-selection / review-redaction / share / export actions. It never masks which
//!    evidence class is selected or omitted, never hides the redaction posture, never
//!    collapses the summary / repro steps / evidence into one blob, and always offers the
//!    same-weight local-only preview so a user is never forced to share to inspect their
//!    own draft.
//!
//! A single parity matrix — [`M5IssueReportBuilderStepPacket`] — binds one row per claimed
//! M5 support-intake consumer (support-center builder, recovery-center builder, Doctor
//! handoff builder, headless/CLI builder, and support-packet export) to the shared
//! builder-step anatomy, the same builder step kinds, evidence classes, data-risk classes,
//! redaction states, step postures, bounded actions, export fields, and non-visual
//! accessibility routes, so the evidence / redaction / boundary vocabulary stays identical
//! across desktop, headless/export, and support-packet consumers.
//!
//! The builder step kind ([`M5ReportBuilderStepKind`]), evidence class
//! ([`M5SupportEvidenceClass`]), redaction state ([`M5SupportRedactionState`]), surface
//! family ([`M5SupportSurfaceFamily`]), deployment line ([`M5SupportDeploymentLine`]),
//! consumer surface ([`M5SupportConsumerSurface`]), accessibility route
//! ([`M5SupportAccessibilityRoute`]), qualification class
//! ([`M5SupportQualificationClass`]), and downgrade trigger
//! ([`M5SupportDowngradeTrigger`]) are reused verbatim from the frozen matrix. The
//! metadata / environment-adjacent / code-adjacent / high-risk data-risk vocabulary
//! ([`DataClass`]) is reused verbatim from the Support Center matrix — the same one Doctor
//! and support-bundle exports use — so included and excluded classes are never given a
//! parallel sensitivity grammar. This module mints new vocabulary only for what those
//! matrices left implicit about the builder step itself: its intake consumers, its anatomy
//! parts, its derived step posture, its bounded actions, and its export fields.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! support boundary; every summary, reproduction step, and step identity is carried only
//! as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_issue_report_builder_step_headless_cli_builder_beta_narrowed,
    seeded_m5_issue_report_builder_step_packet,
    seeded_m5_issue_report_builder_step_recovery_center_builder_preview_narrowed,
    M5_ISSUE_REPORT_BUILDER_STEP_PACKET_ID,
};

// The builder step kind, evidence class, redaction state, surface family, deployment line,
// consumer surface, accessibility route, qualification class, and downgrade triggers are
// frozen once, in the support-intake / escalation component matrix. This primitive reuses
// them verbatim so it never invents a parallel evidence or redaction vocabulary.
pub use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5ReportBuilderStepKind, M5SupportAccessibilityRoute, M5SupportConsumerSurface,
    M5SupportDeploymentLine, M5SupportDowngradeTrigger, M5SupportEvidenceClass,
    M5SupportQualificationClass, M5SupportRedactionState, M5SupportSurfaceFamily,
};

// The data-risk class vocabulary (metadata-only / environment-adjacent / code-adjacent /
// high-risk) is the one already used by Doctor and support-bundle exports; reuse it so the
// included and excluded evidence classes never get a local synonym for "safe to export".
pub use crate::m5_support_center_matrix::DataClass;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5IssueReportBuilderStepPacket`].
pub const M5_ISSUE_REPORT_BUILDER_STEP_RECORD_KIND: &str =
    "implement_m5_issue_report_builder_steps_and_evidence_class_selectors_with_included_excluded_redaction_repro_and_local_only_preview_truth_across_claimed_m5_support_flows";

/// Schema version for M5 issue-report-builder-step records.
pub const M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the builder-step boundary schema.
pub const M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF: &str =
    "schemas/ui/m5-support-issue-report-builder-step.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ISSUE_REPORT_BUILDER_STEP_DOC_REF: &str =
    "docs/support/m5_support_issue_report_builder_step_primitive.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix this
/// primitive narrows from.
pub const M5_ISSUE_REPORT_BUILDER_STEP_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-matrix.schema.json";

/// Repo-relative path of the data-risk-class contract this primitive binds its included /
/// excluded evidence sensitivity against.
pub const M5_ISSUE_REPORT_BUILDER_STEP_DATA_RISK_CLASS_REF: &str =
    "schemas/support/data_risk_class.schema.json";

/// Repo-relative path of the export-redaction-profile contract this primitive binds its
/// redaction posture against.
pub const M5_ISSUE_REPORT_BUILDER_STEP_EXPORT_REDACTION_PROFILE_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the support-bundle-manifest contract this primitive binds its
/// selected / excluded evidence against.
pub const M5_ISSUE_REPORT_BUILDER_STEP_SUPPORT_BUNDLE_MANIFEST_REF: &str =
    "schemas/support/support_bundle_manifest.schema.json";

/// Repo-relative path of the Doctor-finding contract behind the Doctor-finding evidence
/// class.
pub const M5_ISSUE_REPORT_BUILDER_STEP_DOCTOR_FINDING_REF: &str =
    "schemas/support/doctor_finding.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_ISSUE_REPORT_BUILDER_STEP_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-issue-report-builder-step-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ISSUE_REPORT_BUILDER_STEP_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-issue-report-builder-step-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_ISSUE_REPORT_BUILDER_STEP_CSV_REF: &str =
    "artifacts/release/m5-support-issue-report-builder-step-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ISSUE_REPORT_BUILDER_STEP_REPORT_REF: &str =
    "artifacts/design/m5-support-issue-report-builder-step-primitive.md";

/// One claimed M5 support-intake consumer that renders the shared issue-report builder
/// step. These are the consumers the acceptance criteria name — the support-center and
/// recovery-center report builders, the Doctor handoff builder, the headless / CLI
/// builder, and the support-packet export — so the same evidence / redaction / boundary
/// grammar works across every claimed support flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IssueReportBuilderConsumerSurface {
    /// The support-center report builder.
    SupportCenterBuilder,
    /// The recovery-center report builder.
    RecoveryCenterBuilder,
    /// The Project Doctor handoff report builder.
    DoctorHandoffBuilder,
    /// The headless / CLI report builder.
    HeadlessCliBuilder,
    /// The support-packet export surface.
    SupportPacketExport,
}

impl M5IssueReportBuilderConsumerSurface {
    /// Every claimed support-intake consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SupportCenterBuilder,
        Self::RecoveryCenterBuilder,
        Self::DoctorHandoffBuilder,
        Self::HeadlessCliBuilder,
        Self::SupportPacketExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportCenterBuilder => "support_center_builder",
            Self::RecoveryCenterBuilder => "recovery_center_builder",
            Self::DoctorHandoffBuilder => "doctor_handoff_builder",
            Self::HeadlessCliBuilder => "headless_cli_builder",
            Self::SupportPacketExport => "support_packet_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SupportCenterBuilder => "Support Center Builder",
            Self::RecoveryCenterBuilder => "Recovery Center Builder",
            Self::DoctorHandoffBuilder => "Doctor Handoff Builder",
            Self::HeadlessCliBuilder => "Headless / CLI Builder",
            Self::SupportPacketExport => "Support Packet Export",
        }
    }
}

/// The derived posture of an issue-report builder step — the resolver's verdict about what
/// will happen to the selected evidence at the local boundary. Computed in a fixed
/// blocking-first order, so a share-blocked or redaction-review-required step never reads
/// as ready to share, and a step that is only previewed locally never reads as one whose
/// evidence has already crossed the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IssueReportBuilderStepPosture {
    /// Export is blocked by policy or unavailability; nothing crosses, only the local-only
    /// preview remains.
    ShareBlocked,
    /// No evidence class is selected yet; nothing will cross the local boundary.
    NoEvidenceSelected,
    /// Code-adjacent or high-risk evidence is selected under a full-metadata posture;
    /// redaction must be reviewed before anything crosses.
    RedactionReviewRequired,
    /// The draft is being previewed locally only; the selected evidence stays on the
    /// device until a share is requested.
    LocalOnlyPreview,
    /// The selected evidence classes are ready to cross the local boundary under the chosen
    /// redaction posture.
    ReadyToShare,
}

impl M5IssueReportBuilderStepPosture {
    /// Every step posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ShareBlocked,
        Self::NoEvidenceSelected,
        Self::RedactionReviewRequired,
        Self::LocalOnlyPreview,
        Self::ReadyToShare,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShareBlocked => "share_blocked",
            Self::NoEvidenceSelected => "no_evidence_selected",
            Self::RedactionReviewRequired => "redaction_review_required",
            Self::LocalOnlyPreview => "local_only_preview",
            Self::ReadyToShare => "ready_to_share",
        }
    }

    /// True when the selected evidence will actually cross the local boundary at this
    /// posture — only when the step is ready to share.
    pub const fn permits_boundary_crossing(self) -> bool {
        matches!(self, Self::ReadyToShare)
    }

    /// True when a redaction review is required before anything sensitive crosses.
    pub const fn needs_redaction_review(self) -> bool {
        matches!(self, Self::RedactionReviewRequired)
    }

    /// True when the step needs operator attention before evidence can cross.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::ShareBlocked | Self::NoEvidenceSelected | Self::RedactionReviewRequired
        )
    }
}

/// One bounded action an issue-report builder step offers, so a step never hides its
/// reveal-boundary / preview-local-only / edit-selection / review-redaction / share /
/// export affordances, and never drops the same-weight local-only preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IssueReportBuilderStepAction {
    /// Reveal which evidence classes cross the local boundary and which remain excluded.
    RevealEvidenceBoundary,
    /// Preview the assembled report locally only, never leaving the device.
    PreviewLocalOnly,
    /// Edit the selected / excluded evidence classes.
    EditEvidenceSelection,
    /// Review the redaction posture before anything sensitive crosses.
    ReviewRedaction,
    /// Share the report beyond the local boundary under the chosen redaction posture.
    ShareReport,
    /// Export the builder step as metadata-only support evidence.
    ExportStep,
}

impl M5IssueReportBuilderStepAction {
    /// Every builder-step action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RevealEvidenceBoundary,
        Self::PreviewLocalOnly,
        Self::EditEvidenceSelection,
        Self::ReviewRedaction,
        Self::ShareReport,
        Self::ExportStep,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealEvidenceBoundary => "reveal_evidence_boundary",
            Self::PreviewLocalOnly => "preview_local_only",
            Self::EditEvidenceSelection => "edit_evidence_selection",
            Self::ReviewRedaction => "review_redaction",
            Self::ShareReport => "share_report",
            Self::ExportStep => "export_step",
        }
    }
}

/// Controlled issue-report-builder-step anatomy part the shared step surfaces. The parts
/// in [`M5IssueReportBuilderStepAnatomyPart::MANDATORY`] are required on every step so the
/// summary, reproduction steps, selected and excluded evidence, redaction posture, boundary
/// disposition, and same-weight local-only preview are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IssueReportBuilderStepAnatomyPart {
    /// The human-readable summary cue.
    SummaryCue,
    /// The ordered reproduction-steps cue.
    ReproStepsCue,
    /// The selected evidence classes cue.
    SelectedEvidenceCue,
    /// The excluded evidence classes cue.
    ExcludedEvidenceCue,
    /// The redaction-posture cue.
    RedactionPostureCue,
    /// The local-boundary crossing disposition cue.
    BoundaryCrossingCue,
    /// The same-weight local-only preview cue.
    LocalOnlyPreviewCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5IssueReportBuilderStepAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SummaryCue,
        Self::ReproStepsCue,
        Self::SelectedEvidenceCue,
        Self::ExcludedEvidenceCue,
        Self::RedactionPostureCue,
        Self::BoundaryCrossingCue,
        Self::LocalOnlyPreviewCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every step must render.
    pub const MANDATORY: [Self; 7] = [
        Self::SummaryCue,
        Self::ReproStepsCue,
        Self::SelectedEvidenceCue,
        Self::ExcludedEvidenceCue,
        Self::RedactionPostureCue,
        Self::BoundaryCrossingCue,
        Self::LocalOnlyPreviewCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummaryCue => "summary_cue",
            Self::ReproStepsCue => "repro_steps_cue",
            Self::SelectedEvidenceCue => "selected_evidence_cue",
            Self::ExcludedEvidenceCue => "excluded_evidence_cue",
            Self::RedactionPostureCue => "redaction_posture_cue",
            Self::BoundaryCrossingCue => "boundary_crossing_cue",
            Self::LocalOnlyPreviewCue => "local_only_preview_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the step export carries so issue-report-builder-step truth is reconstructable.
/// The fields in [`M5IssueReportBuilderStepExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IssueReportBuilderStepExportField {
    /// The builder step kind.
    StepKind,
    /// The human-readable summary.
    Summary,
    /// The ordered reproduction steps.
    ReproSteps,
    /// The selected evidence classes.
    SelectedEvidence,
    /// The excluded evidence classes.
    ExcludedEvidence,
    /// The redaction posture.
    RedactionState,
    /// The derived step posture.
    StepPosture,
    /// The per-class boundary disposition (data-risk class + crossing).
    EvidenceBoundary,
    /// Whether the same-weight local-only preview is available.
    LocalOnlyPreviewAvailable,
}

impl M5IssueReportBuilderStepExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StepKind,
        Self::Summary,
        Self::ReproSteps,
        Self::SelectedEvidence,
        Self::ExcludedEvidence,
        Self::RedactionState,
        Self::StepPosture,
        Self::EvidenceBoundary,
        Self::LocalOnlyPreviewAvailable,
    ];

    /// The export fields every step must carry.
    pub const MANDATORY: [Self; 8] = [
        Self::StepKind,
        Self::Summary,
        Self::ReproSteps,
        Self::SelectedEvidence,
        Self::ExcludedEvidence,
        Self::RedactionState,
        Self::EvidenceBoundary,
        Self::LocalOnlyPreviewAvailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StepKind => "step_kind",
            Self::Summary => "summary",
            Self::ReproSteps => "repro_steps",
            Self::SelectedEvidence => "selected_evidence",
            Self::ExcludedEvidence => "excluded_evidence",
            Self::RedactionState => "redaction_state",
            Self::StepPosture => "step_posture",
            Self::EvidenceBoundary => "evidence_boundary",
            Self::LocalOnlyPreviewAvailable => "local_only_preview_available",
        }
    }
}

/// The data-risk class an evidence class carries, mapped onto the frozen metadata-only /
/// environment-adjacent / code-adjacent / high-risk vocabulary Doctor and support-bundle
/// exports already use. This is the single source that tells a user how sensitive a class
/// is before it crosses the local boundary.
pub const fn evidence_data_class(evidence_class: M5SupportEvidenceClass) -> DataClass {
    match evidence_class {
        // Timestamps, event ids, and counts only.
        M5SupportEvidenceClass::ActivityTimeline => DataClass::MetadataOnly,
        // Platform, channel, and topology descriptors.
        M5SupportEvidenceClass::EnvironmentSnapshot => DataClass::EnvironmentAdjacent,
        // File paths, symbol names, stack frames, and diffs.
        M5SupportEvidenceClass::DoctorFinding
        | M5SupportEvidenceClass::CrashForensics
        | M5SupportEvidenceClass::RepairTransaction => DataClass::CodeAdjacent,
        // Free-form user text that may carry anything.
        M5SupportEvidenceClass::UserNote => DataClass::HighRisk,
    }
}

/// True when a data-risk class is sensitive enough that a full-metadata export must be
/// reviewed before it crosses the local boundary.
const fn data_class_is_sensitive(data_class: DataClass) -> bool {
    matches!(data_class, DataClass::CodeAdjacent | DataClass::HighRisk)
}

/// One evidence class's boundary disposition: its data-risk class, whether the user
/// selected it, and whether it will cross the local boundary at the resolved posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportEvidenceDisposition {
    /// The evidence class.
    pub evidence_class: M5SupportEvidenceClass,
    /// The data-risk class the evidence carries.
    pub data_class: DataClass,
    /// True when the user selected this class for the report.
    pub selected: bool,
    /// True when this class will cross the local boundary at the resolved posture.
    pub crosses_local_boundary: bool,
}

// ---- issue-report-builder-step resolver ---------------------------------

/// The full input to the issue-report-builder-step resolver for one step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepResolutionInput {
    /// Which builder step this is.
    pub step_kind: M5ReportBuilderStepKind,
    /// The opaque human-readable summary (must be non-empty).
    pub summary: String,
    /// The ordered, opaque reproduction steps (each must be non-empty when present).
    pub repro_steps: Vec<String>,
    /// The evidence classes selected for the report.
    pub selected_evidence: Vec<M5SupportEvidenceClass>,
    /// The evidence classes explicitly excluded from the report.
    pub excluded_evidence: Vec<M5SupportEvidenceClass>,
    /// The redaction posture the export will apply.
    pub redaction_state: M5SupportRedactionState,
    /// True when the user has requested a share beyond the local boundary (vs a local-only
    /// preview).
    pub share_requested: bool,
    /// The opaque stable step identity (must be non-empty).
    pub step_identity: String,
}

/// The resolved issue-report-builder-step truth for one step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedIssueReportBuilderStep {
    /// Which builder step this is.
    pub step_kind: M5ReportBuilderStepKind,
    /// The opaque human-readable summary, preserved exactly from the input.
    pub summary: String,
    /// The ordered reproduction steps, preserved exactly from the input.
    pub repro_steps: Vec<String>,
    /// The selected evidence classes, preserved exactly from the input.
    pub selected_evidence: Vec<M5SupportEvidenceClass>,
    /// The excluded evidence classes, preserved exactly from the input.
    pub excluded_evidence: Vec<M5SupportEvidenceClass>,
    /// The redaction posture, preserved exactly from the input.
    pub redaction_state: M5SupportRedactionState,
    /// The opaque stable step identity, preserved exactly from the input.
    pub step_identity: String,
    /// The derived step posture.
    pub step_posture: M5IssueReportBuilderStepPosture,
    /// One boundary disposition per decided evidence class (selected first, then excluded).
    pub evidence_dispositions: Vec<M5IssueReportEvidenceDisposition>,
    /// The evidence classes that will actually cross the local boundary (in selection
    /// order). Empty unless the step is ready to share.
    pub crossing_classes: Vec<M5SupportEvidenceClass>,
    /// The bounded actions this step offers.
    pub available_actions: Vec<M5IssueReportBuilderStepAction>,
    /// True when the selected evidence will cross the local boundary at this posture.
    pub will_cross_local_boundary: bool,
    /// True when a redaction review is required before anything sensitive crosses.
    pub needs_redaction_review: bool,
    /// True when the step carries code-adjacent or high-risk evidence in its selection.
    pub carries_sensitive_evidence: bool,
    /// True when the same-weight local-only preview is available. Always `true`: the
    /// local-only preview is never dropped.
    pub local_only_preview_available: bool,
    /// True when the step needs operator attention before evidence can cross.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_issue_report_builder_step`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5IssueReportBuilderStepResolutionError {
    /// The summary was empty.
    EmptySummary,
    /// The step identity was empty.
    EmptyStepIdentity,
    /// A reproduction step was blank.
    EmptyReproStep,
    /// An evidence class appeared in both the selected and excluded lists.
    EvidenceClassOverlap,
    /// A step descriptor carried forbidden material.
    ForbiddenReportMaterial,
}

impl M5IssueReportBuilderStepResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySummary => "empty_summary",
            Self::EmptyStepIdentity => "empty_step_identity",
            Self::EmptyReproStep => "empty_repro_step",
            Self::EvidenceClassOverlap => "evidence_class_overlap",
            Self::ForbiddenReportMaterial => "forbidden_report_material",
        }
    }
}

impl fmt::Display for M5IssueReportBuilderStepResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "issue report builder step resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5IssueReportBuilderStepResolutionError {}

/// Resolves one issue-report builder step from its declared state.
///
/// The derived step posture is computed in a fixed blocking-first order: an export blocked
/// by policy or unavailability wins first (nothing crosses, only the local-only preview
/// remains), then a step with no evidence selected (nothing will cross), then a step whose
/// selection carries code-adjacent or high-risk evidence under a full-metadata posture
/// (redaction must be reviewed first), then a step that is only being previewed locally
/// (evidence stays on the device), and otherwise a step ready to share. The summary,
/// reproduction steps, and selected / excluded evidence are carried explicitly and never
/// collapsed into one opaque blob; each decided evidence class gets a boundary disposition
/// naming its data-risk class and whether it will cross; and the step always offers the
/// same-weight local-only preview so a user can inspect their own draft without being
/// forced to share.
pub fn resolve_issue_report_builder_step(
    input: &M5IssueReportBuilderStepResolutionInput,
) -> Result<M5ResolvedIssueReportBuilderStep, M5IssueReportBuilderStepResolutionError> {
    if input.summary.trim().is_empty() {
        return Err(M5IssueReportBuilderStepResolutionError::EmptySummary);
    }
    if input.step_identity.trim().is_empty() {
        return Err(M5IssueReportBuilderStepResolutionError::EmptyStepIdentity);
    }
    if input.repro_steps.iter().any(|step| step.trim().is_empty()) {
        return Err(M5IssueReportBuilderStepResolutionError::EmptyReproStep);
    }
    let excluded_set: BTreeSet<M5SupportEvidenceClass> =
        input.excluded_evidence.iter().copied().collect();
    if input
        .selected_evidence
        .iter()
        .any(|class| excluded_set.contains(class))
    {
        return Err(M5IssueReportBuilderStepResolutionError::EvidenceClassOverlap);
    }
    if value_repr_is_forbidden(&input.summary)
        || value_repr_is_forbidden(&input.step_identity)
        || input
            .repro_steps
            .iter()
            .any(|step| value_repr_is_forbidden(step))
    {
        return Err(M5IssueReportBuilderStepResolutionError::ForbiddenReportMaterial);
    }

    let carries_sensitive_evidence = input
        .selected_evidence
        .iter()
        .any(|class| data_class_is_sensitive(evidence_data_class(*class)));
    let step_posture = derive_step_posture(
        &input.selected_evidence,
        input.redaction_state,
        input.share_requested,
        carries_sensitive_evidence,
    );
    let permits_crossing = step_posture.permits_boundary_crossing();

    let mut evidence_dispositions = Vec::new();
    for &class in &input.selected_evidence {
        evidence_dispositions.push(M5IssueReportEvidenceDisposition {
            evidence_class: class,
            data_class: evidence_data_class(class),
            selected: true,
            crosses_local_boundary: permits_crossing,
        });
    }
    for &class in &input.excluded_evidence {
        evidence_dispositions.push(M5IssueReportEvidenceDisposition {
            evidence_class: class,
            data_class: evidence_data_class(class),
            selected: false,
            crosses_local_boundary: false,
        });
    }

    let crossing_classes: Vec<M5SupportEvidenceClass> = if permits_crossing {
        input.selected_evidence.clone()
    } else {
        Vec::new()
    };
    let will_cross_local_boundary = !crossing_classes.is_empty();
    let needs_redaction_review = step_posture.needs_redaction_review();
    let available_actions = derive_step_actions(
        step_posture,
        carries_sensitive_evidence,
        needs_redaction_review,
    );

    Ok(M5ResolvedIssueReportBuilderStep {
        step_kind: input.step_kind,
        summary: input.summary.clone(),
        repro_steps: input.repro_steps.clone(),
        selected_evidence: input.selected_evidence.clone(),
        excluded_evidence: input.excluded_evidence.clone(),
        redaction_state: input.redaction_state,
        step_identity: input.step_identity.clone(),
        step_posture,
        evidence_dispositions,
        crossing_classes,
        available_actions,
        will_cross_local_boundary,
        needs_redaction_review,
        carries_sensitive_evidence,
        local_only_preview_available: true,
        needs_attention: step_posture.needs_attention(),
    })
}

/// The fixed blocking-first step-posture ladder.
fn derive_step_posture(
    selected_evidence: &[M5SupportEvidenceClass],
    redaction_state: M5SupportRedactionState,
    share_requested: bool,
    carries_sensitive_evidence: bool,
) -> M5IssueReportBuilderStepPosture {
    use M5IssueReportBuilderStepPosture as Posture;
    if matches!(redaction_state, M5SupportRedactionState::ExportBlocked) {
        Posture::ShareBlocked
    } else if selected_evidence.is_empty() {
        Posture::NoEvidenceSelected
    } else if carries_sensitive_evidence
        && matches!(redaction_state, M5SupportRedactionState::FullMetadata)
    {
        Posture::RedactionReviewRequired
    } else if !share_requested {
        Posture::LocalOnlyPreview
    } else {
        Posture::ReadyToShare
    }
}

/// Derives the bounded action set from the step posture and evidence sensitivity.
///
/// Reveal-boundary, preview-local-only, edit-selection, and export-step are always offered
/// so the boundary disposition is always inspectable, the same-weight local-only preview is
/// never lost, the selection is always editable, and the step is always exportable as
/// metadata; review-redaction is offered whenever the selection carries sensitive evidence
/// or a review is required; share-report is offered only when the step is ready to share.
fn derive_step_actions(
    step_posture: M5IssueReportBuilderStepPosture,
    carries_sensitive_evidence: bool,
    needs_redaction_review: bool,
) -> Vec<M5IssueReportBuilderStepAction> {
    use M5IssueReportBuilderStepAction as Action;
    let mut actions = vec![Action::RevealEvidenceBoundary, Action::PreviewLocalOnly];
    actions.push(Action::EditEvidenceSelection);
    if carries_sensitive_evidence || needs_redaction_review {
        actions.push(Action::ReviewRedaction);
    }
    if step_posture.permits_boundary_crossing() {
        actions.push(Action::ShareReport);
    }
    actions.push(Action::ExportStep);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked issue-report-builder-step resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepResolutionCase {
    /// The resolver input.
    pub input: M5IssueReportBuilderStepResolutionInput,
    /// The resolved truth. Must equal `resolve_issue_report_builder_step(&input)`.
    pub resolved: M5ResolvedIssueReportBuilderStep,
}

impl M5IssueReportBuilderStepResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5IssueReportBuilderStepResolutionInput) -> Self {
        let resolved =
            resolve_issue_report_builder_step(&input).expect("seed builder step case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_issue_report_builder_step(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved report preserves the input's summary, reproduction steps, and
    /// selected / excluded evidence exactly — never collapsing them into one opaque blob.
    pub fn preserves_report(&self) -> bool {
        self.resolved.summary == self.input.summary
            && self.resolved.repro_steps == self.input.repro_steps
            && self.resolved.selected_evidence == self.input.selected_evidence
            && self.resolved.excluded_evidence == self.input.excluded_evidence
            && self.resolved.step_identity == self.input.step_identity
    }
}

/// One row in the primitive matrix: one support-intake consumer bound to the shared
/// builder-step anatomy, builder step kinds, evidence classes, data-risk classes, redaction
/// states, step postures, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderConsumerRow {
    /// Support-intake consumer family.
    pub consumer_surface: M5IssueReportBuilderConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5SupportQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 support / escalation surface families that render / consume this step.
    pub surface_families: Vec<M5SupportSurfaceFamily>,
    /// Deployment lines this step keeps the same truth across.
    pub deployment_lines: Vec<M5SupportDeploymentLine>,
    /// Anatomy parts this step renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5IssueReportBuilderStepAnatomyPart>,
    /// Builder step kinds this consumer distinguishes.
    pub builder_step_kinds: Vec<M5ReportBuilderStepKind>,
    /// Evidence classes this consumer distinguishes.
    pub evidence_classes: Vec<M5SupportEvidenceClass>,
    /// Data-risk classes this consumer distinguishes.
    pub data_classes: Vec<DataClass>,
    /// Redaction states this consumer distinguishes.
    pub redaction_states: Vec<M5SupportRedactionState>,
    /// Step postures this consumer distinguishes.
    pub step_postures: Vec<M5IssueReportBuilderStepPosture>,
    /// Bounded step actions this consumer offers.
    pub step_actions: Vec<M5IssueReportBuilderStepAction>,
    /// Export fields this step carries (must include the mandatory fields).
    pub export_fields: Vec<M5IssueReportBuilderStepExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5SupportAccessibilityRoute>,
    /// Support / escalation subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5SupportConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5SupportDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked builder-step resolutions proving the resolver on this consumer.
    pub builder_examples: Vec<M5IssueReportBuilderStepResolutionCase>,
    /// Hard invariant: this consumer never masks which evidence class it selects or omits.
    /// MUST be `false`.
    pub masks_evidence_class: bool,
    /// Hard invariant: this consumer never hides its redaction posture. MUST be `false`.
    pub hides_redaction_state: bool,
    /// Hard invariant: this consumer never drops the same-weight local-only preview. MUST
    /// be `false`.
    pub drops_local_only_preview: bool,
    /// Hard invariant: this consumer never collapses the summary / repro steps / evidence
    /// into one opaque report-draft blob. MUST be `false`.
    pub collapses_report_into_blob: bool,
}

impl M5IssueReportBuilderConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5IssueReportBuilderStepAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5IssueReportBuilderStepAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5IssueReportBuilderStepExportField> =
            self.export_fields.iter().copied().collect();
        M5IssueReportBuilderStepExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_evidence_class
            && !self.hides_redaction_state
            && !self.drops_local_only_preview
            && !self.collapses_report_into_blob
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepVocabularySet {
    /// Support-intake-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Step-posture tokens.
    pub step_postures: Vec<String>,
    /// Step-action tokens.
    pub step_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Builder-step-kind tokens (reused from the frozen matrix).
    pub builder_step_kinds: Vec<String>,
    /// Evidence-class tokens (reused from the frozen matrix).
    pub evidence_classes: Vec<String>,
    /// Data-risk-class tokens (reused from the Support Center data-risk vocabulary).
    pub data_classes: Vec<String>,
    /// Redaction-state tokens (reused from the frozen matrix).
    pub redaction_states: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5IssueReportBuilderStepVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5IssueReportBuilderConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5IssueReportBuilderStepAnatomyPart::ALL, |v| v.as_str()),
            step_postures: tokens(&M5IssueReportBuilderStepPosture::ALL, |v| v.as_str()),
            step_actions: tokens(&M5IssueReportBuilderStepAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5IssueReportBuilderStepExportField::ALL, |v| v.as_str()),
            builder_step_kinds: tokens(&M5ReportBuilderStepKind::ALL, |v| v.as_str()),
            evidence_classes: tokens(&M5SupportEvidenceClass::ALL, |v| v.as_str()),
            data_classes: tokens(&DataClass::ALL, |v| v.as_str()),
            redaction_states: tokens(&M5SupportRedactionState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5SupportSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5SupportDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5SupportAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepGovernanceReview {
    /// The builder step shows its human-readable summary and ordered reproduction steps.
    pub builder_step_shows_summary_and_repro_steps: bool,
    /// The builder step shows both its selected and its excluded evidence classes.
    pub builder_step_shows_selected_and_excluded_evidence: bool,
    /// The builder step shows its redaction posture.
    pub builder_step_shows_redaction_posture: bool,
    /// Included and excluded classes use the shared data-risk-class vocabulary Doctor and
    /// support-bundle exports use.
    pub included_and_excluded_use_shared_data_class_vocabulary: bool,
    /// A user can tell exactly which evidence classes will cross the local boundary.
    pub user_can_tell_which_classes_cross_local_boundary: bool,
    /// Reproduction steps and selected evidence survive reopen / export without collapse.
    pub repro_and_evidence_survive_reopen_without_collapse: bool,
    /// The same-weight local-only preview is never dropped.
    pub same_weight_local_only_preview_never_dropped: bool,
    /// A redaction review is required before sensitive evidence can be shared.
    pub redaction_review_required_before_sensitive_share: bool,
    /// Builder steps keep the same truth across every deployment line.
    pub builder_steps_stable_across_deployment_lines: bool,
    /// Builder steps keep the same truth across desktop, headless/export, and support
    /// packet consumers.
    pub builder_steps_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs summary, repro, evidence, and redaction
    /// truth.
    pub support_export_reconstructs_builder_truth: bool,
    /// Later M5 rows cannot invent parallel evidence / redaction vocabulary.
    pub later_rows_cannot_invent_parallel_evidence_vocabulary: bool,
    /// No consumer masks the evidence class or the redaction state.
    pub no_surface_masks_evidence_or_redaction: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepConsumerProjection {
    /// Doctor and support surfaces consume the shared evidence vocabulary.
    pub doctor_and_support_surfaces_consume_evidence_vocabulary: bool,
    /// The step-posture resolver reads a single canonical source.
    pub step_posture_reads_single_source: bool,
    /// The boundary-action derivation reads a single canonical source.
    pub boundary_actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop builders read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the builder step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting support-case audit.
    pub support_case_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5IssueReportBuilderStepPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5IssueReportBuilderStepPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Support-intake rows.
    pub rows: Vec<M5IssueReportBuilderConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5IssueReportBuilderStepVocabularySet,
    /// Governance-review block.
    pub governance_review: M5IssueReportBuilderStepGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5IssueReportBuilderStepConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5IssueReportBuilderStepProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5IssueReportBuilderStepReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 issue-report-builder-step primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IssueReportBuilderStepPacket {
    /// Record kind; must equal [`M5_ISSUE_REPORT_BUILDER_STEP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Support-intake rows.
    pub rows: Vec<M5IssueReportBuilderConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5IssueReportBuilderStepVocabularySet,
    /// Governance-review block.
    pub governance_review: M5IssueReportBuilderStepGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5IssueReportBuilderStepConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5IssueReportBuilderStepProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5IssueReportBuilderStepReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5IssueReportBuilderStepPacket {
    /// Builds an M5 builder-step-primitive packet from stable-lane input.
    pub fn new(input: M5IssueReportBuilderStepPacketInput) -> Self {
        Self {
            record_kind: M5_ISSUE_REPORT_BUILDER_STEP_RECORD_KIND.to_owned(),
            schema_version: M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 builder-step-primitive invariants.
    pub fn validate(&self) -> Vec<M5IssueReportBuilderStepViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ISSUE_REPORT_BUILDER_STEP_RECORD_KIND {
            violations.push(M5IssueReportBuilderStepViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_VERSION {
            violations.push(M5IssueReportBuilderStepViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5IssueReportBuilderStepViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_builder_step_coverage(self, &mut violations);
        validate_evidence_class_coverage(self, &mut violations);
        validate_data_class_coverage(self, &mut violations);
        validate_boundary_coverage(self, &mut violations);
        validate_local_preview_coverage(self, &mut violations);
        validate_share_gating_coverage(self, &mut violations);
        validate_redaction_review_coverage(self, &mut violations);
        validate_report_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 builder step primitive packet serializes"),
        ) {
            violations.push(M5IssueReportBuilderStepViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 builder step primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per support-intake consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,builder_step_kinds,evidence_classes,data_classes,redaction_states,step_postures,step_actions,builder_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.builder_step_kinds, |v| v.as_str()),
                join_tokens(&row.evidence_classes, |v| v.as_str()),
                join_tokens(&row.data_classes, |v| v.as_str()),
                join_tokens(&row.redaction_states, |v| v.as_str()),
                join_tokens(&row.step_postures, |v| v.as_str()),
                join_tokens(&row.step_actions, |v| v.as_str()),
                row.builder_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Issue-Report-Builder-Step Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Support-intake consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Step postures: {}\n",
            self.vocabulary_set.step_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Step actions: {}\n",
            self.vocabulary_set.step_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Evidence classes: {}\n",
            self.vocabulary_set.evidence_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Data-risk classes: {}\n",
            self.vocabulary_set.data_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Support-intake consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked steps: {}\n",
                row.builder_examples.len()
            ));
            for case in &row.builder_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (crosses `{}`, review `{}`, local-only `{}`)\n",
                    case.resolved.step_identity,
                    case.resolved.step_kind.as_str(),
                    case.resolved.step_posture.as_str(),
                    case.resolved.will_cross_local_boundary,
                    case.resolved.needs_redaction_review,
                    case.resolved.local_only_preview_available,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 builder-step-primitive export.
#[derive(Debug)]
pub enum M5IssueReportBuilderStepArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5IssueReportBuilderStepViolation>),
}

impl fmt::Display for M5IssueReportBuilderStepArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 builder step primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 builder step primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5IssueReportBuilderStepArtifactError {}

/// Validation failures emitted by [`M5IssueReportBuilderStepPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5IssueReportBuilderStepViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required support-intake consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A support-intake row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked builder resolutions.
    BuilderExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every builder step kind.
    BuilderStepCoverageUnproven,
    /// The worked resolutions do not exercise every evidence class.
    EvidenceClassCoverageUnproven,
    /// The worked resolutions do not exercise every data-risk class.
    DataClassCoverageUnproven,
    /// The worked resolutions do not prove both a boundary crossing and an exclusion.
    BoundaryCoverageUnproven,
    /// A worked resolution does not offer the same-weight local-only preview.
    LocalPreviewCoverageUnproven,
    /// The worked resolutions do not prove both a ready-to-share and a non-crossing step.
    ShareGatingCoverageUnproven,
    /// The worked resolutions do not prove a redaction-review-required step.
    RedactionReviewCoverageUnproven,
    /// A worked resolution collapses or drops its summary, repro steps, or evidence.
    ReportPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5IssueReportBuilderStepViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::BuilderExampleMissing => "builder_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::BuilderStepCoverageUnproven => "builder_step_coverage_unproven",
            Self::EvidenceClassCoverageUnproven => "evidence_class_coverage_unproven",
            Self::DataClassCoverageUnproven => "data_class_coverage_unproven",
            Self::BoundaryCoverageUnproven => "boundary_coverage_unproven",
            Self::LocalPreviewCoverageUnproven => "local_preview_coverage_unproven",
            Self::ShareGatingCoverageUnproven => "share_gating_coverage_unproven",
            Self::RedactionReviewCoverageUnproven => "redaction_review_coverage_unproven",
            Self::ReportPreservationUnproven => "report_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 builder-step-primitive export.
pub fn current_stable_m5_issue_report_builder_step_export(
) -> Result<M5IssueReportBuilderStepPacket, M5IssueReportBuilderStepArtifactError> {
    let packet: M5IssueReportBuilderStepPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-issue-report-builder-step-primitive-proof/support_export.json"
    )))
    .map_err(M5IssueReportBuilderStepArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5IssueReportBuilderStepArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_DOC_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_COMPONENT_MATRIX_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_DATA_RISK_CLASS_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_EXPORT_REDACTION_PROFILE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5IssueReportBuilderStepViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5IssueReportBuilderStepViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let present: BTreeSet<M5IssueReportBuilderConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5IssueReportBuilderConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5IssueReportBuilderStepViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.builder_step_kinds.is_empty()
            || row.evidence_classes.is_empty()
            || row.data_classes.is_empty()
            || row.redaction_states.is_empty()
            || row.step_postures.is_empty()
            || row.step_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5IssueReportBuilderStepViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5IssueReportBuilderStepViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5IssueReportBuilderStepViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5SupportAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5IssueReportBuilderStepViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5IssueReportBuilderStepViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5IssueReportBuilderStepViolation::DowngradeTriggersMissing);
        }
        if row.builder_examples.is_empty() {
            violations.push(M5IssueReportBuilderStepViolation::BuilderExampleMissing);
        }
        if row
            .builder_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5IssueReportBuilderStepViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5IssueReportBuilderStepViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5IssueReportBuilderStepViolation::RowInvariantViolated);
        }
    }
}

/// Every builder step kind must be exercised by some worked resolution — the
/// implementation requirement that a builder never collapses or skips a step silently.
fn validate_builder_step_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let exercised: BTreeSet<M5ReportBuilderStepKind> = packet
        .rows
        .iter()
        .flat_map(|row| row.builder_examples.iter())
        .map(|case| case.resolved.step_kind)
        .collect();
    let covered = M5ReportBuilderStepKind::ALL
        .iter()
        .all(|kind| exercised.contains(kind));
    if !covered {
        violations.push(M5IssueReportBuilderStepViolation::BuilderStepCoverageUnproven);
    }
}

/// Every evidence class must appear — selected or excluded — in some worked resolution, so
/// the included / excluded vocabulary is proven end to end.
fn validate_evidence_class_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let exercised: BTreeSet<M5SupportEvidenceClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.builder_examples.iter())
        .flat_map(|case| case.resolved.evidence_dispositions.iter())
        .map(|disposition| disposition.evidence_class)
        .collect();
    let covered = M5SupportEvidenceClass::ALL
        .iter()
        .all(|class| exercised.contains(class));
    if !covered {
        violations.push(M5IssueReportBuilderStepViolation::EvidenceClassCoverageUnproven);
    }
}

/// Every data-risk class — metadata-only, environment-adjacent, code-adjacent, and
/// high-risk — must be exercised, so the shared sensitivity vocabulary is proven and a user
/// can see the full range of what may or may not cross the local boundary.
fn validate_data_class_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let exercised: BTreeSet<DataClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.builder_examples.iter())
        .flat_map(|case| case.resolved.evidence_dispositions.iter())
        .map(|disposition| disposition.data_class)
        .collect();
    let covered = DataClass::ALL.iter().all(|class| exercised.contains(class));
    if !covered {
        violations.push(M5IssueReportBuilderStepViolation::DataClassCoverageUnproven);
    }
}

/// At least one worked resolution must prove a class that crosses the local boundary and at
/// least one must prove a class that stays excluded — the acceptance criterion that a user
/// can tell exactly which evidence classes will leave the local boundary and which remain
/// excluded.
fn validate_boundary_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let dispositions = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.builder_examples.iter())
            .flat_map(|case| case.resolved.evidence_dispositions.iter())
    };
    let has_crossing = dispositions().any(|disposition| disposition.crosses_local_boundary);
    let has_excluded = dispositions().any(|disposition| !disposition.selected);
    if !(has_crossing && has_excluded) {
        violations.push(M5IssueReportBuilderStepViolation::BoundaryCoverageUnproven);
    }
}

/// Every worked resolution must offer the same-weight local-only preview — the invariant
/// that a user can inspect their own draft without being forced to share.
fn validate_local_preview_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.builder_examples.iter())
        .all(|case| {
            case.resolved.local_only_preview_available
                && case
                    .resolved
                    .available_actions
                    .contains(&M5IssueReportBuilderStepAction::PreviewLocalOnly)
        });
    if !preserved {
        violations.push(M5IssueReportBuilderStepViolation::LocalPreviewCoverageUnproven);
    }
}

/// At least one worked resolution must prove a ready-to-share step that offers the share
/// action and at least one must prove a step where nothing crosses and the share action is
/// withheld — so a share is never faked and never silently allowed.
fn validate_share_gating_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let has_ready = packet.rows.iter().any(|row| {
        row.builder_examples.iter().any(|case| {
            case.resolved.will_cross_local_boundary
                && case
                    .resolved
                    .available_actions
                    .contains(&M5IssueReportBuilderStepAction::ShareReport)
        })
    });
    let has_withheld = packet.rows.iter().any(|row| {
        row.builder_examples.iter().any(|case| {
            !case.resolved.will_cross_local_boundary
                && !case
                    .resolved
                    .available_actions
                    .contains(&M5IssueReportBuilderStepAction::ShareReport)
        })
    });
    if !(has_ready && has_withheld) {
        violations.push(M5IssueReportBuilderStepViolation::ShareGatingCoverageUnproven);
    }
}

/// At least one worked resolution must prove a redaction-review-required step — the
/// implementation requirement that sensitive evidence forces a redaction review before it
/// can cross the local boundary.
fn validate_redaction_review_coverage(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let has_review = packet.rows.iter().any(|row| {
        row.builder_examples
            .iter()
            .any(|case| case.resolved.needs_redaction_review)
    });
    if !has_review {
        violations.push(M5IssueReportBuilderStepViolation::RedactionReviewCoverageUnproven);
    }
}

/// Every worked resolution must preserve its summary, reproduction steps, and selected /
/// excluded evidence exactly — the acceptance criterion that repro steps and selected
/// evidence survive reopen / export without being collapsed into one opaque report-draft
/// blob.
fn validate_report_preservation(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.builder_examples.iter())
        .all(|case| case.preserves_report());
    if !preserved {
        violations.push(M5IssueReportBuilderStepViolation::ReportPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.builder_step_shows_summary_and_repro_steps,
        review.builder_step_shows_selected_and_excluded_evidence,
        review.builder_step_shows_redaction_posture,
        review.included_and_excluded_use_shared_data_class_vocabulary,
        review.user_can_tell_which_classes_cross_local_boundary,
        review.repro_and_evidence_survive_reopen_without_collapse,
        review.same_weight_local_only_preview_never_dropped,
        review.redaction_review_required_before_sensitive_share,
        review.builder_steps_stable_across_deployment_lines,
        review.builder_steps_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_builder_truth,
        review.later_rows_cannot_invent_parallel_evidence_vocabulary,
        review.no_surface_masks_evidence_or_redaction,
    ] {
        if !ok {
            violations.push(M5IssueReportBuilderStepViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.doctor_and_support_surfaces_consume_evidence_vocabulary,
        projection.step_posture_reads_single_source,
        projection.boundary_actions_read_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5IssueReportBuilderStepViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5IssueReportBuilderStepViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5IssueReportBuilderStepPacket,
    violations: &mut Vec<M5IssueReportBuilderStepViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.support_case_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5IssueReportBuilderStepViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
