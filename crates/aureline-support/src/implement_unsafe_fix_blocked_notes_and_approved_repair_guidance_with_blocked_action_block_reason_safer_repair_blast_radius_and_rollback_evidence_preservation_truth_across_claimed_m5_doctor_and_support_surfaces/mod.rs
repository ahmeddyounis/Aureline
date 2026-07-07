//! Two reusable M5 unsafe-fix / approved-repair primitives — the unsafe-fix blocked note
//! and the approved-repair guidance card — so support guidance about a destructive repair
//! stays bounded, attributable, and state-preserving instead of reading like normative
//! folklore:
//!
//! - the unsafe-fix blocked note keeps the *blocked action*, *why it is not approved for
//!   the current scenario*, the *recommended safer repair*, and the *preserved rollback /
//!   evidence posture* legible, with explicit dismiss / preserve-evidence actions, so a
//!   destructive reset suggestion never reads as equivalent to a reviewed repair
//!   transaction; and
//! - the approved-repair guidance card keeps the repair's *blast radius*, its *changed-
//!   versus-unchanged classes*, its *reversibility*, and its *user-decline continuity*
//!   legible, so a user can see why a recommended repair is safer and exactly what evidence
//!   remains if they decline it.
//!
//! Aureline's frozen support-intake / escalation component matrix
//! ([`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`])
//! names the unsafe-fix blocked note as a governed component family and freezes its
//! controlled vocabulary — the unsafe-fix block reasons, the approved repair classes, and
//! the shared case dispositions — plus the scenario families and Doctor finding families
//! the lineage binds to, the redaction states, the surface families, deployment lines,
//! consumer surfaces, accessibility routes, qualification classes, and downgrade triggers.
//! This module *implements* that contract as two reusable resolvers so a blocked unsafe fix
//! always names its reason and a safer repair, and the safer repair always names its blast
//! radius, what it changes, what it leaves untouched, and what remains if the user declines
//! it — turning unsafe-repair folklore into a bounded, non-normative note.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_unsafe_fix_blocked_note`] — takes one note's id, its blocked action label,
//!    its scenario family, its related finding families and opaque evidence ids, the block
//!    reason, the recommended safer repair, the redaction posture, its build / profile
//!    identity, and its case disposition, and produces one [`M5ResolvedBlockedNote`]
//!    carrying the derived note posture (no-safe-alternative, irreversible-blocked,
//!    approval-required-blocked, policy-blocked, or evidence-or-scope-blocked), whether a
//!    safer repair is offered, whether rollback state and evidence remain preserved, whether
//!    the note is kept distinct from a reviewed repair transaction, and the bounded
//!    reveal-reason / view-safer-repair / preserve-evidence / dismiss / export actions. It
//!    never masks the block reason, never hides the recommended safer repair, always
//!    preserves the rollback / evidence posture, and always presents the blocked destructive
//!    action as blocked — never as a reviewed transaction.
//! 2. [`resolve_approved_repair_guidance`] — takes one repair's id, its approved repair
//!    class, its blast radius, its changed and unchanged classes, and its reversibility, and
//!    produces one [`M5ResolvedApprovedRepairGuidance`] carrying the derived guidance posture
//!    (no-repair-available, irreversible-repair, partially-reversible-repair,
//!    broad-reversible-repair, or scoped-reviewed-repair), whether the repair is a reviewed
//!    reversible transaction, whether declining it keeps the evidence, and the bounded
//!    reveal-blast-radius / view-changed-classes / request-approval / decline / export
//!    actions. It never drops the blast radius, never collapses the changed / unchanged
//!    classes into one opaque blob, and always keeps a decline path that preserves evidence.
//!
//! A single parity matrix — [`M5UnsafeRepairPacket`] — binds one row per claimed M5 Doctor /
//! support consumer (Doctor suggested-repair review, support-center unsafe-fix desk,
//! recovery-center repair guidance, headless / CLI repair review, and support repair export)
//! to the shared blocked-note and approved-repair-guidance anatomy, the same scenario
//! families, finding families, block reasons, approved repair classes, redaction states,
//! case dispositions, blast radii, change classes, reversibilities, postures, bounded
//! actions, export fields, and non-visual accessibility routes, so the block-reason /
//! safer-repair / rollback-evidence vocabulary stays identical across desktop, headless /
//! export, and support consumers.
//!
//! The scenario family ([`M5SupportScenarioFamily`]), Doctor finding family
//! ([`M5DoctorFindingFamily`]), unsafe-fix block reason ([`M5UnsafeFixBlockReason`]),
//! approved repair class ([`M5ApprovedRepairClass`]), redaction state
//! ([`M5SupportRedactionState`]), case disposition ([`M5SupportCaseDisposition`]), surface
//! family ([`M5SupportSurfaceFamily`]), deployment line ([`M5SupportDeploymentLine`]),
//! consumer surface ([`M5SupportConsumerSurface`]), accessibility route
//! ([`M5SupportAccessibilityRoute`]), qualification class
//! ([`M5SupportQualificationClass`]), and downgrade trigger
//! ([`M5SupportDowngradeTrigger`]) are reused verbatim from the frozen matrix so this lane
//! never invents a parallel reason, repair, or evidence vocabulary. This module mints new
//! vocabulary only for what the matrix left implicit about the two components themselves:
//! their Doctor / support consumers, their derived postures, their bounded actions, their
//! anatomy parts, their export fields, and — for the approved-repair guidance — its blast
//! radius, its change classes, and its reversibility.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the support
//! boundary; every note id, guidance id, evidence id, and build / profile identity is
//! carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_unsafe_repair_headless_cli_repair_review_beta_narrowed,
    seeded_m5_unsafe_repair_packet,
    seeded_m5_unsafe_repair_recovery_center_repair_guidance_preview_narrowed,
    M5_UNSAFE_REPAIR_PACKET_ID,
};

// The scenario family, Doctor finding family, unsafe-fix block reason, approved repair
// class, redaction state, case disposition, surface family, deployment line, consumer
// surface, accessibility route, qualification class, and downgrade triggers are frozen once,
// in the support-intake / escalation component matrix. This primitive reuses them verbatim
// so it never invents a parallel reason / repair / evidence vocabulary.
pub use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix::{
    M5ApprovedRepairClass, M5DoctorFindingFamily, M5SupportAccessibilityRoute,
    M5SupportCaseDisposition, M5SupportConsumerSurface, M5SupportDeploymentLine,
    M5SupportDowngradeTrigger, M5SupportQualificationClass, M5SupportRedactionState,
    M5SupportScenarioFamily, M5SupportSurfaceFamily, M5UnsafeFixBlockReason,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5UnsafeRepairPacket`].
pub const M5_UNSAFE_REPAIR_RECORD_KIND: &str =
    "implement_m5_unsafe_fix_blocked_notes_and_approved_repair_guidance_with_blocked_action_block_reason_safer_repair_blast_radius_and_rollback_evidence_preservation_truth_across_claimed_m5_doctor_and_support_surfaces";

/// Schema version for M5 unsafe-fix-blocked-note / approved-repair-guidance records.
pub const M5_UNSAFE_REPAIR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the blocked-note / approved-repair-guidance boundary schema.
pub const M5_UNSAFE_REPAIR_SCHEMA_REF: &str =
    "schemas/ui/m5-support-unsafe-fix-blocked-note.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_UNSAFE_REPAIR_DOC_REF: &str =
    "docs/support/m5_support_unsafe_fix_blocked_note_approved_repair_guidance_primitive.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix this
/// primitive narrows from.
pub const M5_UNSAFE_REPAIR_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-matrix.schema.json";

/// Repo-relative path of the repair-transaction contract the safer repair binds against — a
/// reviewed repair transaction is the reference a blocked destructive fix must never be
/// presented as equivalent to.
pub const M5_UNSAFE_REPAIR_REPAIR_TRANSACTION_REF: &str =
    "schemas/support/repair_transaction.schema.json";

/// Repo-relative path of the recovery-action contract behind the approved repair classes.
pub const M5_UNSAFE_REPAIR_RECOVERY_ACTION_REF: &str =
    "schemas/support/recovery_action.schema.json";

/// Repo-relative path of the recovery-ladder contract the approved repair classes are drawn
/// from.
pub const M5_UNSAFE_REPAIR_RECOVERY_LADDER_REF: &str =
    "schemas/support/harden_recovery_ladder_flows_for_cache_rebuild_settings_repair_state_migration_repair_and_targeted_resets.schema.json";

/// Repo-relative path of the export-redaction-profile contract this primitive binds its
/// redaction posture against.
pub const M5_UNSAFE_REPAIR_EXPORT_REDACTION_PROFILE_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the Doctor-finding contract behind the finding-family lineage.
pub const M5_UNSAFE_REPAIR_DOCTOR_FINDING_REF: &str = "schemas/support/doctor_finding.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_UNSAFE_REPAIR_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_UNSAFE_REPAIR_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_UNSAFE_REPAIR_CSV_REF: &str =
    "artifacts/release/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_UNSAFE_REPAIR_REPORT_REF: &str =
    "artifacts/design/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive.md";

/// One claimed M5 Doctor / support consumer that renders the shared unsafe-fix blocked note
/// and approved-repair guidance card. These are the consumers the acceptance criteria name —
/// the Doctor suggested-repair review, the support-center unsafe-fix desk, the recovery-
/// center repair guidance, the headless / CLI repair review, and the support repair export —
/// so the same block-reason / safer-repair / rollback-evidence grammar works across every
/// claimed lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UnsafeRepairConsumerSurface {
    /// The Project Doctor suggested-repair review.
    DoctorRepairReview,
    /// The support-center unsafe-fix desk.
    SupportCenterUnsafeFixDesk,
    /// The recovery-center repair guidance surface.
    RecoveryCenterRepairGuidance,
    /// The headless / CLI repair review surface.
    HeadlessCliRepairReview,
    /// The support repair export surface.
    SupportRepairExport,
}

impl M5UnsafeRepairConsumerSurface {
    /// Every claimed unsafe-fix / repair consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DoctorRepairReview,
        Self::SupportCenterUnsafeFixDesk,
        Self::RecoveryCenterRepairGuidance,
        Self::HeadlessCliRepairReview,
        Self::SupportRepairExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorRepairReview => "doctor_repair_review",
            Self::SupportCenterUnsafeFixDesk => "support_center_unsafe_fix_desk",
            Self::RecoveryCenterRepairGuidance => "recovery_center_repair_guidance",
            Self::HeadlessCliRepairReview => "headless_cli_repair_review",
            Self::SupportRepairExport => "support_repair_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DoctorRepairReview => "Doctor Repair Review",
            Self::SupportCenterUnsafeFixDesk => "Support Center Unsafe-Fix Desk",
            Self::RecoveryCenterRepairGuidance => "Recovery Center Repair Guidance",
            Self::HeadlessCliRepairReview => "Headless / CLI Repair Review",
            Self::SupportRepairExport => "Support Repair Export",
        }
    }
}

// ---- unsafe-fix-blocked-note vocabulary ---------------------------------

/// The derived posture of an unsafe-fix blocked note — the resolver's verdict about why the
/// suggested fix is blocked and whether a safer repair remains. Computed in a fixed order so
/// a note with no safe alternative never reads as one that offers a safer repair, and an
/// irreversible destructive fix always reads as blocked rather than approved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockedNotePosture {
    /// No safe repair is available; only evidence preservation and a local review remain.
    NoSafeAlternative,
    /// The blocked fix is irreversible; a reviewed safer repair replaces it.
    IrreversibleBlocked,
    /// The fix is blocked pending an explicit approval.
    ApprovalRequiredBlocked,
    /// The fix is blocked by policy.
    PolicyBlocked,
    /// The fix is blocked pending more evidence or a supported scope.
    EvidenceOrScopeBlocked,
}

impl M5BlockedNotePosture {
    /// Every note posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoSafeAlternative,
        Self::IrreversibleBlocked,
        Self::ApprovalRequiredBlocked,
        Self::PolicyBlocked,
        Self::EvidenceOrScopeBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSafeAlternative => "no_safe_alternative",
            Self::IrreversibleBlocked => "irreversible_blocked",
            Self::ApprovalRequiredBlocked => "approval_required_blocked",
            Self::PolicyBlocked => "policy_blocked",
            Self::EvidenceOrScopeBlocked => "evidence_or_scope_blocked",
        }
    }

    /// True when the note needs operator attention before a repair can proceed.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::NoSafeAlternative | Self::ApprovalRequiredBlocked | Self::PolicyBlocked
        )
    }
}

/// One bounded action an unsafe-fix blocked note offers, so a note never hides its
/// reveal-reason / view-safer-repair / preserve-evidence / dismiss / export affordances, and
/// always offers a dismiss so a user is never trapped and a preserve-evidence so the
/// evidence never silently drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockedNoteAction {
    /// Reveal why the fix is blocked.
    RevealBlockReason,
    /// View the recommended safer repair.
    ViewSaferRepair,
    /// Preserve the evidence that remains if the fix is declined.
    PreserveEvidence,
    /// Dismiss the note without applying the blocked fix.
    DismissNote,
    /// Export the blocked note as metadata-only support evidence.
    ExportNote,
}

impl M5BlockedNoteAction {
    /// Every note action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealBlockReason,
        Self::ViewSaferRepair,
        Self::PreserveEvidence,
        Self::DismissNote,
        Self::ExportNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealBlockReason => "reveal_block_reason",
            Self::ViewSaferRepair => "view_safer_repair",
            Self::PreserveEvidence => "preserve_evidence",
            Self::DismissNote => "dismiss_note",
            Self::ExportNote => "export_note",
        }
    }
}

/// Controlled unsafe-fix-blocked-note anatomy part the shared note surfaces. The parts in
/// [`M5BlockedNoteAnatomyPart::MANDATORY`] are required on every note so the blocked action,
/// block reason, scenario, finding lineage, recommended safer repair, rollback posture,
/// evidence posture, and case disposition are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockedNoteAnatomyPart {
    /// The blocked-action cue.
    BlockedActionCue,
    /// The block-reason cue.
    BlockReasonCue,
    /// The scenario-code cue.
    ScenarioCue,
    /// The related finding / crash lineage cue.
    FindingLineageCue,
    /// The recommended-safer-repair cue.
    RecommendedRepairCue,
    /// The preserved-rollback-posture cue.
    RollbackPostureCue,
    /// The preserved-evidence-posture cue.
    EvidencePostureCue,
    /// The case-disposition cue.
    CaseDispositionCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5BlockedNoteAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::BlockedActionCue,
        Self::BlockReasonCue,
        Self::ScenarioCue,
        Self::FindingLineageCue,
        Self::RecommendedRepairCue,
        Self::RollbackPostureCue,
        Self::EvidencePostureCue,
        Self::CaseDispositionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every note must render.
    pub const MANDATORY: [Self; 8] = [
        Self::BlockedActionCue,
        Self::BlockReasonCue,
        Self::ScenarioCue,
        Self::FindingLineageCue,
        Self::RecommendedRepairCue,
        Self::RollbackPostureCue,
        Self::EvidencePostureCue,
        Self::CaseDispositionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedActionCue => "blocked_action_cue",
            Self::BlockReasonCue => "block_reason_cue",
            Self::ScenarioCue => "scenario_cue",
            Self::FindingLineageCue => "finding_lineage_cue",
            Self::RecommendedRepairCue => "recommended_repair_cue",
            Self::RollbackPostureCue => "rollback_posture_cue",
            Self::EvidencePostureCue => "evidence_posture_cue",
            Self::CaseDispositionCue => "case_disposition_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the blocked-note export carries so unsafe-fix truth is reconstructable. The fields
/// in [`M5BlockedNoteExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockedNoteExportField {
    /// The note id.
    NoteId,
    /// The blocked action label.
    BlockedActionLabel,
    /// The scenario family (scenario code).
    ScenarioFamily,
    /// The related finding families.
    FindingFamilies,
    /// The unsafe-fix block reason.
    BlockReason,
    /// The recommended safer repair.
    RecommendedRepair,
    /// Whether the rollback state remains preserved.
    RollbackPreserved,
    /// Whether the evidence remains preserved.
    EvidencePreserved,
    /// The case disposition.
    CaseDisposition,
    /// The derived note posture.
    NotePosture,
}

impl M5BlockedNoteExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::NoteId,
        Self::BlockedActionLabel,
        Self::ScenarioFamily,
        Self::FindingFamilies,
        Self::BlockReason,
        Self::RecommendedRepair,
        Self::RollbackPreserved,
        Self::EvidencePreserved,
        Self::CaseDisposition,
        Self::NotePosture,
    ];

    /// The export fields every note must carry.
    pub const MANDATORY: [Self; 9] = [
        Self::NoteId,
        Self::BlockedActionLabel,
        Self::ScenarioFamily,
        Self::FindingFamilies,
        Self::BlockReason,
        Self::RecommendedRepair,
        Self::RollbackPreserved,
        Self::EvidencePreserved,
        Self::CaseDisposition,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoteId => "note_id",
            Self::BlockedActionLabel => "blocked_action_label",
            Self::ScenarioFamily => "scenario_family",
            Self::FindingFamilies => "finding_families",
            Self::BlockReason => "block_reason",
            Self::RecommendedRepair => "recommended_repair",
            Self::RollbackPreserved => "rollback_preserved",
            Self::EvidencePreserved => "evidence_preserved",
            Self::CaseDisposition => "case_disposition",
            Self::NotePosture => "note_posture",
        }
    }
}

// ---- approved-repair-guidance vocabulary --------------------------------

/// Controlled blast radius of an approved repair — how wide a change the repair makes, so a
/// scoped reviewed repair never reads as broad as a device-wide reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairBlastRadius {
    /// No change is made.
    NoChange,
    /// A single artifact changes.
    SingleArtifact,
    /// The change is workspace-scoped.
    WorkspaceScoped,
    /// The change is profile-scoped.
    ProfileScoped,
    /// The change is device-wide.
    DeviceWide,
}

impl M5RepairBlastRadius {
    /// Every blast radius, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoChange,
        Self::SingleArtifact,
        Self::WorkspaceScoped,
        Self::ProfileScoped,
        Self::DeviceWide,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::SingleArtifact => "single_artifact",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::ProfileScoped => "profile_scoped",
            Self::DeviceWide => "device_wide",
        }
    }

    /// True when the blast radius reaches beyond a single workspace.
    pub const fn is_broad(self) -> bool {
        matches!(self, Self::ProfileScoped | Self::DeviceWide)
    }
}

/// Controlled class of state a repair may change or leave unchanged, so a repair's changed-
/// versus-unchanged surface is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairChangeClass {
    /// Cache artifacts.
    CacheArtifacts,
    /// The search index.
    SearchIndex,
    /// Settings.
    Settings,
    /// Workspace state.
    WorkspaceState,
    /// Generated files.
    GeneratedFiles,
    /// User content.
    UserContent,
}

impl M5RepairChangeClass {
    /// Every change class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CacheArtifacts,
        Self::SearchIndex,
        Self::Settings,
        Self::WorkspaceState,
        Self::GeneratedFiles,
        Self::UserContent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CacheArtifacts => "cache_artifacts",
            Self::SearchIndex => "search_index",
            Self::Settings => "settings",
            Self::WorkspaceState => "workspace_state",
            Self::GeneratedFiles => "generated_files",
            Self::UserContent => "user_content",
        }
    }
}

/// Controlled reversibility of an approved repair, so a reviewed reversible transaction is
/// never confused with an irreversible destructive change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairReversibility {
    /// A reviewed, fully reversible transaction with a checkpoint / rollback.
    ReversibleTransaction,
    /// A partially reversible repair.
    PartiallyReversible,
    /// An irreversible change.
    Irreversible,
}

impl M5RepairReversibility {
    /// Every reversibility, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReversibleTransaction,
        Self::PartiallyReversible,
        Self::Irreversible,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleTransaction => "reversible_transaction",
            Self::PartiallyReversible => "partially_reversible",
            Self::Irreversible => "irreversible",
        }
    }
}

/// The derived posture of an approved-repair guidance card — where the recommended repair
/// sits between a fully reviewed, scoped transaction and an irreversible device-wide change.
/// Computed in a fixed order so a scoped reviewed repair never reads as broad, and an
/// irreversible repair never reads as a reviewed transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovedRepairGuidancePosture {
    /// No safe repair is available.
    NoRepairAvailable,
    /// The repair is irreversible.
    IrreversibleRepair,
    /// The repair is only partially reversible.
    PartiallyReversibleRepair,
    /// The repair is reversible but broad (profile- or device-wide).
    BroadReversibleRepair,
    /// The repair is a scoped, reviewed, reversible transaction.
    ScopedReviewedRepair,
}

impl M5ApprovedRepairGuidancePosture {
    /// Every guidance posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoRepairAvailable,
        Self::IrreversibleRepair,
        Self::PartiallyReversibleRepair,
        Self::BroadReversibleRepair,
        Self::ScopedReviewedRepair,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRepairAvailable => "no_repair_available",
            Self::IrreversibleRepair => "irreversible_repair",
            Self::PartiallyReversibleRepair => "partially_reversible_repair",
            Self::BroadReversibleRepair => "broad_reversible_repair",
            Self::ScopedReviewedRepair => "scoped_reviewed_repair",
        }
    }

    /// True when the repair needs an explicit approval before it can proceed.
    pub const fn needs_approval(self) -> bool {
        matches!(
            self,
            Self::NoRepairAvailable | Self::IrreversibleRepair | Self::PartiallyReversibleRepair
        )
    }

    /// True when the repair needs operator attention.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::NoRepairAvailable | Self::IrreversibleRepair | Self::BroadReversibleRepair
        )
    }
}

/// One bounded action an approved-repair guidance card offers, so a card never hides its
/// reveal-blast-radius / view-changed-classes / request-approval / decline / export
/// affordances, and always offers a decline path that preserves the evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovedRepairGuidanceAction {
    /// Reveal the blast radius of the repair.
    RevealBlastRadius,
    /// View the changed and unchanged classes.
    ViewChangedClasses,
    /// Request an explicit approval before the repair can proceed.
    RequestApproval,
    /// Decline the repair while keeping the evidence.
    DeclineRepair,
    /// Export the guidance card as metadata-only support evidence.
    ExportGuidance,
}

impl M5ApprovedRepairGuidanceAction {
    /// Every guidance action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealBlastRadius,
        Self::ViewChangedClasses,
        Self::RequestApproval,
        Self::DeclineRepair,
        Self::ExportGuidance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealBlastRadius => "reveal_blast_radius",
            Self::ViewChangedClasses => "view_changed_classes",
            Self::RequestApproval => "request_approval",
            Self::DeclineRepair => "decline_repair",
            Self::ExportGuidance => "export_guidance",
        }
    }
}

/// Controlled approved-repair-guidance anatomy part the shared card surfaces. The parts in
/// [`M5ApprovedRepairGuidanceAnatomyPart::MANDATORY`] are required on every card so the
/// repair class, blast radius, changed classes, unchanged classes, reversibility, and
/// decline-continuity are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovedRepairGuidanceAnatomyPart {
    /// The repair-class cue.
    RepairClassCue,
    /// The blast-radius cue.
    BlastRadiusCue,
    /// The changed-classes cue.
    ChangedClassesCue,
    /// The unchanged-classes cue.
    UnchangedClassesCue,
    /// The reversibility cue.
    ReversibilityCue,
    /// The decline-continuity cue.
    DeclineContinuityCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5ApprovedRepairGuidanceAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RepairClassCue,
        Self::BlastRadiusCue,
        Self::ChangedClassesCue,
        Self::UnchangedClassesCue,
        Self::ReversibilityCue,
        Self::DeclineContinuityCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every card must render.
    pub const MANDATORY: [Self; 6] = [
        Self::RepairClassCue,
        Self::BlastRadiusCue,
        Self::ChangedClassesCue,
        Self::UnchangedClassesCue,
        Self::ReversibilityCue,
        Self::DeclineContinuityCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairClassCue => "repair_class_cue",
            Self::BlastRadiusCue => "blast_radius_cue",
            Self::ChangedClassesCue => "changed_classes_cue",
            Self::UnchangedClassesCue => "unchanged_classes_cue",
            Self::ReversibilityCue => "reversibility_cue",
            Self::DeclineContinuityCue => "decline_continuity_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the approved-repair guidance export carries so repair-guidance truth is
/// reconstructable. The fields in [`M5ApprovedRepairGuidanceExportField::MANDATORY`] are
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovedRepairGuidanceExportField {
    /// The guidance id.
    GuidanceId,
    /// The approved repair class.
    RepairClass,
    /// The blast radius.
    BlastRadius,
    /// The changed classes.
    ChangedClasses,
    /// The unchanged classes.
    UnchangedClasses,
    /// The reversibility.
    Reversibility,
    /// Whether declining the repair keeps the evidence.
    DeclineKeepsEvidence,
    /// The derived guidance posture.
    GuidancePosture,
}

impl M5ApprovedRepairGuidanceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::GuidanceId,
        Self::RepairClass,
        Self::BlastRadius,
        Self::ChangedClasses,
        Self::UnchangedClasses,
        Self::Reversibility,
        Self::DeclineKeepsEvidence,
        Self::GuidancePosture,
    ];

    /// The export fields every card must carry.
    pub const MANDATORY: [Self; 7] = [
        Self::GuidanceId,
        Self::RepairClass,
        Self::BlastRadius,
        Self::ChangedClasses,
        Self::UnchangedClasses,
        Self::Reversibility,
        Self::DeclineKeepsEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuidanceId => "guidance_id",
            Self::RepairClass => "repair_class",
            Self::BlastRadius => "blast_radius",
            Self::ChangedClasses => "changed_classes",
            Self::UnchangedClasses => "unchanged_classes",
            Self::Reversibility => "reversibility",
            Self::DeclineKeepsEvidence => "decline_keeps_evidence",
            Self::GuidancePosture => "guidance_posture",
        }
    }
}

// ---- unsafe-fix-blocked-note resolver -----------------------------------

/// The full input to the unsafe-fix-blocked-note resolver for one note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedNoteResolutionInput {
    /// The opaque stable note id (must be non-empty).
    pub note_id: String,
    /// The specific label of the blocked action (must be non-empty and never generic).
    pub blocked_action_label: String,
    /// The scenario family (scenario code) behind the note.
    pub scenario_family: M5SupportScenarioFamily,
    /// The related Doctor finding families (the finding lineage).
    pub finding_families: Vec<M5DoctorFindingFamily>,
    /// The related opaque finding / crash evidence ids (each must be non-empty when present).
    pub related_evidence_ids: Vec<String>,
    /// Why the suggested fix is not approved for the current scenario.
    pub block_reason: M5UnsafeFixBlockReason,
    /// The recommended safer repair a user may take instead.
    pub recommended_repair: M5ApprovedRepairClass,
    /// The redaction posture the note's evidence export will apply.
    pub redaction_state: M5SupportRedactionState,
    /// The opaque build / profile identity (must be non-empty).
    pub build_profile_identity: String,
    /// The shared case disposition.
    pub case_disposition: M5SupportCaseDisposition,
}

/// The resolved unsafe-fix-blocked-note truth for one note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBlockedNote {
    /// The opaque stable note id, preserved exactly from the input.
    pub note_id: String,
    /// The blocked action label, preserved exactly from the input.
    pub blocked_action_label: String,
    /// The scenario family, preserved exactly from the input.
    pub scenario_family: M5SupportScenarioFamily,
    /// The related finding families, preserved exactly from the input.
    pub finding_families: Vec<M5DoctorFindingFamily>,
    /// The related evidence ids, preserved exactly from the input.
    pub related_evidence_ids: Vec<String>,
    /// The block reason, preserved exactly from the input.
    pub block_reason: M5UnsafeFixBlockReason,
    /// The recommended safer repair, preserved exactly from the input.
    pub recommended_repair: M5ApprovedRepairClass,
    /// The redaction posture, preserved exactly from the input.
    pub redaction_state: M5SupportRedactionState,
    /// The build / profile identity, preserved exactly from the input.
    pub build_profile_identity: String,
    /// The case disposition, preserved exactly from the input.
    pub case_disposition: M5SupportCaseDisposition,
    /// The derived note posture.
    pub note_posture: M5BlockedNotePosture,
    /// The bounded actions this note offers.
    pub available_actions: Vec<M5BlockedNoteAction>,
    /// True when a safer repair is offered (the recommended repair is not `no_safe_repair`).
    pub safer_repair_offered: bool,
    /// True when the rollback state remains preserved (always `true`: the blocked fix is
    /// never applied, so nothing is changed to roll back). The core AC-1 signal.
    pub rollback_preserved: bool,
    /// True when the evidence remains preserved (always `true`: the note never drops the
    /// evidence a user keeps if they decline). The core AC-2 signal.
    pub evidence_preserved: bool,
    /// True when this blocked destructive suggestion is kept distinct from a reviewed repair
    /// transaction (always `true`: a blocked fix is presented as blocked, never as an
    /// approved transaction). The core AC-1 signal.
    pub distinct_from_reviewed_transaction: bool,
    /// True when the scenario / finding lineage is continuous (a committed scenario with at
    /// least one bound finding family).
    pub lineage_continuous: bool,
    /// True when the note needs operator attention before a repair can proceed.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_unsafe_fix_blocked_note`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BlockedNoteResolutionError {
    /// The note id was empty.
    EmptyNoteId,
    /// The blocked action label was empty.
    EmptyBlockedActionLabel,
    /// The build / profile identity was empty.
    EmptyBuildProfileIdentity,
    /// A related evidence id was blank.
    EmptyEvidenceId,
    /// A note descriptor carried forbidden material.
    ForbiddenNoteMaterial,
}

impl M5BlockedNoteResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyNoteId => "empty_note_id",
            Self::EmptyBlockedActionLabel => "empty_blocked_action_label",
            Self::EmptyBuildProfileIdentity => "empty_build_profile_identity",
            Self::EmptyEvidenceId => "empty_evidence_id",
            Self::ForbiddenNoteMaterial => "forbidden_note_material",
        }
    }
}

impl fmt::Display for M5BlockedNoteResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unsafe-fix blocked note resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BlockedNoteResolutionError {}

/// Resolves one unsafe-fix blocked note from its declared state.
///
/// The derived note posture is computed in a fixed order: a note with no safe repair
/// available wins first (only evidence preservation and a local review remain), then a note
/// whose blocked fix is irreversible (a reviewed safer repair replaces it), then a note
/// blocked pending an explicit approval, then a note blocked by policy, and otherwise a note
/// blocked pending more evidence or a supported scope. The blocked action, block reason,
/// scenario, finding lineage, recommended safer repair, and case disposition are carried
/// explicitly and never collapsed into one opaque blob; the note always offers a dismiss so a
/// user is never trapped and a preserve-evidence so the evidence a user keeps if they decline
/// is never dropped, always keeps the rollback state preserved (the blocked fix is never
/// applied), and always presents the blocked destructive action as blocked rather than as a
/// reviewed repair transaction.
pub fn resolve_unsafe_fix_blocked_note(
    input: &M5BlockedNoteResolutionInput,
) -> Result<M5ResolvedBlockedNote, M5BlockedNoteResolutionError> {
    if input.note_id.trim().is_empty() {
        return Err(M5BlockedNoteResolutionError::EmptyNoteId);
    }
    if input.blocked_action_label.trim().is_empty() {
        return Err(M5BlockedNoteResolutionError::EmptyBlockedActionLabel);
    }
    if input.build_profile_identity.trim().is_empty() {
        return Err(M5BlockedNoteResolutionError::EmptyBuildProfileIdentity);
    }
    if input
        .related_evidence_ids
        .iter()
        .any(|id| id.trim().is_empty())
    {
        return Err(M5BlockedNoteResolutionError::EmptyEvidenceId);
    }
    if value_repr_is_forbidden(&input.note_id)
        || value_repr_is_forbidden(&input.blocked_action_label)
        || value_repr_is_forbidden(&input.build_profile_identity)
        || input
            .related_evidence_ids
            .iter()
            .any(|id| value_repr_is_forbidden(id))
    {
        return Err(M5BlockedNoteResolutionError::ForbiddenNoteMaterial);
    }

    let safer_repair_offered = !matches!(
        input.recommended_repair,
        M5ApprovedRepairClass::NoSafeRepair
    );
    let lineage_continuous = !matches!(
        input.scenario_family,
        M5SupportScenarioFamily::UncategorizedScenario
    ) && !input.finding_families.is_empty();
    let note_posture = derive_note_posture(input.block_reason, safer_repair_offered);
    let available_actions = derive_note_actions(safer_repair_offered);

    Ok(M5ResolvedBlockedNote {
        note_id: input.note_id.clone(),
        blocked_action_label: input.blocked_action_label.clone(),
        scenario_family: input.scenario_family,
        finding_families: input.finding_families.clone(),
        related_evidence_ids: input.related_evidence_ids.clone(),
        block_reason: input.block_reason,
        recommended_repair: input.recommended_repair,
        redaction_state: input.redaction_state,
        build_profile_identity: input.build_profile_identity.clone(),
        case_disposition: input.case_disposition,
        note_posture,
        available_actions,
        safer_repair_offered,
        rollback_preserved: true,
        evidence_preserved: true,
        distinct_from_reviewed_transaction: true,
        lineage_continuous,
        needs_attention: note_posture.needs_attention(),
    })
}

/// The fixed note-posture ladder.
fn derive_note_posture(
    block_reason: M5UnsafeFixBlockReason,
    safer_repair_offered: bool,
) -> M5BlockedNotePosture {
    use M5BlockedNotePosture as Posture;
    if !safer_repair_offered {
        Posture::NoSafeAlternative
    } else {
        match block_reason {
            M5UnsafeFixBlockReason::IrreversibleChange => Posture::IrreversibleBlocked,
            M5UnsafeFixBlockReason::ApprovalRequired => Posture::ApprovalRequiredBlocked,
            M5UnsafeFixBlockReason::PolicyBlocked => Posture::PolicyBlocked,
            M5UnsafeFixBlockReason::InsufficientEvidence
            | M5UnsafeFixBlockReason::OutOfScopeRepair
            | M5UnsafeFixBlockReason::UnsupportedScenario => Posture::EvidenceOrScopeBlocked,
        }
    }
}

/// Derives the bounded note action set.
///
/// Reveal-reason, preserve-evidence, dismiss, and export are always offered so the reason is
/// always inspectable, the evidence is always preserved, a user is never trapped, and the
/// note is always exportable as metadata; view-safer-repair is offered whenever a safer
/// repair is actually recommended.
fn derive_note_actions(safer_repair_offered: bool) -> Vec<M5BlockedNoteAction> {
    use M5BlockedNoteAction as Action;
    let mut actions = vec![Action::RevealBlockReason];
    if safer_repair_offered {
        actions.push(Action::ViewSaferRepair);
    }
    actions.push(Action::PreserveEvidence);
    actions.push(Action::DismissNote);
    actions.push(Action::ExportNote);
    actions
}

// ---- approved-repair-guidance resolver ----------------------------------

/// The full input to the approved-repair-guidance resolver for one repair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovedRepairGuidanceResolutionInput {
    /// The opaque stable guidance id (must be non-empty).
    pub guidance_id: String,
    /// The approved repair class this guidance describes.
    pub repair_class: M5ApprovedRepairClass,
    /// The blast radius of the repair.
    pub blast_radius: M5RepairBlastRadius,
    /// The classes of state this repair changes.
    pub changed_classes: Vec<M5RepairChangeClass>,
    /// The classes of state this repair leaves unchanged (must be non-empty: something is
    /// always preserved).
    pub unchanged_classes: Vec<M5RepairChangeClass>,
    /// The reversibility of the repair.
    pub reversibility: M5RepairReversibility,
}

/// The resolved approved-repair-guidance truth for one repair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedApprovedRepairGuidance {
    /// The opaque guidance id, preserved exactly from the input.
    pub guidance_id: String,
    /// The approved repair class, preserved exactly from the input.
    pub repair_class: M5ApprovedRepairClass,
    /// The blast radius, preserved exactly from the input.
    pub blast_radius: M5RepairBlastRadius,
    /// The changed classes, preserved exactly from the input.
    pub changed_classes: Vec<M5RepairChangeClass>,
    /// The unchanged classes, preserved exactly from the input.
    pub unchanged_classes: Vec<M5RepairChangeClass>,
    /// The reversibility, preserved exactly from the input.
    pub reversibility: M5RepairReversibility,
    /// The derived guidance posture.
    pub guidance_posture: M5ApprovedRepairGuidancePosture,
    /// The bounded actions this guidance offers.
    pub available_actions: Vec<M5ApprovedRepairGuidanceAction>,
    /// True when the repair is a reviewed, fully reversible transaction — the reference a
    /// blocked destructive fix must never be presented as equivalent to. The core AC-1
    /// signal.
    pub is_reviewed_transaction: bool,
    /// True when declining the repair keeps the evidence (always `true`: the user-decline
    /// path never drops the evidence). The core AC-2 signal.
    pub decline_keeps_evidence: bool,
    /// True when the changed and unchanged classes are both explicit (always `true`: both are
    /// typed and never collapsed into one opaque blob).
    pub changed_and_unchanged_explicit: bool,
    /// True when the repair needs an explicit approval before it can proceed.
    pub needs_approval: bool,
    /// True when the repair needs operator attention.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_approved_repair_guidance`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ApprovedRepairGuidanceResolutionError {
    /// The guidance id was empty.
    EmptyGuidanceId,
    /// No preserved (unchanged) class was declared.
    PreservedClassesMissing,
    /// A change class was declared as both changed and unchanged.
    OverlappingChangeClass,
    /// A guidance descriptor carried forbidden material.
    ForbiddenGuidanceMaterial,
}

impl M5ApprovedRepairGuidanceResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyGuidanceId => "empty_guidance_id",
            Self::PreservedClassesMissing => "preserved_classes_missing",
            Self::OverlappingChangeClass => "overlapping_change_class",
            Self::ForbiddenGuidanceMaterial => "forbidden_guidance_material",
        }
    }
}

impl fmt::Display for M5ApprovedRepairGuidanceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "approved-repair guidance resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ApprovedRepairGuidanceResolutionError {}

/// Resolves one approved-repair guidance card from its declared state.
///
/// The derived guidance posture is computed in a fixed order: a card with no safe repair
/// available wins first, then an irreversible repair, then a partially reversible repair,
/// then a reversible repair whose blast radius is broad (profile- or device-wide), and
/// otherwise a scoped, reviewed, reversible transaction. The blast radius, changed classes,
/// unchanged classes, and reversibility are carried explicitly and never collapsed into one
/// opaque blob; the guidance always keeps a decline path that preserves the evidence, and a
/// reversible transaction is always kept distinct from an irreversible change so a
/// destructive reset never reads as equivalent to a reviewed repair transaction.
pub fn resolve_approved_repair_guidance(
    input: &M5ApprovedRepairGuidanceResolutionInput,
) -> Result<M5ResolvedApprovedRepairGuidance, M5ApprovedRepairGuidanceResolutionError> {
    if input.guidance_id.trim().is_empty() {
        return Err(M5ApprovedRepairGuidanceResolutionError::EmptyGuidanceId);
    }
    if input.unchanged_classes.is_empty() {
        return Err(M5ApprovedRepairGuidanceResolutionError::PreservedClassesMissing);
    }
    let changed: BTreeSet<M5RepairChangeClass> = input.changed_classes.iter().copied().collect();
    if input
        .unchanged_classes
        .iter()
        .any(|class| changed.contains(class))
    {
        return Err(M5ApprovedRepairGuidanceResolutionError::OverlappingChangeClass);
    }
    if value_repr_is_forbidden(&input.guidance_id) {
        return Err(M5ApprovedRepairGuidanceResolutionError::ForbiddenGuidanceMaterial);
    }

    let is_reviewed_transaction = matches!(
        input.reversibility,
        M5RepairReversibility::ReversibleTransaction
    );
    let guidance_posture =
        derive_guidance_posture(input.repair_class, input.reversibility, input.blast_radius);
    let available_actions = derive_guidance_actions(guidance_posture);

    Ok(M5ResolvedApprovedRepairGuidance {
        guidance_id: input.guidance_id.clone(),
        repair_class: input.repair_class,
        blast_radius: input.blast_radius,
        changed_classes: input.changed_classes.clone(),
        unchanged_classes: input.unchanged_classes.clone(),
        reversibility: input.reversibility,
        guidance_posture,
        available_actions,
        is_reviewed_transaction,
        decline_keeps_evidence: true,
        changed_and_unchanged_explicit: true,
        needs_approval: guidance_posture.needs_approval(),
        needs_attention: guidance_posture.needs_attention(),
    })
}

/// The fixed guidance-posture ladder.
fn derive_guidance_posture(
    repair_class: M5ApprovedRepairClass,
    reversibility: M5RepairReversibility,
    blast_radius: M5RepairBlastRadius,
) -> M5ApprovedRepairGuidancePosture {
    use M5ApprovedRepairGuidancePosture as Posture;
    if matches!(repair_class, M5ApprovedRepairClass::NoSafeRepair) {
        Posture::NoRepairAvailable
    } else {
        match reversibility {
            M5RepairReversibility::Irreversible => Posture::IrreversibleRepair,
            M5RepairReversibility::PartiallyReversible => Posture::PartiallyReversibleRepair,
            M5RepairReversibility::ReversibleTransaction => {
                if blast_radius.is_broad() {
                    Posture::BroadReversibleRepair
                } else {
                    Posture::ScopedReviewedRepair
                }
            }
        }
    }
}

/// Derives the bounded guidance action set.
///
/// Reveal-blast-radius, view-changed-classes, decline, and export are always offered so the
/// blast radius is always legible, the changed / unchanged classes are always inspectable, a
/// user can always decline while keeping the evidence, and the guidance is always exportable
/// as metadata; request-approval is offered whenever the repair is not a fully reviewed
/// reversible transaction.
fn derive_guidance_actions(
    guidance_posture: M5ApprovedRepairGuidancePosture,
) -> Vec<M5ApprovedRepairGuidanceAction> {
    use M5ApprovedRepairGuidanceAction as Action;
    let mut actions = vec![Action::RevealBlastRadius, Action::ViewChangedClasses];
    if guidance_posture.needs_approval() {
        actions.push(Action::RequestApproval);
    }
    actions.push(Action::DeclineRepair);
    actions.push(Action::ExportGuidance);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked unsafe-fix-blocked-note resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedNoteResolutionCase {
    /// The resolver input.
    pub input: M5BlockedNoteResolutionInput,
    /// The resolved truth. Must equal `resolve_unsafe_fix_blocked_note(&input)`.
    pub resolved: M5ResolvedBlockedNote,
}

impl M5BlockedNoteResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BlockedNoteResolutionInput) -> Self {
        let resolved =
            resolve_unsafe_fix_blocked_note(&input).expect("seed blocked note case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_unsafe_fix_blocked_note(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved note preserves the input's note id, blocked action, scenario,
    /// finding lineage, evidence ids, block reason, recommended repair, and disposition
    /// exactly — never collapsing them into one opaque blob.
    pub fn preserves_lineage(&self) -> bool {
        self.resolved.note_id == self.input.note_id
            && self.resolved.blocked_action_label == self.input.blocked_action_label
            && self.resolved.scenario_family == self.input.scenario_family
            && self.resolved.finding_families == self.input.finding_families
            && self.resolved.related_evidence_ids == self.input.related_evidence_ids
            && self.resolved.block_reason == self.input.block_reason
            && self.resolved.recommended_repair == self.input.recommended_repair
            && self.resolved.case_disposition == self.input.case_disposition
    }
}

/// One worked approved-repair-guidance resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovedRepairGuidanceResolutionCase {
    /// The resolver input.
    pub input: M5ApprovedRepairGuidanceResolutionInput,
    /// The resolved truth. Must equal `resolve_approved_repair_guidance(&input)`.
    pub resolved: M5ResolvedApprovedRepairGuidance,
}

impl M5ApprovedRepairGuidanceResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ApprovedRepairGuidanceResolutionInput) -> Self {
        let resolved = resolve_approved_repair_guidance(&input)
            .expect("seed approved repair guidance case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_approved_repair_guidance(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved guidance preserves the input's guidance id, repair class, blast
    /// radius, changed classes, unchanged classes, and reversibility exactly — never
    /// collapsing them into one opaque blob.
    pub fn preserves_lineage(&self) -> bool {
        self.resolved.guidance_id == self.input.guidance_id
            && self.resolved.repair_class == self.input.repair_class
            && self.resolved.blast_radius == self.input.blast_radius
            && self.resolved.changed_classes == self.input.changed_classes
            && self.resolved.unchanged_classes == self.input.unchanged_classes
            && self.resolved.reversibility == self.input.reversibility
    }
}

/// One row in the primitive matrix: one Doctor / support consumer bound to the shared
/// blocked-note and approved-repair-guidance anatomy, scenario families, finding families,
/// block reasons, approved repair classes, redaction states, case dispositions, blast radii,
/// change classes, reversibilities, postures, bounded actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UnsafeRepairConsumerRow {
    /// Doctor / support consumer family.
    pub consumer_surface: M5UnsafeRepairConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5SupportQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 Doctor / support surface families that render / consume these components.
    pub surface_families: Vec<M5SupportSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5SupportDeploymentLine>,
    /// Blocked-note anatomy parts this consumer renders (must include the mandatory parts).
    pub note_anatomy_parts: Vec<M5BlockedNoteAnatomyPart>,
    /// Guidance anatomy parts this consumer renders (must include the mandatory parts).
    pub guidance_anatomy_parts: Vec<M5ApprovedRepairGuidanceAnatomyPart>,
    /// Scenario families this consumer distinguishes.
    pub scenario_families: Vec<M5SupportScenarioFamily>,
    /// Doctor finding families this consumer distinguishes.
    pub finding_families: Vec<M5DoctorFindingFamily>,
    /// Unsafe-fix block reasons this consumer distinguishes.
    pub block_reasons: Vec<M5UnsafeFixBlockReason>,
    /// Approved repair classes this consumer distinguishes.
    pub approved_repair_classes: Vec<M5ApprovedRepairClass>,
    /// Redaction states this consumer distinguishes.
    pub redaction_states: Vec<M5SupportRedactionState>,
    /// Case dispositions this consumer distinguishes.
    pub case_dispositions: Vec<M5SupportCaseDisposition>,
    /// Blast radii this consumer distinguishes.
    pub blast_radii: Vec<M5RepairBlastRadius>,
    /// Change classes this consumer distinguishes.
    pub change_classes: Vec<M5RepairChangeClass>,
    /// Reversibilities this consumer distinguishes.
    pub reversibilities: Vec<M5RepairReversibility>,
    /// Blocked-note postures this consumer distinguishes.
    pub note_postures: Vec<M5BlockedNotePosture>,
    /// Bounded blocked-note actions this consumer offers.
    pub note_actions: Vec<M5BlockedNoteAction>,
    /// Guidance postures this consumer distinguishes.
    pub guidance_postures: Vec<M5ApprovedRepairGuidancePosture>,
    /// Bounded guidance actions this consumer offers.
    pub guidance_actions: Vec<M5ApprovedRepairGuidanceAction>,
    /// Blocked-note export fields this consumer carries (must include the mandatory fields).
    pub note_export_fields: Vec<M5BlockedNoteExportField>,
    /// Guidance export fields this consumer carries (must include the mandatory fields).
    pub guidance_export_fields: Vec<M5ApprovedRepairGuidanceExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5SupportAccessibilityRoute>,
    /// Support subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5SupportConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5SupportDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked blocked-note resolutions proving the resolver on this consumer.
    pub note_examples: Vec<M5BlockedNoteResolutionCase>,
    /// Worked approved-repair-guidance resolutions proving the resolver on this consumer.
    pub guidance_examples: Vec<M5ApprovedRepairGuidanceResolutionCase>,
    /// Hard invariant: this consumer never masks the block reason or the recommended safer
    /// repair. MUST be `false`.
    pub masks_block_reason_or_repair: bool,
    /// Hard invariant: this consumer never presents a destructive reset as equivalent to a
    /// reviewed repair transaction. MUST be `false`.
    pub presents_reset_as_reviewed_transaction: bool,
    /// Hard invariant: this consumer never drops the rollback or evidence posture. MUST be
    /// `false`.
    pub drops_rollback_or_evidence_posture: bool,
    /// Hard invariant: this consumer never collapses the guidance into one opaque blob. MUST
    /// be `false`.
    pub collapses_guidance_into_blob: bool,
}

impl M5UnsafeRepairConsumerRow {
    /// True when the row declares every mandatory note and guidance anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let note: BTreeSet<M5BlockedNoteAnatomyPart> =
            self.note_anatomy_parts.iter().copied().collect();
        let guidance: BTreeSet<M5ApprovedRepairGuidanceAnatomyPart> =
            self.guidance_anatomy_parts.iter().copied().collect();
        M5BlockedNoteAnatomyPart::MANDATORY
            .iter()
            .all(|part| note.contains(part))
            && M5ApprovedRepairGuidanceAnatomyPart::MANDATORY
                .iter()
                .all(|part| guidance.contains(part))
    }

    /// True when the row declares every mandatory note and guidance export field.
    fn declares_mandatory_export(&self) -> bool {
        let note: BTreeSet<M5BlockedNoteExportField> =
            self.note_export_fields.iter().copied().collect();
        let guidance: BTreeSet<M5ApprovedRepairGuidanceExportField> =
            self.guidance_export_fields.iter().copied().collect();
        M5BlockedNoteExportField::MANDATORY
            .iter()
            .all(|field| note.contains(field))
            && M5ApprovedRepairGuidanceExportField::MANDATORY
                .iter()
                .all(|field| guidance.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_block_reason_or_repair
            && !self.presents_reset_as_reviewed_transaction
            && !self.drops_rollback_or_evidence_posture
            && !self.collapses_guidance_into_blob
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UnsafeRepairVocabularySet {
    /// Doctor / support consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Blocked-note anatomy-part tokens.
    pub note_anatomy_parts: Vec<String>,
    /// Guidance anatomy-part tokens.
    pub guidance_anatomy_parts: Vec<String>,
    /// Blocked-note posture tokens.
    pub note_postures: Vec<String>,
    /// Blocked-note action tokens.
    pub note_actions: Vec<String>,
    /// Guidance posture tokens.
    pub guidance_postures: Vec<String>,
    /// Guidance action tokens.
    pub guidance_actions: Vec<String>,
    /// Blocked-note export-field tokens.
    pub note_export_fields: Vec<String>,
    /// Guidance export-field tokens.
    pub guidance_export_fields: Vec<String>,
    /// Blast-radius tokens.
    pub blast_radii: Vec<String>,
    /// Change-class tokens.
    pub change_classes: Vec<String>,
    /// Reversibility tokens.
    pub reversibilities: Vec<String>,
    /// Scenario-family tokens (reused from the frozen matrix).
    pub scenario_families: Vec<String>,
    /// Doctor finding-family tokens (reused from the frozen matrix).
    pub finding_families: Vec<String>,
    /// Unsafe-fix block-reason tokens (reused from the frozen matrix).
    pub block_reasons: Vec<String>,
    /// Approved-repair-class tokens (reused from the frozen matrix).
    pub approved_repair_classes: Vec<String>,
    /// Redaction-state tokens (reused from the frozen matrix).
    pub redaction_states: Vec<String>,
    /// Case-disposition tokens (reused from the frozen matrix).
    pub case_dispositions: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5UnsafeRepairVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5UnsafeRepairConsumerSurface::ALL, |v| v.as_str()),
            note_anatomy_parts: tokens(&M5BlockedNoteAnatomyPart::ALL, |v| v.as_str()),
            guidance_anatomy_parts: tokens(&M5ApprovedRepairGuidanceAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            note_postures: tokens(&M5BlockedNotePosture::ALL, |v| v.as_str()),
            note_actions: tokens(&M5BlockedNoteAction::ALL, |v| v.as_str()),
            guidance_postures: tokens(&M5ApprovedRepairGuidancePosture::ALL, |v| v.as_str()),
            guidance_actions: tokens(&M5ApprovedRepairGuidanceAction::ALL, |v| v.as_str()),
            note_export_fields: tokens(&M5BlockedNoteExportField::ALL, |v| v.as_str()),
            guidance_export_fields: tokens(&M5ApprovedRepairGuidanceExportField::ALL, |v| {
                v.as_str()
            }),
            blast_radii: tokens(&M5RepairBlastRadius::ALL, |v| v.as_str()),
            change_classes: tokens(&M5RepairChangeClass::ALL, |v| v.as_str()),
            reversibilities: tokens(&M5RepairReversibility::ALL, |v| v.as_str()),
            scenario_families: tokens(&M5SupportScenarioFamily::ALL, |v| v.as_str()),
            finding_families: tokens(&M5DoctorFindingFamily::ALL, |v| v.as_str()),
            block_reasons: tokens(&M5UnsafeFixBlockReason::ALL, |v| v.as_str()),
            approved_repair_classes: tokens(&M5ApprovedRepairClass::ALL, |v| v.as_str()),
            redaction_states: tokens(&M5SupportRedactionState::ALL, |v| v.as_str()),
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
pub struct M5UnsafeRepairGovernanceReview {
    /// The blocked note shows the blocked action and why it is not approved for the scenario.
    pub note_shows_blocked_action_and_reason: bool,
    /// The blocked note shows the recommended safer repair.
    pub note_shows_recommended_safer_repair: bool,
    /// The blocked note shows the preserved rollback and evidence posture.
    pub note_shows_rollback_and_evidence_posture: bool,
    /// The blocked note always offers dismiss and preserve-evidence affordances.
    pub note_always_offers_dismiss_and_preserve_evidence: bool,
    /// A destructive reset suggestion never reads as equivalent to a reviewed repair
    /// transaction.
    pub destructive_reset_never_equals_reviewed_transaction: bool,
    /// The approved-repair guidance shows its blast radius and changed / unchanged classes.
    pub guidance_shows_blast_radius_and_change_classes: bool,
    /// The approved-repair guidance keeps the user-decline continuity explicit.
    pub guidance_shows_decline_continuity: bool,
    /// A user can see why a recommended repair is safer and what evidence remains if they
    /// decline it.
    pub user_can_see_why_safer_and_evidence_remains: bool,
    /// An explicit approval is required before an irreversible repair can proceed.
    pub approval_required_before_irreversible_repair: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components keep the same truth across desktop, headless / export, and support
    /// consumers.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs the block and guidance truth.
    pub support_export_reconstructs_block_and_guidance_truth: bool,
    /// Later M5 rows cannot invent parallel reason / repair / evidence vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
    /// No consumer masks the block reason, the recommended repair, or the evidence.
    pub no_surface_masks_reason_repair_or_evidence: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UnsafeRepairConsumerProjection {
    /// Doctor and support surfaces consume the shared reason / repair vocabulary.
    pub doctor_and_support_surfaces_consume_reason_vocabulary: bool,
    /// The note-posture resolver reads a single canonical source.
    pub note_posture_reads_single_source: bool,
    /// The guidance-posture resolver reads a single canonical source.
    pub guidance_posture_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop consumers read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UnsafeRepairProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the two components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UnsafeRepairReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting support-case audit.
    pub support_case_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5UnsafeRepairPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5UnsafeRepairPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Doctor / support rows.
    pub rows: Vec<M5UnsafeRepairConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5UnsafeRepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5UnsafeRepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5UnsafeRepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5UnsafeRepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5UnsafeRepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 unsafe-fix-blocked-note / approved-repair-guidance primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5UnsafeRepairPacket {
    /// Record kind; must equal [`M5_UNSAFE_REPAIR_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_UNSAFE_REPAIR_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Doctor / support rows.
    pub rows: Vec<M5UnsafeRepairConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5UnsafeRepairVocabularySet,
    /// Governance-review block.
    pub governance_review: M5UnsafeRepairGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5UnsafeRepairConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5UnsafeRepairProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5UnsafeRepairReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5UnsafeRepairPacket {
    /// Builds an M5 unsafe-fix / approved-repair primitive packet from stable-lane input.
    pub fn new(input: M5UnsafeRepairPacketInput) -> Self {
        Self {
            record_kind: M5_UNSAFE_REPAIR_RECORD_KIND.to_owned(),
            schema_version: M5_UNSAFE_REPAIR_SCHEMA_VERSION,
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

    /// Validates the M5 unsafe-fix / approved-repair primitive invariants.
    pub fn validate(&self) -> Vec<M5UnsafeRepairViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_UNSAFE_REPAIR_RECORD_KIND {
            violations.push(M5UnsafeRepairViolation::WrongRecordKind);
        }
        if self.schema_version != M5_UNSAFE_REPAIR_SCHEMA_VERSION {
            violations.push(M5UnsafeRepairViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5UnsafeRepairViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_note_posture_coverage(self, &mut violations);
        validate_guidance_posture_coverage(self, &mut violations);
        validate_scenario_lineage_coverage(self, &mut violations);
        validate_block_reason_coverage(self, &mut violations);
        validate_recommended_repair_coverage(self, &mut violations);
        validate_redaction_state_coverage(self, &mut violations);
        validate_case_disposition_coverage(self, &mut violations);
        validate_blast_radius_coverage(self, &mut violations);
        validate_reversibility_coverage(self, &mut violations);
        validate_change_class_coverage(self, &mut violations);
        validate_note_gating_coverage(self, &mut violations);
        validate_irreversible_distinction_coverage(self, &mut violations);
        validate_reviewed_transaction_coverage(self, &mut violations);
        validate_lineage_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 unsafe repair primitive packet serializes"),
        ) {
            violations.push(M5UnsafeRepairViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 unsafe repair primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per Doctor / support consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,scenario_families,block_reasons,approved_repair_classes,blast_radii,reversibilities,note_postures,guidance_postures,note_examples,guidance_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.scenario_families, |v| v.as_str()),
                join_tokens(&row.block_reasons, |v| v.as_str()),
                join_tokens(&row.approved_repair_classes, |v| v.as_str()),
                join_tokens(&row.blast_radii, |v| v.as_str()),
                join_tokens(&row.reversibilities, |v| v.as_str()),
                join_tokens(&row.note_postures, |v| v.as_str()),
                join_tokens(&row.guidance_postures, |v| v.as_str()),
                row.note_examples.len(),
                row.guidance_examples.len(),
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
        out.push_str("# M5 Unsafe-Fix-Blocked-Note / Approved-Repair-Guidance Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Doctor / support consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Note postures: {}\n",
            self.vocabulary_set.note_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Guidance postures: {}\n",
            self.vocabulary_set.guidance_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Block reasons: {}\n",
            self.vocabulary_set.block_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Approved repair classes: {}\n",
            self.vocabulary_set.approved_repair_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Doctor / support consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked blocked notes: {}\n",
                row.note_examples.len()
            ));
            for case in &row.note_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (safer `{}`, rollback `{}`, evidence `{}`)\n",
                    case.resolved.note_id,
                    case.resolved.block_reason.as_str(),
                    case.resolved.note_posture.as_str(),
                    case.resolved.safer_repair_offered,
                    case.resolved.rollback_preserved,
                    case.resolved.evidence_preserved,
                ));
            }
            out.push_str(&format!(
                "  - Worked repair guidance: {}\n",
                row.guidance_examples.len()
            ));
            for case in &row.guidance_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (reviewed `{}`, decline-keeps-evidence `{}`)\n",
                    case.resolved.guidance_id,
                    case.resolved.repair_class.as_str(),
                    case.resolved.guidance_posture.as_str(),
                    case.resolved.is_reviewed_transaction,
                    case.resolved.decline_keeps_evidence,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 unsafe-fix / approved-repair primitive
/// export.
#[derive(Debug)]
pub enum M5UnsafeRepairArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5UnsafeRepairViolation>),
}

impl fmt::Display for M5UnsafeRepairArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 unsafe repair primitive export parse failed: {error}"
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
                    "m5 unsafe repair primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5UnsafeRepairArtifactError {}

/// Validation failures emitted by [`M5UnsafeRepairPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5UnsafeRepairViolation {
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
    /// A required Doctor / support consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A Doctor / support row is incomplete.
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
    /// A row declares no worked blocked-note or guidance resolutions.
    WorkedExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every blocked-note posture.
    NotePostureCoverageUnproven,
    /// The worked resolutions do not exercise every guidance posture.
    GuidancePostureCoverageUnproven,
    /// The worked resolutions do not exercise every scenario family or finding family.
    ScenarioLineageCoverageUnproven,
    /// The worked resolutions do not exercise every unsafe-fix block reason.
    BlockReasonCoverageUnproven,
    /// The worked resolutions do not exercise every approved repair class.
    RecommendedRepairCoverageUnproven,
    /// The worked resolutions do not exercise every redaction state.
    RedactionStateCoverageUnproven,
    /// The worked resolutions do not exercise every case disposition.
    CaseDispositionCoverageUnproven,
    /// The worked resolutions do not exercise every blast radius.
    BlastRadiusCoverageUnproven,
    /// The worked resolutions do not exercise every reversibility.
    ReversibilityCoverageUnproven,
    /// The worked resolutions do not exercise every change class.
    ChangeClassCoverageUnproven,
    /// The worked resolutions do not prove both a safer-repair-offered and a no-safe-repair
    /// note.
    NoteGatingCoverageUnproven,
    /// The worked resolutions do not prove an irreversible-blocked note.
    IrreversibleDistinctionCoverageUnproven,
    /// The worked resolutions do not prove both a reviewed and a non-reviewed repair.
    ReviewedTransactionCoverageUnproven,
    /// A worked resolution collapses or drops its lineage, evidence, or guidance.
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

impl M5UnsafeRepairViolation {
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
            Self::NotePostureCoverageUnproven => "note_posture_coverage_unproven",
            Self::GuidancePostureCoverageUnproven => "guidance_posture_coverage_unproven",
            Self::ScenarioLineageCoverageUnproven => "scenario_lineage_coverage_unproven",
            Self::BlockReasonCoverageUnproven => "block_reason_coverage_unproven",
            Self::RecommendedRepairCoverageUnproven => "recommended_repair_coverage_unproven",
            Self::RedactionStateCoverageUnproven => "redaction_state_coverage_unproven",
            Self::CaseDispositionCoverageUnproven => "case_disposition_coverage_unproven",
            Self::BlastRadiusCoverageUnproven => "blast_radius_coverage_unproven",
            Self::ReversibilityCoverageUnproven => "reversibility_coverage_unproven",
            Self::ChangeClassCoverageUnproven => "change_class_coverage_unproven",
            Self::NoteGatingCoverageUnproven => "note_gating_coverage_unproven",
            Self::IrreversibleDistinctionCoverageUnproven => {
                "irreversible_distinction_coverage_unproven"
            }
            Self::ReviewedTransactionCoverageUnproven => "reviewed_transaction_coverage_unproven",
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

/// Reads and validates the checked-in stable M5 unsafe-fix / approved-repair primitive
/// export.
pub fn current_stable_m5_unsafe_repair_export(
) -> Result<M5UnsafeRepairPacket, M5UnsafeRepairArtifactError> {
    let packet: M5UnsafeRepairPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive-proof/support_export.json"
    )))
    .map_err(M5UnsafeRepairArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5UnsafeRepairArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_UNSAFE_REPAIR_SCHEMA_REF,
        M5_UNSAFE_REPAIR_DOC_REF,
        M5_UNSAFE_REPAIR_COMPONENT_MATRIX_REF,
        M5_UNSAFE_REPAIR_REPAIR_TRANSACTION_REF,
        M5_UNSAFE_REPAIR_RECOVERY_ACTION_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5UnsafeRepairViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5UnsafeRepairViolation::VocabularySetDrift);
    }
}

fn validate_rows(packet: &M5UnsafeRepairPacket, violations: &mut Vec<M5UnsafeRepairViolation>) {
    let present: BTreeSet<M5UnsafeRepairConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5UnsafeRepairConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5UnsafeRepairViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.note_anatomy_parts.is_empty()
            || row.guidance_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.scenario_families.is_empty()
            || row.finding_families.is_empty()
            || row.block_reasons.is_empty()
            || row.approved_repair_classes.is_empty()
            || row.redaction_states.is_empty()
            || row.case_dispositions.is_empty()
            || row.blast_radii.is_empty()
            || row.change_classes.is_empty()
            || row.reversibilities.is_empty()
            || row.note_postures.is_empty()
            || row.note_actions.is_empty()
            || row.guidance_postures.is_empty()
            || row.guidance_actions.is_empty()
            || row.note_export_fields.is_empty()
            || row.guidance_export_fields.is_empty()
        {
            violations.push(M5UnsafeRepairViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5UnsafeRepairViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5UnsafeRepairViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5SupportAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5UnsafeRepairViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5UnsafeRepairViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5UnsafeRepairViolation::DowngradeTriggersMissing);
        }
        if row.note_examples.is_empty() || row.guidance_examples.is_empty() {
            violations.push(M5UnsafeRepairViolation::WorkedExampleMissing);
        }
        if row
            .note_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .guidance_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5UnsafeRepairViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5UnsafeRepairViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5UnsafeRepairViolation::RowInvariantViolated);
        }
    }
}

fn note_cases(packet: &M5UnsafeRepairPacket) -> impl Iterator<Item = &M5BlockedNoteResolutionCase> {
    packet.rows.iter().flat_map(|row| row.note_examples.iter())
}

fn guidance_cases(
    packet: &M5UnsafeRepairPacket,
) -> impl Iterator<Item = &M5ApprovedRepairGuidanceResolutionCase> {
    packet
        .rows
        .iter()
        .flat_map(|row| row.guidance_examples.iter())
}

/// Every blocked-note posture must be exercised by some worked resolution.
fn validate_note_posture_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5BlockedNotePosture> = note_cases(packet)
        .map(|case| case.resolved.note_posture)
        .collect();
    if !M5BlockedNotePosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5UnsafeRepairViolation::NotePostureCoverageUnproven);
    }
}

/// Every guidance posture must be exercised by some worked resolution.
fn validate_guidance_posture_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5ApprovedRepairGuidancePosture> = guidance_cases(packet)
        .map(|case| case.resolved.guidance_posture)
        .collect();
    if !M5ApprovedRepairGuidancePosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5UnsafeRepairViolation::GuidancePostureCoverageUnproven);
    }
}

/// Every scenario family and every finding family must appear in some worked blocked note,
/// so the scenario / finding lineage is proven end to end.
fn validate_scenario_lineage_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let scenarios: BTreeSet<M5SupportScenarioFamily> = note_cases(packet)
        .map(|case| case.resolved.scenario_family)
        .collect();
    let findings: BTreeSet<M5DoctorFindingFamily> = note_cases(packet)
        .flat_map(|case| case.resolved.finding_families.iter().copied())
        .collect();
    let covered = M5SupportScenarioFamily::ALL
        .iter()
        .all(|scenario| scenarios.contains(scenario))
        && M5DoctorFindingFamily::ALL
            .iter()
            .all(|finding| findings.contains(finding));
    if !covered {
        violations.push(M5UnsafeRepairViolation::ScenarioLineageCoverageUnproven);
    }
}

/// Every unsafe-fix block reason must be exercised, so "why the fix is blocked" is proven
/// across the full vocabulary.
fn validate_block_reason_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5UnsafeFixBlockReason> = note_cases(packet)
        .map(|case| case.resolved.block_reason)
        .collect();
    if !M5UnsafeFixBlockReason::ALL
        .iter()
        .all(|reason| exercised.contains(reason))
    {
        violations.push(M5UnsafeRepairViolation::BlockReasonCoverageUnproven);
    }
}

/// Every approved repair class must appear as some blocked note's recommended safer repair,
/// so the safe-repair vocabulary is proven end to end.
fn validate_recommended_repair_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5ApprovedRepairClass> = note_cases(packet)
        .map(|case| case.resolved.recommended_repair)
        .collect();
    if !M5ApprovedRepairClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5UnsafeRepairViolation::RecommendedRepairCoverageUnproven);
    }
}

/// Every redaction state must be exercised.
fn validate_redaction_state_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5SupportRedactionState> = note_cases(packet)
        .map(|case| case.resolved.redaction_state)
        .collect();
    if !M5SupportRedactionState::ALL
        .iter()
        .all(|state| exercised.contains(state))
    {
        violations.push(M5UnsafeRepairViolation::RedactionStateCoverageUnproven);
    }
}

/// Every case disposition must be exercised.
fn validate_case_disposition_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5SupportCaseDisposition> = note_cases(packet)
        .map(|case| case.resolved.case_disposition)
        .collect();
    if !M5SupportCaseDisposition::ALL
        .iter()
        .all(|disposition| exercised.contains(disposition))
    {
        violations.push(M5UnsafeRepairViolation::CaseDispositionCoverageUnproven);
    }
}

/// Every blast radius must be exercised.
fn validate_blast_radius_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5RepairBlastRadius> = guidance_cases(packet)
        .map(|case| case.resolved.blast_radius)
        .collect();
    if !M5RepairBlastRadius::ALL
        .iter()
        .all(|radius| exercised.contains(radius))
    {
        violations.push(M5UnsafeRepairViolation::BlastRadiusCoverageUnproven);
    }
}

/// Every reversibility must be exercised.
fn validate_reversibility_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5RepairReversibility> = guidance_cases(packet)
        .map(|case| case.resolved.reversibility)
        .collect();
    if !M5RepairReversibility::ALL
        .iter()
        .all(|reversibility| exercised.contains(reversibility))
    {
        violations.push(M5UnsafeRepairViolation::ReversibilityCoverageUnproven);
    }
}

/// Every change class must appear as a changed or unchanged class in some guidance, so the
/// changed-versus-unchanged surface is proven across the full vocabulary.
fn validate_change_class_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let exercised: BTreeSet<M5RepairChangeClass> = guidance_cases(packet)
        .flat_map(|case| {
            case.resolved
                .changed_classes
                .iter()
                .chain(case.resolved.unchanged_classes.iter())
                .copied()
        })
        .collect();
    if !M5RepairChangeClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5UnsafeRepairViolation::ChangeClassCoverageUnproven);
    }
}

/// At least one worked note must prove a safer repair is offered (with the view-safer-repair
/// action) and at least one must prove no safe repair is available (without it), so a safer
/// repair is never faked and never silently required.
fn validate_note_gating_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let has_safer = note_cases(packet).any(|case| {
        case.resolved.safer_repair_offered
            && case
                .resolved
                .available_actions
                .contains(&M5BlockedNoteAction::ViewSaferRepair)
    });
    let has_none = note_cases(packet).any(|case| {
        !case.resolved.safer_repair_offered
            && !case
                .resolved
                .available_actions
                .contains(&M5BlockedNoteAction::ViewSaferRepair)
    });
    if !(has_safer && has_none) {
        violations.push(M5UnsafeRepairViolation::NoteGatingCoverageUnproven);
    }
}

/// At least one worked note must prove an irreversible-blocked posture — the AC-1 requirement
/// that a destructive irreversible fix is blocked rather than presented as an approved
/// transaction.
fn validate_irreversible_distinction_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    if !note_cases(packet).any(|case| {
        matches!(
            case.resolved.note_posture,
            M5BlockedNotePosture::IrreversibleBlocked
        )
    }) {
        violations.push(M5UnsafeRepairViolation::IrreversibleDistinctionCoverageUnproven);
    }
}

/// At least one worked guidance must prove a reviewed reversible transaction and at least one
/// must prove a non-reviewed (irreversible or partial) repair, so the AC-1 distinction
/// between a reviewed transaction and a destructive change is proven both ways.
fn validate_reviewed_transaction_coverage(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let has_reviewed = guidance_cases(packet).any(|case| case.resolved.is_reviewed_transaction);
    let has_unreviewed = guidance_cases(packet).any(|case| !case.resolved.is_reviewed_transaction);
    if !(has_reviewed && has_unreviewed) {
        violations.push(M5UnsafeRepairViolation::ReviewedTransactionCoverageUnproven);
    }
}

/// Every worked resolution must preserve its lineage exactly — the acceptance criteria that
/// the block reason, safer repair, evidence, blast radius, and change classes stay legible
/// without collapsing into one opaque blob.
fn validate_lineage_preservation(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let note_ok = note_cases(packet).all(|case| case.preserves_lineage());
    let guidance_ok = guidance_cases(packet).all(|case| case.preserves_lineage());
    if !(note_ok && guidance_ok) {
        violations.push(M5UnsafeRepairViolation::LineagePreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.note_shows_blocked_action_and_reason,
        review.note_shows_recommended_safer_repair,
        review.note_shows_rollback_and_evidence_posture,
        review.note_always_offers_dismiss_and_preserve_evidence,
        review.destructive_reset_never_equals_reviewed_transaction,
        review.guidance_shows_blast_radius_and_change_classes,
        review.guidance_shows_decline_continuity,
        review.user_can_see_why_safer_and_evidence_remains,
        review.approval_required_before_irreversible_repair,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_block_and_guidance_truth,
        review.later_rows_cannot_invent_parallel_vocabulary,
        review.no_surface_masks_reason_repair_or_evidence,
    ] {
        if !ok {
            violations.push(M5UnsafeRepairViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.doctor_and_support_surfaces_consume_reason_vocabulary,
        projection.note_posture_reads_single_source,
        projection.guidance_posture_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5UnsafeRepairViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5UnsafeRepairViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5UnsafeRepairPacket,
    violations: &mut Vec<M5UnsafeRepairViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.support_case_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5UnsafeRepairViolation::ReleasePostureIncomplete);
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
