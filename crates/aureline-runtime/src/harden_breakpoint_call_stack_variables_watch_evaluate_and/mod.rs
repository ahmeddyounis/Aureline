//! Breakpoint / call-stack / variables / watch / evaluate / debug-console
//! fidelity hardening truth packet for the M4 stable lane.
//!
//! This module pins how local, remote/helper, container, and notebook-bridge
//! debug sessions serialize one canonical debug-fidelity truth across the
//! six debug-fidelity wedges (`breakpoint_fidelity`, `call_stack_fidelity`,
//! `variables_fidelity`, `watch_fidelity`, `evaluate_fidelity`,
//! `debug_console_fidelity`). Variables, watches, evaluate, and
//! console-linked inspector rows MUST distinguish `live`, `snapshot`,
//! `stale`, `limited`, `unavailable`, and `policy_blocked` state on every
//! claimed stable debug lane, including remote/helper lanes and any
//! notebook-adjacent bridge that appears in-product. Mapping-fidelity and
//! host-lane identity badges (`exact`, `approximate`, `partial`,
//! `unavailable`, `stale`, `mismatched`) MUST remain visible in stack,
//! watch, evaluate, and debug-console flows and MUST survive export /
//! support packets rather than being flattened into generic error copy.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `debug_fidelity_quality` row cannot prove:
//!
//! - the six debug-fidelity wedges (`breakpoint_fidelity`,
//!   `call_stack_fidelity`, `variables_fidelity`, `watch_fidelity`,
//!   `evaluate_fidelity`, `debug_console_fidelity`) each have a
//!   structured `wedge_admission` row,
//! - the six inspector-state classes (`live`, `snapshot`, `stale`,
//!   `limited`, `unavailable`, `policy_blocked`) each have a structured
//!   `inspector_state_admission` row so reviewers cannot infer freshness
//!   from generic error copy,
//! - the six mapping-fidelity badge classes (`exact`, `approximate`,
//!   `partial`, `unavailable`, `stale`, `mismatched`) each have a
//!   structured `mapping_fidelity_badge_admission` row so the badges
//!   survive into export and support packets,
//! - the six inspector-surface bindings (`breakpoint_surface`,
//!   `call_stack_surface`, `variables_surface`, `watch_surface`,
//!   `evaluate_surface`, `debug_console_surface`) each carry an
//!   `inspector_surface_binding` row attesting that the surface
//!   preserves the inspector-state and mapping-fidelity vocabularies it
//!   is required to preserve,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through every emitted debug-session envelope and downstream
//!   consumer surface.
//!
//! Every row binds a closed `debug_fidelity_lane_class`,
//! `debug_fidelity_row_class`, `support_class`, `wedge_class`,
//! `inspector_state_class`, `mapping_fidelity_badge_class`,
//! `inspector_surface_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `debug_fidelity_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the row
//! is narrowed below launch-stable, declares a non-`none_declared` known
//! limit, or binds a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw debugger payloads,
//! raw stack frames, raw memory bytes, raw watch expressions, raw evaluate
//! input/output, raw console scrollback bodies, raw command lines, raw
//! process environment bytes, raw secrets, or ambient credentials past
//! the boundary. A row that claims `launch_stable` while leaving its
//! known limit, downgrade automation, or evidence class unbound is
//! refused; the validator narrows below launch-stable instead of
//! inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`DebugFidelityTruthPacket`].
pub const DEBUG_FIDELITY_TRUTH_PACKET_RECORD_KIND: &str =
    "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_packet";

/// Stable record-kind tag for [`DebugFidelityTruthSupportExport`].
pub const DEBUG_FIDELITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_support_export";

/// Integer schema version for the debug-fidelity truth packet.
pub const DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DEBUG_FIDELITY_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const DEBUG_FIDELITY_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const DEBUG_FIDELITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const DEBUG_FIDELITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and";

/// Repo-relative path of the checked-in stable packet.
pub const DEBUG_FIDELITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json";

/// Closed debug-fidelity lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugFidelityLaneClass {
    /// Local-host debug session lane.
    LocalLane,
    /// Remote / helper attach debug session lane.
    RemoteHelperLane,
    /// Container-attached debug session lane.
    ContainerLane,
    /// Notebook-bridge debug session lane (kernel debugger bridge).
    NotebookBridgeLane,
}

impl DebugFidelityLaneClass {
    /// Every required debug-fidelity lane, in declaration order.
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

/// Closed debug-fidelity row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugFidelityRowClass {
    /// The lane's headline debug-fidelity qualification row.
    DebugFidelityQuality,
    /// A row admitting one of the six debug-fidelity wedges.
    WedgeAdmission,
    /// A row admitting one inspector-state class (`live`, `snapshot`,
    /// `stale`, `limited`, `unavailable`, `policy_blocked`).
    InspectorStateAdmission,
    /// A row admitting one mapping-fidelity badge class (`exact`,
    /// `approximate`, `partial`, `unavailable`, `stale`, `mismatched`).
    MappingFidelityBadgeAdmission,
    /// A row binding one inspector surface (`breakpoint_surface`,
    /// `call_stack_surface`, `variables_surface`, `watch_surface`,
    /// `evaluate_surface`, `debug_console_surface`) and attesting that
    /// the surface preserves the inspector-state and mapping-fidelity
    /// vocabularies it must preserve.
    InspectorSurfaceBinding,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted debug-session truth and downstream
    /// consumer surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl DebugFidelityRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DebugFidelityQuality => "debug_fidelity_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::InspectorStateAdmission => "inspector_state_admission",
            Self::MappingFidelityBadgeAdmission => "mapping_fidelity_badge_admission",
            Self::InspectorSurfaceBinding => "inspector_surface_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound inspector state.
    pub const fn requires_inspector_state(self) -> bool {
        matches!(self, Self::InspectorStateAdmission)
    }

    /// True when this row class requires a bound mapping-fidelity badge.
    pub const fn requires_mapping_fidelity_badge(self) -> bool {
        matches!(self, Self::MappingFidelityBadgeAdmission)
    }

    /// True when this row class requires a bound inspector surface.
    pub const fn requires_inspector_surface(self) -> bool {
        matches!(self, Self::InspectorSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a debug-fidelity row.
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

/// Closed debug-fidelity wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each required
/// wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Breakpoint fidelity wedge (set/clear, verified state, conditions,
    /// hit counts, logpoints, source-mapped vs unverified breakpoints).
    BreakpointFidelity,
    /// Call-stack fidelity wedge (frames, source mapping, async/inlined
    /// frame disclosure, host-lane identity badge surfacing).
    CallStackFidelity,
    /// Variables fidelity wedge (scopes, lazy/eager evaluation,
    /// inspector-state distinction, redaction posture).
    VariablesFidelity,
    /// Watch fidelity wedge (watch expressions, inspector-state and
    /// mapping-fidelity preservation, restore behavior).
    WatchFidelity,
    /// Evaluate fidelity wedge (REPL-style evaluation, inspector-state
    /// preservation, mapping-fidelity preservation, redaction posture).
    EvaluateFidelity,
    /// Debug-console fidelity wedge (console-linked inspector output,
    /// state and mapping-fidelity preservation, transcript truth).
    DebugConsoleFidelity,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::BreakpointFidelity,
        Self::CallStackFidelity,
        Self::VariablesFidelity,
        Self::WatchFidelity,
        Self::EvaluateFidelity,
        Self::DebugConsoleFidelity,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BreakpointFidelity => "breakpoint_fidelity",
            Self::CallStackFidelity => "call_stack_fidelity",
            Self::VariablesFidelity => "variables_fidelity",
            Self::WatchFidelity => "watch_fidelity",
            Self::EvaluateFidelity => "evaluate_fidelity",
            Self::DebugConsoleFidelity => "debug_console_fidelity",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed inspector-state vocabulary. Every lane claiming `launch_stable`
/// MUST publish an `inspector_state_admission` row for each required state.
/// These states are the truth model that variables, watches, evaluate, and
/// console-linked inspector rows MUST distinguish so reviewers cannot
/// infer freshness from generic error copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorStateClass {
    /// `live` — inspector value reflects the current paused frame.
    Live,
    /// `snapshot` — inspector value reflects a captured snapshot.
    Snapshot,
    /// `stale` — inspector value is older than the current pause point.
    Stale,
    /// `limited` — inspector value is partially available (lazy/limited).
    Limited,
    /// `unavailable` — inspector value cannot be retrieved.
    Unavailable,
    /// `policy_blocked` — inspector value retrieval is blocked by policy.
    PolicyBlocked,
    /// The row is not bound to an inspector state.
    NotApplicable,
}

impl InspectorStateClass {
    /// Every required inspector state for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::Live,
        Self::Snapshot,
        Self::Stale,
        Self::Limited,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Snapshot => "snapshot",
            Self::Stale => "stale",
            Self::Limited => "limited",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed mapping-fidelity badge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `mapping_fidelity_badge_admission` row
/// for each required badge so stack, watch, evaluate, and debug-console
/// flows preserve the badge instead of flattening it into generic error
/// copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingFidelityBadgeClass {
    /// `exact` — mapping is exact for the frame / variable / watch.
    Exact,
    /// `approximate` — mapping is approximate but trustworthy.
    Approximate,
    /// `partial` — mapping is partial; some detail is missing.
    Partial,
    /// `unavailable` — mapping is unavailable for the frame / variable.
    Unavailable,
    /// `stale` — mapping is stale (e.g. source changed since launch).
    Stale,
    /// `mismatched` — mapping is mismatched (e.g. binary / source skew).
    Mismatched,
    /// The row is not bound to a mapping-fidelity badge.
    NotApplicable,
}

impl MappingFidelityBadgeClass {
    /// Every required mapping-fidelity badge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::Exact,
        Self::Approximate,
        Self::Partial,
        Self::Unavailable,
        Self::Stale,
        Self::Mismatched,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
            Self::Stale => "stale",
            Self::Mismatched => "mismatched",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed inspector-surface vocabulary. Every lane claiming `launch_stable`
/// MUST publish an `inspector_surface_binding` row for each required
/// surface so the inspector-state and mapping-fidelity vocabularies
/// survive into product chrome, export bundles, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorSurfaceClass {
    /// Breakpoint surface (verified / unverified breakpoint chips).
    BreakpointSurface,
    /// Call-stack surface (frame list, host-lane identity badges,
    /// mapping-fidelity badges).
    CallStackSurface,
    /// Variables surface (scopes, lazy evaluation, inspector state).
    VariablesSurface,
    /// Watch surface (watch expressions, inspector state + mapping
    /// fidelity).
    WatchSurface,
    /// Evaluate surface (REPL-style evaluation, inspector state +
    /// mapping fidelity).
    EvaluateSurface,
    /// Debug-console surface (console-linked inspector output, state +
    /// mapping fidelity).
    DebugConsoleSurface,
    /// The row is not bound to an inspector surface.
    NotApplicable,
}

impl InspectorSurfaceClass {
    /// Every required inspector surface for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::BreakpointSurface,
        Self::CallStackSurface,
        Self::VariablesSurface,
        Self::WatchSurface,
        Self::EvaluateSurface,
        Self::DebugConsoleSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BreakpointSurface => "breakpoint_surface",
            Self::CallStackSurface => "call_stack_surface",
            Self::VariablesSurface => "variables_surface",
            Self::WatchSurface => "watch_surface",
            Self::EvaluateSurface => "evaluate_surface",
            Self::DebugConsoleSurface => "debug_console_surface",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this surface MUST attest that it preserves the
    /// inspector-state vocabulary.
    pub const fn requires_inspector_state_attestation(self) -> bool {
        matches!(
            self,
            Self::VariablesSurface
                | Self::WatchSurface
                | Self::EvaluateSurface
                | Self::DebugConsoleSurface
        )
    }

    /// True when this surface MUST attest that it preserves the
    /// mapping-fidelity badge vocabulary.
    pub const fn requires_mapping_fidelity_attestation(self) -> bool {
        matches!(
            self,
            Self::CallStackSurface
                | Self::WatchSurface
                | Self::EvaluateSurface
                | Self::DebugConsoleSurface
        )
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

/// Closed known-limit vocabulary attached to a debug-fidelity row.
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
    /// The lane only certifies a subset of the six debug-fidelity wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the six inspector states.
    InspectorStateSubsetOnly,
    /// The lane only certifies a subset of the six mapping-fidelity badges.
    MappingFidelityBadgeSubsetOnly,
    /// The lane only certifies a subset of the six inspector surfaces.
    InspectorSurfaceSubsetOnly,
    /// The lane reports inspector-state attestation skew on one or more
    /// state-bearing surfaces.
    InspectorStateAttestationSkewDeclared,
    /// The lane reports mapping-fidelity attestation skew on one or more
    /// badge-bearing surfaces.
    MappingFidelityAttestationSkewDeclared,
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
            Self::InspectorStateSubsetOnly => "inspector_state_subset_only",
            Self::MappingFidelityBadgeSubsetOnly => "mapping_fidelity_badge_subset_only",
            Self::InspectorSurfaceSubsetOnly => "inspector_surface_subset_only",
            Self::InspectorStateAttestationSkewDeclared => {
                "inspector_state_attestation_skew_declared"
            }
            Self::MappingFidelityAttestationSkewDeclared => {
                "mapping_fidelity_attestation_skew_declared"
            }
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
    /// Automatically narrow when a required inspector-state admission is
    /// missing.
    AutoNarrowOnInspectorStateGap,
    /// Automatically narrow when a required mapping-fidelity badge
    /// admission is missing.
    AutoNarrowOnMappingFidelityBadgeGap,
    /// Automatically narrow when a required inspector-surface binding is
    /// missing.
    AutoNarrowOnInspectorSurfaceGap,
    /// Automatically narrow when an inspector-surface row drops a required
    /// inspector-state attestation.
    AutoNarrowOnInspectorStateAttestationGap,
    /// Automatically narrow when an inspector-surface row drops a required
    /// mapping-fidelity attestation.
    AutoNarrowOnMappingFidelityAttestationGap,
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
            Self::AutoNarrowOnInspectorStateGap => "auto_narrow_on_inspector_state_gap",
            Self::AutoNarrowOnMappingFidelityBadgeGap => {
                "auto_narrow_on_mapping_fidelity_badge_gap"
            }
            Self::AutoNarrowOnInspectorSurfaceGap => "auto_narrow_on_inspector_surface_gap",
            Self::AutoNarrowOnInspectorStateAttestationGap => {
                "auto_narrow_on_inspector_state_attestation_gap"
            }
            Self::AutoNarrowOnMappingFidelityAttestationGap => {
                "auto_narrow_on_mapping_fidelity_attestation_gap"
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

/// Closed confidence-class vocabulary for a debug-fidelity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugFidelityConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl DebugFidelityConfidenceClass {
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
    /// A lane claiming launch_stable is missing a required inspector-state admission.
    MissingInspectorStateCoverage,
    /// A lane claiming launch_stable is missing a required mapping-fidelity-badge admission.
    MissingMappingFidelityBadgeCoverage,
    /// A lane claiming launch_stable is missing a required inspector-surface binding.
    MissingInspectorSurfaceCoverage,
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
    /// An inspector-state-admission row drops its inspector-state binding.
    InspectorStateNotApplicable,
    /// A non-inspector-state row binds an inspector state it cannot certify.
    InspectorStateNotPermittedOnRowClass,
    /// A mapping-fidelity-badge-admission row drops its badge binding.
    MappingFidelityBadgeNotApplicable,
    /// A non-mapping-fidelity row binds a badge it cannot certify.
    MappingFidelityBadgeNotPermittedOnRowClass,
    /// An inspector-surface-binding row drops its surface binding.
    InspectorSurfaceNotApplicable,
    /// A non-inspector-surface row binds an inspector surface it cannot certify.
    InspectorSurfaceNotPermittedOnRowClass,
    /// An inspector-surface row fails to attest the inspector-state vocabulary it must preserve.
    InspectorSurfaceMissingInspectorStateAttestation,
    /// An inspector-surface row fails to attest the mapping-fidelity vocabulary it must preserve.
    InspectorSurfaceMissingMappingFidelityAttestation,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A row admits raw debugger payloads, raw stack frames, raw memory bytes,
    /// raw watch expressions, raw evaluate input/output, raw console scrollback
    /// bodies, raw command lines, or raw process environment bytes past the boundary.
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
    /// A projection collapses the inspector-state vocabulary.
    InspectorStateVocabularyCollapsed,
    /// A projection collapses the mapping-fidelity-badge vocabulary.
    MappingFidelityBadgeVocabularyCollapsed,
    /// A projection collapses the inspector-surface vocabulary.
    InspectorSurfaceVocabularyCollapsed,
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
            Self::MissingInspectorStateCoverage => "missing_inspector_state_coverage",
            Self::MissingMappingFidelityBadgeCoverage => {
                "missing_mapping_fidelity_badge_coverage"
            }
            Self::MissingInspectorSurfaceCoverage => "missing_inspector_surface_coverage",
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
            Self::InspectorStateNotApplicable => "inspector_state_not_applicable",
            Self::InspectorStateNotPermittedOnRowClass => {
                "inspector_state_not_permitted_on_row_class"
            }
            Self::MappingFidelityBadgeNotApplicable => "mapping_fidelity_badge_not_applicable",
            Self::MappingFidelityBadgeNotPermittedOnRowClass => {
                "mapping_fidelity_badge_not_permitted_on_row_class"
            }
            Self::InspectorSurfaceNotApplicable => "inspector_surface_not_applicable",
            Self::InspectorSurfaceNotPermittedOnRowClass => {
                "inspector_surface_not_permitted_on_row_class"
            }
            Self::InspectorSurfaceMissingInspectorStateAttestation => {
                "inspector_surface_missing_inspector_state_attestation"
            }
            Self::InspectorSurfaceMissingMappingFidelityAttestation => {
                "inspector_surface_missing_mapping_fidelity_attestation"
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
            Self::InspectorStateVocabularyCollapsed => "inspector_state_vocabulary_collapsed",
            Self::MappingFidelityBadgeVocabularyCollapsed => {
                "mapping_fidelity_badge_vocabulary_collapsed"
            }
            Self::InspectorSurfaceVocabularyCollapsed => "inspector_surface_vocabulary_collapsed",
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
    /// Breakpoint surface (verified / unverified breakpoint chips).
    BreakpointSurface,
    /// Call-stack surface (frame list, host-lane identity badges).
    CallStackSurface,
    /// Variables / locals surface.
    VariablesSurface,
    /// Watch surface (watch expressions).
    WatchSurface,
    /// Evaluate surface (REPL-style evaluation).
    EvaluateSurface,
    /// Debug-console surface (console-linked inspector output).
    DebugConsoleSurface,
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
    pub const REQUIRED: [Self; 12] = [
        Self::BreakpointSurface,
        Self::CallStackSurface,
        Self::VariablesSurface,
        Self::WatchSurface,
        Self::EvaluateSurface,
        Self::DebugConsoleSurface,
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
            Self::BreakpointSurface => "breakpoint_surface",
            Self::CallStackSurface => "call_stack_surface",
            Self::VariablesSurface => "variables_surface",
            Self::WatchSurface => "watch_surface",
            Self::EvaluateSurface => "evaluate_surface",
            Self::DebugConsoleSurface => "debug_console_surface",
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

/// One debug-fidelity truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugFidelityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Debug-fidelity lane this row certifies.
    pub lane_class: DebugFidelityLaneClass,
    /// Row class.
    pub row_class: DebugFidelityRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Inspector state bound by the row (or `not_applicable`).
    pub inspector_state_class: InspectorStateClass,
    /// Mapping-fidelity badge bound by the row (or `not_applicable`).
    pub mapping_fidelity_badge_class: MappingFidelityBadgeClass,
    /// Inspector surface bound by the row (or `not_applicable`).
    pub inspector_surface_class: InspectorSurfaceClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: DebugFidelityConfidenceClass,
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
    /// For inspector_surface_binding rows, true when the surface
    /// preserves the inspector-state vocabulary verbatim.
    #[serde(default)]
    pub attests_inspector_state_preserved: bool,
    /// For inspector_surface_binding rows, true when the surface
    /// preserves the mapping-fidelity badge vocabulary verbatim.
    #[serde(default)]
    pub attests_mapping_fidelity_preserved: bool,
    /// True when raw debugger payloads, raw stack frames, raw memory bytes,
    /// raw watch expressions, raw evaluate input/output, raw console
    /// scrollback bodies, raw command lines, or raw process environment
    /// bytes are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl DebugFidelityRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugFidelityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Debug-fidelity packet id consumed by the projection.
    pub debug_fidelity_truth_packet_id_ref: String,
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
    /// True when the inspector-state vocabulary is preserved verbatim.
    pub preserves_inspector_state_vocabulary: bool,
    /// True when the mapping-fidelity badge vocabulary is preserved verbatim.
    pub preserves_mapping_fidelity_badge_vocabulary: bool,
    /// True when the inspector-surface vocabulary is preserved verbatim.
    pub preserves_inspector_surface_vocabulary: bool,
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

impl DebugFidelityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.debug_fidelity_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_inspector_state_vocabulary
            && self.preserves_mapping_fidelity_badge_vocabulary
            && self.preserves_inspector_surface_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DebugFidelityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugFidelityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Debug-fidelity lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<DebugFidelityLaneClass>,
    /// Debug-fidelity rows.
    #[serde(default)]
    pub rows: Vec<DebugFidelityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DebugFidelityConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying local, remote/helper, container, and
/// notebook-bridge debug-fidelity truth at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugFidelityTruthPacket {
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
    /// Debug-fidelity lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<DebugFidelityLaneClass>,
    /// Debug-fidelity rows.
    #[serde(default)]
    pub rows: Vec<DebugFidelityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DebugFidelityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl DebugFidelityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: DebugFidelityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DEBUG_FIDELITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable debug-fidelity invariants.
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
        set.into_iter().map(DebugFidelityLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(DebugFidelityRowClass::as_str).collect()
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

    /// Returns the unique inspector-state tokens observed across rows.
    pub fn inspector_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.inspector_state_class);
        }
        set.into_iter().map(InspectorStateClass::as_str).collect()
    }

    /// Returns the unique mapping-fidelity-badge tokens observed across rows.
    pub fn mapping_fidelity_badge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.mapping_fidelity_badge_class);
        }
        set.into_iter()
            .map(MappingFidelityBadgeClass::as_str)
            .collect()
    }

    /// Returns the unique inspector-surface tokens observed across rows.
    pub fn inspector_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.inspector_surface_class);
        }
        set.into_iter().map(InspectorSurfaceClass::as_str).collect()
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
    ) -> DebugFidelityTruthSupportExport {
        DebugFidelityTruthSupportExport {
            record_kind: DEBUG_FIDELITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            debug_fidelity_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            debug_fidelity_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != DEBUG_FIDELITY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "debug-fidelity truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "debug-fidelity truth packet has the wrong schema version",
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
                "packet must declare at least one covered debug-fidelity lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers debug-fidelity lane {}", lane.as_str()),
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
                        "row {} admits raw debugger payloads, raw stack frames, raw memory bytes, raw watch expressions, raw evaluate input/output, raw console scrollback bodies, raw command lines, or raw env bytes past the boundary",
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

            if row.row_class.requires_inspector_state()
                && matches!(row.inspector_state_class, InspectorStateClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::InspectorStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an inspector_state_admission but has no bound state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_inspector_state()
                && !matches!(row.inspector_state_class, InspectorStateClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::InspectorStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds inspector state {}; only inspector_state_admission rows may bind a state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.inspector_state_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_mapping_fidelity_badge()
                && matches!(
                    row.mapping_fidelity_badge_class,
                    MappingFidelityBadgeClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MappingFidelityBadgeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a mapping_fidelity_badge_admission but has no bound badge",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_mapping_fidelity_badge()
                && !matches!(
                    row.mapping_fidelity_badge_class,
                    MappingFidelityBadgeClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MappingFidelityBadgeNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds mapping-fidelity badge {}; only mapping_fidelity_badge_admission rows may bind a badge",
                        row.row_id,
                        row.row_class.as_str(),
                        row.mapping_fidelity_badge_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_inspector_surface()
                && matches!(
                    row.inspector_surface_class,
                    InspectorSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::InspectorSurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an inspector_surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_inspector_surface()
                && !matches!(
                    row.inspector_surface_class,
                    InspectorSurfaceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::InspectorSurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds inspector surface {}; only inspector_surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.inspector_surface_class.as_str()
                    ),
                ));
            }

            if matches!(
                row.row_class,
                DebugFidelityRowClass::InspectorSurfaceBinding
            ) {
                if row
                    .inspector_surface_class
                    .requires_inspector_state_attestation()
                    && !row.attests_inspector_state_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::InspectorSurfaceMissingInspectorStateAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds inspector surface {} but does not attest inspector-state preservation",
                            row.row_id,
                            row.inspector_surface_class.as_str()
                        ),
                    ));
                }
                if row
                    .inspector_surface_class
                    .requires_mapping_fidelity_attestation()
                    && !row.attests_mapping_fidelity_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::InspectorSurfaceMissingMappingFidelityAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds inspector surface {} but does not attest mapping-fidelity preservation",
                            row.row_id,
                            row.inspector_surface_class.as_str()
                        ),
                    ));
                }
            }

            if matches!(row.row_class, DebugFidelityRowClass::LineageAdmission)
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

            if matches!(
                row.confidence_class,
                DebugFidelityConfidenceClass::LowConfidence
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
                    && matches!(row.row_class, DebugFidelityRowClass::DebugFidelityQuality)
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, DebugFidelityRowClass::WedgeAdmission)
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

            for state in InspectorStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebugFidelityRowClass::InspectorStateAdmission
                        )
                        && row.inspector_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingInspectorStateCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no inspector_state_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for badge in MappingFidelityBadgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebugFidelityRowClass::MappingFidelityBadgeAdmission
                        )
                        && row.mapping_fidelity_badge_class == badge
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingMappingFidelityBadgeCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no mapping_fidelity_badge_admission row for {}",
                            lane.as_str(),
                            badge.as_str()
                        ),
                    ));
                }
            }

            for surface in InspectorSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            DebugFidelityRowClass::InspectorSurfaceBinding
                        )
                        && row.inspector_surface_class == surface
                        && (!surface.requires_inspector_state_attestation()
                            || row.attests_inspector_state_preserved)
                        && (!surface.requires_mapping_fidelity_attestation()
                            || row.attests_mapping_fidelity_preserved)
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingInspectorSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no fully-attested inspector_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, DebugFidelityRowClass::LineageAdmission)
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
                        "projection {} does not preserve debug-fidelity truth",
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
            if !projection.preserves_inspector_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::InspectorStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the inspector-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_mapping_fidelity_badge_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::MappingFidelityBadgeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the mapping-fidelity-badge vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_inspector_surface_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::InspectorSurfaceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the inspector-surface vocabulary",
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
pub struct DebugFidelityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub debug_fidelity_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub debug_fidelity_truth_packet: DebugFidelityTruthPacket,
}

impl DebugFidelityTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DEBUG_FIDELITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION
            && self.debug_fidelity_truth_packet_id_ref
                == self.debug_fidelity_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.debug_fidelity_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum DebugFidelityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for DebugFidelityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "debug-fidelity truth packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "debug-fidelity truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DebugFidelityTruthArtifactError {}

/// Returns the checked-in stable debug-fidelity truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_debug_fidelity_truth_packet(
) -> Result<DebugFidelityTruthPacket, DebugFidelityTruthArtifactError> {
    let packet: DebugFidelityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json"
    )))
    .map_err(DebugFidelityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DebugFidelityTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        DEBUG_FIDELITY_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        DEBUG_FIDELITY_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: DebugFidelityLaneClass) -> DebugFidelityRow {
        DebugFidelityRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: DebugFidelityRowClass::DebugFidelityQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            inspector_state_class: InspectorStateClass::NotApplicable,
            mapping_fidelity_badge_class: MappingFidelityBadgeClass::NotApplicable,
            inspector_surface_class: InspectorSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: DebugFidelityConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            attests_inspector_state_preserved: false,
            attests_mapping_fidelity_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: DebugFidelityLaneClass,
        wedge: WedgeClass,
    ) -> DebugFidelityRow {
        DebugFidelityRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: DebugFidelityRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            inspector_state_class: InspectorStateClass::NotApplicable,
            mapping_fidelity_badge_class: MappingFidelityBadgeClass::NotApplicable,
            inspector_surface_class: InspectorSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            confidence_class: DebugFidelityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_inspector_state_preserved: false,
            attests_mapping_fidelity_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn inspector_state_row(
        prefix: &str,
        lane: DebugFidelityLaneClass,
        state: InspectorStateClass,
    ) -> DebugFidelityRow {
        DebugFidelityRow {
            row_id: format!("row:{prefix}:inspector_state:{}", state.as_str()),
            lane_class: lane,
            row_class: DebugFidelityRowClass::InspectorStateAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            inspector_state_class: state,
            mapping_fidelity_badge_class: MappingFidelityBadgeClass::NotApplicable,
            inspector_surface_class: InspectorSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnInspectorStateGap,
            confidence_class: DebugFidelityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_inspector_state_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_inspector_state_preserved: false,
            attests_mapping_fidelity_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn mapping_fidelity_badge_row(
        prefix: &str,
        lane: DebugFidelityLaneClass,
        badge: MappingFidelityBadgeClass,
    ) -> DebugFidelityRow {
        DebugFidelityRow {
            row_id: format!("row:{prefix}:mapping_fidelity_badge:{}", badge.as_str()),
            lane_class: lane,
            row_class: DebugFidelityRowClass::MappingFidelityBadgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            inspector_state_class: InspectorStateClass::NotApplicable,
            mapping_fidelity_badge_class: badge,
            inspector_surface_class: InspectorSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnMappingFidelityBadgeGap,
            confidence_class: DebugFidelityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_mapping_fidelity_badge_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_inspector_state_preserved: false,
            attests_mapping_fidelity_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn inspector_surface_row(
        prefix: &str,
        lane: DebugFidelityLaneClass,
        surface: InspectorSurfaceClass,
    ) -> DebugFidelityRow {
        DebugFidelityRow {
            row_id: format!("row:{prefix}:inspector_surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: DebugFidelityRowClass::InspectorSurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            inspector_state_class: InspectorStateClass::NotApplicable,
            mapping_fidelity_badge_class: MappingFidelityBadgeClass::NotApplicable,
            inspector_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnInspectorSurfaceGap,
            confidence_class: DebugFidelityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_inspector_surface_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_inspector_state_preserved: surface.requires_inspector_state_attestation(),
            attests_mapping_fidelity_preserved: surface.requires_mapping_fidelity_attestation(),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: DebugFidelityLaneClass) -> DebugFidelityRow {
        DebugFidelityRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: DebugFidelityRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            inspector_state_class: InspectorStateClass::NotApplicable,
            mapping_fidelity_badge_class: MappingFidelityBadgeClass::NotApplicable,
            inspector_surface_class: InspectorSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: DebugFidelityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:{prefix}:debug_fidelity_lineage")),
            attests_inspector_state_preserved: false,
            attests_mapping_fidelity_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> DebugFidelityConsumerProjection {
        DebugFidelityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            debug_fidelity_truth_packet_id_ref:
                "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_inspector_state_vocabulary: true,
            preserves_mapping_fidelity_badge_vocabulary: true,
            preserves_inspector_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: DebugFidelityLaneClass, prefix: &str) -> Vec<DebugFidelityRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for state in InspectorStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(inspector_state_row(prefix, lane, state));
        }
        for badge in MappingFidelityBadgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(mapping_fidelity_badge_row(prefix, lane, badge));
        }
        for surface in InspectorSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(inspector_surface_row(prefix, lane, surface));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> DebugFidelityTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(DebugFidelityLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(DebugFidelityLaneClass::RemoteHelperLane, "remote"));
        rows.extend(lane_rows(DebugFidelityLaneClass::ContainerLane, "container"));
        rows.extend(lane_rows(
            DebugFidelityLaneClass::NotebookBridgeLane,
            "notebook",
        ));
        DebugFidelityTruthPacketInput {
            packet_id: "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: DebugFidelityLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(DebugFidelityLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(
            DebugFidelityLaneClass::NotebookBridgeLane.as_str(),
            "notebook_bridge_lane"
        );
        assert_eq!(
            DebugFidelityRowClass::DebugFidelityQuality.as_str(),
            "debug_fidelity_quality"
        );
        assert_eq!(
            DebugFidelityRowClass::InspectorStateAdmission.as_str(),
            "inspector_state_admission"
        );
        assert_eq!(
            DebugFidelityRowClass::MappingFidelityBadgeAdmission.as_str(),
            "mapping_fidelity_badge_admission"
        );
        assert_eq!(
            DebugFidelityRowClass::InspectorSurfaceBinding.as_str(),
            "inspector_surface_binding"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(WedgeClass::BreakpointFidelity.as_str(), "breakpoint_fidelity");
        assert_eq!(WedgeClass::DebugConsoleFidelity.as_str(), "debug_console_fidelity");
        assert_eq!(InspectorStateClass::Live.as_str(), "live");
        assert_eq!(InspectorStateClass::PolicyBlocked.as_str(), "policy_blocked");
        assert_eq!(MappingFidelityBadgeClass::Exact.as_str(), "exact");
        assert_eq!(MappingFidelityBadgeClass::Mismatched.as_str(), "mismatched");
        assert_eq!(
            InspectorSurfaceClass::CallStackSurface.as_str(),
            "call_stack_surface"
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
            FindingKind::InspectorSurfaceMissingInspectorStateAttestation.as_str(),
            "inspector_surface_missing_inspector_state_attestation"
        );
        assert_eq!(
            FindingKind::InspectorSurfaceMissingMappingFidelityAttestation.as_str(),
            "inspector_surface_missing_mapping_fidelity_attestation"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = DebugFidelityTruthPacket::materialize(sample_input());
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
                "support:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = DebugFidelityTruthPacket::materialize(input);
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
    fn missing_inspector_state_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                DebugFidelityRowClass::InspectorStateAdmission
            ) && row.inspector_state_class == InspectorStateClass::PolicyBlocked
                && row.lane_class == DebugFidelityLaneClass::LocalLane)
        });
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingInspectorStateCoverage));
    }

    #[test]
    fn missing_mapping_fidelity_badge_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                DebugFidelityRowClass::MappingFidelityBadgeAdmission
            ) && row.mapping_fidelity_badge_class == MappingFidelityBadgeClass::Mismatched
                && row.lane_class == DebugFidelityLaneClass::LocalLane)
        });
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingMappingFidelityBadgeCoverage
        }));
    }

    #[test]
    fn inspector_surface_missing_state_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                DebugFidelityRowClass::InspectorSurfaceBinding
            ) && row.lane_class == DebugFidelityLaneClass::LocalLane
                && row.inspector_surface_class == InspectorSurfaceClass::WatchSurface
            {
                row.attests_inspector_state_preserved = false;
                break;
            }
        }
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::InspectorSurfaceMissingInspectorStateAttestation
        }));
    }

    #[test]
    fn inspector_surface_missing_mapping_fidelity_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                DebugFidelityRowClass::InspectorSurfaceBinding
            ) && row.lane_class == DebugFidelityLaneClass::LocalLane
                && row.inspector_surface_class == InspectorSurfaceClass::CallStackSurface
            {
                row.attests_mapping_fidelity_preserved = false;
                break;
            }
        }
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::InspectorSurfaceMissingMappingFidelityAttestation
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, DebugFidelityRowClass::LineageAdmission)
                && row.lane_class == DebugFidelityLaneClass::LocalLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = DebugFidelityTruthPacket::materialize(input);
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
        let packet = DebugFidelityTruthPacket::materialize(input);
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
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_inspector_state_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_inspector_state_vocabulary = false;
            }
        }
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::InspectorStateVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = DebugFidelityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
