//! Environment + toolchain manager and execution-context inspector
//! parity truth packet for the M4 stable lane.
//!
//! This module pins how the local, remote/helper, container, and
//! managed execution-context lanes return the **same** inspector
//! fields and the same recovery explanations across the UI (editor
//! run surface, terminal pane, task panel), the CLI/headless
//! inspector (`aureline env inspect`), the Help/About proof card,
//! and the support/export bundle. Surfaces MUST NOT mint local
//! copies, paraphrase fields, or fork their own runtime semantics;
//! they read this packet verbatim.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `inspector_parity_quality` row cannot prove:
//!
//! - the same inspector reveals interpreter, SDK, shell, container
//!   target, remote target, activator, trust state, and policy
//!   source consistently (one `inspector_field_admission` row per
//!   field),
//! - the four parity surfaces (UI, CLI/headless, Help/About,
//!   support/export) each have a `parity_surface_binding` row so the
//!   inspector returns the same resolved fields and the same
//!   explanation paths regardless of where it is invoked,
//!   environment + toolchain manager identity is admitted via a
//!   `toolchain_manager_admission` row so the resolved toolchain
//!   manager is reviewable and never drifts silently,
//! - the five recovery postures (`reconnect`, `restore_no_rerun`,
//!   `blocked_target`, `degraded_helper`, `artifact_provenance`)
//!   each carry a `recovery_admission` row so the lane explains
//!   reconnect, restore-no-rerun, blocked-target, degraded-helper,
//!   and artifact-provenance posture without support-only knowledge,
//! - one stable `execution_context_id` (or equivalent lineage
//!   object) threads through event streams, support packets,
//!   approval tickets, and evidence exports.
//!
//! Every row binds a closed `execution_lane_class`,
//! `inspector_parity_row_class`, `support_class`,
//! `inspector_field_class`, `parity_surface_class`,
//! `recovery_state_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `inspector_parity_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below launch-stable, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! command lines, raw process environment bytes, raw secrets, raw
//! capsule bodies, or ambient credentials past the boundary. A row
//! that claims `launch_stable` while leaving its known limit,
//! downgrade automation, or evidence class unbound is refused; the
//! validator narrows below launch-stable instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`InspectorParityTruthPacket`].
pub const INSPECTOR_PARITY_TRUTH_PACKET_RECORD_KIND: &str =
    "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_packet";

/// Stable record-kind tag for [`InspectorParityTruthSupportExport`].
pub const INSPECTOR_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "finalize_environment_and_toolchain_manager_parity_across_ui_truth_support_export";

/// Integer schema version for the inspector-parity truth packet.
pub const INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INSPECTOR_PARITY_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const INSPECTOR_PARITY_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const INSPECTOR_PARITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const INSPECTOR_PARITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui";

/// Repo-relative path of the checked-in stable packet.
pub const INSPECTOR_PARITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json";

/// Closed execution-context lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityLaneClass {
    /// Local-host execution lane.
    LocalLane,
    /// Remote / helper attach execution lane (SSH and remote agents).
    RemoteHelperLane,
    /// Container execution lane (ad-hoc containers and devcontainers).
    ContainerLane,
    /// Managed execution lane (managed workspace, remote workspace VM,
    /// prebuild runtime, and AI sandbox).
    ManagedLane,
}

impl InspectorParityLaneClass {
    /// Every required execution-context lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::ContainerLane,
        Self::ManagedLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::ContainerLane => "container_lane",
            Self::ManagedLane => "managed_lane",
        }
    }
}

/// Closed inspector-parity row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityRowClass {
    /// The lane's headline inspector-parity qualification row.
    InspectorParityQuality,
    /// A row admitting one inspector field that MUST be revealed
    /// consistently across the parity surfaces.
    InspectorFieldAdmission,
    /// A row binding one parity surface (UI, CLI/headless, Help/About,
    /// or support/export) to the shared inspector object.
    ParitySurfaceBinding,
    /// A row admitting one of the five recovery postures (reconnect,
    /// restore-no-rerun, blocked target, degraded helper, artifact
    /// provenance).
    RecoveryAdmission,
    /// A row admitting environment + toolchain manager identity for
    /// the lane.
    ToolchainManagerAdmission,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into event streams, support packets, approval
    /// tickets, and evidence exports.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl InspectorParityRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectorParityQuality => "inspector_parity_quality",
            Self::InspectorFieldAdmission => "inspector_field_admission",
            Self::ParitySurfaceBinding => "parity_surface_binding",
            Self::RecoveryAdmission => "recovery_admission",
            Self::ToolchainManagerAdmission => "toolchain_manager_admission",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound inspector field token.
    pub const fn requires_inspector_field(self) -> bool {
        matches!(self, Self::InspectorFieldAdmission)
    }

    /// True when this row class requires a bound parity-surface token.
    pub const fn requires_parity_surface(self) -> bool {
        matches!(self, Self::ParitySurfaceBinding)
    }

    /// True when this row class requires a bound recovery-state token.
    pub const fn requires_recovery_state(self) -> bool {
        matches!(self, Self::RecoveryAdmission)
    }
}

/// Closed support-class vocabulary applied to an inspector-parity row.
/// A row is never `launch_stable` while its known limit, downgrade
/// automation, or evidence class is unbound; the validator demotes it
/// instead of inheriting an adjacent launch-stable row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParitySupportClass {
    /// Row claims M4 launch-stable grade for the execution lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable; the narrowing is disclosed.
    LaunchStableBelow,
    /// Row is at beta-grade only (capability sample, not launch-stable).
    BetaGradeOnly,
    /// Row is at preview only (under-review wedge).
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl InspectorParitySupportClass {
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

/// Closed inspector-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `inspector_field_admission` row for
/// each required field so the inspector reveals interpreter, SDK,
/// shell, container target, remote target, activator, trust state, and
/// policy source consistently across the parity surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorFieldClass {
    /// Resolved interpreter (e.g. Python interpreter, Node binary).
    Interpreter,
    /// Resolved SDK / language toolchain (e.g. .NET SDK, Rust toolchain).
    Sdk,
    /// Resolved shell binding (login shell, devcontainer shell).
    Shell,
    /// Resolved container target (ad-hoc container or devcontainer).
    ContainerTarget,
    /// Resolved remote target (SSH host, remote helper, workspace VM).
    RemoteTarget,
    /// Resolved activator (env-manager, devcontainer build, capsule activator).
    Activator,
    /// Resolved trust state (policy epoch, capability envelope).
    TrustState,
    /// Resolved policy source (which layer wins the trust decision).
    PolicySource,
    /// The row is not bound to an inspector field (non-field row classes).
    NotApplicable,
}

impl InspectorFieldClass {
    /// Every required inspector field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 8] = [
        Self::Interpreter,
        Self::Sdk,
        Self::Shell,
        Self::ContainerTarget,
        Self::RemoteTarget,
        Self::Activator,
        Self::TrustState,
        Self::PolicySource,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::Sdk => "sdk",
            Self::Shell => "shell",
            Self::ContainerTarget => "container_target",
            Self::RemoteTarget => "remote_target",
            Self::Activator => "activator",
            Self::TrustState => "trust_state",
            Self::PolicySource => "policy_source",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed parity-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `parity_surface_binding` row for each
/// required surface so the inspector returns the same fields and
/// explanation paths in the UI, CLI/headless, Help/About, and
/// support/export surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurfaceClass {
    /// Editor UI surfaces (run surface, terminal pane, task panel,
    /// debug-prep, request workspace, AI tool, artifact view).
    Ui,
    /// CLI / headless inspector (`aureline env inspect`).
    CliHeadless,
    /// Help / About proof card.
    HelpAbout,
    /// Support / export bundle.
    SupportExport,
    /// The row is not bound to a parity surface (non-surface row classes).
    NotApplicable,
}

impl ParitySurfaceClass {
    /// Every required parity surface that MUST be bound per
    /// `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Ui,
        Self::CliHeadless,
        Self::HelpAbout,
        Self::SupportExport,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::CliHeadless => "cli_headless",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed recovery-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `recovery_admission` row for each
/// structured recovery posture so the inspector can explain reconnect,
/// restore-no-rerun, blocked-target, degraded-helper, and
/// artifact-provenance without support-only knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStateClass {
    /// Reattach / reconnect posture (helper online, target reachable).
    Reconnect,
    /// Restore brought metadata back without silently rerunning tasks,
    /// reattaching debuggers, or reusing a drifted target.
    RestoreNoRerun,
    /// Requested target is blocked by trust, policy, or capability.
    BlockedTarget,
    /// Helper / remote agent reports degraded capabilities.
    DegradedHelper,
    /// Artifact provenance (produced output) survives reattach and
    /// support export with its target identity intact.
    ArtifactProvenance,
    /// The row is not bound to a recovery state (non-recovery row classes).
    NotApplicable,
}

impl RecoveryStateClass {
    /// Every certified recovery state in declaration order. A
    /// `launch_stable` lane MUST cover every state.
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
pub enum InspectorParityEvidenceClass {
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
    /// The row is backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl InspectorParityEvidenceClass {
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

/// Closed known-limit vocabulary attached to an inspector-parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityKnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the container subset.
    ContainerSubsetOnly,
    /// The lane only certifies the managed subset.
    ManagedSubsetOnly,
    /// The lane only certifies a subset of the eight required
    /// inspector fields.
    InspectorFieldSubsetOnly,
    /// The lane only certifies a subset of the four required parity
    /// surfaces.
    ParitySurfaceSubsetOnly,
    /// The lane only certifies a subset of the five required recovery
    /// postures.
    RecoveryStateSubsetOnly,
    /// The lane certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl InspectorParityKnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::LocalLaneSubsetOnly => "local_lane_subset_only",
            Self::RemoteHelperSubsetOnly => "remote_helper_subset_only",
            Self::ContainerSubsetOnly => "container_subset_only",
            Self::ManagedSubsetOnly => "managed_subset_only",
            Self::InspectorFieldSubsetOnly => "inspector_field_subset_only",
            Self::ParitySurfaceSubsetOnly => "parity_surface_subset_only",
            Self::RecoveryStateSubsetOnly => "recovery_state_subset_only",
            Self::UnsupportedRuntimeTarget => "unsupported_runtime_target",
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

/// Closed downgrade-automation vocabulary attached to an
/// inspector-parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityDowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when the inspector returns different
    /// fields or explanations on two parity surfaces.
    AutoNarrowOnParitySurfaceBreak,
    /// Automatically narrow when a required inspector field is unbound.
    AutoNarrowOnInspectorFieldGap,
    /// Automatically narrow when a required recovery posture is
    /// unbound.
    AutoNarrowOnRecoveryStateGap,
    /// Automatically narrow when the toolchain / environment manager
    /// identity drifts between two parity surfaces.
    AutoNarrowOnToolchainManagerDrift,
    /// Automatically narrow when the lineage object breaks (no
    /// `execution_context_id` binding survives across event streams,
    /// support packets, approval tickets, or evidence exports).
    AutoNarrowOnLineageBreak,
    /// Automatically narrow when restore would silently rerun a task,
    /// reattach a debugger, or reuse a drifted target.
    AutoNarrowOnSilentRerun,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl InspectorParityDowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnParitySurfaceBreak => "auto_narrow_on_parity_surface_break",
            Self::AutoNarrowOnInspectorFieldGap => "auto_narrow_on_inspector_field_gap",
            Self::AutoNarrowOnRecoveryStateGap => "auto_narrow_on_recovery_state_gap",
            Self::AutoNarrowOnToolchainManagerDrift => "auto_narrow_on_toolchain_manager_drift",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoNarrowOnSilentRerun => "auto_narrow_on_silent_rerun",
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

/// Closed confidence-class vocabulary for an inspector-parity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl InspectorParityConfidenceClass {
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
pub enum InspectorParityPromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl InspectorParityPromotionState {
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
pub enum InspectorParityFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the inspector-parity packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required execution lane has no row.
    MissingExecutionLaneCoverage,
    /// A lane claiming launch_stable is missing a required inspector
    /// field admission row.
    MissingInspectorFieldCoverage,
    /// A lane claiming launch_stable is missing a required parity
    /// surface binding.
    MissingParitySurfaceBindingCoverage,
    /// A lane claiming launch_stable is missing a required recovery
    /// admission row.
    MissingRecoveryStateCoverage,
    /// A lane claiming launch_stable is missing the required
    /// toolchain-manager admission row.
    MissingToolchainManagerAdmission,
    /// A lane claiming launch_stable is missing the required lineage
    /// admission row.
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
    /// A row with a non-`none_declared` known limit drops its
    /// disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its
    /// disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// An inspector-field-admission row drops its inspector field
    /// binding.
    InspectorFieldNotApplicable,
    /// A non-inspector-field row binds a field it cannot certify.
    InspectorFieldNotPermittedOnRowClass,
    /// A parity-surface-binding row drops its parity-surface binding.
    ParitySurfaceNotApplicable,
    /// A non-parity-surface row binds a surface it cannot certify.
    ParitySurfaceNotPermittedOnRowClass,
    /// A recovery-admission row drops its recovery-state binding.
    RecoveryStateNotApplicable,
    /// A non-recovery row binds a recovery state it cannot certify.
    RecoveryStateNotPermittedOnRowClass,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A toolchain-manager-admission row does not bind a toolchain or
    /// environment manager id.
    ToolchainManagerAdmissionMissingManagerId,
    /// A recovery_admission row binding restore_no_rerun does not
    /// attest no-silent-rerun.
    RestoreRecoveryAdmitsSilentRerun,
    /// A row admits raw command lines, process environment bytes, or
    /// other private material past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops inspector-parity truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the inspector-field vocabulary.
    InspectorFieldVocabularyCollapsed,
    /// A projection collapses the parity-surface vocabulary.
    ParitySurfaceVocabularyCollapsed,
    /// A projection collapses the recovery-state vocabulary.
    RecoveryStateVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl InspectorParityFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingExecutionLaneCoverage => "missing_execution_lane_coverage",
            Self::MissingInspectorFieldCoverage => "missing_inspector_field_coverage",
            Self::MissingParitySurfaceBindingCoverage => "missing_parity_surface_binding_coverage",
            Self::MissingRecoveryStateCoverage => "missing_recovery_state_coverage",
            Self::MissingToolchainManagerAdmission => "missing_toolchain_manager_admission",
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
            Self::InspectorFieldNotApplicable => "inspector_field_not_applicable",
            Self::InspectorFieldNotPermittedOnRowClass => {
                "inspector_field_not_permitted_on_row_class"
            }
            Self::ParitySurfaceNotApplicable => "parity_surface_not_applicable",
            Self::ParitySurfaceNotPermittedOnRowClass => {
                "parity_surface_not_permitted_on_row_class"
            }
            Self::RecoveryStateNotApplicable => "recovery_state_not_applicable",
            Self::RecoveryStateNotPermittedOnRowClass => {
                "recovery_state_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::ToolchainManagerAdmissionMissingManagerId => {
                "toolchain_manager_admission_missing_manager_id"
            }
            Self::RestoreRecoveryAdmitsSilentRerun => "restore_recovery_admits_silent_rerun",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::InspectorFieldVocabularyCollapsed => "inspector_field_vocabulary_collapsed",
            Self::ParitySurfaceVocabularyCollapsed => "parity_surface_vocabulary_collapsed",
            Self::RecoveryStateVocabularyCollapsed => "recovery_state_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the inspector-parity packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorParityConsumerSurface {
    /// Editor run / launch surface (per-pane "why this target?" chip).
    EditorRunSurface,
    /// Terminal pane chrome and session header.
    TerminalPane,
    /// Task panel chrome and per-run header.
    TaskPanel,
    /// CLI or headless inspection surface (`aureline env inspect`).
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

impl InspectorParityConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::EditorRunSurface,
        Self::TerminalPane,
        Self::TaskPanel,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRunSurface => "editor_run_surface",
            Self::TerminalPane => "terminal_pane",
            Self::TaskPanel => "task_panel",
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
pub struct InspectorParityValidationFinding {
    /// Closed finding kind.
    pub finding_kind: InspectorParityFindingKind,
    /// Finding severity.
    pub severity: InspectorParityFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl InspectorParityValidationFinding {
    fn new(
        finding_kind: InspectorParityFindingKind,
        severity: InspectorParityFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One inspector-parity truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorParityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Execution-context lane this row certifies.
    pub lane_class: InspectorParityLaneClass,
    /// Inspector-parity row class.
    pub row_class: InspectorParityRowClass,
    /// Support class claimed by the row.
    pub support_class: InspectorParitySupportClass,
    /// Inspector field certified by the row (or `not_applicable`).
    pub inspector_field_class: InspectorFieldClass,
    /// Parity surface certified by the row (or `not_applicable`).
    pub parity_surface_class: ParitySurfaceClass,
    /// Recovery state admitted by the row (or `not_applicable`).
    pub recovery_state_class: RecoveryStateClass,
    /// Evidence class backing the row.
    pub evidence_class: InspectorParityEvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: InspectorParityKnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: InspectorParityDowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: InspectorParityConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For toolchain_manager_admission rows, the bound toolchain or
    /// environment manager id (e.g. `pyenv`, `rye`, `volta`, `nvm`,
    /// `cargo-rustup`, `mise`). Required when
    /// `row_class == ToolchainManagerAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toolchain_manager_id_binding: Option<String>,
    /// For recovery_admission rows that bind `restore_no_rerun`, true
    /// when the row attests that restore brought metadata back without
    /// silently rerunning tasks, reattaching debuggers, or reusing a
    /// drifted target.
    #[serde(default)]
    pub restore_preserves_no_rerun: bool,
    /// True when raw command lines / process env bytes / raw capsule
    /// bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl InspectorParityRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorParityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: InspectorParityConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Inspector-parity packet id consumed by the projection.
    pub inspector_parity_packet_id_ref: String,
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
    /// True when the inspector-field vocabulary is preserved verbatim.
    pub preserves_inspector_field_vocabulary: bool,
    /// True when the parity-surface vocabulary is preserved verbatim.
    pub preserves_parity_surface_vocabulary: bool,
    /// True when the recovery-state vocabulary is preserved verbatim.
    pub preserves_recovery_state_vocabulary: bool,
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

impl InspectorParityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.inspector_parity_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_inspector_field_vocabulary
            && self.preserves_parity_surface_vocabulary
            && self.preserves_recovery_state_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`InspectorParityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorParityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Execution-context lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<InspectorParityLaneClass>,
    /// Inspector-parity rows.
    #[serde(default)]
    pub rows: Vec<InspectorParityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<InspectorParityConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying local, remote/helper, container,
/// and managed environment + toolchain manager and execution-context
/// inspector parity at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorParityTruthPacket {
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
    /// Execution-context lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<InspectorParityLaneClass>,
    /// Inspector-parity rows.
    #[serde(default)]
    pub rows: Vec<InspectorParityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<InspectorParityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: InspectorParityPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<InspectorParityValidationFinding>,
}

impl InspectorParityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: InspectorParityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: INSPECTOR_PARITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: InspectorParityPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable inspector-parity invariants.
    pub fn validate(&self) -> Vec<InspectorParityValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == InspectorParityFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: InspectorParityConsumerSurface) -> bool {
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
            .map(InspectorParityLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(InspectorParityRowClass::as_str)
            .collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter()
            .map(InspectorParitySupportClass::as_str)
            .collect()
    }

    /// Returns the unique inspector-field tokens observed across rows.
    pub fn inspector_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.inspector_field_class);
        }
        set.into_iter().map(InspectorFieldClass::as_str).collect()
    }

    /// Returns the unique parity-surface tokens observed across rows.
    pub fn parity_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.parity_surface_class);
        }
        set.into_iter().map(ParitySurfaceClass::as_str).collect()
    }

    /// Returns the unique recovery-state tokens observed across rows.
    pub fn recovery_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.recovery_state_class);
        }
        set.into_iter().map(RecoveryStateClass::as_str).collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter()
            .map(InspectorParityEvidenceClass::as_str)
            .collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter()
            .map(InspectorParityKnownLimitClass::as_str)
            .collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(InspectorParityDowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> InspectorParityTruthSupportExport {
        InspectorParityTruthSupportExport {
            record_kind: INSPECTOR_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            inspector_parity_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            inspector_parity_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<InspectorParityValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != INSPECTOR_PARITY_TRUTH_PACKET_RECORD_KIND {
            findings.push(InspectorParityValidationFinding::new(
                InspectorParityFindingKind::WrongRecordKind,
                InspectorParityFindingSeverity::Blocker,
                "inspector-parity packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION {
            findings.push(InspectorParityValidationFinding::new(
                InspectorParityFindingKind::WrongSchemaVersion,
                InspectorParityFindingSeverity::Blocker,
                "inspector-parity packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(InspectorParityValidationFinding::new(
                InspectorParityFindingKind::MissingIdentity,
                InspectorParityFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(InspectorParityValidationFinding::new(
                InspectorParityFindingKind::MissingExecutionLaneCoverage,
                InspectorParityFindingSeverity::Blocker,
                "packet must declare at least one covered execution-context lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingExecutionLaneCoverage,
                    InspectorParityFindingSeverity::Blocker,
                    format!("no row covers execution-context lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingIdentity,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::RawSourceMaterialPresent,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw command lines, raw env bytes, or raw capsule bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::SecretsPresent,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::AmbientAuthorityPresent,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingSupportClass,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingKnownLimit,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingDowngradeAutomation,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingEvidenceClass,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(row.support_class, InspectorParitySupportClass::LaunchStable)
                && !row.all_bindings_satisfied()
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::LaunchStableWithUnboundBinding,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::NarrowedRowMissingDisclosureRef,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::KnownLimitMissingDisclosureRef,
                    InspectorParityFindingSeverity::Blocker,
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
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::DowngradeAutomationMissingDisclosureRef,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingEvidenceRefs,
                    InspectorParityFindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if row.row_class.requires_inspector_field()
                && matches!(
                    row.inspector_field_class,
                    InspectorFieldClass::NotApplicable
                )
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::InspectorFieldNotApplicable,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} is an inspector_field_admission but has no bound field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_inspector_field()
                && !matches!(
                    row.inspector_field_class,
                    InspectorFieldClass::NotApplicable
                )
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::InspectorFieldNotPermittedOnRowClass,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds inspector field {}; only inspector_field_admission rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.inspector_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_parity_surface()
                && matches!(row.parity_surface_class, ParitySurfaceClass::NotApplicable)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::ParitySurfaceNotApplicable,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} is a parity_surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_parity_surface()
                && !matches!(row.parity_surface_class, ParitySurfaceClass::NotApplicable)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::ParitySurfaceNotPermittedOnRowClass,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds parity surface {}; only parity_surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.parity_surface_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_recovery_state()
                && matches!(row.recovery_state_class, RecoveryStateClass::NotApplicable)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::RecoveryStateNotApplicable,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} is a recovery_admission but has no bound recovery state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_recovery_state()
                && !matches!(row.recovery_state_class, RecoveryStateClass::NotApplicable)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::RecoveryStateNotPermittedOnRowClass,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds recovery state {}; only recovery_admission rows may bind a state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.recovery_state_class.as_str()
                    ),
                ));
            }

            if matches!(row.row_class, InspectorParityRowClass::LineageAdmission)
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::LineageAdmissionMissingExecutionContextId,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} is a lineage_admission but has no bound execution_context_id",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.row_class,
                InspectorParityRowClass::ToolchainManagerAdmission
            ) && row
                .toolchain_manager_id_binding
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                .unwrap_or(true)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::ToolchainManagerAdmissionMissingManagerId,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} is a toolchain_manager_admission but has no bound manager id",
                        row.row_id
                    ),
                ));
            }

            if matches!(row.row_class, InspectorParityRowClass::RecoveryAdmission)
                && matches!(row.recovery_state_class, RecoveryStateClass::RestoreNoRerun)
                && !row.restore_preserves_no_rerun
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::RestoreRecoveryAdmitsSilentRerun,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "row {} binds restore_no_rerun but does not attest no-silent-rerun",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.confidence_class,
                InspectorParityConfidenceClass::LowConfidence
            ) && matches!(row.support_class, InspectorParitySupportClass::LaunchStable)
            {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::LaunchStableWithUnboundBinding,
                    InspectorParityFindingSeverity::Warning,
                    format!(
                        "row {} claims launch_stable at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        InspectorParityRowClass::InspectorParityQuality
                    )
                    && matches!(row.support_class, InspectorParitySupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for field in InspectorFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            InspectorParityRowClass::InspectorFieldAdmission
                        )
                        && row.inspector_field_class == field
                });
                if !covered {
                    findings.push(InspectorParityValidationFinding::new(
                        InspectorParityFindingKind::MissingInspectorFieldCoverage,
                        InspectorParityFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no inspector_field_admission row for {}",
                            lane.as_str(),
                            field.as_str()
                        ),
                    ));
                }
            }

            for surface in ParitySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, InspectorParityRowClass::ParitySurfaceBinding)
                        && row.parity_surface_class == surface
                });
                if !covered {
                    findings.push(InspectorParityValidationFinding::new(
                        InspectorParityFindingKind::MissingParitySurfaceBindingCoverage,
                        InspectorParityFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no parity_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            for state in RecoveryStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, InspectorParityRowClass::RecoveryAdmission)
                        && row.recovery_state_class == state
                });
                if !covered {
                    findings.push(InspectorParityValidationFinding::new(
                        InspectorParityFindingKind::MissingRecoveryStateCoverage,
                        InspectorParityFindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no recovery_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            let has_toolchain_manager = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        InspectorParityRowClass::ToolchainManagerAdmission
                    )
                    && row
                        .toolchain_manager_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_toolchain_manager {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingToolchainManagerAdmission,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no toolchain_manager_admission row binding a manager id",
                        lane.as_str()
                    ),
                ));
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, InspectorParityRowClass::LineageAdmission)
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_lineage {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingLineageAdmission,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no lineage_admission row binding execution_context_id",
                        lane.as_str()
                    ),
                ));
            }
        }

        for required_surface in InspectorParityConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::MissingConsumerProjection,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::ConsumerProjectionDrift,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve inspector-parity truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::LaneVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::RowClassVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::SupportClassVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_inspector_field_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::InspectorFieldVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the inspector-field vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_parity_surface_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::ParitySurfaceVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the parity-surface vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_recovery_state_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::RecoveryStateVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the recovery-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::KnownLimitVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::DowngradeAutomationVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::EvidenceClassVocabularyCollapsed,
                    InspectorParityFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != InspectorParityFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(InspectorParityValidationFinding::new(
                    InspectorParityFindingKind::PromotionStateMismatch,
                    InspectorParityFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[InspectorParityValidationFinding],
) -> InspectorParityPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == InspectorParityFindingSeverity::Blocker)
    {
        InspectorParityPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == InspectorParityFindingSeverity::Warning)
    {
        InspectorParityPromotionState::NarrowedBelowStable
    } else {
        InspectorParityPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorParityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub inspector_parity_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub inspector_parity_packet: InspectorParityTruthPacket,
}

impl InspectorParityTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == INSPECTOR_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION
            && self.inspector_parity_packet_id_ref == self.inspector_parity_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.inspector_parity_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable inspector-parity packet.
#[derive(Debug)]
pub enum InspectorParityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<InspectorParityValidationFinding>),
}

impl fmt::Display for InspectorParityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "inspector-parity packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "inspector-parity packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for InspectorParityTruthArtifactError {}

/// Returns the checked-in stable inspector-parity truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_inspector_parity_truth_packet(
) -> Result<InspectorParityTruthPacket, InspectorParityTruthArtifactError> {
    let packet: InspectorParityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json"
    )))
    .map_err(InspectorParityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(InspectorParityTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        INSPECTOR_PARITY_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        INSPECTOR_PARITY_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: InspectorParityLaneClass) -> InspectorParityRow {
        InspectorParityRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: InspectorParityRowClass::InspectorParityQuality,
            support_class: InspectorParitySupportClass::LaunchStable,
            inspector_field_class: InspectorFieldClass::NotApplicable,
            parity_surface_class: ParitySurfaceClass::NotApplicable,
            recovery_state_class: RecoveryStateClass::NotApplicable,
            evidence_class: InspectorParityEvidenceClass::ReleaseEvidenceReview,
            known_limit_class: InspectorParityKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                InspectorParityDowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: InspectorParityConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            toolchain_manager_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn field_row(
        prefix: &str,
        lane: InspectorParityLaneClass,
        field: InspectorFieldClass,
    ) -> InspectorParityRow {
        InspectorParityRow {
            row_id: format!("row:{prefix}:field:{}", field.as_str()),
            lane_class: lane,
            row_class: InspectorParityRowClass::InspectorFieldAdmission,
            support_class: InspectorParitySupportClass::LaunchStable,
            inspector_field_class: field,
            parity_surface_class: ParitySurfaceClass::NotApplicable,
            recovery_state_class: RecoveryStateClass::NotApplicable,
            evidence_class: InspectorParityEvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: InspectorParityKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                InspectorParityDowngradeAutomationClass::AutoNarrowOnInspectorFieldGap,
            confidence_class: InspectorParityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_inspector_field_gap", doc_ref())),
            execution_context_id_binding: None,
            toolchain_manager_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn surface_row(
        prefix: &str,
        lane: InspectorParityLaneClass,
        surface: ParitySurfaceClass,
    ) -> InspectorParityRow {
        InspectorParityRow {
            row_id: format!("row:{prefix}:surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: InspectorParityRowClass::ParitySurfaceBinding,
            support_class: InspectorParitySupportClass::LaunchStable,
            inspector_field_class: InspectorFieldClass::NotApplicable,
            parity_surface_class: surface,
            recovery_state_class: RecoveryStateClass::NotApplicable,
            evidence_class: InspectorParityEvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: InspectorParityKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                InspectorParityDowngradeAutomationClass::AutoNarrowOnParitySurfaceBreak,
            confidence_class: InspectorParityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_parity_surface_break", doc_ref())),
            execution_context_id_binding: None,
            toolchain_manager_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn recovery_row(
        prefix: &str,
        lane: InspectorParityLaneClass,
        state: RecoveryStateClass,
    ) -> InspectorParityRow {
        let restore_attest = matches!(state, RecoveryStateClass::RestoreNoRerun);
        let automation = if restore_attest {
            InspectorParityDowngradeAutomationClass::AutoNarrowOnSilentRerun
        } else {
            InspectorParityDowngradeAutomationClass::AutoNarrowOnRecoveryStateGap
        };
        let disclosure = if restore_attest {
            format!("{}#auto_narrow_on_silent_rerun", doc_ref())
        } else {
            format!("{}#auto_narrow_on_recovery_state_gap", doc_ref())
        };
        InspectorParityRow {
            row_id: format!("row:{prefix}:recovery:{}", state.as_str()),
            lane_class: lane,
            row_class: InspectorParityRowClass::RecoveryAdmission,
            support_class: InspectorParitySupportClass::LaunchStable,
            inspector_field_class: InspectorFieldClass::NotApplicable,
            parity_surface_class: ParitySurfaceClass::NotApplicable,
            recovery_state_class: state,
            evidence_class: InspectorParityEvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: InspectorParityKnownLimitClass::NoneDeclared,
            downgrade_automation_class: automation,
            confidence_class: InspectorParityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(disclosure),
            execution_context_id_binding: None,
            toolchain_manager_id_binding: None,
            restore_preserves_no_rerun: restore_attest,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn toolchain_manager_row(prefix: &str, lane: InspectorParityLaneClass) -> InspectorParityRow {
        InspectorParityRow {
            row_id: format!("row:{prefix}:toolchain_manager_admission"),
            lane_class: lane,
            row_class: InspectorParityRowClass::ToolchainManagerAdmission,
            support_class: InspectorParitySupportClass::LaunchStable,
            inspector_field_class: InspectorFieldClass::NotApplicable,
            parity_surface_class: ParitySurfaceClass::NotApplicable,
            recovery_state_class: RecoveryStateClass::NotApplicable,
            evidence_class: InspectorParityEvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: InspectorParityKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                InspectorParityDowngradeAutomationClass::AutoNarrowOnToolchainManagerDrift,
            confidence_class: InspectorParityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_toolchain_manager_drift",
                doc_ref()
            )),
            execution_context_id_binding: None,
            toolchain_manager_id_binding: Some(format!("toolchain_manager:m4:{prefix}")),
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: InspectorParityLaneClass) -> InspectorParityRow {
        InspectorParityRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: InspectorParityRowClass::LineageAdmission,
            support_class: InspectorParitySupportClass::LaunchStable,
            inspector_field_class: InspectorFieldClass::NotApplicable,
            parity_surface_class: ParitySurfaceClass::NotApplicable,
            recovery_state_class: RecoveryStateClass::NotApplicable,
            evidence_class: InspectorParityEvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: InspectorParityKnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                InspectorParityDowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: InspectorParityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:lineage")),
            toolchain_manager_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: InspectorParityConsumerSurface) -> InspectorParityConsumerProjection {
        InspectorParityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            inspector_parity_packet_id_ref:
                "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_inspector_field_vocabulary: true,
            preserves_parity_surface_vocabulary: true,
            preserves_recovery_state_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: InspectorParityLaneClass, prefix: &str) -> Vec<InspectorParityRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for field in InspectorFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(field_row(prefix, lane, field));
        }
        for surface in ParitySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(surface_row(prefix, lane, surface));
        }
        for state in RecoveryStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(recovery_row(prefix, lane, state));
        }
        out.push(toolchain_manager_row(prefix, lane));
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> InspectorParityTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(InspectorParityLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            InspectorParityLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(
            InspectorParityLaneClass::ContainerLane,
            "container",
        ));
        rows.extend(lane_rows(InspectorParityLaneClass::ManagedLane, "managed"));
        InspectorParityTruthPacketInput {
            packet_id: "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.finalize_environment_and_toolchain_manager_parity_across_ui"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: InspectorParityLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: InspectorParityConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(InspectorParityLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(
            InspectorParityLaneClass::ManagedLane.as_str(),
            "managed_lane"
        );
        assert_eq!(
            InspectorParityRowClass::InspectorParityQuality.as_str(),
            "inspector_parity_quality"
        );
        assert_eq!(
            InspectorParityRowClass::ToolchainManagerAdmission.as_str(),
            "toolchain_manager_admission"
        );
        assert_eq!(
            InspectorParitySupportClass::LaunchStable.as_str(),
            "launch_stable"
        );
        assert_eq!(
            InspectorParitySupportClass::SupportUnbound.as_str(),
            "support_unbound"
        );
        assert_eq!(InspectorFieldClass::Interpreter.as_str(), "interpreter");
        assert_eq!(InspectorFieldClass::PolicySource.as_str(), "policy_source");
        assert_eq!(ParitySurfaceClass::Ui.as_str(), "ui");
        assert_eq!(ParitySurfaceClass::SupportExport.as_str(), "support_export");
        assert_eq!(RecoveryStateClass::Reconnect.as_str(), "reconnect");
        assert_eq!(
            RecoveryStateClass::ArtifactProvenance.as_str(),
            "artifact_provenance"
        );
        assert_eq!(
            InspectorParityEvidenceClass::EvidenceUnbound.as_str(),
            "evidence_unbound"
        );
        assert_eq!(
            InspectorParityKnownLimitClass::LimitUnbound.as_str(),
            "limit_unbound"
        );
        assert_eq!(
            InspectorParityDowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            InspectorParityConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            InspectorParityPromotionState::BlocksStable.as_str(),
            "blocks_stable"
        );
        assert_eq!(
            InspectorParityFindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            InspectorParityFindingKind::ToolchainManagerAdmissionMissingManagerId.as_str(),
            "toolchain_manager_admission_missing_manager_id"
        );
        assert_eq!(
            InspectorParityFindingKind::RestoreRecoveryAdmitsSilentRerun.as_str(),
            "restore_recovery_admits_silent_rerun"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = InspectorParityTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::Stable,
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
                "support:m4:finalize_environment_and_toolchain_manager_parity_across_ui",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = InspectorParityEvidenceClass::EvidenceUnbound;
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(
            |finding| finding.finding_kind == InspectorParityFindingKind::MissingEvidenceClass
        ));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_inspector_field_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                InspectorParityRowClass::InspectorFieldAdmission
            ) && row.inspector_field_class == InspectorFieldClass::PolicySource
                && row.lane_class == InspectorParityLaneClass::LocalLane)
        });
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::MissingInspectorFieldCoverage));
    }

    #[test]
    fn missing_parity_surface_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(row.row_class, InspectorParityRowClass::ParitySurfaceBinding)
                && row.parity_surface_class == ParitySurfaceClass::HelpAbout
                && row.lane_class == InspectorParityLaneClass::LocalLane)
        });
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::MissingParitySurfaceBindingCoverage));
    }

    #[test]
    fn missing_recovery_state_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(row.row_class, InspectorParityRowClass::RecoveryAdmission)
                && row.recovery_state_class == RecoveryStateClass::ArtifactProvenance
                && row.lane_class == InspectorParityLaneClass::LocalLane)
        });
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::MissingRecoveryStateCoverage));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, InspectorParityRowClass::LineageAdmission)
                && row.lane_class == InspectorParityLaneClass::LocalLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind
                == InspectorParityFindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn toolchain_manager_admission_without_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                InspectorParityRowClass::ToolchainManagerAdmission
            ) && row.lane_class == InspectorParityLaneClass::LocalLane
            {
                row.toolchain_manager_id_binding = None;
                break;
            }
        }
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind
                == InspectorParityFindingKind::ToolchainManagerAdmissionMissingManagerId
        }));
    }

    #[test]
    fn restore_recovery_admitting_silent_rerun_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, InspectorParityRowClass::RecoveryAdmission)
                && row.recovery_state_class == RecoveryStateClass::RestoreNoRerun
                && row.lane_class == InspectorParityLaneClass::LocalLane
            {
                row.restore_preserves_no_rerun = false;
                break;
            }
        }
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == InspectorParityFindingKind::RestoreRecoveryAdmitsSilentRerun
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = InspectorParitySupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != InspectorParityConsumerSurface::ConformanceDashboard
        });
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_parity_surface_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == InspectorParityConsumerSurface::HelpAbout {
                projection.preserves_parity_surface_vocabulary = false;
            }
        }
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::ParitySurfaceVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = InspectorParityTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            InspectorParityPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == InspectorParityFindingKind::RawSourceMaterialPresent));
    }
}
