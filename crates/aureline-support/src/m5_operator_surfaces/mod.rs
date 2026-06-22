//! M5 operator-surface matrix: the frozen, typed contract for Aureline's
//! operator-facing overview, triage, action-plan, handoff, maintenance, and
//! boundary surfaces.
//!
//! Aureline's operator surfaces — operational overview boards, triage inboxes,
//! action plans, evidence handoff bundles, shift digests, service-ownership /
//! on-call strips, runbook-step cards, maintenance / read-only / drain windows,
//! failover notices, and embedded provider/auth boundary states — are governed
//! product contracts, not support-only chrome. Each one already has its own
//! boundary schema under `schemas/ops/` and at least one producing crate. What
//! was missing was a single place that names the surface *families*, freezes
//! their stable identifiers, pins one shared state vocabulary across them, and
//! states the invariants every operator surface must hold. This lane is that
//! place.
//!
//! The matrix does three things:
//!
//! 1. **Names the surface families** ([`OperatorSurfaceClass`]) and, for each,
//!    cites the canonical `schemas/ops/` boundary schema(s) it binds and the
//!    crate(s) that already produce that truth — so dashboards and queues point
//!    at the same underlying objects incident/support/review/admin flows use
//!    rather than inventing a parallel truth model.
//! 2. **Freezes one state vocabulary** ([`OperatorStateClass`]) spanning
//!    operational overview, triage, maintenance windows, failover, reconciling,
//!    boundary drift, and embedded browser/auth handoff. Stable tokens, state
//!    terms, ownership fields, freshness rules, redaction classes, local-versus-
//!    shared scope, and live-versus-snapshot labels are defined once and reused
//!    by UI, CLI/headless, support export, and companion/browser consumers.
//! 3. **Covers every operator path** ([`OperatorPathClass`]): local, remote,
//!    managed, mirrored/offline, browser/webview, and imported-snapshot, with
//!    the write posture and boundary-recheck rule each path carries.
//!
//! [`operator_surface_matrix`] is the canonical binding: it builds the matrix
//! deterministically and computes each [`MatrixInvariant`]'s `holds` flag from
//! the built data, so the checked-in fixture and the replay gate freeze the
//! contract byte-for-byte and an inconsistent edit flips an invariant and fails
//! CI. The record carries no endpoint URLs, hostnames, credentials, raw payloads,
//! or absolute paths — only opaque object refs, stable tokens, and short
//! reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the operator-surface matrix.
pub const M5_OPERATOR_SURFACES_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the operator-surface matrix.
pub const M5_OPERATOR_SURFACES_SCHEMA_REF: &str = "schemas/ops/m5-operator-surfaces.schema.json";

/// Stable record-kind tag for the operator-surface matrix.
pub const M5_OPERATOR_SURFACES_RECORD_KIND: &str = "m5_operator_surface_matrix";

/// Stable id for the canonical operator-surface matrix.
pub const M5_OPERATOR_SURFACES_MATRIX_ID: &str = "m5-operator-surfaces:matrix:0001";

/// Evaluation stamp for the canonical matrix. Held as a constant so the
/// canonical binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_OPERATOR_SURFACES_AS_OF: &str = "2026-06-22T00:00:00Z";

// ---------------------------------------------------------------------------
// Surface families.
// ---------------------------------------------------------------------------

/// The closed set of operator-surface families this matrix freezes.
///
/// Each family is one operator-facing contract. Adding a family is a breaking
/// change to the matrix; renaming one breaks every consumer that resolves a
/// surface by token, so the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorSurfaceClass {
    /// The operational overview board: freshness cards and service-health rows
    /// that headline state without ever showing unconfirmed green.
    OperationalOverviewBoard,
    /// A triage inbox / queue: ordered rows that name their order and narrowing
    /// reasons and point at the same incident/review/support/admin objects.
    TriageInbox,
    /// An action plan: the ordered, ownership-bearing set of mitigation and
    /// inspection steps for an incident or operation.
    ActionPlan,
    /// An evidence handoff bundle: a frozen, scope/freshness/ownership/redaction-
    /// preserving export with explicit live-versus-snapshot truth.
    HandoffBundle,
    /// A shift digest: a windowed, coverage-labeled roll-up of operator events.
    ShiftDigest,
    /// A service ownership / on-call strip: who owns a service, its contract
    /// state, and its local-continuity posture.
    ServiceOwnershipStrip,
    /// A runbook-step card: one guided response step with its intent, sandbox /
    /// approval admission, and outcome.
    RunbookStepCard,
    /// A planned maintenance / read-only / drain window notice.
    MaintenanceNotice,
    /// A failover / migration notice with boundary-axis state and continuity
    /// action posture.
    FailoverNotice,
    /// An embedded provider/auth boundary state: route, drift, approval source,
    /// and the verbatim visible language a webview/browser/auth surface shows.
    EmbeddedBoundaryState,
}

impl OperatorSurfaceClass {
    /// All surface families, in matrix order.
    pub const ALL: [Self; 10] = [
        Self::OperationalOverviewBoard,
        Self::TriageInbox,
        Self::ActionPlan,
        Self::HandoffBundle,
        Self::ShiftDigest,
        Self::ServiceOwnershipStrip,
        Self::RunbookStepCard,
        Self::MaintenanceNotice,
        Self::FailoverNotice,
        Self::EmbeddedBoundaryState,
    ];

    /// Stable snake_case token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperationalOverviewBoard => "operational_overview_board",
            Self::TriageInbox => "triage_inbox",
            Self::ActionPlan => "action_plan",
            Self::HandoffBundle => "handoff_bundle",
            Self::ShiftDigest => "shift_digest",
            Self::ServiceOwnershipStrip => "service_ownership_strip",
            Self::RunbookStepCard => "runbook_step_card",
            Self::MaintenanceNotice => "maintenance_notice",
            Self::FailoverNotice => "failover_notice",
            Self::EmbeddedBoundaryState => "embedded_boundary_state",
        }
    }

    /// Stable surface id, namespaced so it is unique across the product.
    pub fn surface_id(self) -> String {
        format!("operator_surface.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OperationalOverviewBoard => "Operational overview board",
            Self::TriageInbox => "Triage inbox",
            Self::ActionPlan => "Action plan",
            Self::HandoffBundle => "Evidence handoff bundle",
            Self::ShiftDigest => "Shift digest",
            Self::ServiceOwnershipStrip => "Service ownership / on-call strip",
            Self::RunbookStepCard => "Runbook-step card",
            Self::MaintenanceNotice => "Maintenance / read-only / drain notice",
            Self::FailoverNotice => "Failover / migration notice",
            Self::EmbeddedBoundaryState => "Embedded provider/auth boundary",
        }
    }
}

// ---------------------------------------------------------------------------
// Unified state vocabulary.
// ---------------------------------------------------------------------------

/// One shared state vocabulary spanning every operator surface.
///
/// The tokens are the union of the per-surface state enums already frozen under
/// `schemas/ops/`; each [`StateTerm`] in the matrix cites the upstream schema
/// enums it derives from, so this vocabulary never silently diverges from the
/// surfaces it summarizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorStateClass {
    /// Healthy and confirmed-fresh.
    Clear,
    /// Would-be green, but the evidence behind it is stale, partial, or cached:
    /// the no-silent-green downgrade.
    Unconfirmed,
    /// Needs attention but is not blocked.
    Attention,
    /// Blocked: an action or surface cannot proceed and says why.
    Blocked,
    /// A maintenance window is announced/scheduled but not yet in effect.
    ScheduledWindow,
    /// A read-only window is in effect: new managed writes are blocked, local
    /// work continues, and publish-later capture is offered.
    ReadOnlyWindow,
    /// A drain window is in effect: existing work finishes, new actions queue.
    DrainWindow,
    /// Reconciling after a window or event before normal operation resumes.
    Reconciling,
    /// A failover is in progress.
    FailoverInProgress,
    /// A tenant/region/residency migration is in progress.
    MigrationInProgress,
    /// A boundary axis (tenant/region/residency/key/endpoint) changed or is
    /// unknown and requires explicit recheck before managed writes resume.
    BoundaryDriftRecheckRequired,
    /// An embedded browser/console/auth handoff is the route: an attributable
    /// exit, never a silent native approval.
    EmbeddedBoundaryHandoff,
    /// Imported/replay evidence with no live target: read-only.
    ImportedSnapshotNoLive,
    /// State could not be determined and requires user review.
    UnknownRequiresReview,
}

impl OperatorStateClass {
    /// All states, in vocabulary order.
    pub const ALL: [Self; 14] = [
        Self::Clear,
        Self::Unconfirmed,
        Self::Attention,
        Self::Blocked,
        Self::ScheduledWindow,
        Self::ReadOnlyWindow,
        Self::DrainWindow,
        Self::Reconciling,
        Self::FailoverInProgress,
        Self::MigrationInProgress,
        Self::BoundaryDriftRecheckRequired,
        Self::EmbeddedBoundaryHandoff,
        Self::ImportedSnapshotNoLive,
        Self::UnknownRequiresReview,
    ];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Unconfirmed => "unconfirmed",
            Self::Attention => "attention",
            Self::Blocked => "blocked",
            Self::ScheduledWindow => "scheduled_window",
            Self::ReadOnlyWindow => "read_only_window",
            Self::DrainWindow => "drain_window",
            Self::Reconciling => "reconciling",
            Self::FailoverInProgress => "failover_in_progress",
            Self::MigrationInProgress => "migration_in_progress",
            Self::BoundaryDriftRecheckRequired => "boundary_drift_recheck_required",
            Self::EmbeddedBoundaryHandoff => "embedded_boundary_handoff",
            Self::ImportedSnapshotNoLive => "imported_snapshot_no_live",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Clear => "Clear",
            Self::Unconfirmed => "Unconfirmed (green downgraded)",
            Self::Attention => "Attention",
            Self::Blocked => "Blocked",
            Self::ScheduledWindow => "Scheduled window",
            Self::ReadOnlyWindow => "Read-only window",
            Self::DrainWindow => "Drain window",
            Self::Reconciling => "Reconciling",
            Self::FailoverInProgress => "Failover in progress",
            Self::MigrationInProgress => "Migration in progress",
            Self::BoundaryDriftRecheckRequired => "Boundary drift — recheck required",
            Self::EmbeddedBoundaryHandoff => "Embedded boundary handoff",
            Self::ImportedSnapshotNoLive => "Imported snapshot — no live target",
            Self::UnknownRequiresReview => "Unknown — requires review",
        }
    }

    /// Whether this state blocks new managed/side-effectful actions by default.
    pub const fn blocking_default(self) -> bool {
        matches!(
            self,
            Self::Blocked
                | Self::ReadOnlyWindow
                | Self::FailoverInProgress
                | Self::MigrationInProgress
                | Self::BoundaryDriftRecheckRequired
        )
    }

    /// The `schemas/ops/` enum tokens this state derives from, for provenance.
    fn derived_from_refs(self) -> Vec<String> {
        let refs: &[&str] = match self {
            Self::Clear => &[
                "dashboard_freshness_card.schema.json#displayed_state_token.clear",
                "service_contract_state.schema.json#service_contract_state_token.ready",
            ],
            Self::Unconfirmed => {
                &["dashboard_freshness_card.schema.json#effective_state_token.unconfirmed"]
            }
            Self::Attention => &["dashboard_freshness_card.schema.json#effective_state_token.attention"],
            Self::Blocked => &["dashboard_freshness_card.schema.json#effective_state_token.blocked"],
            Self::ScheduledWindow => &[
                "maintenance_notice.schema.json#maintenance_state_class.scheduled_window",
                "continuity_notice_view.schema.json#notice_kind_class.scheduled_maintenance_window",
            ],
            Self::ReadOnlyWindow => &[
                "maintenance_notice.schema.json#maintenance_state_class.read_only_window",
                "continuity_notice_view.schema.json#notice_kind_class.read_only_window",
            ],
            Self::DrainWindow => &[
                "maintenance_notice.schema.json#maintenance_state_class.drain_window",
                "continuity_notice_view.schema.json#notice_kind_class.drain_window",
            ],
            Self::Reconciling => &[
                "outage_notice.schema.json#notice_state_class.reconciling",
                "continuity_notice_view.schema.json#notice_kind_class.post_event_reconciliation",
            ],
            Self::FailoverInProgress => &[
                "failover_banner.schema.json#trigger_kind_class.regional_failover",
                "outage_notice.schema.json#control_plane_effect_state.failover_in_progress",
            ],
            Self::MigrationInProgress => &[
                "tenant_migration_event.schema.json#event_state_class.in_progress_migration",
                "tenant_migration_event.schema.json#event_state_class.in_progress_failover",
            ],
            Self::BoundaryDriftRecheckRequired => &[
                "failover_banner.schema.json#boundary_axis_state_class.unknown_recheck_required",
                "route_timeline.schema.json#drift_state.drifted",
            ],
            Self::EmbeddedBoundaryHandoff => &[
                "evidence_handoff_bundle.schema.json#handoff_destination_class.system_browser_review_handoff",
                "route_timeline.schema.json#boundary_class.browser_webview_mediated",
            ],
            Self::ImportedSnapshotNoLive => &[
                "evidence_handoff_bundle.schema.json#bundle_kind_class.imported_evidence_replay_bundle_no_live_target",
                "incident_workspace.schema.json#alert_freshness_class.imported_snapshot_no_refresh_path",
            ],
            Self::UnknownRequiresReview => {
                &["route_timeline.schema.json#drift_state.unknown_requires_review"]
            }
        };
        refs.iter().map(|r| format!("schemas/ops/{r}")).collect()
    }
}

// ---------------------------------------------------------------------------
// Operator paths.
// ---------------------------------------------------------------------------

/// The operator deployment/connectivity paths the matrix must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorPathClass {
    /// Local-first, single host, no control plane.
    Local,
    /// Remote workspace / runtime attach.
    Remote,
    /// Managed cloud / enterprise control plane.
    Managed,
    /// Mirror-backed offline: last-synced read-only view.
    MirroredOffline,
    /// Browser / webview-mediated operator surface.
    BrowserWebview,
    /// Imported snapshot: replayed evidence with no live target.
    ImportedSnapshot,
}

impl OperatorPathClass {
    /// All paths, in matrix order.
    pub const ALL: [Self; 6] = [
        Self::Local,
        Self::Remote,
        Self::Managed,
        Self::MirroredOffline,
        Self::BrowserWebview,
        Self::ImportedSnapshot,
    ];

    /// Stable snake_case token for this path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Managed => "managed",
            Self::MirroredOffline => "mirrored_offline",
            Self::BrowserWebview => "browser_webview",
            Self::ImportedSnapshot => "imported_snapshot",
        }
    }

    /// Stable path id, namespaced for uniqueness.
    pub fn path_id(self) -> String {
        format!("operator_path.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Remote => "Remote workspace",
            Self::Managed => "Managed / control plane",
            Self::MirroredOffline => "Mirrored / offline",
            Self::BrowserWebview => "Browser / webview",
            Self::ImportedSnapshot => "Imported snapshot",
        }
    }
}

// ---------------------------------------------------------------------------
// Shared, reused token vocabularies.
// ---------------------------------------------------------------------------

/// Deployment profile, mirroring `deployment_profile_class` used across the
/// `schemas/ops/` surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    /// Individual, local-first install.
    IndividualLocal,
    /// Self-hosted runtime.
    SelfHosted,
    /// Enterprise online.
    EnterpriseOnline,
    /// Air-gapped, offline-only.
    AirGapped,
    /// Managed cloud.
    ManagedCloud,
}

/// Default redaction posture, mirroring `redaction_class` used across the
/// `schemas/ops/` surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default — the export default for operator surfaces.
    MetadataSafeDefault,
    /// Restricted to operators.
    OperatorOnlyRestricted,
    /// Restricted to internal support.
    InternalSupportRestricted,
    /// Signing-evidence only.
    SigningEvidenceOnly,
    /// Private triage only.
    PrivateTriageOnly,
}

/// Trust posture, mirroring `trust_posture` used across the `schemas/ops/`
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPosture {
    /// Untrusted.
    Untrusted,
    /// Restricted.
    Restricted,
    /// Trusted.
    Trusted,
    /// Managed admin.
    ManagedAdmin,
}

/// Local-versus-shared scope of a surface's underlying objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    /// Local and private to this operator/host.
    LocalPrivate,
    /// Shared across a team.
    SharedTeam,
    /// Defined and governed at the managed-org / control-plane level.
    ManagedOrg,
}

/// Whether a surface is live, can be snapshotted, or is snapshot-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveSnapshotClass {
    /// Always live; never persisted as a frozen snapshot.
    LiveOnly,
    /// Live when connected, captured as a labeled snapshot on export/handoff.
    SnapshotCapable,
    /// Snapshot-only: imported/replay evidence with no live refresh path.
    SnapshotOnly,
}

/// The consumers that render an operator surface instead of restating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    /// Desktop shell UI.
    ShellUi,
    /// CLI / headless inspect.
    CliHeadless,
    /// Incident workspace.
    IncidentWorkspace,
    /// Support export / bundle.
    SupportExport,
    /// Admin queue / console.
    AdminQueue,
    /// Release evidence review.
    ReleaseEvidence,
    /// Managed-service / control-plane consumer.
    ManagedService,
    /// Companion / browser surface.
    CompanionBrowser,
}

/// The write posture a path admits for side-effectful operator actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathWritePostureClass {
    /// Connected: managed writes run live (still subject to per-action approval).
    WritesLive,
    /// Writes are captured and queued to publish later.
    PublishLaterQueued,
    /// Writes are preserved as a local draft only.
    LocalDraftPreserved,
    /// Read-only replay of imported evidence; no writes admitted.
    ReadOnlyReplay,
    /// Writes are blocked pending a boundary recheck.
    BlockedPendingBoundaryRecheck,
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One `(token, label)` definition in the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenDef {
    /// Stable token.
    pub token: String,
    /// Human-readable label.
    pub label: String,
}

/// The reused token vocabularies and the source schemas this matrix binds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedVocabulary {
    /// Deployment profiles.
    pub deployment_profiles: Vec<TokenDef>,
    /// Redaction classes.
    pub redaction_classes: Vec<TokenDef>,
    /// Trust postures.
    pub trust_postures: Vec<TokenDef>,
    /// Scope classes.
    pub scope_classes: Vec<TokenDef>,
    /// Live-versus-snapshot classes.
    pub live_snapshot_classes: Vec<TokenDef>,
    /// Consumer classes.
    pub consumer_classes: Vec<TokenDef>,
    /// Boundary axes.
    pub boundary_axes: Vec<TokenDef>,
    /// The `schemas/ops/` boundary schemas this matrix binds as truth sources.
    pub source_schema_refs: Vec<String>,
}

/// One state in the unified vocabulary, with provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateTerm {
    /// The state.
    pub state: OperatorStateClass,
    /// Stable token (equals `state.as_str()`), surfaced for reuse by consumers.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// Whether this state blocks new managed/side-effectful actions by default.
    pub blocking_default: bool,
    /// The `schemas/ops/` enum tokens this state derives from.
    pub derived_from_refs: Vec<String>,
}

/// One ownership/decision-right field a surface must carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipField {
    /// Stable field id.
    pub field_id: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the field is required on every row of the surface.
    pub required: bool,
}

/// The freshness rule a surface applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessRule {
    /// The age tokens the surface uses, oldest path last.
    pub age_tokens: Vec<String>,
    /// Whether a stale/partial age downgrades a would-be-green headline
    /// (the no-silent-green rule).
    pub downgrades_green: bool,
    /// One reviewable sentence stating the rule.
    pub rule: String,
}

/// One operator-surface family entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSurfaceEntry {
    /// The surface family.
    pub surface: OperatorSurfaceClass,
    /// Stable, namespaced surface id.
    pub surface_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the surface.
    pub summary: String,
    /// The canonical `schemas/ops/` boundary schema(s) this surface binds.
    pub canonical_schema_refs: Vec<String>,
    /// The crate module(s) that already produce this truth.
    pub produced_by_refs: Vec<String>,
    /// The consumers that render this surface.
    pub consumed_by: Vec<ConsumerClass>,
    /// The states from the unified vocabulary this surface can show.
    pub applicable_states: Vec<OperatorStateClass>,
    /// The ownership/decision-right fields this surface carries.
    pub ownership_fields: Vec<OwnershipField>,
    /// The freshness rule this surface applies.
    pub freshness_rule: FreshnessRule,
    /// The default redaction posture on export.
    pub default_redaction: RedactionClass,
    /// Local-versus-shared scope of the underlying objects.
    pub scope: ScopeClass,
    /// Live-versus-snapshot posture.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// Whether this surface captures user writes (claims, notes, drafts).
    pub captures_user_writes: bool,
    /// The local-safe actions that stay available during read-only/drain windows.
    pub local_safe_actions: Vec<String>,
    /// Whether publish-later / draft capture is offered during read-only/drain.
    pub publish_later_capture: bool,
    /// Whether the surface is boundary-honest (no native-approval impersonation).
    pub boundary_honest: bool,
    /// One reviewable sentence stating the boundary-honesty rule.
    pub boundary_note: String,
    /// Whether the surface is typed (never screenshot-only / generic prose).
    pub typed_not_screenshot_only: bool,
}

/// One operator-path entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorPathEntry {
    /// The path.
    pub path: OperatorPathClass,
    /// Stable, namespaced path id.
    pub path_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the path.
    pub summary: String,
    /// The deployment profiles that map to this path.
    pub deployment_profiles: Vec<DeploymentProfileClass>,
    /// The default live-versus-snapshot posture on this path.
    pub default_live_vs_snapshot: LiveSnapshotClass,
    /// The write posture this path admits.
    pub write_posture: PathWritePostureClass,
    /// Whether managed writes require a boundary recheck on this path.
    pub boundary_recheck_required: bool,
    /// The `schemas/ops/` local-safe baseline this path leans on.
    pub local_safe_baseline_ref: String,
    /// One reviewable sentence of path-specific notes.
    pub notes: String,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built matrix satisfies the invariant.
    pub holds: bool,
}

/// The frozen operator-surface matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorSurfaceMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_operator_surfaces_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// One reviewable sentence summarizing the matrix.
    pub summary: String,
    /// The reused token vocabularies and bound source schemas.
    pub shared_vocabulary: SharedVocabulary,
    /// The unified state vocabulary.
    pub state_vocabulary: Vec<StateTerm>,
    /// The surface-family entries.
    pub surfaces: Vec<OperatorSurfaceEntry>,
    /// The operator-path entries.
    pub operator_paths: Vec<OperatorPathEntry>,
    /// The computed invariants.
    pub invariants: Vec<MatrixInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the matrix fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for MatrixValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "operator-surface matrix invalid: {}", self.reason)
    }
}

impl std::error::Error for MatrixValidationError {}

impl OperatorSurfaceMatrix {
    /// Returns the entry for a surface family, if present.
    pub fn surface(&self, surface: OperatorSurfaceClass) -> Option<&OperatorSurfaceEntry> {
        self.surfaces.iter().find(|s| s.surface == surface)
    }

    /// Returns the entry for an operator path, if present.
    pub fn path(&self, path: OperatorPathClass) -> Option<&OperatorPathEntry> {
        self.operator_paths.iter().find(|p| p.path == path)
    }

    /// Returns the state term for a state, if present.
    pub fn state_term(&self, state: OperatorStateClass) -> Option<&StateTerm> {
        self.state_vocabulary.iter().find(|t| t.state == state)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref, never a URL, host,
    /// credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    /// Every ref string carried by the matrix, for export-safety auditing.
    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let from_shared = self
            .shared_vocabulary
            .source_schema_refs
            .iter()
            .map(String::as_str);
        let from_states = self
            .state_vocabulary
            .iter()
            .flat_map(|t| t.derived_from_refs.iter().map(String::as_str));
        let from_surfaces = self.surfaces.iter().flat_map(|s| {
            s.canonical_schema_refs
                .iter()
                .map(String::as_str)
                .chain(s.produced_by_refs.iter().map(String::as_str))
        });
        let from_paths = self
            .operator_paths
            .iter()
            .map(|p| p.local_safe_baseline_ref.as_str());
        from_shared
            .chain(from_states)
            .chain(from_surfaces)
            .chain(from_paths)
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`MatrixInvariant`]s with the
    /// uniqueness and completeness checks a consumer relies on.
    pub fn validate(&self) -> Result<(), MatrixValidationError> {
        let fail = |reason: String| Err(MatrixValidationError { reason });

        if self.record_kind != M5_OPERATOR_SURFACES_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_OPERATOR_SURFACES_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        // Every family and every path is present exactly once.
        for surface in OperatorSurfaceClass::ALL {
            if self
                .surfaces
                .iter()
                .filter(|s| s.surface == surface)
                .count()
                != 1
            {
                return fail(format!(
                    "surface {} not present exactly once",
                    surface.as_str()
                ));
            }
        }
        for path in OperatorPathClass::ALL {
            if self
                .operator_paths
                .iter()
                .filter(|p| p.path == path)
                .count()
                != 1
            {
                return fail(format!("path {} not present exactly once", path.as_str()));
            }
        }
        for state in OperatorStateClass::ALL {
            if self
                .state_vocabulary
                .iter()
                .filter(|t| t.state == state)
                .count()
                != 1
            {
                return fail(format!("state {} not present exactly once", state.as_str()));
            }
        }

        // Stable ids are unique.
        if !all_unique(self.surfaces.iter().map(|s| s.surface_id.as_str())) {
            return fail("surface ids are not unique".to_owned());
        }
        if !all_unique(self.operator_paths.iter().map(|p| p.path_id.as_str())) {
            return fail("path ids are not unique".to_owned());
        }
        if !all_unique(self.state_vocabulary.iter().map(|t| t.token.as_str())) {
            return fail("state tokens are not unique".to_owned());
        }

        // Per-surface structural floor: typed, evidenced, owned, fresh.
        for entry in &self.surfaces {
            if entry.surface_id != entry.surface.surface_id() {
                return fail(format!(
                    "surface id mismatch for {}",
                    entry.surface.as_str()
                ));
            }
            if entry.canonical_schema_refs.is_empty() {
                return fail(format!(
                    "surface {} cites no schema",
                    entry.surface.as_str()
                ));
            }
            if entry.produced_by_refs.is_empty() {
                return fail(format!(
                    "surface {} has no producer",
                    entry.surface.as_str()
                ));
            }
            if entry.applicable_states.is_empty() {
                return fail(format!(
                    "surface {} declares no states",
                    entry.surface.as_str()
                ));
            }
            if entry.ownership_fields.is_empty() {
                return fail(format!(
                    "surface {} declares no ownership",
                    entry.surface.as_str()
                ));
            }
            if entry.freshness_rule.age_tokens.is_empty() {
                return fail(format!(
                    "surface {} has no freshness rule",
                    entry.surface.as_str()
                ));
            }
            // Every applicable state is a defined vocabulary term.
            for state in &entry.applicable_states {
                if self.state_term(*state).is_none() {
                    return fail(format!(
                        "surface {} references undefined state {}",
                        entry.surface.as_str(),
                        state.as_str()
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("matrix is not support-export safe".to_owned());
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

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical operator-surface matrix.
///
/// Deterministic: the same bytes every call. The invariant `holds` flags are
/// computed from the built surfaces, paths, and states, so an inconsistent edit
/// flips an invariant rather than silently passing.
pub fn operator_surface_matrix() -> OperatorSurfaceMatrix {
    let state_vocabulary = build_state_vocabulary();
    let surfaces = build_surfaces();
    let operator_paths = build_paths();
    let shared_vocabulary = build_shared_vocabulary(&surfaces);
    let invariants = compute_invariants(&surfaces, &operator_paths, &state_vocabulary);

    OperatorSurfaceMatrix {
        record_kind: M5_OPERATOR_SURFACES_RECORD_KIND.to_owned(),
        m5_operator_surfaces_schema_version: M5_OPERATOR_SURFACES_SCHEMA_VERSION,
        schema_ref: M5_OPERATOR_SURFACES_SCHEMA_REF.to_owned(),
        matrix_id: M5_OPERATOR_SURFACES_MATRIX_ID.to_owned(),
        as_of: M5_OPERATOR_SURFACES_AS_OF.to_owned(),
        summary: "One frozen, typed matrix for Aureline's operator-facing overview, triage, \
                  action-plan, handoff, shift-digest, service-ownership, runbook-step, \
                  maintenance, failover, and embedded-boundary surfaces across local, remote, \
                  managed, mirrored/offline, browser/webview, and imported-snapshot paths."
            .to_owned(),
        shared_vocabulary,
        state_vocabulary,
        surfaces,
        operator_paths,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_state_vocabulary() -> Vec<StateTerm> {
    OperatorStateClass::ALL
        .iter()
        .map(|state| StateTerm {
            state: *state,
            token: state.as_str().to_owned(),
            label: state.label().to_owned(),
            blocking_default: state.blocking_default(),
            derived_from_refs: state.derived_from_refs(),
        })
        .collect()
}

fn schema_ops(name: &str) -> String {
    format!("schemas/ops/{name}.schema.json")
}

fn own(field_id: &str, label: &str, required: bool) -> OwnershipField {
    OwnershipField {
        field_id: field_id.to_owned(),
        label: label.to_owned(),
        required,
    }
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

const FRESHNESS_AGE_TOKENS: [&str; 5] = ["fresh", "recent", "stale", "very_stale", "never"];

fn freshness(downgrades_green: bool, rule: &str) -> FreshnessRule {
    FreshnessRule {
        age_tokens: strvec(&FRESHNESS_AGE_TOKENS),
        downgrades_green,
        rule: rule.to_owned(),
    }
}

fn build_surfaces() -> Vec<OperatorSurfaceEntry> {
    use ConsumerClass::*;
    use OperatorStateClass::*;

    vec![
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::OperationalOverviewBoard,
        surface_id: OperatorSurfaceClass::OperationalOverviewBoard.surface_id(),
        label: OperatorSurfaceClass::OperationalOverviewBoard.label().to_owned(),
        summary: "Freshness cards and service-health rows that headline operational state and \
                  downgrade any would-be-green card whose evidence is stale, partial, or cached."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("dashboard_freshness_card"),
            schema_ops("service_health_card"),
            schema_ops("service_contract_state"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/dashboard_truth/model.rs",
            "crates/aureline-service-health/src/lib.rs",
        ]),
        consumed_by: vec![ShellUi, CliHeadless, SupportExport, AdminQueue, IncidentWorkspace],
        applicable_states: vec![
            Clear,
            Unconfirmed,
            Attention,
            Blocked,
            ScheduledWindow,
            Reconciling,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("surface_token", "Surface", true),
            own("displayed_state", "Displayed state", true),
            own("effective_state", "Effective state", true),
            own("evidence_ref", "Evidence object", true),
            own("evidence_age", "Evidence age", true),
        ],
        freshness_rule: freshness(
            true,
            "A card's effective state is clear only when displayed_state is clear, freshness is \
             fresh, and evidence age is fresh or recent; any other combination becomes \
             unconfirmed and lights an honesty marker.",
        ),
        default_redaction: RedactionClass::MetadataSafeDefault,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&["open_evidence", "export_diagnostics", "open_history"]),
        publish_later_capture: false,
        boundary_honest: true,
        boundary_note: "Every card routes its open-details affordance to a canonical object ref; \
                        no card asserts a healthy state it cannot evidence."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::TriageInbox,
        surface_id: OperatorSurfaceClass::TriageInbox.surface_id(),
        label: OperatorSurfaceClass::TriageInbox.label().to_owned(),
        summary: "An ordered queue whose rows name their order and narrowing reasons and open the \
                  same incident/review/support/admin objects the detail surfaces use."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("dashboard_freshness_card"),
            schema_ops("queue_order_reason"),
            schema_ops("incident_workspace"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-shell/src/dashboard_truth/corpus.rs",
            "crates/aureline-support/src/incident_workspace/mod.rs",
        ]),
        consumed_by: vec![ShellUi, CliHeadless, IncidentWorkspace, AdminQueue, SupportExport],
        applicable_states: vec![
            Clear,
            Unconfirmed,
            Attention,
            Blocked,
            Reconciling,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("queue_surface_token", "Queue surface", true),
            own("item_object_ref", "Item object", true),
            own("order_reason", "Order reason", true),
            own("assignee_owner", "Assignee", false),
            own("narrowing_reason", "Narrowing reason", false),
        ],
        freshness_rule: freshness(
            true,
            "Queue order names its order_reason and any narrowing_reason; an offline or partial \
             list is labeled offline_partial_list, never silently truncated.",
        ),
        default_redaction: RedactionClass::MetadataSafeDefault,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&["open_item", "filter_scope", "export_queue_snapshot"]),
        publish_later_capture: true,
        boundary_honest: true,
        boundary_note: "Rows point at the canonical incident/review/support/admin objects; claims \
                        and assignments queue to publish later when writes are blocked."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::ActionPlan,
        surface_id: OperatorSurfaceClass::ActionPlan.surface_id(),
        label: OperatorSurfaceClass::ActionPlan.label().to_owned(),
        summary: "The ordered, ownership-bearing set of inspection and mitigation steps for an \
                  incident or operation, each disclosing its approval admission and outcome."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("runbook_packet"),
            schema_ops("incident_workspace"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-incident/src/lib.rs",
            "crates/aureline-support/src/stabilize_runbook_source_step_envelope_and_handoff_truth/mod.rs",
        ]),
        consumed_by: vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport, ManagedService],
        applicable_states: vec![
            Clear,
            Attention,
            Blocked,
            ReadOnlyWindow,
            DrainWindow,
            Reconciling,
            EmbeddedBoundaryHandoff,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("plan_owner", "Plan owner", true),
            own("decision_right", "Decision right", true),
            own("step_intent", "Step intent", true),
            own("approval_admission", "Approval admission", false),
            own("outcome", "Outcome", false),
        ],
        freshness_rule: freshness(
            false,
            "An imported plan is read-only and labeled; a live plan names each step's intent and \
             whether its outcome is observed or pending.",
        ),
        default_redaction: RedactionClass::OperatorOnlyRestricted,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&[
            "inspect_step",
            "capture_evidence",
            "export_plan_snapshot",
            "draft_step_note",
        ]),
        publish_later_capture: true,
        boundary_honest: true,
        boundary_note: "A mutating step discloses its approval admission and never implies a \
                        managed apply happened locally."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::HandoffBundle,
        surface_id: OperatorSurfaceClass::HandoffBundle.surface_id(),
        label: OperatorSurfaceClass::HandoffBundle.label().to_owned(),
        summary: "A frozen export that preserves scope, freshness, ownership, redaction, and \
                  live-versus-snapshot truth; imported replay bundles declare no live target."
            .to_owned(),
        canonical_schema_refs: vec![schema_ops("evidence_handoff_bundle")],
        produced_by_refs: strvec(&[
            "crates/aureline-support/src/portable_bundle_handoff/mod.rs",
            "crates/aureline-companion/src/lib.rs",
        ]),
        consumed_by: vec![
            ShellUi,
            CliHeadless,
            SupportExport,
            ManagedService,
            CompanionBrowser,
            IncidentWorkspace,
        ],
        applicable_states: vec![
            Clear,
            Blocked,
            EmbeddedBoundaryHandoff,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("bundle_scope", "Bundle scope", true),
            own("evidence_origin", "Evidence origin", true),
            own("retention_owner", "Retention owner", true),
            own("redaction_class", "Redaction class", true),
            own("live_vs_snapshot", "Live vs snapshot", true),
            own("integrity_class", "Integrity class", true),
        ],
        freshness_rule: freshness(
            false,
            "Integrity is frozen, superseded, withdrawn, expired, or unverifiable — never silently \
             rewritten; an imported/replay bundle is labeled no-live-target and read-only.",
        ),
        default_redaction: RedactionClass::MetadataSafeDefault,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&["preview_bundle", "export_bundle", "open_local_review"]),
        publish_later_capture: false,
        boundary_honest: true,
        boundary_note: "Browser/console handoff is an attributable exit, never a silent native \
                        approval; an imported replay bundle declares no live destination."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::ShiftDigest,
        surface_id: OperatorSurfaceClass::ShiftDigest.surface_id(),
        label: OperatorSurfaceClass::ShiftDigest.label().to_owned(),
        summary: "A windowed roll-up of operator events that names its coverage window and labels \
                  gaps rather than implying complete coverage."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("dashboard_freshness_card"),
            schema_ops("event_provenance_row"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-runtime/src/log_metric_slice_and_incident_timeline_contract/mod.rs",
        ]),
        consumed_by: vec![ShellUi, CliHeadless, SupportExport, IncidentWorkspace],
        applicable_states: vec![
            Clear,
            Unconfirmed,
            Attention,
            Reconciling,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("digest_owner", "Digest owner", true),
            own("coverage_window", "Coverage window", true),
            own("event_origin_lane", "Event origin lane", false),
            own("redaction_class", "Redaction class", false),
        ],
        freshness_rule: freshness(
            true,
            "A digest names its coverage window; gaps and partial coverage are labeled rather than \
             implied complete.",
        ),
        default_redaction: RedactionClass::InternalSupportRestricted,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&["open_digest", "export_digest", "open_source_event"]),
        publish_later_capture: false,
        boundary_honest: true,
        boundary_note: "Each digest row carries event provenance and origin lane; nothing claims \
                        more coverage than its window."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::ServiceOwnershipStrip,
        surface_id: OperatorSurfaceClass::ServiceOwnershipStrip.surface_id(),
        label: OperatorSurfaceClass::ServiceOwnershipStrip.label().to_owned(),
        summary: "Who owns a service, who is on call, its contract state, and its local-continuity \
                  posture; a stale last-checked age downgrades the green dot."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("service_health_card"),
            schema_ops("service_contract_state"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-service-health/src/lib.rs",
            "crates/aureline-service-health-feed/src/lib.rs",
        ]),
        consumed_by: vec![ShellUi, CliHeadless, IncidentWorkspace, AdminQueue, SupportExport],
        applicable_states: vec![
            Clear,
            Unconfirmed,
            Attention,
            Blocked,
            ScheduledWindow,
            FailoverInProgress,
            Reconciling,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("service_family", "Service family", true),
            own("on_call_owner", "On-call owner", true),
            own("decision_right", "Decision right", true),
            own("contract_state", "Contract state", false),
            own("local_continuity", "Local continuity", false),
        ],
        freshness_rule: freshness(
            true,
            "The on-call owner is named and a stale last-checked age downgrades the green dot; the \
             strip says when it last verified.",
        ),
        default_redaction: RedactionClass::MetadataSafeDefault,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&[
            "open_service_health",
            "open_owner_contact",
            "open_contract_details",
        ]),
        publish_later_capture: false,
        boundary_honest: true,
        boundary_note: "Local-only services say local_safe; the strip never shows a remote-required \
                        service as locally healthy."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::RunbookStepCard,
        surface_id: OperatorSurfaceClass::RunbookStepCard.surface_id(),
        label: OperatorSurfaceClass::RunbookStepCard.label().to_owned(),
        summary: "One guided response step with its intent, sandbox and approval admission, raw-ref \
                  class, and outcome; mutating and handoff steps are labeled."
            .to_owned(),
        canonical_schema_refs: vec![schema_ops("runbook_packet")],
        produced_by_refs: strvec(&[
            "crates/aureline-support/src/stabilize_runbook_source_step_envelope_and_handoff_truth/mod.rs",
            "crates/aureline-support/src/publish_supportability_runbooks_field_playbooks_and_incident_advisory/mod.rs",
        ]),
        consumed_by: vec![
            ShellUi,
            CliHeadless,
            IncidentWorkspace,
            SupportExport,
            ManagedService,
            CompanionBrowser,
        ],
        applicable_states: vec![
            Clear,
            Attention,
            Blocked,
            ReadOnlyWindow,
            DrainWindow,
            EmbeddedBoundaryHandoff,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("step_owner", "Step owner", true),
            own("step_intent", "Step intent", true),
            own("sandbox_admission", "Sandbox admission", true),
            own("approval_admission", "Approval admission", false),
            own("outcome", "Outcome", false),
            own("raw_ref_class", "Raw-ref class", false),
        ],
        freshness_rule: freshness(
            false,
            "A mutating step states its sandbox and approval admission; a step with no fresh \
             approval is blocked, not silently run.",
        ),
        default_redaction: RedactionClass::OperatorOnlyRestricted,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&[
            "inspect_step",
            "dry_run_in_sandbox",
            "capture_step_evidence",
            "draft_deviation_note",
        ]),
        publish_later_capture: true,
        boundary_honest: true,
        boundary_note: "Browser/console handoff steps are external-only and labeled; a mutating \
                        apply is recorded, never implied."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::MaintenanceNotice,
        surface_id: OperatorSurfaceClass::MaintenanceNotice.surface_id(),
        label: OperatorSurfaceClass::MaintenanceNotice.label().to_owned(),
        summary: "A planned maintenance / read-only / drain window that separates control-plane \
                  effect from local-core data-plane safety and offers publish-later capture."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("maintenance_notice"),
            schema_ops("continuity_notice_view"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-service-health/src/stabilize_maintenance_and_drain_windows/mod.rs",
            "crates/aureline-shell/src/continuity_notices/model.rs",
        ]),
        consumed_by: vec![ShellUi, CliHeadless, SupportExport, ManagedService, IncidentWorkspace],
        applicable_states: vec![
            Clear,
            ScheduledWindow,
            ReadOnlyWindow,
            DrainWindow,
            Reconciling,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("maintenance_owner", "Maintenance owner", true),
            own("window_basis", "Window time basis", true),
            own("control_plane_effect", "Control-plane effect", true),
            own("data_plane_effect", "Data-plane effect", true),
            own("local_core_status", "Local-core status", false),
        ],
        freshness_rule: freshness(
            false,
            "A superseded or completed window is labeled historical; an active window names the \
             read-only/drain effect and the local-safe subset that stays available.",
        ),
        default_redaction: RedactionClass::MetadataSafeDefault,
        scope: ScopeClass::ManagedOrg,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&[
            "continue_local",
            "export_before_maintenance",
            "publish_later",
            "open_continuity_packet",
        ]),
        publish_later_capture: true,
        boundary_honest: true,
        boundary_note: "Local editing and save stay available unless explicitly blocked; the notice \
                        never implies the whole product is down."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::FailoverNotice,
        surface_id: OperatorSurfaceClass::FailoverNotice.surface_id(),
        label: OperatorSurfaceClass::FailoverNotice.label().to_owned(),
        summary: "A failover / migration notice that names the boundary-axis state and refuses \
                  authority-changing actions instead of silently retrying them."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("failover_banner"),
            schema_ops("outage_notice"),
            schema_ops("tenant_migration_event"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-recovery/src/failover_alpha/mod.rs",
            "crates/aureline-shell/src/continuity_notices/mod.rs",
        ]),
        consumed_by: vec![
            ShellUi,
            CliHeadless,
            SupportExport,
            ManagedService,
            IncidentWorkspace,
            AdminQueue,
        ],
        applicable_states: vec![
            Clear,
            Blocked,
            DrainWindow,
            FailoverInProgress,
            MigrationInProgress,
            Reconciling,
            BoundaryDriftRecheckRequired,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("failover_owner", "Failover owner", true),
            own("trigger_kind", "Trigger kind", true),
            own("boundary_axis_state", "Boundary-axis state", true),
            own("continuity_action_state", "Continuity action state", false),
            own("required_user_step", "Required user step", false),
            own("local_core_status", "Local-core status", false),
        ],
        freshness_rule: freshness(
            false,
            "A changed-or-unknown boundary axis requires recheck before managed writes resume; a \
             superseded notice is labeled historical.",
        ),
        default_redaction: RedactionClass::MetadataSafeDefault,
        scope: ScopeClass::ManagedOrg,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: true,
        local_safe_actions: strvec(&[
            "continue_local",
            "export_diagnostics",
            "review_new_boundary",
            "open_continuity_packet",
        ]),
        publish_later_capture: true,
        boundary_honest: true,
        boundary_note: "Authority-changing actions are refused, not silently retried; a changed \
                        boundary is surfaced for explicit review, never auto-accepted."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    OperatorSurfaceEntry {
        surface: OperatorSurfaceClass::EmbeddedBoundaryState,
        surface_id: OperatorSurfaceClass::EmbeddedBoundaryState.surface_id(),
        label: OperatorSurfaceClass::EmbeddedBoundaryState.label().to_owned(),
        summary: "The route, drift, approval source, and verbatim visible language a webview / \
                  browser / auth surface shows; it never impersonates a native approval."
            .to_owned(),
        canonical_schema_refs: vec![
            schema_ops("route_timeline"),
            schema_ops("event_provenance_row"),
        ],
        produced_by_refs: strvec(&[
            "crates/aureline-support/src/route_exposure_beta/mod.rs",
            "crates/aureline-companion/src/lib.rs",
        ]),
        consumed_by: vec![
            ShellUi,
            CliHeadless,
            CompanionBrowser,
            SupportExport,
            IncidentWorkspace,
            ManagedService,
        ],
        applicable_states: vec![
            Clear,
            Blocked,
            EmbeddedBoundaryHandoff,
            BoundaryDriftRecheckRequired,
            ImportedSnapshotNoLive,
            UnknownRequiresReview,
        ],
        ownership_fields: vec![
            own("boundary_class", "Boundary class", true),
            own("approval_source", "Approval source", true),
            own("drift_state", "Route drift state", true),
            own("replay_requirement", "Replay requirement", false),
            own("trust_posture", "Trust posture", false),
        ],
        freshness_rule: freshness(
            false,
            "A drifted route requires review before replay; the surface shows its required visible \
             language verbatim and never impersonates a native approval.",
        ),
        default_redaction: RedactionClass::OperatorOnlyRestricted,
        scope: ScopeClass::SharedTeam,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        captures_user_writes: false,
        local_safe_actions: strvec(&[
            "open_route_details",
            "open_boundary_details",
            "inspect_imported_evidence",
        ]),
        publish_later_capture: false,
        boundary_honest: true,
        boundary_note: "Required visible language is shown verbatim (for example 'Browser-mediated \
                        route'); approval reuse, reapproval triggers, and privacy consequences are \
                        explicit."
            .to_owned(),
        typed_not_screenshot_only: true,
    },
    ]
}

fn build_paths() -> Vec<OperatorPathEntry> {
    use DeploymentProfileClass::*;

    vec![
        OperatorPathEntry {
            path: OperatorPathClass::Local,
            path_id: OperatorPathClass::Local.path_id(),
            label: OperatorPathClass::Local.label().to_owned(),
            summary:
                "Local-first single host: operator surfaces render against local objects with \
                      no control-plane dependency."
                    .to_owned(),
            deployment_profiles: vec![IndividualLocal, SelfHosted],
            default_live_vs_snapshot: LiveSnapshotClass::LiveOnly,
            write_posture: PathWritePostureClass::WritesLive,
            boundary_recheck_required: false,
            local_safe_baseline_ref: schema_ops("local_safe_baseline"),
            notes: "Local editing, save, search, git, build/test, and export stay available; \
                    surfaces label any field that would need a remote source."
                .to_owned(),
        },
        OperatorPathEntry {
            path: OperatorPathClass::Remote,
            path_id: OperatorPathClass::Remote.path_id(),
            label: OperatorPathClass::Remote.label().to_owned(),
            summary: "Remote workspace/runtime attach: surfaces show the remote target and the \
                      route that reached it."
                .to_owned(),
            deployment_profiles: vec![SelfHosted, EnterpriseOnline],
            default_live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
            write_posture: PathWritePostureClass::WritesLive,
            boundary_recheck_required: false,
            local_safe_baseline_ref: schema_ops("local_safe_baseline"),
            notes: "A remote attach names its target and route; losing the route degrades to the \
                    mirrored/offline path rather than silently failing."
                .to_owned(),
        },
        OperatorPathEntry {
            path: OperatorPathClass::Managed,
            path_id: OperatorPathClass::Managed.path_id(),
            label: OperatorPathClass::Managed.label().to_owned(),
            summary: "Managed cloud / control plane: maintenance, failover, and migration notices \
                      apply and managed writes carry approval and boundary state."
                .to_owned(),
            deployment_profiles: vec![EnterpriseOnline, ManagedCloud],
            default_live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
            write_posture: PathWritePostureClass::WritesLive,
            boundary_recheck_required: true,
            local_safe_baseline_ref: schema_ops("local_safe_baseline"),
            notes: "Managed writes require a fresh approval and pass a boundary recheck after any \
                    failover or migration; refused authority-changing actions are never retried \
                    silently."
                .to_owned(),
        },
        OperatorPathEntry {
            path: OperatorPathClass::MirroredOffline,
            path_id: OperatorPathClass::MirroredOffline.path_id(),
            label: OperatorPathClass::MirroredOffline.label().to_owned(),
            summary: "Mirror-backed offline: the last-synced read-only view with freshness labels \
                      and publish-later capture for queued writes."
                .to_owned(),
            deployment_profiles: vec![AirGapped, ManagedCloud, SelfHosted],
            default_live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
            write_posture: PathWritePostureClass::LocalDraftPreserved,
            boundary_recheck_required: true,
            local_safe_baseline_ref: schema_ops("local_safe_baseline"),
            notes:
                "Reads are mirror-backed and labeled by freshness; writes are preserved as local \
                    drafts and queued to publish later, never lost."
                    .to_owned(),
        },
        OperatorPathEntry {
            path: OperatorPathClass::BrowserWebview,
            path_id: OperatorPathClass::BrowserWebview.path_id(),
            label: OperatorPathClass::BrowserWebview.label().to_owned(),
            summary: "Browser / webview-mediated surface: the route is shown as browser-mediated \
                      and the embedded boundary never impersonates a native approval."
                .to_owned(),
            deployment_profiles: vec![EnterpriseOnline, ManagedCloud],
            default_live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
            write_posture: PathWritePostureClass::PublishLaterQueued,
            boundary_recheck_required: true,
            local_safe_baseline_ref: schema_ops("local_safe_baseline"),
            notes:
                "Handoff is an attributable exit with verbatim visible language; a drifted route \
                    requires review before any replay."
                    .to_owned(),
        },
        OperatorPathEntry {
            path: OperatorPathClass::ImportedSnapshot,
            path_id: OperatorPathClass::ImportedSnapshot.path_id(),
            label: OperatorPathClass::ImportedSnapshot.label().to_owned(),
            summary:
                "Imported snapshot: replayed evidence with no live target, rendered read-only \
                      and labeled imported."
                    .to_owned(),
            deployment_profiles: vec![IndividualLocal, SelfHosted, EnterpriseOnline, ManagedCloud],
            default_live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
            write_posture: PathWritePostureClass::ReadOnlyReplay,
            boundary_recheck_required: false,
            local_safe_baseline_ref: schema_ops("local_safe_baseline"),
            notes:
                "Every surface is labeled imported with no live destination; no action targets a \
                    live system from an imported snapshot."
                    .to_owned(),
        },
    ]
}

fn build_shared_vocabulary(surfaces: &[OperatorSurfaceEntry]) -> SharedVocabulary {
    let def = |token: &str, label: &str| TokenDef {
        token: token.to_owned(),
        label: label.to_owned(),
    };

    // The bound source schemas are exactly the union of every surface's cited
    // schema, plus the local-safe baseline the paths lean on.
    let mut source_schema_refs: Vec<String> = surfaces
        .iter()
        .flat_map(|s| s.canonical_schema_refs.iter().cloned())
        .chain(std::iter::once(schema_ops("local_safe_baseline")))
        .collect();
    source_schema_refs.sort();
    source_schema_refs.dedup();

    SharedVocabulary {
        deployment_profiles: vec![
            def("individual_local", "Individual local"),
            def("self_hosted", "Self-hosted"),
            def("enterprise_online", "Enterprise online"),
            def("air_gapped", "Air-gapped"),
            def("managed_cloud", "Managed cloud"),
        ],
        redaction_classes: vec![
            def("metadata_safe_default", "Metadata-safe default"),
            def("operator_only_restricted", "Operator-only restricted"),
            def("internal_support_restricted", "Internal-support restricted"),
            def("signing_evidence_only", "Signing-evidence only"),
            def("private_triage_only", "Private-triage only"),
        ],
        trust_postures: vec![
            def("untrusted", "Untrusted"),
            def("restricted", "Restricted"),
            def("trusted", "Trusted"),
            def("managed_admin", "Managed admin"),
        ],
        scope_classes: vec![
            def("local_private", "Local / private"),
            def("shared_team", "Shared / team"),
            def("managed_org", "Managed / org"),
        ],
        live_snapshot_classes: vec![
            def("live_only", "Live only"),
            def("snapshot_capable", "Snapshot-capable"),
            def("snapshot_only", "Snapshot only"),
        ],
        consumer_classes: vec![
            def("shell_ui", "Shell UI"),
            def("cli_headless", "CLI / headless"),
            def("incident_workspace", "Incident workspace"),
            def("support_export", "Support export"),
            def("admin_queue", "Admin queue"),
            def("release_evidence", "Release evidence"),
            def("managed_service", "Managed service"),
            def("companion_browser", "Companion / browser"),
        ],
        boundary_axes: vec![
            def("tenant", "Tenant"),
            def("region", "Region"),
            def("residency", "Residency"),
            def("key_ownership", "Key ownership"),
            def("endpoint_identity", "Endpoint identity"),
        ],
        source_schema_refs,
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> MatrixInvariant {
    MatrixInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    surfaces: &[OperatorSurfaceEntry],
    paths: &[OperatorPathEntry],
    states: &[StateTerm],
) -> Vec<MatrixInvariant> {
    use OperatorStateClass::*;

    let mut out = Vec::new();

    // Every surface points at a canonical object and a producer.
    out.push(invariant(
        "operator_surfaces.canonical_object_identity",
        "Every surface cites at least one canonical schemas/ops boundary schema and at least one \
         producing crate, so dashboards and queues point at the same underlying objects.",
        surfaces
            .iter()
            .all(|s| !s.canonical_schema_refs.is_empty() && !s.produced_by_refs.is_empty()),
    ));

    // No-silent-green: every freshness-headlined surface carries the unconfirmed
    // downgrade and downgrades green.
    let green_headlined = [
        OperatorSurfaceClass::OperationalOverviewBoard,
        OperatorSurfaceClass::TriageInbox,
        OperatorSurfaceClass::ShiftDigest,
        OperatorSurfaceClass::ServiceOwnershipStrip,
    ];
    out.push(invariant(
        "operator_surfaces.no_silent_green",
        "Every freshness-headlined surface carries the unconfirmed state and a freshness rule that \
         downgrades a would-be-green headline when its evidence is stale, partial, or cached.",
        green_headlined.iter().all(|class| {
            surfaces
                .iter()
                .find(|s| s.surface == *class)
                .is_some_and(|s| {
                    s.applicable_states.contains(&Unconfirmed) && s.freshness_rule.downgrades_green
                })
        }),
    ));

    // Ownership and decision-rights stay visible.
    out.push(invariant(
        "operator_surfaces.ownership_visible",
        "Every surface declares at least one required ownership/decision-right field.",
        surfaces
            .iter()
            .all(|s| s.ownership_fields.iter().any(|f| f.required)),
    ));

    // Freshness stays visible.
    out.push(invariant(
        "operator_surfaces.freshness_visible",
        "Every surface declares a non-empty freshness rule.",
        surfaces
            .iter()
            .all(|s| !s.freshness_rule.age_tokens.is_empty() && !s.freshness_rule.rule.is_empty()),
    ));

    // Local-safe actions during read-only/drain windows, with publish-later
    // capture for write-bearing surfaces.
    out.push(invariant(
        "operator_surfaces.local_safe_during_windows",
        "Every surface that can show a read-only or drain window keeps local-safe actions \
         available, and every write-bearing one of those offers publish-later/draft capture.",
        surfaces.iter().all(|s| {
            let in_window = s.applicable_states.contains(&ReadOnlyWindow)
                || s.applicable_states.contains(&DrainWindow);
            if !in_window {
                return true;
            }
            if s.local_safe_actions.is_empty() {
                return false;
            }
            !s.captures_user_writes || s.publish_later_capture
        }),
    ));

    // Boundary honesty: embedded/handoff surfaces never impersonate native
    // approvals.
    out.push(invariant(
        "operator_surfaces.boundary_honest_no_impersonation",
        "Every surface that can show an embedded browser/auth handoff is boundary-honest and \
         carries a stated boundary-honesty rule.",
        surfaces.iter().all(|s| {
            if !s.applicable_states.contains(&EmbeddedBoundaryHandoff) {
                return true;
            }
            s.boundary_honest && !s.boundary_note.is_empty()
        }),
    ));

    // Handoff bundles preserve scope/freshness/ownership/redaction/live-vs-snapshot.
    let handoff = surfaces
        .iter()
        .find(|s| s.surface == OperatorSurfaceClass::HandoffBundle);
    out.push(invariant(
        "operator_surfaces.handoff_truth_preserved",
        "The handoff-bundle surface preserves scope, freshness, ownership, redaction, and \
         live-versus-snapshot truth and labels imported replay bundles as having no live target.",
        handoff.is_some_and(|s| {
            let has = |id: &str| {
                s.ownership_fields
                    .iter()
                    .any(|f| f.field_id == id && f.required)
            };
            has("bundle_scope")
                && has("retention_owner")
                && has("redaction_class")
                && has("live_vs_snapshot")
                && !s.freshness_rule.age_tokens.is_empty()
                && s.applicable_states.contains(&ImportedSnapshotNoLive)
        }),
    ));

    // Stable ids and tokens defined once and unique.
    out.push(invariant(
        "operator_surfaces.stable_ids_unique",
        "Surface ids, path ids, and state tokens are each defined once and unique, so consumers can \
         resolve a surface, path, or state by a stable token.",
        all_unique(surfaces.iter().map(|s| s.surface_id.as_str()))
            && all_unique(paths.iter().map(|p| p.path_id.as_str()))
            && all_unique(states.iter().map(|t| t.token.as_str())),
    ));

    // Every operator path is covered.
    out.push(invariant(
        "operator_surfaces.all_paths_covered",
        "The matrix covers local, remote, managed, mirrored/offline, browser/webview, and \
         imported-snapshot operator paths.",
        OperatorPathClass::ALL
            .iter()
            .all(|class| paths.iter().any(|p| p.path == *class)),
    ));

    // Every surface family is present.
    out.push(invariant(
        "operator_surfaces.all_surfaces_present",
        "Every operator-surface family in the matrix is present exactly once.",
        OperatorSurfaceClass::ALL
            .iter()
            .all(|class| surfaces.iter().filter(|s| s.surface == *class).count() == 1),
    ));

    // Typed, never screenshot-only.
    out.push(invariant(
        "operator_surfaces.typed_not_screenshot_only",
        "Every surface is typed: it carries state terms and schema refs and is never reduced to a \
         screenshot or generic outage prose.",
        surfaces.iter().all(|s| {
            s.typed_not_screenshot_only
                && !s.applicable_states.is_empty()
                && !s.canonical_schema_refs.is_empty()
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the matrix as human-readable lines for CLI/headless and support.
pub fn operator_surface_lines(matrix: &OperatorSurfaceMatrix) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator-surface matrix — {} ({})",
        matrix.matrix_id, matrix.as_of
    ));
    lines.push(matrix.summary.clone());
    lines.push(format!(
        "Surfaces: {}  Paths: {}  States: {}",
        matrix.surfaces.len(),
        matrix.operator_paths.len(),
        matrix.state_vocabulary.len()
    ));

    lines.push("Surfaces:".to_owned());
    for s in &matrix.surfaces {
        let states: Vec<&str> = s.applicable_states.iter().map(|st| st.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] scope={} live={:?} redaction={:?}",
            s.surface.as_str(),
            s.surface_id,
            scope_token(s.scope),
            s.live_vs_snapshot,
            s.default_redaction,
        ));
        lines.push(format!("      {}", s.summary));
        lines.push(format!("      states: {}", states.join(", ")));
        lines.push(format!(
            "      schemas: {}",
            s.canonical_schema_refs.join(", ")
        ));
        if !s.local_safe_actions.is_empty() {
            lines.push(format!(
                "      local-safe: {} (publish-later: {})",
                s.local_safe_actions.join(", "),
                s.publish_later_capture
            ));
        }
    }

    lines.push("Paths:".to_owned());
    for p in &matrix.operator_paths {
        lines.push(format!(
            "  - {} [{}] write={:?} boundary_recheck={}",
            p.path.as_str(),
            p.path_id,
            p.write_posture,
            p.boundary_recheck_required
        ));
        lines.push(format!("      {}", p.summary));
    }

    lines.push("Invariants:".to_owned());
    for i in &matrix.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}

fn scope_token(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::LocalPrivate => "local_private",
        ScopeClass::SharedTeam => "shared_team",
        ScopeClass::ManagedOrg => "managed_org",
    }
}
