//! Canonical M5 build-intelligence, host-boundary, and managed-workspace
//! execution-truth matrix with a non-inheriting claim gate that auto-narrows any
//! underqualified execution surface.
//!
//! Each [`SurfaceGovernanceRow`] names a marketed M5 execution surface and answers,
//! for that surface, how its target was discovered ([`TargetDiscoveryClass`]), how
//! confident the build/runtime adapter is ([`AdapterConfidence`]), where the work
//! runs ([`HostBoundary`]), which plane still owns mutable truth
//! ([`ControlPlaneOwnership`]), where its managed workspace sits in its lifecycle
//! ([`ManagedWorkspaceLifecycle`]), what it does to live resources
//! ([`MutationClass`]), whether preview/approval is satisfied ([`ApprovalState`]),
//! how fresh the qualifying evidence is ([`EvidenceFreshness`]), and what recovery
//! path applies ([`RollbackPosture`]). The row also records the live-resource
//! context — [`PersistenceClass`] and [`ExpiryClass`] — and then publishes an
//! [`ExecutionClaim`] no input can exceed.
//!
//! The [`ExecutionClaim`] a surface may publish is the weakest ceiling implied by
//! its observed states, so an undiscovered target, an unverified adapter, an unbound
//! host, an unknown control-plane owner, an unavailable workspace, a destructive
//! mutation, a bypassed approval, stale evidence, or an incomplete rollback all
//! narrow or withhold the claim automatically. The [`ClaimDecision`] records the
//! result and the recomputed [`NarrowingReason`]s explain it; all three are
//! validated against the gate so no surface can assert or hide a narrowing by hand.
//!
//! The packet is checked in at
//! `artifacts/execution/m5/m5-build-and-host-governance.json` and embedded here. It
//! is metadata-only: every field is a typed state or an opaque ref, and it carries
//! no credential bodies, raw provider payloads, host tokens, or control-plane
//! secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 build-and-host governance matrix schema version.
pub const M5_BUILD_AND_HOST_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_BUILD_AND_HOST_GOVERNANCE_RECORD_KIND: &str = "m5_build_and_host_governance_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_BUILD_AND_HOST_GOVERNANCE_PATH: &str =
    "artifacts/execution/m5/m5-build-and-host-governance.json";

/// Embedded checked-in packet JSON.
pub const M5_BUILD_AND_HOST_GOVERNANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/execution/m5/m5-build-and-host-governance.json"
));

/// A marketed M5 execution surface the matrix makes claims about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionSurface {
    /// Local build target on the developer machine.
    LocalBuildTarget,
    /// Framework-pack build intelligence target.
    FrameworkPackBuild,
    /// Remote preview session.
    RemotePreviewSession,
    /// Managed-workspace runtime.
    ManagedWorkspaceRuntime,
    /// Connector/control-plane backed service.
    ConnectorBackedService,
    /// Cluster-context execution target.
    ClusterContextExec,
    /// Live-resource (infrastructure) target.
    LiveResourceTarget,
    /// Incident-replay / ops-adjacent target.
    IncidentReplayTarget,
}

impl ExecutionSurface {
    /// Every execution surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LocalBuildTarget,
        Self::FrameworkPackBuild,
        Self::RemotePreviewSession,
        Self::ManagedWorkspaceRuntime,
        Self::ConnectorBackedService,
        Self::ClusterContextExec,
        Self::LiveResourceTarget,
        Self::IncidentReplayTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalBuildTarget => "local_build_target",
            Self::FrameworkPackBuild => "framework_pack_build",
            Self::RemotePreviewSession => "remote_preview_session",
            Self::ManagedWorkspaceRuntime => "managed_workspace_runtime",
            Self::ConnectorBackedService => "connector_backed_service",
            Self::ClusterContextExec => "cluster_context_exec",
            Self::LiveResourceTarget => "live_resource_target",
            Self::IncidentReplayTarget => "incident_replay_target",
        }
    }
}

/// Strength of a surface's published execution-truth claim.
///
/// Ordered low-to-high by [`ExecutionClaim::rank`]: an [`ExecutionClaim::Withheld`]
/// surface carries no claim, and an [`ExecutionClaim::Authoritative`] surface
/// carries a full, current, evidence-backed "this is the target, it runs here"
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionClaim {
    /// Full, current, provenance-backed execution claim.
    Authoritative,
    /// Disclosed-caveat claim; published as a qualified target.
    Qualified,
    /// Advisory-only claim; published as provisional.
    Provisional,
    /// No claim; the gate withholds it.
    Withheld,
}

impl ExecutionClaim {
    /// Every execution claim, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Authoritative,
        Self::Qualified,
        Self::Provisional,
        Self::Withheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Qualified => "qualified",
            Self::Provisional => "provisional",
            Self::Withheld => "withheld",
        }
    }

    /// Monotonic rank; higher means a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Withheld => 0,
            Self::Provisional => 1,
            Self::Qualified => 2,
            Self::Authoritative => 3,
        }
    }

    /// The weaker (lower-rank) of two claims.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// How the execution target was discovered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetDiscoveryClass {
    /// Discovered from a declared project manifest.
    DeclaredManifest,
    /// Discovered by probing the workspace.
    WorkspaceProbe,
    /// Inferred by a build/runtime adapter heuristic.
    AdapterInferred,
    /// Listed by an external control plane.
    ControlPlaneListed,
    /// Supplied directly by the user.
    UserSupplied,
    /// Not discovered; identity unknown.
    Undiscovered,
}

impl TargetDiscoveryClass {
    /// Every target-discovery class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DeclaredManifest,
        Self::WorkspaceProbe,
        Self::AdapterInferred,
        Self::ControlPlaneListed,
        Self::UserSupplied,
        Self::Undiscovered,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredManifest => "declared_manifest",
            Self::WorkspaceProbe => "workspace_probe",
            Self::AdapterInferred => "adapter_inferred",
            Self::ControlPlaneListed => "control_plane_listed",
            Self::UserSupplied => "user_supplied",
            Self::Undiscovered => "undiscovered",
        }
    }

    /// Highest claim this discovery class permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::DeclaredManifest | Self::WorkspaceProbe => ExecutionClaim::Authoritative,
            Self::AdapterInferred | Self::ControlPlaneListed => ExecutionClaim::Qualified,
            Self::UserSupplied => ExecutionClaim::Provisional,
            Self::Undiscovered => ExecutionClaim::Withheld,
        }
    }

    /// Whether this class raises the [`NarrowingReason::TargetUndiscovered`] trigger.
    pub const fn is_undiscovered_trigger(self) -> bool {
        matches!(self, Self::Undiscovered)
    }
}

/// Confidence the build/runtime adapter has in the resolved target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterConfidence {
    /// Verified against the live target.
    Verified,
    /// High confidence from a strong signal.
    High,
    /// Heuristic match only; depth narrows to qualified.
    Heuristic,
    /// Unverified; depth narrows to provisional.
    Unverified,
}

impl AdapterConfidence {
    /// Every adapter-confidence level, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::High,
        Self::Heuristic,
        Self::Unverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::High => "high",
            Self::Heuristic => "heuristic",
            Self::Unverified => "unverified",
        }
    }

    /// Highest claim this confidence level permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::Verified | Self::High => ExecutionClaim::Authoritative,
            Self::Heuristic => ExecutionClaim::Qualified,
            Self::Unverified => ExecutionClaim::Provisional,
        }
    }

    /// Whether this level raises the [`NarrowingReason::AdapterConfidenceLow`]
    /// trigger.
    pub const fn is_low_confidence_trigger(self) -> bool {
        matches!(self, Self::Unverified)
    }
}

/// Where a surface's work actually runs (its execution origin).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundary {
    /// Runs on the local machine.
    LocalHost,
    /// Runs in a managed cloud workspace.
    ManagedWorkspace,
    /// Runs on a remote attached host; depth narrows to qualified.
    RemoteAttached,
    /// Runs in a cluster context; depth narrows to qualified.
    ClusterContext,
    /// Runs through a bridge; depth narrows to provisional.
    BridgedHost,
    /// No host is established; carries no claim.
    UnboundHost,
}

impl HostBoundary {
    /// Every host boundary, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalHost,
        Self::ManagedWorkspace,
        Self::RemoteAttached,
        Self::ClusterContext,
        Self::BridgedHost,
        Self::UnboundHost,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::ManagedWorkspace => "managed_workspace",
            Self::RemoteAttached => "remote_attached",
            Self::ClusterContext => "cluster_context",
            Self::BridgedHost => "bridged_host",
            Self::UnboundHost => "unbound_host",
        }
    }

    /// Highest claim this host boundary permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::LocalHost | Self::ManagedWorkspace => ExecutionClaim::Authoritative,
            Self::RemoteAttached | Self::ClusterContext => ExecutionClaim::Qualified,
            Self::BridgedHost => ExecutionClaim::Provisional,
            Self::UnboundHost => ExecutionClaim::Withheld,
        }
    }

    /// Whether this boundary raises the [`NarrowingReason::HostUnbound`] trigger.
    pub const fn is_unbound_trigger(self) -> bool {
        matches!(self, Self::UnboundHost)
    }
}

/// Which plane still owns the surface's mutable truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlPlaneOwnership {
    /// The product owns mutable truth.
    ProductOwned,
    /// Ownership is shared; depth narrows to qualified.
    CoOwned,
    /// An external plane owns mutable truth; depth narrows to qualified.
    ExternalOwned,
    /// The owning plane is unknown; carries no claim.
    UnknownOwner,
}

impl ControlPlaneOwnership {
    /// Every control-plane ownership class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProductOwned,
        Self::CoOwned,
        Self::ExternalOwned,
        Self::UnknownOwner,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductOwned => "product_owned",
            Self::CoOwned => "co_owned",
            Self::ExternalOwned => "external_owned",
            Self::UnknownOwner => "unknown_owner",
        }
    }

    /// Highest claim this ownership class permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::ProductOwned => ExecutionClaim::Authoritative,
            Self::CoOwned | Self::ExternalOwned => ExecutionClaim::Qualified,
            Self::UnknownOwner => ExecutionClaim::Withheld,
        }
    }

    /// Whether this class raises the [`NarrowingReason::ControlPlaneUnknown`]
    /// trigger.
    pub const fn is_unknown_owner_trigger(self) -> bool {
        matches!(self, Self::UnknownOwner)
    }

    /// Whether an external plane still owns the surface's mutable truth.
    pub const fn is_external(self) -> bool {
        matches!(self, Self::ExternalOwned)
    }
}

/// Lifecycle state of a surface's managed workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedWorkspaceLifecycle {
    /// Active and serving.
    Active,
    /// Provisioning; depth narrows to qualified.
    Provisioning,
    /// Suspended; depth narrows to provisional.
    Suspended,
    /// Draining; depth narrows to provisional.
    Draining,
    /// Terminated; carries no claim.
    Terminated,
    /// No managed workspace applies (for example, a local host).
    NotApplicable,
}

impl ManagedWorkspaceLifecycle {
    /// Every managed-workspace lifecycle state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Active,
        Self::Provisioning,
        Self::Suspended,
        Self::Draining,
        Self::Terminated,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Provisioning => "provisioning",
            Self::Suspended => "suspended",
            Self::Draining => "draining",
            Self::Terminated => "terminated",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest claim this lifecycle state permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::Active | Self::NotApplicable => ExecutionClaim::Authoritative,
            Self::Provisioning => ExecutionClaim::Qualified,
            Self::Suspended | Self::Draining => ExecutionClaim::Provisional,
            Self::Terminated => ExecutionClaim::Withheld,
        }
    }

    /// Whether this state raises the [`NarrowingReason::WorkspaceUnavailable`]
    /// trigger.
    pub const fn is_unavailable_trigger(self) -> bool {
        matches!(self, Self::Terminated)
    }
}

/// What the surface does to live resources (its preview/runtime mutation class).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationClass {
    /// Reads only; mutates nothing.
    ReadOnly,
    /// Produces a preview without applying it.
    PreviewOnly,
    /// Applies a reversible change; depth narrows to qualified.
    ReversibleApply,
    /// Applies an irreversible change; depth narrows to provisional.
    IrreversibleApply,
    /// Applies a destructive change; carries no claim until previewed/approved.
    DestructiveApply,
}

impl MutationClass {
    /// Every mutation class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReadOnly,
        Self::PreviewOnly,
        Self::ReversibleApply,
        Self::IrreversibleApply,
        Self::DestructiveApply,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PreviewOnly => "preview_only",
            Self::ReversibleApply => "reversible_apply",
            Self::IrreversibleApply => "irreversible_apply",
            Self::DestructiveApply => "destructive_apply",
        }
    }

    /// Highest claim this mutation class permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::ReadOnly | Self::PreviewOnly => ExecutionClaim::Authoritative,
            Self::ReversibleApply => ExecutionClaim::Qualified,
            Self::IrreversibleApply => ExecutionClaim::Provisional,
            Self::DestructiveApply => ExecutionClaim::Withheld,
        }
    }

    /// Whether this class raises the [`NarrowingReason::UnsafeMutation`] trigger.
    pub const fn is_unsafe_trigger(self) -> bool {
        matches!(self, Self::DestructiveApply)
    }
}

/// Whether the surface's preview/approval handoff is satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalState {
    /// No approval is required.
    NotRequired,
    /// Approval was granted.
    Approved,
    /// Preview is pending; depth narrows to qualified.
    PreviewPending,
    /// Approval is required but not yet granted; depth narrows to provisional.
    ApprovalRequiredUnmet,
    /// Approval was bypassed; carries no claim.
    Bypassed,
}

impl ApprovalState {
    /// Every approval state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotRequired,
        Self::Approved,
        Self::PreviewPending,
        Self::ApprovalRequiredUnmet,
        Self::Bypassed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Approved => "approved",
            Self::PreviewPending => "preview_pending",
            Self::ApprovalRequiredUnmet => "approval_required_unmet",
            Self::Bypassed => "bypassed",
        }
    }

    /// Highest claim this approval state permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::NotRequired | Self::Approved => ExecutionClaim::Authoritative,
            Self::PreviewPending => ExecutionClaim::Qualified,
            Self::ApprovalRequiredUnmet => ExecutionClaim::Provisional,
            Self::Bypassed => ExecutionClaim::Withheld,
        }
    }

    /// Whether this state raises the [`NarrowingReason::ApprovalBypassed`] trigger.
    pub const fn is_bypassed_trigger(self) -> bool {
        matches!(self, Self::Bypassed)
    }
}

/// Freshness of a surface's qualifying evidence relative to its freshness SLO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Current within its freshness SLO.
    Current,
    /// Present but past its freshness SLO; depth narrows to qualified.
    Stale,
    /// Expired; depth narrows to provisional.
    Expired,
    /// Freshness cannot be established; depth narrows to qualified.
    Unknown,
}

impl EvidenceFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Expired, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the evidence is current within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Highest claim this freshness alone permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::Current => ExecutionClaim::Authoritative,
            Self::Stale | Self::Unknown => ExecutionClaim::Qualified,
            Self::Expired => ExecutionClaim::Provisional,
        }
    }

    /// Whether this freshness raises the [`NarrowingReason::EvidenceStale`] trigger.
    ///
    /// Stale and expired evidence both raise the trigger; `unknown` lowers the
    /// ceiling but is treated as a soft state, not a headline trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// Rollback posture for a surface's mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPosture {
    /// Exact reversal verified.
    ReversibleVerified,
    /// Reversal path declared but not verified; depth narrows to qualified.
    ReversibleUnverified,
    /// Only a compensating reversal is available; depth narrows to qualified.
    CompensatingOnly,
    /// No reversal is possible; depth narrows to provisional.
    Irreversible,
    /// No rollback applies.
    NotApplicable,
}

impl RollbackPosture {
    /// Every rollback posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReversibleVerified,
        Self::ReversibleUnverified,
        Self::CompensatingOnly,
        Self::Irreversible,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleVerified => "reversible_verified",
            Self::ReversibleUnverified => "reversible_unverified",
            Self::CompensatingOnly => "compensating_only",
            Self::Irreversible => "irreversible",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest claim this rollback posture permits a surface to publish.
    pub const fn claim_ceiling(self) -> ExecutionClaim {
        match self {
            Self::ReversibleVerified | Self::NotApplicable => ExecutionClaim::Authoritative,
            Self::ReversibleUnverified | Self::CompensatingOnly => ExecutionClaim::Qualified,
            Self::Irreversible => ExecutionClaim::Provisional,
        }
    }

    /// Whether this posture raises the [`NarrowingReason::RollbackIncomplete`]
    /// trigger.
    ///
    /// An unverified reversal and an irreversible mutation both raise the trigger; a
    /// compensating-only reversal lowers the ceiling but is a disclosed, accepted
    /// posture rather than a headline trigger.
    pub const fn is_incomplete_trigger(self) -> bool {
        matches!(self, Self::ReversibleUnverified | Self::Irreversible)
    }
}

/// Persistence class of a surface's live-resource context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceClass {
    /// Durable beyond the session.
    Durable,
    /// Scoped to the current session.
    SessionScoped,
    /// Ephemeral; discarded on teardown.
    Ephemeral,
    /// Persistence cannot be established.
    Unknown,
}

impl PersistenceClass {
    /// Every persistence class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Durable,
        Self::SessionScoped,
        Self::Ephemeral,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Durable => "durable",
            Self::SessionScoped => "session_scoped",
            Self::Ephemeral => "ephemeral",
            Self::Unknown => "unknown",
        }
    }
}

/// Expiry class of a surface's live-resource context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryClass {
    /// No expiry applies.
    NoExpiry,
    /// A future expiry is scheduled.
    ScheduledExpiry,
    /// Already expired.
    Expired,
    /// Expiry cannot be established.
    Unknown,
}

impl ExpiryClass {
    /// Every expiry class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoExpiry,
        Self::ScheduledExpiry,
        Self::Expired,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoExpiry => "no_expiry",
            Self::ScheduledExpiry => "scheduled_expiry",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }
}

/// A headline reason the governance gate narrows a surface.
///
/// These are the canonical execution-truth release-control triggers: an
/// undiscovered target, low adapter confidence, an unbound host, an unknown
/// control-plane owner, an unavailable workspace, a destructive mutation, a bypassed
/// approval, stale evidence, and an incomplete rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The target could not be discovered.
    TargetUndiscovered,
    /// The adapter confidence is unverified.
    AdapterConfidenceLow,
    /// No host is established for the surface.
    HostUnbound,
    /// The control-plane owner is unknown.
    ControlPlaneUnknown,
    /// The managed workspace is unavailable.
    WorkspaceUnavailable,
    /// The mutation is destructive.
    UnsafeMutation,
    /// Preview/approval was bypassed.
    ApprovalBypassed,
    /// The qualifying evidence is stale or expired.
    EvidenceStale,
    /// The rollback path is unverified or irreversible.
    RollbackIncomplete,
}

impl NarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TargetUndiscovered,
        Self::AdapterConfidenceLow,
        Self::HostUnbound,
        Self::ControlPlaneUnknown,
        Self::WorkspaceUnavailable,
        Self::UnsafeMutation,
        Self::ApprovalBypassed,
        Self::EvidenceStale,
        Self::RollbackIncomplete,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetUndiscovered => "target_undiscovered",
            Self::AdapterConfidenceLow => "adapter_confidence_low",
            Self::HostUnbound => "host_unbound",
            Self::ControlPlaneUnknown => "control_plane_unknown",
            Self::WorkspaceUnavailable => "workspace_unavailable",
            Self::UnsafeMutation => "unsafe_mutation",
            Self::ApprovalBypassed => "approval_bypassed",
            Self::EvidenceStale => "evidence_stale",
            Self::RollbackIncomplete => "rollback_incomplete",
        }
    }
}

/// The action the governance gate takes on a surface relative to an authoritative
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimDecision {
    /// No narrowing; the surface publishes an authoritative claim.
    Publish,
    /// Narrow the published claim to qualified.
    NarrowToQualified,
    /// Narrow the published claim to provisional.
    NarrowToProvisional,
    /// Withhold the claim entirely.
    Withhold,
}

impl ClaimDecision {
    /// Every claim decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Publish,
        Self::NarrowToQualified,
        Self::NarrowToProvisional,
        Self::Withhold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::NarrowToQualified => "narrow_to_qualified",
            Self::NarrowToProvisional => "narrow_to_provisional",
            Self::Withhold => "withhold",
        }
    }

    /// The decision implied by a published execution claim.
    pub const fn for_published(claim: ExecutionClaim) -> Self {
        match claim {
            ExecutionClaim::Authoritative => Self::Publish,
            ExecutionClaim::Qualified => Self::NarrowToQualified,
            ExecutionClaim::Provisional => Self::NarrowToProvisional,
            ExecutionClaim::Withheld => Self::Withhold,
        }
    }

    /// Whether the gate narrowed or withheld the surface.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Publish)
    }
}

/// One governance row for a marketed M5 execution surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceGovernanceRow {
    /// Stable surface-governance id.
    pub surface_id: String,
    /// Marketed M5 execution surface this row governs.
    pub execution_surface: ExecutionSurface,
    /// How the target was discovered.
    pub target_discovery_class: TargetDiscoveryClass,
    /// Adapter confidence in the resolved target.
    pub adapter_confidence: AdapterConfidence,
    /// Where the work runs (execution origin).
    pub host_boundary: HostBoundary,
    /// Which plane still owns mutable truth.
    pub control_plane_ownership: ControlPlaneOwnership,
    /// Managed-workspace lifecycle state.
    pub managed_workspace_lifecycle: ManagedWorkspaceLifecycle,
    /// Preview/runtime mutation class.
    pub mutation_class: MutationClass,
    /// Preview/approval handoff state.
    pub approval_state: ApprovalState,
    /// Freshness of the surface's qualifying evidence.
    pub evidence_freshness: EvidenceFreshness,
    /// Rollback posture for the surface's mutation.
    pub rollback_posture: RollbackPosture,
    /// Persistence class of the live-resource context.
    pub persistence_class: PersistenceClass,
    /// Expiry class of the live-resource context.
    pub expiry_class: ExpiryClass,
    /// Claim the surface's own evidence asserts, before the gate.
    pub declared_claim: ExecutionClaim,
    /// Claim actually published after the gate narrows the surface.
    ///
    /// Must equal [`SurfaceGovernanceRow::effective_claim`].
    pub published_claim: ExecutionClaim,
    /// Decision the gate takes; must equal the recomputed decision.
    pub claim_decision: ClaimDecision,
    /// Headline narrowing reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Ref to the surface's target-identity record.
    pub target_identity_ref: String,
    /// Ref to the surface's host-boundary record.
    pub host_boundary_ref: String,
    /// Ref to the surface's control-plane ownership record.
    pub control_plane_ref: String,
    /// Ref to the surface's mutation/preview story.
    pub mutation_preview_ref: String,
    /// Ref to the surface's durable rollback path.
    pub rollback_ref: String,
    /// Ref binding this row into desktop, CLI, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl SurfaceGovernanceRow {
    /// The claim the surface's own evidence asserted, before environmental
    /// narrowing.
    pub fn capability_floor(&self) -> ExecutionClaim {
        self.declared_claim
    }

    /// The claim the gate permits this surface to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the target
    /// discovery class, adapter confidence, host boundary, control-plane ownership,
    /// managed-workspace lifecycle, mutation class, approval state, evidence
    /// freshness, and rollback posture, so an undiscovered target, an unverified
    /// adapter, an unbound host, an unknown owner, an unavailable workspace, a
    /// destructive mutation, a bypassed approval, stale evidence, or an incomplete
    /// rollback can never publish an authoritative claim.
    pub fn effective_claim(&self) -> ExecutionClaim {
        self.capability_floor()
            .min(self.target_discovery_class.claim_ceiling())
            .min(self.adapter_confidence.claim_ceiling())
            .min(self.host_boundary.claim_ceiling())
            .min(self.control_plane_ownership.claim_ceiling())
            .min(self.managed_workspace_lifecycle.claim_ceiling())
            .min(self.mutation_class.claim_ceiling())
            .min(self.approval_state.claim_ceiling())
            .min(self.evidence_freshness.claim_ceiling())
            .min(self.rollback_posture.claim_ceiling())
    }

    /// The headline narrowing reasons recomputed from the surface's observed states.
    pub fn computed_narrowing_reasons(&self) -> Vec<NarrowingReason> {
        let mut reasons = Vec::new();
        if self.target_discovery_class.is_undiscovered_trigger() {
            reasons.push(NarrowingReason::TargetUndiscovered);
        }
        if self.adapter_confidence.is_low_confidence_trigger() {
            reasons.push(NarrowingReason::AdapterConfidenceLow);
        }
        if self.host_boundary.is_unbound_trigger() {
            reasons.push(NarrowingReason::HostUnbound);
        }
        if self.control_plane_ownership.is_unknown_owner_trigger() {
            reasons.push(NarrowingReason::ControlPlaneUnknown);
        }
        if self.managed_workspace_lifecycle.is_unavailable_trigger() {
            reasons.push(NarrowingReason::WorkspaceUnavailable);
        }
        if self.mutation_class.is_unsafe_trigger() {
            reasons.push(NarrowingReason::UnsafeMutation);
        }
        if self.approval_state.is_bypassed_trigger() {
            reasons.push(NarrowingReason::ApprovalBypassed);
        }
        if self.evidence_freshness.is_stale_trigger() {
            reasons.push(NarrowingReason::EvidenceStale);
        }
        if self.rollback_posture.is_incomplete_trigger() {
            reasons.push(NarrowingReason::RollbackIncomplete);
        }
        reasons
    }

    /// The decision the gate must record for this surface.
    pub fn required_decision(&self) -> ClaimDecision {
        ClaimDecision::for_published(self.effective_claim())
    }

    /// Whether the surface may publish an authoritative claim.
    pub fn is_publishable(&self) -> bool {
        self.effective_claim() == ExecutionClaim::Authoritative
    }

    /// Whether the surface carries its own non-empty identity, host, control-plane,
    /// mutation/preview, rollback, and support-export refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.target_identity_ref.trim().is_empty()
            && !self.host_boundary_ref.trim().is_empty()
            && !self.control_plane_ref.trim().is_empty()
            && !self.mutation_preview_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published claim, decision, and narrowing reasons all agree
    /// with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_claim == self.effective_claim()
            && self.claim_decision == self.required_decision()
            && self.narrowing_reasons == self.computed_narrowing_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BuildAndHostGovernanceSummary {
    /// Total surface rows.
    pub total_surfaces: usize,
    /// Number of marketed surfaces claimed.
    pub surface_count: usize,
    /// Surfaces published with an authoritative claim.
    pub authoritative_surfaces: usize,
    /// Surfaces published with a qualified claim.
    pub qualified_surfaces: usize,
    /// Surfaces published with a provisional claim.
    pub provisional_surfaces: usize,
    /// Surfaces whose claim is withheld.
    pub withheld_surfaces: usize,
    /// Surfaces that may publish an authoritative claim.
    pub publishable_surfaces: usize,
    /// Surfaces the gate narrowed or withheld in any way.
    pub narrowed_surfaces: usize,
    /// Surfaces the gate withheld entirely.
    pub withheld_decision_surfaces: usize,
    /// Surfaces with current evidence freshness.
    pub current_evidence_surfaces: usize,
    /// Surfaces with unverified adapter confidence.
    pub low_confidence_surfaces: usize,
    /// Surfaces whose host is unbound.
    pub unbound_host_surfaces: usize,
    /// Surfaces whose mutable truth is externally owned.
    pub external_control_plane_surfaces: usize,
    /// Surfaces carrying at least one narrowing reason.
    pub surfaces_with_narrowing_reasons: usize,
}

/// A redaction-safe export row projected from a surface governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildAndHostGovernanceExportRow {
    /// Surface-governance id.
    pub surface_id: String,
    /// Execution-surface token.
    pub execution_surface: String,
    /// Target-discovery-class token.
    pub target_discovery_class: String,
    /// Adapter-confidence token.
    pub adapter_confidence: String,
    /// Host-boundary token.
    pub host_boundary: String,
    /// Control-plane-ownership token.
    pub control_plane_ownership: String,
    /// Managed-workspace-lifecycle token.
    pub managed_workspace_lifecycle: String,
    /// Mutation-class token.
    pub mutation_class: String,
    /// Approval-state token.
    pub approval_state: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Rollback-posture token.
    pub rollback_posture: String,
    /// Persistence-class token.
    pub persistence_class: String,
    /// Expiry-class token.
    pub expiry_class: String,
    /// Declared-claim token.
    pub declared_claim: String,
    /// Published-claim token.
    pub published_claim: String,
    /// Claim-decision token.
    pub claim_decision: String,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Target-identity ref.
    pub target_identity_ref: String,
    /// Whether the surface publishes an authoritative claim.
    pub publication_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BuildAndHostGovernanceExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub surfaces: Vec<M5BuildAndHostGovernanceExportRow>,
    /// Whether every surface's published claim and decision agree with the gate.
    pub all_surfaces_gate_consistent: bool,
    /// Surfaces that may publish an authoritative claim.
    pub publishable_count: usize,
    /// Surfaces the gate narrowed or withheld.
    pub narrowed_count: usize,
    /// Surfaces the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 build-and-host governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BuildAndHostGovernanceMatrix {
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
    /// Marketed surfaces the packet claims; one row per surface.
    pub execution_surfaces: Vec<ExecutionSurface>,
    /// Closed execution-claim vocabulary.
    pub execution_claims: Vec<ExecutionClaim>,
    /// Closed target-discovery-class vocabulary.
    pub target_discovery_classes: Vec<TargetDiscoveryClass>,
    /// Closed adapter-confidence vocabulary.
    pub adapter_confidences: Vec<AdapterConfidence>,
    /// Closed host-boundary vocabulary.
    pub host_boundaries: Vec<HostBoundary>,
    /// Closed control-plane-ownership vocabulary.
    pub control_plane_ownerships: Vec<ControlPlaneOwnership>,
    /// Closed managed-workspace-lifecycle vocabulary.
    pub managed_workspace_lifecycles: Vec<ManagedWorkspaceLifecycle>,
    /// Closed mutation-class vocabulary.
    pub mutation_classes: Vec<MutationClass>,
    /// Closed approval-state vocabulary.
    pub approval_states: Vec<ApprovalState>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed rollback-posture vocabulary.
    pub rollback_postures: Vec<RollbackPosture>,
    /// Closed persistence-class vocabulary.
    pub persistence_classes: Vec<PersistenceClass>,
    /// Closed expiry-class vocabulary.
    pub expiry_classes: Vec<ExpiryClass>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed claim-decision vocabulary.
    pub claim_decisions: Vec<ClaimDecision>,
    /// Governance rows, one per marketed surface.
    #[serde(default)]
    pub surfaces: Vec<SurfaceGovernanceRow>,
    /// Summary counts.
    pub summary: M5BuildAndHostGovernanceSummary,
}

impl M5BuildAndHostGovernanceMatrix {
    /// Returns the row for a marketed surface.
    pub fn surface(&self, surface: ExecutionSurface) -> Option<&SurfaceGovernanceRow> {
        self.surfaces
            .iter()
            .find(|s| s.execution_surface == surface)
    }

    /// Surfaces that may publish an authoritative claim.
    pub fn publishable_surfaces(&self) -> impl Iterator<Item = &SurfaceGovernanceRow> {
        self.surfaces.iter().filter(|s| s.is_publishable())
    }

    /// Surfaces the gate narrowed or withheld in any way.
    pub fn narrowed_surfaces(&self) -> impl Iterator<Item = &SurfaceGovernanceRow> {
        self.surfaces
            .iter()
            .filter(|s| s.required_decision().is_narrowed())
    }

    /// Surfaces the gate withheld entirely.
    pub fn withheld_surfaces(&self) -> impl Iterator<Item = &SurfaceGovernanceRow> {
        self.surfaces
            .iter()
            .filter(|s| s.required_decision() == ClaimDecision::Withhold)
    }

    /// Whether every surface's stored published claim, decision, and reasons agree
    /// with the recomputed gate decision.
    pub fn all_surfaces_gate_consistent(&self) -> bool {
        self.surfaces.iter().all(|s| s.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5BuildAndHostGovernanceSummary {
        let count_published = |claim: ExecutionClaim| {
            self.surfaces
                .iter()
                .filter(|s| s.published_claim == claim)
                .count()
        };
        M5BuildAndHostGovernanceSummary {
            total_surfaces: self.surfaces.len(),
            surface_count: self.execution_surfaces.len(),
            authoritative_surfaces: count_published(ExecutionClaim::Authoritative),
            qualified_surfaces: count_published(ExecutionClaim::Qualified),
            provisional_surfaces: count_published(ExecutionClaim::Provisional),
            withheld_surfaces: count_published(ExecutionClaim::Withheld),
            publishable_surfaces: self.publishable_surfaces().count(),
            narrowed_surfaces: self.narrowed_surfaces().count(),
            withheld_decision_surfaces: self.withheld_surfaces().count(),
            current_evidence_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.evidence_freshness.is_current())
                .count(),
            low_confidence_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.adapter_confidence.is_low_confidence_trigger())
                .count(),
            unbound_host_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.host_boundary.is_unbound_trigger())
                .count(),
            external_control_plane_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.control_plane_ownership.is_external())
                .count(),
            surfaces_with_narrowing_reasons: self
                .surfaces
                .iter()
                .filter(|s| !s.narrowing_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — desktop and CLI
    /// target pickers, service-health and Help/About, support exports, and
    /// release/public-truth packets — render instead of restating M5 execution,
    /// host, and managed-workspace status text by hand.
    pub fn export_projection(&self) -> M5BuildAndHostGovernanceExportProjection {
        let surfaces = self
            .surfaces
            .iter()
            .map(|s| M5BuildAndHostGovernanceExportRow {
                surface_id: s.surface_id.clone(),
                execution_surface: s.execution_surface.as_str().to_owned(),
                target_discovery_class: s.target_discovery_class.as_str().to_owned(),
                adapter_confidence: s.adapter_confidence.as_str().to_owned(),
                host_boundary: s.host_boundary.as_str().to_owned(),
                control_plane_ownership: s.control_plane_ownership.as_str().to_owned(),
                managed_workspace_lifecycle: s.managed_workspace_lifecycle.as_str().to_owned(),
                mutation_class: s.mutation_class.as_str().to_owned(),
                approval_state: s.approval_state.as_str().to_owned(),
                evidence_freshness: s.evidence_freshness.as_str().to_owned(),
                rollback_posture: s.rollback_posture.as_str().to_owned(),
                persistence_class: s.persistence_class.as_str().to_owned(),
                expiry_class: s.expiry_class.as_str().to_owned(),
                declared_claim: s.declared_claim.as_str().to_owned(),
                published_claim: s.published_claim.as_str().to_owned(),
                claim_decision: s.claim_decision.as_str().to_owned(),
                narrowing_reasons: s
                    .narrowing_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                target_identity_ref: s.target_identity_ref.clone(),
                publication_ready: s.is_publishable(),
                summary: format!(
                    "{}: discovery {}, confidence {}, host {}, control plane {}, workspace {}, mutation {}, approval {}, declared {}, published {} ({}), rollback {}",
                    s.execution_surface.as_str(),
                    s.target_discovery_class.as_str(),
                    s.adapter_confidence.as_str(),
                    s.host_boundary.as_str(),
                    s.control_plane_ownership.as_str(),
                    s.managed_workspace_lifecycle.as_str(),
                    s.mutation_class.as_str(),
                    s.approval_state.as_str(),
                    s.declared_claim.as_str(),
                    s.published_claim.as_str(),
                    s.claim_decision.as_str(),
                    s.rollback_posture.as_str()
                ),
            })
            .collect();
        M5BuildAndHostGovernanceExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            surfaces,
            all_surfaces_gate_consistent: self.all_surfaces_gate_consistent(),
            publishable_count: self.publishable_surfaces().count(),
            narrowed_count: self.narrowed_surfaces().count(),
            withheld_count: self.withheld_surfaces().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5BuildAndHostGovernanceViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ExecutionSurface> = self.execution_surfaces.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for row in &self.surfaces {
            if !seen_ids.insert(row.surface_id.clone()) {
                violations.push(M5BuildAndHostGovernanceViolation::DuplicateSurfaceId {
                    surface_id: row.surface_id.clone(),
                });
            }
            if !seen_surfaces.insert(row.execution_surface) {
                violations.push(M5BuildAndHostGovernanceViolation::DuplicateSurfaceRow {
                    surface: row.execution_surface.as_str(),
                });
            }
            if !claimed.contains(&row.execution_surface) {
                violations.push(M5BuildAndHostGovernanceViolation::UnclaimedSurfaceRow {
                    surface_id: row.surface_id.clone(),
                    surface: row.execution_surface.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed surface must carry its own row, so a surface never inherits
        // a claim from an adjacent authoritative one.
        for &surface in &self.execution_surfaces {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5BuildAndHostGovernanceViolation::MissingSurfaceRow {
                    surface: surface.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5BuildAndHostGovernanceViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5BuildAndHostGovernanceViolation>) {
        if self.schema_version != M5_BUILD_AND_HOST_GOVERNANCE_SCHEMA_VERSION {
            violations.push(
                M5BuildAndHostGovernanceViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_BUILD_AND_HOST_GOVERNANCE_RECORD_KIND {
            violations.push(M5BuildAndHostGovernanceViolation::UnsupportedRecordKind {
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
                violations.push(M5BuildAndHostGovernanceViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "execution_surfaces",
                self.execution_surfaces == ExecutionSurface::ALL.to_vec(),
            ),
            (
                "execution_claims",
                self.execution_claims == ExecutionClaim::ALL.to_vec(),
            ),
            (
                "target_discovery_classes",
                self.target_discovery_classes == TargetDiscoveryClass::ALL.to_vec(),
            ),
            (
                "adapter_confidences",
                self.adapter_confidences == AdapterConfidence::ALL.to_vec(),
            ),
            (
                "host_boundaries",
                self.host_boundaries == HostBoundary::ALL.to_vec(),
            ),
            (
                "control_plane_ownerships",
                self.control_plane_ownerships == ControlPlaneOwnership::ALL.to_vec(),
            ),
            (
                "managed_workspace_lifecycles",
                self.managed_workspace_lifecycles == ManagedWorkspaceLifecycle::ALL.to_vec(),
            ),
            (
                "mutation_classes",
                self.mutation_classes == MutationClass::ALL.to_vec(),
            ),
            (
                "approval_states",
                self.approval_states == ApprovalState::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "rollback_postures",
                self.rollback_postures == RollbackPosture::ALL.to_vec(),
            ),
            (
                "persistence_classes",
                self.persistence_classes == PersistenceClass::ALL.to_vec(),
            ),
            (
                "expiry_classes",
                self.expiry_classes == ExpiryClass::ALL.to_vec(),
            ),
            (
                "narrowing_reasons",
                self.narrowing_reasons == NarrowingReason::ALL.to_vec(),
            ),
            (
                "claim_decisions",
                self.claim_decisions == ClaimDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5BuildAndHostGovernanceViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &SurfaceGovernanceRow,
        violations: &mut Vec<M5BuildAndHostGovernanceViolation>,
    ) {
        for (field, value) in [
            ("surface_id", &row.surface_id),
            ("target_identity_ref", &row.target_identity_ref),
            ("host_boundary_ref", &row.host_boundary_ref),
            ("control_plane_ref", &row.control_plane_ref),
            ("mutation_preview_ref", &row.mutation_preview_ref),
            ("rollback_ref", &row.rollback_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5BuildAndHostGovernanceViolation::EmptyField {
                    id: row.surface_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrowing_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5BuildAndHostGovernanceViolation::DuplicateNarrowingReason {
                        surface_id: row.surface_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published claim must equal the gate's recomputed decision, so a surface
        // can never claim more certainty, host stability, control-plane ownership, or
        // recovery than its observed states support.
        let effective = row.effective_claim();
        if row.published_claim != effective {
            violations.push(
                M5BuildAndHostGovernanceViolation::OverstatedPublishedClaim {
                    surface_id: row.surface_id.clone(),
                    published: row.published_claim.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The recorded decision must match the published claim, so release tooling
        // proves underqualified surfaces narrow automatically.
        let required = row.required_decision();
        if row.claim_decision != required {
            violations.push(M5BuildAndHostGovernanceViolation::DecisionMismatch {
                surface_id: row.surface_id.clone(),
                declared: row.claim_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded narrowing reasons must equal the reasons recomputed from the
        // observed states, so a narrowing can never be asserted or hidden by hand.
        let computed = row.computed_narrowing_reasons();
        if row.narrowing_reasons != computed {
            violations.push(
                M5BuildAndHostGovernanceViolation::NarrowingReasonsMismatch {
                    surface_id: row.surface_id.clone(),
                },
            );
        }

        // A publishable surface must be genuinely clean: an authoritative-ceiling
        // discovery, confidence, host, control-plane, workspace, mutation, approval,
        // and rollback state, current evidence, an authoritative capability floor,
        // and no narrowing reason. This is the non-inheritance guardrail.
        if row.is_publishable()
            && (row.target_discovery_class.claim_ceiling() != ExecutionClaim::Authoritative
                || row.adapter_confidence.claim_ceiling() != ExecutionClaim::Authoritative
                || row.host_boundary.claim_ceiling() != ExecutionClaim::Authoritative
                || row.control_plane_ownership.claim_ceiling() != ExecutionClaim::Authoritative
                || row.managed_workspace_lifecycle.claim_ceiling() != ExecutionClaim::Authoritative
                || row.mutation_class.claim_ceiling() != ExecutionClaim::Authoritative
                || row.approval_state.claim_ceiling() != ExecutionClaim::Authoritative
                || row.rollback_posture.claim_ceiling() != ExecutionClaim::Authoritative
                || !row.evidence_freshness.is_current()
                || row.capability_floor() != ExecutionClaim::Authoritative
                || !row.narrowing_reasons.is_empty())
        {
            violations.push(
                M5BuildAndHostGovernanceViolation::PublishedSurfaceNotClean {
                    surface_id: row.surface_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the M5 build-and-host governance packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BuildAndHostGovernanceViolation {
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
    /// A surface-governance id appears more than once.
    DuplicateSurfaceId {
        /// Duplicate surface id.
        surface_id: String,
    },
    /// A marketed surface carries more than one row.
    DuplicateSurfaceRow {
        /// Surface token.
        surface: &'static str,
    },
    /// A claimed marketed surface has no row.
    MissingSurfaceRow {
        /// Surface token.
        surface: &'static str,
    },
    /// A row covers a surface the packet does not claim.
    UnclaimedSurfaceRow {
        /// Row id.
        surface_id: String,
        /// Surface token.
        surface: &'static str,
    },
    /// A row lists a narrowing reason more than once.
    DuplicateNarrowingReason {
        /// Row id.
        surface_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A surface publishes a claim beyond what its evidence supports.
    OverstatedPublishedClaim {
        /// Row id.
        surface_id: String,
        /// Published claim token.
        published: &'static str,
        /// Computed effective claim token.
        computed: &'static str,
    },
    /// A surface's decision disagrees with its published claim.
    DecisionMismatch {
        /// Row id.
        surface_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A surface's narrowing reasons disagree with the recomputed reasons.
    NarrowingReasonsMismatch {
        /// Row id.
        surface_id: String,
    },
    /// A publishable surface still carries a narrowing reason or a non-clean state.
    PublishedSurfaceNotClean {
        /// Row id.
        surface_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5BuildAndHostGovernanceViolation {
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
            Self::DuplicateSurfaceId { surface_id } => {
                write!(f, "duplicate surface id {surface_id}")
            }
            Self::DuplicateSurfaceRow { surface } => {
                write!(f, "duplicate row for surface {surface}")
            }
            Self::MissingSurfaceRow { surface } => {
                write!(f, "missing row for claimed surface {surface}")
            }
            Self::UnclaimedSurfaceRow {
                surface_id,
                surface,
            } => {
                write!(f, "row {surface_id} covers unclaimed surface {surface}")
            }
            Self::DuplicateNarrowingReason { surface_id, reason } => {
                write!(f, "row {surface_id} repeats narrowing reason {reason}")
            }
            Self::OverstatedPublishedClaim {
                surface_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {surface_id} publishes claim {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                surface_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {surface_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::NarrowingReasonsMismatch { surface_id } => {
                write!(
                    f,
                    "row {surface_id} narrowing reasons disagree with the gate"
                )
            }
            Self::PublishedSurfaceNotClean { surface_id } => {
                write!(
                    f,
                    "row {surface_id} is publishable but carries a narrowing reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5BuildAndHostGovernanceViolation {}

/// Loads the embedded M5 build-and-host governance matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5BuildAndHostGovernanceMatrix`].
pub fn current_m5_build_and_host_governance_matrix(
) -> Result<M5BuildAndHostGovernanceMatrix, serde_json::Error> {
    serde_json::from_str(M5_BUILD_AND_HOST_GOVERNANCE_JSON)
}

#[cfg(test)]
mod tests;
