//! Frozen M5 support-scenario-picker-row, issue-report-builder-step,
//! escalation-packet-summary, handoff-timeline-row, and unsafe-fix-blocked-note
//! component matrix.
//!
//! This module locks Aureline's reusable support-intake and escalation components
//! into one export-safe packet. Every supportability- and escalation-facing
//! subcomponent M5 claims that still drifts too easily by Project Doctor, support
//! center, Help, recovery-center, or admin surface — the support-scenario picker row,
//! the issue-report builder step, the escalation-packet summary, the handoff-timeline
//! row, and the unsafe-fix blocked note — is named once here and constrained by the
//! same scenario family, incident scope, Doctor finding lineage, selected and omitted
//! evidence classes, approved repair class, packet destination, redaction state, case
//! disposition, and next human step regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves:
//! the component families, the scenario families, incident scopes, and Doctor finding
//! families the picker row binds, the report-builder step kinds and evidence classes,
//! the escalation-packet destinations and redaction states, the handoff stages and
//! next human steps, the unsafe-fix block reasons and approved repair classes, the
//! shared case dispositions (local-only, vendor-case, uncategorized, unsafe-fix-
//! blocked), the deployment lines every component must survive, the non-visual
//! accessibility routes, and the mandatory labels every component must be able to
//! show. It does not re-architect Doctor finding generation, repair engines, or
//! support-bundle storage that already own those records — it is the shared support-
//! intake / escalation contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 Doctor, support
//! center, Help, recovery-center, admin, or CLI support surface may publish a
//! scenario, scope, evidence, repair, destination, redaction, or next-step claim.
//! Doctor, support-center, report-builder, escalation-desk, recovery-center, Help, and
//! admin consumers all read this packet so one scenario picker row names its scenario
//! family, incident scope, and the Doctor finding family it binds, one report-builder
//! step names which evidence classes it selects and omits, one escalation-packet
//! summary names its destination and redaction, one handoff-timeline row names its
//! stage and the next human step, and one unsafe-fix blocked note names why a repair
//! is blocked and which repair class is approved instead. No M5 lane invents a second
//! support grammar or an alternate label for the local-only, vendor-case,
//! uncategorized, or unsafe-fix-blocked states.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5SupportIntakeEscalationComponentVocabularySet`] rather than minted per surface.
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! support boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_support_intake_escalation_component_matrix,
    seeded_m5_support_intake_escalation_component_matrix_escalation_packet_summary_beta_narrowed,
    seeded_m5_support_intake_escalation_component_matrix_unsafe_fix_blocked_note_preview_narrowed,
    M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SupportIntakeEscalationComponentMatrixPacket`].
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix";

/// Schema version for M5 support-intake / escalation component-matrix records.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the support-intake / escalation component boundary schema.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOC_REF: &str =
    "docs/support/m5_support_intake_escalation_component_matrix.md";

/// Repo-relative path of the scenario-picker contract this matrix binds against.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCENARIO_PICKER_REF: &str =
    "schemas/support/scenario_picker.schema.json";

/// Repo-relative path of the Doctor-finding contract this matrix binds against.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOCTOR_FINDING_REF: &str =
    "schemas/support/doctor_finding.schema.json";

/// Repo-relative path of the escalation-packet contract this matrix binds against.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ESCALATION_PACKET_REF: &str =
    "schemas/support/escalation_packet.schema.json";

/// Repo-relative path of the recovery-action (approved-repair) contract this matrix
/// binds against.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_RECOVERY_ACTION_REF: &str =
    "schemas/support/recovery_action.schema.json";

/// Repo-relative path of the export-redaction-profile contract this matrix binds
/// against.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_REDACTION_PROFILE_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-intake-escalation-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-support-intake-escalation-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-support-intake-escalation-component-matrix.md";

/// One of the five governed support-intake / escalation component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeEscalationComponentFamily {
    /// A support-scenario picker row carrying a scenario family, incident scope, and
    /// bound Doctor finding family.
    SupportScenarioPickerRow,
    /// An issue-report builder step carrying its step kind and evidence classes.
    IssueReportBuilderStep,
    /// An escalation-packet summary carrying its packet destination and redaction
    /// state.
    EscalationPacketSummary,
    /// A handoff-timeline row carrying its handoff stage and next human step.
    HandoffTimelineRow,
    /// An unsafe-fix blocked note carrying its block reason and approved repair class.
    UnsafeFixBlockedNote,
}

impl M5SupportIntakeEscalationComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SupportScenarioPickerRow,
        Self::IssueReportBuilderStep,
        Self::EscalationPacketSummary,
        Self::HandoffTimelineRow,
        Self::UnsafeFixBlockedNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportScenarioPickerRow => "support_scenario_picker_row",
            Self::IssueReportBuilderStep => "issue_report_builder_step",
            Self::EscalationPacketSummary => "escalation_packet_summary",
            Self::HandoffTimelineRow => "handoff_timeline_row",
            Self::UnsafeFixBlockedNote => "unsafe_fix_blocked_note",
        }
    }

    /// `true` when this family is a support-scenario picker row and must therefore
    /// declare its scenario families, incident scopes, and Doctor finding families.
    pub const fn is_support_scenario_picker_row(self) -> bool {
        matches!(self, Self::SupportScenarioPickerRow)
    }

    /// `true` when this family is an issue-report builder step and must therefore
    /// declare its builder step kinds and evidence classes.
    pub const fn is_issue_report_builder_step(self) -> bool {
        matches!(self, Self::IssueReportBuilderStep)
    }

    /// `true` when this family is an escalation-packet summary and must therefore
    /// declare its packet destinations and redaction states.
    pub const fn is_escalation_packet_summary(self) -> bool {
        matches!(self, Self::EscalationPacketSummary)
    }

    /// `true` when this family is a handoff-timeline row and must therefore declare its
    /// handoff stages and next human steps.
    pub const fn is_handoff_timeline_row(self) -> bool {
        matches!(self, Self::HandoffTimelineRow)
    }

    /// `true` when this family is an unsafe-fix blocked note and must therefore declare
    /// its block reasons and approved repair classes.
    pub const fn is_unsafe_fix_blocked_note(self) -> bool {
        matches!(self, Self::UnsafeFixBlockedNote)
    }
}

/// Controlled scenario family — what class of problem a support scenario names, bound
/// to the Doctor finding families, so a picker row never leaves the scenario implicit
/// or invents a parallel taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportScenarioFamily {
    /// A crash / restart-loop recovery scenario.
    CrashRecovery,
    /// A performance / responsiveness health scenario.
    PerformanceHealth,
    /// An extension / plugin conflict scenario.
    ExtensionConflict,
    /// A data-integrity / workspace-corruption scenario.
    DataIntegrity,
    /// A connectivity / sync scenario.
    ConnectivitySync,
    /// An uncategorized scenario that does not yet map to a Doctor finding family.
    UncategorizedScenario,
}

impl M5SupportScenarioFamily {
    /// Every scenario family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CrashRecovery,
        Self::PerformanceHealth,
        Self::ExtensionConflict,
        Self::DataIntegrity,
        Self::ConnectivitySync,
        Self::UncategorizedScenario,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashRecovery => "crash_recovery",
            Self::PerformanceHealth => "performance_health",
            Self::ExtensionConflict => "extension_conflict",
            Self::DataIntegrity => "data_integrity",
            Self::ConnectivitySync => "connectivity_sync",
            Self::UncategorizedScenario => "uncategorized_scenario",
        }
    }
}

/// Controlled incident scope — how wide the incident a scenario picker row names
/// reaches, so scope is never left implicit or understated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIncidentScope {
    /// A single file.
    SingleFile,
    /// The whole workspace.
    Workspace,
    /// The account.
    Account,
    /// The device / host.
    DeviceHost,
    /// A remote service.
    RemoteService,
    /// An unknown / not-yet-determined scope.
    UnknownScope,
}

impl M5SupportIncidentScope {
    /// Every incident scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleFile,
        Self::Workspace,
        Self::Account,
        Self::DeviceHost,
        Self::RemoteService,
        Self::UnknownScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::Workspace => "workspace",
            Self::Account => "account",
            Self::DeviceHost => "device_host",
            Self::RemoteService => "remote_service",
            Self::UnknownScope => "unknown_scope",
        }
    }
}

/// Controlled Doctor finding family the scenario picker row binds to, so scenario
/// vocabulary is bound to the Doctor finding families rather than reinvented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DoctorFindingFamily {
    /// A startup / restart-loop health finding.
    StartupHealth,
    /// An index / search-integrity finding.
    IndexIntegrity,
    /// A storage-pressure finding.
    StoragePressure,
    /// An extension / plugin-fault finding.
    ExtensionFault,
    /// A sync / connectivity finding.
    SyncConnectivity,
    /// An uncategorized finding with no committed family yet.
    UncategorizedFinding,
}

impl M5DoctorFindingFamily {
    /// Every Doctor finding family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StartupHealth,
        Self::IndexIntegrity,
        Self::StoragePressure,
        Self::ExtensionFault,
        Self::SyncConnectivity,
        Self::UncategorizedFinding,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartupHealth => "startup_health",
            Self::IndexIntegrity => "index_integrity",
            Self::StoragePressure => "storage_pressure",
            Self::ExtensionFault => "extension_fault",
            Self::SyncConnectivity => "sync_connectivity",
            Self::UncategorizedFinding => "uncategorized_finding",
        }
    }
}

/// Controlled report-builder step kind — which step of the issue-report builder a step
/// component represents, so a builder never collapses or skips a step silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReportBuilderStepKind {
    /// Choose the scenario.
    ChooseScenario,
    /// Describe the symptom.
    DescribeSymptom,
    /// Attach evidence.
    AttachEvidence,
    /// Review redaction.
    ReviewRedaction,
    /// Confirm the scope.
    ConfirmScope,
    /// Submit or export.
    SubmitOrExport,
}

impl M5ReportBuilderStepKind {
    /// Every builder step kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ChooseScenario,
        Self::DescribeSymptom,
        Self::AttachEvidence,
        Self::ReviewRedaction,
        Self::ConfirmScope,
        Self::SubmitOrExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChooseScenario => "choose_scenario",
            Self::DescribeSymptom => "describe_symptom",
            Self::AttachEvidence => "attach_evidence",
            Self::ReviewRedaction => "review_redaction",
            Self::ConfirmScope => "confirm_scope",
            Self::SubmitOrExport => "submit_or_export",
        }
    }
}

/// Controlled evidence class — what class of evidence an issue-report step selects or
/// omits, so selected and omitted evidence is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportEvidenceClass {
    /// A Doctor finding.
    DoctorFinding,
    /// Crash forensics.
    CrashForensics,
    /// A repair transaction.
    RepairTransaction,
    /// An activity timeline.
    ActivityTimeline,
    /// An environment snapshot.
    EnvironmentSnapshot,
    /// A user note.
    UserNote,
}

impl M5SupportEvidenceClass {
    /// Every evidence class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DoctorFinding,
        Self::CrashForensics,
        Self::RepairTransaction,
        Self::ActivityTimeline,
        Self::EnvironmentSnapshot,
        Self::UserNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorFinding => "doctor_finding",
            Self::CrashForensics => "crash_forensics",
            Self::RepairTransaction => "repair_transaction",
            Self::ActivityTimeline => "activity_timeline",
            Self::EnvironmentSnapshot => "environment_snapshot",
            Self::UserNote => "user_note",
        }
    }
}

/// Controlled escalation-packet destination — where an escalation packet is bound, so
/// a summary never leaves the destination implicit or mislabels a local-only bundle as
/// a shared case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EscalationPacketDestination {
    /// A local-only bundle that never leaves the device.
    LocalOnlyBundle,
    /// A self-serve export the user shares themselves.
    SelfServeExport,
    /// A vendor support case.
    VendorSupportCase,
    /// An enterprise admin queue.
    EnterpriseAdmin,
    /// A community forum.
    CommunityForum,
    /// A blocked destination the packet cannot yet reach.
    BlockedDestination,
}

impl M5EscalationPacketDestination {
    /// Every packet destination, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnlyBundle,
        Self::SelfServeExport,
        Self::VendorSupportCase,
        Self::EnterpriseAdmin,
        Self::CommunityForum,
        Self::BlockedDestination,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyBundle => "local_only_bundle",
            Self::SelfServeExport => "self_serve_export",
            Self::VendorSupportCase => "vendor_support_case",
            Self::EnterpriseAdmin => "enterprise_admin",
            Self::CommunityForum => "community_forum",
            Self::BlockedDestination => "blocked_destination",
        }
    }
}

/// Controlled redaction state — how an escalation packet redacts on export, so a
/// summary never shows a redacted packet as a full export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportRedactionState {
    /// Full metadata export.
    FullMetadata,
    /// Paths redacted.
    PathsRedacted,
    /// Bodies omitted.
    BodiesOmitted,
    /// Credentials scrubbed.
    CredentialsScrubbed,
    /// Restricted by policy.
    PolicyRestricted,
    /// Export blocked.
    ExportBlocked,
}

impl M5SupportRedactionState {
    /// Every redaction state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullMetadata,
        Self::PathsRedacted,
        Self::BodiesOmitted,
        Self::CredentialsScrubbed,
        Self::PolicyRestricted,
        Self::ExportBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullMetadata => "full_metadata",
            Self::PathsRedacted => "paths_redacted",
            Self::BodiesOmitted => "bodies_omitted",
            Self::CredentialsScrubbed => "credentials_scrubbed",
            Self::PolicyRestricted => "policy_restricted",
            Self::ExportBlocked => "export_blocked",
        }
    }
}

/// Controlled handoff stage — where in the diagnosis-to-handoff timeline a row sits, so
/// a timeline never collapses or skips a stage silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffStage {
    /// Diagnosis has started.
    DiagnosisStarted,
    /// A repair was suggested.
    RepairSuggested,
    /// A repair was attempted.
    RepairAttempted,
    /// A case was built.
    CaseBuilt,
    /// The case was handed off.
    HandedOff,
    /// Awaiting a human response.
    AwaitingHuman,
}

impl M5HandoffStage {
    /// Every handoff stage, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DiagnosisStarted,
        Self::RepairSuggested,
        Self::RepairAttempted,
        Self::CaseBuilt,
        Self::HandedOff,
        Self::AwaitingHuman,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosisStarted => "diagnosis_started",
            Self::RepairSuggested => "repair_suggested",
            Self::RepairAttempted => "repair_attempted",
            Self::CaseBuilt => "case_built",
            Self::HandedOff => "handed_off",
            Self::AwaitingHuman => "awaiting_human",
        }
    }
}

/// Controlled next human step — what the user or a human owner should do next, so the
/// next step is always explicit and never left as a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NextHumanStep {
    /// Run Project Doctor.
    RunDoctor,
    /// Apply the approved repair.
    ApplyApprovedRepair,
    /// Gather more evidence.
    GatherMoreEvidence,
    /// Export the bundle.
    ExportBundle,
    /// Contact the vendor.
    ContactVendor,
    /// Wait for a response.
    WaitForResponse,
}

impl M5NextHumanStep {
    /// Every next human step, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunDoctor,
        Self::ApplyApprovedRepair,
        Self::GatherMoreEvidence,
        Self::ExportBundle,
        Self::ContactVendor,
        Self::WaitForResponse,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunDoctor => "run_doctor",
            Self::ApplyApprovedRepair => "apply_approved_repair",
            Self::GatherMoreEvidence => "gather_more_evidence",
            Self::ExportBundle => "export_bundle",
            Self::ContactVendor => "contact_vendor",
            Self::WaitForResponse => "wait_for_response",
        }
    }
}

/// Controlled unsafe-fix block reason — why a suggested fix is blocked, so an unsafe-
/// fix note never hides why a repair cannot be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UnsafeFixBlockReason {
    /// Explicit approval is required.
    ApprovalRequired,
    /// The change is irreversible.
    IrreversibleChange,
    /// The repair is out of scope.
    OutOfScopeRepair,
    /// There is insufficient evidence.
    InsufficientEvidence,
    /// The repair is blocked by policy.
    PolicyBlocked,
    /// The scenario is unsupported.
    UnsupportedScenario,
}

impl M5UnsafeFixBlockReason {
    /// Every block reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ApprovalRequired,
        Self::IrreversibleChange,
        Self::OutOfScopeRepair,
        Self::InsufficientEvidence,
        Self::PolicyBlocked,
        Self::UnsupportedScenario,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApprovalRequired => "approval_required",
            Self::IrreversibleChange => "irreversible_change",
            Self::OutOfScopeRepair => "out_of_scope_repair",
            Self::InsufficientEvidence => "insufficient_evidence",
            Self::PolicyBlocked => "policy_blocked",
            Self::UnsupportedScenario => "unsupported_scenario",
        }
    }
}

/// Controlled approved repair class — which repair class is approved instead of a
/// blocked unsafe fix, bound to the recovery-ladder repair classes, so an unsafe-fix
/// note always names the safe repair a user may take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovedRepairClass {
    /// A cache rebuild.
    CacheRebuild,
    /// An index repair.
    IndexRepair,
    /// A settings repair.
    SettingsRepair,
    /// A state migration.
    StateMigration,
    /// A targeted reset.
    TargetedReset,
    /// No safe repair is available.
    NoSafeRepair,
}

impl M5ApprovedRepairClass {
    /// Every approved repair class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CacheRebuild,
        Self::IndexRepair,
        Self::SettingsRepair,
        Self::StateMigration,
        Self::TargetedReset,
        Self::NoSafeRepair,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CacheRebuild => "cache_rebuild",
            Self::IndexRepair => "index_repair",
            Self::SettingsRepair => "settings_repair",
            Self::StateMigration => "state_migration",
            Self::TargetedReset => "targeted_reset",
            Self::NoSafeRepair => "no_safe_repair",
        }
    }
}

/// Controlled case disposition — the shared classification of a support case, so no
/// surface invents an alternate label for the local-only, vendor-case, uncategorized,
/// or unsafe-fix-blocked states. Declared by the escalation-packet summary and the
/// unsafe-fix blocked note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportCaseDisposition {
    /// A local-only case that never leaves the device.
    LocalOnly,
    /// A vendor case handed to a support vendor.
    VendorCase,
    /// An uncategorized case with no committed scenario yet.
    Uncategorized,
    /// A case blocked because the only fix is unsafe.
    UnsafeFixBlocked,
    /// A case resolved locally without escalation.
    ResolvedLocally,
}

impl M5SupportCaseDisposition {
    /// Every case disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::VendorCase,
        Self::Uncategorized,
        Self::UnsafeFixBlocked,
        Self::ResolvedLocally,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::VendorCase => "vendor_case",
            Self::Uncategorized => "uncategorized",
            Self::UnsafeFixBlocked => "unsafe_fix_blocked",
            Self::ResolvedLocally => "resolved_locally",
        }
    }
}

/// Claimed M5 support / escalation surface family that renders / consumes a support-
/// intake or escalation component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportSurfaceFamily {
    /// The Project Doctor surface.
    ProjectDoctor,
    /// The support-center surface.
    SupportCenter,
    /// The recovery-center surface.
    RecoveryCenter,
    /// The Help-center surface.
    HelpCenter,
    /// The admin-console surface.
    AdminConsole,
    /// The CLI support surface.
    CliSupport,
}

impl M5SupportSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProjectDoctor,
        Self::SupportCenter,
        Self::RecoveryCenter,
        Self::HelpCenter,
        Self::AdminConsole,
        Self::CliSupport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDoctor => "project_doctor",
            Self::SupportCenter => "support_center",
            Self::RecoveryCenter => "recovery_center",
            Self::HelpCenter => "help_center",
            Self::AdminConsole => "admin_console",
            Self::CliSupport => "cli_support",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// scenario, scope, evidence, or redaction truth never silently narrows or widens
/// between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5SupportDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Support / escalation subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportConsumerSurface {
    /// The Project Doctor UI.
    DoctorUi,
    /// The support-center UI.
    SupportCenterUi,
    /// The report-builder UI.
    ReportBuilderUi,
    /// The escalation-desk UI.
    EscalationDeskUi,
    /// The recovery-center UI.
    RecoveryCenterUi,
    /// The Help-center UI.
    HelpCenterUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5SupportConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DoctorUi,
        Self::SupportCenterUi,
        Self::ReportBuilderUi,
        Self::EscalationDeskUi,
        Self::RecoveryCenterUi,
        Self::HelpCenterUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorUi => "doctor_ui",
            Self::SupportCenterUi => "support_center_ui",
            Self::ReportBuilderUi => "report_builder_ui",
            Self::EscalationDeskUi => "escalation_desk_ui",
            Self::RecoveryCenterUi => "recovery_center_ui",
            Self::HelpCenterUi => "help_center_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no support truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5SupportAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed support-intake / escalation component must be able to
/// show. The first three are hard requirements on every component; the remaining three
/// close the acceptance-criteria ambiguity about scenario/scope, evidence/redaction,
/// and destination/next-step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportRequiredLabel {
    /// The component's stable identity / what support object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The scenario family and incident scope behind the component.
    ScenarioAndScope,
    /// The evidence classes and redaction state behind the component.
    EvidenceAndRedaction,
    /// The packet destination and next human step behind the component.
    DestinationAndNextStep,
}

impl M5SupportRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ScenarioAndScope,
        Self::EvidenceAndRedaction,
        Self::DestinationAndNextStep,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ScenarioAndScope => "scenario_and_scope",
            Self::EvidenceAndRedaction => "evidence_and_redaction",
            Self::DestinationAndNextStep => "destination_and_next_step",
        }
    }
}

/// Qualification class for an M5 support-intake / escalation component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5SupportQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a support-intake / escalation component below its
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportDowngradeTrigger {
    /// A picker row left its scenario or scope unstated.
    ScenarioOrScopeUnstated,
    /// A picker row left the bound Doctor finding lineage unstated.
    DoctorFindingLineageUnstated,
    /// A builder step masked which evidence class it selected or omitted.
    EvidenceClassMasked,
    /// A component left its redaction state undisclosed.
    RedactionStateUndisclosed,
    /// An escalation-packet summary left its destination unstated.
    PacketDestinationUnstated,
    /// A handoff-timeline row left the next human step unstated.
    NextHumanStepUnstated,
    /// An unsafe-fix note masked the approved repair class.
    ApprovedRepairClassMasked,
    /// An unsafe-fix note hid why the fix is blocked.
    UnsafeFixBlockReasonHidden,
    /// A component left the case disposition unstated.
    CaseDispositionUnstated,
    /// A handoff-timeline row collapsed or skipped a stage.
    HandoffStageCollapsed,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5SupportDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ScenarioOrScopeUnstated,
        Self::DoctorFindingLineageUnstated,
        Self::EvidenceClassMasked,
        Self::RedactionStateUndisclosed,
        Self::PacketDestinationUnstated,
        Self::NextHumanStepUnstated,
        Self::ApprovedRepairClassMasked,
        Self::UnsafeFixBlockReasonHidden,
        Self::CaseDispositionUnstated,
        Self::HandoffStageCollapsed,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScenarioOrScopeUnstated => "scenario_or_scope_unstated",
            Self::DoctorFindingLineageUnstated => "doctor_finding_lineage_unstated",
            Self::EvidenceClassMasked => "evidence_class_masked",
            Self::RedactionStateUndisclosed => "redaction_state_undisclosed",
            Self::PacketDestinationUnstated => "packet_destination_unstated",
            Self::NextHumanStepUnstated => "next_human_step_unstated",
            Self::ApprovedRepairClassMasked => "approved_repair_class_masked",
            Self::UnsafeFixBlockReasonHidden => "unsafe_fix_block_reason_hidden",
            Self::CaseDispositionUnstated => "case_disposition_unstated",
            Self::HandoffStageCollapsed => "handoff_stage_collapsed",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed support-intake / escalation component family
/// bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeEscalationComponentRow {
    /// Governed component family.
    pub component_family: M5SupportIntakeEscalationComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5SupportQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 support / escalation surface families that render / consume this
    /// component.
    pub surface_families: Vec<M5SupportSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5SupportDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5SupportRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5SupportRequiredLabel>,
    /// Scenario families this component names (support-scenario-picker-row only).
    pub scenario_families: Vec<M5SupportScenarioFamily>,
    /// Incident scopes this component distinguishes (support-scenario-picker-row only).
    pub incident_scopes: Vec<M5SupportIncidentScope>,
    /// Doctor finding families this component binds (support-scenario-picker-row only).
    pub doctor_finding_families: Vec<M5DoctorFindingFamily>,
    /// Report-builder step kinds this component names (issue-report-builder-step only).
    pub builder_step_kinds: Vec<M5ReportBuilderStepKind>,
    /// Evidence classes this component selects / omits (issue-report-builder-step
    /// only).
    pub evidence_classes: Vec<M5SupportEvidenceClass>,
    /// Packet destinations this component distinguishes (escalation-packet-summary
    /// only).
    pub packet_destinations: Vec<M5EscalationPacketDestination>,
    /// Redaction states this component discloses (escalation-packet-summary only).
    pub redaction_states: Vec<M5SupportRedactionState>,
    /// Handoff stages this component names (handoff-timeline-row only).
    pub handoff_stages: Vec<M5HandoffStage>,
    /// Next human steps this component names (handoff-timeline-row only).
    pub next_human_steps: Vec<M5NextHumanStep>,
    /// Block reasons this component discloses (unsafe-fix-blocked-note only).
    pub unsafe_fix_block_reasons: Vec<M5UnsafeFixBlockReason>,
    /// Approved repair classes this component names (unsafe-fix-blocked-note only).
    pub approved_repair_classes: Vec<M5ApprovedRepairClass>,
    /// Case dispositions this component distinguishes (escalation-packet-summary and
    /// unsafe-fix-blocked-note).
    pub case_dispositions: Vec<M5SupportCaseDisposition>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5SupportAccessibilityRoute>,
    /// Support / escalation subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5SupportConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5SupportDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its scenario family or incident
    /// scope. MUST be `false`.
    pub masks_scenario_or_scope: bool,
    /// Hard invariant: this component never hides why an unsafe fix is blocked. MUST be
    /// `false`.
    pub hides_unsafe_fix_block_reason: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never bypasses the escalation-packet minimums.
    /// MUST be `false`.
    pub bypasses_escalation_packet_minimums: bool,
}

impl M5SupportIntakeEscalationComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5SupportRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5SupportRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_scenario_or_scope
            && !self.hides_unsafe_fix_block_reason
            && !self.invents_alternate_state_label
            && !self.bypasses_escalation_packet_minimums
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeEscalationComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Scenario-family tokens.
    pub scenario_families: Vec<String>,
    /// Incident-scope tokens.
    pub incident_scopes: Vec<String>,
    /// Doctor-finding-family tokens.
    pub doctor_finding_families: Vec<String>,
    /// Report-builder-step-kind tokens.
    pub builder_step_kinds: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Packet-destination tokens.
    pub packet_destinations: Vec<String>,
    /// Redaction-state tokens.
    pub redaction_states: Vec<String>,
    /// Handoff-stage tokens.
    pub handoff_stages: Vec<String>,
    /// Next-human-step tokens.
    pub next_human_steps: Vec<String>,
    /// Unsafe-fix-block-reason tokens.
    pub unsafe_fix_block_reasons: Vec<String>,
    /// Approved-repair-class tokens.
    pub approved_repair_classes: Vec<String>,
    /// Case-disposition tokens.
    pub case_dispositions: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5SupportIntakeEscalationComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5SupportIntakeEscalationComponentFamily::ALL, |v| {
                v.as_str()
            }),
            scenario_families: tokens(&M5SupportScenarioFamily::ALL, |v| v.as_str()),
            incident_scopes: tokens(&M5SupportIncidentScope::ALL, |v| v.as_str()),
            doctor_finding_families: tokens(&M5DoctorFindingFamily::ALL, |v| v.as_str()),
            builder_step_kinds: tokens(&M5ReportBuilderStepKind::ALL, |v| v.as_str()),
            evidence_classes: tokens(&M5SupportEvidenceClass::ALL, |v| v.as_str()),
            packet_destinations: tokens(&M5EscalationPacketDestination::ALL, |v| v.as_str()),
            redaction_states: tokens(&M5SupportRedactionState::ALL, |v| v.as_str()),
            handoff_stages: tokens(&M5HandoffStage::ALL, |v| v.as_str()),
            next_human_steps: tokens(&M5NextHumanStep::ALL, |v| v.as_str()),
            unsafe_fix_block_reasons: tokens(&M5UnsafeFixBlockReason::ALL, |v| v.as_str()),
            approved_repair_classes: tokens(&M5ApprovedRepairClass::ALL, |v| v.as_str()),
            case_dispositions: tokens(&M5SupportCaseDisposition::ALL, |v| v.as_str()),
            surface_families: tokens(&M5SupportSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5SupportDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5SupportConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5SupportAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5SupportRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5SupportIntakeEscalationComponentGovernanceReview {
    /// The support-scenario picker row shows its scenario family and incident scope.
    pub scenario_picker_row_shows_scenario_and_scope: bool,
    /// The support-scenario picker row binds its scenario to a Doctor finding family.
    pub scenario_picker_row_binds_doctor_finding_family: bool,
    /// The issue-report builder step shows which evidence it selects and omits.
    pub report_builder_step_shows_selected_and_omitted_evidence: bool,
    /// The escalation-packet summary shows its destination and redaction state.
    pub escalation_packet_summary_shows_destination_and_redaction: bool,
    /// The handoff-timeline row shows its stage and next human step.
    pub handoff_timeline_row_shows_stage_and_next_step: bool,
    /// The unsafe-fix blocked note shows its block reason and approved repair class.
    pub unsafe_fix_blocked_note_shows_block_reason_and_approved_repair: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Local-only, vendor-case, uncategorized, and unsafe-fix-blocked are named once.
    pub local_only_vendor_case_uncategorized_and_unsafe_blocked_named_once: bool,
    /// The Doctor finding lineage behind a scenario is always explicit.
    pub doctor_finding_lineage_always_explicit: bool,
    /// The approved repair class is always explicit.
    pub approved_repair_class_always_explicit: bool,
    /// The escalation-packet minimums are always enforced.
    pub escalation_packet_minimums_always_enforced: bool,
    /// The next human step is always explicit.
    pub next_human_step_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel support vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeEscalationComponentConsumerProjection {
    /// Doctor and support surfaces consume the shared scenario vocabulary.
    pub doctor_and_support_surfaces_consume_scenario_vocabulary: bool,
    /// Report-builder surfaces consume the evidence vocabulary.
    pub report_builder_surfaces_consume_evidence_vocabulary: bool,
    /// Escalation surfaces consume the destination and redaction vocabulary.
    pub escalation_surfaces_consume_destination_and_redaction_vocabulary: bool,
    /// Unsafe-fix surfaces consume the block-reason vocabulary.
    pub unsafe_fix_surfaces_consume_block_reason_vocabulary: bool,
    /// Support / export reads a single canonical support source.
    pub support_export_reads_single_source: bool,
    /// Help and admin surfaces read a single canonical support source.
    pub help_and_admin_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeEscalationComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the support-intake / escalation component
/// lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeEscalationComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting support-case audit for the lane.
    pub support_case_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SupportIntakeEscalationComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SupportIntakeEscalationComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5SupportIntakeEscalationComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportIntakeEscalationComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportIntakeEscalationComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportIntakeEscalationComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportIntakeEscalationComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportIntakeEscalationComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 support-intake / escalation component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SupportIntakeEscalationComponentMatrixPacket {
    /// Record kind; must equal
    /// [`M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5SupportIntakeEscalationComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SupportIntakeEscalationComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SupportIntakeEscalationComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SupportIntakeEscalationComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SupportIntakeEscalationComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SupportIntakeEscalationComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SupportIntakeEscalationComponentMatrixPacket {
    /// Builds an M5 support-intake / escalation component matrix packet from
    /// stable-lane input.
    pub fn new(input: M5SupportIntakeEscalationComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 support-intake / escalation component matrix invariants.
    pub fn validate(&self) -> Vec<M5SupportIntakeEscalationComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 support-intake escalation component matrix packet serializes"),
        ) {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 support-intake escalation component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Support-Scenario-Picker-Row, Issue-Report-Builder-Step, Escalation-Packet-Summary, Handoff-Timeline-Row, and Unsafe-Fix-Blocked-Note Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Scenario families: {}\n",
            self.vocabulary_set.scenario_families.join(", ")
        ));
        out.push_str(&format!(
            "- Case dispositions: {}\n",
            self.vocabulary_set.case_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 support-intake matrix export.
#[derive(Debug)]
pub enum M5SupportIntakeEscalationComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SupportIntakeEscalationComponentMatrixViolation>),
}

impl fmt::Display for M5SupportIntakeEscalationComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 support-intake escalation component matrix export parse failed: {error}"
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
                    "m5 support-intake escalation component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SupportIntakeEscalationComponentMatrixArtifactError {}

/// Validation failures emitted by
/// [`M5SupportIntakeEscalationComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SupportIntakeEscalationComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A support-scenario-picker-row component declares no scenario families.
    ScenarioFamilyMissing,
    /// A support-scenario-picker-row component declares no incident scopes.
    IncidentScopeMissing,
    /// A support-scenario-picker-row component binds no Doctor finding families.
    DoctorFindingFamilyMissing,
    /// An issue-report-builder-step component declares no builder step kinds.
    BuilderStepKindMissing,
    /// An issue-report-builder-step component declares no evidence classes.
    EvidenceClassMissing,
    /// An escalation-packet-summary component declares no packet destinations.
    PacketDestinationMissing,
    /// An escalation-packet-summary component declares no redaction states.
    RedactionStateMissing,
    /// A handoff-timeline-row component declares no handoff stages.
    HandoffStageMissing,
    /// A handoff-timeline-row component declares no next human steps.
    NextHumanStepMissing,
    /// An unsafe-fix-blocked-note component declares no block reasons.
    UnsafeFixBlockReasonMissing,
    /// An unsafe-fix-blocked-note component declares no approved repair classes.
    ApprovedRepairClassMissing,
    /// An escalation-packet-summary or unsafe-fix-blocked-note component declares no
    /// case dispositions.
    CaseDispositionMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked scenario/scope, hidden unsafe-fix
    /// block reason, invented alternate state label, or bypassed escalation-packet
    /// minimums).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SupportIntakeEscalationComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ScenarioFamilyMissing => "scenario_family_missing",
            Self::IncidentScopeMissing => "incident_scope_missing",
            Self::DoctorFindingFamilyMissing => "doctor_finding_family_missing",
            Self::BuilderStepKindMissing => "builder_step_kind_missing",
            Self::EvidenceClassMissing => "evidence_class_missing",
            Self::PacketDestinationMissing => "packet_destination_missing",
            Self::RedactionStateMissing => "redaction_state_missing",
            Self::HandoffStageMissing => "handoff_stage_missing",
            Self::NextHumanStepMissing => "next_human_step_missing",
            Self::UnsafeFixBlockReasonMissing => "unsafe_fix_block_reason_missing",
            Self::ApprovedRepairClassMissing => "approved_repair_class_missing",
            Self::CaseDispositionMissing => "case_disposition_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 support-intake matrix export.
pub fn current_stable_m5_support_intake_escalation_component_matrix_export() -> Result<
    M5SupportIntakeEscalationComponentMatrixPacket,
    M5SupportIntakeEscalationComponentMatrixArtifactError,
> {
    let packet: M5SupportIntakeEscalationComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-support-intake-escalation-proof/support_export.json"
        )))
        .map_err(M5SupportIntakeEscalationComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SupportIntakeEscalationComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOC_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCENARIO_PICKER_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_DOCTOR_FINDING_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ESCALATION_PACKET_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_RECOVERY_ACTION_REF,
        M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_REDACTION_PROFILE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SupportIntakeEscalationComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    let present: BTreeSet<M5SupportIntakeEscalationComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5SupportIntakeEscalationComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_support_scenario_picker_row() && row.scenario_families.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::ScenarioFamilyMissing);
        }
        if family.is_support_scenario_picker_row() && row.incident_scopes.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::IncidentScopeMissing);
        }
        if family.is_support_scenario_picker_row() && row.doctor_finding_families.is_empty() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::DoctorFindingFamilyMissing);
        }
        if family.is_issue_report_builder_step() && row.builder_step_kinds.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::BuilderStepKindMissing);
        }
        if family.is_issue_report_builder_step() && row.evidence_classes.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::EvidenceClassMissing);
        }
        if family.is_escalation_packet_summary() && row.packet_destinations.is_empty() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::PacketDestinationMissing);
        }
        if family.is_escalation_packet_summary() && row.redaction_states.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::RedactionStateMissing);
        }
        if family.is_handoff_timeline_row() && row.handoff_stages.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::HandoffStageMissing);
        }
        if family.is_handoff_timeline_row() && row.next_human_steps.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::NextHumanStepMissing);
        }
        if family.is_unsafe_fix_blocked_note() && row.unsafe_fix_block_reasons.is_empty() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::UnsafeFixBlockReasonMissing);
        }
        if family.is_unsafe_fix_blocked_note() && row.approved_repair_classes.is_empty() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::ApprovedRepairClassMissing);
        }
        // Case disposition is shared by the escalation-packet summary and the
        // unsafe-fix blocked note.
        if (family.is_escalation_packet_summary() || family.is_unsafe_fix_blocked_note())
            && row.case_dispositions.is_empty()
        {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::CaseDispositionMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SupportIntakeEscalationComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.scenario_picker_row_shows_scenario_and_scope,
        review.scenario_picker_row_binds_doctor_finding_family,
        review.report_builder_step_shows_selected_and_omitted_evidence,
        review.escalation_packet_summary_shows_destination_and_redaction,
        review.handoff_timeline_row_shows_stage_and_next_step,
        review.unsafe_fix_blocked_note_shows_block_reason_and_approved_repair,
        review.no_surface_invents_alternate_state_label,
        review.local_only_vendor_case_uncategorized_and_unsafe_blocked_named_once,
        review.doctor_finding_lineage_always_explicit,
        review.approved_repair_class_always_explicit,
        review.escalation_packet_minimums_always_enforced,
        review.next_human_step_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5SupportIntakeEscalationComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.doctor_and_support_surfaces_consume_scenario_vocabulary,
        projection.report_builder_surfaces_consume_evidence_vocabulary,
        projection.escalation_surfaces_consume_destination_and_redaction_vocabulary,
        projection.unsafe_fix_surfaces_consume_block_reason_vocabulary,
        projection.support_export_reads_single_source,
        projection.help_and_admin_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(
                M5SupportIntakeEscalationComponentMatrixViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SupportIntakeEscalationComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SupportIntakeEscalationComponentMatrixPacket,
    violations: &mut Vec<M5SupportIntakeEscalationComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.support_case_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SupportIntakeEscalationComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
