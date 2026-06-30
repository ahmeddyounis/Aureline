//! The M5 event-provenance inspector — the surface that explains, for each *queued or replayable*
//! M5 action, *where the event came from, what changed since it was planned, and whether replaying
//! it is still safe*, bound to the same gate-bound state grammar the
//! [governance matrix](crate::m5_assurance_route_governance) froze.
//!
//! Some consequential M5 work does not run the instant it is requested: a prompt is queued for a
//! provider, a model download is deferred to a mirror, a credential rotation is scheduled, a data
//! export is published later, a control-plane sync is queued, a policy push is retried, a support
//! bundle is handed off, an audit export is replayed. Between *plan* and *run* the world can move —
//! the endpoint, tenant, region, proxy, certificate, mirror, or policy a queued action assumed can
//! drift. When it does, replaying as if the old boundary still holds is exactly the unsafe step this
//! lane makes visible. Each deferred action carries three reusable facets:
//!
//! - [`EventProvenanceRow`]s. One per action — the stable lineage row attached to the log,
//!   diagnostic, artifact, or audit [surface](ProvenanceSurface) the event landed on. It names the
//!   [event id](EventProvenanceRow::event_id), the mutation / run / session it links to, the
//!   [host lane](HostLane) the work ran in, the [retrieval epoch](EventProvenanceRow::retrieval_epoch)
//!   it was read as-of, and the [redaction posture](RedactionPosture) of the row. Its active
//!   [provenance state](ProvenanceState) is read from the matrix vocabulary, so a row can never read
//!   more traceable than its proof.
//! - [`RouteDriftBanner`]s. One per action — a banner that compares the action's current route facts
//!   against its [baseline](DriftBaseline) (the plan, or the last success) and names every
//!   [fact that drifted](DriftFacet): endpoint, tenant, region, proxy, certificate, mirror, or
//!   policy. The banner binds the action's [route state](RouteHopState) and auto-narrows when a fact
//!   drifts and blocks when a drift crosses a hard boundary (a tenant change), so a changed route
//!   never reads as a clean pass.
//! - [`ReplayReapprovalGate`]s. One per action — the gate that decides whether the deferred action
//!   may [replay as-is, must be re-approved, or is held](ReapprovalDecision), bound to the current
//!   [capability-boundary state](CapabilityBoundaryState) and the [approval state](ApprovalState)
//!   from the runtime authority vocabulary. When current boundary facts invalidate the earlier route
//!   or approval assumptions, the gate requires a replay / publish-later / approve-again decision
//!   rather than continuing silently.
//!
//! Each action's three facets roll up into a [`DeferredEvent`] whose effective gate is the *worst*
//! of the three, so the event never reads safer than its least-attested facet. The packet also
//! carries a [`EventProvenanceExportPreview`] that reduces each event to the exact provenance / route
//! / approval vocabulary the facets show, so an exported support / audit pack and the in-product
//! inspector can never drift. The [`M5EventProvenance`] packet is the one inspectable,
//! serde-serializable truth record this lane produces: it preserves event / route / proof lineage as
//! refs only and carries no credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/public-truth/m5-event-provenance.schema.json`](../../../../../schemas/public-truth/m5-event-provenance.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-event-provenance-contract.md`](../../../../../docs/public-truth/m5-event-provenance-contract.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_event_provenance, seeded_m5_event_provenance_drift_region_narrowed,
    seeded_m5_event_provenance_drift_tenant_blocked,
    seeded_m5_event_provenance_provenance_stale_narrowed,
    seeded_m5_event_provenance_reapproval_blocked,
    seeded_m5_event_provenance_reapproval_required_narrowed, M5_EVENT_PROVENANCE_PACKET_ID,
};

use serde::{Deserialize, Serialize};

// The event-provenance inspector reuses the governance matrix's frozen provenance / route-hop /
// approval / capability-boundary state vocabulary and the descriptor / badge gate runtime, so the
// in-product facets and the exported support / audit preview can never drift to a different grammar.
use crate::m5_assurance_route_governance::{
    ApprovalState, CapabilityBoundaryState, EvidenceClass, ProvenanceState, RouteHopState,
    TrustBoundary,
};
use crate::m5_descriptor_badge::{
    ConsumerStatus, DescriptorGate, DescriptorSignal, FreshnessState, QualificationClass,
};

/// Record-kind tag carried by [`M5EventProvenance`].
pub const M5_EVENT_PROVENANCE_RECORD_KIND: &str = "m5_event_provenance";

/// Record-kind tag carried by the embedded [`EventProvenanceExportPreview`].
pub const M5_EVENT_PROVENANCE_EXPORT_RECORD_KIND: &str = "m5_event_provenance_export_preview";

/// Schema version for the event-provenance packet.
pub const M5_EVENT_PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the event-provenance packet schema.
pub const M5_EVENT_PROVENANCE_SCHEMA_REF: &str =
    "schemas/public-truth/m5-event-provenance.schema.json";

/// Repo-relative path of the published event-provenance inventory.
pub const M5_EVENT_PROVENANCE_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-event-provenance.json";

/// Repo-relative path of the rendered event-provenance overview document.
pub const M5_EVENT_PROVENANCE_OVERVIEW_REF: &str = "artifacts/public-truth/m5-event-provenance.md";

/// Repo-relative path of the machine-readable event / facet matrix export.
pub const M5_EVENT_PROVENANCE_EVENTS_CSV_REF: &str =
    "artifacts/public-truth/m5-event-provenance-events.csv";

/// Repo-relative path of the release-grade event-provenance parity proof.
pub const M5_EVENT_PROVENANCE_PROOF_REF: &str =
    "artifacts/public-truth/m5-event-provenance-proof/event-provenance.json";

/// Repo-relative path of the exported redaction-safe export preview.
pub const M5_EVENT_PROVENANCE_EXPORT_PREVIEW_REF: &str =
    "artifacts/public-truth/m5-event-provenance-proof/export-preview.json";

/// Repo-relative path of the event-provenance contract doc.
pub const M5_EVENT_PROVENANCE_DOC_REF: &str = "docs/public-truth/m5-event-provenance-contract.md";

/// Repo-relative directory of the per-state event-provenance fixtures.
pub const M5_EVENT_PROVENANCE_FIXTURE_DIR: &str = "fixtures/public-truth/m5-event-provenance/";

/// Prefix every event-provenance message id carries so consumers can route it.
pub const M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX: &str = "public_truth.event_provenance.";

/// Repo-relative proof ref backing the event-provenance facet — drawn from the governance-matrix
/// proofs rather than a parallel evidence family.
const PROVENANCE_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/event-provenance.json";

/// Repo-relative proof ref backing the route-drift facet.
const ROUTE_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/route-hop.json";

/// Repo-relative proof ref backing the replay / reapproval facet.
const REAPPROVAL_PROOF_REF: &str =
    "artifacts/release-proof/m5-assurance-route-governance/approval-ticket.json";

/// Owner role accountable for keeping the event-provenance facet current.
const PROVENANCE_OWNER_ROLE: &str = "event_provenance_owner";

/// Owner role accountable for keeping the route-drift facet current.
const ROUTE_OWNER_ROLE: &str = "route_explainability_owner";

/// Owner role accountable for keeping the replay / reapproval facet current.
const REAPPROVAL_OWNER_ROLE: &str = "runtime_authority_owner";

// ---------------------------------------------------------------------------------------------
// Deferred actions
// ---------------------------------------------------------------------------------------------

/// One queued or replayable M5 action the inspector explains — the deferred local / remote /
/// provider / support operations whose event provenance, route drift, and replay safety are worth
/// inspecting. The set invents no new replay queue; it names the existing deferred-action mechanics
/// across the AI, provider, remote, and support flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredAction {
    /// A queued prompt / inference is replayed to a remote provider.
    QueuedPromptReplay,
    /// A deferred model-pack acquisition is fetched over a mirror.
    DeferredModelDownload,
    /// A scheduled provider-credential rotation runs through the control plane.
    ScheduledCredentialRotation,
    /// A workspace data export is published later to an external sink.
    PublishLaterDataExport,
    /// A queued workspace-metadata sync runs against the managed control plane.
    QueuedControlPlaneSync,
    /// A failed admin policy push is retried against the control plane.
    RetriedPolicyPush,
    /// A diagnostic bundle handoff to vendor support is deferred.
    DeferredSupportHandoff,
    /// An audit / diagnostics export is replayed for support or compliance.
    ReplayedAuditExport,
}

impl DeferredAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::QueuedPromptReplay,
        Self::DeferredModelDownload,
        Self::ScheduledCredentialRotation,
        Self::PublishLaterDataExport,
        Self::QueuedControlPlaneSync,
        Self::RetriedPolicyPush,
        Self::DeferredSupportHandoff,
        Self::ReplayedAuditExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueuedPromptReplay => "queued_prompt_replay",
            Self::DeferredModelDownload => "deferred_model_download",
            Self::ScheduledCredentialRotation => "scheduled_credential_rotation",
            Self::PublishLaterDataExport => "publish_later_data_export",
            Self::QueuedControlPlaneSync => "queued_control_plane_sync",
            Self::RetriedPolicyPush => "retried_policy_push",
            Self::DeferredSupportHandoff => "deferred_support_handoff",
            Self::ReplayedAuditExport => "replayed_audit_export",
        }
    }

    /// Reader-facing action label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::QueuedPromptReplay => "Queued prompt replay",
            Self::DeferredModelDownload => "Deferred model download",
            Self::ScheduledCredentialRotation => "Scheduled credential rotation",
            Self::PublishLaterDataExport => "Publish-later data export",
            Self::QueuedControlPlaneSync => "Queued control-plane sync",
            Self::RetriedPolicyPush => "Retried policy push",
            Self::DeferredSupportHandoff => "Deferred support handoff",
            Self::ReplayedAuditExport => "Replayed audit export",
        }
    }

    /// The product flow this action belongs to.
    pub const fn flow(self) -> ActionFlow {
        match self {
            Self::QueuedPromptReplay | Self::DeferredModelDownload => ActionFlow::Ai,
            Self::ScheduledCredentialRotation => ActionFlow::Provider,
            Self::PublishLaterDataExport
            | Self::QueuedControlPlaneSync
            | Self::RetriedPolicyPush => ActionFlow::Remote,
            Self::DeferredSupportHandoff | Self::ReplayedAuditExport => ActionFlow::Support,
        }
    }

    /// The surface the action's event-provenance row attaches to.
    pub const fn surface(self) -> ProvenanceSurface {
        match self {
            Self::QueuedPromptReplay | Self::QueuedControlPlaneSync => ProvenanceSurface::Log,
            Self::DeferredModelDownload | Self::PublishLaterDataExport => {
                ProvenanceSurface::Artifact
            }
            Self::ScheduledCredentialRotation
            | Self::RetriedPolicyPush
            | Self::ReplayedAuditExport => ProvenanceSurface::Audit,
            Self::DeferredSupportHandoff => ProvenanceSurface::Diagnostic,
        }
    }

    /// The host lane the action's work runs in.
    pub const fn host_lane(self) -> HostLane {
        match self {
            Self::QueuedPromptReplay | Self::PublishLaterDataExport => HostLane::RemoteRegion,
            Self::DeferredModelDownload => HostLane::MirrorEdge,
            Self::ScheduledCredentialRotation
            | Self::QueuedControlPlaneSync
            | Self::RetriedPolicyPush => HostLane::ControlPlane,
            Self::DeferredSupportHandoff => HostLane::VendorEdge,
            Self::ReplayedAuditExport => HostLane::LocalMachine,
        }
    }

    /// The replay semantics the deferred action carries — what kind of gate it must clear before it
    /// may run again.
    pub const fn deferred_kind(self) -> DeferredKind {
        match self {
            Self::QueuedPromptReplay
            | Self::DeferredModelDownload
            | Self::QueuedControlPlaneSync
            | Self::ReplayedAuditExport => DeferredKind::Replay,
            Self::PublishLaterDataExport | Self::DeferredSupportHandoff => {
                DeferredKind::PublishLater
            }
            Self::ScheduledCredentialRotation | Self::RetriedPolicyPush => {
                DeferredKind::ApproveAgain
            }
        }
    }

    /// The redaction posture of the action's event-provenance row.
    pub const fn redaction_posture(self) -> RedactionPosture {
        match self {
            Self::QueuedPromptReplay | Self::QueuedControlPlaneSync | Self::RetriedPolicyPush => {
                RedactionPosture::MetadataOnly
            }
            Self::DeferredModelDownload
            | Self::PublishLaterDataExport
            | Self::ReplayedAuditExport => RedactionPosture::ReferenceOnly,
            Self::ScheduledCredentialRotation => RedactionPosture::SealedLocal,
            Self::DeferredSupportHandoff => RedactionPosture::RedactedBody,
        }
    }

    /// An export-safe one-line summary (category labels only, no secrets).
    pub const fn export_safe_summary(self) -> &'static str {
        match self {
            Self::QueuedPromptReplay => {
                "Queued prompt replayed to the remote provider on its recorded route."
            }
            Self::DeferredModelDownload => {
                "Deferred model pack fetched from the labelled mirror edge."
            }
            Self::ScheduledCredentialRotation => {
                "Scheduled credential rotation run through the managed control plane."
            }
            Self::PublishLaterDataExport => {
                "Workspace export published later to the recorded external sink."
            }
            Self::QueuedControlPlaneSync => {
                "Queued workspace-metadata sync run against the managed control plane."
            }
            Self::RetriedPolicyPush => {
                "Admin policy push retried against the managed control plane."
            }
            Self::DeferredSupportHandoff => {
                "Diagnostic bundle handed off to vendor support after a deferral."
            }
            Self::ReplayedAuditExport => "Audit / diagnostics export replayed locally for review.",
        }
    }

    /// The stable event id the action's row links to (refs only).
    fn event_id(self) -> String {
        format!("evt:{}:0001", self.as_str())
    }

    /// The mutation the action's event links to (refs only).
    fn mutation_ref(self) -> String {
        format!("mut:{}", self.as_str())
    }

    /// The run the action's event links to (refs only).
    fn run_ref(self) -> String {
        format!("run:{}", self.as_str())
    }

    /// The session the action's event links to (refs only).
    fn session_ref(self) -> String {
        format!("ses:{}", self.as_str())
    }
}

// ---------------------------------------------------------------------------------------------
// Lane vocabularies
// ---------------------------------------------------------------------------------------------

/// The product flow a deferred action belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionFlow {
    /// An AI / model flow.
    Ai,
    /// A provider / credential flow.
    Provider,
    /// A remote / control-plane flow.
    Remote,
    /// A support / diagnostics flow.
    Support,
}

impl ActionFlow {
    /// Every flow, in declaration order.
    pub const ALL: [Self; 4] = [Self::Ai, Self::Provider, Self::Remote, Self::Support];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ai => "ai",
            Self::Provider => "provider",
            Self::Remote => "remote",
            Self::Support => "support",
        }
    }

    /// Reader-facing flow label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ai => "AI",
            Self::Provider => "Provider",
            Self::Remote => "Remote",
            Self::Support => "Support",
        }
    }
}

/// The surface an event-provenance row attaches to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceSurface {
    /// A runtime log line.
    Log,
    /// A diagnostic / doctor record.
    Diagnostic,
    /// A produced artifact.
    Artifact,
    /// An audit record.
    Audit,
}

impl ProvenanceSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 4] = [Self::Log, Self::Diagnostic, Self::Artifact, Self::Audit];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Diagnostic => "diagnostic",
            Self::Artifact => "artifact",
            Self::Audit => "audit",
        }
    }

    /// Reader-facing surface label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Log => "Log",
            Self::Diagnostic => "Diagnostic",
            Self::Artifact => "Artifact",
            Self::Audit => "Audit",
        }
    }
}

/// The host lane an event ran in. The set names the existing local-first, remote-region,
/// control-plane, mirror, and vendor lanes rather than inventing new ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostLane {
    /// The event ran on the local machine.
    LocalMachine,
    /// The event ran in a remote provider region.
    RemoteRegion,
    /// The event ran in the managed control plane.
    ControlPlane,
    /// The event ran on a mirror edge.
    MirrorEdge,
    /// The event ran on a vendor edge.
    VendorEdge,
}

impl HostLane {
    /// Every host lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalMachine,
        Self::RemoteRegion,
        Self::ControlPlane,
        Self::MirrorEdge,
        Self::VendorEdge,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalMachine => "local_machine",
            Self::RemoteRegion => "remote_region",
            Self::ControlPlane => "control_plane",
            Self::MirrorEdge => "mirror_edge",
            Self::VendorEdge => "vendor_edge",
        }
    }

    /// Reader-facing lane label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalMachine => "Local machine",
            Self::RemoteRegion => "Remote region",
            Self::ControlPlane => "Control plane",
            Self::MirrorEdge => "Mirror edge",
            Self::VendorEdge => "Vendor edge",
        }
    }

    /// True when the lane stays on the local-first side of the trust boundary.
    pub const fn is_local(self) -> bool {
        matches!(self, Self::LocalMachine)
    }

    /// The trust boundaries the lane spans.
    fn trust_boundaries(self) -> Vec<TrustBoundary> {
        if self.is_local() {
            vec![TrustBoundary::LocalFirst]
        } else {
            vec![TrustBoundary::LocalFirst, TrustBoundary::ControlPlane]
        }
    }
}

/// The redaction posture of an event-provenance row. Every posture is export-safe: it labels how the
/// event's body is handled rather than carrying the body itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPosture {
    /// Only metadata is recorded; no payload crosses into the row.
    MetadataOnly,
    /// Only refs to the event payload are recorded.
    ReferenceOnly,
    /// The body is recorded but redacted to category labels.
    RedactedBody,
    /// The body is sealed and stays local; only its presence is recorded.
    SealedLocal,
}

impl RedactionPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MetadataOnly,
        Self::ReferenceOnly,
        Self::RedactedBody,
        Self::SealedLocal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::ReferenceOnly => "reference_only",
            Self::RedactedBody => "redacted_body",
            Self::SealedLocal => "sealed_local",
        }
    }

    /// Reader-facing posture label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::MetadataOnly => "Metadata only",
            Self::ReferenceOnly => "Reference only",
            Self::RedactedBody => "Redacted body",
            Self::SealedLocal => "Sealed, local-only",
        }
    }
}

/// The replay semantics a deferred action carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredKind {
    /// The action replays a previously planned run.
    Replay,
    /// The action publishes a previously prepared payload later.
    PublishLater,
    /// The action re-runs an operation that needs its approval re-checked.
    ApproveAgain,
}

impl DeferredKind {
    /// Every deferred kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::Replay, Self::PublishLater, Self::ApproveAgain];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::PublishLater => "publish_later",
            Self::ApproveAgain => "approve_again",
        }
    }

    /// Reader-facing kind label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Replay => "Replay",
            Self::PublishLater => "Publish later",
            Self::ApproveAgain => "Approve again",
        }
    }
}

/// The baseline a route-drift banner compares the current route facts against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftBaseline {
    /// The route facts recorded when the action was planned.
    Plan,
    /// The route facts recorded at the action's last success.
    LastSuccess,
}

impl DriftBaseline {
    /// Every baseline, in declaration order.
    pub const ALL: [Self; 2] = [Self::Plan, Self::LastSuccess];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::LastSuccess => "last_success",
        }
    }

    /// Reader-facing baseline label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Plan => "Plan",
            Self::LastSuccess => "Last success",
        }
    }
}

/// One route fact whose change a route-drift banner watches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftFacet {
    /// The endpoint moved.
    Endpoint,
    /// The tenant changed — a hard isolation boundary.
    Tenant,
    /// The region changed.
    Region,
    /// The proxy changed.
    Proxy,
    /// The certificate changed from the pinned one.
    Certificate,
    /// A mirror silently replaced the named target.
    Mirror,
    /// The governing policy changed since the baseline.
    Policy,
}

impl DriftFacet {
    /// Every drift facet, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Endpoint,
        Self::Tenant,
        Self::Region,
        Self::Proxy,
        Self::Certificate,
        Self::Mirror,
        Self::Policy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Endpoint => "endpoint",
            Self::Tenant => "tenant",
            Self::Region => "region",
            Self::Proxy => "proxy",
            Self::Certificate => "certificate",
            Self::Mirror => "mirror",
            Self::Policy => "policy",
        }
    }

    /// Reader-facing facet label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Endpoint => "Endpoint",
            Self::Tenant => "Tenant",
            Self::Region => "Region",
            Self::Proxy => "Proxy",
            Self::Certificate => "Certificate",
            Self::Mirror => "Mirror",
            Self::Policy => "Policy",
        }
    }

    /// The gate posture a change in this fact imposes. A tenant change crosses a hard isolation
    /// boundary and blocks; every other drift narrows the route until it is re-attributed.
    pub const fn gate_posture(self) -> DescriptorGate {
        match self {
            Self::Tenant => DescriptorGate::Blocked,
            Self::Endpoint
            | Self::Region
            | Self::Proxy
            | Self::Certificate
            | Self::Mirror
            | Self::Policy => DescriptorGate::Narrowed,
        }
    }
}

/// The decision a replay / reapproval gate reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReapprovalDecision {
    /// The deferred action may replay on its recorded route and approval.
    ReplayAsIs,
    /// Current facts narrowed the boundary; the action must be re-approved before it runs.
    RequireReapproval,
    /// Current facts invalidate the action; it is held until the boundary is restored.
    HoldBlocked,
}

impl ReapprovalDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::ReplayAsIs, Self::RequireReapproval, Self::HoldBlocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReplayAsIs => "replay_as_is",
            Self::RequireReapproval => "require_reapproval",
            Self::HoldBlocked => "hold_blocked",
        }
    }

    /// Reader-facing decision label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReplayAsIs => "Replay as-is",
            Self::RequireReapproval => "Require reapproval",
            Self::HoldBlocked => "Hold (blocked)",
        }
    }

    /// The decision a gate posture implies: governed replays, narrowed re-approves, blocked holds.
    const fn for_gate(gate: DescriptorGate) -> Self {
        match gate {
            DescriptorGate::Governed => Self::ReplayAsIs,
            DescriptorGate::Narrowed => Self::RequireReapproval,
            DescriptorGate::Blocked => Self::HoldBlocked,
        }
    }
}

/// One evaluation / export action the inspector offers for a deferred event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationAction {
    /// Inspect the event-provenance row.
    InspectProvenance,
    /// Review the route-drift banner.
    ReviewDrift,
    /// Decide the replay / reapproval gate.
    DecideReapproval,
    /// Export a redaction-safe preview of the event.
    ExportPreview,
}

impl EvaluationAction {
    /// Every evaluation action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InspectProvenance,
        Self::ReviewDrift,
        Self::DecideReapproval,
        Self::ExportPreview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectProvenance => "inspect_provenance",
            Self::ReviewDrift => "review_drift",
            Self::DecideReapproval => "decide_reapproval",
            Self::ExportPreview => "export_preview",
        }
    }
}

/// One facet of a deferred event whose gate can narrow or block the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventFacet {
    /// The event-provenance row.
    Provenance,
    /// The route-drift banner.
    Drift,
    /// The replay / reapproval gate.
    Reapproval,
}

impl EventFacet {
    /// Every facet, in declaration order.
    pub const ALL: [Self; 3] = [Self::Provenance, Self::Drift, Self::Reapproval];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provenance => "provenance",
            Self::Drift => "drift",
            Self::Reapproval => "reapproval",
        }
    }
}

/// The kind of gap a facet's non-governed gate opens on a deferred event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventGapKind {
    /// The facet narrowed the event below fully governed.
    FacetNarrowed,
    /// The facet blocked the event.
    FacetBlocked,
}

impl EventGapKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetNarrowed => "facet_narrowed",
            Self::FacetBlocked => "facet_blocked",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate helpers
// ---------------------------------------------------------------------------------------------

/// The qualification floor a gate posture implies.
const fn floor_for_gate(gate: DescriptorGate) -> QualificationClass {
    match gate {
        DescriptorGate::Governed => QualificationClass::Stable,
        DescriptorGate::Narrowed => QualificationClass::Beta,
        DescriptorGate::Blocked => QualificationClass::Unavailable,
    }
}

/// Restrictiveness rank of a gate posture (least restrictive first).
const fn gate_rank(gate: DescriptorGate) -> usize {
    match gate {
        DescriptorGate::Governed => 0,
        DescriptorGate::Narrowed => 1,
        DescriptorGate::Blocked => 2,
    }
}

/// The more restrictive of two gate postures.
const fn worse_gate(a: DescriptorGate, b: DescriptorGate) -> DescriptorGate {
    if gate_rank(a) >= gate_rank(b) {
        a
    } else {
        b
    }
}

/// The gate posture an evidence freshness implies on its own: current keeps it governed, stale
/// narrows, expired / missing block.
const fn freshness_gate(freshness: FreshnessState) -> DescriptorGate {
    match freshness {
        FreshnessState::Current => DescriptorGate::Governed,
        FreshnessState::Stale => DescriptorGate::Narrowed,
        FreshnessState::Expired | FreshnessState::Missing => DescriptorGate::Blocked,
    }
}

/// Maps a gate posture to the coverage status it implies.
const fn gate_status(gate: DescriptorGate) -> ConsumerStatus {
    match gate {
        DescriptorGate::Governed => ConsumerStatus::Mapped,
        DescriptorGate::Narrowed => ConsumerStatus::Provisional,
        DescriptorGate::Blocked => ConsumerStatus::Unmapped,
    }
}

/// Maps a gate posture to the event-gap kind it implies, when not governed.
const fn gap_kind_for_gate(gate: DescriptorGate) -> Option<EventGapKind> {
    match gate {
        DescriptorGate::Governed => None,
        DescriptorGate::Narrowed => Some(EventGapKind::FacetNarrowed),
        DescriptorGate::Blocked => Some(EventGapKind::FacetBlocked),
    }
}

// ---------------------------------------------------------------------------------------------
// Event-provenance row
// ---------------------------------------------------------------------------------------------

/// One event-provenance row: for one deferred action, the stable lineage attached to the surface the
/// event landed on. It links the event to its mutation / run / session, names the host lane and
/// retrieval epoch, and declares the redaction posture. Its active state is read from the matrix
/// [provenance vocabulary](ProvenanceState), and the effective gate folds in evidence freshness so the
/// row can never read more traceable than its proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventProvenanceRow {
    /// The action this row describes.
    pub action: DeferredAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The flow the action belongs to.
    pub flow: ActionFlow,
    /// Reader-facing flow label.
    pub flow_label: String,
    /// The surface the row attaches to.
    pub surface: ProvenanceSurface,
    /// Reader-facing surface label.
    pub surface_label: String,
    /// The stable event id this row links to.
    pub event_id: String,
    /// The mutation the event links to (ref only).
    pub mutation_ref: String,
    /// The run the event links to (ref only).
    pub run_ref: String,
    /// The session the event links to (ref only).
    pub session_ref: String,
    /// The host lane the work ran in.
    pub host_lane: HostLane,
    /// Reader-facing host-lane label.
    pub host_lane_label: String,
    /// The retrieval epoch the row was read as-of.
    pub retrieval_epoch: String,
    /// The redaction posture of the row.
    pub redaction_posture: RedactionPosture,
    /// Reader-facing redaction-posture label.
    pub redaction_posture_label: String,
    /// The active provenance state.
    pub provenance_state: ProvenanceState,
    /// Reader-facing provenance-state label.
    pub provenance_state_label: String,
    /// Whether the host lane stays on the local-first side of the trust boundary.
    pub is_local: bool,
    /// Freshness of the provenance evidence.
    pub evidence_freshness: FreshnessState,
    /// The gate the provenance state and freshness together imply (the more restrictive of the two).
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The trust boundaries the event spans.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// True when the event leaves the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// The evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the provenance proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the row.
    pub proof_ref: String,
    /// An export-safe one-line summary (no secrets).
    pub export_safe_summary: String,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl EventProvenanceRow {
    /// Derives an event-provenance row from the action, its provenance state, evidence freshness, and
    /// retrieval epoch.
    fn derive(
        action: DeferredAction,
        provenance_state: ProvenanceState,
        evidence_freshness: FreshnessState,
        retrieval_epoch: &str,
    ) -> Self {
        let host_lane = action.host_lane();
        let effective_gate = worse_gate(
            provenance_state.gate_posture(),
            freshness_gate(evidence_freshness),
        );
        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            flow: action.flow(),
            flow_label: action.flow().label().to_owned(),
            surface: action.surface(),
            surface_label: action.surface().label().to_owned(),
            event_id: action.event_id(),
            mutation_ref: action.mutation_ref(),
            run_ref: action.run_ref(),
            session_ref: action.session_ref(),
            host_lane,
            host_lane_label: host_lane.label().to_owned(),
            retrieval_epoch: retrieval_epoch.to_owned(),
            redaction_posture: action.redaction_posture(),
            redaction_posture_label: action.redaction_posture().label().to_owned(),
            provenance_state,
            provenance_state_label: provenance_state.label().to_owned(),
            is_local: host_lane.is_local(),
            evidence_freshness,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            trust_boundaries: host_lane.trust_boundaries(),
            crosses_trust_boundary: !host_lane.is_local(),
            evidence_class: EvidenceClass::ProvenanceLedger,
            owner_role: PROVENANCE_OWNER_ROLE.to_owned(),
            proof_ref: PROVENANCE_PROOF_REF.to_owned(),
            export_safe_summary: action.export_safe_summary().to_owned(),
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}provenance.{}",
                M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// Validates the row's invariants: every derived field matches the action, the effective gate
    /// matches the provenance state and freshness, the linkage and disclosure are complete, and the
    /// message id carries the lane prefix.
    fn validate(&self) -> Vec<M5EventProvenanceViolation> {
        let mut out = Vec::new();
        let probe = Self::derive(
            self.action,
            self.provenance_state,
            self.evidence_freshness,
            &self.retrieval_epoch,
        );
        if probe != *self {
            out.push(M5EventProvenanceViolation::ProvenanceRowDrift);
        }
        let expected_gate = worse_gate(
            self.provenance_state.gate_posture(),
            freshness_gate(self.evidence_freshness),
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5EventProvenanceViolation::ProvenanceGateDrift);
        }
        if self.event_id.trim().is_empty()
            || self.mutation_ref.trim().is_empty()
            || self.run_ref.trim().is_empty()
            || self.session_ref.trim().is_empty()
        {
            out.push(M5EventProvenanceViolation::ProvenanceLinkageIncomplete);
        }
        if self.retrieval_epoch.trim().is_empty()
            || self.proof_ref.trim().is_empty()
            || self.export_safe_summary.trim().is_empty()
        {
            out.push(M5EventProvenanceViolation::ProvenanceDisclosureIncomplete);
        }
        if !self
            .detail_message_id
            .starts_with(M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX)
        {
            out.push(M5EventProvenanceViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Route-drift banner
// ---------------------------------------------------------------------------------------------

/// Seed input for one drifted route fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DriftFactSeed {
    /// The route fact that drifted.
    pub facet: DriftFacet,
    /// An export-safe category ref for the planned value.
    pub planned_ref: &'static str,
    /// An export-safe category ref for the current value.
    pub current_ref: &'static str,
}

/// One drifted route fact: the facet that changed, the baseline it changed from, export-safe refs for
/// the planned and current category values, and the gate the change imposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftFactEntry {
    /// The route fact that drifted.
    pub facet: DriftFacet,
    /// Reader-facing facet label.
    pub facet_label: String,
    /// The baseline the fact changed from.
    pub baseline: DriftBaseline,
    /// Export-safe category ref for the planned value (no raw identifiers).
    pub planned_ref: String,
    /// Export-safe category ref for the current value (no raw identifiers).
    pub current_ref: String,
    /// The gate posture this drift imposes.
    pub facet_gate: DescriptorGate,
    /// Stable message id; prefixed [`M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl DriftFactEntry {
    /// Builds a drift-fact entry from its seed at the given action and baseline.
    fn from_seed(action: DeferredAction, baseline: DriftBaseline, seed: DriftFactSeed) -> Self {
        Self {
            facet: seed.facet,
            facet_label: seed.facet.label().to_owned(),
            baseline,
            planned_ref: seed.planned_ref.to_owned(),
            current_ref: seed.current_ref.to_owned(),
            facet_gate: seed.facet.gate_posture(),
            detail_message_id: format!(
                "{}drift.{}.{}",
                M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX,
                action.as_str(),
                seed.facet.as_str()
            ),
        }
    }
}

/// One route-drift banner: for one deferred action, the baseline it compares against, the canonical
/// [route state](RouteHopState), and every route fact that drifted since the baseline. The banner's
/// effective gate folds in the worst drifted-fact gate and the route evidence freshness, so a changed
/// route never reads more attributable than its facts allow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDriftBanner {
    /// The action this banner describes.
    pub action: DeferredAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The baseline the current route facts are compared against.
    pub baseline: DriftBaseline,
    /// Reader-facing baseline label.
    pub baseline_label: String,
    /// The canonical route-hop state for the action's current route.
    pub route_state: RouteHopState,
    /// Reader-facing route-state label.
    pub route_state_label: String,
    /// Every route fact that drifted since the baseline.
    pub drifted_facets: Vec<DriftFactEntry>,
    /// Count of drifted facts.
    pub drift_count: u32,
    /// True when at least one route fact drifted.
    pub has_drift: bool,
    /// Freshness of the route evidence.
    pub evidence_freshness: FreshnessState,
    /// The gate the route state, drift, and freshness together imply (the most restrictive).
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The evidence class backing the banner.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the route proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the banner.
    pub proof_ref: String,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX`].
    pub banner_message_id: String,
}

impl RouteDriftBanner {
    /// Derives a route-drift banner from the action, its route state, baseline, drifted facts, and
    /// evidence freshness. The effective gate is the worst of the route state's gate, the worst
    /// drifted-fact gate, and the freshness gate.
    fn derive(
        action: DeferredAction,
        route_state: RouteHopState,
        baseline: DriftBaseline,
        fact_seeds: &[DriftFactSeed],
        evidence_freshness: FreshnessState,
    ) -> Self {
        let drifted_facets: Vec<DriftFactEntry> = fact_seeds
            .iter()
            .map(|seed| DriftFactEntry::from_seed(action, baseline, *seed))
            .collect();
        let worst_drift_gate = drifted_facets
            .iter()
            .map(|f| f.facet_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        let effective_gate = worse_gate(
            worse_gate(route_state.gate_posture(), worst_drift_gate),
            freshness_gate(evidence_freshness),
        );
        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            baseline,
            baseline_label: baseline.label().to_owned(),
            route_state,
            route_state_label: route_state.label().to_owned(),
            drift_count: drifted_facets.len() as u32,
            has_drift: !drifted_facets.is_empty(),
            drifted_facets,
            evidence_freshness,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            evidence_class: EvidenceClass::RouteTimeline,
            owner_role: ROUTE_OWNER_ROLE.to_owned(),
            proof_ref: ROUTE_PROOF_REF.to_owned(),
            status,
            signal: status.signal(),
            banner_message_id: format!(
                "{}drift.{}.banner",
                M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// Validates the banner's invariants: every derived field matches the action, the effective gate
    /// folds the route state, drift, and freshness, and the message ids carry the lane prefix.
    fn validate(&self) -> Vec<M5EventProvenanceViolation> {
        let mut out = Vec::new();
        let worst_drift_gate = self
            .drifted_facets
            .iter()
            .map(|f| f.facet_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        let expected_gate = worse_gate(
            worse_gate(self.route_state.gate_posture(), worst_drift_gate),
            freshness_gate(self.evidence_freshness),
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5EventProvenanceViolation::DriftGateDrift);
        }
        if self.drift_count as usize != self.drifted_facets.len()
            || self.has_drift != !self.drifted_facets.is_empty()
        {
            out.push(M5EventProvenanceViolation::DriftBannerInconsistent);
        }
        for fact in &self.drifted_facets {
            if fact.facet_gate != fact.facet.gate_posture()
                || fact.facet_label != fact.facet.label()
                || fact.baseline != self.baseline
                || fact.planned_ref.trim().is_empty()
                || fact.current_ref.trim().is_empty()
            {
                out.push(M5EventProvenanceViolation::DriftBannerInconsistent);
            }
            if !fact
                .detail_message_id
                .starts_with(M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX)
            {
                out.push(M5EventProvenanceViolation::UnprefixedMessageId);
            }
        }
        if self.proof_ref.trim().is_empty()
            || !self
                .banner_message_id
                .starts_with(M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX)
        {
            out.push(M5EventProvenanceViolation::DriftDisclosureIncomplete);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Replay / reapproval gate
// ---------------------------------------------------------------------------------------------

/// One replay / reapproval gate: for one deferred action, the kind of replay it carries, the current
/// [capability-boundary state](CapabilityBoundaryState) and [approval state](ApprovalState), and the
/// [decision](ReapprovalDecision) those facts imply. When current boundary facts invalidate the
/// earlier route or approval assumptions, the gate requires re-approval or holds the action rather
/// than continuing silently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayReapprovalGate {
    /// The action this gate describes.
    pub action: DeferredAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The replay semantics the deferred action carries.
    pub deferred_kind: DeferredKind,
    /// Reader-facing deferred-kind label.
    pub deferred_kind_label: String,
    /// The current capability-boundary state.
    pub boundary_state: CapabilityBoundaryState,
    /// Reader-facing boundary-state label.
    pub boundary_state_label: String,
    /// The current approval state.
    pub approval_state: ApprovalState,
    /// Reader-facing approval-state label.
    pub approval_state_label: String,
    /// The decision the current facts imply.
    pub decision: ReapprovalDecision,
    /// Reader-facing decision label.
    pub decision_label: String,
    /// Freshness of the approval evidence.
    pub evidence_freshness: FreshnessState,
    /// The gate the boundary state, approval state, and freshness together imply (the most
    /// restrictive).
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The evidence class backing the gate.
    pub evidence_class: EvidenceClass,
    /// Owner role accountable for the approval proof.
    pub owner_role: String,
    /// Repo-relative proof ref backing the gate.
    pub proof_ref: String,
    /// The approval ticket this gate links to (ref only).
    pub ticket_ref: String,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id; prefixed [`M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl ReplayReapprovalGate {
    /// Derives a replay / reapproval gate from the action, its boundary state, approval state, and
    /// evidence freshness. The effective gate is the worst of the boundary state's gate, the approval
    /// state's gate, and the freshness gate; the decision follows the effective gate.
    fn derive(
        action: DeferredAction,
        boundary_state: CapabilityBoundaryState,
        approval_state: ApprovalState,
        evidence_freshness: FreshnessState,
    ) -> Self {
        let effective_gate = worse_gate(
            worse_gate(boundary_state.gate_posture(), approval_state.gate_posture()),
            freshness_gate(evidence_freshness),
        );
        let decision = ReapprovalDecision::for_gate(effective_gate);
        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            deferred_kind: action.deferred_kind(),
            deferred_kind_label: action.deferred_kind().label().to_owned(),
            boundary_state,
            boundary_state_label: boundary_state.label().to_owned(),
            approval_state,
            approval_state_label: approval_state.label().to_owned(),
            decision,
            decision_label: decision.label().to_owned(),
            evidence_freshness,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            evidence_class: EvidenceClass::RuntimeApprovalRecord,
            owner_role: REAPPROVAL_OWNER_ROLE.to_owned(),
            proof_ref: REAPPROVAL_PROOF_REF.to_owned(),
            ticket_ref: format!("ticket:{}", action.as_str()),
            status,
            signal: status.signal(),
            detail_message_id: format!(
                "{}reapproval.{}",
                M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// Validates the gate's invariants: every derived field matches the action, the effective gate
    /// folds boundary / approval / freshness, the decision follows the gate, and the message id
    /// carries the lane prefix.
    fn validate(&self) -> Vec<M5EventProvenanceViolation> {
        let mut out = Vec::new();
        let probe = Self::derive(
            self.action,
            self.boundary_state,
            self.approval_state,
            self.evidence_freshness,
        );
        if probe != *self {
            out.push(M5EventProvenanceViolation::ReapprovalGateDrift);
        }
        let expected_gate = worse_gate(
            worse_gate(
                self.boundary_state.gate_posture(),
                self.approval_state.gate_posture(),
            ),
            freshness_gate(self.evidence_freshness),
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.decision != ReapprovalDecision::for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5EventProvenanceViolation::ReapprovalDecisionDrift);
        }
        if self.ticket_ref.trim().is_empty() || self.proof_ref.trim().is_empty() {
            out.push(M5EventProvenanceViolation::ReapprovalDisclosureIncomplete);
        }
        if !self
            .detail_message_id
            .starts_with(M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX)
        {
            out.push(M5EventProvenanceViolation::UnprefixedMessageId);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Deferred event
// ---------------------------------------------------------------------------------------------

/// One per-facet gap on a deferred event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventGap {
    /// The action whose facet opened the gap.
    pub action: DeferredAction,
    /// The facet that opened the gap.
    pub facet: EventFacet,
    /// Whether the facet narrowed or blocked the event.
    pub gap_kind: EventGapKind,
    /// Stable message id naming the cause; prefixed [`M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX`].
    pub cause_message_id: String,
}

/// One deferred event: the three facets (an event-provenance row, a route-drift banner, and a
/// replay / reapproval gate), rolled up to a verdict that is the *worst* gate of the three, so the
/// event never reads safer than its least-attested facet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredEvent {
    /// The action this event explains.
    pub action: DeferredAction,
    /// Reader-facing action label.
    pub action_label: String,
    /// The flow the action belongs to.
    pub flow: ActionFlow,
    /// Reader-facing flow label.
    pub flow_label: String,
    /// The event-provenance row.
    pub provenance_row: EventProvenanceRow,
    /// The route-drift banner.
    pub drift_banner: RouteDriftBanner,
    /// The replay / reapproval gate.
    pub reapproval_gate: ReplayReapprovalGate,
    /// The effective gate — the worst of the three facets' gates.
    pub effective_gate: DescriptorGate,
    /// Effective qualification implied by the effective gate.
    pub effective_qualification: QualificationClass,
    /// The trust boundaries the event spans.
    pub trust_boundaries: Vec<TrustBoundary>,
    /// True when the event leaves the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// The exact per-facet gaps for this event.
    pub gaps: Vec<EventGap>,
    /// The evaluation / export actions offered for this event.
    pub evaluation_actions: Vec<EvaluationAction>,
    /// Coverage status implied by the effective gate.
    pub status: ConsumerStatus,
    /// Traffic-light signal (mirrors [`Self::status`]).
    pub signal: DescriptorSignal,
    /// Stable message id for the event verdict; prefixed [`M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX`].
    pub verdict_message_id: String,
}

impl DeferredEvent {
    /// Derives a deferred event from its seed, building the three facets and folding their gates.
    fn derive(seed: &DeferredEventSeed) -> Self {
        let action = seed.action;
        let provenance_row = EventProvenanceRow::derive(
            action,
            seed.provenance_state,
            seed.provenance_freshness,
            &seed.retrieval_epoch,
        );
        let drift_banner = RouteDriftBanner::derive(
            action,
            seed.route_state,
            seed.baseline,
            &seed.drifted_facets,
            seed.route_freshness,
        );
        let reapproval_gate = ReplayReapprovalGate::derive(
            action,
            seed.boundary_state,
            seed.approval_state,
            seed.approval_freshness,
        );

        let effective_gate = worse_gate(
            worse_gate(provenance_row.effective_gate, drift_banner.effective_gate),
            reapproval_gate.effective_gate,
        );

        // Gaps: one per facet whose effective gate is not governed.
        let mut gaps = Vec::new();
        for (facet, gate) in [
            (EventFacet::Provenance, provenance_row.effective_gate),
            (EventFacet::Drift, drift_banner.effective_gate),
            (EventFacet::Reapproval, reapproval_gate.effective_gate),
        ] {
            if let Some(kind) = gap_kind_for_gate(gate) {
                gaps.push(EventGap {
                    action,
                    facet,
                    gap_kind: kind,
                    cause_message_id: format!(
                        "{}event.{}.{}.{}.gap",
                        M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX,
                        action.as_str(),
                        facet.as_str(),
                        kind.as_str()
                    ),
                });
            }
        }

        let status = gate_status(effective_gate);
        Self {
            action,
            action_label: action.label().to_owned(),
            flow: action.flow(),
            flow_label: action.flow().label().to_owned(),
            trust_boundaries: provenance_row.trust_boundaries.clone(),
            crosses_trust_boundary: provenance_row.crosses_trust_boundary,
            provenance_row,
            drift_banner,
            reapproval_gate,
            effective_gate,
            effective_qualification: floor_for_gate(effective_gate),
            gaps,
            evaluation_actions: EvaluationAction::ALL.to_vec(),
            status,
            signal: status.signal(),
            verdict_message_id: format!(
                "{}event.{}.verdict",
                M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX,
                action.as_str()
            ),
        }
    }

    /// True when every facet stands fully governed.
    pub fn is_governed(&self) -> bool {
        matches!(self.effective_gate, DescriptorGate::Governed)
    }

    /// True when a facet narrowed the event below fully governed.
    pub fn is_narrowed(&self) -> bool {
        matches!(self.effective_gate, DescriptorGate::Narrowed)
    }

    /// True when a facet blocked the event.
    pub fn is_blocked(&self) -> bool {
        matches!(self.effective_gate, DescriptorGate::Blocked)
    }

    /// Validates the event's invariants: the facets self-validate, the effective gate is the worst of
    /// the three, the gaps match the facet gates, and the message ids carry the lane prefix.
    fn validate(&self) -> Vec<M5EventProvenanceViolation> {
        let mut out = Vec::new();
        if self.provenance_row.action != self.action
            || self.drift_banner.action != self.action
            || self.reapproval_gate.action != self.action
            || self.action_label != self.action.label()
            || self.flow != self.action.flow()
        {
            out.push(M5EventProvenanceViolation::EventFieldMismatch);
        }
        out.extend(self.provenance_row.validate());
        out.extend(self.drift_banner.validate());
        out.extend(self.reapproval_gate.validate());

        let expected_gate = worse_gate(
            worse_gate(
                self.provenance_row.effective_gate,
                self.drift_banner.effective_gate,
            ),
            self.reapproval_gate.effective_gate,
        );
        if self.effective_gate != expected_gate
            || self.effective_qualification != floor_for_gate(expected_gate)
            || self.status != gate_status(expected_gate)
            || self.signal != self.status.signal()
        {
            out.push(M5EventProvenanceViolation::EventGateDrift);
        }

        // Gaps must name exactly the not-governed facets.
        let expected_gaps: Vec<(EventFacet, EventGapKind)> = [
            (EventFacet::Provenance, self.provenance_row.effective_gate),
            (EventFacet::Drift, self.drift_banner.effective_gate),
            (EventFacet::Reapproval, self.reapproval_gate.effective_gate),
        ]
        .into_iter()
        .filter_map(|(facet, gate)| gap_kind_for_gate(gate).map(|kind| (facet, kind)))
        .collect();
        let actual_gaps: Vec<(EventFacet, EventGapKind)> =
            self.gaps.iter().map(|g| (g.facet, g.gap_kind)).collect();
        if actual_gaps != expected_gaps {
            out.push(M5EventProvenanceViolation::EventGapDrift);
        }
        for gap in &self.gaps {
            if !gap
                .cause_message_id
                .starts_with(M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX)
            {
                out.push(M5EventProvenanceViolation::UnprefixedMessageId);
            }
        }
        if self.evaluation_actions != EvaluationAction::ALL.to_vec()
            || !self
                .verdict_message_id
                .starts_with(M5_EVENT_PROVENANCE_MESSAGE_ID_PREFIX)
        {
            out.push(M5EventProvenanceViolation::EventFieldMismatch);
        }
        out
    }
}

// ---------------------------------------------------------------------------------------------
// Export preview (redaction-safe)
// ---------------------------------------------------------------------------------------------

/// One event entry in the exported redaction-safe preview — the same provenance / route / approval
/// vocabulary the facets show, reduced to refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventExportEntry {
    /// The action token.
    pub action: DeferredAction,
    /// The flow.
    pub flow: ActionFlow,
    /// The surface the provenance row attaches to.
    pub surface: ProvenanceSurface,
    /// The host lane.
    pub host_lane: HostLane,
    /// The event id (ref only).
    pub event_id: String,
    /// The mutation ref.
    pub mutation_ref: String,
    /// The run ref.
    pub run_ref: String,
    /// The session ref.
    pub session_ref: String,
    /// The redaction posture.
    pub redaction_posture: RedactionPosture,
    /// The provenance state.
    pub provenance_state: ProvenanceState,
    /// The route state.
    pub route_state: RouteHopState,
    /// The boundary state.
    pub boundary_state: CapabilityBoundaryState,
    /// The approval state.
    pub approval_state: ApprovalState,
    /// The deferred kind.
    pub deferred_kind: DeferredKind,
    /// The reapproval decision.
    pub reapproval_decision: ReapprovalDecision,
    /// The route facts that drifted since the baseline.
    pub drifted_facets: Vec<DriftFacet>,
    /// The event's effective gate.
    pub effective_gate: DescriptorGate,
    /// Effective qualification.
    pub effective_qualification: QualificationClass,
    /// True when the event crosses the local-first trust boundary.
    pub crosses_trust_boundary: bool,
    /// The proof refs backing the event's facets (refs only).
    pub proof_refs: Vec<String>,
}

/// The exported redaction-safe preview: each deferred event reduced to the exact provenance / route /
/// approval vocabulary the in-product facets show, so an exported support / audit pack and the live
/// inspector can never read differently. The preview is metadata-only: it preserves event / route /
/// proof lineage as refs and carries no credential bodies or raw provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventProvenanceExportPreview {
    /// Record kind; must equal [`M5_EVENT_PROVENANCE_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; mirrors the parent packet.
    pub schema_version: u32,
    /// Stable export-preview id.
    pub packet_id: String,
    /// The event-provenance packet this export was generated from.
    pub generated_from: String,
    /// The evaluation date the preview was computed as-of.
    pub evaluated_at: String,
    /// The event entries.
    pub events: Vec<EventExportEntry>,
    /// The controlled vocabulary the entries draw from.
    pub vocabulary: EventProvenanceVocabulary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

impl EventProvenanceExportPreview {
    /// Builds the export preview from the deferred events.
    fn derive(
        packet_id: &str,
        generated_from: &str,
        evaluated_at: &str,
        redaction_class_token: &str,
        events: &[DeferredEvent],
    ) -> Self {
        let entries = events
            .iter()
            .map(|e| EventExportEntry {
                action: e.action,
                flow: e.flow,
                surface: e.provenance_row.surface,
                host_lane: e.provenance_row.host_lane,
                event_id: e.provenance_row.event_id.clone(),
                mutation_ref: e.provenance_row.mutation_ref.clone(),
                run_ref: e.provenance_row.run_ref.clone(),
                session_ref: e.provenance_row.session_ref.clone(),
                redaction_posture: e.provenance_row.redaction_posture,
                provenance_state: e.provenance_row.provenance_state,
                route_state: e.drift_banner.route_state,
                boundary_state: e.reapproval_gate.boundary_state,
                approval_state: e.reapproval_gate.approval_state,
                deferred_kind: e.reapproval_gate.deferred_kind,
                reapproval_decision: e.reapproval_gate.decision,
                drifted_facets: e
                    .drift_banner
                    .drifted_facets
                    .iter()
                    .map(|f| f.facet)
                    .collect(),
                effective_gate: e.effective_gate,
                effective_qualification: e.effective_qualification,
                crosses_trust_boundary: e.crosses_trust_boundary,
                proof_refs: vec![
                    e.provenance_row.proof_ref.clone(),
                    e.drift_banner.proof_ref.clone(),
                    e.reapproval_gate.proof_ref.clone(),
                ],
            })
            .collect();
        Self {
            record_kind: M5_EVENT_PROVENANCE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_EVENT_PROVENANCE_SCHEMA_VERSION,
            packet_id: packet_id.to_owned(),
            generated_from: generated_from.to_owned(),
            evaluated_at: evaluated_at.to_owned(),
            events: entries,
            vocabulary: EventProvenanceVocabulary::canonical(),
            redaction_class_token: redaction_class_token.to_owned(),
        }
    }

    /// Deterministic export-safe JSON for the export preview.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 event provenance export preview serializes")
    }

    /// True when every token the preview carries is a member of the canonical vocabulary, so the
    /// export reuses the same grammar the inspector shows.
    fn reuses_canonical_vocabulary(&self) -> bool {
        if !self.vocabulary.matches_canonical() {
            return false;
        }
        let vocab = &self.vocabulary;
        self.events.iter().all(|e| {
            vocab.actions.contains(&e.action.as_str().to_owned())
                && vocab.flows.contains(&e.flow.as_str().to_owned())
                && vocab.surfaces.contains(&e.surface.as_str().to_owned())
                && vocab.host_lanes.contains(&e.host_lane.as_str().to_owned())
                && vocab
                    .redaction_postures
                    .contains(&e.redaction_posture.as_str().to_owned())
                && vocab
                    .provenance_states
                    .contains(&e.provenance_state.as_str().to_owned())
                && vocab
                    .route_states
                    .contains(&e.route_state.as_str().to_owned())
                && vocab
                    .boundary_states
                    .contains(&e.boundary_state.as_str().to_owned())
                && vocab
                    .approval_states
                    .contains(&e.approval_state.as_str().to_owned())
                && vocab
                    .deferred_kinds
                    .contains(&e.deferred_kind.as_str().to_owned())
                && vocab
                    .reapproval_decisions
                    .contains(&e.reapproval_decision.as_str().to_owned())
                && e.drifted_facets
                    .iter()
                    .all(|f| vocab.drift_facets.contains(&f.as_str().to_owned()))
        })
    }
}

// ---------------------------------------------------------------------------------------------
// Vocabulary / summary / conformance
// ---------------------------------------------------------------------------------------------

/// Self-describing controlled-vocabulary set so the packet resolves every token it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventProvenanceVocabulary {
    /// Deferred-action tokens.
    pub actions: Vec<String>,
    /// Flow tokens.
    pub flows: Vec<String>,
    /// Provenance-surface tokens.
    pub surfaces: Vec<String>,
    /// Host-lane tokens.
    pub host_lanes: Vec<String>,
    /// Redaction-posture tokens.
    pub redaction_postures: Vec<String>,
    /// Provenance-state tokens.
    pub provenance_states: Vec<String>,
    /// Drift-baseline tokens.
    pub drift_baselines: Vec<String>,
    /// Drift-facet tokens.
    pub drift_facets: Vec<String>,
    /// Route-hop state tokens.
    pub route_states: Vec<String>,
    /// Deferred-kind tokens.
    pub deferred_kinds: Vec<String>,
    /// Capability-boundary state tokens.
    pub boundary_states: Vec<String>,
    /// Approval state tokens.
    pub approval_states: Vec<String>,
    /// Reapproval-decision tokens.
    pub reapproval_decisions: Vec<String>,
    /// Evaluation-action tokens.
    pub evaluation_actions: Vec<String>,
    /// Event-facet tokens.
    pub facets: Vec<String>,
    /// Trust-boundary tokens.
    pub trust_boundaries: Vec<String>,
    /// Evidence-class tokens.
    pub evidence_classes: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Qualification-class tokens.
    pub qualification_classes: Vec<String>,
    /// Gate-decision tokens.
    pub gate_decisions: Vec<String>,
}

impl EventProvenanceVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            actions: tokens(&DeferredAction::ALL, |a| a.as_str()),
            flows: tokens(&ActionFlow::ALL, |f| f.as_str()),
            surfaces: tokens(&ProvenanceSurface::ALL, |s| s.as_str()),
            host_lanes: tokens(&HostLane::ALL, |l| l.as_str()),
            redaction_postures: tokens(&RedactionPosture::ALL, |r| r.as_str()),
            provenance_states: tokens(&ProvenanceState::ALL, |s| s.as_str()),
            drift_baselines: tokens(&DriftBaseline::ALL, |b| b.as_str()),
            drift_facets: tokens(&DriftFacet::ALL, |f| f.as_str()),
            route_states: tokens(&RouteHopState::ALL, |s| s.as_str()),
            deferred_kinds: tokens(&DeferredKind::ALL, |k| k.as_str()),
            boundary_states: tokens(&CapabilityBoundaryState::ALL, |s| s.as_str()),
            approval_states: tokens(&ApprovalState::ALL, |s| s.as_str()),
            reapproval_decisions: tokens(&ReapprovalDecision::ALL, |d| d.as_str()),
            evaluation_actions: tokens(&EvaluationAction::ALL, |a| a.as_str()),
            facets: tokens(&EventFacet::ALL, |f| f.as_str()),
            trust_boundaries: tokens(&TrustBoundary::ALL, |b| b.as_str()),
            evidence_classes: tokens(&EvidenceClass::ALL, |c| c.as_str()),
            freshness_states: tokens(&FreshnessState::ALL, |f| f.as_str()),
            qualification_classes: tokens(&QualificationClass::ALL, |q| q.as_str()),
            gate_decisions: tokens(&DescriptorGate::ALL, |g| g.as_str()),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Compact event-provenance summary — the scoreboard the renderers and exports read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventProvenanceSummary {
    /// Total deferred events.
    pub total_events: u32,
    /// Events fully governed.
    pub governed_events: u32,
    /// Events narrowed by a facet.
    pub narrowed_events: u32,
    /// Events blocked by a facet.
    pub blocked_events: u32,
    /// Events that cross the local-first trust boundary.
    pub crossing_events: u32,
    /// Events whose route carries at least one drifted fact.
    pub drifted_events: u32,
    /// Events whose reapproval gate requires re-approval.
    pub reapproval_required: u32,
    /// Events whose reapproval gate holds the action.
    pub reapproval_blocked: u32,
    /// Total event-provenance rows.
    pub total_provenance_rows: u32,
    /// Total route-drift banners.
    pub total_drift_banners: u32,
    /// Total replay / reapproval gates.
    pub total_reapproval_gates: u32,
    /// True when at least one event is blocked.
    pub blocks_stable_promotion: bool,
}

/// Conformance review. Every flag is a hard invariant; all must hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventProvenanceConformance {
    /// Every provenance row links the event to a mutation, run, and session.
    pub provenance_row_links_event_mutation_run_session: bool,
    /// Every provenance row declares its host lane, retrieval epoch, and redaction posture.
    pub provenance_row_declares_host_lane_epoch_and_redaction: bool,
    /// Every route-drift banner names the changed facts and the baseline they changed from.
    pub drift_banner_names_changed_facts_and_baseline: bool,
    /// Every reapproval gate binds the deferred kind, boundary state, and approval state.
    pub reapproval_gate_binds_kind_boundary_and_approval: bool,
    /// Every event's gate is the worst of its three facets — it never overstates.
    pub event_gate_is_worst_of_facets: bool,
    /// A route-fact drift narrows or blocks the banner deterministically.
    pub drift_narrows_or_blocks_deterministically: bool,
    /// Changed boundary facts force a re-approval or hold instead of a silent replay.
    pub changed_boundary_facts_require_reapproval: bool,
    /// A tenant drift blocks the event and holds Stable promotion.
    pub tenant_drift_blocks_stable_promotion: bool,
    /// Every provenance row binds the provenance vocabulary.
    pub provenance_state_bound_to_provenance_vocabulary: bool,
    /// Every drift banner binds the route-hop vocabulary.
    pub route_state_bound_to_route_vocabulary: bool,
    /// Every reapproval gate binds the runtime approval vocabulary.
    pub approval_state_bound_to_approval_vocabulary: bool,
    /// The exported preview reuses the in-product vocabulary.
    pub export_preview_reuses_ui_vocabulary: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// The export carries no raw provider material — event / route / proof lineage stays refs-only.
    pub export_carries_no_raw_material: bool,
}

impl EventProvenanceConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.provenance_row_links_event_mutation_run_session
            && self.provenance_row_declares_host_lane_epoch_and_redaction
            && self.drift_banner_names_changed_facts_and_baseline
            && self.reapproval_gate_binds_kind_boundary_and_approval
            && self.event_gate_is_worst_of_facets
            && self.drift_narrows_or_blocks_deterministically
            && self.changed_boundary_facts_require_reapproval
            && self.tenant_drift_blocks_stable_promotion
            && self.provenance_state_bound_to_provenance_vocabulary
            && self.route_state_bound_to_route_vocabulary
            && self.approval_state_bound_to_approval_vocabulary
            && self.export_preview_reuses_ui_vocabulary
            && self.controlled_enums_frozen
            && self.export_carries_no_raw_material
    }
}

// ---------------------------------------------------------------------------------------------
// Packet
// ---------------------------------------------------------------------------------------------

/// Seed input for one deferred event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredEventSeed {
    /// The action.
    pub action: DeferredAction,
    /// The provenance state.
    pub provenance_state: ProvenanceState,
    /// Freshness of the provenance evidence.
    pub provenance_freshness: FreshnessState,
    /// The retrieval epoch the row was read as-of.
    pub retrieval_epoch: String,
    /// The route-hop state of the current route.
    pub route_state: RouteHopState,
    /// Freshness of the route evidence.
    pub route_freshness: FreshnessState,
    /// The baseline the route facts are compared against.
    pub baseline: DriftBaseline,
    /// The route facts that drifted since the baseline.
    pub drifted_facets: Vec<DriftFactSeed>,
    /// The capability-boundary state.
    pub boundary_state: CapabilityBoundaryState,
    /// The approval state.
    pub approval_state: ApprovalState,
    /// Freshness of the approval evidence.
    pub approval_freshness: FreshnessState,
}

/// Constructor input for [`M5EventProvenance::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EventProvenanceInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The per-event seeds.
    pub event_seeds: Vec<DeferredEventSeed>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable event-provenance truth packet: the per-action deferred
/// events (each carrying an event-provenance row, a route-drift banner, and a replay / reapproval
/// gate), the exported redaction-safe preview, the controlled vocabulary, a summary, and a
/// conformance review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EventProvenance {
    /// Record kind; must equal [`M5_EVENT_PROVENANCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EVENT_PROVENANCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the packet was computed as-of.
    pub evaluated_at: String,
    /// The per-action deferred events, in action order.
    pub deferred_events: Vec<DeferredEvent>,
    /// The exported redaction-safe preview.
    pub export_preview: EventProvenanceExportPreview,
    /// Controlled-vocabulary set.
    pub vocabulary: EventProvenanceVocabulary,
    /// Compact summary.
    pub summary: EventProvenanceSummary,
    /// Conformance review block.
    pub conformance: EventProvenanceConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EventProvenance {
    /// Builds an event-provenance packet from seed input, deriving each deferred event from its seed
    /// and the summary / conformance / export preview from all of them.
    pub fn new(input: M5EventProvenanceInput) -> Self {
        let mut deferred_events: Vec<DeferredEvent> = input
            .event_seeds
            .iter()
            .map(DeferredEvent::derive)
            .collect();
        deferred_events.sort_by_key(|e| action_rank(e.action));
        deferred_events.dedup_by_key(|e| e.action);

        let export_preview = EventProvenanceExportPreview::derive(
            &format!("{}:preview", input.packet_id),
            &input.packet_id,
            &input.evaluated_at,
            &input.redaction_class_token,
            &deferred_events,
        );

        let summary = derive_summary(&deferred_events);
        let conformance = derive_conformance(&deferred_events, &export_preview);

        Self {
            record_kind: M5_EVENT_PROVENANCE_RECORD_KIND.to_owned(),
            schema_version: M5_EVENT_PROVENANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            deferred_events,
            export_preview,
            vocabulary: EventProvenanceVocabulary::canonical(),
            summary,
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// True when the release automation must hold Stable promotion — at least one event is blocked.
    pub fn blocks_stable_promotion(&self) -> bool {
        self.summary.blocks_stable_promotion
    }

    /// Finds a deferred event by action.
    pub fn event(&self, action: DeferredAction) -> Option<&DeferredEvent> {
        self.deferred_events.iter().find(|e| e.action == action)
    }

    /// The event-provenance rows, in action order.
    pub fn provenance_rows(&self) -> Vec<&EventProvenanceRow> {
        self.deferred_events
            .iter()
            .map(|e| &e.provenance_row)
            .collect()
    }

    /// The route-drift banners, in action order.
    pub fn drift_banners(&self) -> Vec<&RouteDriftBanner> {
        self.deferred_events
            .iter()
            .map(|e| &e.drift_banner)
            .collect()
    }

    /// The replay / reapproval gates, in action order.
    pub fn reapproval_gates(&self) -> Vec<&ReplayReapprovalGate> {
        self.deferred_events
            .iter()
            .map(|e| &e.reapproval_gate)
            .collect()
    }

    /// Renders the packet for a generation channel. The output is identical for every channel — the
    /// channel parameter exists only to prove desktop, CLI / headless, and offline / mirror
    /// generation produce byte-identical output.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn render_for_channel(&self, _channel: EventProvenanceChannel) -> String {
        self.export_safe_json()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 event provenance serializes")
    }

    /// The exported redaction-safe preview's JSON.
    pub fn render_export_preview(&self) -> String {
        self.export_preview.export_safe_json()
    }

    /// Deterministic, machine-readable event / facet matrix CSV: one row per action, naming the
    /// flow / surface, the provenance state, the route state and drift count, the boundary / approval
    /// state, the reapproval decision, and the event verdict.
    pub fn render_events_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "action,flow,surface,host_lane,provenance_state,route_state,drift_count,baseline,boundary_state,approval_state,deferred_kind,reapproval_decision,crosses_boundary,effective_gate,effective_qualification\n",
        );
        for e in &self.deferred_events {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                e.action.as_str(),
                e.flow.as_str(),
                e.provenance_row.surface.as_str(),
                e.provenance_row.host_lane.as_str(),
                e.provenance_row.provenance_state.as_str(),
                e.drift_banner.route_state.as_str(),
                e.drift_banner.drift_count,
                e.drift_banner.baseline.as_str(),
                e.reapproval_gate.boundary_state.as_str(),
                e.reapproval_gate.approval_state.as_str(),
                e.reapproval_gate.deferred_kind.as_str(),
                e.reapproval_gate.decision.as_str(),
                e.crosses_trust_boundary,
                e.effective_gate.as_str(),
                e.effective_qualification.as_str(),
            ));
        }
        out
    }

    /// Deterministic event-provenance overview document for review, support, docs, or evaluator
    /// handoff.
    pub fn render_overview_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Event Provenance\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Events: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_events,
            self.summary.governed_events,
            self.summary.narrowed_events,
            self.summary.blocked_events
        ));
        out.push_str(&format!(
            "- Boundary crossings: {} | Drifted events: {} | Reapproval required: {} | Held: {}\n",
            self.summary.crossing_events,
            self.summary.drifted_events,
            self.summary.reapproval_required,
            self.summary.reapproval_blocked
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));

        out.push_str("\n## Deferred events\n\n");
        out.push_str(
            "| Action | Flow | Provenance | Route | Reapproval | Gate | Qualification |\n",
        );
        out.push_str(
            "|--------|------|------------|-------|------------|------|---------------|\n",
        );
        for e in &self.deferred_events {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                e.action.as_str(),
                e.flow.as_str(),
                e.provenance_row.provenance_state.as_str(),
                e.drift_banner.route_state.as_str(),
                e.reapproval_gate.decision.as_str(),
                e.effective_gate.as_str(),
                e.effective_qualification.as_str(),
            ));
        }

        out.push_str("\n## Event-provenance rows\n\n");
        out.push_str(
            "| Action | Surface | Host lane | Event | Mutation | Run | Session | Redaction |\n",
        );
        out.push_str(
            "|--------|---------|-----------|-------|----------|-----|---------|-----------|\n",
        );
        for r in self.provenance_rows() {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                r.action.as_str(),
                r.surface.as_str(),
                r.host_lane.as_str(),
                r.event_id,
                r.mutation_ref,
                r.run_ref,
                r.session_ref,
                r.redaction_posture.as_str(),
            ));
        }

        out.push_str("\n## Route-drift banners\n\n");
        for b in self.drift_banners() {
            if b.has_drift {
                let facets: Vec<String> = b
                    .drifted_facets
                    .iter()
                    .map(|f| format!("{}[{}]", f.facet.as_str(), f.facet_gate.as_str()))
                    .collect();
                out.push_str(&format!(
                    "- `{}` — `{}` vs `{}`: {} drift → {}\n",
                    b.action.as_str(),
                    b.route_state.as_str(),
                    b.baseline.as_str(),
                    b.drift_count,
                    facets.join(", ")
                ));
            } else {
                out.push_str(&format!(
                    "- `{}` — `{}` vs `{}`: no drift\n",
                    b.action.as_str(),
                    b.route_state.as_str(),
                    b.baseline.as_str(),
                ));
            }
        }

        out.push_str("\n## Replay / reapproval gates\n\n");
        out.push_str("| Action | Kind | Boundary | Approval | Decision |\n");
        out.push_str("|--------|------|----------|----------|----------|\n");
        for g in self.reapproval_gates() {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                g.action.as_str(),
                g.deferred_kind.as_str(),
                g.boundary_state.as_str(),
                g.approval_state.as_str(),
                g.decision.as_str(),
            ));
        }
        out
    }

    /// Compact Markdown report for the release-grade parity proof.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Event Provenance — Proof\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!(
            "- Events: {} ({} governed, {} narrowed, {} blocked)\n",
            self.summary.total_events,
            self.summary.governed_events,
            self.summary.narrowed_events,
            self.summary.blocked_events
        ));
        out.push_str(&format!(
            "- Stable promotion: {}\n",
            if self.summary.blocks_stable_promotion {
                "blocked"
            } else {
                "pass"
            }
        ));
        out.push_str(&format!(
            "- Export preview: `{}`\n",
            M5_EVENT_PROVENANCE_EXPORT_PREVIEW_REF
        ));
        out.push_str(&format!(
            "- Events CSV: `{}`\n",
            M5_EVENT_PROVENANCE_EVENTS_CSV_REF
        ));
        out
    }

    /// Validates the packet's invariants.
    pub fn validate(&self) -> Vec<M5EventProvenanceViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_EVENT_PROVENANCE_RECORD_KIND {
            out.push(M5EventProvenanceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EVENT_PROVENANCE_SCHEMA_VERSION {
            out.push(M5EventProvenanceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5EventProvenanceViolation::MissingIdentity);
        }

        // Every action inspected exactly once and self-consistent.
        let mut seen = std::collections::BTreeSet::new();
        for event in &self.deferred_events {
            if !seen.insert(event.action) {
                out.push(M5EventProvenanceViolation::DuplicateAction);
            }
            out.extend(event.validate());
        }
        for action in DeferredAction::ALL {
            if !self.deferred_events.iter().any(|e| e.action == action) {
                out.push(M5EventProvenanceViolation::ActionNotInspected);
            }
        }

        let expected_preview = EventProvenanceExportPreview::derive(
            &self.export_preview.packet_id,
            &self.packet_id,
            &self.evaluated_at,
            &self.redaction_class_token,
            &self.deferred_events,
        );
        if self.export_preview != expected_preview
            || !self.export_preview.reuses_canonical_vocabulary()
        {
            out.push(M5EventProvenanceViolation::ExportPreviewDrift);
        }

        if !self.vocabulary.matches_canonical() {
            out.push(M5EventProvenanceViolation::VocabularyMismatch);
        }
        if self.summary != derive_summary(&self.deferred_events) {
            out.push(M5EventProvenanceViolation::SummaryDrift);
        }
        if self.conformance != derive_conformance(&self.deferred_events, &self.export_preview)
            || !self.conformance.all_hold()
        {
            out.push(M5EventProvenanceViolation::ConformanceReviewFailed);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 event provenance serializes"),
        ) {
            out.push(M5EventProvenanceViolation::RawMaterialInExport);
        }
        out
    }
}

/// The generation channel an event-provenance packet is produced on. Every channel produces
/// byte-identical output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventProvenanceChannel {
    /// The desktop product UI.
    DesktopUi,
    /// The CLI / headless structured-output path.
    CliHeadless,
    /// Offline / mirror-safe packet generation.
    OfflineMirror,
}

impl EventProvenanceChannel {
    /// Every channel, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopUi, Self::CliHeadless, Self::OfflineMirror];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::OfflineMirror => "offline_mirror",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Derivations
// ---------------------------------------------------------------------------------------------

/// Derives the summary from the deferred events.
fn derive_summary(events: &[DeferredEvent]) -> EventProvenanceSummary {
    let gate_count = |gate: DescriptorGate| -> u32 {
        events.iter().filter(|e| e.effective_gate == gate).count() as u32
    };
    let blocked = gate_count(DescriptorGate::Blocked);
    EventProvenanceSummary {
        total_events: events.len() as u32,
        governed_events: gate_count(DescriptorGate::Governed),
        narrowed_events: gate_count(DescriptorGate::Narrowed),
        blocked_events: blocked,
        crossing_events: events.iter().filter(|e| e.crosses_trust_boundary).count() as u32,
        drifted_events: events.iter().filter(|e| e.drift_banner.has_drift).count() as u32,
        reapproval_required: events
            .iter()
            .filter(|e| {
                matches!(
                    e.reapproval_gate.decision,
                    ReapprovalDecision::RequireReapproval
                )
            })
            .count() as u32,
        reapproval_blocked: events
            .iter()
            .filter(|e| matches!(e.reapproval_gate.decision, ReapprovalDecision::HoldBlocked))
            .count() as u32,
        total_provenance_rows: events.len() as u32,
        total_drift_banners: events.len() as u32,
        total_reapproval_gates: events.len() as u32,
        blocks_stable_promotion: blocked > 0,
    }
}

/// Derives the conformance review, so the stored block reflects the actual packet.
fn derive_conformance(
    events: &[DeferredEvent],
    export_preview: &EventProvenanceExportPreview,
) -> EventProvenanceConformance {
    let linkage_ok = events.iter().all(|e| {
        let r = &e.provenance_row;
        !r.event_id.trim().is_empty()
            && !r.mutation_ref.trim().is_empty()
            && !r.run_ref.trim().is_empty()
            && !r.session_ref.trim().is_empty()
            && r.event_id == e.action.event_id()
    });

    let disclosure_ok = events.iter().all(|e| {
        let r = &e.provenance_row;
        !r.retrieval_epoch.trim().is_empty()
            && r.host_lane == e.action.host_lane()
            && r.redaction_posture == e.action.redaction_posture()
            && !r.export_safe_summary.trim().is_empty()
            && !r.proof_ref.trim().is_empty()
    });

    let drift_ok = events.iter().all(|e| {
        let b = &e.drift_banner;
        b.drift_count as usize == b.drifted_facets.len()
            && b.has_drift == !b.drifted_facets.is_empty()
            && b.drifted_facets
                .iter()
                .all(|f| f.baseline == b.baseline && !f.planned_ref.trim().is_empty())
            && !b.proof_ref.trim().is_empty()
    });

    let reapproval_ok = events.iter().all(|e| {
        let g = &e.reapproval_gate;
        g.deferred_kind == e.action.deferred_kind()
            && !g.ticket_ref.trim().is_empty()
            && !g.proof_ref.trim().is_empty()
            && g.decision == ReapprovalDecision::for_gate(g.effective_gate)
    });

    let worst_of_facets = events.iter().all(|e| {
        let expected = worse_gate(
            worse_gate(
                e.provenance_row.effective_gate,
                e.drift_banner.effective_gate,
            ),
            e.reapproval_gate.effective_gate,
        );
        e.effective_gate == expected
    });

    // A banner whose facts carry a narrowing drift must not read governed; a blocking drift must
    // block.
    let drift_narrows = events.iter().all(|e| {
        let worst_drift = e
            .drift_banner
            .drifted_facets
            .iter()
            .map(|f| f.facet_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        match gap_kind_for_gate(worst_drift) {
            None => true,
            Some(EventGapKind::FacetNarrowed) => {
                e.drift_banner.effective_gate != DescriptorGate::Governed
            }
            Some(EventGapKind::FacetBlocked) => {
                e.drift_banner.effective_gate == DescriptorGate::Blocked
            }
        }
    });

    // A reapproval gate that is not governed must not read replay_as_is; a tenant drift blocks.
    let changed_facts_reapprove = events.iter().all(|e| {
        let g = &e.reapproval_gate;
        match g.effective_gate {
            DescriptorGate::Governed => matches!(g.decision, ReapprovalDecision::ReplayAsIs),
            DescriptorGate::Narrowed => matches!(g.decision, ReapprovalDecision::RequireReapproval),
            DescriptorGate::Blocked => matches!(g.decision, ReapprovalDecision::HoldBlocked),
        }
    });
    let tenant_blocks = events.iter().all(|e| {
        let has_tenant_drift = e
            .drift_banner
            .drifted_facets
            .iter()
            .any(|f| matches!(f.facet, DriftFacet::Tenant));
        !has_tenant_drift || e.is_blocked()
    });

    let provenance_vocab = events
        .iter()
        .all(|e| ProvenanceState::ALL.contains(&e.provenance_row.provenance_state));
    let route_vocab = events
        .iter()
        .all(|e| RouteHopState::ALL.contains(&e.drift_banner.route_state));
    let approval_vocab = events
        .iter()
        .all(|e| ApprovalState::ALL.contains(&e.reapproval_gate.approval_state));

    let export_clean =
        !json_contains_forbidden_material(&serde_json::to_value(events).expect("events serialize"));

    EventProvenanceConformance {
        provenance_row_links_event_mutation_run_session: linkage_ok,
        provenance_row_declares_host_lane_epoch_and_redaction: disclosure_ok,
        drift_banner_names_changed_facts_and_baseline: drift_ok,
        reapproval_gate_binds_kind_boundary_and_approval: reapproval_ok,
        event_gate_is_worst_of_facets: worst_of_facets,
        drift_narrows_or_blocks_deterministically: drift_narrows,
        changed_boundary_facts_require_reapproval: changed_facts_reapprove,
        tenant_drift_blocks_stable_promotion: tenant_blocks,
        provenance_state_bound_to_provenance_vocabulary: provenance_vocab,
        route_state_bound_to_route_vocabulary: route_vocab,
        approval_state_bound_to_approval_vocabulary: approval_vocab,
        export_preview_reuses_ui_vocabulary: export_preview.reuses_canonical_vocabulary(),
        controlled_enums_frozen: EventProvenanceVocabulary::canonical().matches_canonical(),
        export_carries_no_raw_material: export_clean,
    }
}

// ---------------------------------------------------------------------------------------------
// Ranking / token helpers
// ---------------------------------------------------------------------------------------------

/// Position of an action in the canonical ordering.
fn action_rank(action: DeferredAction) -> usize {
    DeferredAction::ALL
        .iter()
        .position(|a| *a == action)
        .unwrap_or(DeferredAction::ALL.len())
}

/// Maps a typed `ALL` array to its stable tokens.
fn tokens<T: Copy>(all: &[T], f: impl Fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| f(*t).to_owned()).collect()
}

// ---------------------------------------------------------------------------------------------
// Violations
// ---------------------------------------------------------------------------------------------

/// Validation failures for the event-provenance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EventProvenanceViolation {
    /// The packet record kind is wrong.
    WrongRecordKind,
    /// The packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// A provenance row drifted from a fresh derivation of its action.
    ProvenanceRowDrift,
    /// A provenance row's gate or qualification drifted.
    ProvenanceGateDrift,
    /// A provenance row omits its event / mutation / run / session linkage.
    ProvenanceLinkageIncomplete,
    /// A provenance row omits its retrieval epoch, proof ref, or export-safe summary.
    ProvenanceDisclosureIncomplete,
    /// A route-drift banner's gate or qualification drifted.
    DriftGateDrift,
    /// A route-drift banner's drift count, facts, or baseline are inconsistent.
    DriftBannerInconsistent,
    /// A route-drift banner omits a proof ref or message id.
    DriftDisclosureIncomplete,
    /// A reapproval gate drifted from a fresh derivation of its action.
    ReapprovalGateDrift,
    /// A reapproval gate's gate, qualification, or decision drifted.
    ReapprovalDecisionDrift,
    /// A reapproval gate omits its ticket ref or proof ref.
    ReapprovalDisclosureIncomplete,
    /// A deferred event cites a field that does not match its action.
    EventFieldMismatch,
    /// A deferred event's gate or qualification drifted from the worst of its facets.
    EventGateDrift,
    /// A deferred event's gaps do not name exactly the not-governed facets.
    EventGapDrift,
    /// Two events name the same action.
    DuplicateAction,
    /// An action has no event.
    ActionNotInspected,
    /// The export preview drifted from the events or its vocabulary.
    ExportPreviewDrift,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The summary disagrees with the events.
    SummaryDrift,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5EventProvenanceViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ProvenanceRowDrift => "provenance_row_drift",
            Self::ProvenanceGateDrift => "provenance_gate_drift",
            Self::ProvenanceLinkageIncomplete => "provenance_linkage_incomplete",
            Self::ProvenanceDisclosureIncomplete => "provenance_disclosure_incomplete",
            Self::DriftGateDrift => "drift_gate_drift",
            Self::DriftBannerInconsistent => "drift_banner_inconsistent",
            Self::DriftDisclosureIncomplete => "drift_disclosure_incomplete",
            Self::ReapprovalGateDrift => "reapproval_gate_drift",
            Self::ReapprovalDecisionDrift => "reapproval_decision_drift",
            Self::ReapprovalDisclosureIncomplete => "reapproval_disclosure_incomplete",
            Self::EventFieldMismatch => "event_field_mismatch",
            Self::EventGateDrift => "event_gate_drift",
            Self::EventGapDrift => "event_gap_drift",
            Self::DuplicateAction => "duplicate_action",
            Self::ActionNotInspected => "action_not_inspected",
            Self::ExportPreviewDrift => "export_preview_drift",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::SummaryDrift => "summary_drift",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of the
/// upstream descriptor / governance lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized value for forbidden material. Returns true when a key (case-insensitive)
/// contains a forbidden substring.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_material(child)
        }),
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        _ => false,
    }
}
