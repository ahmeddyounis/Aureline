//! M5 maintenance / failover / reconciliation windows: planned operational
//! windows with exact times, blocked write classes, and local-safe / publish-later
//! continuity, bound to the frozen operator-surface matrix.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the
//! *families* of operator surface — including the maintenance / read-only / drain
//! notice and the failover / migration notice — the one shared state vocabulary,
//! and the invariants every surface must hold. The
//! [response panes](crate::m5_response_panes) added the first *response* surfaces.
//! This lane builds the first real **planned-operation** surfaces: the windows
//! that make scheduled maintenance, read-only windows, drain phases, failover,
//! migration, and post-window reconciliation feel *different* from random
//! breakage by naming exact scope, time, and the next safe action before a user
//! crosses the boundary.
//!
//! Each [`MaintenanceWindow`] pins five things that a generic outage banner never
//! does:
//!
//! 1. **An exact operational phase and exact times.** A window declares its
//!    [`OperationalPhaseClass`] (scheduled, read-only, drain, migration, failover,
//!    reconciling, or resolved), an exact start and end timestamp *with* an IANA
//!    time zone and UTC offset, and the latest-refresh stamp and freshness behind
//!    the claim — never a vague "we'll be back soon". The phase maps to the matrix
//!    [`OperatorStateClass`], and [`compute_effective_state`] downgrades a
//!    resolved-but-unconfirmed window so a stale "all clear" never reads as green.
//! 2. **Exactly which write classes are blocked.** A window in effect lists the
//!    [`BlockedWriteClass`]es it blocks and, for each, the local-safe alternative,
//!    so an operator can see what is refused rather than discovering it on apply.
//! 3. **What stays safely local and how writes are preserved.** A window names its
//!    local-safe actions and its [`WritePostureClass`] — writes-live, local-draft,
//!    publish-later-queued, blocked-pending-recheck, or read-only — so blocked
//!    managed writes are captured and replayed, never lost.
//! 4. **Changed boundary truth.** A failover or migration that changes the tenant,
//!    region, residency, key ownership, or endpoint posture restates that
//!    [`BoundaryAxisClass`] explicitly in its [`BoundaryDisclosure`] instead of
//!    implying an unchanged route.
//! 5. **Whether queued work must be reviewed before replay.** When queued actions
//!    would cross a changed policy, tenant, endpoint, or authority boundary after
//!    the window ends, [`compute_replay_review_required`] forces a
//!    [`ReplayReview`] so the queue is reconciled, not silently replayed against a
//!    moved boundary.
//!
//! [`maintenance_window_set`] is the canonical binding: it builds the windows
//! deterministically and computes each [`WindowInvariant`]'s `holds` flag, the
//! per-window effective state, and the replay-review requirement from the built
//! data, so the checked-in fixture and the replay gate freeze the contract
//! byte-for-byte and an inconsistent edit flips an invariant rather than silently
//! passing. The record carries no endpoint URLs, hostnames, credentials, raw
//! payloads, or absolute paths — only opaque object refs, stable tokens, and short
//! reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::m5_operator_boards::{compute_effective_state, BlockerWaiverClass, FreshnessClass};
use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorStateClass, OperatorSurfaceClass, RedactionClass,
    ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the maintenance-window set.
pub const M5_MAINTENANCE_WINDOWS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the maintenance-window set.
pub const M5_MAINTENANCE_WINDOWS_SCHEMA_REF: &str =
    "schemas/ops/m5-maintenance-windows.schema.json";

/// Stable record-kind tag for the maintenance-window set.
pub const M5_MAINTENANCE_WINDOWS_RECORD_KIND: &str = "m5_maintenance_window_set";

/// Stable id for the canonical maintenance-window set.
pub const M5_MAINTENANCE_WINDOWS_SET_ID: &str = "m5-maintenance-windows:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_MAINTENANCE_WINDOWS_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this set binds for surface identity.
pub const M5_MAINTENANCE_WINDOWS_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this set binds.
pub const M5_MAINTENANCE_WINDOWS_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Operational phase.
// ---------------------------------------------------------------------------

/// The operational-state of a planned window, in lifecycle order.
///
/// These are the operational-state objects the lane freezes: a window is in
/// exactly one phase at a time, and each phase maps to a matrix
/// [`OperatorStateClass`] so dashboards, notices, and support exports share one
/// state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalPhaseClass {
    /// Announced and scheduled, but not yet in effect.
    Scheduled,
    /// A read-only window is in effect: managed writes are blocked, local work
    /// continues, and publish-later capture is offered.
    ReadOnly,
    /// A drain window is in effect: in-flight work finishes, new actions queue.
    Drain,
    /// A tenant / region / residency migration is in progress.
    Migration,
    /// A failover is in progress.
    Failover,
    /// Reconciling after the window or event before normal operation resumes.
    Reconciling,
    /// The window ended and normal operation has resumed.
    Resolved,
}

impl OperationalPhaseClass {
    /// All phases, in lifecycle order.
    pub const ALL: [Self; 7] = [
        Self::Scheduled,
        Self::ReadOnly,
        Self::Drain,
        Self::Migration,
        Self::Failover,
        Self::Reconciling,
        Self::Resolved,
    ];

    /// Stable snake_case token for this phase.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scheduled => "scheduled",
            Self::ReadOnly => "read_only",
            Self::Drain => "drain",
            Self::Migration => "migration",
            Self::Failover => "failover",
            Self::Reconciling => "reconciling",
            Self::Resolved => "resolved",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Scheduled => "Scheduled",
            Self::ReadOnly => "Read-only window",
            Self::Drain => "Drain window",
            Self::Migration => "Migration in progress",
            Self::Failover => "Failover in progress",
            Self::Reconciling => "Reconciling",
            Self::Resolved => "Resolved",
        }
    }

    /// The matrix state vocabulary token this phase maps to.
    pub const fn matrix_state(self) -> OperatorStateClass {
        match self {
            Self::Scheduled => OperatorStateClass::ScheduledWindow,
            Self::ReadOnly => OperatorStateClass::ReadOnlyWindow,
            Self::Drain => OperatorStateClass::DrainWindow,
            Self::Migration => OperatorStateClass::MigrationInProgress,
            Self::Failover => OperatorStateClass::FailoverInProgress,
            Self::Reconciling => OperatorStateClass::Reconciling,
            Self::Resolved => OperatorStateClass::Clear,
        }
    }

    /// Whether new managed / side-effectful writes are blocked while in this phase.
    pub const fn blocks_managed_writes(self) -> bool {
        matches!(
            self,
            Self::ReadOnly | Self::Drain | Self::Migration | Self::Failover | Self::Reconciling
        )
    }
}

// ---------------------------------------------------------------------------
// Window kind.
// ---------------------------------------------------------------------------

/// The kind of operation a window describes; this selects the matrix surface
/// family the window renders on and the phases that are valid for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowKindClass {
    /// A planned maintenance operation (scheduled, read-only, or drain).
    PlannedMaintenance,
    /// A regional failover.
    RegionalFailover,
    /// A tenant / region / residency migration.
    TenantMigration,
}

impl WindowKindClass {
    /// All kinds, in set order.
    pub const ALL: [Self; 3] = [
        Self::PlannedMaintenance,
        Self::RegionalFailover,
        Self::TenantMigration,
    ];

    /// Stable snake_case token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlannedMaintenance => "planned_maintenance",
            Self::RegionalFailover => "regional_failover",
            Self::TenantMigration => "tenant_migration",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PlannedMaintenance => "Planned maintenance",
            Self::RegionalFailover => "Regional failover",
            Self::TenantMigration => "Tenant migration",
        }
    }

    /// The operator-surface matrix family this kind renders on: planned
    /// maintenance binds the maintenance notice; failover and migration bind the
    /// failover notice.
    pub const fn surface(self) -> OperatorSurfaceClass {
        match self {
            Self::PlannedMaintenance => OperatorSurfaceClass::MaintenanceNotice,
            Self::RegionalFailover | Self::TenantMigration => OperatorSurfaceClass::FailoverNotice,
        }
    }

    /// Whether the phase is valid for this kind of window.
    pub fn permits_phase(self, phase: OperationalPhaseClass) -> bool {
        use OperationalPhaseClass::*;
        match self {
            Self::PlannedMaintenance => {
                matches!(phase, Scheduled | ReadOnly | Drain | Reconciling | Resolved)
            }
            Self::RegionalFailover => {
                matches!(phase, Scheduled | Failover | Reconciling | Resolved)
            }
            Self::TenantMigration => {
                matches!(
                    phase,
                    Scheduled | Drain | Migration | Reconciling | Resolved
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Write posture.
// ---------------------------------------------------------------------------

/// The write posture a window admits while in effect; mirrors the matrix path
/// write posture but is stated per window so an operator can see how blocked
/// writes are preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritePostureClass {
    /// Connected: managed writes run live (still subject to per-action approval).
    WritesLive,
    /// Writes are preserved as a local draft only.
    LocalDraftPreserved,
    /// Writes are captured and queued to publish when the window ends.
    PublishLaterQueued,
    /// Writes are blocked pending a boundary recheck after the window ends.
    BlockedPendingBoundaryRecheck,
    /// Read-only replay of imported evidence; no writes admitted.
    ReadOnlyReplay,
}

impl WritePostureClass {
    /// All postures, in set order.
    pub const ALL: [Self; 5] = [
        Self::WritesLive,
        Self::LocalDraftPreserved,
        Self::PublishLaterQueued,
        Self::BlockedPendingBoundaryRecheck,
        Self::ReadOnlyReplay,
    ];

    /// Stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WritesLive => "writes_live",
            Self::LocalDraftPreserved => "local_draft_preserved",
            Self::PublishLaterQueued => "publish_later_queued",
            Self::BlockedPendingBoundaryRecheck => "blocked_pending_boundary_recheck",
            Self::ReadOnlyReplay => "read_only_replay",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WritesLive => "Writes live",
            Self::LocalDraftPreserved => "Local draft preserved",
            Self::PublishLaterQueued => "Publish later (queued)",
            Self::BlockedPendingBoundaryRecheck => "Blocked pending boundary recheck",
            Self::ReadOnlyReplay => "Read-only replay",
        }
    }

    /// Whether this posture blocks live managed writes (anything but
    /// [`WritePostureClass::WritesLive`]).
    pub const fn blocks_live_writes(self) -> bool {
        !matches!(self, Self::WritesLive)
    }
}

// ---------------------------------------------------------------------------
// Blocked write classes.
// ---------------------------------------------------------------------------

/// A class of write a window can block while in effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedWriteClass {
    /// Applying a managed setting / configuration change.
    ManagedSettingsApply,
    /// Changing a managed policy.
    ManagedPolicyChange,
    /// A side-effectful provider / endpoint mutation.
    ProviderMutation,
    /// A write to a remote workspace.
    RemoteWorkspaceWrite,
    /// An authority-changing action (tenant, region, residency, or key posture).
    AuthorityChange,
    /// Publishing a ticket / incident / support update to the provider.
    TicketOrIncidentPublish,
    /// Publishing a release.
    ReleasePublish,
}

impl BlockedWriteClass {
    /// All classes, in set order.
    pub const ALL: [Self; 7] = [
        Self::ManagedSettingsApply,
        Self::ManagedPolicyChange,
        Self::ProviderMutation,
        Self::RemoteWorkspaceWrite,
        Self::AuthorityChange,
        Self::TicketOrIncidentPublish,
        Self::ReleasePublish,
    ];

    /// Stable snake_case token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedSettingsApply => "managed_settings_apply",
            Self::ManagedPolicyChange => "managed_policy_change",
            Self::ProviderMutation => "provider_mutation",
            Self::RemoteWorkspaceWrite => "remote_workspace_write",
            Self::AuthorityChange => "authority_change",
            Self::TicketOrIncidentPublish => "ticket_or_incident_publish",
            Self::ReleasePublish => "release_publish",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ManagedSettingsApply => "Managed settings apply",
            Self::ManagedPolicyChange => "Managed policy change",
            Self::ProviderMutation => "Provider mutation",
            Self::RemoteWorkspaceWrite => "Remote workspace write",
            Self::AuthorityChange => "Authority change",
            Self::TicketOrIncidentPublish => "Ticket / incident publish",
            Self::ReleasePublish => "Release publish",
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary axes.
// ---------------------------------------------------------------------------

/// A boundary axis a failover or migration can move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAxisClass {
    /// The serving tenant.
    Tenant,
    /// The serving region.
    Region,
    /// The data-residency boundary.
    Residency,
    /// Key ownership / signing authority.
    KeyOwnership,
    /// The endpoint identity / route target.
    EndpointIdentity,
}

impl BoundaryAxisClass {
    /// All axes, in set order.
    pub const ALL: [Self; 5] = [
        Self::Tenant,
        Self::Region,
        Self::Residency,
        Self::KeyOwnership,
        Self::EndpointIdentity,
    ];

    /// Stable snake_case token for this axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tenant => "tenant",
            Self::Region => "region",
            Self::Residency => "residency",
            Self::KeyOwnership => "key_ownership",
            Self::EndpointIdentity => "endpoint_identity",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tenant => "Tenant",
            Self::Region => "Region",
            Self::Residency => "Residency",
            Self::KeyOwnership => "Key ownership",
            Self::EndpointIdentity => "Endpoint identity",
        }
    }
}

/// The state of a boundary axis across a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAxisStateClass {
    /// The axis is unchanged across the window.
    Unchanged,
    /// The axis changed and must be restated explicitly.
    Changed,
    /// The axis state is unknown and requires a recheck before managed writes
    /// resume.
    Unknown,
}

impl BoundaryAxisStateClass {
    /// All states, in set order.
    pub const ALL: [Self; 3] = [Self::Unchanged, Self::Changed, Self::Unknown];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Changed => "changed",
            Self::Unknown => "unknown",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unchanged => "Unchanged",
            Self::Changed => "Changed",
            Self::Unknown => "Unknown — recheck required",
        }
    }

    /// Whether this state means the axis moved or is unknown (a crossing that
    /// requires recheck).
    pub const fn crossed(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

// ---------------------------------------------------------------------------
// Replay-review trigger.
// ---------------------------------------------------------------------------

/// Why queued actions must be reviewed before replay after a window ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayReviewTriggerClass {
    /// No review is needed: nothing queued crosses a changed boundary.
    NoReviewNeeded,
    /// A policy boundary changed.
    ChangedPolicy,
    /// The tenant changed.
    ChangedTenant,
    /// The serving region changed.
    ChangedRegion,
    /// The data-residency boundary changed.
    ChangedResidency,
    /// The endpoint identity / route target changed.
    ChangedEndpoint,
    /// An authority / key-ownership boundary changed.
    ChangedAuthority,
}

impl ReplayReviewTriggerClass {
    /// All triggers, in set order.
    pub const ALL: [Self; 7] = [
        Self::NoReviewNeeded,
        Self::ChangedPolicy,
        Self::ChangedTenant,
        Self::ChangedRegion,
        Self::ChangedResidency,
        Self::ChangedEndpoint,
        Self::ChangedAuthority,
    ];

    /// Stable snake_case token for this trigger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReviewNeeded => "no_review_needed",
            Self::ChangedPolicy => "changed_policy",
            Self::ChangedTenant => "changed_tenant",
            Self::ChangedRegion => "changed_region",
            Self::ChangedResidency => "changed_residency",
            Self::ChangedEndpoint => "changed_endpoint",
            Self::ChangedAuthority => "changed_authority",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoReviewNeeded => "No review needed",
            Self::ChangedPolicy => "Policy changed",
            Self::ChangedTenant => "Tenant changed",
            Self::ChangedRegion => "Region changed",
            Self::ChangedResidency => "Residency changed",
            Self::ChangedEndpoint => "Endpoint changed",
            Self::ChangedAuthority => "Authority changed",
        }
    }

    /// Whether the trigger actually requires a review (anything but
    /// [`ReplayReviewTriggerClass::NoReviewNeeded`]).
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::NoReviewNeeded)
    }
}

/// Computes whether queued actions must be reviewed before replay after a window
/// ends.
///
/// This is the review-before-replay rule made executable: a review is required
/// only when there are queued actions *and* the window crossed a changed or
/// unknown boundary, so a queue replayed against an unchanged route is not gated
/// while a queue replayed against a moved tenant / region / endpoint / authority
/// is.
pub const fn compute_replay_review_required(
    queued_actions_present: bool,
    boundary_crossed: bool,
) -> bool {
    queued_actions_present && boundary_crossed
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// The exact timing of a window: start, end, time zone, and latest-refresh state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowTime {
    /// Exact start timestamp (RFC3339, with an explicit UTC offset).
    pub starts_at: String,
    /// Exact end timestamp (RFC3339, with an explicit UTC offset).
    pub ends_at: String,
    /// IANA time-zone name the window is announced in (for example
    /// `America/New_York`).
    pub time_zone: String,
    /// The UTC offset the timestamps carry (for example `-04:00`, or `+00:00`).
    pub utc_offset: String,
    /// Whether the end timestamp is an estimate rather than a committed time.
    pub end_is_estimated: bool,
    /// The latest moment this window's state was refreshed (RFC3339).
    pub latest_refresh_at: String,
    /// The freshness of the latest refresh.
    pub refresh_freshness: FreshnessClass,
}

/// One blocked write class with its local-safe alternative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedWrite {
    /// The blocked write class.
    pub class: BlockedWriteClass,
    /// Human-readable label.
    pub label: String,
    /// The local-safe alternative that keeps the operator's work (for example a
    /// local draft, or a publish-later queue).
    pub local_alternative: String,
}

/// One boundary axis and how it moved across a window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryAxisDisclosure {
    /// The boundary axis.
    pub axis: BoundaryAxisClass,
    /// The state of the axis across the window.
    pub state: BoundaryAxisStateClass,
    /// One reviewable sentence restating the boundary truth (no raw hostnames,
    /// URLs, or credentials).
    pub disclosure: String,
}

/// The boundary disclosure for a window, with the computed recheck requirement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryDisclosure {
    /// The per-axis disclosures.
    pub axes: Vec<BoundaryAxisDisclosure>,
    /// Whether any axis changed or is unknown and a recheck is therefore required
    /// before managed writes resume.
    pub recheck_required: bool,
}

impl BoundaryDisclosure {
    /// Whether any axis changed or is unknown.
    pub fn any_crossed(&self) -> bool {
        self.axes.iter().any(|a| a.state.crossed())
    }
}

/// The review-before-replay posture for a window's queued actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayReview {
    /// Whether queued actions must be reviewed before replay or reconcile.
    pub required: bool,
    /// Why the review is required (or that none is needed).
    pub trigger: ReplayReviewTriggerClass,
    /// One reviewable sentence naming the reconcile action, when a review is
    /// required; empty otherwise.
    pub reconcile_action: String,
}

/// One planned operational window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// Stable window id.
    pub window_id: String,
    /// The canonical object handle this window is about.
    pub object_ref: String,
    /// Short title.
    pub title: String,
    /// One reviewable sentence describing the window.
    pub summary: String,
    /// The operation kind.
    pub kind: WindowKindClass,
    /// The bound matrix surface family (maintenance notice or failover notice).
    pub surface: OperatorSurfaceClass,
    /// The bound matrix surface id.
    pub surface_id: String,
    /// The current operational phase.
    pub phase: OperationalPhaseClass,
    /// The computed effective state — the no-silent-green downgrade of the phase's
    /// matrix state and the latest-refresh freshness.
    pub effective_state: OperatorStateClass,
    /// The maintenance / failover owner.
    pub owner: String,
    /// Who holds the decision right for this window.
    pub decision_right: String,
    /// Local-versus-shared scope of the underlying object.
    pub scope: ScopeClass,
    /// The default redaction posture on export.
    pub default_redaction: RedactionClass,
    /// The consumers that render this window.
    pub consumed_by: Vec<ConsumerClass>,
    /// Live-versus-snapshot posture.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// The exact timing of the window.
    pub window_time: WindowTime,
    /// The write posture while the window is in effect.
    pub write_posture: WritePostureClass,
    /// The write classes blocked while the window is in effect.
    pub blocked_writes: Vec<BlockedWrite>,
    /// The local-safe actions that stay available during the window.
    pub local_safe_actions: Vec<String>,
    /// Whether publish-later / draft capture is offered while writes are blocked.
    pub publish_later_available: bool,
    /// The recommended next safe action token.
    pub next_safe_action: String,
    /// The changed-boundary disclosure (empty axes for an unchanged boundary).
    pub boundary_disclosure: BoundaryDisclosure,
    /// Whether the window currently has queued actions awaiting the window's end.
    pub queued_actions_present: bool,
    /// The review-before-replay posture for the window's queued actions.
    pub replay_review: ReplayReview,
    /// Whether this window is distinguishable from a generic outage.
    pub distinguishable_from_outage: bool,
    /// One reviewable sentence stating why this is a named operational window, not
    /// random breakage.
    pub outage_distinction: String,
    /// The canonical object the open-details affordance routes to (equals
    /// [`MaintenanceWindow::object_ref`]).
    pub open_detail_ref: String,
}

impl MaintenanceWindow {
    /// Whether the window blocks managed writes (by its phase).
    pub fn blocks_managed_writes(&self) -> bool {
        self.phase.blocks_managed_writes()
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen maintenance / failover / reconciliation window set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceWindowSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_maintenance_windows_schema_version: u32,
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
    /// The operational-phase vocabulary, for consumers.
    pub phase_vocabulary: Vec<TokenDef>,
    /// The write-posture vocabulary, for consumers.
    pub write_posture_vocabulary: Vec<TokenDef>,
    /// The blocked-write-class vocabulary, for consumers.
    pub blocked_write_vocabulary: Vec<TokenDef>,
    /// The boundary-axis vocabulary, for consumers.
    pub boundary_axis_vocabulary: Vec<TokenDef>,
    /// The windows.
    pub windows: Vec<MaintenanceWindow>,
    /// The computed invariants.
    pub invariants: Vec<WindowInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for WindowValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "maintenance-window set invalid: {}", self.reason)
    }
}

impl std::error::Error for WindowValidationError {}

impl MaintenanceWindowSet {
    /// Returns the window with the given id, if present.
    pub fn window(&self, window_id: &str) -> Option<&MaintenanceWindow> {
        self.windows.iter().find(|w| w.window_id == window_id)
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
        for w in &self.windows {
            refs.push(w.object_ref.as_str());
            refs.push(w.open_detail_ref.as_str());
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    /// Complements the computed [`WindowInvariant`]s with the uniqueness, computed-
    /// state, surface-binding, and time-format checks a consumer relies on.
    pub fn validate(&self) -> Result<(), WindowValidationError> {
        let fail = |reason: String| Err(WindowValidationError { reason });

        if self.record_kind != M5_MAINTENANCE_WINDOWS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_MAINTENANCE_WINDOWS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_MAINTENANCE_WINDOWS_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }
        if self.windows.is_empty() {
            return fail("set has no windows".to_owned());
        }

        if !all_unique(self.windows.iter().map(|w| w.window_id.as_str())) {
            return fail("window ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
        let bind = |surface: OperatorSurfaceClass, surface_id: &str| -> bool {
            surface_id == surface.surface_id() && matrix.surface(surface).is_some()
        };

        for w in &self.windows {
            // Surface binding: the window binds the matrix family its kind selects.
            if w.surface != w.kind.surface() || !bind(w.surface, &w.surface_id) {
                return fail(format!(
                    "window {} does not bind its matrix surface",
                    w.window_id
                ));
            }
            // Phase is valid for the kind.
            if !w.kind.permits_phase(w.phase) {
                return fail(format!(
                    "window {} phase {} is not valid for kind {}",
                    w.window_id,
                    w.phase.as_str(),
                    w.kind.as_str()
                ));
            }
            // Canonical identity.
            if !w.object_ref.starts_with("aureline://") || w.open_detail_ref != w.object_ref {
                return fail(format!("window {} hides its canonical object", w.window_id));
            }
            if w.owner.is_empty() || w.decision_right.is_empty() {
                return fail(format!(
                    "window {} hides owner / decision-right",
                    w.window_id
                ));
            }
            // Exact times and time zone.
            let t = &w.window_time;
            if !timestamp_carries_offset(&t.starts_at) || !timestamp_carries_offset(&t.ends_at) {
                return fail(format!(
                    "window {} start/end timestamps must carry an explicit offset",
                    w.window_id
                ));
            }
            if t.time_zone.is_empty() || !is_utc_offset(&t.utc_offset) {
                return fail(format!(
                    "window {} hides its time zone / UTC offset",
                    w.window_id
                ));
            }
            if !offset_matches(&t.starts_at, &t.utc_offset)
                || !offset_matches(&t.ends_at, &t.utc_offset)
            {
                return fail(format!(
                    "window {} timestamps disagree with the stated UTC offset",
                    w.window_id
                ));
            }
            match (parse_rfc3339(&t.starts_at), parse_rfc3339(&t.ends_at)) {
                (Some(start), Some(end)) if start <= end => {}
                (Some(_), Some(_)) => {
                    return fail(format!("window {} ends before it starts", w.window_id))
                }
                _ => {
                    return fail(format!(
                        "window {} has an unparseable window time",
                        w.window_id
                    ))
                }
            }
            if parse_rfc3339(&t.latest_refresh_at).is_none() {
                return fail(format!(
                    "window {} has an unparseable latest-refresh stamp",
                    w.window_id
                ));
            }
            // Effective state is the computed no-silent-green downgrade.
            let expected = compute_effective_state(
                w.phase.matrix_state(),
                t.refresh_freshness,
                BlockerWaiverClass::None,
            );
            if w.effective_state != expected {
                return fail(format!(
                    "window {} effective state is not the computed no-silent-green state",
                    w.window_id
                ));
            }
            // Local-safe actions are explicit.
            if w.local_safe_actions.is_empty() {
                return fail(format!(
                    "window {} lists no local-safe actions",
                    w.window_id
                ));
            }
            // Write posture agrees with the phase, and blocked writes are named.
            if w.write_posture.blocks_live_writes() != w.blocks_managed_writes() {
                return fail(format!(
                    "window {} write posture disagrees with its phase",
                    w.window_id
                ));
            }
            if w.blocks_managed_writes() {
                if w.blocked_writes.is_empty() {
                    return fail(format!(
                        "window {} blocks managed writes but names no blocked class",
                        w.window_id
                    ));
                }
                if w.blocked_writes
                    .iter()
                    .any(|b| b.local_alternative.is_empty())
                {
                    return fail(format!(
                        "window {} blocks a write class with no local alternative",
                        w.window_id
                    ));
                }
                if !w.publish_later_available {
                    return fail(format!(
                        "window {} blocks managed writes without publish-later capture",
                        w.window_id
                    ));
                }
            }
            // Boundary disclosure: failover / migration windows disclose their axes;
            // any crossed axis forces recheck and a stated disclosure.
            let crossed = w.boundary_disclosure.any_crossed();
            if w.boundary_disclosure.recheck_required != crossed {
                return fail(format!(
                    "window {} recheck flag disagrees with its axes",
                    w.window_id
                ));
            }
            if w.surface == OperatorSurfaceClass::FailoverNotice
                && w.boundary_disclosure.axes.is_empty()
            {
                return fail(format!(
                    "window {} is a failover/migration notice with no boundary disclosure",
                    w.window_id
                ));
            }
            for axis in &w.boundary_disclosure.axes {
                if axis.state.crossed() && axis.disclosure.is_empty() {
                    return fail(format!(
                        "window {} crosses {} without restating it",
                        w.window_id,
                        axis.axis.as_str()
                    ));
                }
            }
            // Review before replay is the computed requirement.
            let want_review = compute_replay_review_required(w.queued_actions_present, crossed);
            if w.replay_review.required != want_review {
                return fail(format!(
                    "window {} replay-review requirement is not computed",
                    w.window_id
                ));
            }
            if want_review {
                if !w.replay_review.trigger.requires_review() {
                    return fail(format!(
                        "window {} requires review but names no trigger",
                        w.window_id
                    ));
                }
                if w.replay_review.reconcile_action.is_empty() {
                    return fail(format!(
                        "window {} requires review but names no reconcile action",
                        w.window_id
                    ));
                }
            } else if w.replay_review.trigger.requires_review() {
                return fail(format!(
                    "window {} names a review trigger but does not require review",
                    w.window_id
                ));
            }
            // Distinguishable from a generic outage.
            if !w.distinguishable_from_outage || w.outage_distinction.is_empty() {
                return fail(format!(
                    "window {} is not distinguishable from a generic outage",
                    w.window_id
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

/// Parses an RFC3339 timestamp, returning `None` on any parse failure.
fn parse_rfc3339(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &Rfc3339).ok()
}

/// Whether a timestamp carries an explicit UTC offset (a trailing `Z` or a
/// `±hh:mm` offset), so no window relies on a vague local-without-zone time.
fn timestamp_carries_offset(ts: &str) -> bool {
    if !ts.contains('T') {
        return false;
    }
    ts.ends_with('Z') || has_numeric_offset_suffix(ts)
}

/// Whether the string ends with a `±hh:mm` offset.
fn has_numeric_offset_suffix(ts: &str) -> bool {
    if ts.len() < 6 {
        return false;
    }
    let suffix = &ts[ts.len() - 6..];
    is_utc_offset(suffix)
}

/// Whether the string is a `±hh:mm` UTC offset (for example `-04:00`).
fn is_utc_offset(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 6 {
        return false;
    }
    (bytes[0] == b'+' || bytes[0] == b'-')
        && bytes[1].is_ascii_digit()
        && bytes[2].is_ascii_digit()
        && bytes[3] == b':'
        && bytes[4].is_ascii_digit()
        && bytes[5].is_ascii_digit()
}

/// Whether a timestamp's offset matches the stated UTC offset (treating `Z` as
/// `+00:00`).
fn offset_matches(ts: &str, utc_offset: &str) -> bool {
    if ts.ends_with('Z') {
        return utc_offset == "+00:00";
    }
    ts.ends_with(utc_offset)
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical maintenance / failover / reconciliation window set.
///
/// Deterministic: the same bytes every call. Each window's effective state, the
/// boundary recheck flag, the replay-review requirement, and the invariant `holds`
/// flags are computed from the built data, so an inconsistent edit flips an
/// invariant rather than silently passing.
pub fn maintenance_window_set() -> MaintenanceWindowSet {
    let windows = build_windows();
    let invariants = compute_invariants(&windows);

    MaintenanceWindowSet {
        record_kind: M5_MAINTENANCE_WINDOWS_RECORD_KIND.to_owned(),
        m5_maintenance_windows_schema_version: M5_MAINTENANCE_WINDOWS_SCHEMA_VERSION,
        schema_ref: M5_MAINTENANCE_WINDOWS_SCHEMA_REF.to_owned(),
        set_id: M5_MAINTENANCE_WINDOWS_SET_ID.to_owned(),
        as_of: M5_MAINTENANCE_WINDOWS_AS_OF.to_owned(),
        summary: "Planned maintenance, read-only, drain, migration, failover, reconciling, and \
                  resolved windows with exact start/end times and time zones, named blocked write \
                  classes, local-safe / publish-later continuity, explicit changed-boundary \
                  disclosure, and computed review-before-replay — bound to the operator-surface \
                  matrix so dashboards, notices, companion, and support exports share one truth."
            .to_owned(),
        matrix_ref: M5_MAINTENANCE_WINDOWS_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_MAINTENANCE_WINDOWS_MATRIX_RECORD_KIND.to_owned(),
        phase_vocabulary: token_defs(
            OperationalPhaseClass::ALL
                .iter()
                .map(|p| (p.as_str(), p.label())),
        ),
        write_posture_vocabulary: token_defs(
            WritePostureClass::ALL
                .iter()
                .map(|p| (p.as_str(), p.label())),
        ),
        blocked_write_vocabulary: token_defs(
            BlockedWriteClass::ALL
                .iter()
                .map(|c| (c.as_str(), c.label())),
        ),
        boundary_axis_vocabulary: token_defs(
            BoundaryAxisClass::ALL
                .iter()
                .map(|a| (a.as_str(), a.label())),
        ),
        windows,
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

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn blocked(class: BlockedWriteClass, local_alternative: &str) -> BlockedWrite {
    BlockedWrite {
        class,
        label: class.label().to_owned(),
        local_alternative: local_alternative.to_owned(),
    }
}

fn axis(
    axis: BoundaryAxisClass,
    state: BoundaryAxisStateClass,
    disclosure: &str,
) -> BoundaryAxisDisclosure {
    BoundaryAxisDisclosure {
        axis,
        state,
        disclosure: disclosure.to_owned(),
    }
}

/// Builds a boundary disclosure, computing its recheck flag from the axes.
fn disclosure(axes: Vec<BoundaryAxisDisclosure>) -> BoundaryDisclosure {
    let recheck_required = axes.iter().any(|a| a.state.crossed());
    BoundaryDisclosure {
        axes,
        recheck_required,
    }
}

/// Builds a replay review, computing the requirement from the window's queued
/// state and whether the boundary crossed.
fn replay_review(
    queued_actions_present: bool,
    boundary: &BoundaryDisclosure,
    trigger: ReplayReviewTriggerClass,
    reconcile_action: &str,
) -> ReplayReview {
    let required = compute_replay_review_required(queued_actions_present, boundary.any_crossed());
    if required {
        ReplayReview {
            required,
            trigger,
            reconcile_action: reconcile_action.to_owned(),
        }
    } else {
        ReplayReview {
            required,
            trigger: ReplayReviewTriggerClass::NoReviewNeeded,
            reconcile_action: String::new(),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn window(
    id: &str,
    object_ref: &str,
    title: &str,
    summary: &str,
    kind: WindowKindClass,
    phase: OperationalPhaseClass,
    owner: &str,
    decision_right: &str,
    window_time: WindowTime,
    write_posture: WritePostureClass,
    blocked_writes: Vec<BlockedWrite>,
    local_safe_actions: Vec<String>,
    publish_later_available: bool,
    next_safe_action: &str,
    boundary_disclosure: BoundaryDisclosure,
    queued_actions_present: bool,
    replay_review: ReplayReview,
    outage_distinction: &str,
) -> MaintenanceWindow {
    let effective_state = compute_effective_state(
        phase.matrix_state(),
        window_time.refresh_freshness,
        BlockerWaiverClass::None,
    );
    MaintenanceWindow {
        window_id: id.to_owned(),
        object_ref: object_ref.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        kind,
        surface: kind.surface(),
        surface_id: kind.surface().surface_id(),
        phase,
        effective_state,
        owner: owner.to_owned(),
        decision_right: decision_right.to_owned(),
        scope: ScopeClass::ManagedOrg,
        default_redaction: RedactionClass::MetadataSafeDefault,
        consumed_by: vec![
            ConsumerClass::ShellUi,
            ConsumerClass::CliHeadless,
            ConsumerClass::IncidentWorkspace,
            ConsumerClass::SupportExport,
            ConsumerClass::ManagedService,
            ConsumerClass::CompanionBrowser,
        ],
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        window_time,
        write_posture,
        blocked_writes,
        local_safe_actions,
        publish_later_available,
        next_safe_action: next_safe_action.to_owned(),
        boundary_disclosure,
        queued_actions_present,
        replay_review,
        distinguishable_from_outage: true,
        outage_distinction: outage_distinction.to_owned(),
        open_detail_ref: object_ref.to_owned(),
    }
}

fn window_time(
    starts_at: &str,
    ends_at: &str,
    time_zone: &str,
    utc_offset: &str,
    end_is_estimated: bool,
    latest_refresh_at: &str,
    refresh_freshness: FreshnessClass,
) -> WindowTime {
    WindowTime {
        starts_at: starts_at.to_owned(),
        ends_at: ends_at.to_owned(),
        time_zone: time_zone.to_owned(),
        utc_offset: utc_offset.to_owned(),
        end_is_estimated,
        latest_refresh_at: latest_refresh_at.to_owned(),
        refresh_freshness,
    }
}

fn build_windows() -> Vec<MaintenanceWindow> {
    use BlockedWriteClass::*;
    use BoundaryAxisClass as Ax;
    use BoundaryAxisStateClass as St;
    use OperationalPhaseClass as Ph;
    use WindowKindClass as K;
    use WritePostureClass as Wp;

    vec![
        // 1. Scheduled planned maintenance — announced, not yet in effect.
        window(
            "maintenance_window.0001",
            "aureline://service-health/managed-control-plane",
            "Scheduled control-plane maintenance",
            "A planned maintenance window for the managed control plane, announced with an exact \
             start, end, and time zone; local editing stays fully available until it begins.",
            K::PlannedMaintenance,
            Ph::Scheduled,
            "platform_oncall",
            "sre_lead",
            window_time(
                "2026-06-23T02:00:00-04:00",
                "2026-06-23T04:00:00-04:00",
                "America/New_York",
                "-04:00",
                false,
                "2026-06-22T08:00:00-04:00",
                FreshnessClass::Fresh,
            ),
            Wp::WritesLive,
            vec![],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_before_window",
                "open_continuity_packet",
            ]),
            false,
            "export_before_window",
            disclosure(vec![]),
            false,
            replay_review(false, &disclosure(vec![]), ReplayReviewTriggerClass::NoReviewNeeded, ""),
            "Named maintenance window with exact start, end, and time zone — not a generic outage; \
             nothing is blocked yet and local work is unaffected.",
        ),
        // 2. Read-only window in effect — managed writes queue, boundary unchanged.
        window(
            "maintenance_window.0002",
            "aureline://service-health/managed-settings-service",
            "Read-only maintenance window in effect",
            "A read-only window is in effect: managed settings and policy writes are blocked and \
             saved as local drafts that publish when the window ends; the route is unchanged.",
            K::PlannedMaintenance,
            Ph::ReadOnly,
            "platform_oncall",
            "sre_lead",
            window_time(
                "2026-06-22T01:00:00Z",
                "2026-06-22T03:00:00Z",
                "UTC",
                "+00:00",
                false,
                "2026-06-22T01:05:00Z",
                FreshnessClass::Fresh,
            ),
            Wp::PublishLaterQueued,
            vec![
                blocked(
                    ManagedSettingsApply,
                    "Saved as a local draft; queues to publish when the window ends.",
                ),
                blocked(
                    ManagedPolicyChange,
                    "Captured to the publish-later queue against the same unchanged route.",
                ),
            ],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_diagnostics",
                "inspect_evidence",
                "publish_later",
            ]),
            true,
            "publish_later",
            disclosure(vec![axis(
                Ax::EndpointIdentity,
                St::Unchanged,
                "Endpoint unchanged: queued writes replay against the same route when the window \
                 ends.",
            )]),
            true,
            replay_review(
                true,
                &disclosure(vec![axis(Ax::EndpointIdentity, St::Unchanged, "")]),
                ReplayReviewTriggerClass::NoReviewNeeded,
                "",
            ),
            "Named read-only window with exact start, end, and time zone — not a generic outage; \
             local editing, save, and search continue and blocked writes queue rather than fail.",
        ),
        // 3. Drain window — in-flight work finishes, new actions queue.
        window(
            "maintenance_window.0003",
            "aureline://service-health/managed-pipeline-service",
            "Drain window before maintenance",
            "A drain window is in effect: in-flight managed work finishes while new managed \
             applies queue to publish later; the route is unchanged.",
            K::PlannedMaintenance,
            Ph::Drain,
            "platform_oncall",
            "sre_lead",
            window_time(
                "2026-06-22T00:30:00-04:00",
                "2026-06-22T01:00:00-04:00",
                "America/New_York",
                "-04:00",
                false,
                "2026-06-22T00:35:00-04:00",
                FreshnessClass::Recent,
            ),
            Wp::PublishLaterQueued,
            vec![blocked(
                ManagedSettingsApply,
                "New applies queue to publish when the drain completes; in-flight work finishes.",
            )],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_diagnostics",
                "inspect_evidence",
                "publish_later",
            ]),
            true,
            "publish_later",
            disclosure(vec![axis(
                Ax::EndpointIdentity,
                St::Unchanged,
                "Endpoint unchanged across the drain: queued applies replay to the same route.",
            )]),
            true,
            replay_review(
                true,
                &disclosure(vec![axis(Ax::EndpointIdentity, St::Unchanged, "")]),
                ReplayReviewTriggerClass::NoReviewNeeded,
                "",
            ),
            "Named drain window with exact start, end, and time zone — not a generic outage; \
             in-flight work completes and new applies queue.",
        ),
        // 4. Regional failover in progress — boundary changed, review required.
        window(
            "maintenance_window.0004",
            "aureline://service-health/managed-region-primary",
            "Regional failover in progress",
            "A regional failover is in progress: the serving region and endpoint identity have \
             changed, authority-changing writes are blocked, and queued actions must be reviewed \
             before they replay against the new region.",
            K::RegionalFailover,
            Ph::Failover,
            "sre_oncall",
            "incident_commander",
            window_time(
                "2026-06-22T00:10:00Z",
                "2026-06-22T02:10:00Z",
                "UTC",
                "+00:00",
                true,
                "2026-06-22T00:12:00Z",
                FreshnessClass::Fresh,
            ),
            Wp::BlockedPendingBoundaryRecheck,
            vec![
                blocked(
                    AuthorityChange,
                    "Refused, not retried: authority-changing actions wait for the new region to \
                     be reviewed.",
                ),
                blocked(
                    ManagedSettingsApply,
                    "Held as a local draft; replays only after the new region is reviewed.",
                ),
                blocked(
                    ProviderMutation,
                    "Queued to publish later against the reviewed endpoint.",
                ),
            ],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_diagnostics",
                "inspect_evidence",
                "review_new_boundary",
                "publish_later",
            ]),
            true,
            "review_new_boundary",
            disclosure(vec![
                axis(
                    Ax::Region,
                    St::Changed,
                    "Region changed: traffic moved from the primary to the standby failover \
                     region.",
                ),
                axis(
                    Ax::EndpointIdentity,
                    St::Changed,
                    "Endpoint identity changed to the standby region's endpoint.",
                ),
            ]),
            true,
            replay_review(
                true,
                &disclosure(vec![
                    axis(Ax::Region, St::Changed, "x"),
                    axis(Ax::EndpointIdentity, St::Changed, "x"),
                ]),
                ReplayReviewTriggerClass::ChangedRegion,
                "Review queued managed writes against the new standby region before replaying \
                 them.",
            ),
            "Named regional failover with exact start, estimated end, and time zone, and an \
             explicit changed region and endpoint — not a generic outage implying an unchanged \
             route.",
        ),
        // 5. Tenant migration in progress — tenant/residency/key changed.
        window(
            "maintenance_window.0005",
            "aureline://service-health/managed-tenant-migration",
            "Tenant migration in progress",
            "A tenant migration is in progress: the serving tenant, data residency, and key \
             ownership have changed, so authority and policy writes are blocked and queued actions \
             must be reviewed before replay against the new tenant.",
            K::TenantMigration,
            Ph::Migration,
            "sre_oncall",
            "incident_commander",
            window_time(
                "2026-06-22T03:00:00+02:00",
                "2026-06-22T05:00:00+02:00",
                "Europe/Berlin",
                "+02:00",
                true,
                "2026-06-22T03:02:00+02:00",
                FreshnessClass::Fresh,
            ),
            Wp::BlockedPendingBoundaryRecheck,
            vec![
                blocked(
                    AuthorityChange,
                    "Refused, not retried: waits for the new tenant and key posture to be \
                     reviewed.",
                ),
                blocked(
                    ManagedPolicyChange,
                    "Held as a local draft; replays only after the new tenant is reviewed.",
                ),
                blocked(
                    ManagedSettingsApply,
                    "Queued to publish later against the reviewed tenant.",
                ),
            ],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_diagnostics",
                "inspect_evidence",
                "review_new_boundary",
                "publish_later",
            ]),
            true,
            "review_new_boundary",
            disclosure(vec![
                axis(
                    Ax::Tenant,
                    St::Changed,
                    "Tenant changed: the serving tenant moved to the migration target.",
                ),
                axis(
                    Ax::Residency,
                    St::Changed,
                    "Residency changed: data now resides in the target region's residency zone.",
                ),
                axis(
                    Ax::KeyOwnership,
                    St::Changed,
                    "Key ownership changed to the target tenant's signing authority.",
                ),
            ]),
            true,
            replay_review(
                true,
                &disclosure(vec![
                    axis(Ax::Tenant, St::Changed, "x"),
                    axis(Ax::Residency, St::Changed, "x"),
                    axis(Ax::KeyOwnership, St::Changed, "x"),
                ]),
                ReplayReviewTriggerClass::ChangedTenant,
                "Review queued managed writes against the new tenant, residency, and key posture \
                 before replaying them.",
            ),
            "Named tenant migration with exact start, estimated end, and time zone, and an \
             explicit changed tenant, residency, and key posture — not a generic outage implying \
             an unchanged route.",
        ),
        // 6. Reconciling after failover — boundary changed, review still required.
        window(
            "maintenance_window.0006",
            "aureline://service-health/managed-region-reconcile",
            "Reconciling after regional failover",
            "The failover has settled on the standby region and the system is reconciling; the \
             changed region and endpoint stay disclosed and queued actions are reviewed before \
             they replay.",
            K::RegionalFailover,
            Ph::Reconciling,
            "sre_oncall",
            "incident_commander",
            window_time(
                "2026-06-22T02:10:00Z",
                "2026-06-22T02:40:00Z",
                "UTC",
                "+00:00",
                true,
                "2026-06-22T02:12:00Z",
                FreshnessClass::Recent,
            ),
            Wp::PublishLaterQueued,
            vec![
                blocked(
                    AuthorityChange,
                    "Held until reconciliation completes and the new region is confirmed.",
                ),
                blocked(
                    ManagedSettingsApply,
                    "Queued to publish later once the queue is reviewed against the new region.",
                ),
            ],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_diagnostics",
                "inspect_evidence",
                "review_new_boundary",
                "publish_later",
            ]),
            true,
            "review_new_boundary",
            disclosure(vec![
                axis(
                    Ax::Region,
                    St::Changed,
                    "Region settled on the standby failover region.",
                ),
                axis(
                    Ax::EndpointIdentity,
                    St::Changed,
                    "Endpoint identity is the standby region's endpoint.",
                ),
            ]),
            true,
            replay_review(
                true,
                &disclosure(vec![
                    axis(Ax::Region, St::Changed, "x"),
                    axis(Ax::EndpointIdentity, St::Changed, "x"),
                ]),
                ReplayReviewTriggerClass::ChangedRegion,
                "Reconcile the queued writes against the new region before replay completes.",
            ),
            "Named reconciliation phase with exact start, estimated end, and time zone, and the \
             changed region still disclosed — not a generic outage implying recovery to the \
             original route.",
        ),
        // 7. Resolved — back to normal; nothing blocked, boundary unchanged.
        window(
            "maintenance_window.0007",
            "aureline://service-health/managed-control-plane",
            "Maintenance window resolved",
            "The planned maintenance window has ended and normal operation has resumed against the \
             unchanged route; managed writes are live again and nothing is queued for review.",
            K::PlannedMaintenance,
            Ph::Resolved,
            "platform_oncall",
            "sre_lead",
            window_time(
                "2026-06-21T02:00:00-04:00",
                "2026-06-21T03:00:00-04:00",
                "America/New_York",
                "-04:00",
                false,
                "2026-06-22T07:50:00-04:00",
                FreshnessClass::Recent,
            ),
            Wp::WritesLive,
            vec![],
            strvec(&[
                "continue_local_edit",
                "save_local",
                "search",
                "export_diagnostics",
                "inspect_evidence",
            ]),
            false,
            "continue_local",
            disclosure(vec![axis(
                Ax::EndpointIdentity,
                St::Unchanged,
                "Endpoint unchanged: operation resumed against the original route.",
            )]),
            false,
            replay_review(
                false,
                &disclosure(vec![axis(Ax::EndpointIdentity, St::Unchanged, "")]),
                ReplayReviewTriggerClass::NoReviewNeeded,
                "",
            ),
            "Named resolved maintenance window with the exact completed start and end and the \
             unchanged route restated — not a silent recovery.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> WindowInvariant {
    WindowInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(windows: &[MaintenanceWindow]) -> Vec<WindowInvariant> {
    let mut out = Vec::new();

    // Surface binding: each window binds the matrix family its kind selects.
    out.push(invariant(
        "maintenance_windows.surface_binding",
        "Every window binds the operator-surface matrix family its kind selects — planned \
         maintenance binds the maintenance notice; failover and migration bind the failover \
         notice — by that matrix's own surface id.",
        windows
            .iter()
            .all(|w| w.surface == w.kind.surface() && w.surface_id == w.surface.surface_id()),
    ));

    // Canonical object identity.
    out.push(invariant(
        "maintenance_windows.canonical_object_identity",
        "Every window resolves to a canonical aureline:// object and opens that same object on \
         detail, so a notice points at the same underlying object the service-health and incident \
         surfaces use.",
        windows
            .iter()
            .all(|w| w.object_ref.starts_with("aureline://") && w.open_detail_ref == w.object_ref),
    ));

    // Exact time and time zone — never a vague relative banner.
    out.push(invariant(
        "maintenance_windows.exact_time_and_zone",
        "Every window names an exact start and end timestamp with an explicit UTC offset, an IANA \
         time zone, and timestamps that agree with the stated offset — never a vague relative \
         time.",
        windows.iter().all(|w| {
            let t = &w.window_time;
            timestamp_carries_offset(&t.starts_at)
                && timestamp_carries_offset(&t.ends_at)
                && !t.time_zone.is_empty()
                && is_utc_offset(&t.utc_offset)
                && offset_matches(&t.starts_at, &t.utc_offset)
                && offset_matches(&t.ends_at, &t.utc_offset)
        }),
    ));

    // Latest-refresh state visible.
    out.push(invariant(
        "maintenance_windows.latest_refresh_visible",
        "Every window names the moment its state was last refreshed and that refresh's freshness, \
         so a stale window is not presented as a live one.",
        windows
            .iter()
            .all(|w| parse_rfc3339(&w.window_time.latest_refresh_at).is_some()),
    ));

    // Effective state is the computed no-silent-green downgrade.
    out.push(invariant(
        "maintenance_windows.effective_state_computed",
        "Every window's effective state is the computed no-silent-green downgrade of its phase's \
         matrix state and its latest-refresh freshness, so a resolved-but-unconfirmed window never \
         reads as a confirmed clear.",
        windows.iter().all(|w| {
            w.effective_state
                == compute_effective_state(
                    w.phase.matrix_state(),
                    w.window_time.refresh_freshness,
                    BlockerWaiverClass::None,
                )
        }),
    ));

    // Blocked write classes are named with a local alternative.
    out.push(invariant(
        "maintenance_windows.blocked_writes_named",
        "Every window that blocks managed writes names at least one blocked write class and, for \
         each, the local-safe alternative that preserves the operator's work.",
        windows.iter().all(|w| {
            if !w.blocks_managed_writes() {
                return true;
            }
            !w.blocked_writes.is_empty()
                && w.blocked_writes
                    .iter()
                    .all(|b| !b.local_alternative.is_empty())
        }),
    ));

    // Local-safe actions explicit.
    out.push(invariant(
        "maintenance_windows.local_safe_explicit",
        "Every window names at least one local-safe action that stays available during it, so a \
         window never reads as a total outage.",
        windows.iter().all(|w| !w.local_safe_actions.is_empty()),
    ));

    // Publish-later when writes are blocked.
    out.push(invariant(
        "maintenance_windows.publish_later_when_blocked",
        "Every window that blocks managed writes offers publish-later / draft capture and a write \
         posture that preserves the work, so blocked writes are queued rather than lost.",
        windows.iter().all(|w| {
            if !w.blocks_managed_writes() {
                return true;
            }
            w.publish_later_available && w.write_posture.blocks_live_writes()
        }),
    ));

    // Boundary disclosure on failover / migration.
    out.push(invariant(
        "maintenance_windows.boundary_disclosed_on_failover",
        "Every failover or migration window discloses at least one boundary axis, sets its recheck \
         flag from those axes, and restates any changed or unknown axis explicitly rather than \
         implying an unchanged route.",
        windows.iter().all(|w| {
            if w.surface != OperatorSurfaceClass::FailoverNotice {
                return w.boundary_disclosure.recheck_required == w.boundary_disclosure.any_crossed();
            }
            !w.boundary_disclosure.axes.is_empty()
                && w.boundary_disclosure.recheck_required == w.boundary_disclosure.any_crossed()
                && w
                    .boundary_disclosure
                    .axes
                    .iter()
                    .all(|a| !a.state.crossed() || !a.disclosure.is_empty())
        }),
    ));

    // Review before replay is computed.
    out.push(invariant(
        "maintenance_windows.review_before_replay_computed",
        "Every window's review-before-replay requirement is the computed rule — required exactly \
         when queued actions would cross a changed or unknown boundary — and a required review \
         names a trigger and a reconcile action.",
        windows.iter().all(|w| {
            let want = compute_replay_review_required(
                w.queued_actions_present,
                w.boundary_disclosure.any_crossed(),
            );
            if w.replay_review.required != want {
                return false;
            }
            if want {
                w.replay_review.trigger.requires_review()
                    && !w.replay_review.reconcile_action.is_empty()
            } else {
                !w.replay_review.trigger.requires_review()
            }
        }),
    ));

    // Distinguishable from a generic outage.
    out.push(invariant(
        "maintenance_windows.distinguishable_from_outage",
        "Every window is marked distinguishable from a generic outage and carries a non-empty \
         sentence naming why it is a known operational window, not random breakage.",
        windows
            .iter()
            .all(|w| w.distinguishable_from_outage && !w.outage_distinction.is_empty()),
    ));

    // Every operational phase is exercised.
    out.push(invariant(
        "maintenance_windows.all_phases_present",
        "The set exercises every operational phase: scheduled, read-only, drain, migration, \
         failover, reconciling, and resolved.",
        OperationalPhaseClass::ALL
            .iter()
            .all(|phase| windows.iter().any(|w| w.phase == *phase)),
    ));

    // Both matrix surfaces are exercised.
    out.push(invariant(
        "maintenance_windows.both_surfaces_present",
        "The set exercises both the maintenance notice and the failover notice matrix surfaces.",
        windows
            .iter()
            .any(|w| w.surface == OperatorSurfaceClass::MaintenanceNotice)
            && windows
                .iter()
                .any(|w| w.surface == OperatorSurfaceClass::FailoverNotice),
    ));

    // Stable ids unique.
    out.push(invariant(
        "maintenance_windows.stable_ids_unique",
        "Window ids are unique, so a consumer can resolve a window by a stable id.",
        all_unique(windows.iter().map(|w| w.window_id.as_str())),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the window set as human-readable lines for CLI/headless and support.
pub fn maintenance_window_lines(set: &MaintenanceWindowSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Maintenance & failover windows — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());

    let maintenance = set
        .windows
        .iter()
        .filter(|w| w.surface == OperatorSurfaceClass::MaintenanceNotice)
        .count();
    let failover = set.windows.len() - maintenance;
    lines.push(format!(
        "Windows: {} ({} maintenance, {} failover)",
        set.windows.len(),
        maintenance,
        failover
    ));

    lines.push("Windows:".to_owned());
    for w in &set.windows {
        let t = &w.window_time;
        lines.push(format!(
            "  - {} [{}] kind={} phase={} → effective={}",
            w.title,
            w.window_id,
            w.kind.as_str(),
            w.phase.as_str(),
            w.effective_state.as_str(),
        ));
        lines.push(format!(
            "      when: {} → {}{} ({} {})",
            t.starts_at,
            t.ends_at,
            if t.end_is_estimated { " (est.)" } else { "" },
            t.time_zone,
            t.utc_offset,
        ));
        lines.push(format!(
            "      refreshed: {} ({})  write-posture: {}  publish-later: {}",
            t.latest_refresh_at,
            t.refresh_freshness.as_str(),
            w.write_posture.as_str(),
            w.publish_later_available,
        ));
        if !w.blocked_writes.is_empty() {
            let names: Vec<&str> = w.blocked_writes.iter().map(|b| b.class.as_str()).collect();
            lines.push(format!("      blocked writes: {}", names.join(", ")));
        }
        if !w.local_safe_actions.is_empty() {
            lines.push(format!(
                "      local-safe: {}  next: {}",
                w.local_safe_actions.join(", "),
                w.next_safe_action,
            ));
        }
        if !w.boundary_disclosure.axes.is_empty() {
            let axes: Vec<String> = w
                .boundary_disclosure
                .axes
                .iter()
                .map(|a| format!("{}={}", a.axis.as_str(), a.state.as_str()))
                .collect();
            lines.push(format!(
                "      boundary: {} (recheck: {})",
                axes.join(", "),
                w.boundary_disclosure.recheck_required,
            ));
        }
        lines.push(format!(
            "      review-before-replay: {} (trigger: {})",
            w.replay_review.required,
            w.replay_review.trigger.as_str(),
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
