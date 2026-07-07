//! Two reusable M5 escalation / handoff primitives — the escalation-packet summary and
//! the handoff-timeline row — so a diagnosis-to-human handoff preserves stable lineage
//! instead of restarting from screenshots and logs:
//!
//! - the escalation-packet summary keeps the packet id, the scenario code, the related
//!   finding / crash ids, the repair attempts, the redaction posture, the build / profile
//!   identity, and the destination class legible, with explicit confirm / cancel actions,
//!   before a packet ever leaves the local boundary; and
//! - the handoff-timeline row keeps an event's identity, its owner at the time, its
//!   related evidence, the current owner, and the next expected human step legible over
//!   time, so a human handoff consumer can reconstruct what was tried and what remains
//!   next without asking the user to restate the case from scratch.
//!
//! Aureline's frozen support-intake / escalation component matrix
//! ([`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`])
//! names the escalation-packet summary and the handoff-timeline row as two governed
//! component families and freezes their controlled vocabulary — the scenario families and
//! Doctor finding families the lineage binds to, the escalation packet destinations, the
//! redaction states, the handoff stages, the next human steps, the approved repair
//! classes, and the shared case dispositions — plus the surface families, deployment
//! lines, consumer surfaces, accessibility routes, qualification classes, and downgrade
//! triggers. This module *implements* that contract as two reusable resolvers so scenario
//! / finding / packet lineage stays continuous from local diagnosis through an exported or
//! shared escalation packet, and a human handoff consumer never loses the owner, the
//! evidence, or the next step between local diagnosis and human handoff.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_escalation_packet_summary`] — takes one packet's id, its scenario family,
//!    its related finding families and opaque evidence ids, its repair attempts, its
//!    redaction posture, its build / profile identity, its destination, its case
//!    disposition, and a share-requested signal, and produces one
//!    [`M5ResolvedEscalationPacketSummary`] carrying the derived summary posture
//!    (escalation-blocked, lineage-incomplete, redaction-review-required, local-only-ready,
//!    or ready-to-escalate), whether the packet will leave the device, whether the scenario
//!    / finding lineage is continuous, and the bounded reveal-lineage / review-redaction /
//!    confirm / cancel / export actions. It never masks the scenario or finding lineage,
//!    never hides the packet destination, never hides the redaction posture, and always
//!    offers a cancel so a user is never trapped mid-escalation.
//! 2. [`resolve_handoff_timeline_row`] — takes one timeline event's identity, its handoff
//!    stage, its owner at the time, its current owner, its related evidence, and its next
//!    expected human step, and produces one [`M5ResolvedHandoffTimelineRow`] carrying the
//!    derived row posture (awaiting-human, ownership-transferred, repair-underway,
//!    case-assembling, or local-diagnosis), whether ownership has transferred, and the
//!    bounded reveal-lineage / view-next-step / contact-current-owner / export actions. It
//!    never drops the next human step and never collapses the owner / current owner /
//!    evidence into one opaque blob.
//!
//! A single parity matrix — [`M5EscalationHandoffPacket`] — binds one row per claimed M5
//! support / escalation consumer (support-center escalation desk, recovery-center handoff,
//! Doctor handoff timeline, headless / CLI escalation, and support-packet export) to the
//! shared escalation-summary and handoff-row anatomy, the same scenario families, finding
//! families, destinations, redaction states, handoff stages, next human steps, approved
//! repair classes, case dispositions, postures, bounded actions, export fields, and
//! non-visual accessibility routes, so the lineage / destination / next-step vocabulary
//! stays identical across desktop, headless / export, and support-packet consumers.
//!
//! The scenario family ([`M5SupportScenarioFamily`]), Doctor finding family
//! ([`M5DoctorFindingFamily`]), escalation packet destination
//! ([`M5EscalationPacketDestination`]), redaction state ([`M5SupportRedactionState`]),
//! handoff stage ([`M5HandoffStage`]), next human step ([`M5NextHumanStep`]), approved
//! repair class ([`M5ApprovedRepairClass`]), case disposition ([`M5SupportCaseDisposition`]),
//! surface family ([`M5SupportSurfaceFamily`]), deployment line
//! ([`M5SupportDeploymentLine`]), consumer surface ([`M5SupportConsumerSurface`]),
//! accessibility route ([`M5SupportAccessibilityRoute`]), qualification class
//! ([`M5SupportQualificationClass`]), and downgrade trigger ([`M5SupportDowngradeTrigger`])
//! are reused verbatim from the frozen matrix so this lane never invents a parallel
//! lineage, destination, or next-step vocabulary. This module mints new vocabulary only for
//! what the matrix left implicit about the two components themselves: their escalation /
//! handoff consumers, their anatomy parts, their derived postures, their bounded actions,
//! and their export fields.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! support boundary; every packet id, evidence id, build / profile identity, event
//! identity, and owner label is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_escalation_handoff_headless_cli_escalation_beta_narrowed,
    seeded_m5_escalation_handoff_packet,
    seeded_m5_escalation_handoff_recovery_center_handoff_preview_narrowed,
    M5_ESCALATION_HANDOFF_PACKET_ID,
};

// The scenario family, Doctor finding family, escalation packet destination, redaction
// state, handoff stage, next human step, approved repair class, case disposition, surface
// family, deployment line, consumer surface, accessibility route, qualification class, and
// downgrade triggers are frozen once, in the support-intake / escalation component matrix.
// This primitive reuses them verbatim so it never invents a parallel lineage / destination
// / next-step vocabulary.
pub use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5ApprovedRepairClass, M5DoctorFindingFamily, M5EscalationPacketDestination, M5HandoffStage,
    M5NextHumanStep, M5SupportAccessibilityRoute, M5SupportCaseDisposition, M5SupportConsumerSurface,
    M5SupportDeploymentLine, M5SupportDowngradeTrigger, M5SupportQualificationClass,
    M5SupportRedactionState, M5SupportScenarioFamily, M5SupportSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5EscalationHandoffPacket`].
pub const M5_ESCALATION_HANDOFF_RECORD_KIND: &str =
    "implement_m5_escalation_packet_summaries_and_handoff_timeline_rows_with_packet_id_scenario_code_finding_repair_lineage_owner_destination_and_next_step_truth_across_claimed_m5_support_lanes";

/// Schema version for M5 escalation-packet-summary / handoff-timeline-row records.
pub const M5_ESCALATION_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the escalation-summary / handoff-row boundary schema.
pub const M5_ESCALATION_HANDOFF_SCHEMA_REF: &str =
    "schemas/ui/m5-support-escalation-packet-summary.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ESCALATION_HANDOFF_DOC_REF: &str =
    "docs/support/m5_support_escalation_packet_summary_handoff_timeline_row_primitive.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix this
/// primitive narrows from.
pub const M5_ESCALATION_HANDOFF_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-matrix.schema.json";

/// Repo-relative path of the escalation-packet contract this primitive binds its packet
/// id, scenario, finding, and destination lineage against.
pub const M5_ESCALATION_HANDOFF_ESCALATION_PACKET_REF: &str =
    "schemas/support/escalation_packet.schema.json";

/// Repo-relative path of the supportability-handoff-packet contract this primitive binds
/// its owner / current-owner / next-step lineage against.
pub const M5_ESCALATION_HANDOFF_HANDOFF_PACKET_REF: &str =
    "schemas/support/m5-supportability-handoff-packets.schema.json";

/// Repo-relative path of the recovery-action contract behind the approved repair classes.
pub const M5_ESCALATION_HANDOFF_RECOVERY_ACTION_REF: &str =
    "schemas/support/recovery_action.schema.json";

/// Repo-relative path of the export-redaction-profile contract this primitive binds its
/// redaction posture against.
pub const M5_ESCALATION_HANDOFF_EXPORT_REDACTION_PROFILE_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the Doctor-finding contract behind the finding-family lineage.
pub const M5_ESCALATION_HANDOFF_DOCTOR_FINDING_REF: &str =
    "schemas/support/doctor_finding.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_ESCALATION_HANDOFF_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-escalation-packet-summary-handoff-timeline-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ESCALATION_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-escalation-packet-summary-handoff-timeline-row-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_ESCALATION_HANDOFF_CSV_REF: &str =
    "artifacts/release/m5-support-escalation-packet-summary-handoff-timeline-row-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ESCALATION_HANDOFF_REPORT_REF: &str =
    "artifacts/design/m5-support-escalation-packet-summary-handoff-timeline-row-primitive.md";

/// One claimed M5 support / escalation consumer that renders the shared escalation-packet
/// summary and handoff-timeline row. These are the consumers the acceptance criteria name
/// — the support-center escalation desk, the recovery-center handoff, the Doctor handoff
/// timeline, the headless / CLI escalation surface, and the support-packet export — so the
/// same lineage / destination / next-step grammar works across every claimed support lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationHandoffConsumerSurface {
    /// The support-center escalation desk.
    SupportCenterEscalationDesk,
    /// The recovery-center handoff surface.
    RecoveryCenterHandoff,
    /// The Project Doctor handoff timeline.
    DoctorHandoffTimeline,
    /// The headless / CLI escalation surface.
    HeadlessCliEscalation,
    /// The support-packet export surface.
    SupportPacketExport,
}

impl M5EscalationHandoffConsumerSurface {
    /// Every claimed escalation / handoff consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SupportCenterEscalationDesk,
        Self::RecoveryCenterHandoff,
        Self::DoctorHandoffTimeline,
        Self::HeadlessCliEscalation,
        Self::SupportPacketExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportCenterEscalationDesk => "support_center_escalation_desk",
            Self::RecoveryCenterHandoff => "recovery_center_handoff",
            Self::DoctorHandoffTimeline => "doctor_handoff_timeline",
            Self::HeadlessCliEscalation => "headless_cli_escalation",
            Self::SupportPacketExport => "support_packet_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SupportCenterEscalationDesk => "Support Center Escalation Desk",
            Self::RecoveryCenterHandoff => "Recovery Center Handoff",
            Self::DoctorHandoffTimeline => "Doctor Handoff Timeline",
            Self::HeadlessCliEscalation => "Headless / CLI Escalation",
            Self::SupportPacketExport => "Support Packet Export",
        }
    }
}

/// True when an escalation destination actually leaves the local device — every
/// destination except the local-only bundle and the blocked destination.
pub const fn destination_leaves_device(destination: M5EscalationPacketDestination) -> bool {
    matches!(
        destination,
        M5EscalationPacketDestination::SelfServeExport
            | M5EscalationPacketDestination::VendorSupportCase
            | M5EscalationPacketDestination::EnterpriseAdmin
            | M5EscalationPacketDestination::CommunityForum
    )
}

// ---- escalation-packet-summary vocabulary -------------------------------

/// The derived posture of an escalation-packet summary — the resolver's verdict about what
/// will happen to the packet at the local boundary. Computed in a fixed blocking-first
/// order, so a blocked or lineage-incomplete packet never reads as ready to escalate, and a
/// packet that stays local never reads as one that has already left the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationPacketSummaryPosture {
    /// The destination is blocked or the export is blocked; nothing leaves, only a local
    /// review remains.
    EscalationBlocked,
    /// The scenario is uncategorized or no finding lineage is bound; the lineage cannot be
    /// continuous, so it must be completed before the packet can escalate.
    LineageIncomplete,
    /// The destination leaves the device under a full-metadata posture; a redaction review
    /// is required before anything crosses.
    RedactionReviewRequired,
    /// The packet stays on the device — a local-only bundle, or a share not yet requested.
    LocalOnlyReady,
    /// The packet's lineage is continuous and it is ready to reach its destination.
    ReadyToEscalate,
}

impl M5EscalationPacketSummaryPosture {
    /// Every summary posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EscalationBlocked,
        Self::LineageIncomplete,
        Self::RedactionReviewRequired,
        Self::LocalOnlyReady,
        Self::ReadyToEscalate,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EscalationBlocked => "escalation_blocked",
            Self::LineageIncomplete => "lineage_incomplete",
            Self::RedactionReviewRequired => "redaction_review_required",
            Self::LocalOnlyReady => "local_only_ready",
            Self::ReadyToEscalate => "ready_to_escalate",
        }
    }

    /// True when the packet will actually leave the device at this posture — only when it
    /// is ready to escalate.
    pub const fn permits_leaving_device(self) -> bool {
        matches!(self, Self::ReadyToEscalate)
    }

    /// True when a redaction review is required before the packet can leave.
    pub const fn needs_redaction_review(self) -> bool {
        matches!(self, Self::RedactionReviewRequired)
    }

    /// True when the summary needs operator attention before the packet can escalate.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::EscalationBlocked | Self::LineageIncomplete | Self::RedactionReviewRequired
        )
    }
}

/// One bounded action an escalation-packet summary offers, so a summary never hides its
/// reveal-lineage / review-redaction / confirm / cancel / export affordances, and always
/// offers a cancel so a user is never trapped mid-escalation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationPacketSummaryAction {
    /// Reveal the scenario / finding / packet lineage.
    RevealLineage,
    /// Review the redaction posture before the packet leaves the device.
    ReviewRedaction,
    /// Confirm the escalation and let the packet reach its destination.
    ConfirmEscalation,
    /// Cancel the escalation and keep the packet local.
    CancelEscalation,
    /// Export the escalation summary as metadata-only support evidence.
    ExportPacket,
}

impl M5EscalationPacketSummaryAction {
    /// Every summary action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealLineage,
        Self::ReviewRedaction,
        Self::ConfirmEscalation,
        Self::CancelEscalation,
        Self::ExportPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealLineage => "reveal_lineage",
            Self::ReviewRedaction => "review_redaction",
            Self::ConfirmEscalation => "confirm_escalation",
            Self::CancelEscalation => "cancel_escalation",
            Self::ExportPacket => "export_packet",
        }
    }
}

/// Controlled escalation-packet-summary anatomy part the shared summary surfaces. The parts
/// in [`M5EscalationPacketSummaryAnatomyPart::MANDATORY`] are required on every summary so
/// the packet id, scenario code, finding / crash lineage, repair attempts, redaction
/// posture, build / profile identity, destination, and confirm / cancel affordance are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationPacketSummaryAnatomyPart {
    /// The packet-id cue.
    PacketIdCue,
    /// The scenario-code cue.
    ScenarioCodeCue,
    /// The related finding / crash lineage cue.
    FindingLineageCue,
    /// The repair-attempts cue.
    RepairAttemptsCue,
    /// The redaction-posture cue.
    RedactionPostureCue,
    /// The build / profile identity cue.
    BuildProfileCue,
    /// The destination-class cue.
    DestinationCue,
    /// The confirm / cancel affordance cue.
    ConfirmCancelCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5EscalationPacketSummaryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PacketIdCue,
        Self::ScenarioCodeCue,
        Self::FindingLineageCue,
        Self::RepairAttemptsCue,
        Self::RedactionPostureCue,
        Self::BuildProfileCue,
        Self::DestinationCue,
        Self::ConfirmCancelCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every summary must render.
    pub const MANDATORY: [Self; 8] = [
        Self::PacketIdCue,
        Self::ScenarioCodeCue,
        Self::FindingLineageCue,
        Self::RepairAttemptsCue,
        Self::RedactionPostureCue,
        Self::BuildProfileCue,
        Self::DestinationCue,
        Self::ConfirmCancelCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketIdCue => "packet_id_cue",
            Self::ScenarioCodeCue => "scenario_code_cue",
            Self::FindingLineageCue => "finding_lineage_cue",
            Self::RepairAttemptsCue => "repair_attempts_cue",
            Self::RedactionPostureCue => "redaction_posture_cue",
            Self::BuildProfileCue => "build_profile_cue",
            Self::DestinationCue => "destination_cue",
            Self::ConfirmCancelCue => "confirm_cancel_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the escalation summary export carries so escalation-packet truth is
/// reconstructable. The fields in [`M5EscalationPacketSummaryExportField::MANDATORY`] are
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationPacketSummaryExportField {
    /// The packet id.
    PacketId,
    /// The scenario family (scenario code).
    ScenarioFamily,
    /// The related finding families.
    FindingFamilies,
    /// The related opaque evidence ids.
    RelatedEvidenceIds,
    /// The repair attempts.
    RepairAttempts,
    /// The redaction posture.
    RedactionState,
    /// The build / profile identity.
    BuildProfileIdentity,
    /// The destination class.
    Destination,
    /// The case disposition.
    CaseDisposition,
    /// The derived summary posture.
    SummaryPosture,
}

impl M5EscalationPacketSummaryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::PacketId,
        Self::ScenarioFamily,
        Self::FindingFamilies,
        Self::RelatedEvidenceIds,
        Self::RepairAttempts,
        Self::RedactionState,
        Self::BuildProfileIdentity,
        Self::Destination,
        Self::CaseDisposition,
        Self::SummaryPosture,
    ];

    /// The export fields every summary must carry.
    pub const MANDATORY: [Self; 9] = [
        Self::PacketId,
        Self::ScenarioFamily,
        Self::FindingFamilies,
        Self::RelatedEvidenceIds,
        Self::RepairAttempts,
        Self::RedactionState,
        Self::BuildProfileIdentity,
        Self::Destination,
        Self::CaseDisposition,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketId => "packet_id",
            Self::ScenarioFamily => "scenario_family",
            Self::FindingFamilies => "finding_families",
            Self::RelatedEvidenceIds => "related_evidence_ids",
            Self::RepairAttempts => "repair_attempts",
            Self::RedactionState => "redaction_state",
            Self::BuildProfileIdentity => "build_profile_identity",
            Self::Destination => "destination",
            Self::CaseDisposition => "case_disposition",
            Self::SummaryPosture => "summary_posture",
        }
    }
}

// ---- handoff-timeline-row vocabulary ------------------------------------

/// The derived posture of a handoff-timeline row — where in the diagnosis-to-handoff
/// timeline the event sits and who must act next. Computed in a fixed order, so an event
/// awaiting a human never reads as still-local, and an event whose ownership has moved
/// never reads as still owned by the reporter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffTimelineRowPosture {
    /// The case is awaiting a human response.
    AwaitingHuman,
    /// Ownership has transferred to a new owner.
    OwnershipTransferred,
    /// A repair is underway (suggested or attempted).
    RepairUnderway,
    /// The case is being assembled.
    CaseAssembling,
    /// Local diagnosis is underway, still owned by the reporter.
    LocalDiagnosis,
}

impl M5HandoffTimelineRowPosture {
    /// Every row posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AwaitingHuman,
        Self::OwnershipTransferred,
        Self::RepairUnderway,
        Self::CaseAssembling,
        Self::LocalDiagnosis,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingHuman => "awaiting_human",
            Self::OwnershipTransferred => "ownership_transferred",
            Self::RepairUnderway => "repair_underway",
            Self::CaseAssembling => "case_assembling",
            Self::LocalDiagnosis => "local_diagnosis",
        }
    }

    /// True when the row is waiting on a human owner to act next.
    pub const fn awaits_human(self) -> bool {
        matches!(self, Self::AwaitingHuman)
    }

    /// True when the row needs a human owner's attention (awaiting or newly transferred).
    pub const fn needs_owner_attention(self) -> bool {
        matches!(self, Self::AwaitingHuman | Self::OwnershipTransferred)
    }
}

/// One bounded action a handoff-timeline row offers, so a row never hides its
/// reveal-lineage / view-next-step / contact-owner / export affordances, and never drops
/// the next human step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffTimelineRowAction {
    /// Reveal the event / owner / evidence lineage.
    RevealHandoffLineage,
    /// View the next expected human step.
    ViewNextStep,
    /// Contact the current owner of the case.
    ContactCurrentOwner,
    /// Export the timeline row as metadata-only support evidence.
    ExportRow,
}

impl M5HandoffTimelineRowAction {
    /// Every row action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RevealHandoffLineage,
        Self::ViewNextStep,
        Self::ContactCurrentOwner,
        Self::ExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealHandoffLineage => "reveal_handoff_lineage",
            Self::ViewNextStep => "view_next_step",
            Self::ContactCurrentOwner => "contact_current_owner",
            Self::ExportRow => "export_row",
        }
    }
}

/// Controlled handoff-timeline-row anatomy part the shared row surfaces. The parts in
/// [`M5HandoffTimelineRowAnatomyPart::MANDATORY`] are required on every row so the event
/// identity, handoff stage, owner, current owner, related evidence, and next step are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffTimelineRowAnatomyPart {
    /// The event-identity cue.
    EventIdentityCue,
    /// The handoff-stage cue.
    HandoffStageCue,
    /// The owner-at-the-time cue.
    OwnerCue,
    /// The current-owner cue.
    CurrentOwnerCue,
    /// The related-evidence cue.
    RelatedEvidenceCue,
    /// The next-step cue.
    NextStepCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5HandoffTimelineRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EventIdentityCue,
        Self::HandoffStageCue,
        Self::OwnerCue,
        Self::CurrentOwnerCue,
        Self::RelatedEvidenceCue,
        Self::NextStepCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every row must render.
    pub const MANDATORY: [Self; 6] = [
        Self::EventIdentityCue,
        Self::HandoffStageCue,
        Self::OwnerCue,
        Self::CurrentOwnerCue,
        Self::RelatedEvidenceCue,
        Self::NextStepCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventIdentityCue => "event_identity_cue",
            Self::HandoffStageCue => "handoff_stage_cue",
            Self::OwnerCue => "owner_cue",
            Self::CurrentOwnerCue => "current_owner_cue",
            Self::RelatedEvidenceCue => "related_evidence_cue",
            Self::NextStepCue => "next_step_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the handoff row export carries so handoff-timeline truth is reconstructable. The
/// fields in [`M5HandoffTimelineRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffTimelineRowExportField {
    /// The event identity.
    EventIdentity,
    /// The handoff stage.
    HandoffStage,
    /// The owner at the time.
    OwnerRole,
    /// The current owner.
    CurrentOwnerRole,
    /// The related opaque evidence ids.
    RelatedEvidenceIds,
    /// The next expected human step.
    NextStep,
    /// Whether ownership has transferred.
    OwnershipTransferred,
    /// The derived row posture.
    RowPosture,
}

impl M5HandoffTimelineRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EventIdentity,
        Self::HandoffStage,
        Self::OwnerRole,
        Self::CurrentOwnerRole,
        Self::RelatedEvidenceIds,
        Self::NextStep,
        Self::OwnershipTransferred,
        Self::RowPosture,
    ];

    /// The export fields every row must carry.
    pub const MANDATORY: [Self; 7] = [
        Self::EventIdentity,
        Self::HandoffStage,
        Self::OwnerRole,
        Self::CurrentOwnerRole,
        Self::RelatedEvidenceIds,
        Self::NextStep,
        Self::OwnershipTransferred,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventIdentity => "event_identity",
            Self::HandoffStage => "handoff_stage",
            Self::OwnerRole => "owner_role",
            Self::CurrentOwnerRole => "current_owner_role",
            Self::RelatedEvidenceIds => "related_evidence_ids",
            Self::NextStep => "next_step",
            Self::OwnershipTransferred => "ownership_transferred",
            Self::RowPosture => "row_posture",
        }
    }
}

// ---- escalation-packet-summary resolver ---------------------------------

/// The full input to the escalation-packet-summary resolver for one packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationPacketSummaryResolutionInput {
    /// The opaque stable packet id (must be non-empty).
    pub packet_id: String,
    /// The scenario family (scenario code) behind the packet.
    pub scenario_family: M5SupportScenarioFamily,
    /// The related Doctor finding families (the finding lineage).
    pub finding_families: Vec<M5DoctorFindingFamily>,
    /// The related opaque finding / crash evidence ids (each must be non-empty when present).
    pub related_evidence_ids: Vec<String>,
    /// The repair attempts made before escalation.
    pub repair_attempts: Vec<M5ApprovedRepairClass>,
    /// The redaction posture the export will apply.
    pub redaction_state: M5SupportRedactionState,
    /// The opaque build / profile identity (must be non-empty).
    pub build_profile_identity: String,
    /// The destination the packet is bound for.
    pub destination: M5EscalationPacketDestination,
    /// The shared case disposition.
    pub case_disposition: M5SupportCaseDisposition,
    /// True when the user has requested the escalation leave the local boundary.
    pub share_requested: bool,
}

/// The resolved escalation-packet-summary truth for one packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEscalationPacketSummary {
    /// The opaque stable packet id, preserved exactly from the input.
    pub packet_id: String,
    /// The scenario family, preserved exactly from the input.
    pub scenario_family: M5SupportScenarioFamily,
    /// The related finding families, preserved exactly from the input.
    pub finding_families: Vec<M5DoctorFindingFamily>,
    /// The related evidence ids, preserved exactly from the input.
    pub related_evidence_ids: Vec<String>,
    /// The repair attempts, preserved exactly from the input.
    pub repair_attempts: Vec<M5ApprovedRepairClass>,
    /// The redaction posture, preserved exactly from the input.
    pub redaction_state: M5SupportRedactionState,
    /// The build / profile identity, preserved exactly from the input.
    pub build_profile_identity: String,
    /// The destination, preserved exactly from the input.
    pub destination: M5EscalationPacketDestination,
    /// The case disposition, preserved exactly from the input.
    pub case_disposition: M5SupportCaseDisposition,
    /// The derived summary posture.
    pub summary_posture: M5EscalationPacketSummaryPosture,
    /// The bounded actions this summary offers.
    pub available_actions: Vec<M5EscalationPacketSummaryAction>,
    /// True when the packet will actually leave the device at this posture.
    pub will_leave_device: bool,
    /// True when a redaction review is required before the packet can leave.
    pub needs_redaction_review: bool,
    /// True when the scenario / finding lineage is continuous (a committed scenario with at
    /// least one bound finding family). The core AC-1 signal.
    pub lineage_continuous: bool,
    /// True when the confirm-escalation action is offered (only when ready to escalate).
    pub confirm_available: bool,
    /// True when the summary needs operator attention before the packet can escalate.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_escalation_packet_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EscalationPacketSummaryResolutionError {
    /// The packet id was empty.
    EmptyPacketId,
    /// The build / profile identity was empty.
    EmptyBuildProfileIdentity,
    /// A related evidence id was blank.
    EmptyEvidenceId,
    /// A packet descriptor carried forbidden material.
    ForbiddenPacketMaterial,
}

impl M5EscalationPacketSummaryResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPacketId => "empty_packet_id",
            Self::EmptyBuildProfileIdentity => "empty_build_profile_identity",
            Self::EmptyEvidenceId => "empty_evidence_id",
            Self::ForbiddenPacketMaterial => "forbidden_packet_material",
        }
    }
}

impl fmt::Display for M5EscalationPacketSummaryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "escalation packet summary resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5EscalationPacketSummaryResolutionError {}

/// Resolves one escalation-packet summary from its declared state.
///
/// The derived summary posture is computed in a fixed blocking-first order: a blocked
/// destination or blocked export wins first (nothing leaves, only a local review remains),
/// then a packet whose scenario is uncategorized or whose finding lineage is empty (the
/// lineage cannot be continuous), then a packet bound for a device-leaving destination
/// under a full-metadata posture (a redaction review is required first), then a packet that
/// stays on the device (a local-only bundle or a share not yet requested), and otherwise a
/// packet ready to escalate. The packet id, scenario code, finding lineage, evidence ids,
/// repair attempts, build / profile identity, destination, and case disposition are carried
/// explicitly and never collapsed into one opaque blob; the summary always offers a cancel
/// so a user is never trapped mid-escalation, and only offers confirm when the packet is
/// genuinely ready to escalate.
pub fn resolve_escalation_packet_summary(
    input: &M5EscalationPacketSummaryResolutionInput,
) -> Result<M5ResolvedEscalationPacketSummary, M5EscalationPacketSummaryResolutionError> {
    if input.packet_id.trim().is_empty() {
        return Err(M5EscalationPacketSummaryResolutionError::EmptyPacketId);
    }
    if input.build_profile_identity.trim().is_empty() {
        return Err(M5EscalationPacketSummaryResolutionError::EmptyBuildProfileIdentity);
    }
    if input
        .related_evidence_ids
        .iter()
        .any(|id| id.trim().is_empty())
    {
        return Err(M5EscalationPacketSummaryResolutionError::EmptyEvidenceId);
    }
    if value_repr_is_forbidden(&input.packet_id)
        || value_repr_is_forbidden(&input.build_profile_identity)
        || input
            .related_evidence_ids
            .iter()
            .any(|id| value_repr_is_forbidden(id))
    {
        return Err(M5EscalationPacketSummaryResolutionError::ForbiddenPacketMaterial);
    }

    let lineage_continuous = !matches!(
        input.scenario_family,
        M5SupportScenarioFamily::UncategorizedScenario
    ) && !input.finding_families.is_empty();
    let summary_posture = derive_summary_posture(
        input.scenario_family,
        &input.finding_families,
        input.redaction_state,
        input.destination,
        input.share_requested,
    );
    let will_leave_device = summary_posture.permits_leaving_device();
    let needs_redaction_review = summary_posture.needs_redaction_review();
    let confirm_available = matches!(
        summary_posture,
        M5EscalationPacketSummaryPosture::ReadyToEscalate
    );
    let available_actions = derive_summary_actions(summary_posture, input.destination);

    Ok(M5ResolvedEscalationPacketSummary {
        packet_id: input.packet_id.clone(),
        scenario_family: input.scenario_family,
        finding_families: input.finding_families.clone(),
        related_evidence_ids: input.related_evidence_ids.clone(),
        repair_attempts: input.repair_attempts.clone(),
        redaction_state: input.redaction_state,
        build_profile_identity: input.build_profile_identity.clone(),
        destination: input.destination,
        case_disposition: input.case_disposition,
        summary_posture,
        available_actions,
        will_leave_device,
        needs_redaction_review,
        lineage_continuous,
        confirm_available,
        needs_attention: summary_posture.needs_attention(),
    })
}

/// The fixed blocking-first summary-posture ladder.
fn derive_summary_posture(
    scenario_family: M5SupportScenarioFamily,
    finding_families: &[M5DoctorFindingFamily],
    redaction_state: M5SupportRedactionState,
    destination: M5EscalationPacketDestination,
    share_requested: bool,
) -> M5EscalationPacketSummaryPosture {
    use M5EscalationPacketSummaryPosture as Posture;
    if matches!(
        destination,
        M5EscalationPacketDestination::BlockedDestination
    ) || matches!(redaction_state, M5SupportRedactionState::ExportBlocked)
    {
        Posture::EscalationBlocked
    } else if matches!(
        scenario_family,
        M5SupportScenarioFamily::UncategorizedScenario
    ) || finding_families.is_empty()
    {
        Posture::LineageIncomplete
    } else if destination_leaves_device(destination)
        && matches!(redaction_state, M5SupportRedactionState::FullMetadata)
    {
        Posture::RedactionReviewRequired
    } else if matches!(destination, M5EscalationPacketDestination::LocalOnlyBundle)
        || !share_requested
    {
        Posture::LocalOnlyReady
    } else {
        Posture::ReadyToEscalate
    }
}

/// Derives the bounded action set from the summary posture and destination.
///
/// Reveal-lineage, cancel, and export are always offered so the lineage is always
/// inspectable, a user is never trapped mid-escalation, and the summary is always
/// exportable as metadata; review-redaction is offered whenever a review is required or the
/// destination leaves the device; confirm-escalation is offered only when the packet is
/// ready to escalate.
fn derive_summary_actions(
    summary_posture: M5EscalationPacketSummaryPosture,
    destination: M5EscalationPacketDestination,
) -> Vec<M5EscalationPacketSummaryAction> {
    use M5EscalationPacketSummaryAction as Action;
    let mut actions = vec![Action::RevealLineage];
    if summary_posture.needs_redaction_review() || destination_leaves_device(destination) {
        actions.push(Action::ReviewRedaction);
    }
    if summary_posture.permits_leaving_device() {
        actions.push(Action::ConfirmEscalation);
    }
    actions.push(Action::CancelEscalation);
    actions.push(Action::ExportPacket);
    actions
}

// ---- handoff-timeline-row resolver --------------------------------------

/// The full input to the handoff-timeline-row resolver for one event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffTimelineRowResolutionInput {
    /// The opaque stable event identity (must be non-empty).
    pub event_identity: String,
    /// The handoff stage this event sits at.
    pub stage: M5HandoffStage,
    /// The opaque owner label at the time of the event (must be non-empty).
    pub owner_role: String,
    /// The opaque current-owner label (must be non-empty).
    pub current_owner_role: String,
    /// The related opaque evidence ids (each must be non-empty when present).
    pub related_evidence_ids: Vec<String>,
    /// The next expected human step.
    pub next_step: M5NextHumanStep,
}

/// The resolved handoff-timeline-row truth for one event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedHandoffTimelineRow {
    /// The opaque event identity, preserved exactly from the input.
    pub event_identity: String,
    /// The handoff stage, preserved exactly from the input.
    pub stage: M5HandoffStage,
    /// The owner at the time, preserved exactly from the input.
    pub owner_role: String,
    /// The current owner, preserved exactly from the input.
    pub current_owner_role: String,
    /// The related evidence ids, preserved exactly from the input.
    pub related_evidence_ids: Vec<String>,
    /// The next expected human step, preserved exactly from the input.
    pub next_step: M5NextHumanStep,
    /// The derived row posture.
    pub row_posture: M5HandoffTimelineRowPosture,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5HandoffTimelineRowAction>,
    /// True when the current owner differs from the owner at the time of the event.
    pub ownership_transferred: bool,
    /// True when the row is awaiting a human response.
    pub awaiting_human: bool,
    /// True when the next expected human step is explicit (always `true`: the next step is
    /// a typed value and never a dead end). The core AC-2 signal.
    pub next_step_explicit: bool,
    /// True when the row needs a human owner's attention.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_handoff_timeline_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HandoffTimelineRowResolutionError {
    /// The event identity was empty.
    EmptyEventIdentity,
    /// The owner label was empty.
    EmptyOwnerRole,
    /// The current-owner label was empty.
    EmptyCurrentOwnerRole,
    /// A related evidence id was blank.
    EmptyEvidenceId,
    /// A row descriptor carried forbidden material.
    ForbiddenRowMaterial,
}

impl M5HandoffTimelineRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyEventIdentity => "empty_event_identity",
            Self::EmptyOwnerRole => "empty_owner_role",
            Self::EmptyCurrentOwnerRole => "empty_current_owner_role",
            Self::EmptyEvidenceId => "empty_evidence_id",
            Self::ForbiddenRowMaterial => "forbidden_row_material",
        }
    }
}

impl fmt::Display for M5HandoffTimelineRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "handoff timeline row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5HandoffTimelineRowResolutionError {}

/// Resolves one handoff-timeline row from its declared state.
///
/// The derived row posture is computed in a fixed order: an event awaiting a human wins
/// first, then an event that has been handed off (or whose current owner differs from the
/// owner at the time), then an event with a repair underway (suggested or attempted), then
/// an event whose case is being assembled, and otherwise an event still in local diagnosis.
/// The event identity, handoff stage, owner at the time, current owner, related evidence,
/// and next step are carried explicitly and never collapsed into one opaque blob; the next
/// expected human step is always preserved so a handoff consumer can reconstruct what
/// remains next without asking the user to restate the case.
pub fn resolve_handoff_timeline_row(
    input: &M5HandoffTimelineRowResolutionInput,
) -> Result<M5ResolvedHandoffTimelineRow, M5HandoffTimelineRowResolutionError> {
    if input.event_identity.trim().is_empty() {
        return Err(M5HandoffTimelineRowResolutionError::EmptyEventIdentity);
    }
    if input.owner_role.trim().is_empty() {
        return Err(M5HandoffTimelineRowResolutionError::EmptyOwnerRole);
    }
    if input.current_owner_role.trim().is_empty() {
        return Err(M5HandoffTimelineRowResolutionError::EmptyCurrentOwnerRole);
    }
    if input
        .related_evidence_ids
        .iter()
        .any(|id| id.trim().is_empty())
    {
        return Err(M5HandoffTimelineRowResolutionError::EmptyEvidenceId);
    }
    if value_repr_is_forbidden(&input.event_identity)
        || value_repr_is_forbidden(&input.owner_role)
        || value_repr_is_forbidden(&input.current_owner_role)
        || input
            .related_evidence_ids
            .iter()
            .any(|id| value_repr_is_forbidden(id))
    {
        return Err(M5HandoffTimelineRowResolutionError::ForbiddenRowMaterial);
    }

    let ownership_transferred = input.current_owner_role.trim() != input.owner_role.trim();
    let row_posture = derive_row_posture(input.stage, ownership_transferred);
    let available_actions = derive_row_actions(row_posture);

    Ok(M5ResolvedHandoffTimelineRow {
        event_identity: input.event_identity.clone(),
        stage: input.stage,
        owner_role: input.owner_role.clone(),
        current_owner_role: input.current_owner_role.clone(),
        related_evidence_ids: input.related_evidence_ids.clone(),
        next_step: input.next_step,
        row_posture,
        available_actions,
        ownership_transferred,
        awaiting_human: row_posture.awaits_human(),
        next_step_explicit: true,
        needs_attention: row_posture.needs_owner_attention(),
    })
}

/// The fixed row-posture ladder.
fn derive_row_posture(
    stage: M5HandoffStage,
    ownership_transferred: bool,
) -> M5HandoffTimelineRowPosture {
    use M5HandoffTimelineRowPosture as Posture;
    match stage {
        M5HandoffStage::AwaitingHuman => Posture::AwaitingHuman,
        M5HandoffStage::HandedOff => Posture::OwnershipTransferred,
        _ if ownership_transferred => Posture::OwnershipTransferred,
        M5HandoffStage::RepairSuggested | M5HandoffStage::RepairAttempted => {
            Posture::RepairUnderway
        }
        M5HandoffStage::CaseBuilt => Posture::CaseAssembling,
        M5HandoffStage::DiagnosisStarted => Posture::LocalDiagnosis,
    }
}

/// Derives the bounded action set from the row posture.
///
/// Reveal-lineage, view-next-step, and export are always offered so the lineage is always
/// inspectable, the next step is always legible, and the row is always exportable as
/// metadata; contact-current-owner is offered whenever the row needs a human owner's
/// attention (awaiting a human, or ownership just transferred).
fn derive_row_actions(row_posture: M5HandoffTimelineRowPosture) -> Vec<M5HandoffTimelineRowAction> {
    use M5HandoffTimelineRowAction as Action;
    let mut actions = vec![Action::RevealHandoffLineage, Action::ViewNextStep];
    if row_posture.needs_owner_attention() {
        actions.push(Action::ContactCurrentOwner);
    }
    actions.push(Action::ExportRow);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked escalation-packet-summary resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationPacketSummaryResolutionCase {
    /// The resolver input.
    pub input: M5EscalationPacketSummaryResolutionInput,
    /// The resolved truth. Must equal `resolve_escalation_packet_summary(&input)`.
    pub resolved: M5ResolvedEscalationPacketSummary,
}

impl M5EscalationPacketSummaryResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5EscalationPacketSummaryResolutionInput) -> Self {
        let resolved = resolve_escalation_packet_summary(&input)
            .expect("seed escalation summary case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_escalation_packet_summary(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved summary preserves the input's packet id, scenario, finding
    /// lineage, evidence ids, repair attempts, build / profile identity, destination, and
    /// disposition exactly — never collapsing them into one opaque blob.
    pub fn preserves_lineage(&self) -> bool {
        self.resolved.packet_id == self.input.packet_id
            && self.resolved.scenario_family == self.input.scenario_family
            && self.resolved.finding_families == self.input.finding_families
            && self.resolved.related_evidence_ids == self.input.related_evidence_ids
            && self.resolved.repair_attempts == self.input.repair_attempts
            && self.resolved.build_profile_identity == self.input.build_profile_identity
            && self.resolved.destination == self.input.destination
            && self.resolved.case_disposition == self.input.case_disposition
    }
}

/// One worked handoff-timeline-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffTimelineRowResolutionCase {
    /// The resolver input.
    pub input: M5HandoffTimelineRowResolutionInput,
    /// The resolved truth. Must equal `resolve_handoff_timeline_row(&input)`.
    pub resolved: M5ResolvedHandoffTimelineRow,
}

impl M5HandoffTimelineRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5HandoffTimelineRowResolutionInput) -> Self {
        let resolved =
            resolve_handoff_timeline_row(&input).expect("seed handoff timeline row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_handoff_timeline_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved row preserves the input's event identity, owner, current
    /// owner, related evidence, and next step exactly — never collapsing them into one
    /// opaque blob and never dropping the next step.
    pub fn preserves_lineage(&self) -> bool {
        self.resolved.event_identity == self.input.event_identity
            && self.resolved.owner_role == self.input.owner_role
            && self.resolved.current_owner_role == self.input.current_owner_role
            && self.resolved.related_evidence_ids == self.input.related_evidence_ids
            && self.resolved.next_step == self.input.next_step
    }
}

/// One row in the primitive matrix: one escalation / handoff consumer bound to the shared
/// escalation-summary and handoff-row anatomy, scenario families, finding families,
/// destinations, redaction states, handoff stages, next human steps, approved repair
/// classes, case dispositions, postures, bounded actions, export fields, and accessibility
/// routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationHandoffConsumerRow {
    /// Escalation / handoff consumer family.
    pub consumer_surface: M5EscalationHandoffConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5SupportQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 support / escalation surface families that render / consume these
    /// components.
    pub surface_families: Vec<M5SupportSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5SupportDeploymentLine>,
    /// Escalation-summary anatomy parts this consumer renders (must include the mandatory
    /// parts).
    pub escalation_anatomy_parts: Vec<M5EscalationPacketSummaryAnatomyPart>,
    /// Handoff-row anatomy parts this consumer renders (must include the mandatory parts).
    pub handoff_anatomy_parts: Vec<M5HandoffTimelineRowAnatomyPart>,
    /// Scenario families this consumer distinguishes.
    pub scenario_families: Vec<M5SupportScenarioFamily>,
    /// Doctor finding families this consumer distinguishes.
    pub finding_families: Vec<M5DoctorFindingFamily>,
    /// Escalation packet destinations this consumer distinguishes.
    pub destinations: Vec<M5EscalationPacketDestination>,
    /// Redaction states this consumer distinguishes.
    pub redaction_states: Vec<M5SupportRedactionState>,
    /// Handoff stages this consumer distinguishes.
    pub handoff_stages: Vec<M5HandoffStage>,
    /// Next human steps this consumer distinguishes.
    pub next_human_steps: Vec<M5NextHumanStep>,
    /// Approved repair classes this consumer distinguishes.
    pub approved_repair_classes: Vec<M5ApprovedRepairClass>,
    /// Case dispositions this consumer distinguishes.
    pub case_dispositions: Vec<M5SupportCaseDisposition>,
    /// Escalation-summary postures this consumer distinguishes.
    pub summary_postures: Vec<M5EscalationPacketSummaryPosture>,
    /// Bounded escalation-summary actions this consumer offers.
    pub summary_actions: Vec<M5EscalationPacketSummaryAction>,
    /// Handoff-row postures this consumer distinguishes.
    pub row_postures: Vec<M5HandoffTimelineRowPosture>,
    /// Bounded handoff-row actions this consumer offers.
    pub row_actions: Vec<M5HandoffTimelineRowAction>,
    /// Escalation-summary export fields this consumer carries (must include the mandatory
    /// fields).
    pub summary_export_fields: Vec<M5EscalationPacketSummaryExportField>,
    /// Handoff-row export fields this consumer carries (must include the mandatory fields).
    pub row_export_fields: Vec<M5HandoffTimelineRowExportField>,
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
    /// Worked escalation-summary resolutions proving the resolver on this consumer.
    pub escalation_examples: Vec<M5EscalationPacketSummaryResolutionCase>,
    /// Worked handoff-row resolutions proving the resolver on this consumer.
    pub handoff_examples: Vec<M5HandoffTimelineRowResolutionCase>,
    /// Hard invariant: this consumer never masks the scenario or finding lineage. MUST be
    /// `false`.
    pub masks_scenario_or_finding_lineage: bool,
    /// Hard invariant: this consumer never hides the packet destination. MUST be `false`.
    pub hides_packet_destination: bool,
    /// Hard invariant: this consumer never drops the next human step. MUST be `false`.
    pub drops_next_human_step: bool,
    /// Hard invariant: this consumer never collapses the case lineage into one opaque blob.
    /// MUST be `false`.
    pub collapses_case_into_blob: bool,
}

impl M5EscalationHandoffConsumerRow {
    /// True when the row declares every mandatory escalation and handoff anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let escalation: BTreeSet<M5EscalationPacketSummaryAnatomyPart> =
            self.escalation_anatomy_parts.iter().copied().collect();
        let handoff: BTreeSet<M5HandoffTimelineRowAnatomyPart> =
            self.handoff_anatomy_parts.iter().copied().collect();
        M5EscalationPacketSummaryAnatomyPart::MANDATORY
            .iter()
            .all(|part| escalation.contains(part))
            && M5HandoffTimelineRowAnatomyPart::MANDATORY
                .iter()
                .all(|part| handoff.contains(part))
    }

    /// True when the row declares every mandatory escalation and handoff export field.
    fn declares_mandatory_export(&self) -> bool {
        let escalation: BTreeSet<M5EscalationPacketSummaryExportField> =
            self.summary_export_fields.iter().copied().collect();
        let handoff: BTreeSet<M5HandoffTimelineRowExportField> =
            self.row_export_fields.iter().copied().collect();
        M5EscalationPacketSummaryExportField::MANDATORY
            .iter()
            .all(|field| escalation.contains(field))
            && M5HandoffTimelineRowExportField::MANDATORY
                .iter()
                .all(|field| handoff.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_scenario_or_finding_lineage
            && !self.hides_packet_destination
            && !self.drops_next_human_step
            && !self.collapses_case_into_blob
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationHandoffVocabularySet {
    /// Escalation / handoff consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Escalation-summary anatomy-part tokens.
    pub escalation_anatomy_parts: Vec<String>,
    /// Handoff-row anatomy-part tokens.
    pub handoff_anatomy_parts: Vec<String>,
    /// Escalation-summary posture tokens.
    pub summary_postures: Vec<String>,
    /// Escalation-summary action tokens.
    pub summary_actions: Vec<String>,
    /// Handoff-row posture tokens.
    pub row_postures: Vec<String>,
    /// Handoff-row action tokens.
    pub row_actions: Vec<String>,
    /// Escalation-summary export-field tokens.
    pub summary_export_fields: Vec<String>,
    /// Handoff-row export-field tokens.
    pub row_export_fields: Vec<String>,
    /// Scenario-family tokens (reused from the frozen matrix).
    pub scenario_families: Vec<String>,
    /// Doctor finding-family tokens (reused from the frozen matrix).
    pub finding_families: Vec<String>,
    /// Escalation packet destination tokens (reused from the frozen matrix).
    pub destinations: Vec<String>,
    /// Redaction-state tokens (reused from the frozen matrix).
    pub redaction_states: Vec<String>,
    /// Handoff-stage tokens (reused from the frozen matrix).
    pub handoff_stages: Vec<String>,
    /// Next-human-step tokens (reused from the frozen matrix).
    pub next_human_steps: Vec<String>,
    /// Approved-repair-class tokens (reused from the frozen matrix).
    pub approved_repair_classes: Vec<String>,
    /// Case-disposition tokens (reused from the frozen matrix).
    pub case_dispositions: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5EscalationHandoffVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5EscalationHandoffConsumerSurface::ALL, |v| v.as_str()),
            escalation_anatomy_parts: tokens(&M5EscalationPacketSummaryAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            handoff_anatomy_parts: tokens(&M5HandoffTimelineRowAnatomyPart::ALL, |v| v.as_str()),
            summary_postures: tokens(&M5EscalationPacketSummaryPosture::ALL, |v| v.as_str()),
            summary_actions: tokens(&M5EscalationPacketSummaryAction::ALL, |v| v.as_str()),
            row_postures: tokens(&M5HandoffTimelineRowPosture::ALL, |v| v.as_str()),
            row_actions: tokens(&M5HandoffTimelineRowAction::ALL, |v| v.as_str()),
            summary_export_fields: tokens(&M5EscalationPacketSummaryExportField::ALL, |v| {
                v.as_str()
            }),
            row_export_fields: tokens(&M5HandoffTimelineRowExportField::ALL, |v| v.as_str()),
            scenario_families: tokens(&M5SupportScenarioFamily::ALL, |v| v.as_str()),
            finding_families: tokens(&M5DoctorFindingFamily::ALL, |v| v.as_str()),
            destinations: tokens(&M5EscalationPacketDestination::ALL, |v| v.as_str()),
            redaction_states: tokens(&M5SupportRedactionState::ALL, |v| v.as_str()),
            handoff_stages: tokens(&M5HandoffStage::ALL, |v| v.as_str()),
            next_human_steps: tokens(&M5NextHumanStep::ALL, |v| v.as_str()),
            approved_repair_classes: tokens(&M5ApprovedRepairClass::ALL, |v| v.as_str()),
            case_dispositions: tokens(&M5SupportCaseDisposition::ALL, |v| v.as_str()),
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
pub struct M5EscalationHandoffGovernanceReview {
    /// The escalation summary shows its packet id, scenario code, and finding / crash
    /// lineage.
    pub summary_shows_packet_scenario_and_finding_lineage: bool,
    /// The escalation summary shows its repair attempts and redaction posture.
    pub summary_shows_repair_attempts_and_redaction: bool,
    /// The escalation summary shows its build / profile identity and destination class.
    pub summary_shows_build_profile_and_destination: bool,
    /// The escalation summary always offers confirm / cancel affordances.
    pub summary_always_offers_confirm_and_cancel: bool,
    /// Scenario / finding / packet lineage stays continuous from local diagnosis through an
    /// exported or shared escalation packet.
    pub lineage_continuous_from_diagnosis_through_export: bool,
    /// The handoff row preserves the event identity, owner, and current owner.
    pub handoff_row_preserves_identity_and_owners: bool,
    /// The handoff row preserves the related evidence and the next expected step.
    pub handoff_row_preserves_evidence_and_next_step: bool,
    /// A human handoff consumer can reconstruct what was tried and what remains next.
    pub handoff_consumer_can_reconstruct_case: bool,
    /// A redaction review is required before a device-leaving packet under a full-metadata
    /// posture can escalate.
    pub redaction_review_required_before_leaving_device: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across desktop, headless / export, and support
    /// packet consumers.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs the escalation and handoff truth.
    pub support_export_reconstructs_escalation_and_handoff_truth: bool,
    /// Later M5 rows cannot invent parallel lineage / destination / next-step vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
    /// No consumer masks the scenario / finding lineage, the destination, or the next step.
    pub no_surface_masks_lineage_destination_or_next_step: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationHandoffConsumerProjection {
    /// Doctor and support surfaces consume the shared lineage vocabulary.
    pub doctor_and_support_surfaces_consume_lineage_vocabulary: bool,
    /// The summary-posture resolver reads a single canonical source.
    pub summary_posture_reads_single_source: bool,
    /// The row-posture resolver reads a single canonical source.
    pub row_posture_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop consumers read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationHandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the two components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationHandoffReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting support-case audit.
    pub support_case_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EscalationHandoffPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EscalationHandoffPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Escalation / handoff rows.
    pub rows: Vec<M5EscalationHandoffConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EscalationHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EscalationHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EscalationHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EscalationHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EscalationHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 escalation-packet-summary / handoff-timeline-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EscalationHandoffPacket {
    /// Record kind; must equal [`M5_ESCALATION_HANDOFF_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ESCALATION_HANDOFF_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Escalation / handoff rows.
    pub rows: Vec<M5EscalationHandoffConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EscalationHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EscalationHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EscalationHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EscalationHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EscalationHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EscalationHandoffPacket {
    /// Builds an M5 escalation / handoff primitive packet from stable-lane input.
    pub fn new(input: M5EscalationHandoffPacketInput) -> Self {
        Self {
            record_kind: M5_ESCALATION_HANDOFF_RECORD_KIND.to_owned(),
            schema_version: M5_ESCALATION_HANDOFF_SCHEMA_VERSION,
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

    /// Validates the M5 escalation / handoff primitive invariants.
    pub fn validate(&self) -> Vec<M5EscalationHandoffViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ESCALATION_HANDOFF_RECORD_KIND {
            violations.push(M5EscalationHandoffViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ESCALATION_HANDOFF_SCHEMA_VERSION {
            violations.push(M5EscalationHandoffViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EscalationHandoffViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_summary_posture_coverage(self, &mut violations);
        validate_row_posture_coverage(self, &mut violations);
        validate_scenario_lineage_coverage(self, &mut violations);
        validate_destination_coverage(self, &mut violations);
        validate_redaction_state_coverage(self, &mut violations);
        validate_repair_attempt_coverage(self, &mut violations);
        validate_case_disposition_coverage(self, &mut violations);
        validate_handoff_stage_coverage(self, &mut violations);
        validate_next_step_coverage(self, &mut violations);
        validate_escalation_gating_coverage(self, &mut violations);
        validate_redaction_review_coverage(self, &mut violations);
        validate_ownership_transfer_coverage(self, &mut violations);
        validate_lineage_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 escalation handoff primitive packet serializes"),
        ) {
            violations.push(M5EscalationHandoffViolation::RawMaterialInExport);
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
            .expect("m5 escalation handoff primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per escalation / handoff consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,scenario_families,destinations,handoff_stages,next_human_steps,summary_postures,row_postures,escalation_examples,handoff_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.scenario_families, |v| v.as_str()),
                join_tokens(&row.destinations, |v| v.as_str()),
                join_tokens(&row.handoff_stages, |v| v.as_str()),
                join_tokens(&row.next_human_steps, |v| v.as_str()),
                join_tokens(&row.summary_postures, |v| v.as_str()),
                join_tokens(&row.row_postures, |v| v.as_str()),
                row.escalation_examples.len(),
                row.handoff_examples.len(),
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
        out.push_str("# M5 Escalation-Packet-Summary / Handoff-Timeline-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Escalation / handoff consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Summary postures: {}\n",
            self.vocabulary_set.summary_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Row postures: {}\n",
            self.vocabulary_set.row_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Destinations: {}\n",
            self.vocabulary_set.destinations.join(", ")
        ));
        out.push_str(&format!(
            "- Next human steps: {}\n",
            self.vocabulary_set.next_human_steps.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Escalation / handoff consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked escalation summaries: {}\n",
                row.escalation_examples.len()
            ));
            for case in &row.escalation_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (leaves `{}`, review `{}`, lineage `{}`)\n",
                    case.resolved.packet_id,
                    case.resolved.scenario_family.as_str(),
                    case.resolved.summary_posture.as_str(),
                    case.resolved.will_leave_device,
                    case.resolved.needs_redaction_review,
                    case.resolved.lineage_continuous,
                ));
            }
            out.push_str(&format!(
                "  - Worked handoff rows: {}\n",
                row.handoff_examples.len()
            ));
            for case in &row.handoff_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (transferred `{}`, next `{}`)\n",
                    case.resolved.event_identity,
                    case.resolved.stage.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.ownership_transferred,
                    case.resolved.next_step.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 escalation / handoff primitive export.
#[derive(Debug)]
pub enum M5EscalationHandoffArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EscalationHandoffViolation>),
}

impl fmt::Display for M5EscalationHandoffArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 escalation handoff primitive export parse failed: {error}"
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
                    "m5 escalation handoff primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EscalationHandoffArtifactError {}

/// Validation failures emitted by [`M5EscalationHandoffPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EscalationHandoffViolation {
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
    /// A required escalation / handoff consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// An escalation / handoff row is incomplete.
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
    /// A row declares no worked escalation or handoff resolutions.
    WorkedExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every escalation-summary posture.
    SummaryPostureCoverageUnproven,
    /// The worked resolutions do not exercise every handoff-row posture.
    RowPostureCoverageUnproven,
    /// The worked resolutions do not exercise every scenario family or finding family.
    ScenarioLineageCoverageUnproven,
    /// The worked resolutions do not exercise every escalation packet destination.
    DestinationCoverageUnproven,
    /// The worked resolutions do not exercise every redaction state.
    RedactionStateCoverageUnproven,
    /// The worked resolutions do not exercise every approved repair class.
    RepairAttemptCoverageUnproven,
    /// The worked resolutions do not exercise every case disposition.
    CaseDispositionCoverageUnproven,
    /// The worked resolutions do not exercise every handoff stage.
    HandoffStageCoverageUnproven,
    /// The worked resolutions do not exercise every next human step.
    NextStepCoverageUnproven,
    /// The worked resolutions do not prove both a ready-to-escalate and a not-ready summary.
    EscalationGatingCoverageUnproven,
    /// The worked resolutions do not prove a redaction-review-required summary.
    RedactionReviewCoverageUnproven,
    /// The worked resolutions do not prove both a transferred and a retained ownership row.
    OwnershipTransferCoverageUnproven,
    /// A worked resolution collapses or drops its lineage, evidence, or next step.
    LineagePreservationUnproven,
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

impl M5EscalationHandoffViolation {
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
            Self::WorkedExampleMissing => "worked_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::SummaryPostureCoverageUnproven => "summary_posture_coverage_unproven",
            Self::RowPostureCoverageUnproven => "row_posture_coverage_unproven",
            Self::ScenarioLineageCoverageUnproven => "scenario_lineage_coverage_unproven",
            Self::DestinationCoverageUnproven => "destination_coverage_unproven",
            Self::RedactionStateCoverageUnproven => "redaction_state_coverage_unproven",
            Self::RepairAttemptCoverageUnproven => "repair_attempt_coverage_unproven",
            Self::CaseDispositionCoverageUnproven => "case_disposition_coverage_unproven",
            Self::HandoffStageCoverageUnproven => "handoff_stage_coverage_unproven",
            Self::NextStepCoverageUnproven => "next_step_coverage_unproven",
            Self::EscalationGatingCoverageUnproven => "escalation_gating_coverage_unproven",
            Self::RedactionReviewCoverageUnproven => "redaction_review_coverage_unproven",
            Self::OwnershipTransferCoverageUnproven => "ownership_transfer_coverage_unproven",
            Self::LineagePreservationUnproven => "lineage_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 escalation / handoff primitive export.
pub fn current_stable_m5_escalation_handoff_export(
) -> Result<M5EscalationHandoffPacket, M5EscalationHandoffArtifactError> {
    let packet: M5EscalationHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-escalation-packet-summary-handoff-timeline-row-primitive-proof/support_export.json"
    )))
    .map_err(M5EscalationHandoffArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EscalationHandoffArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ESCALATION_HANDOFF_SCHEMA_REF,
        M5_ESCALATION_HANDOFF_DOC_REF,
        M5_ESCALATION_HANDOFF_COMPONENT_MATRIX_REF,
        M5_ESCALATION_HANDOFF_ESCALATION_PACKET_REF,
        M5_ESCALATION_HANDOFF_HANDOFF_PACKET_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EscalationHandoffViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5EscalationHandoffViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let present: BTreeSet<M5EscalationHandoffConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5EscalationHandoffConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5EscalationHandoffViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.escalation_anatomy_parts.is_empty()
            || row.handoff_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.scenario_families.is_empty()
            || row.finding_families.is_empty()
            || row.destinations.is_empty()
            || row.redaction_states.is_empty()
            || row.handoff_stages.is_empty()
            || row.next_human_steps.is_empty()
            || row.approved_repair_classes.is_empty()
            || row.case_dispositions.is_empty()
            || row.summary_postures.is_empty()
            || row.summary_actions.is_empty()
            || row.row_postures.is_empty()
            || row.row_actions.is_empty()
            || row.summary_export_fields.is_empty()
            || row.row_export_fields.is_empty()
        {
            violations.push(M5EscalationHandoffViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5EscalationHandoffViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5EscalationHandoffViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5SupportAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5EscalationHandoffViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5EscalationHandoffViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5EscalationHandoffViolation::DowngradeTriggersMissing);
        }
        if row.escalation_examples.is_empty() || row.handoff_examples.is_empty() {
            violations.push(M5EscalationHandoffViolation::WorkedExampleMissing);
        }
        if row
            .escalation_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .handoff_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5EscalationHandoffViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5EscalationHandoffViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5EscalationHandoffViolation::RowInvariantViolated);
        }
    }
}

fn escalation_cases(
    packet: &M5EscalationHandoffPacket,
) -> impl Iterator<Item = &M5EscalationPacketSummaryResolutionCase> {
    packet
        .rows
        .iter()
        .flat_map(|row| row.escalation_examples.iter())
}

fn handoff_cases(
    packet: &M5EscalationHandoffPacket,
) -> impl Iterator<Item = &M5HandoffTimelineRowResolutionCase> {
    packet
        .rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
}

/// Every escalation-summary posture must be exercised by some worked resolution.
fn validate_summary_posture_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5EscalationPacketSummaryPosture> = escalation_cases(packet)
        .map(|case| case.resolved.summary_posture)
        .collect();
    if !M5EscalationPacketSummaryPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5EscalationHandoffViolation::SummaryPostureCoverageUnproven);
    }
}

/// Every handoff-row posture must be exercised by some worked resolution.
fn validate_row_posture_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5HandoffTimelineRowPosture> = handoff_cases(packet)
        .map(|case| case.resolved.row_posture)
        .collect();
    if !M5HandoffTimelineRowPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5EscalationHandoffViolation::RowPostureCoverageUnproven);
    }
}

/// Every scenario family and every finding family must appear in some worked escalation
/// summary, so the scenario / finding lineage is proven end to end — the AC-1 requirement
/// that lineage stays continuous from local diagnosis through export.
fn validate_scenario_lineage_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let scenarios: BTreeSet<M5SupportScenarioFamily> = escalation_cases(packet)
        .map(|case| case.resolved.scenario_family)
        .collect();
    let findings: BTreeSet<M5DoctorFindingFamily> = escalation_cases(packet)
        .flat_map(|case| case.resolved.finding_families.iter().copied())
        .collect();
    let covered = M5SupportScenarioFamily::ALL
        .iter()
        .all(|scenario| scenarios.contains(scenario))
        && M5DoctorFindingFamily::ALL
            .iter()
            .all(|finding| findings.contains(finding));
    if !covered {
        violations.push(M5EscalationHandoffViolation::ScenarioLineageCoverageUnproven);
    }
}

/// Every escalation packet destination must be exercised.
fn validate_destination_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5EscalationPacketDestination> = escalation_cases(packet)
        .map(|case| case.resolved.destination)
        .collect();
    if !M5EscalationPacketDestination::ALL
        .iter()
        .all(|destination| exercised.contains(destination))
    {
        violations.push(M5EscalationHandoffViolation::DestinationCoverageUnproven);
    }
}

/// Every redaction state must be exercised.
fn validate_redaction_state_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5SupportRedactionState> = escalation_cases(packet)
        .map(|case| case.resolved.redaction_state)
        .collect();
    if !M5SupportRedactionState::ALL
        .iter()
        .all(|state| exercised.contains(state))
    {
        violations.push(M5EscalationHandoffViolation::RedactionStateCoverageUnproven);
    }
}

/// Every approved repair class must appear in some escalation summary's repair attempts, so
/// "what was tried" is proven across the full vocabulary.
fn validate_repair_attempt_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5ApprovedRepairClass> = escalation_cases(packet)
        .flat_map(|case| case.resolved.repair_attempts.iter().copied())
        .collect();
    if !M5ApprovedRepairClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5EscalationHandoffViolation::RepairAttemptCoverageUnproven);
    }
}

/// Every case disposition must be exercised.
fn validate_case_disposition_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5SupportCaseDisposition> = escalation_cases(packet)
        .map(|case| case.resolved.case_disposition)
        .collect();
    if !M5SupportCaseDisposition::ALL
        .iter()
        .all(|disposition| exercised.contains(disposition))
    {
        violations.push(M5EscalationHandoffViolation::CaseDispositionCoverageUnproven);
    }
}

/// Every handoff stage must be exercised.
fn validate_handoff_stage_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5HandoffStage> = handoff_cases(packet)
        .map(|case| case.resolved.stage)
        .collect();
    if !M5HandoffStage::ALL
        .iter()
        .all(|stage| exercised.contains(stage))
    {
        violations.push(M5EscalationHandoffViolation::HandoffStageCoverageUnproven);
    }
}

/// Every next human step must be exercised, so "what remains next" is proven across the
/// full vocabulary — the AC-2 requirement.
fn validate_next_step_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let exercised: BTreeSet<M5NextHumanStep> = handoff_cases(packet)
        .map(|case| case.resolved.next_step)
        .collect();
    if !M5NextHumanStep::ALL
        .iter()
        .all(|step| exercised.contains(step))
    {
        violations.push(M5EscalationHandoffViolation::NextStepCoverageUnproven);
    }
}

/// At least one worked summary must prove a ready-to-escalate packet that offers the confirm
/// action and at least one must prove a summary where the packet does not leave and the
/// confirm action is withheld — so an escalation is never faked and never silently allowed.
fn validate_escalation_gating_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let has_ready = escalation_cases(packet).any(|case| {
        case.resolved.will_leave_device
            && case
                .resolved
                .available_actions
                .contains(&M5EscalationPacketSummaryAction::ConfirmEscalation)
    });
    let has_withheld = escalation_cases(packet).any(|case| {
        !case.resolved.will_leave_device
            && !case
                .resolved
                .available_actions
                .contains(&M5EscalationPacketSummaryAction::ConfirmEscalation)
    });
    if !(has_ready && has_withheld) {
        violations.push(M5EscalationHandoffViolation::EscalationGatingCoverageUnproven);
    }
}

/// At least one worked summary must prove a redaction-review-required posture — the
/// requirement that a device-leaving packet under a full-metadata posture forces a review
/// before it can escalate.
fn validate_redaction_review_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    if !escalation_cases(packet).any(|case| case.resolved.needs_redaction_review) {
        violations.push(M5EscalationHandoffViolation::RedactionReviewCoverageUnproven);
    }
}

/// At least one worked handoff row must prove a transferred ownership and at least one must
/// prove a retained ownership, so ownership continuity is proven both ways.
fn validate_ownership_transfer_coverage(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let has_transferred = handoff_cases(packet).any(|case| case.resolved.ownership_transferred);
    let has_retained = handoff_cases(packet).any(|case| !case.resolved.ownership_transferred);
    if !(has_transferred && has_retained) {
        violations.push(M5EscalationHandoffViolation::OwnershipTransferCoverageUnproven);
    }
}

/// Every worked resolution must preserve its lineage exactly — the acceptance criteria that
/// scenario / finding / packet lineage stays continuous and that a handoff consumer can
/// reconstruct the case (owner, evidence, next step) without it collapsing into one opaque
/// blob.
fn validate_lineage_preservation(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let escalation_ok = escalation_cases(packet).all(|case| case.preserves_lineage());
    let handoff_ok = handoff_cases(packet).all(|case| case.preserves_lineage());
    if !(escalation_ok && handoff_ok) {
        violations.push(M5EscalationHandoffViolation::LineagePreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.summary_shows_packet_scenario_and_finding_lineage,
        review.summary_shows_repair_attempts_and_redaction,
        review.summary_shows_build_profile_and_destination,
        review.summary_always_offers_confirm_and_cancel,
        review.lineage_continuous_from_diagnosis_through_export,
        review.handoff_row_preserves_identity_and_owners,
        review.handoff_row_preserves_evidence_and_next_step,
        review.handoff_consumer_can_reconstruct_case,
        review.redaction_review_required_before_leaving_device,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_escalation_and_handoff_truth,
        review.later_rows_cannot_invent_parallel_vocabulary,
        review.no_surface_masks_lineage_destination_or_next_step,
    ] {
        if !ok {
            violations.push(M5EscalationHandoffViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.doctor_and_support_surfaces_consume_lineage_vocabulary,
        projection.summary_posture_reads_single_source,
        projection.row_posture_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5EscalationHandoffViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EscalationHandoffViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EscalationHandoffPacket,
    violations: &mut Vec<M5EscalationHandoffViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.support_case_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EscalationHandoffViolation::ReleasePostureIncomplete);
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
