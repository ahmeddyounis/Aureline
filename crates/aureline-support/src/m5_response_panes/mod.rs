//! M5 response panes: service-ownership / on-call strips, runbook-guided
//! response panes, and local-outage continuity views, bound to the frozen
//! operator-surface matrix.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the
//! *families* of operator surface — what a service-ownership strip, runbook-step
//! card, or failover notice is, the one shared state vocabulary, and the
//! invariants every surface must hold. The
//! [overview boards](crate::m5_operator_boards) built the first summary surfaces
//! on top of it. This lane builds the first real **response surfaces**: the strips
//! and panes an operator works *from* once an alert fires, plus the continuity
//! truth that keeps the product usable when a provider or managed boundary is
//! impaired.
//!
//! Three things have to stay honest from first alert to export:
//!
//! 1. **Service ownership and on-call authority stay visible.** Each
//!    [`ServiceOwnershipStrip`] names the service, its environment, its primary
//!    and backup owner, the active on-call lane, whether its source is
//!    *authoritative* or only *advisory*, an escalation action, and its
//!    last-checked freshness. A stale strip never shows a confirmed green dot —
//!    [`compute_effective_state`] downgrades it.
//! 2. **Runbook steps disclose their authority and preview path.** Each
//!    [`RunbookStep`] declares its [`StepIntentClass`] (observe, verify, mitigate,
//!    rollback, communicate), its [`ActionBoundaryClass`] (local, remote, managed,
//!    or browser handoff), whether a dry run is available, its approval gate, and
//!    a rollback note. [`compute_step_execution`] turns those into a
//!    [`StepExecutionClass`] admission, so a mutating step is *never* silently run:
//!    it previews before applying, blocks awaiting approval, blocks behind a
//!    boundary, hands off to a browser, or is read-only on imported evidence.
//! 3. **Local continuity during an outage is explicit.** Each
//!    [`ContinuityView`] names which boundary failed, what still works locally, and
//!    the next safe action — so an impaired provider or managed control plane never
//!    reads as "the whole product is down".
//!
//! [`response_pane_set`] is the canonical binding: it builds the strips, panes, and
//! continuity views deterministically and computes each [`PaneInvariant`]'s `holds`
//! flag from the built data, so the checked-in fixture and the replay gate freeze
//! the contract byte-for-byte and an inconsistent edit flips an invariant rather
//! than silently passing. The record carries no endpoint URLs, hostnames,
//! credentials, raw payloads, or absolute paths — only opaque object refs, stable
//! tokens, and short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::m5_operator_boards::{compute_effective_state, BlockerWaiverClass, FreshnessClass};
use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorStateClass, OperatorSurfaceClass, RedactionClass,
    ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the response-pane set.
pub const M5_RESPONSE_PANES_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the response-pane set.
pub const M5_RESPONSE_PANES_SCHEMA_REF: &str = "schemas/ops/m5-response-panes.schema.json";

/// Stable record-kind tag for the response-pane set.
pub const M5_RESPONSE_PANES_RECORD_KIND: &str = "m5_response_pane_set";

/// Stable id for the canonical response-pane set.
pub const M5_RESPONSE_PANES_SET_ID: &str = "m5-response-panes:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_RESPONSE_PANES_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this set binds for surface identity.
pub const M5_RESPONSE_PANES_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this set binds.
pub const M5_RESPONSE_PANES_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Service-ownership vocabulary.
// ---------------------------------------------------------------------------

/// Whether a service-ownership strip's state comes from an authoritative source
/// or is only advisory.
///
/// An advisory source is never allowed to assert a confirmed-healthy service;
/// consumers downgrade an advisory clear the same way a stale one is downgraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySourceClass {
    /// First-party authoritative service-health truth.
    Authoritative,
    /// Advisory, mirror-backed or cached truth from a last sync.
    AdvisoryMirror,
    /// Advisory third-party status, not owned by this product.
    AdvisoryThirdParty,
}

impl AuthoritySourceClass {
    /// All authority sources, in vocabulary order.
    pub const ALL: [Self; 3] = [
        Self::Authoritative,
        Self::AdvisoryMirror,
        Self::AdvisoryThirdParty,
    ];

    /// Stable snake_case token for this source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::AdvisoryMirror => "advisory_mirror",
            Self::AdvisoryThirdParty => "advisory_third_party",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Authoritative => "Authoritative",
            Self::AdvisoryMirror => "Advisory (mirror)",
            Self::AdvisoryThirdParty => "Advisory (third-party)",
        }
    }

    /// Whether the source is advisory and so can never assert a confirmed-green
    /// service on its own.
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::AdvisoryMirror | Self::AdvisoryThirdParty)
    }
}

/// What stays usable locally when the service behind a strip is impaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityClass {
    /// Fully local: the service runs on-host and an outage elsewhere does not
    /// touch it.
    FullyLocal,
    /// The local core (edit, save, search, git, build/test, export) stays
    /// available while the remote/managed feature is impaired.
    LocalCoreSafe,
    /// A last-synced mirror is readable read-only; writes queue to publish later.
    MirrorReadOnly,
    /// The service is remote-required: no local fallback exists and the strip says
    /// so plainly.
    RemoteRequiredNoFallback,
}

impl LocalContinuityClass {
    /// All continuity postures, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::FullyLocal,
        Self::LocalCoreSafe,
        Self::MirrorReadOnly,
        Self::RemoteRequiredNoFallback,
    ];

    /// Stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyLocal => "fully_local",
            Self::LocalCoreSafe => "local_core_safe",
            Self::MirrorReadOnly => "mirror_read_only",
            Self::RemoteRequiredNoFallback => "remote_required_no_fallback",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullyLocal => "Fully local",
            Self::LocalCoreSafe => "Local core safe",
            Self::MirrorReadOnly => "Mirror read-only",
            Self::RemoteRequiredNoFallback => "Remote-required (no local fallback)",
        }
    }
}

// ---------------------------------------------------------------------------
// Runbook-step vocabulary.
// ---------------------------------------------------------------------------

/// The intent of a runbook step: what kind of action it is.
///
/// The split that matters for safety is read-only versus mutating. Observe and
/// verify steps never change state; mitigate and rollback steps do and so carry
/// an approval gate and a rollback note. Communicate steps coordinate people and
/// never touch a system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepIntentClass {
    /// Observe: read live signals; no change.
    Observe,
    /// Verify: run a verification-only check; no change.
    Verify,
    /// Mitigate: change live or managed state to reduce impact.
    Mitigate,
    /// Rollback: revert a prior change.
    Rollback,
    /// Communicate: coordinate people or hand off; no system change.
    Communicate,
}

impl StepIntentClass {
    /// All intents, in vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::Observe,
        Self::Verify,
        Self::Mitigate,
        Self::Rollback,
        Self::Communicate,
    ];

    /// Stable snake_case token for this intent.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Verify => "verify",
            Self::Mitigate => "mitigate",
            Self::Rollback => "rollback",
            Self::Communicate => "communicate",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Observe => "Observe",
            Self::Verify => "Verify",
            Self::Mitigate => "Mitigate",
            Self::Rollback => "Rollback",
            Self::Communicate => "Communicate",
        }
    }

    /// Whether the step changes live or managed state.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::Mitigate | Self::Rollback)
    }

    /// Whether the step only reads state.
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::Observe | Self::Verify)
    }
}

/// Where a step's action lands: the local-versus-remote/managed boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionBoundaryClass {
    /// Acts only on local objects, with no remote or managed call.
    LocalOnly,
    /// Acts on a remote workspace / runtime.
    RemoteWorkspace,
    /// Acts on the managed control plane.
    ManagedControlPlane,
    /// Exits to a browser / console: an attributable handoff, never a native apply.
    BrowserHandoff,
}

impl ActionBoundaryClass {
    /// All boundaries, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::RemoteWorkspace,
        Self::ManagedControlPlane,
        Self::BrowserHandoff,
    ];

    /// Stable snake_case token for this boundary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::RemoteWorkspace => "remote_workspace",
            Self::ManagedControlPlane => "managed_control_plane",
            Self::BrowserHandoff => "browser_handoff",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::RemoteWorkspace => "Remote workspace",
            Self::ManagedControlPlane => "Managed control plane",
            Self::BrowserHandoff => "Browser handoff",
        }
    }

    /// Whether the boundary reaches a remote or managed target, so a blocking
    /// window or boundary drift on that target stops a mutating apply.
    pub const fn is_remote_or_managed(self) -> bool {
        matches!(self, Self::RemoteWorkspace | Self::ManagedControlPlane)
    }
}

/// The approval gate a mutating step must clear before it applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalGateClass {
    /// No approval required.
    None,
    /// One approver must grant before apply.
    SingleApproval,
    /// Two-person control: two approvers must grant before apply.
    DualControl,
}

impl ApprovalGateClass {
    /// All gates, in vocabulary order.
    pub const ALL: [Self; 3] = [Self::None, Self::SingleApproval, Self::DualControl];

    /// Stable snake_case token for this gate.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SingleApproval => "single_approval",
            Self::DualControl => "dual_control",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::SingleApproval => "Single approval",
            Self::DualControl => "Dual control",
        }
    }

    /// Whether this gate requires an approval before a mutating apply.
    pub const fn requires_approval(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The state of a step's approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStateClass {
    /// No approval is required for this step.
    NotRequired,
    /// An approval is required and has not yet been granted.
    Pending,
    /// A fresh approval has been granted.
    Granted,
    /// An approval was granted but has since lapsed; the gate is in force again.
    Expired,
}

impl ApprovalStateClass {
    /// All states, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::NotRequired,
        Self::Pending,
        Self::Granted,
        Self::Expired,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Pending => "pending",
            Self::Granted => "granted",
            Self::Expired => "expired",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotRequired => "Not required",
            Self::Pending => "Pending",
            Self::Granted => "Granted",
            Self::Expired => "Expired",
        }
    }
}

/// The computed admission for a step: whether and how it may run *now*.
///
/// This is the executable form of the preview/approval rule. A read-only step
/// runs locally; a mutating step is never reported as runnable-now — it previews
/// before applying, blocks awaiting approval, blocks behind a boundary, hands off
/// to a browser, or is read-only because the evidence is imported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepExecutionClass {
    /// Runs now against local/live evidence; no mutation and no approval needed.
    RunLocal,
    /// A mutating apply is admitted but must be previewed (dry-run) before it runs.
    PreviewBeforeApply,
    /// A mutating apply is blocked until its approval gate is satisfied.
    BlockedAwaitingApproval,
    /// A mutating apply is blocked by an active window, failover, migration, or
    /// boundary drift on its remote/managed target.
    BlockedByBoundary,
    /// The step exits to a browser/console: an attributable handoff, not a native
    /// apply.
    ExternalBrowserHandoff,
    /// The step targets imported/replay evidence with no live target; read-only.
    ReadOnlyImportedSnapshot,
}

impl StepExecutionClass {
    /// All admissions, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::RunLocal,
        Self::PreviewBeforeApply,
        Self::BlockedAwaitingApproval,
        Self::BlockedByBoundary,
        Self::ExternalBrowserHandoff,
        Self::ReadOnlyImportedSnapshot,
    ];

    /// Stable snake_case token for this admission.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunLocal => "run_local",
            Self::PreviewBeforeApply => "preview_before_apply",
            Self::BlockedAwaitingApproval => "blocked_awaiting_approval",
            Self::BlockedByBoundary => "blocked_by_boundary",
            Self::ExternalBrowserHandoff => "external_browser_handoff",
            Self::ReadOnlyImportedSnapshot => "read_only_imported_snapshot",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RunLocal => "Run locally",
            Self::PreviewBeforeApply => "Preview before apply",
            Self::BlockedAwaitingApproval => "Blocked — awaiting approval",
            Self::BlockedByBoundary => "Blocked — boundary",
            Self::ExternalBrowserHandoff => "External browser handoff",
            Self::ReadOnlyImportedSnapshot => "Read-only (imported snapshot)",
        }
    }

    /// Whether this admission performs a live mutation (only the preview-then-apply
    /// path does, and only after its preview).
    pub const fn applies_live_mutation(self) -> bool {
        matches!(self, Self::PreviewBeforeApply)
    }
}

/// Computes a step's admission from its intent, boundary, approval, and the
/// current state of its target.
///
/// The order encodes the safety priority: an imported snapshot is always
/// read-only; a browser handoff is always an external exit; read-only and
/// communicate intents always run locally; and only then is a mutating intent
/// evaluated — boundary first (a window/failover/drift blocks managed writes),
/// then approval, and finally the preview-before-apply admission. A mutating step
/// therefore can never resolve to [`StepExecutionClass::RunLocal`].
pub fn compute_step_execution(
    intent: StepIntentClass,
    boundary: ActionBoundaryClass,
    approval_gate: ApprovalGateClass,
    approval_state: ApprovalStateClass,
    boundary_state: OperatorStateClass,
    live_target_present: bool,
) -> StepExecutionClass {
    if !live_target_present {
        return StepExecutionClass::ReadOnlyImportedSnapshot;
    }
    if boundary == ActionBoundaryClass::BrowserHandoff {
        return StepExecutionClass::ExternalBrowserHandoff;
    }
    if !intent.is_mutating() {
        return StepExecutionClass::RunLocal;
    }
    if boundary.is_remote_or_managed() && boundary_state.blocking_default() {
        return StepExecutionClass::BlockedByBoundary;
    }
    if approval_gate.requires_approval() && approval_state != ApprovalStateClass::Granted {
        return StepExecutionClass::BlockedAwaitingApproval;
    }
    StepExecutionClass::PreviewBeforeApply
}

// ---------------------------------------------------------------------------
// Continuity vocabulary.
// ---------------------------------------------------------------------------

/// The kind of continuity event a [`ContinuityView`] describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityKindClass {
    /// A planned maintenance window.
    PlannedMaintenance,
    /// A read-only window: managed writes are blocked, local work continues.
    ReadOnlyWindow,
    /// A drain window: in-flight work finishes, new actions queue.
    DrainWindow,
    /// A regional failover is in progress.
    RegionalFailover,
    /// A tenant/region/residency migration is in progress.
    TenantMigration,
    /// A provider/service outage is impairing a remote dependency.
    ProviderOutage,
}

impl ContinuityKindClass {
    /// All kinds, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::PlannedMaintenance,
        Self::ReadOnlyWindow,
        Self::DrainWindow,
        Self::RegionalFailover,
        Self::TenantMigration,
        Self::ProviderOutage,
    ];

    /// Stable snake_case token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlannedMaintenance => "planned_maintenance",
            Self::ReadOnlyWindow => "read_only_window",
            Self::DrainWindow => "drain_window",
            Self::RegionalFailover => "regional_failover",
            Self::TenantMigration => "tenant_migration",
            Self::ProviderOutage => "provider_outage",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PlannedMaintenance => "Planned maintenance",
            Self::ReadOnlyWindow => "Read-only window",
            Self::DrainWindow => "Drain window",
            Self::RegionalFailover => "Regional failover",
            Self::TenantMigration => "Tenant migration",
            Self::ProviderOutage => "Provider outage",
        }
    }

    /// The operator-surface matrix family this continuity kind renders on:
    /// planned/read-only/drain windows bind the maintenance notice, while
    /// failover, migration, and provider outages bind the failover notice.
    pub const fn surface(self) -> OperatorSurfaceClass {
        match self {
            Self::PlannedMaintenance | Self::ReadOnlyWindow | Self::DrainWindow => {
                OperatorSurfaceClass::MaintenanceNotice
            }
            Self::RegionalFailover | Self::TenantMigration | Self::ProviderOutage => {
                OperatorSurfaceClass::FailoverNotice
            }
        }
    }
}

/// Which boundary failed (or is changing) in a continuity event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailedBoundaryClass {
    /// Nothing failed: a planned, informational, or scheduled state.
    None,
    /// The managed control plane is unreachable or in maintenance.
    ControlPlane,
    /// A region is failing over.
    Region,
    /// A tenant is migrating.
    Tenant,
    /// A provider endpoint is impaired.
    ProviderEndpoint,
    /// Network reachability is lost: the host is offline.
    NetworkReachability,
}

impl FailedBoundaryClass {
    /// All boundaries, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::ControlPlane,
        Self::Region,
        Self::Tenant,
        Self::ProviderEndpoint,
        Self::NetworkReachability,
    ];

    /// Stable snake_case token for this boundary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ControlPlane => "control_plane",
            Self::Region => "region",
            Self::Tenant => "tenant",
            Self::ProviderEndpoint => "provider_endpoint",
            Self::NetworkReachability => "network_reachability",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::ControlPlane => "Control plane",
            Self::Region => "Region",
            Self::Tenant => "Tenant",
            Self::ProviderEndpoint => "Provider endpoint",
            Self::NetworkReachability => "Network reachability",
        }
    }

    /// Whether an actual boundary failed (anything but [`FailedBoundaryClass::None`]).
    pub const fn failed(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// The local-safe capabilities a continuity view says still work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCapabilityClass {
    /// Edit files locally.
    Edit,
    /// Save changes to disk.
    Save,
    /// Search the local workspace.
    Search,
    /// Use local git / version control.
    GitVersioning,
    /// Run local build and test.
    BuildTest,
    /// Export local diagnostics / evidence.
    ExportDiagnostics,
    /// Inspect evidence and history locally.
    InspectEvidence,
    /// Capture writes to publish later when the boundary is restored.
    PublishLater,
    /// Open local history / timeline.
    OpenLocalHistory,
}

impl LocalCapabilityClass {
    /// All capabilities, in vocabulary order.
    pub const ALL: [Self; 9] = [
        Self::Edit,
        Self::Save,
        Self::Search,
        Self::GitVersioning,
        Self::BuildTest,
        Self::ExportDiagnostics,
        Self::InspectEvidence,
        Self::PublishLater,
        Self::OpenLocalHistory,
    ];

    /// Stable snake_case token for this capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Edit => "edit",
            Self::Save => "save",
            Self::Search => "search",
            Self::GitVersioning => "git_versioning",
            Self::BuildTest => "build_test",
            Self::ExportDiagnostics => "export_diagnostics",
            Self::InspectEvidence => "inspect_evidence",
            Self::PublishLater => "publish_later",
            Self::OpenLocalHistory => "open_local_history",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Edit => "Edit",
            Self::Save => "Save",
            Self::Search => "Search",
            Self::GitVersioning => "Git / version control",
            Self::BuildTest => "Build / test",
            Self::ExportDiagnostics => "Export diagnostics",
            Self::InspectEvidence => "Inspect evidence",
            Self::PublishLater => "Publish later",
            Self::OpenLocalHistory => "Open local history",
        }
    }
}

/// The next safe action a continuity view recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextSafeActionClass {
    /// Continue working locally; nothing local is blocked.
    ContinueLocal,
    /// Capture writes to publish when the boundary is restored.
    PublishLater,
    /// Export diagnostics now for the handoff.
    ExportDiagnostics,
    /// Review the changed boundary before resuming managed writes.
    ReviewNewBoundary,
    /// Retry when the provider/boundary is restored.
    RetryWhenRestored,
    /// Open the continuity packet for full detail.
    OpenContinuityPacket,
}

impl NextSafeActionClass {
    /// All actions, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::ContinueLocal,
        Self::PublishLater,
        Self::ExportDiagnostics,
        Self::ReviewNewBoundary,
        Self::RetryWhenRestored,
        Self::OpenContinuityPacket,
    ];

    /// Stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueLocal => "continue_local",
            Self::PublishLater => "publish_later",
            Self::ExportDiagnostics => "export_diagnostics",
            Self::ReviewNewBoundary => "review_new_boundary",
            Self::RetryWhenRestored => "retry_when_restored",
            Self::OpenContinuityPacket => "open_continuity_packet",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ContinueLocal => "Continue local",
            Self::PublishLater => "Publish later",
            Self::ExportDiagnostics => "Export diagnostics",
            Self::ReviewNewBoundary => "Review new boundary",
            Self::RetryWhenRestored => "Retry when restored",
            Self::OpenContinuityPacket => "Open continuity packet",
        }
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// The escalation action a service-ownership strip offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscalationAction {
    /// Stable token for the action.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// The canonical object the escalation routes to (a contact, page, or owner
    /// record handle).
    pub routes_to_ref: String,
    /// One reviewable sentence describing the escalation.
    pub summary: String,
}

/// One service-ownership / on-call strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOwnershipStrip {
    /// Stable, namespaced strip id.
    pub strip_id: String,
    /// The canonical service-health object handle this strip summarizes.
    pub object_ref: String,
    /// The service family / name token.
    pub service_family: String,
    /// The environment label (for example `production`); never a host or URL.
    pub environment: String,
    /// The bound matrix surface family (always the service-ownership strip).
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`).
    pub surface_id: String,
    /// The primary owner.
    pub primary_owner: String,
    /// The backup owner.
    pub backup_owner: String,
    /// The active on-call lane / rotation.
    pub on_call_lane: String,
    /// Who holds the decision right for changes to this service.
    pub decision_right: String,
    /// Whether the strip's state is authoritative or advisory.
    pub authority_source: AuthoritySourceClass,
    /// The escalation action this strip offers.
    pub escalation: EscalationAction,
    /// The state the strip would headline before the no-silent-green downgrade.
    pub displayed_state: OperatorStateClass,
    /// The freshness age of the last health check behind the displayed state.
    pub freshness: FreshnessClass,
    /// The computed effective state; a stale strip is never reported `clear`.
    pub effective_state: OperatorStateClass,
    /// What stays usable locally if this service is impaired.
    pub local_continuity: LocalContinuityClass,
    /// The canonical evidence object behind the displayed state.
    pub evidence_ref: String,
    /// Local-versus-shared scope of the underlying object.
    pub scope: ScopeClass,
    /// The open-detail route; equals [`ServiceOwnershipStrip::object_ref`].
    pub open_detail_ref: String,
}

/// One runbook step within a response pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookStep {
    /// Stable, pane-namespaced step id.
    pub step_id: String,
    /// 1-based position in the pane's ordered steps.
    pub ordinal: u32,
    /// Short title.
    pub title: String,
    /// The canonical runbook-step object handle.
    pub object_ref: String,
    /// The step intent (observe, verify, mitigate, rollback, communicate).
    pub intent: StepIntentClass,
    /// Where the step's action lands (local, remote, managed, browser handoff).
    pub boundary: ActionBoundaryClass,
    /// One reviewable sentence naming the target scope of the step.
    pub target_scope: String,
    /// Whether a dry run / preview is available before any apply.
    pub dry_run_available: bool,
    /// The approval gate a mutating apply must clear.
    pub approval_gate: ApprovalGateClass,
    /// The current state of that approval.
    pub approval_state: ApprovalStateClass,
    /// The rollback note: how to undo a mutating step (empty for non-mutating).
    pub rollback_note: String,
    /// The current state of the step's action boundary (a window, failover, drift,
    /// or clear), used to compute the admission.
    pub boundary_state: OperatorStateClass,
    /// Whether a live target exists; false for imported/replay evidence.
    pub live_target_present: bool,
    /// The computed admission ([`compute_step_execution`]); a mutating step is
    /// never `run_local`.
    pub execution: StepExecutionClass,
    /// The canonical evidence object behind the step.
    pub evidence_ref: String,
    /// The open-detail route; equals [`RunbookStep::object_ref`].
    pub open_detail_ref: String,
}

/// One runbook-guided response pane: an ordered set of steps for one incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookResponsePane {
    /// Stable, namespaced pane id.
    pub pane_id: String,
    /// The canonical runbook object handle this pane renders.
    pub object_ref: String,
    /// Short title.
    pub title: String,
    /// One reviewable sentence describing the pane.
    pub summary: String,
    /// The bound matrix surface family (always the runbook-step card).
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`).
    pub surface_id: String,
    /// The canonical incident object this runbook responds to.
    pub incident_ref: String,
    /// The pane owner.
    pub owner: String,
    /// Who holds the decision right for mutating steps in this pane.
    pub decision_right: String,
    /// Local-versus-shared scope of the underlying objects.
    pub scope: ScopeClass,
    /// The default redaction posture on export.
    pub default_redaction: RedactionClass,
    /// The consumers that render this pane.
    pub consumed_by: Vec<ConsumerClass>,
    /// Live-versus-snapshot posture of the pane.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// The ordered steps.
    pub steps: Vec<RunbookStep>,
    /// The open-detail route; equals [`RunbookResponsePane::object_ref`].
    pub open_detail_ref: String,
}

/// One local-outage continuity view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityView {
    /// Stable, namespaced view id.
    pub view_id: String,
    /// The canonical continuity / notice object handle this view summarizes.
    pub object_ref: String,
    /// Short title.
    pub title: String,
    /// One reviewable sentence describing the continuity state.
    pub summary: String,
    /// The continuity kind.
    pub kind: ContinuityKindClass,
    /// The bound matrix surface family (maintenance notice or failover notice).
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`).
    pub surface_id: String,
    /// The continuity owner.
    pub owner: String,
    /// The state the view would headline before the no-silent-green downgrade.
    pub displayed_state: OperatorStateClass,
    /// The freshness age of the evidence behind the displayed state.
    pub freshness: FreshnessClass,
    /// The computed effective state.
    pub effective_state: OperatorStateClass,
    /// Which boundary failed (or `none` for a planned, informational state).
    pub failed_boundary: FailedBoundaryClass,
    /// The local-safe capabilities that still work during this event.
    pub local_capabilities: Vec<LocalCapabilityClass>,
    /// The capabilities that are blocked, named plainly.
    pub blocked_capabilities: Vec<String>,
    /// The next safe action this view recommends.
    pub next_safe_action: NextSafeActionClass,
    /// Whether publish-later capture is offered while managed writes are blocked.
    pub publish_later_capture: bool,
    /// Local-versus-shared scope of the underlying object.
    pub scope: ScopeClass,
    /// The open-detail route; equals [`ContinuityView::object_ref`].
    pub open_detail_ref: String,
}

impl ContinuityView {
    /// Whether this view's effective state blocks managed/side-effectful writes.
    pub fn blocks_managed_writes(&self) -> bool {
        self.effective_state.blocking_default()
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen response-pane set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsePaneSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_response_panes_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The operator-surface matrix fixture this set binds for surface identity.
    pub matrix_ref: String,
    /// The matrix record kind this set binds.
    pub matrix_record_kind: String,
    /// The step-intent vocabulary, for consumers.
    pub step_intents: Vec<TokenDef>,
    /// The action-boundary vocabulary, for consumers.
    pub action_boundaries: Vec<TokenDef>,
    /// The step-execution (admission) vocabulary, for consumers.
    pub execution_classes: Vec<TokenDef>,
    /// The service-ownership / on-call strips.
    pub service_strips: Vec<ServiceOwnershipStrip>,
    /// The runbook-guided response panes.
    pub runbook_panes: Vec<RunbookResponsePane>,
    /// The local-outage continuity views.
    pub continuity_views: Vec<ContinuityView>,
    /// The computed invariants.
    pub invariants: Vec<PaneInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for PaneValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "response-pane set invalid: {}", self.reason)
    }
}

impl std::error::Error for PaneValidationError {}

impl ResponsePaneSet {
    /// Returns the strip with the given id, if present.
    pub fn strip(&self, strip_id: &str) -> Option<&ServiceOwnershipStrip> {
        self.service_strips.iter().find(|s| s.strip_id == strip_id)
    }

    /// Returns the pane with the given id, if present.
    pub fn pane(&self, pane_id: &str) -> Option<&RunbookResponsePane> {
        self.runbook_panes.iter().find(|p| p.pane_id == pane_id)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().iter().all(|r| is_export_safe_ref(r))
    }

    /// Every ref string carried by the set, for export-safety auditing.
    fn all_refs(&self) -> Vec<&str> {
        let mut refs = vec![self.matrix_ref.as_str(), self.schema_ref.as_str()];
        for s in &self.service_strips {
            refs.push(s.object_ref.as_str());
            refs.push(s.evidence_ref.as_str());
            refs.push(s.open_detail_ref.as_str());
            refs.push(s.escalation.routes_to_ref.as_str());
        }
        for p in &self.runbook_panes {
            refs.push(p.object_ref.as_str());
            refs.push(p.incident_ref.as_str());
            refs.push(p.open_detail_ref.as_str());
            for step in &p.steps {
                refs.push(step.object_ref.as_str());
                refs.push(step.evidence_ref.as_str());
                refs.push(step.open_detail_ref.as_str());
            }
        }
        for v in &self.continuity_views {
            refs.push(v.object_ref.as_str());
            refs.push(v.open_detail_ref.as_str());
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`PaneInvariant`]s with the uniqueness, computed-
    /// state, and surface-binding checks a consumer relies on.
    pub fn validate(&self) -> Result<(), PaneValidationError> {
        let fail = |reason: String| Err(PaneValidationError { reason });

        if self.record_kind != M5_RESPONSE_PANES_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_RESPONSE_PANES_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_RESPONSE_PANES_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }
        if self.service_strips.is_empty() {
            return fail("set has no service strips".to_owned());
        }
        if self.runbook_panes.is_empty() {
            return fail("set has no runbook panes".to_owned());
        }
        if self.continuity_views.is_empty() {
            return fail("set has no continuity views".to_owned());
        }

        // Ids are unique across the whole set.
        if !all_unique(self.service_strips.iter().map(|s| s.strip_id.as_str())) {
            return fail("strip ids are not unique".to_owned());
        }
        if !all_unique(self.runbook_panes.iter().map(|p| p.pane_id.as_str())) {
            return fail("pane ids are not unique".to_owned());
        }
        if !all_unique(self.continuity_views.iter().map(|v| v.view_id.as_str())) {
            return fail("view ids are not unique".to_owned());
        }
        if !all_unique(
            self.runbook_panes
                .iter()
                .flat_map(|p| p.steps.iter().map(|s| s.step_id.as_str())),
        ) {
            return fail("step ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
        let bind = |surface: OperatorSurfaceClass, surface_id: &str| -> bool {
            surface_id == surface.surface_id() && matrix.surface(surface).is_some()
        };

        // Service strips: owner/on-call/authority visible, computed no-silent-green,
        // canonical identity, escalation present.
        for s in &self.service_strips {
            if s.surface != OperatorSurfaceClass::ServiceOwnershipStrip
                || !bind(s.surface, &s.surface_id)
            {
                return fail(format!(
                    "strip {} does not bind its matrix surface",
                    s.strip_id
                ));
            }
            if !s.object_ref.starts_with("aureline://") || s.open_detail_ref != s.object_ref {
                return fail(format!("strip {} hides its canonical object", s.strip_id));
            }
            if s.primary_owner.is_empty()
                || s.on_call_lane.is_empty()
                || s.decision_right.is_empty()
            {
                return fail(format!(
                    "strip {} hides owner/on-call/decision-right",
                    s.strip_id
                ));
            }
            if s.escalation.routes_to_ref.is_empty() || s.escalation.label.is_empty() {
                return fail(format!("strip {} has no escalation action", s.strip_id));
            }
            let expected =
                compute_effective_state(s.displayed_state, s.freshness, BlockerWaiverClass::None);
            if s.effective_state != expected {
                return fail(format!(
                    "strip {} effective state is not the computed no-silent-green state",
                    s.strip_id
                ));
            }
        }

        // Runbook panes: ordered steps, computed admission, gated mutation.
        for p in &self.runbook_panes {
            if p.surface != OperatorSurfaceClass::RunbookStepCard || !bind(p.surface, &p.surface_id)
            {
                return fail(format!(
                    "pane {} does not bind its matrix surface",
                    p.pane_id
                ));
            }
            if !p.object_ref.starts_with("aureline://") || p.open_detail_ref != p.object_ref {
                return fail(format!("pane {} hides its canonical object", p.pane_id));
            }
            if !p.incident_ref.starts_with("aureline://") {
                return fail(format!("pane {} has no canonical incident ref", p.pane_id));
            }
            if p.steps.is_empty() {
                return fail(format!("pane {} has no steps", p.pane_id));
            }
            for (idx, step) in p.steps.iter().enumerate() {
                if step.ordinal as usize != idx + 1 {
                    return fail(format!(
                        "pane {} step {} is out of order",
                        p.pane_id, step.step_id
                    ));
                }
                if !step.object_ref.starts_with("aureline://")
                    || step.open_detail_ref != step.object_ref
                {
                    return fail(format!("step {} hides its canonical object", step.step_id));
                }
                let expected = compute_step_execution(
                    step.intent,
                    step.boundary,
                    step.approval_gate,
                    step.approval_state,
                    step.boundary_state,
                    step.live_target_present,
                );
                if step.execution != expected {
                    return fail(format!(
                        "step {} execution is not the computed admission",
                        step.step_id
                    ));
                }
                if step.intent.is_mutating() {
                    if step.execution == StepExecutionClass::RunLocal {
                        return fail(format!(
                            "step {} mutates but is admitted to run locally without preview",
                            step.step_id
                        ));
                    }
                    if !step.dry_run_available {
                        return fail(format!(
                            "step {} mutates without a dry-run / preview path",
                            step.step_id
                        ));
                    }
                    if step.rollback_note.is_empty() {
                        return fail(format!(
                            "step {} mutates without a rollback note",
                            step.step_id
                        ));
                    }
                } else {
                    if step.approval_gate != ApprovalGateClass::None
                        || step.approval_state != ApprovalStateClass::NotRequired
                    {
                        return fail(format!(
                            "non-mutating step {} carries an approval gate",
                            step.step_id
                        ));
                    }
                    if step.execution == StepExecutionClass::BlockedAwaitingApproval {
                        return fail(format!(
                            "non-mutating step {} is blocked on approval",
                            step.step_id
                        ));
                    }
                }
                // Approval gate and state must agree.
                if step.approval_gate == ApprovalGateClass::None
                    && step.approval_state != ApprovalStateClass::NotRequired
                {
                    return fail(format!(
                        "step {} has no gate but a non-trivial approval state",
                        step.step_id
                    ));
                }
                if step.approval_gate.requires_approval()
                    && step.approval_state == ApprovalStateClass::NotRequired
                {
                    return fail(format!(
                        "step {} requires approval but its state says not-required",
                        step.step_id
                    ));
                }
            }
        }

        // Continuity views: local capabilities explicit, no total-outage claim,
        // publish-later when writes are blocked.
        for v in &self.continuity_views {
            if v.surface != v.kind.surface() || !bind(v.surface, &v.surface_id) {
                return fail(format!(
                    "view {} does not bind its matrix surface",
                    v.view_id
                ));
            }
            if !v.object_ref.starts_with("aureline://") || v.open_detail_ref != v.object_ref {
                return fail(format!("view {} hides its canonical object", v.view_id));
            }
            if v.local_capabilities.is_empty() {
                return fail(format!("view {} lists no local capabilities", v.view_id));
            }
            if v.failed_boundary.failed() && v.local_capabilities.is_empty() {
                return fail(format!(
                    "view {} claims a total outage with no local capability",
                    v.view_id
                ));
            }
            let expected =
                compute_effective_state(v.displayed_state, v.freshness, BlockerWaiverClass::None);
            if v.effective_state != expected {
                return fail(format!(
                    "view {} effective state is not the computed no-silent-green state",
                    v.view_id
                ));
            }
            if v.blocks_managed_writes() && !v.publish_later_capture {
                return fail(format!(
                    "view {} blocks managed writes without publish-later capture",
                    v.view_id
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("set is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

fn scope_token(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::LocalPrivate => "local_private",
        ScopeClass::SharedTeam => "shared_team",
        ScopeClass::ManagedOrg => "managed_org",
    }
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical response-pane set.
///
/// Deterministic: the same bytes every call. Strip effective states, step
/// admissions, and the invariant `holds` flags are computed from the built data,
/// so an inconsistent edit flips an invariant rather than silently passing.
pub fn response_pane_set() -> ResponsePaneSet {
    let service_strips = build_service_strips();
    let runbook_panes = build_runbook_panes();
    let continuity_views = build_continuity_views();
    let invariants = compute_invariants(&service_strips, &runbook_panes, &continuity_views);

    ResponsePaneSet {
        record_kind: M5_RESPONSE_PANES_RECORD_KIND.to_owned(),
        m5_response_panes_schema_version: M5_RESPONSE_PANES_SCHEMA_VERSION,
        schema_ref: M5_RESPONSE_PANES_SCHEMA_REF.to_owned(),
        set_id: M5_RESPONSE_PANES_SET_ID.to_owned(),
        as_of: M5_RESPONSE_PANES_AS_OF.to_owned(),
        summary: "The first real Aureline operator response surfaces — service-ownership / on-call \
                  strips, runbook-guided response panes with computed mutating-step preview/approval \
                  admission, and local-outage continuity views — bound to the operator-surface \
                  matrix so ownership, on-call authority, step authority, and local continuity stay \
                  visible and exportable from first alert to handoff."
            .to_owned(),
        matrix_ref: M5_RESPONSE_PANES_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_RESPONSE_PANES_MATRIX_RECORD_KIND.to_owned(),
        step_intents: token_defs(StepIntentClass::ALL.iter().map(|i| (i.as_str(), i.label()))),
        action_boundaries: token_defs(
            ActionBoundaryClass::ALL
                .iter()
                .map(|b| (b.as_str(), b.label())),
        ),
        execution_classes: token_defs(
            StepExecutionClass::ALL
                .iter()
                .map(|e| (e.as_str(), e.label())),
        ),
        service_strips,
        runbook_panes,
        continuity_views,
        invariants,
        raw_payload_excluded: true,
    }
}

fn token_defs<'a>(items: impl Iterator<Item = (&'a str, &'a str)>) -> Vec<TokenDef> {
    items
        .map(|(token, label)| TokenDef {
            token: token.to_owned(),
            label: label.to_owned(),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn strip(
    n: u32,
    object_ref: &str,
    service_family: &str,
    environment: &str,
    primary_owner: &str,
    backup_owner: &str,
    on_call_lane: &str,
    decision_right: &str,
    authority_source: AuthoritySourceClass,
    escalation: EscalationAction,
    displayed_state: OperatorStateClass,
    freshness: FreshnessClass,
    local_continuity: LocalContinuityClass,
    evidence_ref: &str,
    scope: ScopeClass,
) -> ServiceOwnershipStrip {
    ServiceOwnershipStrip {
        strip_id: format!("service_strip.{n:04}"),
        object_ref: object_ref.to_owned(),
        service_family: service_family.to_owned(),
        environment: environment.to_owned(),
        surface: OperatorSurfaceClass::ServiceOwnershipStrip,
        surface_id: OperatorSurfaceClass::ServiceOwnershipStrip.surface_id(),
        primary_owner: primary_owner.to_owned(),
        backup_owner: backup_owner.to_owned(),
        on_call_lane: on_call_lane.to_owned(),
        decision_right: decision_right.to_owned(),
        authority_source,
        escalation,
        displayed_state,
        freshness,
        effective_state: compute_effective_state(
            displayed_state,
            freshness,
            BlockerWaiverClass::None,
        ),
        local_continuity,
        evidence_ref: evidence_ref.to_owned(),
        scope,
        open_detail_ref: object_ref.to_owned(),
    }
}

fn escalation(token: &str, label: &str, routes_to_ref: &str, summary: &str) -> EscalationAction {
    EscalationAction {
        token: token.to_owned(),
        label: label.to_owned(),
        routes_to_ref: routes_to_ref.to_owned(),
        summary: summary.to_owned(),
    }
}

fn build_service_strips() -> Vec<ServiceOwnershipStrip> {
    use AuthoritySourceClass as A;
    use FreshnessClass as F;
    use LocalContinuityClass as L;
    use OperatorStateClass as S;

    vec![
        strip(
            1,
            "aureline://service-health/svc-auth-provider",
            "auth_provider",
            "production",
            "identity_oncall",
            "identity_lead",
            "identity_primary_rotation",
            "identity_lead",
            A::Authoritative,
            escalation(
                "page_identity_oncall",
                "Page identity on-call",
                "aureline://on-call/identity_primary_rotation",
                "Escalate to the active identity on-call rotation.",
            ),
            S::Attention,
            F::Fresh,
            L::LocalCoreSafe,
            "aureline://evidence/svc-auth-provider-card",
            ScopeClass::SharedTeam,
        ),
        // Advisory + stale: the green dot is downgraded to unconfirmed.
        strip(
            2,
            "aureline://service-health/svc-search-index",
            "search_index",
            "production",
            "platform_oncall",
            "platform_lead",
            "platform_primary_rotation",
            "platform_lead",
            A::AdvisoryMirror,
            escalation(
                "page_platform_oncall",
                "Page platform on-call",
                "aureline://on-call/platform_primary_rotation",
                "Escalate to the active platform on-call rotation.",
            ),
            S::Clear,
            F::Stale,
            L::MirrorReadOnly,
            "aureline://evidence/svc-search-index-card",
            ScopeClass::SharedTeam,
        ),
        // Managed control plane mid-failover: state carries through unchanged.
        strip(
            3,
            "aureline://service-health/svc-managed-control-plane",
            "managed_control_plane",
            "production",
            "platform_oncall",
            "sre_lead",
            "platform_primary_rotation",
            "sre_lead",
            A::Authoritative,
            escalation(
                "page_sre_oncall",
                "Page SRE on-call",
                "aureline://on-call/sre_primary_rotation",
                "Escalate to the active SRE on-call rotation for control-plane events.",
            ),
            S::FailoverInProgress,
            F::Recent,
            L::LocalCoreSafe,
            "aureline://evidence/svc-managed-control-plane-card",
            ScopeClass::ManagedOrg,
        ),
        // Fully local service: an outage elsewhere does not touch it.
        strip(
            4,
            "aureline://service-health/svc-local-index",
            "local_workspace_index",
            "local",
            "workspace_owner",
            "workspace_owner",
            "local_self_serve",
            "workspace_owner",
            A::Authoritative,
            escalation(
                "open_local_diagnostics",
                "Open local diagnostics",
                "aureline://diagnostics/local-index",
                "Open the local index diagnostics; no remote escalation is required.",
            ),
            S::Clear,
            F::Fresh,
            L::FullyLocal,
            "aureline://evidence/svc-local-index-card",
            ScopeClass::LocalPrivate,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn step(
    pane_id: &str,
    ordinal: u32,
    title: &str,
    object_ref: &str,
    intent: StepIntentClass,
    boundary: ActionBoundaryClass,
    target_scope: &str,
    dry_run_available: bool,
    approval_gate: ApprovalGateClass,
    approval_state: ApprovalStateClass,
    rollback_note: &str,
    boundary_state: OperatorStateClass,
    live_target_present: bool,
    evidence_ref: &str,
) -> RunbookStep {
    RunbookStep {
        step_id: format!("{pane_id}.step.{ordinal:02}"),
        ordinal,
        title: title.to_owned(),
        object_ref: object_ref.to_owned(),
        intent,
        boundary,
        target_scope: target_scope.to_owned(),
        dry_run_available,
        approval_gate,
        approval_state,
        rollback_note: rollback_note.to_owned(),
        boundary_state,
        live_target_present,
        execution: compute_step_execution(
            intent,
            boundary,
            approval_gate,
            approval_state,
            boundary_state,
            live_target_present,
        ),
        evidence_ref: evidence_ref.to_owned(),
        open_detail_ref: object_ref.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn pane(
    n: u32,
    object_ref: &str,
    title: &str,
    summary: &str,
    incident_ref: &str,
    owner: &str,
    decision_right: &str,
    scope: ScopeClass,
    default_redaction: RedactionClass,
    consumed_by: Vec<ConsumerClass>,
    live_vs_snapshot: LiveSnapshotClass,
    make_steps: impl FnOnce(&str) -> Vec<RunbookStep>,
) -> RunbookResponsePane {
    let pane_id = format!("response_pane.{n:04}");
    let steps = make_steps(&pane_id);
    RunbookResponsePane {
        pane_id: pane_id.clone(),
        object_ref: object_ref.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        surface: OperatorSurfaceClass::RunbookStepCard,
        surface_id: OperatorSurfaceClass::RunbookStepCard.surface_id(),
        incident_ref: incident_ref.to_owned(),
        owner: owner.to_owned(),
        decision_right: decision_right.to_owned(),
        scope,
        default_redaction,
        consumed_by,
        live_vs_snapshot,
        steps,
        open_detail_ref: object_ref.to_owned(),
    }
}

fn build_runbook_panes() -> Vec<RunbookResponsePane> {
    use ActionBoundaryClass as B;
    use ApprovalGateClass as G;
    use ApprovalStateClass as AS;
    use ConsumerClass::*;
    use OperatorStateClass as S;
    use StepIntentClass as I;

    let auth = pane(
        1,
        "aureline://runbook/rb-auth-latency",
        "Auth provider latency response",
        "Guided response for elevated auth-provider latency: observe, verify, then mitigate behind \
         preview and approval, with a browser handoff for the provider console.",
        "aureline://incident/inc-2048",
        "identity_oncall",
        "incident_commander",
        ScopeClass::SharedTeam,
        RedactionClass::OperatorOnlyRestricted,
        vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport, ManagedService],
        LiveSnapshotClass::SnapshotCapable,
        |pid| {
            vec![
                step(
                    pid,
                    1,
                    "Observe auth latency dashboard",
                    "aureline://runbook-step/rb-auth-latency-01",
                    I::Observe,
                    B::LocalOnly,
                    "Read the local auth-latency dashboard; no change.",
                    false,
                    G::None,
                    AS::NotRequired,
                    "",
                    S::Clear,
                    true,
                    "aureline://evidence/rb-auth-latency-01-metric",
                ),
                step(
                    pid,
                    2,
                    "Verify token-refresh path",
                    "aureline://runbook-step/rb-auth-latency-02",
                    I::Verify,
                    B::RemoteWorkspace,
                    "Run the read-only token-refresh probe against the remote workspace.",
                    false,
                    G::None,
                    AS::NotRequired,
                    "",
                    S::Clear,
                    true,
                    "aureline://evidence/rb-auth-latency-02-probe",
                ),
                step(
                    pid,
                    3,
                    "Raise auth connection pool ceiling",
                    "aureline://runbook-step/rb-auth-latency-03",
                    I::Mitigate,
                    B::ManagedControlPlane,
                    "Raise the managed connection-pool ceiling for the auth service.",
                    true,
                    G::SingleApproval,
                    AS::Granted,
                    "Restore the prior pool ceiling from the captured pre-change value.",
                    S::Clear,
                    true,
                    "aureline://evidence/rb-auth-latency-03-plan",
                ),
                step(
                    pid,
                    4,
                    "Shift auth traffic to standby region",
                    "aureline://runbook-step/rb-auth-latency-04",
                    I::Mitigate,
                    B::ManagedControlPlane,
                    "Shift a fraction of auth traffic to the standby region.",
                    true,
                    G::DualControl,
                    AS::Pending,
                    "Shift traffic back to the primary region and confirm health.",
                    S::Clear,
                    true,
                    "aureline://evidence/rb-auth-latency-04-plan",
                ),
                step(
                    pid,
                    5,
                    "Open provider status console",
                    "aureline://runbook-step/rb-auth-latency-05",
                    I::Communicate,
                    B::BrowserHandoff,
                    "Open the provider status console in the system browser to confirm provider-side \
                     state.",
                    false,
                    G::None,
                    AS::NotRequired,
                    "",
                    S::EmbeddedBoundaryHandoff,
                    true,
                    "aureline://evidence/rb-auth-latency-05-route",
                ),
                step(
                    pid,
                    6,
                    "Roll back pool ceiling change",
                    "aureline://runbook-step/rb-auth-latency-06",
                    I::Rollback,
                    B::ManagedControlPlane,
                    "Revert the connection-pool ceiling to its pre-change value.",
                    true,
                    G::SingleApproval,
                    AS::Granted,
                    "This step is itself the rollback; re-apply the change only via step 3.",
                    S::Clear,
                    true,
                    "aureline://evidence/rb-auth-latency-06-plan",
                ),
            ]
        },
    );

    let control_plane = pane(
        2,
        "aureline://runbook/rb-control-plane-failover",
        "Managed control-plane failover response",
        "Guided response while the managed control plane is failing over: local observation stays \
         available, but managed mutations are blocked behind the boundary until the failover \
         settles.",
        "aureline://incident/inc-2050",
        "sre_oncall",
        "incident_commander",
        ScopeClass::ManagedOrg,
        RedactionClass::OperatorOnlyRestricted,
        vec![
            ShellUi,
            CliHeadless,
            IncidentWorkspace,
            SupportExport,
            ManagedService,
        ],
        LiveSnapshotClass::SnapshotCapable,
        |pid| {
            vec![
                step(
                    pid,
                    1,
                    "Observe control-plane health",
                    "aureline://runbook-step/rb-control-plane-01",
                    I::Observe,
                    B::LocalOnly,
                    "Read the cached control-plane health card; no change.",
                    false,
                    G::None,
                    AS::NotRequired,
                    "",
                    S::FailoverInProgress,
                    true,
                    "aureline://evidence/rb-control-plane-01-card",
                ),
                step(
                    pid,
                    2,
                    "Re-pin managed endpoint",
                    "aureline://runbook-step/rb-control-plane-02",
                    I::Mitigate,
                    B::ManagedControlPlane,
                    "Re-pin the managed endpoint to the surviving control-plane replica.",
                    true,
                    G::SingleApproval,
                    AS::Granted,
                    "Re-pin to the prior endpoint captured before the failover.",
                    S::FailoverInProgress,
                    true,
                    "aureline://evidence/rb-control-plane-02-plan",
                ),
                step(
                    pid,
                    3,
                    "Verify local queue is preserved",
                    "aureline://runbook-step/rb-control-plane-03",
                    I::Verify,
                    B::LocalOnly,
                    "Verify the publish-later queue retained the deferred writes.",
                    false,
                    G::None,
                    AS::NotRequired,
                    "",
                    S::Clear,
                    true,
                    "aureline://evidence/rb-control-plane-03-queue",
                ),
                step(
                    pid,
                    4,
                    "Roll back endpoint re-pin",
                    "aureline://runbook-step/rb-control-plane-04",
                    I::Rollback,
                    B::ManagedControlPlane,
                    "Revert the endpoint re-pin once the primary control plane is healthy.",
                    true,
                    G::DualControl,
                    AS::Pending,
                    "This step is the rollback; re-pin only via step 2 with fresh approval.",
                    S::BoundaryDriftRecheckRequired,
                    true,
                    "aureline://evidence/rb-control-plane-04-plan",
                ),
            ]
        },
    );

    // An imported replay pane: no live target, so every step is read-only.
    let imported = pane(
        3,
        "aureline://runbook/rb-imported-replay",
        "Imported incident replay (no live target)",
        "An imported runbook replay attached to a snapshot incident: every step is read-only \
         because there is no live system to act on.",
        "aureline://incident/inc-archive-1990",
        "support_reviewer",
        "support_lead",
        ScopeClass::SharedTeam,
        RedactionClass::InternalSupportRestricted,
        vec![ShellUi, CliHeadless, SupportExport, IncidentWorkspace],
        LiveSnapshotClass::SnapshotOnly,
        |pid| {
            vec![
                step(
                    pid,
                    1,
                    "Review imported observe step",
                    "aureline://runbook-step/rb-imported-01",
                    I::Observe,
                    B::LocalOnly,
                    "Read the imported observation; replay only.",
                    false,
                    G::None,
                    AS::NotRequired,
                    "",
                    S::ImportedSnapshotNoLive,
                    false,
                    "aureline://evidence/rb-imported-01-metric",
                ),
                step(
                    pid,
                    2,
                    "Review imported mitigation step",
                    "aureline://runbook-step/rb-imported-02",
                    I::Mitigate,
                    B::ManagedControlPlane,
                    "Read the imported mitigation; no live target exists to apply against.",
                    true,
                    G::SingleApproval,
                    AS::Granted,
                    "Recorded rollback note preserved from the original runbook.",
                    S::ImportedSnapshotNoLive,
                    false,
                    "aureline://evidence/rb-imported-02-plan",
                ),
            ]
        },
    );

    vec![auth, control_plane, imported]
}

#[allow(clippy::too_many_arguments)]
fn continuity_view(
    n: u32,
    object_ref: &str,
    title: &str,
    summary: &str,
    kind: ContinuityKindClass,
    owner: &str,
    displayed_state: OperatorStateClass,
    freshness: FreshnessClass,
    failed_boundary: FailedBoundaryClass,
    local_capabilities: &[LocalCapabilityClass],
    blocked_capabilities: &[&str],
    next_safe_action: NextSafeActionClass,
    publish_later_capture: bool,
    scope: ScopeClass,
) -> ContinuityView {
    ContinuityView {
        view_id: format!("continuity_view.{n:04}"),
        object_ref: object_ref.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        kind,
        surface: kind.surface(),
        surface_id: kind.surface().surface_id(),
        owner: owner.to_owned(),
        displayed_state,
        freshness,
        effective_state: compute_effective_state(
            displayed_state,
            freshness,
            BlockerWaiverClass::None,
        ),
        failed_boundary,
        local_capabilities: local_capabilities.to_vec(),
        blocked_capabilities: blocked_capabilities
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        next_safe_action,
        publish_later_capture,
        scope,
        open_detail_ref: object_ref.to_owned(),
    }
}

fn build_continuity_views() -> Vec<ContinuityView> {
    use ContinuityKindClass as K;
    use FailedBoundaryClass as FB;
    use FreshnessClass as F;
    use LocalCapabilityClass as C;
    use NextSafeActionClass as N;
    use OperatorStateClass as S;

    vec![
        continuity_view(
            1,
            "aureline://continuity/maint-read-only-0007",
            "Planned read-only maintenance",
            "A planned read-only maintenance window blocks managed writes; local editing, saving, \
             search, build/test, and export stay available and writes queue to publish later.",
            K::ReadOnlyWindow,
            "platform_lead",
            S::ReadOnlyWindow,
            F::Fresh,
            FB::None,
            &[
                C::Edit,
                C::Save,
                C::Search,
                C::GitVersioning,
                C::BuildTest,
                C::ExportDiagnostics,
                C::InspectEvidence,
                C::PublishLater,
            ],
            &["managed_writes", "managed_settings_apply"],
            N::PublishLater,
            true,
            ScopeClass::ManagedOrg,
        ),
        continuity_view(
            2,
            "aureline://continuity/failover-region-0012",
            "Regional failover in progress",
            "A region is failing over; managed authority-changing actions are blocked until the new \
             boundary is reviewed, but local work and export continue.",
            K::RegionalFailover,
            "sre_lead",
            S::FailoverInProgress,
            F::Recent,
            FB::Region,
            &[
                C::Edit,
                C::Save,
                C::Search,
                C::InspectEvidence,
                C::ExportDiagnostics,
                C::PublishLater,
            ],
            &["managed_writes", "authority_changes"],
            N::ReviewNewBoundary,
            true,
            ScopeClass::ManagedOrg,
        ),
        continuity_view(
            3,
            "aureline://continuity/outage-provider-0030",
            "Provider outage — local work continues",
            "A provider endpoint is impaired; provider-backed calls are blocked, but the local core \
             stays fully usable and queued work retries when the provider is restored.",
            K::ProviderOutage,
            "identity_lead",
            S::Blocked,
            F::Fresh,
            FB::ProviderEndpoint,
            &[
                C::Edit,
                C::Save,
                C::Search,
                C::GitVersioning,
                C::BuildTest,
                C::ExportDiagnostics,
                C::InspectEvidence,
                C::OpenLocalHistory,
            ],
            &["provider_calls"],
            N::RetryWhenRestored,
            true,
            ScopeClass::SharedTeam,
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> PaneInvariant {
    PaneInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    strips: &[ServiceOwnershipStrip],
    panes: &[RunbookResponsePane],
    views: &[ContinuityView],
) -> Vec<PaneInvariant> {
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    let bind = |surface: OperatorSurfaceClass, surface_id: &str| -> bool {
        surface_id == surface.surface_id() && matrix.surface(surface).is_some()
    };
    let mut out = Vec::new();

    // Every surface binds a real matrix family by the matrix's own surface id.
    out.push(invariant(
        "response_panes.surface_binding",
        "Every strip, pane, and continuity view binds a surface family that exists in the \
         operator-surface matrix, by the matrix's own surface id, rather than cloning a per-surface \
         model.",
        strips
            .iter()
            .all(|s| s.surface == OperatorSurfaceClass::ServiceOwnershipStrip && bind(s.surface, &s.surface_id))
            && panes
                .iter()
                .all(|p| p.surface == OperatorSurfaceClass::RunbookStepCard && bind(p.surface, &p.surface_id))
            && views
                .iter()
                .all(|v| v.surface == v.kind.surface() && bind(v.surface, &v.surface_id)),
    ));

    // Every surface points at a canonical object and routes open-detail to it.
    out.push(invariant(
        "response_panes.canonical_object_identity",
        "Every strip, pane, step, and continuity view carries a canonical aureline:// object handle \
         and routes open-detail to that exact handle.",
        strips
            .iter()
            .all(|s| s.object_ref.starts_with("aureline://") && s.open_detail_ref == s.object_ref)
            && panes.iter().all(|p| {
                p.object_ref.starts_with("aureline://")
                    && p.open_detail_ref == p.object_ref
                    && p.incident_ref.starts_with("aureline://")
                    && p.steps.iter().all(|s| {
                        s.object_ref.starts_with("aureline://") && s.open_detail_ref == s.object_ref
                    })
            })
            && views
                .iter()
                .all(|v| v.object_ref.starts_with("aureline://") && v.open_detail_ref == v.object_ref),
    ));

    // Service ownership and on-call authority stay visible.
    out.push(invariant(
        "response_panes.service_owner_oncall_visible",
        "Every service strip names a primary owner, an active on-call lane, a decision right, and an \
         escalation action, so service ownership and on-call authority are never hidden in side \
         docs.",
        strips.iter().all(|s| {
            !s.primary_owner.is_empty()
                && !s.on_call_lane.is_empty()
                && !s.decision_right.is_empty()
                && !s.escalation.routes_to_ref.is_empty()
                && !s.escalation.label.is_empty()
        }),
    ));

    // Advisory-versus-authoritative source is always declared.
    out.push(invariant(
        "response_panes.authority_source_visible",
        "Every service strip declares whether its state is authoritative or advisory.",
        strips
            .iter()
            .all(|s| AuthoritySourceClass::ALL.contains(&s.authority_source)),
    ));

    // No silent green on strips and continuity views.
    out.push(invariant(
        "response_panes.no_silent_green",
        "Every strip and continuity view's effective state equals the computed no-silent-green \
         state, so a stale strip or view is never reported clear.",
        strips.iter().all(|s| {
            s.effective_state
                == compute_effective_state(s.displayed_state, s.freshness, BlockerWaiverClass::None)
        }) && views.iter().all(|v| {
            v.effective_state
                == compute_effective_state(v.displayed_state, v.freshness, BlockerWaiverClass::None)
        }),
    ));

    // Every strip states what stays usable locally.
    out.push(invariant(
        "response_panes.local_continuity_explicit",
        "Every service strip declares a local-continuity posture, so an operator can answer what \
         still works locally if the service is impaired.",
        strips
            .iter()
            .all(|s| LocalContinuityClass::ALL.contains(&s.local_continuity)),
    ));

    // Steps are contiguously ordered.
    out.push(invariant(
        "response_panes.steps_ordered",
        "Every pane's steps are contiguously ordered from 1, so the guided response has a defined \
         sequence.",
        panes.iter().all(|p| {
            p.steps
                .iter()
                .enumerate()
                .all(|(idx, s)| s.ordinal as usize == idx + 1)
        }),
    ));

    // Step admission is the computed value.
    out.push(invariant(
        "response_panes.execution_computed",
        "Every step's execution admission equals the computed admission from its intent, boundary, \
         approval, boundary state, and live-target presence.",
        panes.iter().all(|p| {
            p.steps.iter().all(|s| {
                s.execution
                    == compute_step_execution(
                        s.intent,
                        s.boundary,
                        s.approval_gate,
                        s.approval_state,
                        s.boundary_state,
                        s.live_target_present,
                    )
            })
        }),
    ));

    // No silent mutation: a mutating step is never admitted to run locally.
    out.push(invariant(
        "response_panes.mutating_steps_gated",
        "Every mutating step (mitigate or rollback) previews before applying, blocks awaiting \
         approval, blocks behind a boundary, hands off to a browser, or is read-only on imported \
         evidence — it is never silently run locally.",
        panes.iter().all(|p| {
            p.steps
                .iter()
                .filter(|s| s.intent.is_mutating())
                .all(|s| s.execution != StepExecutionClass::RunLocal)
        }),
    ));

    // Mutating steps carry a dry-run path and a rollback note.
    out.push(invariant(
        "response_panes.mutating_steps_previewable",
        "Every mutating step offers a dry-run / preview path and carries a rollback note, so a \
         live mutation can always be previewed and undone.",
        panes.iter().all(|p| {
            p.steps
                .iter()
                .filter(|s| s.intent.is_mutating())
                .all(|s| s.dry_run_available && !s.rollback_note.is_empty())
        }),
    ));

    // Read-only steps never block on approval.
    out.push(invariant(
        "response_panes.read_only_steps_unblocked",
        "Observe, verify, and communicate steps never carry an approval gate and never block \
         awaiting approval.",
        panes.iter().all(|p| {
            p.steps.iter().filter(|s| !s.intent.is_mutating()).all(|s| {
                s.approval_gate == ApprovalGateClass::None
                    && s.approval_state == ApprovalStateClass::NotRequired
                    && s.execution != StepExecutionClass::BlockedAwaitingApproval
            })
        }),
    ));

    // Continuity views are explicit about local capability, boundary, and action.
    out.push(invariant(
        "response_panes.continuity_explicit",
        "Every continuity view lists at least one local capability, names which boundary failed, \
         and recommends a next safe action — so an outage never reads as a total product outage.",
        views.iter().all(|v| {
            !v.local_capabilities.is_empty()
                && FailedBoundaryClass::ALL.contains(&v.failed_boundary)
                && NextSafeActionClass::ALL.contains(&v.next_safe_action)
        }),
    ));

    // Publish-later capture whenever managed writes are blocked.
    out.push(invariant(
        "response_panes.publish_later_when_blocked",
        "Every continuity view whose effective state blocks managed writes offers publish-later \
         capture, so blocked writes are queued rather than lost.",
        views
            .iter()
            .all(|v| !v.blocks_managed_writes() || v.publish_later_capture),
    ));

    // Stable ids unique.
    out.push(invariant(
        "response_panes.stable_ids_unique",
        "Strip ids, pane ids, step ids, and continuity-view ids are each defined once and unique \
         across the set.",
        all_unique(strips.iter().map(|s| s.strip_id.as_str()))
            && all_unique(panes.iter().map(|p| p.pane_id.as_str()))
            && all_unique(views.iter().map(|v| v.view_id.as_str()))
            && all_unique(
                panes
                    .iter()
                    .flat_map(|p| p.steps.iter().map(|s| s.step_id.as_str())),
            ),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the response-pane set as human-readable lines for CLI/headless and
/// support.
pub fn response_pane_lines(set: &ResponsePaneSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator response panes — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Strips: {}  Panes: {}  Continuity views: {}  bound to {}",
        set.service_strips.len(),
        set.runbook_panes.len(),
        set.continuity_views.len(),
        set.matrix_ref,
    ));

    lines.push("Service ownership / on-call strips:".to_owned());
    for s in &set.service_strips {
        lines.push(format!(
            "  - {} [{}] env={} owner={} backup={} on_call={} source={} -> {}",
            s.service_family,
            s.object_ref,
            s.environment,
            s.primary_owner,
            s.backup_owner,
            s.on_call_lane,
            s.authority_source.as_str(),
            s.effective_state.as_str(),
        ));
        lines.push(format!(
            "      displayed={} freshness={} local_continuity={} escalation={}",
            s.displayed_state.as_str(),
            s.freshness.as_str(),
            s.local_continuity.as_str(),
            s.escalation.token,
        ));
    }

    lines.push("Runbook-guided response panes:".to_owned());
    for p in &set.runbook_panes {
        lines.push(format!(
            "  Pane {} [{}] incident={} owner={} scope={} live={:?}",
            p.title,
            p.pane_id,
            p.incident_ref,
            p.owner,
            scope_token(p.scope),
            p.live_vs_snapshot,
        ));
        for st in &p.steps {
            lines.push(format!(
                "    {}. {} intent={} boundary={} approval={}/{} -> {}",
                st.ordinal,
                st.title,
                st.intent.as_str(),
                st.boundary.as_str(),
                st.approval_gate.as_str(),
                st.approval_state.as_str(),
                st.execution.as_str(),
            ));
        }
    }

    lines.push("Local-outage continuity views:".to_owned());
    for v in &set.continuity_views {
        let caps: Vec<&str> = v.local_capabilities.iter().map(|c| c.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] kind={} failed_boundary={} -> {} next={}",
            v.title,
            v.object_ref,
            v.kind.as_str(),
            v.failed_boundary.as_str(),
            v.effective_state.as_str(),
            v.next_safe_action.as_str(),
        ));
        lines.push(format!(
            "      local-safe: {} (publish-later: {})",
            caps.join(", "),
            v.publish_later_capture
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
