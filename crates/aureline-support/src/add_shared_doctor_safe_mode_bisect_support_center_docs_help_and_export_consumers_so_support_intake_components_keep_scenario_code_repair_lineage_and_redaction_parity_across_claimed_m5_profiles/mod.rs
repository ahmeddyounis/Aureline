//! Shared consumers for the reusable M5 support-intake and escalation components, so
//! the support-scenario picker row, issue-report builder step, escalation-packet
//! summary, handoff-timeline row, and unsafe-fix blocked note keep scenario-code,
//! packet-id, redaction-class, and approved-repair language aligned across every claimed
//! M5 support surface where a user starts diagnosis in Project Doctor, recovers through
//! safe mode or extension bisect, works the support center, reads Help / docs, or exports
//! a case.
//!
//! Aureline's frozen support-intake / escalation component matrix
//! (`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`)
//! names the five governed component families, and four sibling `implement_*` lanes
//! narrow those families into working primitives, each with its own canonical schema,
//! contract doc, and support-export artifact:
//!
//! * the support-scenario picker row (`implement_support_scenario_picker_rows_...`),
//! * the issue-report builder step (`implement_issue_report_builder_steps_...`),
//! * the escalation-packet summary and handoff-timeline row
//!   (`implement_escalation_packet_summaries_and_handoff_timeline_rows_...`), and
//! * the unsafe-fix blocked note (`implement_unsafe_fix_blocked_notes_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the five families
//! are reusable components — not one Doctor result plus a few isolated export objects — by
//! binding every claimed M5 support consumer (Project Doctor results, the safe-mode
//! recovery flow, the extension-bisect recovery flow, the support center, Help / docs, and
//! the CLI / headless export desk) to the same canonical component schemas and the same
//! descriptor vocabulary. Each consumer points at the primitive's canonical schema and
//! support-export artifact rather than re-wording scenario, packet, redaction, or repair
//! facts in local prose, and each keeps that vocabulary truthful even when scenario
//! classification is uncertain, evidence classes are incomplete, a packet destination is
//! unavailable under current policy, or redaction review is still pending.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_support_intake_binding`] — that takes one consumer's
//!    adoption of one component family, the descriptor set it surfaces, the parity-health
//!    mode it renders under, and any export caveats, and produces one
//!    [`M5SupportIntakeResolvedBinding`] carrying the derived claim-parity state and —
//!    whenever parity is weakened — a self-contained [`M5SupportIntakeAutoNarrowBanner`]
//!    that names the exact reason (uncertain scenario classification, incomplete evidence
//!    classes, an unavailable packet destination, or pending redaction review), the
//!    descriptors that stay preserved, and the recovery action, rather than a generic
//!    "degraded" note. The resolver never lets a narrowed context drop a required
//!    descriptor and never invents a second escalation grammar.
//! 2. A parity matrix — [`M5SupportIntakeComponentConsumerPacket`] — that binds one row
//!    per claimed M5 support consumer to the five canonical component families, the one
//!    shared descriptor vocabulary, the same parity-health modes, export caveats, parity
//!    states, narrowing reasons, recovery actions, export fields, and non-visual
//!    accessibility routes, so scenario-code / packet-id / redaction-class / approved-repair
//!    facts stop diverging between the product UI, the docs, and the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the five component families themselves
//! are reused verbatim from the frozen support-intake / escalation component matrix. This
//! module mints new vocabulary only for what the adoption lane itself needs: its support
//! consumers, the shared descriptor vocabulary, the parity-health modes, the export
//! caveats, the claim-parity states, the narrowing reasons and recovery actions, the
//! consumer anatomy parts, and the export fields.
//!
//! Raw crash bodies, raw paths, credentials, and external endpoints stay outside the
//! support boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! `schemas/ui/m5-support-intake-escalation-component-consumer.schema.json` and the
//! contract doc is `docs/support/m5_support_intake_escalation_component_consumers.md`. The
//! protected fixture directory is
//! `fixtures/ui/m5-support-intake-escalation-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_support_intake_escalation_component_consumer_bisect_preview_narrowed,
    seeded_m5_support_intake_escalation_component_consumer_docs_help_beta_narrowed,
    seeded_m5_support_intake_escalation_component_consumer_packet,
    M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the five component families are frozen
// once, in the support-intake / escalation component matrix. This adoption lane reuses
// them verbatim so it never invents a parallel support vocabulary.
pub use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5SupportAccessibilityRoute, M5SupportConsumerSurface, M5SupportDeploymentLine,
    M5SupportDowngradeTrigger, M5SupportIntakeEscalationComponentFamily,
    M5SupportQualificationClass, M5SupportSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at,
// rather than re-wording their facts in local prose.
use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOC_REF, M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_escalation_packet_summaries_and_handoff_timeline_rows_with_packet_id_scenario_code_finding_repair_lineage_owner_destination_and_next_step_truth_across_claimed_m5_support_lanes::{
    M5_ESCALATION_HANDOFF_ARTIFACT_REF, M5_ESCALATION_HANDOFF_DOC_REF, M5_ESCALATION_HANDOFF_SCHEMA_REF,
};
use crate::implement_issue_report_builder_steps_and_evidence_class_selectors_with_included_excluded_redaction_repro_and_local_only_preview_truth_across_claimed_m5_support_flows::{
    M5_ISSUE_REPORT_BUILDER_STEP_ARTIFACT_REF, M5_ISSUE_REPORT_BUILDER_STEP_DOC_REF,
    M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF,
};
use crate::implement_support_scenario_picker_rows_and_seeded_symptom_scope_cues_with_start_diagnosis_parity_across_claimed_m5_support_intake_surfaces::{
    M5_SUPPORT_SCENARIO_PICKER_ROW_ARTIFACT_REF, M5_SUPPORT_SCENARIO_PICKER_ROW_DOC_REF,
    M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF,
};
use crate::implement_unsafe_fix_blocked_notes_and_approved_repair_guidance_with_blocked_action_block_reason_safer_repair_blast_radius_and_rollback_evidence_preservation_truth_across_claimed_m5_doctor_and_support_surfaces::{
    M5_UNSAFE_REPAIR_ARTIFACT_REF, M5_UNSAFE_REPAIR_DOC_REF, M5_UNSAFE_REPAIR_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SupportIntakeComponentConsumerPacket`].
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_doctor_safe_mode_bisect_support_center_docs_help_and_export_consumers_so_support_intake_components_keep_scenario_code_repair_lineage_and_redaction_parity_across_claimed_m5_profiles";

/// Schema version for M5 support-intake / escalation component-consumer records.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the support-intake / escalation component-consumer boundary
/// schema.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/support/m5_support_intake_escalation_component_consumers.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix this
/// lane adopts from.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str =
    M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-intake-escalation-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A
/// consumer that adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(
    family: M5SupportIntakeEscalationComponentFamily,
) -> &'static str {
    use M5SupportIntakeEscalationComponentFamily as Family;
    match family {
        Family::SupportScenarioPickerRow => M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF,
        Family::IssueReportBuilderStep => M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF,
        Family::EscalationPacketSummary | Family::HandoffTimelineRow => {
            M5_ESCALATION_HANDOFF_SCHEMA_REF
        }
        Family::UnsafeFixBlockedNote => M5_UNSAFE_REPAIR_SCHEMA_REF,
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(
    family: M5SupportIntakeEscalationComponentFamily,
) -> &'static str {
    use M5SupportIntakeEscalationComponentFamily as Family;
    match family {
        Family::SupportScenarioPickerRow => M5_SUPPORT_SCENARIO_PICKER_ROW_DOC_REF,
        Family::IssueReportBuilderStep => M5_ISSUE_REPORT_BUILDER_STEP_DOC_REF,
        Family::EscalationPacketSummary | Family::HandoffTimelineRow => {
            M5_ESCALATION_HANDOFF_DOC_REF
        }
        Family::UnsafeFixBlockedNote => M5_UNSAFE_REPAIR_DOC_REF,
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a
/// family.
pub const fn family_canonical_artifact_ref(
    family: M5SupportIntakeEscalationComponentFamily,
) -> &'static str {
    use M5SupportIntakeEscalationComponentFamily as Family;
    match family {
        Family::SupportScenarioPickerRow => M5_SUPPORT_SCENARIO_PICKER_ROW_ARTIFACT_REF,
        Family::IssueReportBuilderStep => M5_ISSUE_REPORT_BUILDER_STEP_ARTIFACT_REF,
        Family::EscalationPacketSummary | Family::HandoffTimelineRow => {
            M5_ESCALATION_HANDOFF_ARTIFACT_REF
        }
        Family::UnsafeFixBlockedNote => M5_UNSAFE_REPAIR_ARTIFACT_REF,
    }
}

/// One claimed M5 support consumer that adopts the shared components. These are the
/// consumers the spec names — Project Doctor results, the safe-mode recovery flow, the
/// extension-bisect recovery flow, the support center, Help / docs, and the CLI / headless
/// export desk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeComponentConsumer {
    /// The Project Doctor results surface.
    DoctorResults,
    /// The safe-mode recovery flow.
    SafeMode,
    /// The extension-bisect recovery flow.
    Bisect,
    /// The support-center surface.
    SupportCenter,
    /// The Help / docs surface.
    DocsHelp,
    /// The CLI / headless export desk and support-bundle preview.
    SupportExport,
}

impl M5SupportIntakeComponentConsumer {
    /// Every claimed support consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DoctorResults,
        Self::SafeMode,
        Self::Bisect,
        Self::SupportCenter,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorResults => "doctor_results",
            Self::SafeMode => "safe_mode",
            Self::Bisect => "bisect",
            Self::SupportCenter => "support_center",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DoctorResults => "Project Doctor Results",
            Self::SafeMode => "Safe-Mode Recovery",
            Self::Bisect => "Extension Bisect",
            Self::SupportCenter => "Support Center",
            Self::DocsHelp => "Help / Docs",
            Self::SupportExport => "Support / Export Desk",
        }
    }

    /// True when this consumer is the support / export desk — the surface singled out for
    /// a canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// The one shared descriptor vocabulary every support-intake / escalation component keeps
/// aligned across surfaces, so no consumer invents a new grammar or stale wording. The
/// descriptors in [`M5SupportIntakeComponentDescriptor::REQUIRED`] must be present on
/// every binding — the acceptance-criterion that scenario codes, packet IDs, redaction
/// classes, and approved-repair guidance stay one truth across in-product and exported
/// support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeComponentDescriptor {
    /// The scenario-code / scenario-family / incident-scope / Doctor-finding-lineage
    /// descriptor.
    ScenarioCode,
    /// The packet-id / packet-destination / next-human-step descriptor.
    PacketId,
    /// The redaction-class / evidence-class descriptor.
    RedactionClass,
    /// The approved-repair / block-reason descriptor.
    ApprovedRepair,
}

impl M5SupportIntakeComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ScenarioCode,
        Self::PacketId,
        Self::RedactionClass,
        Self::ApprovedRepair,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioCode => "scenario_code",
            Self::PacketId => "packet_id",
            Self::RedactionClass => "redaction_class",
            Self::ApprovedRepair => "approved_repair",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still
/// keeps the descriptor vocabulary — it only discloses that parity is narrowed relative to
/// the authoritative support-center rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConsumerParityHealth {
    /// Full parity: the authoritative support-center rendering.
    FullParity,
    /// Uncertain scenario classification weakens parity (the scenario is not yet mapped).
    ScenarioUncertainNarrowed,
    /// Incomplete evidence classes weaken parity (the report is not yet complete).
    EvidenceIncompleteNarrowed,
    /// An unavailable packet destination weakens parity (the case cannot escalate here).
    DestinationUnavailableNarrowed,
    /// Pending redaction review weakens parity (the export is not yet shareable).
    RedactionPendingNarrowed,
}

impl M5SupportIntakeConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ScenarioUncertainNarrowed,
        Self::EvidenceIncompleteNarrowed,
        Self::DestinationUnavailableNarrowed,
        Self::RedactionPendingNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ScenarioUncertainNarrowed => "scenario_uncertain_narrowed",
            Self::EvidenceIncompleteNarrowed => "evidence_incomplete_narrowed",
            Self::DestinationUnavailableNarrowed => "destination_unavailable_narrowed",
            Self::RedactionPendingNarrowed => "redaction_pending_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose
    /// a self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5SupportIntakeConsumerNarrowingReason> {
        Some(match self {
            Self::ScenarioUncertainNarrowed => {
                M5SupportIntakeConsumerNarrowingReason::ScenarioClassificationUncertain
            }
            Self::EvidenceIncompleteNarrowed => {
                M5SupportIntakeConsumerNarrowingReason::EvidenceClassesIncomplete
            }
            Self::DestinationUnavailableNarrowed => {
                M5SupportIntakeConsumerNarrowingReason::PacketDestinationUnavailable
            }
            Self::RedactionPendingNarrowed => {
                M5SupportIntakeConsumerNarrowingReason::RedactionReviewRequired
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow
/// banner never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConsumerNarrowingReason {
    /// The scenario classification is uncertain, so the scenario is not yet mapped to a
    /// Doctor finding family.
    ScenarioClassificationUncertain,
    /// The selected evidence classes are incomplete, so the report is not full evidence.
    EvidenceClassesIncomplete,
    /// The packet destination is unavailable under current policy / deployment state, so
    /// the case cannot escalate here.
    PacketDestinationUnavailable,
    /// Redaction review is still required, so the export is not yet shareable.
    RedactionReviewRequired,
}

impl M5SupportIntakeConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ScenarioClassificationUncertain,
        Self::EvidenceClassesIncomplete,
        Self::PacketDestinationUnavailable,
        Self::RedactionReviewRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioClassificationUncertain => "scenario_classification_uncertain",
            Self::EvidenceClassesIncomplete => "evidence_classes_incomplete",
            Self::PacketDestinationUnavailable => "packet_destination_unavailable",
            Self::RedactionReviewRequired => "redaction_review_required",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ScenarioClassificationUncertain => {
                "the scenario classification is uncertain, so it is not yet mapped to a Doctor finding family"
            }
            Self::EvidenceClassesIncomplete => {
                "the selected evidence classes are incomplete, so the report is not yet full evidence"
            }
            Self::PacketDestinationUnavailable => {
                "the packet destination is unavailable under current policy, so the case cannot escalate here"
            }
            Self::RedactionReviewRequired => {
                "redaction review is still required, so the export is a local-only bundle rather than shareable"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5SupportIntakeConsumerRecoveryAction {
        match self {
            Self::ScenarioClassificationUncertain => {
                M5SupportIntakeConsumerRecoveryAction::ClassifyScenarioBeforeEscalating
            }
            Self::EvidenceClassesIncomplete => {
                M5SupportIntakeConsumerRecoveryAction::CompleteEvidenceSelectionFirst
            }
            Self::PacketDestinationUnavailable => {
                M5SupportIntakeConsumerRecoveryAction::ChooseAvailableDestinationOrExportLocally
            }
            Self::RedactionReviewRequired => {
                M5SupportIntakeConsumerRecoveryAction::CompleteRedactionReviewFirst
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is
/// actionable from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConsumerRecoveryAction {
    /// Classify the scenario against a Doctor finding family before escalating.
    ClassifyScenarioBeforeEscalating,
    /// Complete the evidence-class selection before treating the report as full evidence.
    CompleteEvidenceSelectionFirst,
    /// Choose an available packet destination or export the case locally instead.
    ChooseAvailableDestinationOrExportLocally,
    /// Complete redaction review before treating the export as shareable.
    CompleteRedactionReviewFirst,
}

impl M5SupportIntakeConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ClassifyScenarioBeforeEscalating,
        Self::CompleteEvidenceSelectionFirst,
        Self::ChooseAvailableDestinationOrExportLocally,
        Self::CompleteRedactionReviewFirst,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassifyScenarioBeforeEscalating => "classify_scenario_before_escalating",
            Self::CompleteEvidenceSelectionFirst => "complete_evidence_selection_first",
            Self::ChooseAvailableDestinationOrExportLocally => {
                "choose_available_destination_or_export_locally"
            }
            Self::CompleteRedactionReviewFirst => "complete_redaction_review_first",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the
/// authoritative support center (an uncertain scenario classification, incomplete
/// evidence classes, an unavailable packet destination, or pending redaction review).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConsumerExportCaveat {
    /// The scenario is uncertain, so the case stays local-only rather than escalating.
    ScenarioUncertainLocalOnly,
    /// The evidence classes are incomplete, so the report is not full evidence.
    EvidenceIncompleteNotFullReport,
    /// The packet destination is unavailable, so only a local bundle is produced.
    DestinationUnavailableLocalBundleOnly,
    /// Redaction review is pending, so the export is not yet shareable.
    RedactionPendingNotShareable,
}

impl M5SupportIntakeConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ScenarioUncertainLocalOnly,
        Self::EvidenceIncompleteNotFullReport,
        Self::DestinationUnavailableLocalBundleOnly,
        Self::RedactionPendingNotShareable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioUncertainLocalOnly => "scenario_uncertain_local_only",
            Self::EvidenceIncompleteNotFullReport => "evidence_incomplete_not_full_report",
            Self::DestinationUnavailableLocalBundleOnly => {
                "destination_unavailable_local_bundle_only"
            }
            Self::RedactionPendingNotShareable => "redaction_pending_not_shareable",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary
/// is preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5SupportIntakeClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5SupportIntakeConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5SupportIntakeConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable from
/// the shared model. The fields in [`M5SupportIntakeConsumerExportField::MANDATORY`] are
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5SupportIntakeConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay
/// preserved, the export caveats, and the recovery action, so a narrowed rendering is
/// understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5SupportIntakeConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5SupportIntakeConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5SupportIntakeComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5SupportIntakeEscalationComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5SupportIntakeComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5SupportIntakeConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the support-intake binding resolver for one consumer/family
/// adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5SupportIntakeComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5SupportIntakeEscalationComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so
    /// scenario code, packet id, redaction class, and approved repair stay explicit.
    pub descriptor_families: Vec<M5SupportIntakeComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5SupportIntakeConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5SupportIntakeConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeResolvedBinding {
    /// The consumer.
    pub consumer: M5SupportIntakeComponentConsumer,
    /// The component family.
    pub component_family: M5SupportIntakeEscalationComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5SupportIntakeComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5SupportIntakeConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5SupportIntakeConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5SupportIntakeClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5SupportIntakeAutoNarrowBanner>,
}

/// Errors returned by [`resolve_support_intake_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SupportIntakeBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5SupportIntakeBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5SupportIntakeBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "support-intake binding error: {}", self.as_str())
    }
}

impl Error for M5SupportIntakeBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that scenario
/// codes, packet IDs, redaction classes, and approved-repair guidance stay explicit on
/// every surface. The claim-parity state is preserved at full parity and auto-narrowed
/// under any weakened parity-health mode, and a weakened mode always produces a
/// self-contained banner naming the exact reason and recovery action while keeping the
/// descriptor vocabulary intact.
pub fn resolve_support_intake_binding(
    input: &M5SupportIntakeBindingInput,
) -> Result<M5SupportIntakeResolvedBinding, M5SupportIntakeBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5SupportIntakeBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5SupportIntakeComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5SupportIntakeComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5SupportIntakeBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5SupportIntakeBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text
        // extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5SupportIntakeBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let claim_parity_state = if is_narrowed {
        M5SupportIntakeClaimParityState::ClaimsAutoNarrowed
    } else {
        M5SupportIntakeClaimParityState::ClaimsPreserved
    };

    let auto_narrow_banner = input.parity_health.narrowing_reason().map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5SupportIntakeAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5SupportIntakeResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeBindingCase {
    /// The resolver input.
    pub input: M5SupportIntakeBindingInput,
    /// The resolved truth. Must equal `resolve_support_intake_binding(&input)`.
    pub resolved: M5SupportIntakeResolvedBinding,
}

impl M5SupportIntakeBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SupportIntakeBindingInput) -> Self {
        let resolved = resolve_support_intake_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_support_intake_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the
/// consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5SupportIntakeEscalationComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical
    /// schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the
    /// family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5SupportIntakeBindingCase>,
}

impl M5SupportIntakeComponentBinding {
    /// True when the binding points at the family's canonical refs and references the
    /// canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one support consumer bound to the canonical component
/// families, the shared descriptor vocabulary, the parity-health modes, export caveats,
/// parity states, narrowing reasons, recovery actions, export fields, and accessibility
/// routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentConsumerRow {
    /// Support consumer.
    pub consumer: M5SupportIntakeComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5SupportQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 support surface families that render / consume this projection.
    pub surface_families: Vec<M5SupportSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5SupportDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5SupportIntakeConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5SupportIntakeComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5SupportIntakeConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5SupportIntakeConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5SupportIntakeClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5SupportIntakeConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5SupportIntakeConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5SupportIntakeConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5SupportAccessibilityRoute>,
    /// Support / escalation subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5SupportConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5SupportDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5SupportIntakeComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be
    /// `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new escalation grammar. MUST be
    /// `false`.
    pub invents_new_escalation_grammar: bool,
    /// Hard invariant: this consumer never drops scenario, packet, redaction, or repair
    /// truth when narrowed. MUST be `false`.
    pub drops_scenario_packet_redaction_or_repair_when_narrowed: bool,
    /// Hard invariant: this consumer never inherits a stronger escalation label from a
    /// healthier profile instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_profile: bool,
}

impl M5SupportIntakeComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SupportIntakeConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SupportIntakeConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SupportIntakeConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5SupportIntakeConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5SupportIntakeComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5SupportIntakeComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5SupportIntakeComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5SupportIntakeEscalationComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_escalation_grammar
            && !self.drops_scenario_packet_redaction_or_repair_when_narrowed
            && !self.inherits_stronger_label_from_healthier_profile
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentConsumerVocabularySet {
    /// Support-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5SupportIntakeComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5SupportIntakeComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5SupportIntakeEscalationComponentFamily::ALL, |v| {
                v.as_str()
            }),
            descriptors: tokens(&M5SupportIntakeComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5SupportIntakeConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5SupportIntakeConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5SupportIntakeConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5SupportIntakeConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5SupportIntakeClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5SupportIntakeConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SupportIntakeConsumerExportField::ALL, |v| v.as_str()),
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
pub struct M5SupportIntakeComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new escalation grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Scenario code, packet id, redaction class, and approved repair stay explicit
    /// everywhere.
    pub scenario_packet_redaction_repair_explicit_on_every_surface: bool,
    /// Uncertain scenario, incomplete evidence, unavailable destination, and pending
    /// redaction scopes auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// The support / export desk presents the same scenario and repair truth shown
    /// in-product.
    pub support_export_presents_same_scenario_and_repair_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentConsumerProjection {
    /// Project Doctor, safe mode, bisect, support center, Help / docs, and the export desk
    /// all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The scenario-code descriptor reads a single canonical source.
    pub scenario_code_reads_single_source: bool,
    /// The packet-id descriptor reads a single canonical source.
    pub packet_id_reads_single_source: bool,
    /// The redaction-class descriptor reads a single canonical source.
    pub redaction_class_reads_single_source: bool,
    /// The approved-repair descriptor reads a single canonical source.
    pub approved_repair_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting support-case audit.
    pub support_case_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SupportIntakeComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SupportIntakeComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5SupportIntakeComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportIntakeComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportIntakeComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportIntakeComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportIntakeComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportIntakeComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 support-intake / escalation component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeComponentConsumerPacket {
    /// Record kind; must equal
    /// [`M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5SupportIntakeComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportIntakeComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportIntakeComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportIntakeComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportIntakeComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportIntakeComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SupportIntakeComponentConsumerPacket {
    /// Builds an M5 support-intake / escalation component-consumer packet from stable-lane
    /// input.
    pub fn new(input: M5SupportIntakeComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 support-intake / escalation component-consumer invariants.
    pub fn validate(&self) -> Vec<M5SupportIntakeComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5SupportIntakeComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5SupportIntakeComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SupportIntakeComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 support-intake escalation component consumer packet serializes"),
        ) {
            violations.push(M5SupportIntakeComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 support-intake escalation component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Support-Intake / Escalation Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Support consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Support consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 support-intake / escalation
/// component-consumer export.
#[derive(Debug)]
pub enum M5SupportIntakeComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SupportIntakeComponentConsumerViolation>),
}

impl fmt::Display for M5SupportIntakeComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 support-intake escalation component consumer export parse failed: {error}"
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
                    "m5 support-intake escalation component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SupportIntakeComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5SupportIntakeComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SupportIntakeComponentConsumerViolation {
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
    /// A required support consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer
    /// (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// The support / export desk consumer does not reference the canonical component
    /// schema.
    SupportExportReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5SupportIntakeComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 support-intake / escalation
/// component-consumer export.
pub fn current_stable_m5_support_intake_escalation_component_consumer_export(
) -> Result<M5SupportIntakeComponentConsumerPacket, M5SupportIntakeComponentConsumerArtifactError> {
    let packet: M5SupportIntakeComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-intake-escalation-component-consumer-proof/support_export.json"
    )))
    .map_err(M5SupportIntakeComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SupportIntakeComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_DOC_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_SUPPORT_SCENARIO_PICKER_ROW_SCHEMA_REF,
        M5_ISSUE_REPORT_BUILDER_STEP_SCHEMA_REF,
        M5_ESCALATION_HANDOFF_SCHEMA_REF,
        M5_UNSAFE_REPAIR_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SupportIntakeComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SupportIntakeComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let present: BTreeSet<M5SupportIntakeComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5SupportIntakeComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5SupportIntakeComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5SupportIntakeComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SupportIntakeComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5SupportIntakeComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5SupportIntakeComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5SupportAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5SupportIntakeComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SupportIntakeComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SupportIntakeComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5SupportIntakeComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5SupportIntakeComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5SupportIntakeComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5SupportIntakeComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SupportIntakeComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SupportIntakeComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers —
/// the acceptance-criterion proof that the families are reusable components rather than
/// one Doctor result plus a few isolated export objects.
fn validate_family_reuse(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    for family in M5SupportIntakeEscalationComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations
                .push(M5SupportIntakeComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose
/// banner carries a specific reason, a recovery action, and a non-empty set of preserved
/// descriptors — the acceptance-criterion example that a consumer which cannot preserve
/// parity is visibly narrowed rather than inheriting stronger escalation labels from
/// healthier profiles.
fn validate_narrowing_disclosure(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5SupportIntakeComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with
/// preserved parity and no banner — the acceptance-criterion example that full-parity
/// consumers keep the descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5SupportIntakeClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5SupportIntakeComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// The support / export desk consumer must reference the canonical component schema for
/// each family it adopts — the acceptance-criterion that a support / export lane can never
/// drift from the product truth.
fn validate_support_export_reference(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5SupportIntakeComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations
                .push(M5SupportIntakeComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.scenario_packet_redaction_repair_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.support_export_presents_same_scenario_and_repair_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SupportIntakeComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.scenario_code_reads_single_source,
        projection.packet_id_reads_single_source,
        projection.redaction_class_reads_single_source,
        projection.approved_repair_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5SupportIntakeComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SupportIntakeComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SupportIntakeComponentConsumerPacket,
    violations: &mut Vec<M5SupportIntakeComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.support_case_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SupportIntakeComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5SupportIntakeComponentConsumerPacket,
) -> impl Iterator<Item = &M5SupportIntakeBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
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
