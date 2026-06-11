//! Canonical M5 host-boundary matrix with a non-inheriting attribution gate that
//! keeps every execution lane honest about *where* its work actually ran and how
//! certain that answer is.
//!
//! Each [`LaneHostBoundaryRow`] names one M5 execution lane that can cross from the
//! local shell to a remote, container, managed-workspace, browser-bridge, or
//! service-plane host — notebook runs, previews, framework actions, profiler
//! captures, request/browser-runtime mutations, incident/resource actions,
//! managed-workspace runs, and service-plane actions — and answers, for that lane,
//! which [`HostKind`] the work ran on, which [`OriginLocus`] its execution-origin
//! receipt may claim, whether an origin receipt was captured
//! ([`OriginReceiptState`]), what the live connection looks like
//! ([`ConnectionState`]), whether the host/target identity is stably bound
//! ([`HostBindingState`]), and whether that identity survives into desktop, CLI,
//! and support exports unchanged ([`ExportContinuityState`]). The row then publishes
//! an [`AttributionConfidence`] no input can exceed.
//!
//! The [`AttributionConfidence`] a lane may publish is the weakest ceiling implied by
//! its observed states, so a missing origin receipt, a bridged or reconnecting
//! connection, a stale context, an unbound host, or a broken export continuity all
//! narrow, flag, or withhold the published origin claim automatically. The guardrail
//! this enforces: a browser, companion, preview, or managed surface can never imply
//! that work ran locally — or claim a confident exact origin — when it actually
//! crossed a remote, bridged, or managed boundary. The [`BoundaryDecision`] records
//! the gate's action and the recomputed [`HostNarrowingReason`]s explain it; all are
//! validated against the gate, and the published [`OriginLocus`] is pinned to the
//! row's [`HostKind`] so a remote host can never masquerade as a local one in an
//! exported receipt.
//!
//! The host vocabulary is closed and shared. [`HostKind`] is the single controlled
//! vocabulary every M5 lane reuses instead of inventing feature-local host badges or
//! route labels, and the locus, receipt, connection, binding, and continuity states
//! travel into the export projection verbatim so support and enterprise review can
//! reason about the same boundary the user saw.
//!
//! The packet is checked in at `artifacts/execution/m5/m5-host-boundary.json` and
//! embedded here. It is metadata-only: every field is a typed state or an opaque ref,
//! and it carries no credential bodies, raw provider payloads, host tokens, or
//! control-plane secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 host-boundary matrix schema version.
pub const M5_HOST_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_HOST_BOUNDARY_RECORD_KIND: &str = "m5_host_boundary_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_HOST_BOUNDARY_PATH: &str = "artifacts/execution/m5/m5-host-boundary.json";

/// Embedded checked-in packet JSON.
pub const M5_HOST_BOUNDARY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/execution/m5/m5-host-boundary.json"
));

/// An M5 execution lane that can cross a host boundary the matrix makes claims
/// about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionLane {
    /// Notebook cell/run execution lane.
    NotebookRun,
    /// Preview-session lane.
    PreviewSession,
    /// Framework generator/action lane.
    FrameworkAction,
    /// Profiler-capture lane.
    ProfilerCapture,
    /// Request/browser-runtime mutation lane.
    RequestRuntimeMutation,
    /// Incident or live-resource action lane.
    IncidentResourceAction,
    /// Managed-workspace run lane.
    ManagedWorkspaceRun,
    /// Connector-backed service-plane action lane.
    ServicePlaneAction,
}

impl ExecutionLane {
    /// Every execution lane, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookRun,
        Self::PreviewSession,
        Self::FrameworkAction,
        Self::ProfilerCapture,
        Self::RequestRuntimeMutation,
        Self::IncidentResourceAction,
        Self::ManagedWorkspaceRun,
        Self::ServicePlaneAction,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRun => "notebook_run",
            Self::PreviewSession => "preview_session",
            Self::FrameworkAction => "framework_action",
            Self::ProfilerCapture => "profiler_capture",
            Self::RequestRuntimeMutation => "request_runtime_mutation",
            Self::IncidentResourceAction => "incident_resource_action",
            Self::ManagedWorkspaceRun => "managed_workspace_run",
            Self::ServicePlaneAction => "service_plane_action",
        }
    }
}

/// The single controlled vocabulary for *where* an M5 lane's work ran.
///
/// Every lane reuses this closed set instead of inventing feature-local host badges
/// or route labels, and the value is preserved verbatim into exported receipts so a
/// remote, container, managed, bridged, or service-plane host stays distinguishable
/// from local execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostKind {
    /// The local desktop host.
    Local,
    /// A remote host reached over SSH.
    Ssh,
    /// A container runtime.
    Container,
    /// A managed cloud workspace runtime.
    ManagedWorkspace,
    /// A browser/companion runtime reached across a bridge.
    BrowserBridge,
    /// A connector-backed service-plane runtime.
    ServicePlane,
}

impl HostKind {
    /// Every host kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Local,
        Self::Ssh,
        Self::Container,
        Self::ManagedWorkspace,
        Self::BrowserBridge,
        Self::ServicePlane,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Ssh => "ssh",
            Self::Container => "container",
            Self::ManagedWorkspace => "managed_workspace",
            Self::BrowserBridge => "browser_bridge",
            Self::ServicePlane => "service_plane",
        }
    }

    /// The execution-origin locus this host kind may claim.
    ///
    /// This is the pinned mapping the gate validates a published locus against, so a
    /// remote, managed, bridged, or service-plane host can never claim a local origin.
    pub const fn locus(self) -> OriginLocus {
        match self {
            Self::Local => OriginLocus::Local,
            Self::Ssh | Self::Container => OriginLocus::Remote,
            Self::ManagedWorkspace => OriginLocus::Managed,
            Self::BrowserBridge => OriginLocus::Bridged,
            Self::ServicePlane => OriginLocus::ServicePlane,
        }
    }

    /// Whether work on this host ran on the local desktop.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::Local)
    }

    /// Whether work on this host crossed a remote, managed, bridged, or service-plane
    /// boundary.
    pub const fn crosses_boundary(self) -> bool {
        !self.is_local()
    }
}

/// The honest origin label an execution-origin receipt may carry.
///
/// Derived from [`HostKind::locus`] and validated against it, so the exported
/// receipt's locus can never disagree with the host the work actually ran on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginLocus {
    /// Work ran on the local desktop host.
    Local,
    /// Work ran on a remote SSH or container host.
    Remote,
    /// Work ran on a managed cloud workspace.
    Managed,
    /// Work ran across a browser/companion bridge.
    Bridged,
    /// Work ran on a connector-backed service plane.
    ServicePlane,
}

impl OriginLocus {
    /// Every origin locus, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Local,
        Self::Remote,
        Self::Managed,
        Self::Bridged,
        Self::ServicePlane,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Managed => "managed",
            Self::Bridged => "bridged",
            Self::ServicePlane => "service_plane",
        }
    }
}

/// Strength of a lane's published execution-origin attribution.
///
/// Ordered low-to-high by [`AttributionConfidence::rank`]: an
/// [`AttributionConfidence::Unattributed`] lane carries no usable origin, and an
/// [`AttributionConfidence::Confirmed`] lane carries a receipt-backed, host-bound,
/// live, export-continuous "this is where it ran" answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionConfidence {
    /// Origin confirmed against a current, receipt-backed, bound host identity.
    Confirmed,
    /// Origin attributed but on a single signal or across a bridge.
    Attributed,
    /// Origin only provisionally known; the host is reconnecting or partial.
    Provisional,
    /// Origin known historically but the context evidence is stale.
    Stale,
    /// No usable origin could be established.
    Unattributed,
}

impl AttributionConfidence {
    /// Every attribution confidence, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Confirmed,
        Self::Attributed,
        Self::Provisional,
        Self::Stale,
        Self::Unattributed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Attributed => "attributed",
            Self::Provisional => "provisional",
            Self::Stale => "stale",
            Self::Unattributed => "unattributed",
        }
    }

    /// Monotonic rank; higher means a more certain origin.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unattributed => 0,
            Self::Stale => 1,
            Self::Provisional => 2,
            Self::Attributed => 3,
            Self::Confirmed => 4,
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
}

/// Whether an execution-origin receipt was captured for the lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginReceiptState {
    /// A receipt was captured and bound to the host identity.
    Signed,
    /// A receipt was captured but is unsigned; caps at attributed.
    Recorded,
    /// The origin was inferred without a receipt; caps at provisional.
    Inferred,
    /// No origin receipt exists; caps at unattributed.
    Missing,
}

impl OriginReceiptState {
    /// Every origin-receipt state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Signed, Self::Recorded, Self::Inferred, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Signed => "signed",
            Self::Recorded => "recorded",
            Self::Inferred => "inferred",
            Self::Missing => "missing",
        }
    }

    /// Highest confidence this receipt state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> AttributionConfidence {
        match self {
            Self::Signed => AttributionConfidence::Confirmed,
            Self::Recorded => AttributionConfidence::Attributed,
            Self::Inferred => AttributionConfidence::Provisional,
            Self::Missing => AttributionConfidence::Unattributed,
        }
    }

    /// Whether this state raises the [`HostNarrowingReason::MissingOriginReceipt`]
    /// trigger.
    pub const fn is_missing_trigger(self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// The live connection state between the desktop and the lane's host.
///
/// These are the fallback states that keep an impaired remote or managed context
/// from poisoning unrelated local context or implying a parity the desktop has not
/// claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    /// The host connection is live and direct.
    Connected,
    /// The host is reached across a bridge; caps at attributed and is flagged.
    Bridged,
    /// The host connection is reconnecting; caps at provisional.
    Reconnecting,
    /// The host context is stale; caps at stale.
    Stale,
}

impl ConnectionState {
    /// Every connection state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::Bridged,
        Self::Reconnecting,
        Self::Stale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Bridged => "bridged",
            Self::Reconnecting => "reconnecting",
            Self::Stale => "stale",
        }
    }

    /// Highest confidence this connection state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> AttributionConfidence {
        match self {
            Self::Connected => AttributionConfidence::Confirmed,
            Self::Bridged => AttributionConfidence::Attributed,
            Self::Reconnecting => AttributionConfidence::Provisional,
            Self::Stale => AttributionConfidence::Stale,
        }
    }

    /// Whether the gate should flag the boundary crossing for review rather than
    /// silently publish it.
    ///
    /// A bridged connection must be surfaced explicitly so a browser or companion
    /// surface never implies that work ran locally.
    pub const fn is_flaggable(self) -> bool {
        matches!(self, Self::Bridged)
    }

    /// Whether this state raises the [`HostNarrowingReason::BridgedBoundary`] trigger.
    pub const fn is_bridged_trigger(self) -> bool {
        matches!(self, Self::Bridged)
    }

    /// Whether this state raises the [`HostNarrowingReason::ReconnectingHost`]
    /// trigger.
    pub const fn is_reconnecting_trigger(self) -> bool {
        matches!(self, Self::Reconnecting)
    }

    /// Whether this state raises the [`HostNarrowingReason::StaleContext`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale)
    }
}

/// Whether the lane's host/target identity is stably bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBindingState {
    /// The host identity is bound and unchanged.
    Bound,
    /// The host identity changed and was rebound; caps at attributed.
    Rebound,
    /// The host identity is not bound; caps at provisional.
    Unbound,
}

impl HostBindingState {
    /// Every host-binding state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Bound, Self::Rebound, Self::Unbound];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::Rebound => "rebound",
            Self::Unbound => "unbound",
        }
    }

    /// Highest confidence this binding state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> AttributionConfidence {
        match self {
            Self::Bound => AttributionConfidence::Confirmed,
            Self::Rebound => AttributionConfidence::Attributed,
            Self::Unbound => AttributionConfidence::Provisional,
        }
    }

    /// Whether the host identity changed and therefore needs a rebind diff.
    pub const fn is_rebind(self) -> bool {
        matches!(self, Self::Rebound)
    }

    /// Whether this state raises the [`HostNarrowingReason::UnboundHost`] trigger.
    pub const fn is_unbound_trigger(self) -> bool {
        matches!(self, Self::Unbound)
    }
}

/// Whether the lane's host/target identity survives into exports unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportContinuityState {
    /// The identity is continuous across desktop, CLI, and support export.
    Continuous,
    /// The identity carries only partially into exports; caps at provisional.
    Partial,
    /// The identity is broken across exports; caps at unattributed.
    Broken,
}

impl ExportContinuityState {
    /// Every export-continuity state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Continuous, Self::Partial, Self::Broken];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continuous => "continuous",
            Self::Partial => "partial",
            Self::Broken => "broken",
        }
    }

    /// Highest confidence this continuity state permits a lane to publish.
    pub const fn confidence_ceiling(self) -> AttributionConfidence {
        match self {
            Self::Continuous => AttributionConfidence::Confirmed,
            Self::Partial => AttributionConfidence::Provisional,
            Self::Broken => AttributionConfidence::Unattributed,
        }
    }

    /// Whether this state raises the [`HostNarrowingReason::ExportContinuityBroken`]
    /// trigger.
    pub const fn is_broken_trigger(self) -> bool {
        matches!(self, Self::Broken)
    }
}

/// A headline reason the host-boundary gate narrows a lane.
///
/// These are the canonical host-boundary release-control triggers: a missing origin
/// receipt, a bridged boundary, a reconnecting host, a stale context, an unbound
/// host, and a broken export continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostNarrowingReason {
    /// No execution-origin receipt was captured.
    MissingOriginReceipt,
    /// The host was reached across a bridge rather than directly.
    BridgedBoundary,
    /// The host connection is reconnecting.
    ReconnectingHost,
    /// The host context is stale.
    StaleContext,
    /// The host identity is not bound.
    UnboundHost,
    /// The host identity is broken across exports.
    ExportContinuityBroken,
}

impl HostNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MissingOriginReceipt,
        Self::BridgedBoundary,
        Self::ReconnectingHost,
        Self::StaleContext,
        Self::UnboundHost,
        Self::ExportContinuityBroken,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingOriginReceipt => "missing_origin_receipt",
            Self::BridgedBoundary => "bridged_boundary",
            Self::ReconnectingHost => "reconnecting_host",
            Self::StaleContext => "stale_context",
            Self::UnboundHost => "unbound_host",
            Self::ExportContinuityBroken => "export_continuity_broken",
        }
    }
}

/// The action the host-boundary gate takes on a lane relative to a confirmed origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryDecision {
    /// No narrowing; the lane publishes a confirmed origin.
    Publish,
    /// Publish the origin, but at a narrowed attribution label.
    Narrow,
    /// Surface the boundary crossing for review before it is adopted.
    FlagForReview,
    /// Withhold the origin entirely; no usable origin was established.
    Withhold,
}

impl BoundaryDecision {
    /// Every boundary decision, in declaration order.
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

/// One host-boundary row for an M5 execution lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneHostBoundaryRow {
    /// Stable lane host-boundary id.
    pub lane_id: String,
    /// M5 execution lane this row governs.
    pub execution_lane: ExecutionLane,
    /// Host kind the lane's work ran on.
    pub host_kind: HostKind,
    /// Origin locus the receipt publishes; must equal [`HostKind::locus`].
    pub published_locus: OriginLocus,
    /// Whether an execution-origin receipt was captured.
    pub origin_receipt_state: OriginReceiptState,
    /// Live connection state between the desktop and the host.
    pub connection_state: ConnectionState,
    /// Whether the host/target identity is stably bound.
    pub host_binding_state: HostBindingState,
    /// Whether the identity survives into desktop, CLI, and support exports.
    pub export_continuity_state: ExportContinuityState,
    /// Attribution the lane's own evidence asserts, before the gate.
    pub declared_attribution: AttributionConfidence,
    /// Attribution actually published after the gate narrows the lane.
    ///
    /// Must equal [`LaneHostBoundaryRow::effective_attribution`].
    pub published_attribution: AttributionConfidence,
    /// Decision the gate takes; must equal the recomputed decision.
    pub boundary_decision: BoundaryDecision,
    /// Headline narrowing reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrowing_reasons: Vec<HostNarrowingReason>,
    /// Ref to the stable host/target identity the lane is bound to.
    pub host_identity_ref: String,
    /// Ref to the previous host identity; required when the host was rebound.
    #[serde(default)]
    pub previous_host_ref: String,
    /// Ref to the rebind diff; required when the host was rebound.
    #[serde(default)]
    pub rebind_diff_ref: String,
    /// Ref to the execution-origin receipt.
    pub origin_receipt_ref: String,
    /// Ref to the host-boundary context strip the user saw.
    pub context_strip_ref: String,
    /// Ref to the in-product execution this boundary applies to.
    pub execution_ref: String,
    /// Ref binding this row into desktop, CLI, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl LaneHostBoundaryRow {
    /// The attribution the lane's own evidence asserted, before environmental
    /// narrowing.
    pub fn capability_floor(&self) -> AttributionConfidence {
        self.declared_attribution
    }

    /// The locus the row's host kind pins the published locus to.
    pub fn derived_locus(&self) -> OriginLocus {
        self.host_kind.locus()
    }

    /// The attribution the gate permits this lane to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the origin
    /// receipt, connection, host binding, and export continuity states, so a missing
    /// receipt, a bridged or reconnecting connection, a stale context, an unbound
    /// host, or a broken export continuity can never publish a confirmed origin.
    pub fn effective_attribution(&self) -> AttributionConfidence {
        self.capability_floor()
            .min(self.origin_receipt_state.confidence_ceiling())
            .min(self.connection_state.confidence_ceiling())
            .min(self.host_binding_state.confidence_ceiling())
            .min(self.export_continuity_state.confidence_ceiling())
    }

    /// The headline narrowing reasons recomputed from the lane's observed states.
    pub fn computed_narrowing_reasons(&self) -> Vec<HostNarrowingReason> {
        let mut reasons = Vec::new();
        if self.origin_receipt_state.is_missing_trigger() {
            reasons.push(HostNarrowingReason::MissingOriginReceipt);
        }
        if self.connection_state.is_bridged_trigger() {
            reasons.push(HostNarrowingReason::BridgedBoundary);
        }
        if self.connection_state.is_reconnecting_trigger() {
            reasons.push(HostNarrowingReason::ReconnectingHost);
        }
        if self.connection_state.is_stale_trigger() {
            reasons.push(HostNarrowingReason::StaleContext);
        }
        if self.host_binding_state.is_unbound_trigger() {
            reasons.push(HostNarrowingReason::UnboundHost);
        }
        if self.export_continuity_state.is_broken_trigger() {
            reasons.push(HostNarrowingReason::ExportContinuityBroken);
        }
        reasons
    }

    /// The decision the gate must record for this lane.
    ///
    /// An unattributed origin is withheld; a bridged boundary is flagged for review
    /// before adoption; a confirmed origin publishes; and anything in between narrows.
    pub fn required_decision(&self) -> BoundaryDecision {
        let effective = self.effective_attribution();
        if effective == AttributionConfidence::Unattributed {
            BoundaryDecision::Withhold
        } else if self.connection_state.is_flaggable() {
            BoundaryDecision::FlagForReview
        } else if effective == AttributionConfidence::Confirmed {
            BoundaryDecision::Publish
        } else {
            BoundaryDecision::Narrow
        }
    }

    /// Whether the lane may publish a confirmed origin.
    pub fn is_publishable(&self) -> bool {
        self.effective_attribution() == AttributionConfidence::Confirmed
    }

    /// Whether the lane carries its own non-empty identity, receipt, strip,
    /// execution, and support-export refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.host_identity_ref.trim().is_empty()
            && !self.origin_receipt_ref.trim().is_empty()
            && !self.context_strip_ref.trim().is_empty()
            && !self.execution_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published attribution, locus, decision, and narrowing
    /// reasons all agree with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_attribution == self.effective_attribution()
            && self.published_locus == self.derived_locus()
            && self.boundary_decision == self.required_decision()
            && self.narrowing_reasons == self.computed_narrowing_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5HostBoundarySummary {
    /// Total lane rows.
    pub total_lanes: usize,
    /// Number of claimed lanes.
    pub lane_count: usize,
    /// Lanes published with a confirmed origin.
    pub confirmed_lanes: usize,
    /// Lanes published with an attributed origin.
    pub attributed_lanes: usize,
    /// Lanes published with a provisional origin.
    pub provisional_lanes: usize,
    /// Lanes published with a stale origin.
    pub stale_lanes: usize,
    /// Lanes whose origin is unattributed.
    pub unattributed_lanes: usize,
    /// Lanes that may publish a confirmed origin.
    pub publishable_lanes: usize,
    /// Lanes the gate narrowed to a lower attribution.
    pub narrowed_lanes: usize,
    /// Lanes the gate flagged for review.
    pub flagged_lanes: usize,
    /// Lanes the gate withheld entirely.
    pub withheld_lanes: usize,
    /// Lanes whose work ran on the local host.
    pub local_lanes: usize,
    /// Lanes whose work crossed a remote, managed, bridged, or service-plane boundary.
    pub boundary_crossing_lanes: usize,
    /// Lanes whose host identity was rebound.
    pub rebound_lanes: usize,
    /// Lanes reached across a bridge.
    pub bridged_lanes: usize,
    /// Lanes missing an execution-origin receipt.
    pub missing_receipt_lanes: usize,
    /// Lanes carrying at least one narrowing reason.
    pub lanes_with_narrowing_reasons: usize,
}

/// A redaction-safe export row projected from a lane host-boundary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostBoundaryExportRow {
    /// Lane host-boundary id.
    pub lane_id: String,
    /// Execution-lane token.
    pub execution_lane: String,
    /// Host-kind token.
    pub host_kind: String,
    /// Published origin-locus token.
    pub published_locus: String,
    /// Origin-receipt-state token.
    pub origin_receipt_state: String,
    /// Connection-state token.
    pub connection_state: String,
    /// Host-binding-state token.
    pub host_binding_state: String,
    /// Export-continuity-state token.
    pub export_continuity_state: String,
    /// Declared-attribution token.
    pub declared_attribution: String,
    /// Published-attribution token.
    pub published_attribution: String,
    /// Boundary-decision token.
    pub boundary_decision: String,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Stable host-identity ref.
    pub host_identity_ref: String,
    /// Execution-origin receipt ref.
    pub origin_receipt_ref: String,
    /// Execution ref the boundary applies to.
    pub execution_ref: String,
    /// Whether the lane's work crossed a host boundary.
    pub crossed_boundary: bool,
    /// Whether the lane publishes a confirmed origin.
    pub confirmed_origin: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HostBoundaryExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub lanes: Vec<M5HostBoundaryExportRow>,
    /// Whether every lane's published attribution and decision agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that may publish a confirmed origin.
    pub publishable_count: usize,
    /// Lanes the gate narrowed, flagged, or withheld.
    pub narrowed_count: usize,
    /// Lanes the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 host-boundary matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5HostBoundaryMatrix {
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
    pub execution_lanes: Vec<ExecutionLane>,
    /// Closed host-kind vocabulary.
    pub host_kinds: Vec<HostKind>,
    /// Closed origin-locus vocabulary.
    pub origin_loci: Vec<OriginLocus>,
    /// Closed attribution-confidence vocabulary.
    pub attribution_confidences: Vec<AttributionConfidence>,
    /// Closed origin-receipt-state vocabulary.
    pub origin_receipt_states: Vec<OriginReceiptState>,
    /// Closed connection-state vocabulary.
    pub connection_states: Vec<ConnectionState>,
    /// Closed host-binding-state vocabulary.
    pub host_binding_states: Vec<HostBindingState>,
    /// Closed export-continuity-state vocabulary.
    pub export_continuity_states: Vec<ExportContinuityState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<HostNarrowingReason>,
    /// Closed boundary-decision vocabulary.
    pub boundary_decisions: Vec<BoundaryDecision>,
    /// Host-boundary rows, one per claimed lane.
    #[serde(default)]
    pub lanes: Vec<LaneHostBoundaryRow>,
    /// Summary counts.
    pub summary: M5HostBoundarySummary,
}

impl M5HostBoundaryMatrix {
    /// Returns the row for a claimed lane.
    pub fn lane(&self, lane: ExecutionLane) -> Option<&LaneHostBoundaryRow> {
        self.lanes.iter().find(|l| l.execution_lane == lane)
    }

    /// Lanes that may publish a confirmed origin.
    pub fn publishable_lanes(&self) -> impl Iterator<Item = &LaneHostBoundaryRow> {
        self.lanes.iter().filter(|l| l.is_publishable())
    }

    /// Lanes the gate narrowed, flagged, or withheld in any way.
    pub fn narrowed_lanes(&self) -> impl Iterator<Item = &LaneHostBoundaryRow> {
        self.lanes
            .iter()
            .filter(|l| l.required_decision().is_narrowed())
    }

    /// Lanes the gate withheld entirely.
    pub fn withheld_lanes(&self) -> impl Iterator<Item = &LaneHostBoundaryRow> {
        self.lanes
            .iter()
            .filter(|l| l.required_decision() == BoundaryDecision::Withhold)
    }

    /// Whether every lane's stored published attribution, locus, decision, and
    /// reasons agree with the recomputed gate decision.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.lanes.iter().all(|l| l.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5HostBoundarySummary {
        let count_published = |confidence: AttributionConfidence| {
            self.lanes
                .iter()
                .filter(|l| l.published_attribution == confidence)
                .count()
        };
        let count_decision = |decision: BoundaryDecision| {
            self.lanes
                .iter()
                .filter(|l| l.boundary_decision == decision)
                .count()
        };
        M5HostBoundarySummary {
            total_lanes: self.lanes.len(),
            lane_count: self.execution_lanes.len(),
            confirmed_lanes: count_published(AttributionConfidence::Confirmed),
            attributed_lanes: count_published(AttributionConfidence::Attributed),
            provisional_lanes: count_published(AttributionConfidence::Provisional),
            stale_lanes: count_published(AttributionConfidence::Stale),
            unattributed_lanes: count_published(AttributionConfidence::Unattributed),
            publishable_lanes: self.publishable_lanes().count(),
            narrowed_lanes: count_decision(BoundaryDecision::Narrow),
            flagged_lanes: count_decision(BoundaryDecision::FlagForReview),
            withheld_lanes: count_decision(BoundaryDecision::Withhold),
            local_lanes: self.lanes.iter().filter(|l| l.host_kind.is_local()).count(),
            boundary_crossing_lanes: self
                .lanes
                .iter()
                .filter(|l| l.host_kind.crosses_boundary())
                .count(),
            rebound_lanes: self
                .lanes
                .iter()
                .filter(|l| l.host_binding_state.is_rebind())
                .count(),
            bridged_lanes: self
                .lanes
                .iter()
                .filter(|l| l.connection_state.is_bridged_trigger())
                .count(),
            missing_receipt_lanes: self
                .lanes
                .iter()
                .filter(|l| l.origin_receipt_state.is_missing_trigger())
                .count(),
            lanes_with_narrowing_reasons: self
                .lanes
                .iter()
                .filter(|l| !l.narrowing_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — desktop and CLI host
    /// strips, notebook/preview/profiler/framework/request/incident lanes, companion
    /// and browser handoff surfaces, support exports, and release/public-truth
    /// packets — render instead of restating where work ran by hand.
    pub fn export_projection(&self) -> M5HostBoundaryExportProjection {
        let lanes = self
            .lanes
            .iter()
            .map(|l| M5HostBoundaryExportRow {
                lane_id: l.lane_id.clone(),
                execution_lane: l.execution_lane.as_str().to_owned(),
                host_kind: l.host_kind.as_str().to_owned(),
                published_locus: l.published_locus.as_str().to_owned(),
                origin_receipt_state: l.origin_receipt_state.as_str().to_owned(),
                connection_state: l.connection_state.as_str().to_owned(),
                host_binding_state: l.host_binding_state.as_str().to_owned(),
                export_continuity_state: l.export_continuity_state.as_str().to_owned(),
                declared_attribution: l.declared_attribution.as_str().to_owned(),
                published_attribution: l.published_attribution.as_str().to_owned(),
                boundary_decision: l.boundary_decision.as_str().to_owned(),
                narrowing_reasons: l
                    .narrowing_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                host_identity_ref: l.host_identity_ref.clone(),
                origin_receipt_ref: l.origin_receipt_ref.clone(),
                execution_ref: l.execution_ref.clone(),
                crossed_boundary: l.host_kind.crosses_boundary(),
                confirmed_origin: l.is_publishable(),
                summary: format!(
                    "{}: host {} ({}), receipt {}, connection {}, binding {}, continuity {}, declared {}, published {} ({})",
                    l.execution_lane.as_str(),
                    l.host_kind.as_str(),
                    l.published_locus.as_str(),
                    l.origin_receipt_state.as_str(),
                    l.connection_state.as_str(),
                    l.host_binding_state.as_str(),
                    l.export_continuity_state.as_str(),
                    l.declared_attribution.as_str(),
                    l.published_attribution.as_str(),
                    l.boundary_decision.as_str()
                ),
            })
            .collect();
        M5HostBoundaryExportProjection {
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
    pub fn validate(&self) -> Vec<M5HostBoundaryViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ExecutionLane> = self.execution_lanes.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for row in &self.lanes {
            if !seen_ids.insert(row.lane_id.clone()) {
                violations.push(M5HostBoundaryViolation::DuplicateLaneId {
                    lane_id: row.lane_id.clone(),
                });
            }
            if !seen_lanes.insert(row.execution_lane) {
                violations.push(M5HostBoundaryViolation::DuplicateLaneRow {
                    lane: row.execution_lane.as_str(),
                });
            }
            if !claimed.contains(&row.execution_lane) {
                violations.push(M5HostBoundaryViolation::UnclaimedLaneRow {
                    lane_id: row.lane_id.clone(),
                    lane: row.execution_lane.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed lane must carry its own row, so a lane never inherits a
        // confirmed origin from an adjacent one.
        for &lane in &self.execution_lanes {
            if !seen_lanes.contains(&lane) {
                violations.push(M5HostBoundaryViolation::MissingLaneRow {
                    lane: lane.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5HostBoundaryViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5HostBoundaryViolation>) {
        if self.schema_version != M5_HOST_BOUNDARY_SCHEMA_VERSION {
            violations.push(M5HostBoundaryViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_HOST_BOUNDARY_RECORD_KIND {
            violations.push(M5HostBoundaryViolation::UnsupportedRecordKind {
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
                violations.push(M5HostBoundaryViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "execution_lanes",
                self.execution_lanes == ExecutionLane::ALL.to_vec(),
            ),
            ("host_kinds", self.host_kinds == HostKind::ALL.to_vec()),
            ("origin_loci", self.origin_loci == OriginLocus::ALL.to_vec()),
            (
                "attribution_confidences",
                self.attribution_confidences == AttributionConfidence::ALL.to_vec(),
            ),
            (
                "origin_receipt_states",
                self.origin_receipt_states == OriginReceiptState::ALL.to_vec(),
            ),
            (
                "connection_states",
                self.connection_states == ConnectionState::ALL.to_vec(),
            ),
            (
                "host_binding_states",
                self.host_binding_states == HostBindingState::ALL.to_vec(),
            ),
            (
                "export_continuity_states",
                self.export_continuity_states == ExportContinuityState::ALL.to_vec(),
            ),
            (
                "narrowing_reasons",
                self.narrowing_reasons == HostNarrowingReason::ALL.to_vec(),
            ),
            (
                "boundary_decisions",
                self.boundary_decisions == BoundaryDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5HostBoundaryViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &LaneHostBoundaryRow,
        violations: &mut Vec<M5HostBoundaryViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &row.lane_id),
            ("host_identity_ref", &row.host_identity_ref),
            ("origin_receipt_ref", &row.origin_receipt_ref),
            ("context_strip_ref", &row.context_strip_ref),
            ("execution_ref", &row.execution_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5HostBoundaryViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // A rebound host must carry its previous-host and rebind-diff refs so the
        // host change is reviewable instead of silently replacing the current host.
        if row.host_binding_state.is_rebind() {
            for (field, value) in [
                ("previous_host_ref", &row.previous_host_ref),
                ("rebind_diff_ref", &row.rebind_diff_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5HostBoundaryViolation::EmptyField {
                        id: row.lane_id.clone(),
                        field_name: field,
                    });
                }
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrowing_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5HostBoundaryViolation::DuplicateNarrowingReason {
                    lane_id: row.lane_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published locus must equal the locus pinned by the host kind, so a
        // remote, managed, bridged, or service-plane host can never imply a local
        // origin in an exported receipt.
        let derived_locus = row.derived_locus();
        if row.published_locus != derived_locus {
            violations.push(M5HostBoundaryViolation::LocusMismatch {
                lane_id: row.lane_id.clone(),
                published: row.published_locus.as_str(),
                derived: derived_locus.as_str(),
            });
        }

        // The published attribution must equal the gate's recomputed ceiling, so an
        // impaired remote, bridged, or managed context can never read as a confirmed
        // origin.
        let effective = row.effective_attribution();
        if row.published_attribution != effective {
            violations.push(M5HostBoundaryViolation::OverstatedAttribution {
                lane_id: row.lane_id.clone(),
                published: row.published_attribution.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded decision must match the gate's recomputed decision.
        let required = row.required_decision();
        if row.boundary_decision != required {
            violations.push(M5HostBoundaryViolation::DecisionMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.boundary_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded narrowing reasons must equal the reasons recomputed from the
        // observed states, so a narrowing can never be asserted or hidden by hand.
        let computed = row.computed_narrowing_reasons();
        if row.narrowing_reasons != computed {
            violations.push(M5HostBoundaryViolation::NarrowingReasonsMismatch {
                lane_id: row.lane_id.clone(),
            });
        }

        // A publishable lane must be genuinely clean: a confirmed-ceiling receipt,
        // connection, binding, and continuity state, a confirmed capability floor, and
        // no narrowing reason. This is the non-inheritance guardrail.
        if row.is_publishable()
            && (row.origin_receipt_state.confidence_ceiling() != AttributionConfidence::Confirmed
                || row.connection_state.confidence_ceiling() != AttributionConfidence::Confirmed
                || row.host_binding_state.confidence_ceiling() != AttributionConfidence::Confirmed
                || row.export_continuity_state.confidence_ceiling()
                    != AttributionConfidence::Confirmed
                || row.capability_floor() != AttributionConfidence::Confirmed
                || !row.narrowing_reasons.is_empty())
        {
            violations.push(M5HostBoundaryViolation::PublishedLaneNotClean {
                lane_id: row.lane_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 host-boundary packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HostBoundaryViolation {
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
    /// A lane host-boundary id appears more than once.
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
    /// A row lists a narrowing reason more than once.
    DuplicateNarrowingReason {
        /// Row id.
        lane_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A row's published locus disagrees with the locus pinned by its host kind.
    LocusMismatch {
        /// Row id.
        lane_id: String,
        /// Published locus token.
        published: &'static str,
        /// Derived locus token.
        derived: &'static str,
    },
    /// A lane publishes an attribution beyond what its evidence supports.
    OverstatedAttribution {
        /// Row id.
        lane_id: String,
        /// Published attribution token.
        published: &'static str,
        /// Computed effective attribution token.
        computed: &'static str,
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

impl fmt::Display for M5HostBoundaryViolation {
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
            Self::DuplicateNarrowingReason { lane_id, reason } => {
                write!(f, "row {lane_id} repeats narrowing reason {reason}")
            }
            Self::LocusMismatch {
                lane_id,
                published,
                derived,
            } => {
                write!(
                    f,
                    "row {lane_id} publishes locus {published} but its host kind pins {derived}"
                )
            }
            Self::OverstatedAttribution {
                lane_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {lane_id} publishes attribution {published} but the gate computes {computed}"
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

impl Error for M5HostBoundaryViolation {}

/// Loads the embedded M5 host-boundary matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5HostBoundaryMatrix`].
pub fn current_m5_host_boundary_matrix() -> Result<M5HostBoundaryMatrix, serde_json::Error> {
    serde_json::from_str(M5_HOST_BOUNDARY_JSON)
}

#[cfg(test)]
mod tests;
