//! Debugger host + adapter negotiation + attach/launch + crash-isolation
//! stabilization truth packet for the M4 stable lane.
//!
//! This module pins how local, remote/helper, container, and notebook-bridge
//! debug sessions serialize one canonical debugger truth across the four
//! debugger wedges (`debugger_host`, `adapter_negotiation`,
//! `attach_launch_flow`, `crash_isolation`). Debug adapter and backend
//! descriptors MUST serialize launch/attach scope, local-versus-remote
//! support class, chronology/replay capability class, and any
//! notebook-bridge or replay-only limitation instead of letting support be
//! inferred from button presence. Attach/launch negotiation MUST degrade
//! explicitly (`supported`, `limited`, `view_only`, `unsupported`, or
//! `policy_blocked`) per runtime/backend row, and those labels MUST flow to
//! UI, CLI/headless, support export, and docs/help without drift.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `debugger_stabilization_quality` row cannot prove:
//!
//! - the four debugger wedges (`debugger_host`, `adapter_negotiation`,
//!   `attach_launch_flow`, `crash_isolation`) each have a structured
//!   wedge_admission row,
//! - adapter/backend descriptors bind the six descriptor fields
//!   (`adapter_identity`, `transport_class`, `launch_attach_scope`,
//!   `local_vs_remote_support_class`,
//!   `chronology_replay_capability_class`,
//!   `notebook_bridge_or_replay_only_limitation`) so reviewers do not
//!   infer support from button presence,
//! - the four attach/launch parity surfaces (`ui_surface`,
//!   `cli_headless`, `support_export`, `docs_help`) carry parity-surface
//!   binding rows that read the same attach/launch posture
//!   (`supported`, `limited`, `view_only`, `unsupported`, or
//!   `policy_blocked`) verbatim,
//! - the five crash-isolation assertions (`bounded_restart_budget`,
//!   `session_quarantine_admission`,
//!   `unrelated_language_host_unaffected`,
//!   `unrelated_terminal_lane_unaffected`,
//!   `unrelated_debug_session_unaffected`) each carry a structured
//!   assertion-binding row attesting the isolation invariant,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through every emitted debug-session envelope and downstream
//!   consumer surface.
//!
//! Every row binds a closed `debugger_stabilization_lane_class`,
//! `debugger_stabilization_row_class`, `support_class`, `wedge_class`,
//! `adapter_descriptor_field_class`, `attach_launch_parity_surface_class`,
//! `attach_launch_posture_class`, `crash_isolation_assertion_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `debugger_stabilization_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw debugger payloads,
//! raw stack frames, raw memory bytes, raw command lines, raw process
//! environment bytes, raw scrollback bodies, raw secrets, or ambient
//! credentials past the boundary. A row that claims `launch_stable`
//! while leaving its known limit, downgrade automation, or evidence
//! class unbound is refused; the validator narrows below launch-stable
//! instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`DebuggerStabilizationTruthPacket`].
pub const DEBUGGER_STABILIZATION_TRUTH_PACKET_RECORD_KIND: &str =
    "stabilize_debugger_host_and_adapter_negotiation_truth_stable_packet";

/// Stable record-kind tag for [`DebuggerStabilizationTruthSupportExport`].
pub const DEBUGGER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stabilize_debugger_host_and_adapter_negotiation_truth_support_export";

/// Integer schema version for the debugger-stabilization truth packet.
pub const DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DEBUGGER_STABILIZATION_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const DEBUGGER_STABILIZATION_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const DEBUGGER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation";

/// Repo-relative path of the checked-in stable packet.
pub const DEBUGGER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/stabilize_debugger_host_and_adapter_negotiation_truth_packet.json";

/// Closed debugger-stabilization lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebuggerStabilizationLaneClass {
    /// Local-host debug session lane.
    LocalLane,
    /// Remote / helper attach debug session lane.
    RemoteHelperLane,
    /// Container-attached debug session lane.
    ContainerLane,
    /// Notebook-bridge debug session lane (kernel debugger bridge).
    NotebookBridgeLane,
}

impl DebuggerStabilizationLaneClass {
    /// Every required debugger-stabilization lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::ContainerLane,
        Self::NotebookBridgeLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::ContainerLane => "container_lane",
            Self::NotebookBridgeLane => "notebook_bridge_lane",
        }
    }
}

/// Closed debugger-stabilization row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebuggerStabilizationRowClass {
    /// The lane's headline debugger-stabilization qualification row.
    DebuggerStabilizationQuality,
    /// A row admitting one of the four debugger wedges (debugger_host,
    /// adapter_negotiation, attach_launch_flow, crash_isolation).
    WedgeAdmission,
    /// A row binding one adapter/backend descriptor field
    /// (`adapter_identity`, `transport_class`, `launch_attach_scope`,
    /// `local_vs_remote_support_class`,
    /// `chronology_replay_capability_class`,
    /// `notebook_bridge_or_replay_only_limitation`).
    AdapterDescriptorFieldBinding,
    /// A row binding one attach/launch parity surface (`ui_surface`,
    /// `cli_headless`, `support_export`, `docs_help`) plus the
    /// propagated `attach_launch_posture_class` the surface carries
    /// without drift.
    AttachLaunchParitySurfaceBinding,
    /// A row attesting one crash-isolation assertion
    /// (`bounded_restart_budget`, `session_quarantine_admission`,
    /// `unrelated_language_host_unaffected`,
    /// `unrelated_terminal_lane_unaffected`,
    /// `unrelated_debug_session_unaffected`).
    CrashIsolationAssertionBinding,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted debug-session truth and downstream
    /// consumer surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl DebuggerStabilizationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebuggerStabilizationQuality => "debugger_stabilization_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::AdapterDescriptorFieldBinding => "adapter_descriptor_field_binding",
            Self::AttachLaunchParitySurfaceBinding => "attach_launch_parity_surface_binding",
            Self::CrashIsolationAssertionBinding => "crash_isolation_assertion_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound adapter descriptor field.
    pub const fn requires_adapter_descriptor_field(self) -> bool {
        matches!(self, Self::AdapterDescriptorFieldBinding)
    }

    /// True when this row class requires a bound attach/launch parity surface.
    pub const fn requires_attach_launch_parity_surface(self) -> bool {
        matches!(self, Self::AttachLaunchParitySurfaceBinding)
    }

    /// True when this row class requires a bound attach/launch posture.
    pub const fn requires_attach_launch_posture(self) -> bool {
        matches!(self, Self::AttachLaunchParitySurfaceBinding)
    }

    /// True when this row class requires a bound crash-isolation assertion.
    pub const fn requires_crash_isolation_assertion(self) -> bool {
        matches!(self, Self::CrashIsolationAssertionBinding)
    }
}

/// Closed support-class vocabulary applied to a debugger-stabilization row.
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

/// Closed debugger-wedge vocabulary. Every lane claiming `launch_stable`
/// MUST publish a `wedge_admission` row for each required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Debugger-host wedge (DAP-style supervisor lifecycle).
    DebuggerHost,
    /// Adapter-negotiation wedge (capability/transport negotiation).
    AdapterNegotiation,
    /// Attach/launch flow wedge (launch and attach lifecycle).
    AttachLaunchFlow,
    /// Crash-isolation wedge (bounded restart budget, session quarantine).
    CrashIsolation,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::DebuggerHost,
        Self::AdapterNegotiation,
        Self::AttachLaunchFlow,
        Self::CrashIsolation,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebuggerHost => "debugger_host",
            Self::AdapterNegotiation => "adapter_negotiation",
            Self::AttachLaunchFlow => "attach_launch_flow",
            Self::CrashIsolation => "crash_isolation",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed adapter/backend descriptor-field vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `adapter_descriptor_field_binding` row
/// for each required field so reviewers cannot infer support from button
/// presence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterDescriptorFieldClass {
    /// Adapter identity (adapter id, version, vendor).
    AdapterIdentity,
    /// Adapter transport class (stdio, socket, named pipe, websocket).
    TransportClass,
    /// Launch/attach scope (launch-only, attach-only, both,
    /// launch-with-attach-fallback).
    LaunchAttachScope,
    /// Local-versus-remote support class (local-only, remote-only,
    /// both, helper-only).
    LocalVsRemoteSupportClass,
    /// Chronology / replay capability class (live-only,
    /// chronology-supported, replay-only,
    /// chronology-and-replay-supported).
    ChronologyReplayCapabilityClass,
    /// Notebook-bridge or replay-only limitation (none, notebook-bridge,
    /// replay-only, notebook-bridge-and-replay-only).
    NotebookBridgeOrReplayOnlyLimitation,
    /// The row is not bound to a descriptor field.
    NotApplicable,
}

impl AdapterDescriptorFieldClass {
    /// Every required adapter descriptor field per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::AdapterIdentity,
        Self::TransportClass,
        Self::LaunchAttachScope,
        Self::LocalVsRemoteSupportClass,
        Self::ChronologyReplayCapabilityClass,
        Self::NotebookBridgeOrReplayOnlyLimitation,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterIdentity => "adapter_identity",
            Self::TransportClass => "transport_class",
            Self::LaunchAttachScope => "launch_attach_scope",
            Self::LocalVsRemoteSupportClass => "local_vs_remote_support_class",
            Self::ChronologyReplayCapabilityClass => "chronology_replay_capability_class",
            Self::NotebookBridgeOrReplayOnlyLimitation => {
                "notebook_bridge_or_replay_only_limitation"
            }
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed attach/launch parity-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `attach_launch_parity_surface_binding`
/// row for each parity surface so the same posture label flows to UI,
/// CLI/headless, support export, and docs/help without drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachLaunchParitySurfaceClass {
    /// UI surface (debug panel, run-and-debug chrome).
    UiSurface,
    /// CLI / headless surface (`aureline debug ...`).
    CliHeadless,
    /// Support export surface.
    SupportExport,
    /// Docs / Help surface (docs site, in-product Help/About).
    DocsHelp,
    /// The row is not bound to an attach/launch parity surface.
    NotApplicable,
}

impl AttachLaunchParitySurfaceClass {
    /// Every required parity surface per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::UiSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::DocsHelp,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiSurface => "ui_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::DocsHelp => "docs_help",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed attach/launch posture vocabulary. Every
/// `attach_launch_parity_surface_binding` row MUST bind a non-`not_applicable`
/// posture; all four parity surfaces for a lane MUST agree on the same
/// posture (no drift).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachLaunchPostureClass {
    /// Attach/launch fully supported on the lane.
    Supported,
    /// Attach/launch supported with disclosed gaps.
    Limited,
    /// Attach/launch view-only (e.g. inspection without resume/step).
    ViewOnly,
    /// Attach/launch unsupported on the lane.
    Unsupported,
    /// Attach/launch blocked by policy.
    PolicyBlocked,
    /// The row is not bound to an attach/launch posture.
    NotApplicable,
}

impl AttachLaunchPostureClass {
    /// Every closed attach/launch posture label.
    pub const CLOSED_VOCABULARY: [Self; 5] = [
        Self::Supported,
        Self::Limited,
        Self::ViewOnly,
        Self::Unsupported,
        Self::PolicyBlocked,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::ViewOnly => "view_only",
            Self::Unsupported => "unsupported",
            Self::PolicyBlocked => "policy_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the posture requires an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::Supported | Self::NotApplicable)
    }
}

/// Closed crash-isolation assertion vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `crash_isolation_assertion_binding` row
/// for each required assertion so adapter crashes, protocol violations, or
/// hangs degrade only the affected session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashIsolationAssertionClass {
    /// Bounded restart budget is applied to the failing session.
    BoundedRestartBudget,
    /// Session quarantine admission on exceeded restart budget.
    SessionQuarantineAdmission,
    /// Unrelated language hosts are unaffected by the session failure.
    UnrelatedLanguageHostUnaffected,
    /// Unrelated terminal lanes are unaffected by the session failure.
    UnrelatedTerminalLaneUnaffected,
    /// Unrelated debug sessions are unaffected by the session failure.
    UnrelatedDebugSessionUnaffected,
    /// The row is not bound to a crash-isolation assertion.
    NotApplicable,
}

impl CrashIsolationAssertionClass {
    /// Every required crash-isolation assertion per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::BoundedRestartBudget,
        Self::SessionQuarantineAdmission,
        Self::UnrelatedLanguageHostUnaffected,
        Self::UnrelatedTerminalLaneUnaffected,
        Self::UnrelatedDebugSessionUnaffected,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedRestartBudget => "bounded_restart_budget",
            Self::SessionQuarantineAdmission => "session_quarantine_admission",
            Self::UnrelatedLanguageHostUnaffected => "unrelated_language_host_unaffected",
            Self::UnrelatedTerminalLaneUnaffected => "unrelated_terminal_lane_unaffected",
            Self::UnrelatedDebugSessionUnaffected => "unrelated_debug_session_unaffected",
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

/// Closed known-limit vocabulary attached to a debugger-stabilization row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the container subset.
    ContainerSubsetOnly,
    /// The lane only certifies the notebook-bridge subset.
    NotebookBridgeSubsetOnly,
    /// The lane only certifies a subset of the four debugger wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the six adapter descriptor fields.
    AdapterDescriptorFieldSubsetOnly,
    /// The lane only certifies a subset of the four parity surfaces.
    AttachLaunchParitySurfaceSubsetOnly,
    /// The lane only certifies a subset of the five crash-isolation assertions.
    CrashIsolationAssertionSubsetOnly,
    /// The lane reports posture drift between parity surfaces.
    AttachLaunchPostureDriftDeclared,
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
            Self::ContainerSubsetOnly => "container_subset_only",
            Self::NotebookBridgeSubsetOnly => "notebook_bridge_subset_only",
            Self::WedgeAdmissionSubsetOnly => "wedge_admission_subset_only",
            Self::AdapterDescriptorFieldSubsetOnly => "adapter_descriptor_field_subset_only",
            Self::AttachLaunchParitySurfaceSubsetOnly => {
                "attach_launch_parity_surface_subset_only"
            }
            Self::CrashIsolationAssertionSubsetOnly => "crash_isolation_assertion_subset_only",
            Self::AttachLaunchPostureDriftDeclared => "attach_launch_posture_drift_declared",
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
    /// Automatically narrow when a required wedge admission is missing.
    AutoNarrowOnWedgeAdmissionGap,
    /// Automatically narrow when a required adapter descriptor field is unbound.
    AutoNarrowOnAdapterDescriptorFieldGap,
    /// Automatically narrow when an attach/launch parity surface binding is
    /// missing.
    AutoNarrowOnAttachLaunchParitySurfaceGap,
    /// Automatically narrow when attach/launch posture drifts between parity
    /// surfaces.
    AutoNarrowOnAttachLaunchPostureDrift,
    /// Automatically narrow when a required crash-isolation assertion is
    /// missing.
    AutoNarrowOnCrashIsolationAssertionGap,
    /// Automatically narrow when the lineage object breaks
    /// (`execution_context_id` does not thread through emitted truth).
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
            Self::AutoNarrowOnWedgeAdmissionGap => "auto_narrow_on_wedge_admission_gap",
            Self::AutoNarrowOnAdapterDescriptorFieldGap => {
                "auto_narrow_on_adapter_descriptor_field_gap"
            }
            Self::AutoNarrowOnAttachLaunchParitySurfaceGap => {
                "auto_narrow_on_attach_launch_parity_surface_gap"
            }
            Self::AutoNarrowOnAttachLaunchPostureDrift => {
                "auto_narrow_on_attach_launch_posture_drift"
            }
            Self::AutoNarrowOnCrashIsolationAssertionGap => {
                "auto_narrow_on_crash_isolation_assertion_gap"
            }
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

/// Closed confidence-class vocabulary for a debugger-stabilization row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebuggerStabilizationConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl DebuggerStabilizationConfidenceClass {
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
    /// A lane claiming launch_stable is missing a required adapter descriptor field.
    MissingAdapterDescriptorFieldCoverage,
    /// A lane claiming launch_stable is missing a required attach/launch parity surface.
    MissingAttachLaunchParitySurfaceCoverage,
    /// A lane claiming launch_stable is missing a required crash-isolation assertion.
    MissingCrashIsolationAssertionCoverage,
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
    /// An adapter-descriptor-field row drops its field binding.
    AdapterDescriptorFieldNotApplicable,
    /// A non-adapter-descriptor-field row binds a field it cannot certify.
    AdapterDescriptorFieldNotPermittedOnRowClass,
    /// An attach/launch parity-surface row drops its surface binding.
    AttachLaunchParitySurfaceNotApplicable,
    /// A non-parity-surface row binds a parity surface it cannot certify.
    AttachLaunchParitySurfaceNotPermittedOnRowClass,
    /// An attach/launch parity-surface row drops its posture binding.
    AttachLaunchPostureNotApplicable,
    /// A non-parity-surface row binds an attach/launch posture it cannot certify.
    AttachLaunchPostureNotPermittedOnRowClass,
    /// An attach/launch posture row narrows below `supported` without a disclosure ref.
    AttachLaunchPostureMissingDisclosureRef,
    /// Attach/launch posture labels drift between parity surfaces on a lane.
    AttachLaunchPostureDrift,
    /// A crash-isolation-assertion row drops its assertion binding.
    CrashIsolationAssertionNotApplicable,
    /// A non-assertion row binds a crash-isolation assertion it cannot certify.
    CrashIsolationAssertionNotPermittedOnRowClass,
    /// A crash-isolation assertion row admits the isolation invariant is not attested.
    CrashIsolationAssertionNotAttested,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A row admits raw debugger payloads, raw stack frames, raw memory bytes,
    /// raw command lines, or raw process environment bytes past the boundary.
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
    /// A projection collapses the adapter-descriptor-field vocabulary.
    AdapterDescriptorFieldVocabularyCollapsed,
    /// A projection collapses the attach/launch parity-surface vocabulary.
    AttachLaunchParitySurfaceVocabularyCollapsed,
    /// A projection collapses the attach/launch posture vocabulary.
    AttachLaunchPostureVocabularyCollapsed,
    /// A projection collapses the crash-isolation assertion vocabulary.
    CrashIsolationAssertionVocabularyCollapsed,
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
            Self::MissingAdapterDescriptorFieldCoverage => {
                "missing_adapter_descriptor_field_coverage"
            }
            Self::MissingAttachLaunchParitySurfaceCoverage => {
                "missing_attach_launch_parity_surface_coverage"
            }
            Self::MissingCrashIsolationAssertionCoverage => {
                "missing_crash_isolation_assertion_coverage"
            }
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
            Self::AdapterDescriptorFieldNotApplicable => "adapter_descriptor_field_not_applicable",
            Self::AdapterDescriptorFieldNotPermittedOnRowClass => {
                "adapter_descriptor_field_not_permitted_on_row_class"
            }
            Self::AttachLaunchParitySurfaceNotApplicable => {
                "attach_launch_parity_surface_not_applicable"
            }
            Self::AttachLaunchParitySurfaceNotPermittedOnRowClass => {
                "attach_launch_parity_surface_not_permitted_on_row_class"
            }
            Self::AttachLaunchPostureNotApplicable => "attach_launch_posture_not_applicable",
            Self::AttachLaunchPostureNotPermittedOnRowClass => {
                "attach_launch_posture_not_permitted_on_row_class"
            }
            Self::AttachLaunchPostureMissingDisclosureRef => {
                "attach_launch_posture_missing_disclosure_ref"
            }
            Self::AttachLaunchPostureDrift => "attach_launch_posture_drift",
            Self::CrashIsolationAssertionNotApplicable => {
                "crash_isolation_assertion_not_applicable"
            }
            Self::CrashIsolationAssertionNotPermittedOnRowClass => {
                "crash_isolation_assertion_not_permitted_on_row_class"
            }
            Self::CrashIsolationAssertionNotAttested => "crash_isolation_assertion_not_attested",
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
            Self::AdapterDescriptorFieldVocabularyCollapsed => {
                "adapter_descriptor_field_vocabulary_collapsed"
            }
            Self::AttachLaunchParitySurfaceVocabularyCollapsed => {
                "attach_launch_parity_surface_vocabulary_collapsed"
            }
            Self::AttachLaunchPostureVocabularyCollapsed => {
                "attach_launch_posture_vocabulary_collapsed"
            }
            Self::CrashIsolationAssertionVocabularyCollapsed => {
                "crash_isolation_assertion_vocabulary_collapsed"
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

/// Consumer surface that must inherit the packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor run / launch / debug surface (per-pane "why this target?" chip).
    EditorDebugSurface,
    /// Debug session panel.
    DebugSessionPanel,
    /// Breakpoint surface.
    BreakpointSurface,
    /// Watch / locals / call-stack surface.
    WatchLocalsSurface,
    /// Crash-loop / quarantine banner surface.
    CrashLoopQuarantineBanner,
    /// CLI / headless inspection surface (`aureline debug ...`).
    CliHeadless,
    /// Evidence export bundle surface.
    EvidenceExport,
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
    pub const REQUIRED: [Self; 11] = [
        Self::EditorDebugSurface,
        Self::DebugSessionPanel,
        Self::BreakpointSurface,
        Self::WatchLocalsSurface,
        Self::CrashLoopQuarantineBanner,
        Self::CliHeadless,
        Self::EvidenceExport,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorDebugSurface => "editor_debug_surface",
            Self::DebugSessionPanel => "debug_session_panel",
            Self::BreakpointSurface => "breakpoint_surface",
            Self::WatchLocalsSurface => "watch_locals_surface",
            Self::CrashLoopQuarantineBanner => "crash_loop_quarantine_banner",
            Self::CliHeadless => "cli_headless",
            Self::EvidenceExport => "evidence_export",
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

/// One debugger-stabilization truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerStabilizationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Debugger-stabilization lane this row certifies.
    pub lane_class: DebuggerStabilizationLaneClass,
    /// Row class.
    pub row_class: DebuggerStabilizationRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Adapter descriptor field bound by the row (or `not_applicable`).
    pub adapter_descriptor_field_class: AdapterDescriptorFieldClass,
    /// Attach/launch parity surface bound by the row (or `not_applicable`).
    pub attach_launch_parity_surface_class: AttachLaunchParitySurfaceClass,
    /// Attach/launch posture bound by the row (or `not_applicable`).
    pub attach_launch_posture_class: AttachLaunchPostureClass,
    /// Crash-isolation assertion bound by the row (or `not_applicable`).
    pub crash_isolation_assertion_class: CrashIsolationAssertionClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: DebuggerStabilizationConfidenceClass,
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
    /// For crash_isolation_assertion_binding rows, true when the row
    /// attests the bound assertion holds.
    #[serde(default)]
    pub attests_crash_isolation_assertion: bool,
    /// True when raw debugger payloads, raw stack frames, raw memory bytes,
    /// raw command lines, raw process environment bytes, or raw scrollback
    /// bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl DebuggerStabilizationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerStabilizationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Debugger-stabilization packet id consumed by the projection.
    pub debugger_stabilization_truth_packet_id_ref: String,
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
    /// True when the adapter-descriptor-field vocabulary is preserved verbatim.
    pub preserves_adapter_descriptor_field_vocabulary: bool,
    /// True when the attach/launch parity-surface vocabulary is preserved verbatim.
    pub preserves_attach_launch_parity_surface_vocabulary: bool,
    /// True when the attach/launch posture vocabulary is preserved verbatim.
    pub preserves_attach_launch_posture_vocabulary: bool,
    /// True when the crash-isolation-assertion vocabulary is preserved verbatim.
    pub preserves_crash_isolation_assertion_vocabulary: bool,
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

impl DebuggerStabilizationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.debugger_stabilization_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_adapter_descriptor_field_vocabulary
            && self.preserves_attach_launch_parity_surface_vocabulary
            && self.preserves_attach_launch_posture_vocabulary
            && self.preserves_crash_isolation_assertion_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DebuggerStabilizationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerStabilizationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Debugger-stabilization lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<DebuggerStabilizationLaneClass>,
    /// Debugger-stabilization rows.
    #[serde(default)]
    pub rows: Vec<DebuggerStabilizationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DebuggerStabilizationConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying local, remote/helper, container, and
/// notebook-bridge debugger truth at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerStabilizationTruthPacket {
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
    /// Debugger-stabilization lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<DebuggerStabilizationLaneClass>,
    /// Debugger-stabilization rows.
    #[serde(default)]
    pub rows: Vec<DebuggerStabilizationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DebuggerStabilizationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl DebuggerStabilizationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: DebuggerStabilizationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DEBUGGER_STABILIZATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable debugger-stabilization invariants.
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
            .map(DebuggerStabilizationLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(DebuggerStabilizationRowClass::as_str)
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

    /// Returns the unique adapter-descriptor-field tokens observed across rows.
    pub fn adapter_descriptor_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.adapter_descriptor_field_class);
        }
        set.into_iter()
            .map(AdapterDescriptorFieldClass::as_str)
            .collect()
    }

    /// Returns the unique attach/launch parity-surface tokens observed across rows.
    pub fn attach_launch_parity_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.attach_launch_parity_surface_class);
        }
        set.into_iter()
            .map(AttachLaunchParitySurfaceClass::as_str)
            .collect()
    }

    /// Returns the unique attach/launch posture tokens observed across rows.
    pub fn attach_launch_posture_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.attach_launch_posture_class);
        }
        set.into_iter()
            .map(AttachLaunchPostureClass::as_str)
            .collect()
    }

    /// Returns the unique crash-isolation-assertion tokens observed across rows.
    pub fn crash_isolation_assertion_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.crash_isolation_assertion_class);
        }
        set.into_iter()
            .map(CrashIsolationAssertionClass::as_str)
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

    /// Builds a support export wrapping the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DebuggerStabilizationTruthSupportExport {
        DebuggerStabilizationTruthSupportExport {
            record_kind: DEBUGGER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            debugger_stabilization_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            debugger_stabilization_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != DEBUGGER_STABILIZATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "debugger-stabilization truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "debugger-stabilization truth packet has the wrong schema version",
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
                "packet must declare at least one covered debugger lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers debugger lane {}", lane.as_str()),
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
                        "row {} admits raw debugger payloads, raw stack frames, raw memory bytes, raw command lines, raw env bytes, or raw scrollback bodies past the boundary",
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

            if row.row_class.requires_adapter_descriptor_field()
                && matches!(
                    row.adapter_descriptor_field_class,
                    AdapterDescriptorFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AdapterDescriptorFieldNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an adapter_descriptor_field_binding but has no bound field",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_adapter_descriptor_field()
                && !matches!(
                    row.adapter_descriptor_field_class,
                    AdapterDescriptorFieldClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AdapterDescriptorFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds adapter descriptor field {}; only adapter_descriptor_field_binding rows may bind a field",
                        row.row_id,
                        row.row_class.as_str(),
                        row.adapter_descriptor_field_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_attach_launch_parity_surface()
                && matches!(
                    row.attach_launch_parity_surface_class,
                    AttachLaunchParitySurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchParitySurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an attach_launch_parity_surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_attach_launch_parity_surface()
                && !matches!(
                    row.attach_launch_parity_surface_class,
                    AttachLaunchParitySurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchParitySurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds attach/launch parity surface {}; only attach_launch_parity_surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.attach_launch_parity_surface_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_attach_launch_posture()
                && matches!(
                    row.attach_launch_posture_class,
                    AttachLaunchPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchPostureNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds an attach/launch parity surface but has no bound posture",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_attach_launch_posture()
                && !matches!(
                    row.attach_launch_posture_class,
                    AttachLaunchPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchPostureNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds attach/launch posture {}; only attach_launch_parity_surface_binding rows may bind a posture",
                        row.row_id,
                        row.row_class.as_str(),
                        row.attach_launch_posture_class.as_str()
                    ),
                ));
            }

            if row.attach_launch_posture_class.requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchPostureMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds attach/launch posture {} without a disclosure ref",
                        row.row_id,
                        row.attach_launch_posture_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_crash_isolation_assertion()
                && matches!(
                    row.crash_isolation_assertion_class,
                    CrashIsolationAssertionClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CrashIsolationAssertionNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a crash_isolation_assertion_binding but has no bound assertion",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_crash_isolation_assertion()
                && !matches!(
                    row.crash_isolation_assertion_class,
                    CrashIsolationAssertionClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CrashIsolationAssertionNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds crash-isolation assertion {}; only crash_isolation_assertion_binding rows may bind an assertion",
                        row.row_id,
                        row.row_class.as_str(),
                        row.crash_isolation_assertion_class.as_str()
                    ),
                ));
            }
            if matches!(
                row.row_class,
                DebuggerStabilizationRowClass::CrashIsolationAssertionBinding
            ) && !row.attests_crash_isolation_assertion
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CrashIsolationAssertionNotAttested,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a crash_isolation_assertion_binding but does not attest the bound assertion",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.row_class,
                DebuggerStabilizationRowClass::LineageAdmission
            ) && row
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

            if matches!(
                row.confidence_class,
                DebuggerStabilizationConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::LaunchStable)
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
                        DebuggerStabilizationRowClass::DebuggerStabilizationQuality
                    )
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, DebuggerStabilizationRowClass::WedgeAdmission)
                        && row.wedge_class == wedge
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingWedgeAdmissionCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no wedge_admission row for {}",
                            lane.as_str(),
                            wedge.as_str()
                        ),
                    ));
                }
            }

            for field in AdapterDescriptorFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebuggerStabilizationRowClass::AdapterDescriptorFieldBinding
                        )
                        && row.adapter_descriptor_field_class == field
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingAdapterDescriptorFieldCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no adapter_descriptor_field_binding row for {}",
                            lane.as_str(),
                            field.as_str()
                        ),
                    ));
                }
            }

            for surface in AttachLaunchParitySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebuggerStabilizationRowClass::AttachLaunchParitySurfaceBinding
                        )
                        && row.attach_launch_parity_surface_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingAttachLaunchParitySurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no attach_launch_parity_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            for assertion in CrashIsolationAssertionClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebuggerStabilizationRowClass::CrashIsolationAssertionBinding
                        )
                        && row.crash_isolation_assertion_class == assertion
                        && row.attests_crash_isolation_assertion
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingCrashIsolationAssertionCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no attested crash_isolation_assertion_binding row for {}",
                            lane.as_str(),
                            assertion.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        DebuggerStabilizationRowClass::LineageAdmission
                    )
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

            let postures: BTreeSet<AttachLaunchPostureClass> = self
                .rows
                .iter()
                .filter(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebuggerStabilizationRowClass::AttachLaunchParitySurfaceBinding
                        )
                        && !matches!(
                            row.attach_launch_posture_class,
                            AttachLaunchPostureClass::NotApplicable
                        )
                })
                .map(|row| row.attach_launch_posture_class)
                .collect();
            if postures.len() > 1 {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchPostureDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} reports attach/launch posture drift across parity surfaces: {:?}",
                        lane.as_str(),
                        postures
                            .iter()
                            .map(|posture| posture.as_str())
                            .collect::<Vec<_>>()
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
                        "projection {} does not preserve debugger-stabilization truth",
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
            if !projection.preserves_adapter_descriptor_field_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AdapterDescriptorFieldVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the adapter-descriptor-field vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_attach_launch_parity_surface_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchParitySurfaceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the attach/launch parity-surface vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_attach_launch_posture_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AttachLaunchPostureVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the attach/launch posture vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_crash_isolation_assertion_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::CrashIsolationAssertionVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the crash-isolation-assertion vocabulary",
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
pub struct DebuggerStabilizationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub debugger_stabilization_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub debugger_stabilization_truth_packet: DebuggerStabilizationTruthPacket,
}

impl DebuggerStabilizationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DEBUGGER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION
            && self.debugger_stabilization_truth_packet_id_ref
                == self.debugger_stabilization_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.debugger_stabilization_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum DebuggerStabilizationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for DebuggerStabilizationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "debugger-stabilization truth packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "debugger-stabilization truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DebuggerStabilizationTruthArtifactError {}

/// Returns the checked-in stable debugger-stabilization truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_debugger_stabilization_truth_packet(
) -> Result<DebuggerStabilizationTruthPacket, DebuggerStabilizationTruthArtifactError> {
    let packet: DebuggerStabilizationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/stabilize_debugger_host_and_adapter_negotiation_truth_packet.json"
    )))
    .map_err(DebuggerStabilizationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DebuggerStabilizationTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        DEBUGGER_STABILIZATION_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(
        prefix: &str,
        lane: DebuggerStabilizationLaneClass,
    ) -> DebuggerStabilizationRow {
        DebuggerStabilizationRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: DebuggerStabilizationRowClass::DebuggerStabilizationQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            adapter_descriptor_field_class: AdapterDescriptorFieldClass::NotApplicable,
            attach_launch_parity_surface_class: AttachLaunchParitySurfaceClass::NotApplicable,
            attach_launch_posture_class: AttachLaunchPostureClass::NotApplicable,
            crash_isolation_assertion_class: CrashIsolationAssertionClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: DebuggerStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            attests_crash_isolation_assertion: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: DebuggerStabilizationLaneClass,
        wedge: WedgeClass,
    ) -> DebuggerStabilizationRow {
        DebuggerStabilizationRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: DebuggerStabilizationRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            adapter_descriptor_field_class: AdapterDescriptorFieldClass::NotApplicable,
            attach_launch_parity_surface_class: AttachLaunchParitySurfaceClass::NotApplicable,
            attach_launch_posture_class: AttachLaunchPostureClass::NotApplicable,
            crash_isolation_assertion_class: CrashIsolationAssertionClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            confidence_class: DebuggerStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_crash_isolation_assertion: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn adapter_descriptor_row(
        prefix: &str,
        lane: DebuggerStabilizationLaneClass,
        field: AdapterDescriptorFieldClass,
    ) -> DebuggerStabilizationRow {
        DebuggerStabilizationRow {
            row_id: format!("row:{prefix}:descriptor:{}", field.as_str()),
            lane_class: lane,
            row_class: DebuggerStabilizationRowClass::AdapterDescriptorFieldBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            adapter_descriptor_field_class: field,
            attach_launch_parity_surface_class: AttachLaunchParitySurfaceClass::NotApplicable,
            attach_launch_posture_class: AttachLaunchPostureClass::NotApplicable,
            crash_isolation_assertion_class: CrashIsolationAssertionClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnAdapterDescriptorFieldGap,
            confidence_class: DebuggerStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_adapter_descriptor_field_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_crash_isolation_assertion: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn parity_surface_row(
        prefix: &str,
        lane: DebuggerStabilizationLaneClass,
        surface: AttachLaunchParitySurfaceClass,
    ) -> DebuggerStabilizationRow {
        DebuggerStabilizationRow {
            row_id: format!("row:{prefix}:parity:{}", surface.as_str()),
            lane_class: lane,
            row_class: DebuggerStabilizationRowClass::AttachLaunchParitySurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            adapter_descriptor_field_class: AdapterDescriptorFieldClass::NotApplicable,
            attach_launch_parity_surface_class: surface,
            attach_launch_posture_class: AttachLaunchPostureClass::Supported,
            crash_isolation_assertion_class: CrashIsolationAssertionClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnAttachLaunchParitySurfaceGap,
            confidence_class: DebuggerStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_attach_launch_parity_surface_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_crash_isolation_assertion: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn crash_isolation_row(
        prefix: &str,
        lane: DebuggerStabilizationLaneClass,
        assertion: CrashIsolationAssertionClass,
    ) -> DebuggerStabilizationRow {
        DebuggerStabilizationRow {
            row_id: format!("row:{prefix}:crash_isolation:{}", assertion.as_str()),
            lane_class: lane,
            row_class: DebuggerStabilizationRowClass::CrashIsolationAssertionBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            adapter_descriptor_field_class: AdapterDescriptorFieldClass::NotApplicable,
            attach_launch_parity_surface_class: AttachLaunchParitySurfaceClass::NotApplicable,
            attach_launch_posture_class: AttachLaunchPostureClass::NotApplicable,
            crash_isolation_assertion_class: assertion,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnCrashIsolationAssertionGap,
            confidence_class: DebuggerStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_crash_isolation_assertion_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_crash_isolation_assertion: true,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(
        prefix: &str,
        lane: DebuggerStabilizationLaneClass,
    ) -> DebuggerStabilizationRow {
        DebuggerStabilizationRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: DebuggerStabilizationRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            adapter_descriptor_field_class: AdapterDescriptorFieldClass::NotApplicable,
            attach_launch_parity_surface_class: AttachLaunchParitySurfaceClass::NotApplicable,
            attach_launch_posture_class: AttachLaunchPostureClass::NotApplicable,
            crash_isolation_assertion_class: CrashIsolationAssertionClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: DebuggerStabilizationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:debugger_lineage")),
            attests_crash_isolation_assertion: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> DebuggerStabilizationConsumerProjection {
        DebuggerStabilizationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            debugger_stabilization_truth_packet_id_ref:
                "packet:m4:stabilize_debugger_host_and_adapter_negotiation".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_adapter_descriptor_field_vocabulary: true,
            preserves_attach_launch_parity_surface_vocabulary: true,
            preserves_attach_launch_posture_vocabulary: true,
            preserves_crash_isolation_assertion_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: DebuggerStabilizationLaneClass,
        prefix: &str,
    ) -> Vec<DebuggerStabilizationRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for field in AdapterDescriptorFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(adapter_descriptor_row(prefix, lane, field));
        }
        for surface in AttachLaunchParitySurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(parity_surface_row(prefix, lane, surface));
        }
        for assertion in CrashIsolationAssertionClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(crash_isolation_row(prefix, lane, assertion));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> DebuggerStabilizationTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(DebuggerStabilizationLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            DebuggerStabilizationLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(
            DebuggerStabilizationLaneClass::ContainerLane,
            "container",
        ));
        rows.extend(lane_rows(
            DebuggerStabilizationLaneClass::NotebookBridgeLane,
            "notebook",
        ));
        DebuggerStabilizationTruthPacketInput {
            packet_id: "packet:m4:stabilize_debugger_host_and_adapter_negotiation".to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: DebuggerStabilizationLaneClass::REQUIRED.to_vec(),
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
            DebuggerStabilizationLaneClass::LocalLane.as_str(),
            "local_lane"
        );
        assert_eq!(
            DebuggerStabilizationLaneClass::RemoteHelperLane.as_str(),
            "remote_helper_lane"
        );
        assert_eq!(
            DebuggerStabilizationLaneClass::ContainerLane.as_str(),
            "container_lane"
        );
        assert_eq!(
            DebuggerStabilizationLaneClass::NotebookBridgeLane.as_str(),
            "notebook_bridge_lane"
        );
        assert_eq!(
            DebuggerStabilizationRowClass::DebuggerStabilizationQuality.as_str(),
            "debugger_stabilization_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(WedgeClass::DebuggerHost.as_str(), "debugger_host");
        assert_eq!(WedgeClass::CrashIsolation.as_str(), "crash_isolation");
        assert_eq!(
            AdapterDescriptorFieldClass::AdapterIdentity.as_str(),
            "adapter_identity"
        );
        assert_eq!(
            AdapterDescriptorFieldClass::NotebookBridgeOrReplayOnlyLimitation.as_str(),
            "notebook_bridge_or_replay_only_limitation"
        );
        assert_eq!(
            AttachLaunchParitySurfaceClass::UiSurface.as_str(),
            "ui_surface"
        );
        assert_eq!(AttachLaunchPostureClass::Supported.as_str(), "supported");
        assert_eq!(
            AttachLaunchPostureClass::PolicyBlocked.as_str(),
            "policy_blocked"
        );
        assert_eq!(
            CrashIsolationAssertionClass::BoundedRestartBudget.as_str(),
            "bounded_restart_budget"
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
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::AttachLaunchPostureDrift.as_str(),
            "attach_launch_posture_drift"
        );
        assert_eq!(
            FindingKind::CrashIsolationAssertionNotAttested.as_str(),
            "crash_isolation_assertion_not_attested"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = DebuggerStabilizationTruthPacket::materialize(sample_input());
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
                "support:m4:stabilize_debugger_host_and_adapter_negotiation",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_adapter_descriptor_field_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                DebuggerStabilizationRowClass::AdapterDescriptorFieldBinding
            ) && row.adapter_descriptor_field_class
                == AdapterDescriptorFieldClass::NotebookBridgeOrReplayOnlyLimitation
                && row.lane_class == DebuggerStabilizationLaneClass::LocalLane)
        });
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingAdapterDescriptorFieldCoverage
        }));
    }

    #[test]
    fn attach_launch_posture_drift_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                DebuggerStabilizationRowClass::AttachLaunchParitySurfaceBinding
            ) && row.lane_class == DebuggerStabilizationLaneClass::LocalLane
                && row.attach_launch_parity_surface_class
                    == AttachLaunchParitySurfaceClass::CliHeadless
            {
                row.attach_launch_posture_class = AttachLaunchPostureClass::Limited;
                row.disclosure_ref =
                    Some(format!("{}#auto_narrow_on_attach_launch_posture_drift", doc_ref()));
                break;
            }
        }
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::AttachLaunchPostureDrift));
    }

    #[test]
    fn crash_isolation_assertion_not_attested_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                DebuggerStabilizationRowClass::CrashIsolationAssertionBinding
            ) && row.lane_class == DebuggerStabilizationLaneClass::LocalLane
                && row.crash_isolation_assertion_class
                    == CrashIsolationAssertionClass::BoundedRestartBudget
            {
                row.attests_crash_isolation_assertion = false;
                break;
            }
        }
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::CrashIsolationAssertionNotAttested
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, DebuggerStabilizationRowClass::LineageAdmission)
                && row.lane_class == DebuggerStabilizationLaneClass::LocalLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerSurface::ConformanceDashboard
        });
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_attach_launch_posture_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_attach_launch_posture_vocabulary = false;
            }
        }
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::AttachLaunchPostureVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = DebuggerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
