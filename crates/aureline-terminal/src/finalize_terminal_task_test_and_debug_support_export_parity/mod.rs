//! Terminal, task, test, and debug support-export parity plus
//! diagnosis packets and repair hooks truth packet for the M4 stable lane.
//!
//! This module pins how terminal, task, test, and debug surfaces expose one
//! support-export truth that downstream consumers (terminal pane, task panel,
//! test explorer, debug surface, CLI/headless inspector, support export,
//! release proof index, Help/About proof card, and the conformance dashboard)
//! read verbatim. Surfaces MUST NOT mint local copies, paraphrase fields, or
//! fork their own export, diagnosis, or repair semantics; they project this
//! packet.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `support_export_parity_quality` row cannot prove:
//!
//! - the four wedges (`support_export_parity`, `diagnosis_packet`,
//!   `repair_hook`, `execution_context_lineage`) each have a structured
//!   wedge_admission row,
//! - each export-parity field is admitted exactly once (`export_class`,
//!   `redaction_state`, `provenance`, `support_summary`, `artifact_provenance`)
//!   via one `export_field_binding` row per field,
//! - each diagnosis-packet field is admitted exactly once (`finding_code`,
//!   `diagnosis_scope`, `redaction_class`, `chain_of_custody`) via one
//!   `diagnosis_packet_binding` row per field,
//! - each repair-hook field is admitted exactly once (`repair_transaction_id`,
//!   `repair_hook_ref`, `repair_authority`, `repair_outcome`) via one
//!   `repair_hook_binding` row per field,
//! - each recovery posture (`reconnect`, `restore_no_rerun`, `blocked_target`,
//!   `degraded_helper`, `artifact_provenance`) carries a
//!   `recovery_posture_admission` row so the lane explains reconnect,
//!   restore-no-rerun, blocked-target, degraded-helper, and artifact-provenance
//!   without support-only knowledge,
//! - one stable `execution_context_id` lineage object threads through every
//!   surface that consumes the execution context.
//!
//! Every row binds a closed `support_export_parity_lane_class`,
//! `support_export_parity_row_class`, `support_class`, `wedge_class`,
//! `export_field_class`, `diagnosis_packet_field_class`, `repair_hook_field_class`,
//! `recovery_posture_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `support_export_parity_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the row is
//! narrowed below launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw command
//! lines, raw process environment bytes, raw scrollback bodies, raw secrets,
//! or ambient credentials past the boundary. A row that claims `launch_stable`
//! while leaving its support, known limit, downgrade automation, or evidence
//! class unbound is refused; the validator narrows below launch-stable instead
//! of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SupportExportParityTruthPacket`].
pub const SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND: &str =
    "finalize_terminal_task_test_and_debug_support_export_parity_truth_stable_packet";

/// Stable record-kind tag for [`SupportExportParityTruthSupportExport`].
pub const SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "finalize_terminal_task_test_and_debug_support_export_parity_truth_support_export";

/// Integer schema version for the support-export-parity truth packet.
pub const SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/finalize_terminal_task_test_and_debug_support_export_parity_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/finalize-terminal-task-test-and-debug-support-export-parity.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const SUPPORT_EXPORT_PARITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/finalize-terminal-task-test-and-debug-support-export-parity.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SUPPORT_EXPORT_PARITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/finalize_terminal_task_test_and_debug_support_export_parity";

/// Repo-relative path of the checked-in stable packet.
pub const SUPPORT_EXPORT_PARITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/finalize_terminal_task_test_and_debug_support_export_parity_truth_packet.json";

/// Closed support-export-parity lane vocabulary. Every required lane
/// MUST have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportParityLaneClass {
    /// Terminal sessions (local, remote, container, restored).
    TerminalLane,
    /// Task runner surfaces (build, run, package scripts).
    TaskLane,
    /// Test runner surfaces (pytest, unit, integration, watch).
    TestLane,
    /// Debug adapter surfaces (attach, launch, evaluate).
    DebugLane,
}

impl SupportExportParityLaneClass {
    /// Every required support-export-parity lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::TerminalLane,
        Self::TaskLane,
        Self::TestLane,
        Self::DebugLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalLane => "terminal_lane",
            Self::TaskLane => "task_lane",
            Self::TestLane => "test_lane",
            Self::DebugLane => "debug_lane",
        }
    }
}

/// Closed support-export-parity row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportParityRowClass {
    /// The lane's headline support-export-parity qualification row.
    SupportExportParityQuality,
    /// A row admitting one of the four wedges (support_export_parity,
    /// diagnosis_packet, repair_hook, execution_context_lineage).
    WedgeAdmission,
    /// A row binding one export-parity field (export_class, redaction_state,
    /// provenance, support_summary, artifact_provenance).
    ExportFieldBinding,
    /// A row binding one diagnosis-packet field (finding_code,
    /// diagnosis_scope, redaction_class, chain_of_custody).
    DiagnosisPacketBinding,
    /// A row binding one repair-hook field (repair_transaction_id,
    /// repair_hook_ref, repair_authority, repair_outcome).
    RepairHookBinding,
    /// A row admitting one recovery posture (reconnect, restore_no_rerun,
    /// blocked_target, degraded_helper, artifact_provenance).
    RecoveryPostureAdmission,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted execution truth and downstream consumer
    /// surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl SupportExportParityRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportExportParityQuality => "support_export_parity_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::ExportFieldBinding => "export_field_binding",
            Self::DiagnosisPacketBinding => "diagnosis_packet_binding",
            Self::RepairHookBinding => "repair_hook_binding",
            Self::RecoveryPostureAdmission => "recovery_posture_admission",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound export field.
    pub const fn requires_export_field(self) -> bool {
        matches!(self, Self::ExportFieldBinding)
    }

    /// True when this row class requires a bound diagnosis-packet field.
    pub const fn requires_diagnosis_packet_field(self) -> bool {
        matches!(self, Self::DiagnosisPacketBinding)
    }

    /// True when this row class requires a bound repair-hook field.
    pub const fn requires_repair_hook_field(self) -> bool {
        matches!(self, Self::RepairHookBinding)
    }

    /// True when this row class requires a bound recovery posture.
    pub const fn requires_recovery_posture(self) -> bool {
        matches!(self, Self::RecoveryPostureAdmission)
    }
}

/// Closed support-class vocabulary applied to a support-export-parity
/// row. A row is never `launch_stable` while its known limit, downgrade
/// automation, or evidence class is unbound; the validator demotes it
/// instead of inheriting an adjacent launch-stable row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade for the lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable.
    LaunchStableBelow,
    /// Row is at beta-grade only.
    BetaGradeOnly,
    /// Row is at preview only.
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl SupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchStable => "launch_stable",
            Self::LaunchStableBelow => "launch_stable_below",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    /// True when this support class satisfies the support-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed support-export-parity wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each
/// required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Support-export parity wedge (all four surfaces have equivalent
    /// export capability and semantics).
    SupportExportParity,
    /// Diagnosis-packet wedge (structured diagnosis information is
    /// available for support and self-service).
    DiagnosisPacket,
    /// Repair-hook wedge (repair transaction hooks are wired with
    /// visible IDs and authority).
    RepairHook,
    /// Execution-context lineage wedge (one execution-context truth
    /// threads across all four surfaces without fork).
    ExecutionContextLineage,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::SupportExportParity,
        Self::DiagnosisPacket,
        Self::RepairHook,
        Self::ExecutionContextLineage,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportExportParity => "support_export_parity",
            Self::DiagnosisPacket => "diagnosis_packet",
            Self::RepairHook => "repair_hook",
            Self::ExecutionContextLineage => "execution_context_lineage",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed export-field vocabulary. Every lane claiming `launch_stable`
/// MUST publish an `export_field_binding` row for each required field
/// so support-export parity never blurs export semantics across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFieldClass {
    /// Export class token (metadata_only, support_bundle_scoped,
    /// ai_promoted_slice).
    ExportClass,
    /// Redaction posture applied to exported bodies.
    RedactionState,
    /// Artifact provenance and chain-of-custody header.
    Provenance,
    /// Human-readable support-safe summary.
    SupportSummary,
    /// Produced-output provenance that survives reattach and export.
    ArtifactProvenance,
    /// The row is not bound to an export field.
    NotApplicable,
}

impl ExportFieldClass {
    /// Every required export field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::ExportClass,
        Self::RedactionState,
        Self::Provenance,
        Self::SupportSummary,
        Self::ArtifactProvenance,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportClass => "export_class",
            Self::RedactionState => "redaction_state",
            Self::Provenance => "provenance",
            Self::SupportSummary => "support_summary",
            Self::ArtifactProvenance => "artifact_provenance",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed diagnosis-packet-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `diagnosis_packet_binding` row for
/// each required field so diagnosis packets preserve stable finding
/// codes, redaction classes, and export-safe chain-of-custody.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosisPacketFieldClass {
    /// Stable finding code for the diagnosis.
    FindingCode,
    /// Scope of the diagnosis (target, toolchain, capsule, policy).
    DiagnosisScope,
    /// Redaction class applied to the diagnosis packet.
    RedactionClass,
    /// Export-safe chain-of-custody field for support and shiproom.
    ChainOfCustody,
    /// The row is not bound to a diagnosis-packet field.
    NotApplicable,
}

impl DiagnosisPacketFieldClass {
    /// Every required diagnosis-packet field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::FindingCode,
        Self::DiagnosisScope,
        Self::RedactionClass,
        Self::ChainOfCustody,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingCode => "finding_code",
            Self::DiagnosisScope => "diagnosis_scope",
            Self::RedactionClass => "redaction_class",
            Self::ChainOfCustody => "chain_of_custody",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed repair-hook-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `repair_hook_binding` row for each
/// required field so repair transactions carry visible IDs, authority,
/// and outcome tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairHookFieldClass {
    /// Stable repair transaction ID.
    RepairTransactionId,
    /// Reference to the callable repair hook.
    RepairHookRef,
    /// Authority required to execute the repair (user, admin, automated).
    RepairAuthority,
    /// Expected or actual repair outcome token.
    RepairOutcome,
    /// The row is not bound to a repair-hook field.
    NotApplicable,
}

impl RepairHookFieldClass {
    /// Every required repair-hook field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::RepairTransactionId,
        Self::RepairHookRef,
        Self::RepairAuthority,
        Self::RepairOutcome,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairTransactionId => "repair_transaction_id",
            Self::RepairHookRef => "repair_hook_ref",
            Self::RepairAuthority => "repair_authority",
            Self::RepairOutcome => "repair_outcome",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed recovery-posture vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `recovery_posture_admission` row for
/// each structured recovery posture so the lane explains reconnect,
/// restore-no-rerun, blocked-target, degraded-helper, and
/// artifact-provenance without support-only knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPostureClass {
    /// Reattach / reconnect posture (helper online, target reachable).
    Reconnect,
    /// Restore brought metadata back without silently rerunning.
    RestoreNoRerun,
    /// Requested target is blocked by trust, policy, or capability.
    BlockedTarget,
    /// Helper / remote agent reports degraded capabilities.
    DegradedHelper,
    /// Artifact provenance survives reattach and support export.
    ArtifactProvenance,
    /// The row is not bound to a recovery posture.
    NotApplicable,
}

impl RecoveryPostureClass {
    /// Every required recovery posture per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::Reconnect,
        Self::RestoreNoRerun,
        Self::BlockedTarget,
        Self::DegradedHelper,
        Self::ArtifactProvenance,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconnect => "reconnect",
            Self::RestoreNoRerun => "restore_no_rerun",
            Self::BlockedTarget => "blocked_target",
            Self::DegradedHelper => "degraded_helper",
            Self::ArtifactProvenance => "artifact_provenance",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// The row is backed by an automated functional / unit suite.
    AutomatedFunctionalEvidence,
    /// The row is backed by a conformance / interoperability suite.
    ConformanceSuiteEvidence,
    /// The row is backed by a failure / recovery drill.
    FailureRecoveryDrillEvidence,
    /// The row is backed by design-QA / UX validation.
    DesignQaEvidence,
    /// The row is backed by release-evidence review.
    ReleaseEvidenceReview,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a benchmark / fitness-function capture.
    BenchmarkEvidence,
    /// The row is backed by a docs / help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::BenchmarkEvidence => "benchmark_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a support-export-parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the terminal subset.
    TerminalLaneSubsetOnly,
    /// The lane only certifies the task subset.
    TaskLaneSubsetOnly,
    /// The lane only certifies the test subset.
    TestLaneSubsetOnly,
    /// The lane only certifies the debug subset.
    DebugLaneSubsetOnly,
    /// The lane only certifies a subset of the five required export fields.
    ExportFieldSubsetOnly,
    /// The lane only certifies a subset of the four required diagnosis-packet fields.
    DiagnosisPacketSubsetOnly,
    /// The lane only certifies a subset of the four required repair-hook fields.
    RepairHookSubsetOnly,
    /// The lane only certifies a subset of the five required recovery postures.
    RecoveryPostureSubsetOnly,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known-limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::TerminalLaneSubsetOnly => "terminal_lane_subset_only",
            Self::TaskLaneSubsetOnly => "task_lane_subset_only",
            Self::TestLaneSubsetOnly => "test_lane_subset_only",
            Self::DebugLaneSubsetOnly => "debug_lane_subset_only",
            Self::ExportFieldSubsetOnly => "export_field_subset_only",
            Self::DiagnosisPacketSubsetOnly => "diagnosis_packet_subset_only",
            Self::RepairHookSubsetOnly => "repair_hook_subset_only",
            Self::RecoveryPostureSubsetOnly => "recovery_posture_subset_only",
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a required export-field binding is missing.
    AutoNarrowOnExportFieldGap,
    /// Automatically narrow when a required diagnosis-packet field is unbound.
    AutoNarrowOnDiagnosisPacketGap,
    /// Automatically narrow when a required repair-hook field is unbound.
    AutoNarrowOnRepairHookGap,
    /// Automatically narrow when a required recovery posture is unbound.
    AutoNarrowOnRecoveryPostureGap,
    /// Automatically narrow when a required wedge admission is missing.
    AutoNarrowOnWedgeAdmissionGap,
    /// Automatically narrow when the lineage object breaks.
    AutoNarrowOnLineageBreak,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnExportFieldGap => "auto_narrow_on_export_field_gap",
            Self::AutoNarrowOnDiagnosisPacketGap => "auto_narrow_on_diagnosis_packet_gap",
            Self::AutoNarrowOnRepairHookGap => "auto_narrow_on_repair_hook_gap",
            Self::AutoNarrowOnRecoveryPostureGap => "auto_narrow_on_recovery_posture_gap",
            Self::AutoNarrowOnWedgeAdmissionGap => "auto_narrow_on_wedge_admission_gap",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for a support-export-parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportParityConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl SupportExportParityConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet has a blocker finding.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required lane has no row.
    MissingLaneCoverage,
    /// A lane claiming launch_stable is missing a required wedge admission.
    MissingWedgeAdmissionCoverage,
    /// A lane claiming launch_stable is missing a required export field.
    MissingExportFieldCoverage,
    /// A lane claiming launch_stable is missing a required diagnosis-packet field.
    MissingDiagnosisPacketCoverage,
    /// A lane claiming launch_stable is missing a required repair-hook field.
    MissingRepairHookCoverage,
    /// A lane claiming launch_stable is missing a required recovery posture.
    MissingRecoveryPostureCoverage,
    /// A lane claiming launch_stable is missing the required lineage admission row.
    MissingLineageAdmission,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims launch_stable while one or more bindings is unbound.
    LaunchStableWithUnboundBinding,
    /// A row narrowed below launch_stable drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A wedge-admission row drops its wedge binding.
    WedgeNotApplicable,
    /// A non-wedge row binds a wedge it cannot certify.
    WedgeNotPermittedOnRowClass,
    /// An export-field row drops its field binding.
    ExportFieldNotApplicable,
    /// A non-export-field row binds a field it cannot certify.
    ExportFieldNotPermittedOnRowClass,
    /// A diagnosis-packet row drops its field binding.
    DiagnosisPacketNotApplicable,
    /// A non-diagnosis-packet row binds a field it cannot certify.
    DiagnosisPacketNotPermittedOnRowClass,
    /// A repair-hook row drops its field binding.
    RepairHookNotApplicable,
    /// A non-repair-hook row binds a field it cannot certify.
    RepairHookNotPermittedOnRowClass,
    /// A recovery-posture row drops its posture binding.
    RecoveryPostureNotApplicable,
    /// A non-recovery-posture row binds a posture it cannot certify.
    RecoveryPostureNotPermittedOnRowClass,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A row admits raw command lines, env bytes, or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the wedge vocabulary.
    WedgeVocabularyCollapsed,
    /// A projection collapses the export-field vocabulary.
    ExportFieldVocabularyCollapsed,
    /// A projection collapses the diagnosis-packet vocabulary.
    DiagnosisPacketVocabularyCollapsed,
    /// A projection collapses the repair-hook vocabulary.
    RepairHookVocabularyCollapsed,
    /// A projection collapses the recovery-posture vocabulary.
    RecoveryPostureVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingWedgeAdmissionCoverage => "missing_wedge_admission_coverage",
            Self::MissingExportFieldCoverage => "missing_export_field_coverage",
            Self::MissingDiagnosisPacketCoverage => "missing_diagnosis_packet_coverage",
            Self::MissingRepairHookCoverage => "missing_repair_hook_coverage",
            Self::MissingRecoveryPostureCoverage => "missing_recovery_posture_coverage",
            Self::MissingLineageAdmission => "missing_lineage_admission",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::WedgeNotApplicable => "wedge_not_applicable",
            Self::WedgeNotPermittedOnRowClass => "wedge_not_permitted_on_row_class",
            Self::ExportFieldNotApplicable => "export_field_not_applicable",
            Self::ExportFieldNotPermittedOnRowClass => "export_field_not_permitted_on_row_class",
            Self::DiagnosisPacketNotApplicable => "diagnosis_packet_not_applicable",
            Self::DiagnosisPacketNotPermittedOnRowClass => {
                "diagnosis_packet_not_permitted_on_row_class"
            }
            Self::RepairHookNotApplicable => "repair_hook_not_applicable",
            Self::RepairHookNotPermittedOnRowClass => "repair_hook_not_permitted_on_row_class",
            Self::RecoveryPostureNotApplicable => "recovery_posture_not_applicable",
            Self::RecoveryPostureNotPermittedOnRowClass => {
                "recovery_posture_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::ExportFieldVocabularyCollapsed => "export_field_vocabulary_collapsed",
            Self::DiagnosisPacketVocabularyCollapsed => "diagnosis_packet_vocabulary_collapsed",
            Self::RepairHookVocabularyCollapsed => "repair_hook_vocabulary_collapsed",
            Self::RecoveryPostureVocabularyCollapsed => "recovery_posture_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Terminal pane chrome (per-pane host-boundary chip and export menu).
    TerminalPane,
    /// Task panel chrome and per-run header.
    TaskPanel,
    /// Test explorer surface and inline results.
    TestExplorer,
    /// Debug surface (call stack, variables, watch, evaluate).
    DebugSurface,
    /// CLI / headless inspection surface.
    CliHeadless,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 9] = [
        Self::TerminalPane,
        Self::TaskPanel,
        Self::TestExplorer,
        Self::DebugSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalPane => "terminal_pane",
            Self::TaskPanel => "task_panel",
            Self::TestExplorer => "test_explorer",
            Self::DebugSurface => "debug_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One support-export-parity truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Support-export-parity lane this row certifies.
    pub lane_class: SupportExportParityLaneClass,
    /// Row class.
    pub row_class: SupportExportParityRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Export field bound by the row (or `not_applicable`).
    pub export_field_class: ExportFieldClass,
    /// Diagnosis-packet field bound by the row (or `not_applicable`).
    pub diagnosis_packet_field_class: DiagnosisPacketFieldClass,
    /// Repair-hook field bound by the row (or `not_applicable`).
    pub repair_hook_field_class: RepairHookFieldClass,
    /// Recovery posture bound by the row (or `not_applicable`).
    pub recovery_posture_class: RecoveryPostureClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: SupportExportParityConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token. Required when `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// True when raw command lines, raw process environment bytes,
    /// or raw scrollback bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl SupportExportParityRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Support-export-parity packet id consumed by the projection.
    pub support_export_parity_truth_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the wedge vocabulary is preserved verbatim.
    pub preserves_wedge_vocabulary: bool,
    /// True when the export-field vocabulary is preserved verbatim.
    pub preserves_export_field_vocabulary: bool,
    /// True when the diagnosis-packet vocabulary is preserved verbatim.
    pub preserves_diagnosis_packet_vocabulary: bool,
    /// True when the repair-hook vocabulary is preserved verbatim.
    pub preserves_repair_hook_vocabulary: bool,
    /// True when the recovery-posture vocabulary is preserved verbatim.
    pub preserves_recovery_posture_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl SupportExportParityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.support_export_parity_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_export_field_vocabulary
            && self.preserves_diagnosis_packet_vocabulary
            && self.preserves_repair_hook_vocabulary
            && self.preserves_recovery_posture_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`SupportExportParityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Support-export-parity lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<SupportExportParityLaneClass>,
    /// Support-export-parity rows.
    #[serde(default)]
    pub rows: Vec<SupportExportParityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SupportExportParityConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Support-export-parity truth packet certifying terminal, task, test,
/// and debug surfaces at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Support-export-parity lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<SupportExportParityLaneClass>,
    /// Support-export-parity rows.
    #[serde(default)]
    pub rows: Vec<SupportExportParityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<SupportExportParityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl SupportExportParityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: SupportExportParityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable support-export-parity invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter()
            .map(SupportExportParityLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(SupportExportParityRowClass::as_str)
            .collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique export-field tokens observed across rows.
    pub fn export_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.export_field_class);
        }
        set.into_iter().map(ExportFieldClass::as_str).collect()
    }

    /// Returns the unique diagnosis-packet tokens observed across rows.
    pub fn diagnosis_packet_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.diagnosis_packet_field_class);
        }
        set.into_iter()
            .map(DiagnosisPacketFieldClass::as_str)
            .collect()
    }

    /// Returns the unique repair-hook tokens observed across rows.
    pub fn repair_hook_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.repair_hook_field_class);
        }
        set.into_iter().map(RepairHookFieldClass::as_str).collect()
    }

    /// Returns the unique recovery-posture tokens observed across rows.
    pub fn recovery_posture_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.recovery_posture_class);
        }
        set.into_iter().map(RecoveryPostureClass::as_str).collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SupportExportParityTruthSupportExport {
        SupportExportParityTruthSupportExport {
            record_kind: SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            support_export_parity_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            support_export_parity_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != SUPPORT_EXPORT_PARITY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "support-export-parity truth packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "support-export-parity truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered support-export-parity lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers support-export-parity lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw command lines, raw env bytes, or raw scrollback bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSupportClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeAutomation,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(row.support_class, SupportClass::LaunchStable)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} discloses known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row
                .downgrade_automation_class
                .requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if row.row_class.requires_wedge()
                && matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a wedge_admission but has no bound wedge",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_wedge()
                && !matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds wedge {}; only wedge_admission rows may bind a wedge",
                        row.row_id,
                        row.row_class.as_str(),
                        row.wedge_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_export_field()
                && matches!(row.export_field_class, ExportFieldClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ExportFieldNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an export_field_binding but has no bound export field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_export_field()
                && !matches!(row.export_field_class, ExportFieldClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ExportFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds export field {}; only export_field_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.export_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_diagnosis_packet_field()
                && matches!(
                    row.diagnosis_packet_field_class,
                    DiagnosisPacketFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiagnosisPacketNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a diagnosis_packet_binding but has no bound diagnosis-packet field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_diagnosis_packet_field()
                && !matches!(
                    row.diagnosis_packet_field_class,
                    DiagnosisPacketFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiagnosisPacketNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds diagnosis-packet field {}; only diagnosis_packet_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.diagnosis_packet_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_repair_hook_field()
                && matches!(
                    row.repair_hook_field_class,
                    RepairHookFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RepairHookNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a repair_hook_binding but has no bound repair-hook field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_repair_hook_field()
                && !matches!(
                    row.repair_hook_field_class,
                    RepairHookFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RepairHookNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds repair-hook field {}; only repair_hook_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.repair_hook_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_recovery_posture()
                && matches!(
                    row.recovery_posture_class,
                    RecoveryPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RecoveryPostureNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a recovery_posture_admission but has no bound recovery posture",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_recovery_posture()
                && !matches!(
                    row.recovery_posture_class,
                    RecoveryPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RecoveryPostureNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds recovery posture {}; only recovery_posture_admission rows may bind a posture",
                        row.row_id,
                        row.row_class.as_str(),
                        row.recovery_posture_class.as_str()
                    ),
                ));
            }

            if matches!(row.row_class, SupportExportParityRowClass::LineageAdmission)
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LineageAdmissionMissingExecutionContextId,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a lineage_admission but has no bound execution_context_id",
                        row.row_id
                    ),
                ));
            }
        }

        for lane in &self.covered_lanes {
            if matches!(
                lane,
                SupportExportParityLaneClass::TerminalLane
                    | SupportExportParityLaneClass::TaskLane
                    | SupportExportParityLaneClass::TestLane
                    | SupportExportParityLaneClass::DebugLane
            ) {
                for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                    let present = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(row.row_class, SupportExportParityRowClass::WedgeAdmission)
                            && row.wedge_class == wedge
                            && matches!(row.support_class, SupportClass::LaunchStable)
                    });
                    if !present {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingWedgeAdmissionCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} is missing launch_stable wedge_admission for {}",
                                lane.as_str(),
                                wedge.as_str()
                            ),
                        ));
                    }
                }

                for field in ExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                    let present = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(
                                row.row_class,
                                SupportExportParityRowClass::ExportFieldBinding
                            )
                            && row.export_field_class == field
                            && matches!(row.support_class, SupportClass::LaunchStable)
                    });
                    if !present {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingExportFieldCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} is missing launch_stable export_field_binding for {}",
                                lane.as_str(),
                                field.as_str()
                            ),
                        ));
                    }
                }

                for field in DiagnosisPacketFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                    let present = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(
                                row.row_class,
                                SupportExportParityRowClass::DiagnosisPacketBinding
                            )
                            && row.diagnosis_packet_field_class == field
                            && matches!(row.support_class, SupportClass::LaunchStable)
                    });
                    if !present {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingDiagnosisPacketCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} is missing launch_stable diagnosis_packet_binding for {}",
                                lane.as_str(),
                                field.as_str()
                            ),
                        ));
                    }
                }

                for field in RepairHookFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                    let present = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(
                                row.row_class,
                                SupportExportParityRowClass::RepairHookBinding
                            )
                            && row.repair_hook_field_class == field
                            && matches!(row.support_class, SupportClass::LaunchStable)
                    });
                    if !present {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingRepairHookCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} is missing launch_stable repair_hook_binding for {}",
                                lane.as_str(),
                                field.as_str()
                            ),
                        ));
                    }
                }

                for posture in RecoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
                    let present = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(
                                row.row_class,
                                SupportExportParityRowClass::RecoveryPostureAdmission
                            )
                            && row.recovery_posture_class == posture
                            && matches!(row.support_class, SupportClass::LaunchStable)
                    });
                    if !present {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingRecoveryPostureCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} is missing launch_stable recovery_posture_admission for {}",
                                lane.as_str(),
                                posture.as_str()
                            ),
                        ));
                    }
                }

                let lineage_present = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, SupportExportParityRowClass::LineageAdmission)
                        && matches!(row.support_class, SupportClass::LaunchStable)
                });
                if !lineage_present {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingLineageAdmission,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} is missing launch_stable lineage_admission row",
                            lane.as_str()
                        ),
                    ));
                }
            }
        }

        for surface in ConsumerSurface::REQUIRED {
            let present = self.consumer_projections.iter().any(|projection| {
                projection.consumer_surface == surface
                    && projection.preserves_truth_for(&self.packet_id)
            });
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!("missing consumer projection for {}", surface.as_str()),
                ));
            }
        }

        for projection in &self.consumer_projections {
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_wedge_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the wedge vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_export_field_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ExportFieldVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the export-field vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_diagnosis_packet_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DiagnosisPacketVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the diagnosis-packet vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_repair_hook_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RepairHookVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the repair-hook vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_recovery_posture_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RecoveryPostureVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the recovery-posture vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EvidenceClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

/// Support-export-parity truth support export wrapping the exact packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Packet id reference.
    pub support_export_parity_truth_packet_id_ref: String,
    /// Exported-at timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
    /// The wrapped packet.
    pub support_export_parity_truth_packet: SupportExportParityTruthPacket,
}

impl SupportExportParityTruthSupportExport {
    /// Returns true when the export is safe for support consumption.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SUPPORT_EXPORT_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SUPPORT_EXPORT_PARITY_TRUTH_SCHEMA_VERSION
            && self.support_export_parity_truth_packet_id_ref
                == self.support_export_parity_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .support_export_parity_truth_packet
                .validate()
                .is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum SupportExportParityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for SupportExportParityTruthArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    f,
                    "support-export-parity truth packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    f,
                    "support-export-parity truth packet validation failed: {tokens}"
                )
            }
        }
    }
}

impl Error for SupportExportParityTruthArtifactError {}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|f| f.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|f| f.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Returns the checked-in stable support-export-parity truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse
/// or validate.
pub fn current_stable_support_export_parity_truth_packet(
) -> Result<SupportExportParityTruthPacket, SupportExportParityTruthArtifactError> {
    let packet: SupportExportParityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/finalize_terminal_task_test_and_debug_support_export_parity_truth_packet.json"
    )))
    .map_err(SupportExportParityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SupportExportParityTruthArtifactError::Validation(findings))
    }
}

fn _sample_input() -> SupportExportParityTruthPacketInput {
    let mut rows = Vec::new();
    rows.extend(_lane_rows(
        SupportExportParityLaneClass::TerminalLane,
        "terminal",
    ));
    rows.extend(_lane_rows(SupportExportParityLaneClass::TaskLane, "task"));
    rows.extend(_lane_rows(SupportExportParityLaneClass::TestLane, "test"));
    rows.extend(_lane_rows(SupportExportParityLaneClass::DebugLane, "debug"));
    SupportExportParityTruthPacketInput {
        packet_id: "packet:m4:finalize_terminal_task_test_and_debug_support_export_parity"
            .to_owned(),
        workflow_or_surface_id:
            "workflow.runtime.finalize_terminal_task_test_and_debug_support_export_parity"
                .to_owned(),
        generated_at: "2026-05-27T12:00:00Z".to_owned(),
        covered_lanes: SupportExportParityLaneClass::REQUIRED.to_vec(),
        rows,
        consumer_projections: ConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(_projection)
            .collect(),
        source_contract_refs: vec![SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF.to_owned()],
    }
}

fn _quality_row(prefix: &str, lane: SupportExportParityLaneClass) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_quality", prefix, lane.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::SupportExportParityQuality,
        support_class: SupportClass::LaunchStable,
        wedge_class: WedgeClass::NotApplicable,
        export_field_class: ExportFieldClass::NotApplicable,
        diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
        repair_hook_field_class: RepairHookFieldClass::NotApplicable,
        recovery_posture_class: RecoveryPostureClass::NotApplicable,
        evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _wedge_row(
    prefix: &str,
    lane: SupportExportParityLaneClass,
    wedge: WedgeClass,
) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_wedge_{}", prefix, lane.as_str(), wedge.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::WedgeAdmission,
        support_class: SupportClass::LaunchStable,
        wedge_class: wedge,
        export_field_class: ExportFieldClass::NotApplicable,
        diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
        repair_hook_field_class: RepairHookFieldClass::NotApplicable,
        recovery_posture_class: RecoveryPostureClass::NotApplicable,
        evidence_class: EvidenceClass::ConformanceSuiteEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}_wedge", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _export_field_row(
    prefix: &str,
    lane: SupportExportParityLaneClass,
    field: ExportFieldClass,
) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_export_{}", prefix, lane.as_str(), field.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::ExportFieldBinding,
        support_class: SupportClass::LaunchStable,
        wedge_class: WedgeClass::NotApplicable,
        export_field_class: field,
        diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
        repair_hook_field_class: RepairHookFieldClass::NotApplicable,
        recovery_posture_class: RecoveryPostureClass::NotApplicable,
        evidence_class: EvidenceClass::ConformanceSuiteEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}_export", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _diagnosis_row(
    prefix: &str,
    lane: SupportExportParityLaneClass,
    field: DiagnosisPacketFieldClass,
) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_diagnosis_{}", prefix, lane.as_str(), field.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::DiagnosisPacketBinding,
        support_class: SupportClass::LaunchStable,
        wedge_class: WedgeClass::NotApplicable,
        export_field_class: ExportFieldClass::NotApplicable,
        diagnosis_packet_field_class: field,
        repair_hook_field_class: RepairHookFieldClass::NotApplicable,
        recovery_posture_class: RecoveryPostureClass::NotApplicable,
        evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}_diagnosis", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _repair_row(
    prefix: &str,
    lane: SupportExportParityLaneClass,
    field: RepairHookFieldClass,
) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_repair_{}", prefix, lane.as_str(), field.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::RepairHookBinding,
        support_class: SupportClass::LaunchStable,
        wedge_class: WedgeClass::NotApplicable,
        export_field_class: ExportFieldClass::NotApplicable,
        diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
        repair_hook_field_class: field,
        recovery_posture_class: RecoveryPostureClass::NotApplicable,
        evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}_repair", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _recovery_row(
    prefix: &str,
    lane: SupportExportParityLaneClass,
    posture: RecoveryPostureClass,
) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_recovery_{}", prefix, lane.as_str(), posture.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::RecoveryPostureAdmission,
        support_class: SupportClass::LaunchStable,
        wedge_class: WedgeClass::NotApplicable,
        export_field_class: ExportFieldClass::NotApplicable,
        diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
        repair_hook_field_class: RepairHookFieldClass::NotApplicable,
        recovery_posture_class: posture,
        evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}_recovery", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: None,
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _lineage_row(prefix: &str, lane: SupportExportParityLaneClass) -> SupportExportParityRow {
    SupportExportParityRow {
        row_id: format!("{}_{}_lineage", prefix, lane.as_str()),
        lane_class: lane,
        row_class: SupportExportParityRowClass::LineageAdmission,
        support_class: SupportClass::LaunchStable,
        wedge_class: WedgeClass::NotApplicable,
        export_field_class: ExportFieldClass::NotApplicable,
        diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
        repair_hook_field_class: RepairHookFieldClass::NotApplicable,
        recovery_posture_class: RecoveryPostureClass::NotApplicable,
        evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_automation_class: DowngradeAutomationClass::None,
        confidence_class: SupportExportParityConfidenceClass::HighConfidence,
        evidence_refs: vec![format!("evidence:{}_lineage", prefix)],
        disclosure_ref: None,
        execution_context_id_binding: Some(format!("ctx:{}_lineage", prefix)),
        raw_source_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
        captured_at: "2026-05-27T12:00:00Z".to_owned(),
    }
}

fn _projection(surface: ConsumerSurface) -> SupportExportParityConsumerProjection {
    SupportExportParityConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("proj:{}", surface.as_str()),
        support_export_parity_truth_packet_id_ref:
            "packet:m4:finalize_terminal_task_test_and_debug_support_export_parity".to_owned(),
        rendered_at: "2026-05-27T12:00:00Z".to_owned(),
        preserves_same_packet: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_support_class_vocabulary: true,
        preserves_wedge_vocabulary: true,
        preserves_export_field_vocabulary: true,
        preserves_diagnosis_packet_vocabulary: true,
        preserves_repair_hook_vocabulary: true,
        preserves_recovery_posture_vocabulary: true,
        preserves_known_limit_vocabulary: true,
        preserves_downgrade_automation_vocabulary: true,
        preserves_evidence_class_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn _lane_rows(lane: SupportExportParityLaneClass, prefix: &str) -> Vec<SupportExportParityRow> {
    let mut rows = vec![_quality_row(prefix, lane)];
    for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
        rows.push(_wedge_row(prefix, lane, wedge));
    }
    for field in ExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
        rows.push(_export_field_row(prefix, lane, field));
    }
    for field in DiagnosisPacketFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
        rows.push(_diagnosis_row(prefix, lane, field));
    }
    for field in RepairHookFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
        rows.push(_repair_row(prefix, lane, field));
    }
    for posture in RecoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
        rows.push(_recovery_row(prefix, lane, posture));
    }
    rows.push(_lineage_row(prefix, lane));
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quality_row(prefix: &str, lane: SupportExportParityLaneClass) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_quality", prefix, lane.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::SupportExportParityQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            export_field_class: ExportFieldClass::NotApplicable,
            diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
            repair_hook_field_class: RepairHookFieldClass::NotApplicable,
            recovery_posture_class: RecoveryPostureClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: None,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: SupportExportParityLaneClass,
        wedge: WedgeClass,
    ) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_wedge_{}", prefix, lane.as_str(), wedge.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            export_field_class: ExportFieldClass::NotApplicable,
            diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
            repair_hook_field_class: RepairHookFieldClass::NotApplicable,
            recovery_posture_class: RecoveryPostureClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}_wedge", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: None,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn export_field_row(
        prefix: &str,
        lane: SupportExportParityLaneClass,
        field: ExportFieldClass,
    ) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_export_{}", prefix, lane.as_str(), field.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::ExportFieldBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            export_field_class: field,
            diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
            repair_hook_field_class: RepairHookFieldClass::NotApplicable,
            recovery_posture_class: RecoveryPostureClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}_export", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: None,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn diagnosis_row(
        prefix: &str,
        lane: SupportExportParityLaneClass,
        field: DiagnosisPacketFieldClass,
    ) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_diagnosis_{}", prefix, lane.as_str(), field.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::DiagnosisPacketBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            export_field_class: ExportFieldClass::NotApplicable,
            diagnosis_packet_field_class: field,
            repair_hook_field_class: RepairHookFieldClass::NotApplicable,
            recovery_posture_class: RecoveryPostureClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}_diagnosis", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: None,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn repair_row(
        prefix: &str,
        lane: SupportExportParityLaneClass,
        field: RepairHookFieldClass,
    ) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_repair_{}", prefix, lane.as_str(), field.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::RepairHookBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            export_field_class: ExportFieldClass::NotApplicable,
            diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
            repair_hook_field_class: field,
            recovery_posture_class: RecoveryPostureClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}_repair", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: None,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn recovery_row(
        prefix: &str,
        lane: SupportExportParityLaneClass,
        posture: RecoveryPostureClass,
    ) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_recovery_{}", prefix, lane.as_str(), posture.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::RecoveryPostureAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            export_field_class: ExportFieldClass::NotApplicable,
            diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
            repair_hook_field_class: RepairHookFieldClass::NotApplicable,
            recovery_posture_class: posture,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}_recovery", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: None,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: SupportExportParityLaneClass) -> SupportExportParityRow {
        SupportExportParityRow {
            row_id: format!("{}_{}_lineage", prefix, lane.as_str()),
            lane_class: lane,
            row_class: SupportExportParityRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            export_field_class: ExportFieldClass::NotApplicable,
            diagnosis_packet_field_class: DiagnosisPacketFieldClass::NotApplicable,
            repair_hook_field_class: RepairHookFieldClass::NotApplicable,
            recovery_posture_class: RecoveryPostureClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::None,
            confidence_class: SupportExportParityConfidenceClass::HighConfidence,
            evidence_refs: vec![format!("evidence:{}_lineage", prefix)],
            disclosure_ref: None,
            execution_context_id_binding: Some(format!("ctx:{}_lineage", prefix)),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> SupportExportParityConsumerProjection {
        SupportExportParityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("proj:{}", surface.as_str()),
            support_export_parity_truth_packet_id_ref:
                "packet:m4:finalize_terminal_task_test_and_debug_support_export_parity".to_owned(),
            rendered_at: "2026-05-27T12:00:00Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_export_field_vocabulary: true,
            preserves_diagnosis_packet_vocabulary: true,
            preserves_repair_hook_vocabulary: true,
            preserves_recovery_posture_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn doc_ref() -> String {
        SUPPORT_EXPORT_PARITY_TRUTH_DOC_REF.to_owned()
    }

    fn lane_rows(lane: SupportExportParityLaneClass, prefix: &str) -> Vec<SupportExportParityRow> {
        let mut rows = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(wedge_row(prefix, lane, wedge));
        }
        for field in ExportFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(export_field_row(prefix, lane, field));
        }
        for field in DiagnosisPacketFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(diagnosis_row(prefix, lane, field));
        }
        for field in RepairHookFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(repair_row(prefix, lane, field));
        }
        for posture in RecoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
            rows.push(recovery_row(prefix, lane, posture));
        }
        rows.push(lineage_row(prefix, lane));
        rows
    }

    fn sample_input() -> SupportExportParityTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            SupportExportParityLaneClass::TerminalLane,
            "terminal",
        ));
        rows.extend(lane_rows(SupportExportParityLaneClass::TaskLane, "task"));
        rows.extend(lane_rows(SupportExportParityLaneClass::TestLane, "test"));
        rows.extend(lane_rows(SupportExportParityLaneClass::DebugLane, "debug"));
        SupportExportParityTruthPacketInput {
            packet_id: "packet:m4:finalize_terminal_task_test_and_debug_support_export_parity"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.finalize_terminal_task_test_and_debug_support_export_parity"
                    .to_owned(),
            generated_at: "2026-05-27T12:00:00Z".to_owned(),
            covered_lanes: SupportExportParityLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            SupportExportParityLaneClass::TerminalLane.as_str(),
            "terminal_lane"
        );
        assert_eq!(
            SupportExportParityLaneClass::DebugLane.as_str(),
            "debug_lane"
        );
        assert_eq!(
            WedgeClass::SupportExportParity.as_str(),
            "support_export_parity"
        );
        assert_eq!(
            WedgeClass::ExecutionContextLineage.as_str(),
            "execution_context_lineage"
        );
        assert_eq!(ExportFieldClass::ExportClass.as_str(), "export_class");
        assert_eq!(
            ExportFieldClass::ArtifactProvenance.as_str(),
            "artifact_provenance"
        );
        assert_eq!(
            DiagnosisPacketFieldClass::FindingCode.as_str(),
            "finding_code"
        );
        assert_eq!(
            DiagnosisPacketFieldClass::ChainOfCustody.as_str(),
            "chain_of_custody"
        );
        assert_eq!(
            RepairHookFieldClass::RepairTransactionId.as_str(),
            "repair_transaction_id"
        );
        assert_eq!(
            RepairHookFieldClass::RepairOutcome.as_str(),
            "repair_outcome"
        );
        assert_eq!(
            RecoveryPostureClass::RestoreNoRerun.as_str(),
            "restore_no_rerun"
        );
        assert_eq!(
            RecoveryPostureClass::DegradedHelper.as_str(),
            "degraded_helper"
        );
        assert_eq!(
            ConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::LineageAdmissionMissingExecutionContextId.as_str(),
            "lineage_admission_missing_execution_context_id"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = SupportExportParityTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
            "expected stable but got findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|f| f.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:finalize_terminal_task_test_and_debug_support_export_parity",
                "2026-05-27T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_export_field_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                SupportExportParityRowClass::ExportFieldBinding
            ) && row.export_field_class == ExportFieldClass::RedactionState
                && row.lane_class == SupportExportParityLaneClass::TaskLane)
        });
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingExportFieldCoverage));
    }

    #[test]
    fn missing_diagnosis_packet_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                SupportExportParityRowClass::DiagnosisPacketBinding
            ) && row.diagnosis_packet_field_class == DiagnosisPacketFieldClass::RedactionClass
                && row.lane_class == SupportExportParityLaneClass::TestLane)
        });
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingDiagnosisPacketCoverage));
    }

    #[test]
    fn missing_repair_hook_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                SupportExportParityRowClass::RepairHookBinding
            ) && row.repair_hook_field_class == RepairHookFieldClass::RepairHookRef
                && row.lane_class == SupportExportParityLaneClass::DebugLane)
        });
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingRepairHookCoverage));
    }

    #[test]
    fn missing_recovery_posture_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                SupportExportParityRowClass::RecoveryPostureAdmission
            ) && row.recovery_posture_class == RecoveryPostureClass::BlockedTarget
                && row.lane_class == SupportExportParityLaneClass::TerminalLane)
        });
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingRecoveryPostureCoverage));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, SupportExportParityRowClass::LineageAdmission)
                && row.lane_class == SupportExportParityLaneClass::TaskLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|p| p.consumer_surface != ConsumerSurface::ConformanceDashboard);
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_diagnosis_packet_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_diagnosis_packet_vocabulary = false;
            }
        }
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::DiagnosisPacketVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = SupportExportParityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|f| f.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
