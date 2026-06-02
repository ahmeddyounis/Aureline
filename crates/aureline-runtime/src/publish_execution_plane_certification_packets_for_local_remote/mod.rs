//! Execution-plane certification truth packet for the M4 stable lane.
//!
//! This module pins how the local, remote/helper, and enterprise-network
//! execution-plane lanes stay one boundary truth across the editor run
//! surface, terminal pane, task panel, test explorer, debug session,
//! artifact viewer, request editor, preview surface, CLI/headless
//! inspector, support export, release proof index, Help/About proof
//! card, and the conformance dashboard. The three execution-plane lanes
//! are certified at the M4 launch-stable grade. Surfaces MUST NOT mint
//! local copies, fork their own runtime semantics, or paraphrase
//! execution-plane posture; they read this packet verbatim.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `execution_plane_certification_quality` row cannot prove:
//!
//! - the same execution-plane fields and explanation paths cross
//!   every run-capable surface (terminal, task, test, debug,
//!   artifact, request_workspace, preview, CLI/headless, docs/help,
//!   support/export),
//! - requested-vs-materialized target identity is admitted with a
//!   structured target admission, not surface-local guesswork,
//! - route truth is admitted for local, remote/helper, and enterprise-network
//!   routes so the surface never falls back to generic "connection failed" copy,
//! - restore and reconnect honesty is preserved — reopening a surface
//!   restores metadata, panes, and transcripts without silently
//!   rerunning tasks, reattaching debuggers, or reusing a drifted
//!   target,
//! - degraded helper posture is admitted so a remote/helper attach
//!   cannot silently assume full capability,
//! - artifact provenance is tracked so output lineage survives after
//!   the live run surface is gone,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through event streams, support packets, approval tickets,
//!   and evidence exports.
//!
//! Every row binds a closed `execution_plane_lane_class`,
//! `execution_plane_row_class`, `support_class`,
//! `surface_binding_class`, `route_state_class`, `reconnect_state_class`,
//! `degraded_helper_state_class`, `artifact_provenance_state_class`,
//! `evidence_class`, `known_limit_class`, `downgrade_automation_class`,
//! and `confidence_class` plus an `evidence_refs` array and a
//! `disclosure_ref` whenever the row is narrowed below launch-stable,
//! declares a non-`none_declared` known limit, or binds a non-`none`
//! downgrade automation.
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

/// Stable record-kind tag for [`ExecutionPlaneTruthPacket`].
pub const EXECUTION_PLANE_TRUTH_PACKET_RECORD_KIND: &str =
    "publish_execution_plane_certification_packets_for_local_remote_truth_stable_packet";

/// Stable record-kind tag for [`ExecutionPlaneTruthSupportExport`].
pub const EXECUTION_PLANE_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "publish_execution_plane_certification_packets_for_local_remote_truth_support_export";

/// Integer schema version for the execution-plane truth packet.
pub const EXECUTION_PLANE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EXECUTION_PLANE_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/publish_execution_plane_certification_packets_for_local_remote.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const EXECUTION_PLANE_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const EXECUTION_PLANE_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const EXECUTION_PLANE_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/publish_execution_plane_certification_packets_for_local_remote";

/// Repo-relative path of the checked-in stable packet.
pub const EXECUTION_PLANE_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/publish_execution_plane_certification_packets_for_local_remote_truth_packet.json";

/// Closed execution-plane lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionPlaneLaneClass {
    /// Local-host execution lane.
    LocalLane,
    /// Remote / helper attach execution lane (SSH and remote agents).
    RemoteHelperLane,
    /// Enterprise-network execution lane (managed VPN, zero-trust,
    /// and corporate-network routes).
    EnterpriseNetworkLane,
}

impl ExecutionPlaneLaneClass {
    /// Every required execution-plane lane, in declaration order.
    pub const REQUIRED: [Self; 3] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::EnterpriseNetworkLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::EnterpriseNetworkLane => "enterprise_network_lane",
        }
    }
}

/// Closed execution-plane row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionPlaneRowClass {
    /// The lane's headline execution-plane-certification qualification row.
    ExecutionPlaneCertificationQuality,
    /// A row binding one run-capable surface (terminal, task, test,
    /// debug, artifact, request_workspace, preview, CLI/headless,
    /// docs/help, support/export) to the shared execution-plane
    /// object.
    SurfaceBinding,
    /// A row admitting requested-vs-materialized target identity for
    /// the lane.
    TargetAdmission,
    /// A row admitting route truth for the lane.
    RouteAdmission,
    /// A row certifying restore/reconnect honesty (metadata restore
    /// without silent rerun, reattach, or drifted-target reuse).
    RestoreRerunHonesty,
    /// A row admitting reconnect posture for the lane.
    ReconnectAdmission,
    /// A row admitting degraded helper posture for the lane.
    DegradedHelperAdmission,
    /// A row admitting artifact provenance posture for the lane.
    ArtifactProvenanceAdmission,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into event streams, support packets, approval
    /// tickets, and evidence exports.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl ExecutionPlaneRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionPlaneCertificationQuality => "execution_plane_certification_quality",
            Self::SurfaceBinding => "surface_binding",
            Self::TargetAdmission => "target_admission",
            Self::RouteAdmission => "route_admission",
            Self::RestoreRerunHonesty => "restore_rerun_honesty",
            Self::ReconnectAdmission => "reconnect_admission",
            Self::DegradedHelperAdmission => "degraded_helper_admission",
            Self::ArtifactProvenanceAdmission => "artifact_provenance_admission",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound surface-binding token.
    pub const fn requires_surface_binding(self) -> bool {
        matches!(self, Self::SurfaceBinding)
    }

    /// True when this row class requires a bound route-state token.
    pub const fn requires_route_state(self) -> bool {
        matches!(self, Self::RouteAdmission)
    }

    /// True when this row class requires a bound reconnect-state token.
    pub const fn requires_reconnect_state(self) -> bool {
        matches!(self, Self::ReconnectAdmission)
    }

    /// True when this row class requires a bound degraded-helper-state token.
    pub const fn requires_degraded_helper_state(self) -> bool {
        matches!(self, Self::DegradedHelperAdmission)
    }

    /// True when this row class requires a bound artifact-provenance-state token.
    pub const fn requires_artifact_provenance_state(self) -> bool {
        matches!(self, Self::ArtifactProvenanceAdmission)
    }
}

/// Closed support-class vocabulary applied to an execution-plane row.
/// A row is never `launch_stable` while its known limit, downgrade
/// automation, or evidence class is unbound; the validator demotes it
/// instead of inheriting an adjacent launch-stable row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
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

/// Closed surface-binding vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `surface_binding` row for each
/// required surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceBindingClass {
    /// The row certifies the terminal-pane binding.
    Terminal,
    /// The row certifies the task-run binding.
    Task,
    /// The row certifies the test-run binding.
    Test,
    /// The row certifies the debug-prep / debug-run binding.
    Debug,
    /// The row certifies the artifact / output binding.
    Artifact,
    /// The row certifies the request-workspace binding.
    RequestWorkspace,
    /// The row certifies the preview surface binding.
    Preview,
    /// The row certifies the CLI / headless inspection binding.
    CliHeadless,
    /// The row certifies the docs / help disclosure binding.
    DocsHelp,
    /// The row certifies the support / export binding.
    SupportExport,
    /// The row certifies the conformance dashboard binding.
    ConformanceDashboard,
    /// The row is not bound to a launch surface.
    NotApplicable,
}

impl SurfaceBindingClass {
    /// Every required run-capable surface that MUST be bound per
    /// `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 11] = [
        Self::Terminal,
        Self::Task,
        Self::Test,
        Self::Debug,
        Self::Artifact,
        Self::RequestWorkspace,
        Self::Preview,
        Self::CliHeadless,
        Self::DocsHelp,
        Self::SupportExport,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Task => "task",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Artifact => "artifact",
            Self::RequestWorkspace => "request_workspace",
            Self::Preview => "preview",
            Self::CliHeadless => "cli_headless",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::ConformanceDashboard => "conformance_dashboard",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed route-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `route_admission` row for each
/// structured route state so the surface never falls back to
/// generic connection-failure copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteStateClass {
    /// Local-host route is resolved and reachable.
    LocalRoute,
    /// Remote / helper route is resolved and reachable.
    RemoteHelperRoute,
    /// Enterprise-network route is resolved and reachable.
    EnterpriseNetworkRoute,
    /// Route / tunnel posture drifted relative to the stored binding.
    RouteDrift,
    /// Target is blocked by policy, trust, or capability.
    BlockedTarget,
    /// The row is not bound to a route state (non-route-admission row classes).
    NotApplicable,
}

impl RouteStateClass {
    /// Every certified route state in declaration order. A
    /// `launch_stable` lane MUST cover every state.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::LocalRoute,
        Self::RemoteHelperRoute,
        Self::EnterpriseNetworkRoute,
        Self::RouteDrift,
        Self::BlockedTarget,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRoute => "local_route",
            Self::RemoteHelperRoute => "remote_helper_route",
            Self::EnterpriseNetworkRoute => "enterprise_network_route",
            Self::RouteDrift => "route_drift",
            Self::BlockedTarget => "blocked_target",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed reconnect-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `reconnect_admission` row for each
/// structured reconnect state so the surface explains reconnect,
/// restore, and rerun posture without support-only knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconnectStateClass {
    /// Reattach / reconnect is required before the surface may dispatch.
    ReconnectRequired,
    /// Reconnect is honest — no silent rerun or drifted-target reuse.
    ReconnectHonest,
    /// Restore brought metadata back without rerunning tasks,
    /// reattaching debuggers, or reusing a drifted target.
    RestoreNoRerun,
    /// The row is not bound to a reconnect state (non-reconnect-admission row classes).
    NotApplicable,
}

impl ReconnectStateClass {
    /// Every certified reconnect state in declaration order. A
    /// `launch_stable` lane MUST cover every state.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::ReconnectRequired,
        Self::ReconnectHonest,
        Self::RestoreNoRerun,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconnectRequired => "reconnect_required",
            Self::ReconnectHonest => "reconnect_honest",
            Self::RestoreNoRerun => "restore_no_rerun",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed degraded-helper-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `degraded_helper_admission` row
/// for each structured degraded-helper state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedHelperStateClass {
    /// Helper / remote agent reports degraded capabilities.
    CapabilityDegraded,
    /// Helper / remote agent is offline or unreachable.
    HelperOffline,
    /// Helper / remote agent reports mixed-version skew.
    HelperSkew,
    /// The row is not bound to a degraded-helper state (non-degraded-helper-admission row classes).
    NotApplicable,
}

impl DegradedHelperStateClass {
    /// Every certified degraded-helper state in declaration order. A
    /// `launch_stable` lane MUST cover every state.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::CapabilityDegraded,
        Self::HelperOffline,
        Self::HelperSkew,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityDegraded => "capability_degraded",
            Self::HelperOffline => "helper_offline",
            Self::HelperSkew => "helper_skew",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed artifact-provenance-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `artifact_provenance_admission`
/// row for each structured provenance state so output lineage
/// survives after the live run surface is gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactProvenanceStateClass {
    /// Artifact provenance is tracked and exportable.
    ProvenanceTracked,
    /// Artifact provenance is missing or incomplete.
    ProvenanceMissing,
    /// The row is not bound to an artifact-provenance state (non-artifact-provenance-admission row classes).
    NotApplicable,
}

impl ArtifactProvenanceStateClass {
    /// Every certified artifact-provenance state in declaration order. A
    /// `launch_stable` lane MUST cover every state.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 2] =
        [Self::ProvenanceTracked, Self::ProvenanceMissing];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenanceTracked => "provenance_tracked",
            Self::ProvenanceMissing => "provenance_missing",
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
    /// The row is backed by a docs/help disclosure (gap label only).
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

/// Closed known-limit vocabulary attached to an execution-plane row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the enterprise-network subset.
    EnterpriseNetworkSubsetOnly,
    /// The lane only certifies a subset of the required route states.
    RouteAdmissionSubsetOnly,
    /// The lane only certifies a subset of the required reconnect states.
    ReconnectAdmissionSubsetOnly,
    /// The lane only certifies a subset of the required degraded-helper states.
    DegradedHelperSubsetOnly,
    /// The lane only certifies a subset of the required artifact-provenance states.
    ArtifactProvenanceSubsetOnly,
    /// The lane only certifies a subset of the required surface bindings.
    SurfaceBindingSubsetOnly,
    /// The lane certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::LocalLaneSubsetOnly => "local_lane_subset_only",
            Self::RemoteHelperSubsetOnly => "remote_helper_subset_only",
            Self::EnterpriseNetworkSubsetOnly => "enterprise_network_subset_only",
            Self::RouteAdmissionSubsetOnly => "route_admission_subset_only",
            Self::ReconnectAdmissionSubsetOnly => "reconnect_admission_subset_only",
            Self::DegradedHelperSubsetOnly => "degraded_helper_subset_only",
            Self::ArtifactProvenanceSubsetOnly => "artifact_provenance_subset_only",
            Self::SurfaceBindingSubsetOnly => "surface_binding_subset_only",
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

/// Closed downgrade-automation vocabulary attached to an execution-plane row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when the resolved target is unreachable.
    AutoNarrowOnTargetUnreachable,
    /// Automatically narrow when helper capability skew is detected.
    AutoNarrowOnHelperSkew,
    /// Automatically narrow when the route/tunnel posture drifts.
    AutoNarrowOnRouteDrift,
    /// Automatically narrow when the lineage object breaks (no
    /// `execution_context_id` binding survives across event streams,
    /// support packets, approval tickets, or evidence exports).
    AutoNarrowOnLineageBreak,
    /// Automatically narrow when the target is blocked.
    AutoNarrowOnBlockedTarget,
    /// Automatically narrow when any reconnect state is unbound.
    AutoNarrowOnReconnectGap,
    /// Automatically narrow when restore would silently rerun a task,
    /// reattach a debugger, or reuse a drifted target.
    AutoNarrowOnSilentRerun,
    /// Automatically narrow when degraded helper state is unbound.
    AutoNarrowOnDegradedHelper,
    /// Automatically narrow when artifact provenance state is unbound.
    AutoNarrowOnArtifactProvenanceGap,
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
            Self::AutoNarrowOnTargetUnreachable => "auto_narrow_on_target_unreachable",
            Self::AutoNarrowOnHelperSkew => "auto_narrow_on_helper_skew",
            Self::AutoNarrowOnRouteDrift => "auto_narrow_on_route_drift",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoNarrowOnBlockedTarget => "auto_narrow_on_blocked_target",
            Self::AutoNarrowOnReconnectGap => "auto_narrow_on_reconnect_gap",
            Self::AutoNarrowOnSilentRerun => "auto_narrow_on_silent_rerun",
            Self::AutoNarrowOnDegradedHelper => "auto_narrow_on_degraded_helper",
            Self::AutoNarrowOnArtifactProvenanceGap => "auto_narrow_on_artifact_provenance_gap",
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

/// Closed confidence-class vocabulary for an execution-plane row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl ConfidenceClass {
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
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
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

/// Closed validation-finding vocabulary for the execution-plane packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required execution lane has no row.
    MissingExecutionLaneCoverage,
    /// A lane claiming launch_stable is missing a required surface binding.
    MissingSurfaceBindingCoverage,
    /// A lane claiming launch_stable is missing a required target admission.
    MissingTargetAdmission,
    /// A lane claiming launch_stable is missing a required route admission.
    MissingRouteAdmission,
    /// A lane claiming launch_stable is missing the required restore-rerun honesty row.
    MissingRestoreRerunHonesty,
    /// A lane claiming launch_stable is missing a required reconnect admission.
    MissingReconnectAdmission,
    /// A lane claiming launch_stable is missing a required degraded-helper admission.
    MissingDegradedHelperAdmission,
    /// A lane claiming launch_stable is missing a required artifact-provenance admission.
    MissingArtifactProvenanceAdmission,
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
    /// A surface-binding row drops its surface binding.
    SurfaceBindingNotApplicable,
    /// A non-surface-binding row binds a surface it cannot certify.
    SurfaceBindingNotPermittedOnRowClass,
    /// A route-admission row drops its route state binding.
    RouteStateNotApplicable,
    /// A non-route-admission row binds a route state it cannot certify.
    RouteStateNotPermittedOnRowClass,
    /// A reconnect-admission row drops its reconnect state binding.
    ReconnectStateNotApplicable,
    /// A non-reconnect-admission row binds a reconnect state it cannot certify.
    ReconnectStateNotPermittedOnRowClass,
    /// A degraded-helper-admission row drops its degraded-helper state binding.
    DegradedHelperStateNotApplicable,
    /// A non-degraded-helper-admission row binds a degraded-helper state it cannot certify.
    DegradedHelperStateNotPermittedOnRowClass,
    /// An artifact-provenance-admission row drops its artifact-provenance state binding.
    ArtifactProvenanceStateNotApplicable,
    /// A non-artifact-provenance-admission row binds an artifact-provenance state it cannot certify.
    ArtifactProvenanceStateNotPermittedOnRowClass,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A restore-rerun-honesty row admits silent rerun, silent
    /// reattach, or drifted-target reuse.
    RestoreRerunHonestyAdmitsSilentRerun,
    /// A row admits raw command lines, process environment bytes, or
    /// other private material past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops execution-plane truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the surface-binding vocabulary.
    SurfaceBindingVocabularyCollapsed,
    /// A projection collapses the route-state vocabulary.
    RouteStateVocabularyCollapsed,
    /// A projection collapses the reconnect-state vocabulary.
    ReconnectStateVocabularyCollapsed,
    /// A projection collapses the degraded-helper-state vocabulary.
    DegradedHelperStateVocabularyCollapsed,
    /// A projection collapses the artifact-provenance-state vocabulary.
    ArtifactProvenanceStateVocabularyCollapsed,
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
            Self::MissingExecutionLaneCoverage => "missing_execution_lane_coverage",
            Self::MissingSurfaceBindingCoverage => "missing_surface_binding_coverage",
            Self::MissingTargetAdmission => "missing_target_admission",
            Self::MissingRouteAdmission => "missing_route_admission",
            Self::MissingRestoreRerunHonesty => "missing_restore_rerun_honesty",
            Self::MissingReconnectAdmission => "missing_reconnect_admission",
            Self::MissingDegradedHelperAdmission => "missing_degraded_helper_admission",
            Self::MissingArtifactProvenanceAdmission => "missing_artifact_provenance_admission",
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
            Self::SurfaceBindingNotApplicable => "surface_binding_not_applicable",
            Self::SurfaceBindingNotPermittedOnRowClass => {
                "surface_binding_not_permitted_on_row_class"
            }
            Self::RouteStateNotApplicable => "route_state_not_applicable",
            Self::RouteStateNotPermittedOnRowClass => "route_state_not_permitted_on_row_class",
            Self::ReconnectStateNotApplicable => "reconnect_state_not_applicable",
            Self::ReconnectStateNotPermittedOnRowClass => {
                "reconnect_state_not_permitted_on_row_class"
            }
            Self::DegradedHelperStateNotApplicable => "degraded_helper_state_not_applicable",
            Self::DegradedHelperStateNotPermittedOnRowClass => {
                "degraded_helper_state_not_permitted_on_row_class"
            }
            Self::ArtifactProvenanceStateNotApplicable => {
                "artifact_provenance_state_not_applicable"
            }
            Self::ArtifactProvenanceStateNotPermittedOnRowClass => {
                "artifact_provenance_state_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::RestoreRerunHonestyAdmitsSilentRerun => {
                "restore_rerun_honesty_admits_silent_rerun"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::SurfaceBindingVocabularyCollapsed => "surface_binding_vocabulary_collapsed",
            Self::RouteStateVocabularyCollapsed => "route_state_vocabulary_collapsed",
            Self::ReconnectStateVocabularyCollapsed => "reconnect_state_vocabulary_collapsed",
            Self::DegradedHelperStateVocabularyCollapsed => {
                "degraded_helper_state_vocabulary_collapsed"
            }
            Self::ArtifactProvenanceStateVocabularyCollapsed => {
                "artifact_provenance_state_vocabulary_collapsed"
            }
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the execution-plane packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
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

impl ConsumerSurface {
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

/// One execution-plane truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPlaneCertificationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Execution-plane lane this row certifies.
    pub lane_class: ExecutionPlaneLaneClass,
    /// Execution-plane row class.
    pub row_class: ExecutionPlaneRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Surface binding certified by the row (or `not_applicable`).
    pub surface_binding_class: SurfaceBindingClass,
    /// Route state admitted by the row (or `not_applicable`).
    pub route_state_class: RouteStateClass,
    /// Reconnect state admitted by the row (or `not_applicable`).
    pub reconnect_state_class: ReconnectStateClass,
    /// Degraded-helper state admitted by the row (or `not_applicable`).
    pub degraded_helper_state_class: DegradedHelperStateClass,
    /// Artifact-provenance state admitted by the row (or `not_applicable`).
    pub artifact_provenance_state_class: ArtifactProvenanceStateClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit,
    /// or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For restore_rerun_honesty rows, true when the row attests that
    /// restore brought metadata back without silently rerunning tasks,
    /// reattaching debuggers, or reusing a drifted target.
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

impl ExecutionPlaneCertificationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPlaneConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Execution-plane packet id consumed by the projection.
    pub execution_plane_packet_id_ref: String,
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
    /// True when the surface-binding vocabulary is preserved verbatim.
    pub preserves_surface_binding_vocabulary: bool,
    /// True when the route-state vocabulary is preserved verbatim.
    pub preserves_route_state_vocabulary: bool,
    /// True when the reconnect-state vocabulary is preserved verbatim.
    pub preserves_reconnect_state_vocabulary: bool,
    /// True when the degraded-helper-state vocabulary is preserved verbatim.
    pub preserves_degraded_helper_state_vocabulary: bool,
    /// True when the artifact-provenance-state vocabulary is preserved verbatim.
    pub preserves_artifact_provenance_state_vocabulary: bool,
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

impl ExecutionPlaneConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.execution_plane_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_surface_binding_vocabulary
            && self.preserves_route_state_vocabulary
            && self.preserves_reconnect_state_vocabulary
            && self.preserves_degraded_helper_state_vocabulary
            && self.preserves_artifact_provenance_state_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`ExecutionPlaneTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPlaneTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Execution-plane lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ExecutionPlaneLaneClass>,
    /// Execution-plane rows.
    #[serde(default)]
    pub rows: Vec<ExecutionPlaneCertificationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ExecutionPlaneConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying local, remote/helper, and
/// enterprise-network execution-plane resolution at the M4 launch-stable
/// grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPlaneTruthPacket {
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
    /// Execution-plane lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ExecutionPlaneLaneClass>,
    /// Execution-plane rows.
    #[serde(default)]
    pub rows: Vec<ExecutionPlaneCertificationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ExecutionPlaneConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl ExecutionPlaneTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: ExecutionPlaneTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: EXECUTION_PLANE_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_PLANE_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable execution-plane invariants.
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
            .map(ExecutionPlaneLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(ExecutionPlaneRowClass::as_str)
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

    /// Returns the unique surface-binding tokens observed across rows.
    pub fn surface_binding_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.surface_binding_class);
        }
        set.into_iter().map(SurfaceBindingClass::as_str).collect()
    }

    /// Returns the unique route-state tokens observed across rows.
    pub fn route_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.route_state_class);
        }
        set.into_iter().map(RouteStateClass::as_str).collect()
    }

    /// Returns the unique reconnect-state tokens observed across rows.
    pub fn reconnect_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.reconnect_state_class);
        }
        set.into_iter().map(ReconnectStateClass::as_str).collect()
    }

    /// Returns the unique degraded-helper-state tokens observed across rows.
    pub fn degraded_helper_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.degraded_helper_state_class);
        }
        set.into_iter()
            .map(DegradedHelperStateClass::as_str)
            .collect()
    }

    /// Returns the unique artifact-provenance-state tokens observed across rows.
    pub fn artifact_provenance_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.artifact_provenance_state_class);
        }
        set.into_iter()
            .map(ArtifactProvenanceStateClass::as_str)
            .collect()
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

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> ExecutionPlaneTruthSupportExport {
        ExecutionPlaneTruthSupportExport {
            record_kind: EXECUTION_PLANE_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_PLANE_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            execution_plane_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            execution_plane_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != EXECUTION_PLANE_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "execution-plane packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != EXECUTION_PLANE_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "execution-plane packet has the wrong schema version",
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
                FindingKind::MissingExecutionLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered execution-plane lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingExecutionLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers execution-plane lane {}", lane.as_str()),
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
                        "row {} admits raw command lines, raw env bytes, or raw capsule bodies past the boundary",
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

            if row.row_class.requires_surface_binding()
                && matches!(
                    row.surface_binding_class,
                    SurfaceBindingClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SurfaceBindingNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_surface_binding()
                && !matches!(
                    row.surface_binding_class,
                    SurfaceBindingClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SurfaceBindingNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds surface {}; only surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.surface_binding_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_route_state()
                && matches!(row.route_state_class, RouteStateClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RouteStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a route_admission but has no bound route state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_route_state()
                && !matches!(row.route_state_class, RouteStateClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RouteStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds route state {}; only route_admission rows may bind a route state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.route_state_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_reconnect_state()
                && matches!(
                    row.reconnect_state_class,
                    ReconnectStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReconnectStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a reconnect_admission but has no bound reconnect state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_reconnect_state()
                && !matches!(
                    row.reconnect_state_class,
                    ReconnectStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReconnectStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds reconnect state {}; only reconnect_admission rows may bind a reconnect state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.reconnect_state_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_degraded_helper_state()
                && matches!(
                    row.degraded_helper_state_class,
                    DegradedHelperStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DegradedHelperStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a degraded_helper_admission but has no bound degraded-helper state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_degraded_helper_state()
                && !matches!(
                    row.degraded_helper_state_class,
                    DegradedHelperStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DegradedHelperStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds degraded-helper state {}; only degraded_helper_admission rows may bind a degraded-helper state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.degraded_helper_state_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_artifact_provenance_state()
                && matches!(
                    row.artifact_provenance_state_class,
                    ArtifactProvenanceStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ArtifactProvenanceStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an artifact_provenance_admission but has no bound artifact-provenance state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_artifact_provenance_state()
                && !matches!(
                    row.artifact_provenance_state_class,
                    ArtifactProvenanceStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ArtifactProvenanceStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds artifact-provenance state {}; only artifact_provenance_admission rows may bind an artifact-provenance state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.artifact_provenance_state_class.as_str()
                    ),
                ));
            }

            if matches!(row.row_class, ExecutionPlaneRowClass::LineageAdmission)
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

            if matches!(row.row_class, ExecutionPlaneRowClass::RestoreRerunHonesty)
                && !row.restore_preserves_no_rerun
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RestoreRerunHonestyAdmitsSilentRerun,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a restore_rerun_honesty row but does not attest no-silent-rerun",
                        row.row_id
                    ),
                ));
            }

            if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
                && matches!(row.support_class, SupportClass::LaunchStable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Warning,
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
                        ExecutionPlaneRowClass::ExecutionPlaneCertificationQuality
                    )
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for surface in SurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, ExecutionPlaneRowClass::SurfaceBinding)
                        && row.surface_binding_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingSurfaceBindingCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_target_admission = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, ExecutionPlaneRowClass::TargetAdmission)
            });
            if !has_target_admission {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingTargetAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no target_admission row",
                        lane.as_str()
                    ),
                ));
            }

            for state in RouteStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, ExecutionPlaneRowClass::RouteAdmission)
                        && row.route_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingRouteAdmission,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no route_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            let has_restore_rerun = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, ExecutionPlaneRowClass::RestoreRerunHonesty)
                    && row.restore_preserves_no_rerun
            });
            if !has_restore_rerun {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRestoreRerunHonesty,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no restore_rerun_honesty row attesting no-silent-rerun",
                        lane.as_str()
                    ),
                ));
            }

            for state in ReconnectStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, ExecutionPlaneRowClass::ReconnectAdmission)
                        && row.reconnect_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingReconnectAdmission,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no reconnect_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for state in DegradedHelperStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ExecutionPlaneRowClass::DegradedHelperAdmission
                        )
                        && row.degraded_helper_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingDegradedHelperAdmission,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no degraded_helper_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for state in ArtifactProvenanceStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            ExecutionPlaneRowClass::ArtifactProvenanceAdmission
                        )
                        && row.artifact_provenance_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingArtifactProvenanceAdmission,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no artifact_provenance_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, ExecutionPlaneRowClass::LineageAdmission)
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_lineage {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLineageAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no lineage_admission row binding execution_context_id",
                        lane.as_str()
                    ),
                ));
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
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
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve execution-plane truth",
                        projection.projection_ref
                    ),
                ));
            }
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
            if !projection.preserves_surface_binding_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SurfaceBindingVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the surface-binding vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_route_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RouteStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the route-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_reconnect_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ReconnectStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the reconnect-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_degraded_helper_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DegradedHelperStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the degraded-helper-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_artifact_provenance_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ArtifactProvenanceStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the artifact-provenance-state vocabulary",
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

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPlaneTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub execution_plane_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub execution_plane_packet: ExecutionPlaneTruthPacket,
}

impl ExecutionPlaneTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == EXECUTION_PLANE_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == EXECUTION_PLANE_TRUTH_SCHEMA_VERSION
            && self.execution_plane_packet_id_ref == self.execution_plane_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.execution_plane_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable execution-plane packet.
#[derive(Debug)]
pub enum ExecutionPlaneTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for ExecutionPlaneTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "execution-plane packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "execution-plane packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ExecutionPlaneTruthArtifactError {}

/// Returns the checked-in stable execution-plane truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_execution_plane_truth_packet(
) -> Result<ExecutionPlaneTruthPacket, ExecutionPlaneTruthArtifactError> {
    let packet: ExecutionPlaneTruthPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/runtime/m4/publish_execution_plane_certification_packets_for_local_remote_truth_packet.json"
        )))
        .map_err(ExecutionPlaneTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(ExecutionPlaneTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        EXECUTION_PLANE_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        EXECUTION_PLANE_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: ExecutionPlaneLaneClass) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::ExecutionPlaneCertificationQuality,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn surface_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
        surface: SurfaceBindingClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::SurfaceBinding,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: surface,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn route_state_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
        state: RouteStateClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:route:{}", state.as_str()),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::RouteAdmission,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: state,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnRouteDrift,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_route_drift", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn target_admission_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:target_admission"),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::TargetAdmission,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnTargetUnreachable,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_target_unreachable", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn restore_rerun_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:restore_rerun_honesty"),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::RestoreRerunHonesty,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnSilentRerun,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_silent_rerun", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: true,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn reconnect_state_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
        state: ReconnectStateClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:reconnect:{}", state.as_str()),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::ReconnectAdmission,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: state,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnReconnectGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_reconnect_gap", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn degraded_helper_state_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
        state: DegradedHelperStateClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:degraded_helper:{}", state.as_str()),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::DegradedHelperAdmission,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: state,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnDegradedHelper,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_degraded_helper", doc_ref())),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn artifact_provenance_state_row(
        prefix: &str,
        lane: ExecutionPlaneLaneClass,
        state: ArtifactProvenanceStateClass,
    ) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:artifact_provenance:{}", state.as_str()),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::ArtifactProvenanceAdmission,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: state,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnArtifactProvenanceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_artifact_provenance_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: ExecutionPlaneLaneClass) -> ExecutionPlaneCertificationRow {
        ExecutionPlaneCertificationRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: ExecutionPlaneRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            surface_binding_class: SurfaceBindingClass::NotApplicable,
            route_state_class: RouteStateClass::NotApplicable,
            reconnect_state_class: ReconnectStateClass::NotApplicable,
            degraded_helper_state_class: DegradedHelperStateClass::NotApplicable,
            artifact_provenance_state_class: ArtifactProvenanceStateClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:lineage")),
            restore_preserves_no_rerun: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> ExecutionPlaneConsumerProjection {
        ExecutionPlaneConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            execution_plane_packet_id_ref: "packet:m4:publish_execution_plane_certification"
                .to_owned(),
            rendered_at: "2026-05-27T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_surface_binding_vocabulary: true,
            preserves_route_state_vocabulary: true,
            preserves_reconnect_state_vocabulary: true,
            preserves_degraded_helper_state_vocabulary: true,
            preserves_artifact_provenance_state_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: ExecutionPlaneLaneClass,
        prefix: &str,
    ) -> Vec<ExecutionPlaneCertificationRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for surface in SurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(surface_row(prefix, lane, surface));
        }
        out.push(target_admission_row(prefix, lane));
        for state in RouteStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(route_state_row(prefix, lane, state));
        }
        out.push(restore_rerun_row(prefix, lane));
        for state in ReconnectStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(reconnect_state_row(prefix, lane, state));
        }
        for state in DegradedHelperStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(degraded_helper_state_row(prefix, lane, state));
        }
        for state in ArtifactProvenanceStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(artifact_provenance_state_row(prefix, lane, state));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> ExecutionPlaneTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(ExecutionPlaneLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            ExecutionPlaneLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(
            ExecutionPlaneLaneClass::EnterpriseNetworkLane,
            "enterprise",
        ));
        ExecutionPlaneTruthPacketInput {
            packet_id: "packet:m4:publish_execution_plane_certification".to_owned(),
            workflow_or_surface_id: "workflow.runtime.publish_execution_plane_certification"
                .to_owned(),
            generated_at: "2026-05-27T12:00:00Z".to_owned(),
            covered_lanes: ExecutionPlaneLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(ExecutionPlaneLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(
            ExecutionPlaneLaneClass::RemoteHelperLane.as_str(),
            "remote_helper_lane"
        );
        assert_eq!(
            ExecutionPlaneLaneClass::EnterpriseNetworkLane.as_str(),
            "enterprise_network_lane"
        );
        assert_eq!(
            ExecutionPlaneRowClass::ExecutionPlaneCertificationQuality.as_str(),
            "execution_plane_certification_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(
            SupportClass::LaunchStableBelow.as_str(),
            "launch_stable_below"
        );
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(SurfaceBindingClass::Terminal.as_str(), "terminal");
        assert_eq!(
            SurfaceBindingClass::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(RouteStateClass::LocalRoute.as_str(), "local_route");
        assert_eq!(RouteStateClass::BlockedTarget.as_str(), "blocked_target");
        assert_eq!(
            ReconnectStateClass::ReconnectRequired.as_str(),
            "reconnect_required"
        );
        assert_eq!(
            DegradedHelperStateClass::CapabilityDegraded.as_str(),
            "capability_degraded"
        );
        assert_eq!(
            ArtifactProvenanceStateClass::ProvenanceTracked.as_str(),
            "provenance_tracked"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
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
    fn sample_input_materializes_stable() {
        let packet = ExecutionPlaneTruthPacket::materialize(sample_input());
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validate().is_empty());
        assert!(packet.is_stable());
        for required in ExecutionPlaneLaneClass::REQUIRED {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required),
                "stable packet must include row for execution-plane lane {}",
                required.as_str()
            );
        }
        for surface in ConsumerSurface::REQUIRED {
            assert!(
                packet.has_projection_for(surface),
                "stable packet must preserve {} consumer projection",
                surface.as_str()
            );
        }
    }

    #[test]
    fn support_export_is_safe_for_stable_packet() {
        let packet = ExecutionPlaneTruthPacket::materialize(sample_input());
        let export = packet.support_export("support-export:test", "2026-05-27T12:00:10Z");
        assert!(export.is_export_safe());
    }

    #[test]
    fn checked_in_artifact_loads_and_validates() {
        let packet = current_stable_execution_plane_truth_packet();
        assert!(
            packet.is_ok(),
            "checked-in artifact must parse and validate: {:?}",
            packet.err()
        );
    }
}
