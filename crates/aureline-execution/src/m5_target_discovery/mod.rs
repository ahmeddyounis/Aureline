//! Canonical M5 target-discovery matrix with a non-inheriting confidence gate
//! that keeps every execution lane honest about how its target was found and how
//! certain that answer is.
//!
//! Each [`LaneDiscoveryRow`] names one M5 execution lane that resolves a target —
//! build targets, notebook kernels, preview runtimes, profiler sessions, framework
//! generators, request/browser-runtime actions, API runtimes, and incident or
//! pipeline-linked reruns — and answers, for that lane, how the target was
//! discovered ([`DiscoveryPath`]), how the discovery was verified
//! ([`VerificationState`]), whether the result is exact or approximate
//! ([`Exactness`]), whether a previously selected target changed ([`ChangeTrigger`])
//! and was reviewed ([`DiffReviewState`]), whether a target-graph snapshot exists
//! ([`TargetGraphState`]), and whether discovery provenance carried into the
//! consuming action ([`ProvenanceState`]). The row then publishes a
//! [`DiscoveryConfidence`] no input can exceed.
//!
//! The [`DiscoveryConfidence`] a lane may publish is the weakest ceiling implied by
//! its observed states, so an undiscovered target, a low-verification signal, a
//! heuristic guess, an unreviewed target change, dropped provenance, or a missing
//! target-graph snapshot all narrow, flag, or withhold the published label
//! automatically. The guardrail this enforces: an approximate or heuristic target can
//! never masquerade as a confident exact target merely because it produced a runnable
//! fallback. The [`DiscoveryDecision`] records the gate's action and the recomputed
//! [`NarrowingReason`]s explain it; all are validated against the gate so a lane can
//! neither overstate its confidence nor silently swap a target out from under the
//! user.
//!
//! The packet is checked in at `artifacts/execution/m5/m5-target-discovery.json` and
//! embedded here. It is metadata-only: every field is a typed state or an opaque ref,
//! and it carries no credential bodies, raw provider payloads, host tokens, or
//! control-plane secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 target-discovery matrix schema version.
pub const M5_TARGET_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_TARGET_DISCOVERY_RECORD_KIND: &str = "m5_target_discovery_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_TARGET_DISCOVERY_PATH: &str = "artifacts/execution/m5/m5-target-discovery.json";

/// Embedded checked-in packet JSON.
pub const M5_TARGET_DISCOVERY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/execution/m5/m5-target-discovery.json"
));

/// An M5 execution lane that resolves a target the matrix makes claims about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryLane {
    /// Build-target selection lane.
    BuildTarget,
    /// Notebook kernel selection lane.
    NotebookKernel,
    /// Preview-runtime selection lane.
    PreviewRuntime,
    /// Profiler-session capture lane.
    ProfilerSession,
    /// Framework generator/tooling lane.
    FrameworkGenerator,
    /// Request/browser-runtime action lane.
    RequestRuntime,
    /// API-runtime selection lane.
    ApiRuntime,
    /// Incident or pipeline-linked rerun lane.
    IncidentRerun,
}

impl DiscoveryLane {
    /// Every discovery lane, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BuildTarget,
        Self::NotebookKernel,
        Self::PreviewRuntime,
        Self::ProfilerSession,
        Self::FrameworkGenerator,
        Self::RequestRuntime,
        Self::ApiRuntime,
        Self::IncidentRerun,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildTarget => "build_target",
            Self::NotebookKernel => "notebook_kernel",
            Self::PreviewRuntime => "preview_runtime",
            Self::ProfilerSession => "profiler_session",
            Self::FrameworkGenerator => "framework_generator",
            Self::RequestRuntime => "request_runtime",
            Self::ApiRuntime => "api_runtime",
            Self::IncidentRerun => "incident_rerun",
        }
    }
}

/// Strength of a lane's published target-discovery confidence.
///
/// Ordered low-to-high by [`DiscoveryConfidence::rank`]: an
/// [`DiscoveryConfidence::Unresolved`] lane carries no usable target, and an
/// [`DiscoveryConfidence::Exact`] lane carries a confirmed, verified "this is the
/// target" answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryConfidence {
    /// Confirmed, verified target identity.
    Exact,
    /// Derived from a structured build-event stream.
    Structured,
    /// Reconstructed from a structured import.
    Imported,
    /// Heuristic guess only.
    Heuristic,
    /// No usable target was resolved.
    Unresolved,
}

impl DiscoveryConfidence {
    /// Every discovery confidence, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Exact,
        Self::Structured,
        Self::Imported,
        Self::Heuristic,
        Self::Unresolved,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Structured => "structured",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
            Self::Unresolved => "unresolved",
        }
    }

    /// Monotonic rank; higher means a more certain target.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unresolved => 0,
            Self::Heuristic => 1,
            Self::Imported => 2,
            Self::Structured => 3,
            Self::Exact => 4,
        }
    }

    /// The weaker (lower-rank) of two confidences.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }

    /// The exactness label this confidence carries.
    pub const fn exactness(self) -> Exactness {
        match self {
            Self::Exact => Exactness::Exact,
            _ => Exactness::Approximate,
        }
    }
}

/// How the lane's target was discovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryPath {
    /// Resolved by a native build/runtime adapter.
    NativeAdapter,
    /// Resolved over a live protocol handshake.
    ProtocolBacked,
    /// Resolved from a structured build-event stream; caps at structured.
    BuildEventStream,
    /// Reconstructed from a structured import; caps at imported.
    StructuredImport,
    /// Inferred by heuristic only; caps at heuristic.
    Heuristic,
    /// Not discovered; identity unknown.
    Undiscovered,
}

impl DiscoveryPath {
    /// Every discovery path, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NativeAdapter,
        Self::ProtocolBacked,
        Self::BuildEventStream,
        Self::StructuredImport,
        Self::Heuristic,
        Self::Undiscovered,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeAdapter => "native_adapter",
            Self::ProtocolBacked => "protocol_backed",
            Self::BuildEventStream => "build_event_stream",
            Self::StructuredImport => "structured_import",
            Self::Heuristic => "heuristic",
            Self::Undiscovered => "undiscovered",
        }
    }

    /// Highest confidence this discovery path permits a lane to publish.
    pub const fn confidence_ceiling(self) -> DiscoveryConfidence {
        match self {
            Self::NativeAdapter | Self::ProtocolBacked => DiscoveryConfidence::Exact,
            Self::BuildEventStream => DiscoveryConfidence::Structured,
            Self::StructuredImport => DiscoveryConfidence::Imported,
            Self::Heuristic => DiscoveryConfidence::Heuristic,
            Self::Undiscovered => DiscoveryConfidence::Unresolved,
        }
    }

    /// Whether this path raises the [`NarrowingReason::TargetUnresolved`] trigger.
    pub const fn is_unresolved_trigger(self) -> bool {
        matches!(self, Self::Undiscovered)
    }

    /// Whether this path raises the [`NarrowingReason::HeuristicFallback`] trigger.
    pub const fn is_heuristic_trigger(self) -> bool {
        matches!(self, Self::Heuristic)
    }
}

/// How thoroughly the discovered target was verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    /// Verified against the live target.
    Verified,
    /// Corroborated by multiple independent signals.
    Corroborated,
    /// Backed by a single signal; caps at structured.
    SingleSignal,
    /// Unverified; caps at heuristic.
    Unverified,
}

impl VerificationState {
    /// Every verification state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Corroborated,
        Self::SingleSignal,
        Self::Unverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Corroborated => "corroborated",
            Self::SingleSignal => "single_signal",
            Self::Unverified => "unverified",
        }
    }

    /// Highest confidence this verification state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> DiscoveryConfidence {
        match self {
            Self::Verified | Self::Corroborated => DiscoveryConfidence::Exact,
            Self::SingleSignal => DiscoveryConfidence::Structured,
            Self::Unverified => DiscoveryConfidence::Heuristic,
        }
    }

    /// Whether this state raises the [`NarrowingReason::LowVerification`] trigger.
    pub const fn is_low_trigger(self) -> bool {
        matches!(self, Self::Unverified)
    }
}

/// Whether the lane's published target is exact or approximate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Exactness {
    /// A confirmed exact target.
    Exact,
    /// An approximate target (structured, imported, heuristic, or unresolved).
    Approximate,
}

impl Exactness {
    /// Every exactness label, in declaration order.
    pub const ALL: [Self; 2] = [Self::Exact, Self::Approximate];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
        }
    }
}

/// Why a previously selected target was re-discovered, if at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeTrigger {
    /// The target is unchanged since last selection.
    Unchanged,
    /// The workspace layout changed.
    WorkspaceChanged,
    /// The active profile changed.
    ProfileChanged,
    /// The build metadata changed.
    BuildMetadataChanged,
    /// The managed/runtime state changed.
    ManagedRuntimeChanged,
    /// The user manually re-selected the target.
    ManualReselection,
}

impl ChangeTrigger {
    /// Every change trigger, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Unchanged,
        Self::WorkspaceChanged,
        Self::ProfileChanged,
        Self::BuildMetadataChanged,
        Self::ManagedRuntimeChanged,
        Self::ManualReselection,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::WorkspaceChanged => "workspace_changed",
            Self::ProfileChanged => "profile_changed",
            Self::BuildMetadataChanged => "build_metadata_changed",
            Self::ManagedRuntimeChanged => "managed_runtime_changed",
            Self::ManualReselection => "manual_reselection",
        }
    }

    /// Whether the target changed and therefore needs a discovery diff.
    pub const fn is_change(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

/// Review state of a target change's discovery diff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffReviewState {
    /// No change, so no diff to review.
    NotApplicable,
    /// The change is surfaced and pending review; caps at imported.
    PendingReview,
    /// The change was reviewed and accepted.
    ReviewedAccepted,
    /// The change was reviewed and rejected; caps at unresolved.
    ReviewedRejected,
    /// The change was applied without review; caps at heuristic.
    AutoAppliedUnreviewed,
}

impl DiffReviewState {
    /// Every diff-review state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotApplicable,
        Self::PendingReview,
        Self::ReviewedAccepted,
        Self::ReviewedRejected,
        Self::AutoAppliedUnreviewed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::PendingReview => "pending_review",
            Self::ReviewedAccepted => "reviewed_accepted",
            Self::ReviewedRejected => "reviewed_rejected",
            Self::AutoAppliedUnreviewed => "auto_applied_unreviewed",
        }
    }

    /// Highest confidence this diff-review state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> DiscoveryConfidence {
        match self {
            Self::NotApplicable | Self::ReviewedAccepted => DiscoveryConfidence::Exact,
            Self::PendingReview => DiscoveryConfidence::Imported,
            Self::AutoAppliedUnreviewed => DiscoveryConfidence::Heuristic,
            Self::ReviewedRejected => DiscoveryConfidence::Unresolved,
        }
    }

    /// Whether this state describes a reviewable target change.
    pub const fn requires_change(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }

    /// Whether the gate should flag the change for review rather than publish it.
    pub const fn is_flaggable(self) -> bool {
        matches!(self, Self::PendingReview | Self::AutoAppliedUnreviewed)
    }

    /// Whether this state raises the [`NarrowingReason::UnreviewedTargetChange`]
    /// trigger.
    ///
    /// Only a silently auto-applied change raises the headline trigger; a
    /// pending-review change is surfaced for review and lowers the ceiling without
    /// being a violation.
    pub const fn is_unreviewed_change_trigger(self) -> bool {
        matches!(self, Self::AutoAppliedUnreviewed)
    }
}

/// Status of the lane's target-graph snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetGraphState {
    /// A current snapshot exists.
    Snapshotted,
    /// A snapshot exists but is stale; caps at structured.
    StaleSnapshot,
    /// No snapshot exists; caps at imported.
    MissingSnapshot,
    /// No target graph applies to this lane.
    NotApplicable,
}

impl TargetGraphState {
    /// Every target-graph state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Snapshotted,
        Self::StaleSnapshot,
        Self::MissingSnapshot,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshotted => "snapshotted",
            Self::StaleSnapshot => "stale_snapshot",
            Self::MissingSnapshot => "missing_snapshot",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest confidence this target-graph state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> DiscoveryConfidence {
        match self {
            Self::Snapshotted | Self::NotApplicable => DiscoveryConfidence::Exact,
            Self::StaleSnapshot => DiscoveryConfidence::Structured,
            Self::MissingSnapshot => DiscoveryConfidence::Imported,
        }
    }

    /// Whether this state raises the [`NarrowingReason::MissingGraphSnapshot`]
    /// trigger.
    pub const fn is_missing_trigger(self) -> bool {
        matches!(self, Self::MissingSnapshot)
    }
}

/// Whether discovery provenance carried into the consuming action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceState {
    /// Provenance carried fully into the consuming action.
    Propagated,
    /// Provenance carried only partially; caps at imported.
    Partial,
    /// Provenance was dropped; caps at heuristic.
    Dropped,
    /// No provenance handoff applies to this lane.
    NotApplicable,
}

impl ProvenanceState {
    /// Every provenance state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Propagated,
        Self::Partial,
        Self::Dropped,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Propagated => "propagated",
            Self::Partial => "partial",
            Self::Dropped => "dropped",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest confidence this provenance state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> DiscoveryConfidence {
        match self {
            Self::Propagated | Self::NotApplicable => DiscoveryConfidence::Exact,
            Self::Partial => DiscoveryConfidence::Imported,
            Self::Dropped => DiscoveryConfidence::Heuristic,
        }
    }

    /// Whether this state raises the [`NarrowingReason::ProvenanceDropped`] trigger.
    pub const fn is_dropped_trigger(self) -> bool {
        matches!(self, Self::Dropped)
    }
}

/// A headline reason the discovery gate narrows a lane.
///
/// These are the canonical target-discovery release-control triggers: an
/// unresolved target, low verification, a heuristic fallback, an unreviewed target
/// change, dropped provenance, and a missing target-graph snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The target could not be discovered.
    TargetUnresolved,
    /// The discovery was not verified.
    LowVerification,
    /// The target rests on a heuristic fallback.
    HeuristicFallback,
    /// A target change was applied without review.
    UnreviewedTargetChange,
    /// Discovery provenance was dropped before the consuming action.
    ProvenanceDropped,
    /// No target-graph snapshot is available.
    MissingGraphSnapshot,
}

impl NarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TargetUnresolved,
        Self::LowVerification,
        Self::HeuristicFallback,
        Self::UnreviewedTargetChange,
        Self::ProvenanceDropped,
        Self::MissingGraphSnapshot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetUnresolved => "target_unresolved",
            Self::LowVerification => "low_verification",
            Self::HeuristicFallback => "heuristic_fallback",
            Self::UnreviewedTargetChange => "unreviewed_target_change",
            Self::ProvenanceDropped => "provenance_dropped",
            Self::MissingGraphSnapshot => "missing_graph_snapshot",
        }
    }
}

/// The action the discovery gate takes on a lane relative to an exact target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryDecision {
    /// No narrowing; the lane publishes an exact target.
    Publish,
    /// Publish the target, but at a narrowed confidence label.
    Narrow,
    /// Surface the target change for review before it is adopted.
    FlagForReview,
    /// Withhold the target entirely; no usable target was resolved.
    Withhold,
}

impl DiscoveryDecision {
    /// Every discovery decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Publish,
        Self::Narrow,
        Self::FlagForReview,
        Self::Withhold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::Narrow => "narrow",
            Self::FlagForReview => "flag_for_review",
            Self::Withhold => "withhold",
        }
    }

    /// Whether the gate narrowed, flagged, or withheld the lane.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Publish)
    }
}

/// One discovery row for an M5 execution lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneDiscoveryRow {
    /// Stable lane-discovery id.
    pub lane_id: String,
    /// M5 execution lane this row governs.
    pub discovery_lane: DiscoveryLane,
    /// How the target was discovered.
    pub discovery_path: DiscoveryPath,
    /// How thoroughly the discovery was verified.
    pub verification_state: VerificationState,
    /// Confidence the lane's own evidence asserts, before the gate.
    pub declared_confidence: DiscoveryConfidence,
    /// Confidence actually published after the gate narrows the lane.
    ///
    /// Must equal [`LaneDiscoveryRow::effective_confidence`].
    pub published_confidence: DiscoveryConfidence,
    /// Exactness label; must equal [`LaneDiscoveryRow::derived_exactness`].
    pub exactness: Exactness,
    /// Why a previously selected target was re-discovered, if at all.
    pub change_trigger: ChangeTrigger,
    /// Review state of the target change's discovery diff.
    pub diff_review_state: DiffReviewState,
    /// Status of the lane's target-graph snapshot.
    pub target_graph_state: TargetGraphState,
    /// Whether discovery provenance carried into the consuming action.
    pub provenance_state: ProvenanceState,
    /// Decision the gate takes; must equal the recomputed decision.
    pub discovery_decision: DiscoveryDecision,
    /// Headline narrowing reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Ref to the currently selected target identity.
    pub selected_target_ref: String,
    /// Ref to the previously selected target; required when the target changed.
    #[serde(default)]
    pub previous_target_ref: String,
    /// Ref to the discovery diff; required when the target changed.
    #[serde(default)]
    pub discovery_diff_ref: String,
    /// Ref to the lane's target-graph snapshot.
    pub target_graph_ref: String,
    /// Ref to the discovery-provenance record carried into the consuming action.
    pub provenance_ref: String,
    /// Ref to the in-product execution this discovery produced.
    pub execution_ref: String,
    /// Ref binding this row into desktop, CLI, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl LaneDiscoveryRow {
    /// The confidence the lane's own evidence asserted, before environmental
    /// narrowing.
    pub fn capability_floor(&self) -> DiscoveryConfidence {
        self.declared_confidence
    }

    /// The confidence the gate permits this lane to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the discovery
    /// path, verification state, diff-review state, provenance state, and
    /// target-graph state, so an undiscovered target, low verification, a heuristic
    /// guess, an unreviewed change, dropped provenance, or a missing target-graph
    /// snapshot can never publish an exact target.
    pub fn effective_confidence(&self) -> DiscoveryConfidence {
        self.capability_floor()
            .min(self.discovery_path.confidence_ceiling())
            .min(self.verification_state.confidence_ceiling())
            .min(self.diff_review_state.confidence_ceiling())
            .min(self.provenance_state.confidence_ceiling())
            .min(self.target_graph_state.confidence_ceiling())
    }

    /// The exactness label implied by the effective confidence.
    pub fn derived_exactness(&self) -> Exactness {
        self.effective_confidence().exactness()
    }

    /// The headline narrowing reasons recomputed from the lane's observed states.
    pub fn computed_narrowing_reasons(&self) -> Vec<NarrowingReason> {
        let mut reasons = Vec::new();
        if self.discovery_path.is_unresolved_trigger() {
            reasons.push(NarrowingReason::TargetUnresolved);
        }
        if self.verification_state.is_low_trigger() {
            reasons.push(NarrowingReason::LowVerification);
        }
        if self.discovery_path.is_heuristic_trigger() {
            reasons.push(NarrowingReason::HeuristicFallback);
        }
        if self.diff_review_state.is_unreviewed_change_trigger() {
            reasons.push(NarrowingReason::UnreviewedTargetChange);
        }
        if self.provenance_state.is_dropped_trigger() {
            reasons.push(NarrowingReason::ProvenanceDropped);
        }
        if self.target_graph_state.is_missing_trigger() {
            reasons.push(NarrowingReason::MissingGraphSnapshot);
        }
        reasons
    }

    /// The decision the gate must record for this lane.
    ///
    /// An unresolved target is withheld; an unreviewed or pending target change is
    /// flagged for review before adoption; a confidence below exact narrows; and a
    /// clean exact target publishes.
    pub fn required_decision(&self) -> DiscoveryDecision {
        let effective = self.effective_confidence();
        if effective == DiscoveryConfidence::Unresolved {
            DiscoveryDecision::Withhold
        } else if self.diff_review_state.is_flaggable() {
            DiscoveryDecision::FlagForReview
        } else if effective == DiscoveryConfidence::Exact {
            DiscoveryDecision::Publish
        } else {
            DiscoveryDecision::Narrow
        }
    }

    /// Whether the lane may publish an exact target.
    pub fn is_publishable(&self) -> bool {
        self.effective_confidence() == DiscoveryConfidence::Exact
    }

    /// Whether the lane carries its own non-empty target, graph, provenance,
    /// execution, and support-export refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.selected_target_ref.trim().is_empty()
            && !self.target_graph_ref.trim().is_empty()
            && !self.provenance_ref.trim().is_empty()
            && !self.execution_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published confidence, exactness, decision, and narrowing
    /// reasons all agree with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_confidence == self.effective_confidence()
            && self.exactness == self.derived_exactness()
            && self.discovery_decision == self.required_decision()
            && self.narrowing_reasons == self.computed_narrowing_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5TargetDiscoverySummary {
    /// Total lane rows.
    pub total_lanes: usize,
    /// Number of claimed lanes.
    pub lane_count: usize,
    /// Lanes published with an exact target.
    pub exact_lanes: usize,
    /// Lanes published with a structured target.
    pub structured_lanes: usize,
    /// Lanes published with an imported target.
    pub imported_lanes: usize,
    /// Lanes published with a heuristic target.
    pub heuristic_lanes: usize,
    /// Lanes whose target is unresolved.
    pub unresolved_lanes: usize,
    /// Lanes that may publish an exact target.
    pub publishable_lanes: usize,
    /// Lanes the gate narrowed to a lower confidence.
    pub narrowed_lanes: usize,
    /// Lanes the gate flagged for review.
    pub flagged_lanes: usize,
    /// Lanes the gate withheld entirely.
    pub withheld_lanes: usize,
    /// Lanes whose published target is approximate.
    pub approximate_lanes: usize,
    /// Lanes whose target changed since last selection.
    pub changed_lanes: usize,
    /// Lanes whose target change was applied without review.
    pub unreviewed_change_lanes: usize,
    /// Lanes with unverified discovery.
    pub low_verification_lanes: usize,
    /// Lanes whose provenance was dropped.
    pub dropped_provenance_lanes: usize,
    /// Lanes carrying at least one narrowing reason.
    pub lanes_with_narrowing_reasons: usize,
}

/// A redaction-safe export row projected from a lane discovery row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TargetDiscoveryExportRow {
    /// Lane-discovery id.
    pub lane_id: String,
    /// Discovery-lane token.
    pub discovery_lane: String,
    /// Discovery-path token.
    pub discovery_path: String,
    /// Verification-state token.
    pub verification_state: String,
    /// Declared-confidence token.
    pub declared_confidence: String,
    /// Published-confidence token.
    pub published_confidence: String,
    /// Exactness token.
    pub exactness: String,
    /// Change-trigger token.
    pub change_trigger: String,
    /// Diff-review-state token.
    pub diff_review_state: String,
    /// Target-graph-state token.
    pub target_graph_state: String,
    /// Provenance-state token.
    pub provenance_state: String,
    /// Discovery-decision token.
    pub discovery_decision: String,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Selected-target ref.
    pub selected_target_ref: String,
    /// Execution ref the discovery produced.
    pub execution_ref: String,
    /// Whether the lane publishes an exact target.
    pub exact_target: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TargetDiscoveryExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub lanes: Vec<M5TargetDiscoveryExportRow>,
    /// Whether every lane's published confidence and decision agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that may publish an exact target.
    pub publishable_count: usize,
    /// Lanes the gate narrowed, flagged, or withheld.
    pub narrowed_count: usize,
    /// Lanes the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 target-discovery matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5TargetDiscoveryMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Claimed lanes; one row per lane.
    pub discovery_lanes: Vec<DiscoveryLane>,
    /// Closed discovery-confidence vocabulary.
    pub discovery_confidences: Vec<DiscoveryConfidence>,
    /// Closed discovery-path vocabulary.
    pub discovery_paths: Vec<DiscoveryPath>,
    /// Closed verification-state vocabulary.
    pub verification_states: Vec<VerificationState>,
    /// Closed exactness vocabulary.
    pub exactness_labels: Vec<Exactness>,
    /// Closed change-trigger vocabulary.
    pub change_triggers: Vec<ChangeTrigger>,
    /// Closed diff-review-state vocabulary.
    pub diff_review_states: Vec<DiffReviewState>,
    /// Closed target-graph-state vocabulary.
    pub target_graph_states: Vec<TargetGraphState>,
    /// Closed provenance-state vocabulary.
    pub provenance_states: Vec<ProvenanceState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed discovery-decision vocabulary.
    pub discovery_decisions: Vec<DiscoveryDecision>,
    /// Discovery rows, one per claimed lane.
    #[serde(default)]
    pub lanes: Vec<LaneDiscoveryRow>,
    /// Summary counts.
    pub summary: M5TargetDiscoverySummary,
}

impl M5TargetDiscoveryMatrix {
    /// Returns the row for a claimed lane.
    pub fn lane(&self, lane: DiscoveryLane) -> Option<&LaneDiscoveryRow> {
        self.lanes.iter().find(|l| l.discovery_lane == lane)
    }

    /// Lanes that may publish an exact target.
    pub fn publishable_lanes(&self) -> impl Iterator<Item = &LaneDiscoveryRow> {
        self.lanes.iter().filter(|l| l.is_publishable())
    }

    /// Lanes the gate narrowed, flagged, or withheld in any way.
    pub fn narrowed_lanes(&self) -> impl Iterator<Item = &LaneDiscoveryRow> {
        self.lanes
            .iter()
            .filter(|l| l.required_decision().is_narrowed())
    }

    /// Lanes the gate withheld entirely.
    pub fn withheld_lanes(&self) -> impl Iterator<Item = &LaneDiscoveryRow> {
        self.lanes
            .iter()
            .filter(|l| l.required_decision() == DiscoveryDecision::Withhold)
    }

    /// Whether every lane's stored published confidence, exactness, decision, and
    /// reasons agree with the recomputed gate decision.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.lanes.iter().all(|l| l.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5TargetDiscoverySummary {
        let count_published = |confidence: DiscoveryConfidence| {
            self.lanes
                .iter()
                .filter(|l| l.published_confidence == confidence)
                .count()
        };
        let count_decision = |decision: DiscoveryDecision| {
            self.lanes
                .iter()
                .filter(|l| l.discovery_decision == decision)
                .count()
        };
        M5TargetDiscoverySummary {
            total_lanes: self.lanes.len(),
            lane_count: self.discovery_lanes.len(),
            exact_lanes: count_published(DiscoveryConfidence::Exact),
            structured_lanes: count_published(DiscoveryConfidence::Structured),
            imported_lanes: count_published(DiscoveryConfidence::Imported),
            heuristic_lanes: count_published(DiscoveryConfidence::Heuristic),
            unresolved_lanes: count_published(DiscoveryConfidence::Unresolved),
            publishable_lanes: self.publishable_lanes().count(),
            narrowed_lanes: count_decision(DiscoveryDecision::Narrow),
            flagged_lanes: count_decision(DiscoveryDecision::FlagForReview),
            withheld_lanes: count_decision(DiscoveryDecision::Withhold),
            approximate_lanes: self
                .lanes
                .iter()
                .filter(|l| l.exactness == Exactness::Approximate)
                .count(),
            changed_lanes: self
                .lanes
                .iter()
                .filter(|l| l.change_trigger.is_change())
                .count(),
            unreviewed_change_lanes: self
                .lanes
                .iter()
                .filter(|l| l.diff_review_state.is_unreviewed_change_trigger())
                .count(),
            low_verification_lanes: self
                .lanes
                .iter()
                .filter(|l| l.verification_state.is_low_trigger())
                .count(),
            dropped_provenance_lanes: self
                .lanes
                .iter()
                .filter(|l| l.provenance_state.is_dropped_trigger())
                .count(),
            lanes_with_narrowing_reasons: self
                .lanes
                .iter()
                .filter(|l| !l.narrowing_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — desktop and CLI
    /// target pickers, notebook/preview/profiler/framework/request/incident lanes,
    /// support exports, and release/public-truth packets — render instead of
    /// restating how a target was discovered by hand.
    pub fn export_projection(&self) -> M5TargetDiscoveryExportProjection {
        let lanes = self
            .lanes
            .iter()
            .map(|l| M5TargetDiscoveryExportRow {
                lane_id: l.lane_id.clone(),
                discovery_lane: l.discovery_lane.as_str().to_owned(),
                discovery_path: l.discovery_path.as_str().to_owned(),
                verification_state: l.verification_state.as_str().to_owned(),
                declared_confidence: l.declared_confidence.as_str().to_owned(),
                published_confidence: l.published_confidence.as_str().to_owned(),
                exactness: l.exactness.as_str().to_owned(),
                change_trigger: l.change_trigger.as_str().to_owned(),
                diff_review_state: l.diff_review_state.as_str().to_owned(),
                target_graph_state: l.target_graph_state.as_str().to_owned(),
                provenance_state: l.provenance_state.as_str().to_owned(),
                discovery_decision: l.discovery_decision.as_str().to_owned(),
                narrowing_reasons: l
                    .narrowing_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                selected_target_ref: l.selected_target_ref.clone(),
                execution_ref: l.execution_ref.clone(),
                exact_target: l.is_publishable(),
                summary: format!(
                    "{}: path {}, verification {}, declared {}, published {} ({}, {}), change {}, diff {}, graph {}, provenance {}",
                    l.discovery_lane.as_str(),
                    l.discovery_path.as_str(),
                    l.verification_state.as_str(),
                    l.declared_confidence.as_str(),
                    l.published_confidence.as_str(),
                    l.exactness.as_str(),
                    l.discovery_decision.as_str(),
                    l.change_trigger.as_str(),
                    l.diff_review_state.as_str(),
                    l.target_graph_state.as_str(),
                    l.provenance_state.as_str()
                ),
            })
            .collect();
        M5TargetDiscoveryExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            lanes,
            all_lanes_gate_consistent: self.all_lanes_gate_consistent(),
            publishable_count: self.publishable_lanes().count(),
            narrowed_count: self.narrowed_lanes().count(),
            withheld_count: self.withheld_lanes().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5TargetDiscoveryViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<DiscoveryLane> = self.discovery_lanes.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for row in &self.lanes {
            if !seen_ids.insert(row.lane_id.clone()) {
                violations.push(M5TargetDiscoveryViolation::DuplicateLaneId {
                    lane_id: row.lane_id.clone(),
                });
            }
            if !seen_lanes.insert(row.discovery_lane) {
                violations.push(M5TargetDiscoveryViolation::DuplicateLaneRow {
                    lane: row.discovery_lane.as_str(),
                });
            }
            if !claimed.contains(&row.discovery_lane) {
                violations.push(M5TargetDiscoveryViolation::UnclaimedLaneRow {
                    lane_id: row.lane_id.clone(),
                    lane: row.discovery_lane.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed lane must carry its own row, so a lane never inherits a
        // confident target from an adjacent exact one.
        for &lane in &self.discovery_lanes {
            if !seen_lanes.contains(&lane) {
                violations.push(M5TargetDiscoveryViolation::MissingLaneRow {
                    lane: lane.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5TargetDiscoveryViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5TargetDiscoveryViolation>) {
        if self.schema_version != M5_TARGET_DISCOVERY_SCHEMA_VERSION {
            violations.push(M5TargetDiscoveryViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_TARGET_DISCOVERY_RECORD_KIND {
            violations.push(M5TargetDiscoveryViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5TargetDiscoveryViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "discovery_lanes",
                self.discovery_lanes == DiscoveryLane::ALL.to_vec(),
            ),
            (
                "discovery_confidences",
                self.discovery_confidences == DiscoveryConfidence::ALL.to_vec(),
            ),
            (
                "discovery_paths",
                self.discovery_paths == DiscoveryPath::ALL.to_vec(),
            ),
            (
                "verification_states",
                self.verification_states == VerificationState::ALL.to_vec(),
            ),
            (
                "exactness_labels",
                self.exactness_labels == Exactness::ALL.to_vec(),
            ),
            (
                "change_triggers",
                self.change_triggers == ChangeTrigger::ALL.to_vec(),
            ),
            (
                "diff_review_states",
                self.diff_review_states == DiffReviewState::ALL.to_vec(),
            ),
            (
                "target_graph_states",
                self.target_graph_states == TargetGraphState::ALL.to_vec(),
            ),
            (
                "provenance_states",
                self.provenance_states == ProvenanceState::ALL.to_vec(),
            ),
            (
                "narrowing_reasons",
                self.narrowing_reasons == NarrowingReason::ALL.to_vec(),
            ),
            (
                "discovery_decisions",
                self.discovery_decisions == DiscoveryDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5TargetDiscoveryViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &LaneDiscoveryRow,
        violations: &mut Vec<M5TargetDiscoveryViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &row.lane_id),
            ("selected_target_ref", &row.selected_target_ref),
            ("target_graph_ref", &row.target_graph_ref),
            ("provenance_ref", &row.provenance_ref),
            ("execution_ref", &row.execution_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5TargetDiscoveryViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // A change trigger and its diff-review state must agree: a changed target
        // carries a reviewable diff, and an unchanged one carries none.
        if row.change_trigger.is_change() != row.diff_review_state.requires_change() {
            violations.push(M5TargetDiscoveryViolation::ChangeReviewMismatch {
                lane_id: row.lane_id.clone(),
                change_trigger: row.change_trigger.as_str(),
                diff_review_state: row.diff_review_state.as_str(),
            });
        }

        // A changed target must carry its previous-target and discovery-diff refs so
        // the change is reviewable instead of silently replacing the current target.
        if row.change_trigger.is_change() {
            for (field, value) in [
                ("previous_target_ref", &row.previous_target_ref),
                ("discovery_diff_ref", &row.discovery_diff_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5TargetDiscoveryViolation::EmptyField {
                        id: row.lane_id.clone(),
                        field_name: field,
                    });
                }
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrowing_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5TargetDiscoveryViolation::DuplicateNarrowingReason {
                    lane_id: row.lane_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published confidence must equal the gate's recomputed ceiling, so a
        // heuristic or approximate target can never masquerade as a confident exact
        // one.
        let effective = row.effective_confidence();
        if row.published_confidence != effective {
            violations.push(M5TargetDiscoveryViolation::OverstatedConfidence {
                lane_id: row.lane_id.clone(),
                published: row.published_confidence.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded exactness must equal the exactness implied by the effective
        // confidence.
        let derived = row.derived_exactness();
        if row.exactness != derived {
            violations.push(M5TargetDiscoveryViolation::ExactnessMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.exactness.as_str(),
                derived: derived.as_str(),
            });
        }

        // The recorded decision must match the gate's recomputed decision.
        let required = row.required_decision();
        if row.discovery_decision != required {
            violations.push(M5TargetDiscoveryViolation::DecisionMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.discovery_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded narrowing reasons must equal the reasons recomputed from the
        // observed states, so a narrowing can never be asserted or hidden by hand.
        let computed = row.computed_narrowing_reasons();
        if row.narrowing_reasons != computed {
            violations.push(M5TargetDiscoveryViolation::NarrowingReasonsMismatch {
                lane_id: row.lane_id.clone(),
            });
        }

        // A publishable lane must be genuinely clean: an exact-ceiling discovery
        // path, verification, diff-review, provenance, and target-graph state,
        // current evidence in the form of an exact capability floor, and no narrowing
        // reason. This is the non-inheritance guardrail.
        if row.is_publishable()
            && (row.discovery_path.confidence_ceiling() != DiscoveryConfidence::Exact
                || row.verification_state.confidence_ceiling() != DiscoveryConfidence::Exact
                || row.diff_review_state.confidence_ceiling() != DiscoveryConfidence::Exact
                || row.provenance_state.confidence_ceiling() != DiscoveryConfidence::Exact
                || row.target_graph_state.confidence_ceiling() != DiscoveryConfidence::Exact
                || row.capability_floor() != DiscoveryConfidence::Exact
                || !row.narrowing_reasons.is_empty())
        {
            violations.push(M5TargetDiscoveryViolation::PublishedLaneNotClean {
                lane_id: row.lane_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 target-discovery packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TargetDiscoveryViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A lane-discovery id appears more than once.
    DuplicateLaneId {
        /// Duplicate lane id.
        lane_id: String,
    },
    /// A claimed lane carries more than one row.
    DuplicateLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A claimed lane has no row.
    MissingLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A row covers a lane the packet does not claim.
    UnclaimedLaneRow {
        /// Row id.
        lane_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A row's change trigger and diff-review state disagree.
    ChangeReviewMismatch {
        /// Row id.
        lane_id: String,
        /// Change-trigger token.
        change_trigger: &'static str,
        /// Diff-review-state token.
        diff_review_state: &'static str,
    },
    /// A row lists a narrowing reason more than once.
    DuplicateNarrowingReason {
        /// Row id.
        lane_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A lane publishes a confidence beyond what its evidence supports.
    OverstatedConfidence {
        /// Row id.
        lane_id: String,
        /// Published confidence token.
        published: &'static str,
        /// Computed effective confidence token.
        computed: &'static str,
    },
    /// A lane's exactness label disagrees with its effective confidence.
    ExactnessMismatch {
        /// Row id.
        lane_id: String,
        /// Declared exactness token.
        declared: &'static str,
        /// Derived exactness token.
        derived: &'static str,
    },
    /// A lane's decision disagrees with its gate decision.
    DecisionMismatch {
        /// Row id.
        lane_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A lane's narrowing reasons disagree with the recomputed reasons.
    NarrowingReasonsMismatch {
        /// Row id.
        lane_id: String,
    },
    /// A publishable lane still carries a narrowing reason or a non-clean state.
    PublishedLaneNotClean {
        /// Row id.
        lane_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5TargetDiscoveryViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateLaneId { lane_id } => {
                write!(f, "duplicate lane id {lane_id}")
            }
            Self::DuplicateLaneRow { lane } => {
                write!(f, "duplicate row for lane {lane}")
            }
            Self::MissingLaneRow { lane } => {
                write!(f, "missing row for claimed lane {lane}")
            }
            Self::UnclaimedLaneRow { lane_id, lane } => {
                write!(f, "row {lane_id} covers unclaimed lane {lane}")
            }
            Self::ChangeReviewMismatch {
                lane_id,
                change_trigger,
                diff_review_state,
            } => {
                write!(
                    f,
                    "row {lane_id} change trigger {change_trigger} disagrees with diff-review state {diff_review_state}"
                )
            }
            Self::DuplicateNarrowingReason { lane_id, reason } => {
                write!(f, "row {lane_id} repeats narrowing reason {reason}")
            }
            Self::OverstatedConfidence {
                lane_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {lane_id} publishes confidence {published} but the gate computes {computed}"
                )
            }
            Self::ExactnessMismatch {
                lane_id,
                declared,
                derived,
            } => {
                write!(
                    f,
                    "row {lane_id} records exactness {declared} but the gate derives {derived}"
                )
            }
            Self::DecisionMismatch {
                lane_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {lane_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::NarrowingReasonsMismatch { lane_id } => {
                write!(f, "row {lane_id} narrowing reasons disagree with the gate")
            }
            Self::PublishedLaneNotClean { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is publishable but carries a narrowing reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5TargetDiscoveryViolation {}

/// Loads the embedded M5 target-discovery matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5TargetDiscoveryMatrix`].
pub fn current_m5_target_discovery_matrix() -> Result<M5TargetDiscoveryMatrix, serde_json::Error> {
    serde_json::from_str(M5_TARGET_DISCOVERY_JSON)
}

#[cfg(test)]
mod tests;
