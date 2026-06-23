//! M5 *durable activity objects and activity-center rows*: the working durable job
//! model every long-running, retryable, or reviewable M5 work item becomes, and the
//! deterministic projection that renders it as an activity-center row across the
//! shell, support export, companion, and operator surfaces.
//!
//! Where [`m5_attention_routing`](crate::m5_attention_routing) *names and freezes
//! the contract* — it declares the
//! [`ActivityObject`](crate::m5_attention_routing::AttentionObjectClass::ActivityObject)
//! family, its required fields, its applicable states, its reopen targets, and its
//! `durable_until_archived` retention rule — this lane *implements that object
//! family*. Every claimed M5 job family (notebook runs, task / CI jobs, AI / agent
//! runs, preview routes, pipeline actions, continuity sync, offboarding, operator
//! handoffs, and managed alerts) becomes one typed [`ActivityObject`] instead of a
//! transient spinner or completion toast, and a single pure [`render_row`] projects
//! it into an [`ActivityRow`] the activity center, support export, companion
//! summary, operator dashboard, and CLI/headless surfaces all consume.
//!
//! Each activity object carries exactly what the spec requires as the contract: a
//! stable job id, the actor subsystem, a coarse [`ActivityPhaseClass`] and a typed
//! [`AttentionStateClass`](crate::m5_attention_routing::AttentionStateClass)
//! progress state, cancel / retry / open-details affordances, evidence links,
//! cost / trust / policy-impact flags, created / updated stamps, and an
//! [`ArchiveStateClass`] derived from a frozen [`ActivityRetentionPolicy`]. Title
//! and action copy is carried as localizable keys, never raw bodies.
//!
//! The honesty rules the track invariant requires are enforced, not just described:
//!
//! - **No spinner- or toast-only truth.** Every object is a durable authoritative
//!   record that survives focus change and restart and reopens its authoritative
//!   object; none is reduced to an ephemeral toast
//!   (`activity.durable_never_toast_only`, `activity.reopen_target_authoritative`).
//! - **Completion and failure history is preserved.** Terminal rows are retained
//!   until archived or expired by policy, never dropped into transient chrome
//!   (`activity.failure_completion_history_retained`).
//! - **Archive / expiry is one shared truth.** A row reports the same archive state
//!   on every surface — desktop, support export, companion, and operator — so the
//!   retention disposition is testable and consistent across clients
//!   (`activity.archive_state_shared_across_surfaces`).
//! - **Privacy never widens on a surface.** Each surface projection applies a
//!   redaction at least as strong as the object default and the surface floor, and
//!   managed-sensitive rows never fan out to the companion
//!   (`activity.privacy_never_widens_on_surface`).
//! - **Badges derive from durable items.** Only durable, attention-pending rows
//!   count toward the badge; nothing toast-only contributes
//!   (`activity.badge_from_durable_items`).
//!
//! The canonical [`activity_objects_bundle`] freezes the job-family registry, the
//! activity-object corpus, every rendered row, and every invariant so the freeze
//! gate and checked-in fixture pin the contract byte-for-byte. Every progress
//! state, reopen target, privacy class, and retention class the bundle uses is one
//! the attention-routing matrix defines, so the working object model can never
//! drift from the frozen contract (`activity.matrix_bound`).
//!
//! The record carries no message bodies, credentials, raw provider payloads,
//! hostnames, or absolute paths — only opaque object refs, localizable copy keys,
//! stable tokens, and short reviewable sentences — so it is safe to embed in a
//! support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{
    all_unique, attention_routing_matrix, is_export_safe_ref, AttentionConsumerClass,
    AttentionObjectClass, AttentionRedactionClass, AttentionScopeClass, AttentionStateClass,
    NotificationPrivacyClass, ReopenTargetClass, M5_ATTENTION_ROUTING_MATRIX_ID,
};
use crate::m5_envelope_routing::SourceSubsystemClass;

#[cfg(test)]
mod tests;

/// Schema version for the activity-objects bundle.
pub const M5_ACTIVITY_OBJECTS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the activity-objects bundle.
pub const M5_ACTIVITY_OBJECTS_SCHEMA_REF: &str = "schemas/activity/m5-activity-objects.schema.json";

/// Stable record-kind tag for the activity-objects bundle.
pub const M5_ACTIVITY_OBJECTS_RECORD_KIND: &str = "m5_activity_objects_bundle";

/// Stable id for the canonical activity-objects bundle.
pub const M5_ACTIVITY_OBJECTS_BUNDLE_ID: &str = "m5-activity-objects:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte. Each object's
/// `age_days` is the retention clock: the number of days between the object's
/// `updated_at` and this stamp.
pub const M5_ACTIVITY_OBJECTS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The attention-routing matrix fixture this lane binds its vocabulary back to.
pub const M5_ACTIVITY_OBJECTS_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

/// The freeze gate that keeps the bundle current. Stable promotion runs this gate;
/// it fails when the in-code bundle drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_ACTIVITY_OBJECTS_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_activity_objects.rs";

// ---------------------------------------------------------------------------
// Job families.
// ---------------------------------------------------------------------------

/// The closed set of M5 job families that become durable activity objects.
///
/// Every claimed M5 long-running, retryable, or reviewable work item maps to
/// exactly one of these. Adding a family is a breaking change to the registry; the
/// tokens are frozen here so a consumer can resolve a family by a stable token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobFamilyClass {
    /// A notebook cell or notebook execution.
    Notebook,
    /// A long-running task-runner job, CI run, or batch job.
    Task,
    /// An AI / agent / composer run.
    AiRun,
    /// A preview route render or navigation that must stay reviewable.
    PreviewRoute,
    /// A pipeline action (build, publish, deploy step).
    PipelineAction,
    /// A continuity sync: backup, restore, or failover.
    Sync,
    /// A workspace / account offboarding job.
    Offboarding,
    /// An operator / admin handoff awaiting review.
    OperatorHandoff,
    /// A managed-policy alert or entitlement change.
    ManagedAlert,
}

impl JobFamilyClass {
    /// All job families, in registry order.
    pub const ALL: [Self; 9] = [
        Self::Notebook,
        Self::Task,
        Self::AiRun,
        Self::PreviewRoute,
        Self::PipelineAction,
        Self::Sync,
        Self::Offboarding,
        Self::OperatorHandoff,
        Self::ManagedAlert,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::Task => "task",
            Self::AiRun => "ai_run",
            Self::PreviewRoute => "preview_route",
            Self::PipelineAction => "pipeline_action",
            Self::Sync => "sync",
            Self::Offboarding => "offboarding",
            Self::OperatorHandoff => "operator_handoff",
            Self::ManagedAlert => "managed_alert",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Notebook => "Notebook run",
            Self::Task => "Task / CI job",
            Self::AiRun => "AI / agent run",
            Self::PreviewRoute => "Preview route",
            Self::PipelineAction => "Pipeline action",
            Self::Sync => "Continuity sync",
            Self::Offboarding => "Offboarding",
            Self::OperatorHandoff => "Operator handoff",
            Self::ManagedAlert => "Managed alert",
        }
    }

    /// The actor subsystem that owns this family's work.
    pub const fn actor_subsystem(self) -> SourceSubsystemClass {
        match self {
            Self::Notebook => SourceSubsystemClass::Notebook,
            Self::Task => SourceSubsystemClass::TaskRunner,
            Self::AiRun => SourceSubsystemClass::Ai,
            Self::PreviewRoute => SourceSubsystemClass::Shell,
            Self::PipelineAction => SourceSubsystemClass::TaskRunner,
            Self::Sync => SourceSubsystemClass::Sync,
            Self::Offboarding => SourceSubsystemClass::Sync,
            Self::OperatorHandoff => SourceSubsystemClass::Operator,
            Self::ManagedAlert => SourceSubsystemClass::ManagedPolicy,
        }
    }

    /// The authoritative object a row of this family reopens.
    pub const fn default_reopen_target(self) -> ReopenTargetClass {
        match self {
            Self::Notebook | Self::Task | Self::PipelineAction | Self::Sync => {
                ReopenTargetClass::ActivityJobRow
            }
            Self::AiRun | Self::OperatorHandoff => ReopenTargetClass::ReviewRequest,
            Self::PreviewRoute => ReopenTargetClass::RouteObject,
            Self::Offboarding | Self::ManagedAlert => ReopenTargetClass::EvidencePacket,
        }
    }

    /// The default privacy class this family's rows render under.
    pub const fn default_privacy(self) -> NotificationPrivacyClass {
        match self {
            Self::OperatorHandoff | Self::ManagedAlert => {
                NotificationPrivacyClass::ManagedSensitive
            }
            _ => NotificationPrivacyClass::WorkspaceSensitive,
        }
    }

    /// The default redaction posture on export and out-of-window surfaces.
    pub const fn default_redaction(self) -> AttentionRedactionClass {
        match self {
            Self::OperatorHandoff | Self::ManagedAlert => {
                AttentionRedactionClass::InternalSupportRestricted
            }
            _ => AttentionRedactionClass::MetadataSafeDefault,
        }
    }

    /// Whether this family produces long-running work (always true: every family in
    /// this lane is admitted because it can become long-running, retryable, or
    /// evidence-bearing).
    pub const fn long_running(self) -> bool {
        true
    }

    /// Whether a failed or partial run of this family can be retried.
    pub const fn retryable(self) -> bool {
        !matches!(
            self,
            Self::PreviewRoute | Self::Offboarding | Self::OperatorHandoff
        )
    }

    /// Whether this family carries evidence links (always true here).
    pub const fn evidence_bearing(self) -> bool {
        true
    }

    /// Whether this family's rows can offer an in-product review/approve action.
    pub const fn needs_review(self) -> bool {
        matches!(self, Self::AiRun | Self::OperatorHandoff)
    }

    /// The retention policy this family applies. Managed families are retained
    /// longer for compliance.
    pub fn retention(self) -> ActivityRetentionPolicy {
        match self {
            Self::OperatorHandoff | Self::ManagedAlert => ActivityRetentionPolicy {
                retention_class: RETENTION_CLASS.to_owned(),
                archive_after_days: 90,
                expire_after_days: 365,
                separate_suppression_from_history: true,
                rule: "The managed activity object is the durable authoritative record: it is \
                       retained for compliance, archived after 90 days and expired after 365, and \
                       its suppression markers never overwrite its history."
                    .to_owned(),
            },
            _ => ActivityRetentionPolicy {
                retention_class: RETENTION_CLASS.to_owned(),
                archive_after_days: 30,
                expire_after_days: 180,
                separate_suppression_from_history: true,
                rule: "The activity object is the durable authoritative record: it survives focus \
                       loss and restart, is archived after 30 days and expired after 180, and is \
                       never reduced to a toast."
                    .to_owned(),
            },
        }
    }
}

/// The retention class every activity object applies; mirrors the attention-routing
/// matrix's `ActivityObject` retention class.
pub const RETENTION_CLASS: &str = "durable_until_archived";

// ---------------------------------------------------------------------------
// Phase, archive state, and affordances.
// ---------------------------------------------------------------------------

/// The coarse lifecycle phase of an activity object, distinct from its fine-grained
/// [`AttentionStateClass`](crate::m5_attention_routing::AttentionStateClass)
/// progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityPhaseClass {
    /// Queued or waiting on a dependency; not yet started.
    Queued,
    /// Actively running or finalizing.
    Running,
    /// Paused for human review; needs a person.
    Review,
    /// Terminal: completed, failed, resolved, or archived.
    Settled,
}

impl ActivityPhaseClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Review => "review",
            Self::Settled => "settled",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Running => "Running",
            Self::Review => "Awaiting review",
            Self::Settled => "Settled",
        }
    }
}

/// The phase implied by a progress state. Phase is always derived from progress so
/// the two can never disagree.
pub fn phase_for(progress: AttentionStateClass) -> ActivityPhaseClass {
    use AttentionStateClass::*;
    match progress {
        QueuedWaiting => ActivityPhaseClass::Queued,
        Running | PartiallyCompleted => ActivityPhaseClass::Running,
        UnknownRequiresReview => ActivityPhaseClass::Review,
        _ => ActivityPhaseClass::Settled,
    }
}

/// The retention disposition of a durable activity object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveStateClass {
    /// Live or recent: kept as an active durable record.
    Active,
    /// Moved into durable history by policy; reopenable but no longer active.
    Archived,
    /// Expired by policy; kept only as a tombstone reference.
    Expired,
}

impl ArchiveStateClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
            Self::Expired => "expired",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Archived => "Archived",
            Self::Expired => "Expired",
        }
    }

    /// Whether the full durable record is still retained (active or archived) rather
    /// than reduced to a tombstone (expired).
    pub const fn retains_full_record(self) -> bool {
        !matches!(self, Self::Expired)
    }
}

/// Whether a progress state is terminal for retention purposes — only terminal
/// states age toward archive / expiry.
pub fn is_terminal_for_retention(progress: AttentionStateClass) -> bool {
    matches!(
        progress,
        AttentionStateClass::Completed
            | AttentionStateClass::Failed
            | AttentionStateClass::Resolved
    )
}

/// Whether a progress state is attention-pending and therefore badge-bearing.
pub fn is_attention_pending(progress: AttentionStateClass) -> bool {
    matches!(
        progress,
        AttentionStateClass::Running
            | AttentionStateClass::QueuedWaiting
            | AttentionStateClass::PartiallyCompleted
            | AttentionStateClass::Failed
            | AttentionStateClass::UnknownRequiresReview
    )
}

/// Computes the archive state from a progress state, retention policy, and age.
///
/// Pure and deterministic: non-terminal work is always [`ArchiveStateClass::Active`];
/// terminal work ages from active to archived to expired by the policy thresholds.
pub fn archive_state_for(
    progress: AttentionStateClass,
    retention: &ActivityRetentionPolicy,
    age_days: u32,
) -> ArchiveStateClass {
    if !is_terminal_for_retention(progress) {
        return ArchiveStateClass::Active;
    }
    if age_days >= retention.expire_after_days {
        ArchiveStateClass::Expired
    } else if age_days >= retention.archive_after_days {
        ArchiveStateClass::Archived
    } else {
        ArchiveStateClass::Active
    }
}

/// One affordance an activity row offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityAffordanceClass {
    /// Cancel in-flight work.
    Cancel,
    /// Retry failed or partial work.
    Retry,
    /// Open / reopen the authoritative object — always available.
    OpenDetails,
    /// Review and approve through the in-product preview/approval flow.
    ReviewApprove,
    /// Acknowledge — mark read, keep the durable record.
    Acknowledge,
    /// Archive the durable record ahead of policy.
    Archive,
}

impl ActivityAffordanceClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::Retry => "retry",
            Self::OpenDetails => "open_details",
            Self::ReviewApprove => "review_approve",
            Self::Acknowledge => "acknowledge",
            Self::Archive => "archive",
        }
    }
}

/// Computes the affordances a row offers from its family and progress state.
///
/// Cancel is offered only while work is in flight; retry only for failed or partial
/// work of a retryable family; open-details is always present (every surface can
/// reopen the authoritative object); review/approve only for review-bearing families
/// awaiting review; acknowledge only on a clean terminal state.
pub fn affordances_for(
    family: JobFamilyClass,
    progress: AttentionStateClass,
) -> Vec<ActivityAffordanceClass> {
    use ActivityAffordanceClass::*;
    use AttentionStateClass::*;
    let in_flight = matches!(progress, Running | QueuedWaiting | PartiallyCompleted);
    let failed_or_partial = matches!(progress, Failed | PartiallyCompleted);

    let mut out = Vec::new();
    if in_flight {
        out.push(Cancel);
    }
    if failed_or_partial && family.retryable() {
        out.push(Retry);
    }
    // Open-details is always available: every surface reopens the authoritative
    // object rather than reissuing a blind side effect.
    out.push(OpenDetails);
    if family.needs_review() && matches!(progress, UnknownRequiresReview) {
        out.push(ReviewApprove);
    }
    if matches!(progress, Completed | Resolved) {
        out.push(Acknowledge);
        out.push(Archive);
    }
    out
}

// ---------------------------------------------------------------------------
// Activity object record.
// ---------------------------------------------------------------------------

/// The cost / trust / policy-impact flags an activity object can raise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityFlags {
    /// Whether the job incurs metered or billable cost.
    pub cost_flag: bool,
    /// Whether the job touched untrusted input and needs trust review.
    pub trust_flag: bool,
    /// Whether the job changes policy, permissions, or managed state.
    pub policy_impact_flag: bool,
}

/// The phase / progress state of an activity object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityProgress {
    /// The coarse lifecycle phase (derived from the progress state).
    pub phase: ActivityPhaseClass,
    /// The typed progress state from the attention-routing matrix vocabulary.
    pub progress_state: AttentionStateClass,
    /// Progress in per-mille (0..=1000) when determinate; 0 when indeterminate.
    pub progress_permille: u16,
    /// Whether the progress measure is determinate.
    pub determinate: bool,
}

/// The frozen retention policy an activity object applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRetentionPolicy {
    /// The retention class token (matches the matrix `ActivityObject` retention).
    pub retention_class: String,
    /// Days after a terminal state before the record is archived.
    pub archive_after_days: u32,
    /// Days after a terminal state before the record is expired.
    pub expire_after_days: u32,
    /// Whether suppression / quiet-hours markers on this object are stored
    /// separately from its durable history rather than overwriting it.
    pub separate_suppression_from_history: bool,
    /// One reviewable sentence stating the rule.
    pub rule: String,
}

/// The authoritative object an activity row reopens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenAnchor {
    /// The authoritative reopen target.
    pub reopen_target: ReopenTargetClass,
    /// The opaque object ref the reopen resolves to (never a URL, host, or path).
    pub object_ref: String,
    /// The localizable label key for the reopen action (never raw copy).
    pub label_key: String,
}

/// The typed, durable unit of long-running, retryable, or reviewable M5 work.
///
/// This is the working record behind the
/// [`ActivityObject`](crate::m5_attention_routing::AttentionObjectClass::ActivityObject)
/// family the matrix names. It carries the required fields the spec makes the
/// contract — job id, actor subsystem, phase / progress, cancel / retry
/// affordances, evidence links, cost / trust / policy flags, created / updated
/// stamps, and archive / expiry state — and is the durable authoritative record,
/// never a transient spinner or toast.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityObject {
    /// Stable, namespaced job id.
    pub activity_job_id: String,
    /// The job family.
    pub job_family: JobFamilyClass,
    /// The actor subsystem that owns the work.
    pub actor_subsystem: SourceSubsystemClass,
    /// The localizable title key (never raw copy).
    pub title_key: String,
    /// The scope namespace the work applies to.
    pub scope: AttentionScopeClass,
    /// The opaque scope object ref.
    pub scope_ref: String,
    /// The privacy class governing what may be shown, mirrored, or exported.
    pub privacy_class: NotificationPrivacyClass,
    /// The default redaction posture on export and out-of-window surfaces.
    pub default_redaction: AttentionRedactionClass,
    /// The phase / progress state.
    pub progress: ActivityProgress,
    /// Whether a cancel affordance is offered now.
    pub can_cancel: bool,
    /// Whether a retry affordance is offered now.
    pub can_retry: bool,
    /// Whether an open-details (reopen) affordance is offered (always true).
    pub can_open_details: bool,
    /// The affordances offered now, derived from family and state.
    pub affordances: Vec<ActivityAffordanceClass>,
    /// The evidence links behind the work (opaque object refs).
    pub evidence_refs: Vec<String>,
    /// The reopen anchor back to the authoritative object.
    pub reopen_anchor: ReopenAnchor,
    /// The cost / trust / policy-impact flags.
    pub flags: ActivityFlags,
    /// Creation stamp (frozen; equal to `updated_at` in this corpus).
    pub created_at: String,
    /// Last-activity stamp.
    pub updated_at: String,
    /// Retention clock: days between `updated_at` and the bundle `as_of`.
    pub age_days: u32,
    /// The retention policy this object applies.
    pub retention: ActivityRetentionPolicy,
    /// The archive / expiry disposition, derived from progress, retention, and age.
    pub archive_state: ArchiveStateClass,
    /// Whether the object is a durable authoritative record (always true).
    pub durable: bool,
    /// Whether the object survives focus change and restart (always true).
    pub survives_focus_change: bool,
    /// Whether the object lives only in an ephemeral toast (always false).
    pub toast_only: bool,
}

impl ActivityObject {
    /// Whether this object contributes to the pending-attention badge: a durable,
    /// active, attention-pending row.
    pub fn badge_bearing(&self) -> bool {
        self.durable
            && !self.toast_only
            && self.archive_state == ArchiveStateClass::Active
            && is_attention_pending(self.progress.progress_state)
    }
}

// ---------------------------------------------------------------------------
// Activity-center row projection.
// ---------------------------------------------------------------------------

/// One surface projection of an activity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRowProjection {
    /// The consumer surface.
    pub consumer: AttentionConsumerClass,
    /// Stable consumer token.
    pub consumer_token: String,
    /// Whether the row is included on this surface.
    pub included: bool,
    /// The archive state shown — identical across every surface of a row.
    pub archive_state: ArchiveStateClass,
    /// The redaction applied on this surface.
    pub applied_redaction: AttentionRedactionClass,
    /// The affordances shown on this surface.
    pub shown_affordances: Vec<ActivityAffordanceClass>,
    /// One reviewable sentence explaining the projection.
    pub reason: String,
}

/// The rendered activity-center row for an activity object, with its per-surface
/// projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityRow {
    /// Stable, namespaced row id.
    pub activity_row_id: String,
    /// The activity job id this row renders.
    pub activity_job_id: String,
    /// The job family.
    pub job_family: JobFamilyClass,
    /// The stable status-label token (the progress-state token).
    pub status_label_token: String,
    /// The reopen target id the primary action resolves to.
    pub reopen_target_id: String,
    /// The reopen anchor the row's primary action uses.
    pub reopen_anchor: ReopenAnchor,
    /// The archive state (the one shared truth across surfaces).
    pub archive_state: ArchiveStateClass,
    /// Whether the row counts toward the pending-attention badge.
    pub badge_counts_toward: bool,
    /// Whether the row is backed by a durable authoritative object (always true).
    pub durable: bool,
    /// The per-surface projections.
    pub surface_projections: Vec<ActivityRowProjection>,
}

impl ActivityRow {
    /// The projection for a consumer, if present.
    pub fn projection(&self, consumer: AttentionConsumerClass) -> Option<&ActivityRowProjection> {
        self.surface_projections
            .iter()
            .find(|p| p.consumer == consumer)
    }
}

/// The consumer surfaces an activity row is projected onto, in canonical order.
const ROW_CONSUMERS: [AttentionConsumerClass; 5] = [
    AttentionConsumerClass::ShellActivityCenter,
    AttentionConsumerClass::CliHeadless,
    AttentionConsumerClass::SupportExport,
    AttentionConsumerClass::CompanionCrossClient,
    AttentionConsumerClass::OperatorDashboard,
];

/// Renders an activity object into its activity-center row, deterministically.
///
/// Pure: the same object yields the same [`ActivityRow`] every call. The archive
/// state is one shared truth on every surface; redaction is raised to each
/// surface's floor; managed-sensitive rows never fan out to the companion, and the
/// shell holds the full affordance set while every other surface offers an
/// open-details (reopen) affordance.
pub fn render_row(object: &ActivityObject) -> ActivityRow {
    let projections = ROW_CONSUMERS
        .iter()
        .map(|consumer| project_row(object, *consumer))
        .collect();

    ActivityRow {
        activity_row_id: format!("activity_row:{}", object.activity_job_id),
        activity_job_id: object.activity_job_id.clone(),
        job_family: object.job_family,
        status_label_token: object.progress.progress_state.as_str().to_owned(),
        reopen_target_id: object.reopen_anchor.reopen_target.as_str().to_owned(),
        reopen_anchor: object.reopen_anchor.clone(),
        archive_state: object.archive_state,
        badge_counts_toward: object.badge_bearing(),
        durable: object.durable,
        surface_projections: projections,
    }
}

fn project_row(object: &ActivityObject, consumer: AttentionConsumerClass) -> ActivityRowProjection {
    use AttentionConsumerClass::*;

    let is_managed = object.privacy_class == NotificationPrivacyClass::ManagedSensitive;
    let is_org_scope = object.scope == AttentionScopeClass::TenantOrg;
    let is_managed_family = matches!(
        object.job_family,
        JobFamilyClass::OperatorHandoff | JobFamilyClass::ManagedAlert
    );
    let expired = object.archive_state == ArchiveStateClass::Expired;

    let (included, floor, reason): (bool, AttentionRedactionClass, &str) = match consumer {
        ShellActivityCenter => (
            true,
            AttentionRedactionClass::MetadataSafeDefault,
            "The in-product activity center is the authoritative surface and always renders the \
             durable row with its full affordances.",
        ),
        CliHeadless => (
            true,
            AttentionRedactionClass::MetadataSafeDefault,
            "CLI / headless inspect renders the durable row so the same truth is reproducible \
             without the UI.",
        ),
        SupportExport => (
            true,
            AttentionRedactionClass::MetadataSafeDefault,
            "Support export includes the durable row — active, archived, or as an expired \
             tombstone — so completion and failure history is reviewable.",
        ),
        CompanionCrossClient => {
            if is_managed {
                (
                    false,
                    AttentionRedactionClass::InternalSupportRestricted,
                    "Managed-sensitive rows stay in-product and on the operator dashboard; they \
                     never fan out to the companion.",
                )
            } else if expired {
                (
                    false,
                    AttentionRedactionClass::SummaryOnly,
                    "Expired rows are kept only as an in-product tombstone and are not mirrored to \
                     the companion.",
                )
            } else {
                (
                    true,
                    AttentionRedactionClass::SummaryOnly,
                    "The companion mirrors a redacted summary of the durable row so cross-client \
                     awareness reopens the authoritative object in-product.",
                )
            }
        }
        OperatorDashboard => {
            if is_org_scope || is_managed_family {
                (
                    true,
                    AttentionRedactionClass::InternalSupportRestricted,
                    "The operator dashboard renders org-scoped and managed rows as a read-only \
                     managed view.",
                )
            } else {
                (
                    false,
                    AttentionRedactionClass::InternalSupportRestricted,
                    "Workspace-private rows are not shown on the operator dashboard.",
                )
            }
        }
        // Surfaces outside ROW_CONSUMERS are not projected here.
        OsNotification | HelpAbout => (
            false,
            AttentionRedactionClass::SummaryOnly,
            "Not a row-rendering surface in this lane.",
        ),
    };

    let applied_redaction = stronger_redaction(object.default_redaction, floor);
    let shown_affordances = if consumer == ShellActivityCenter {
        object.affordances.clone()
    } else {
        // Every other surface reopens the authoritative object rather than acting
        // inline.
        vec![ActivityAffordanceClass::OpenDetails]
    };

    ActivityRowProjection {
        consumer,
        consumer_token: consumer_token(consumer).to_owned(),
        included,
        // The archive state is one shared truth on every surface.
        archive_state: object.archive_state,
        applied_redaction,
        shown_affordances,
        reason: reason.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Redaction / consumer helpers.
// ---------------------------------------------------------------------------

fn redaction_rank(r: AttentionRedactionClass) -> u8 {
    use AttentionRedactionClass::*;
    match r {
        MetadataSafeDefault => 1,
        SummaryOnly => 2,
        RedactedPayload => 3,
        CountOnly => 4,
        InternalSupportRestricted => 5,
    }
}

fn redaction_token(r: AttentionRedactionClass) -> &'static str {
    use AttentionRedactionClass::*;
    match r {
        MetadataSafeDefault => "metadata_safe_default",
        SummaryOnly => "summary_only",
        RedactedPayload => "redacted_payload",
        CountOnly => "count_only",
        InternalSupportRestricted => "internal_support_restricted",
    }
}

fn stronger_redaction(
    a: AttentionRedactionClass,
    b: AttentionRedactionClass,
) -> AttentionRedactionClass {
    if redaction_rank(a) >= redaction_rank(b) {
        a
    } else {
        b
    }
}

fn consumer_token(c: AttentionConsumerClass) -> &'static str {
    use AttentionConsumerClass::*;
    match c {
        ShellActivityCenter => "shell_activity_center",
        OsNotification => "os_notification",
        CompanionCrossClient => "companion_cross_client",
        OperatorDashboard => "operator_dashboard",
        SupportExport => "support_export",
        HelpAbout => "help_about",
        CliHeadless => "cli_headless",
    }
}

// ---------------------------------------------------------------------------
// Registry, invariants, and bundle record.
// ---------------------------------------------------------------------------

/// One job-family registry entry: a claimed M5 family that becomes a durable
/// activity object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobFamilyEntry {
    /// The job family.
    pub job_family: JobFamilyClass,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the family.
    pub summary: String,
    /// The actor subsystem that owns the work.
    pub actor_subsystem: SourceSubsystemClass,
    /// Whether the family produces long-running work (always true).
    pub long_running: bool,
    /// Whether failed or partial runs can be retried.
    pub retryable: bool,
    /// Whether the family carries evidence links (always true).
    pub evidence_bearing: bool,
    /// The authoritative object a row of this family reopens.
    pub default_reopen_target: ReopenTargetClass,
    /// The default privacy class.
    pub default_privacy: NotificationPrivacyClass,
    /// The default redaction posture.
    pub default_redaction: AttentionRedactionClass,
    /// The retention policy this family applies.
    pub retention: ActivityRetentionPolicy,
    /// The crate module(s) that produce this family's activity objects.
    pub produced_by_refs: Vec<String>,
    /// The activity-object ids this family contributes to the corpus.
    pub example_object_ids: Vec<String>,
    /// Whether the family is ever represented by a spinner or toast only (always
    /// false — the spec forbids it once work is long-running, retryable, or
    /// evidence-bearing).
    pub spinner_or_toast_only: bool,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen activity-objects bundle: the job-family registry, the activity-object
/// corpus, every rendered row, and the computed invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityObjectsBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_activity_objects_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The attention-routing matrix this bundle binds its vocabulary back to.
    pub matrix_ref: String,
    /// The matrix id the bundle binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps the bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the bundle.
    pub summary: String,
    /// The job-family registry.
    pub families: Vec<JobFamilyEntry>,
    /// The canonical activity-object corpus.
    pub objects: Vec<ActivityObject>,
    /// Every rendered activity-center row (one per object).
    pub rows: Vec<ActivityRow>,
    /// The computed invariants.
    pub invariants: Vec<ActivityInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityObjectsValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for ActivityObjectsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "activity-objects bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for ActivityObjectsValidationError {}

impl ActivityObjectsBundle {
    /// The object with a given id, if present.
    pub fn object(&self, job_id: &str) -> Option<&ActivityObject> {
        self.objects.iter().find(|o| o.activity_job_id == job_id)
    }

    /// The family entry for a family, if present.
    pub fn family(&self, family: JobFamilyClass) -> Option<&JobFamilyEntry> {
        self.families.iter().find(|f| f.job_family == family)
    }

    /// The row for a given job id, if present.
    pub fn row(&self, job_id: &str) -> Option<&ActivityRow> {
        self.rows.iter().find(|r| r.activity_job_id == job_id)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque
    /// `aureline://` handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> impl Iterator<Item = &str> {
        let fixed = [
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
            self.schema_ref.as_str(),
        ]
        .into_iter();
        let from_families = self
            .families
            .iter()
            .flat_map(|f| f.produced_by_refs.iter().map(String::as_str));
        let from_objects = self.objects.iter().flat_map(|o| {
            std::iter::once(o.scope_ref.as_str())
                .chain(std::iter::once(o.reopen_anchor.object_ref.as_str()))
                .chain(o.evidence_refs.iter().map(String::as_str))
        });
        fixed.chain(from_families).chain(from_objects)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), ActivityObjectsValidationError> {
        let fail = |reason: String| Err(ActivityObjectsValidationError { reason });

        if self.record_kind != M5_ACTIVITY_OBJECTS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ACTIVITY_OBJECTS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.families.is_empty() || self.objects.is_empty() || self.rows.is_empty() {
            return fail("families, objects, and rows must be non-empty".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.objects.iter().map(|o| o.activity_job_id.as_str())) {
            return fail("activity job ids are not unique".to_owned());
        }
        if !all_unique(self.rows.iter().map(|r| r.activity_row_id.as_str())) {
            return fail("activity row ids are not unique".to_owned());
        }
        if !all_unique(self.families.iter().map(|f| f.job_family.as_str())) {
            return fail("family entries are not unique".to_owned());
        }

        // Every family has an entry and at least one object.
        for family in JobFamilyClass::ALL {
            let entry = match self.family(family) {
                Some(e) => e,
                None => return fail(format!("family {} has no registry entry", family.as_str())),
            };
            if entry.spinner_or_toast_only {
                return fail(format!(
                    "family {} is marked spinner-or-toast-only",
                    family.as_str()
                ));
            }
            if !self.objects.iter().any(|o| o.job_family == family) {
                return fail(format!("family {} has no activity object", family.as_str()));
            }
        }

        // Every object is a durable, reopen-safe record and recomputes its derived
        // fields identically.
        for object in &self.objects {
            if !object.durable || !object.survives_focus_change || object.toast_only {
                return fail(format!(
                    "object {} is not a durable reopen-safe record",
                    object.activity_job_id
                ));
            }
            if object.evidence_refs.is_empty() || object.reopen_anchor.object_ref.is_empty() {
                return fail(format!(
                    "object {} is missing an evidence link or reopen anchor",
                    object.activity_job_id
                ));
            }
            if self.family(object.job_family).is_none() {
                return fail(format!(
                    "object {} references unknown family {}",
                    object.activity_job_id,
                    object.job_family.as_str()
                ));
            }
            if object.progress.phase != phase_for(object.progress.progress_state) {
                return fail(format!(
                    "object {} phase disagrees with its progress state",
                    object.activity_job_id
                ));
            }
            let expected_archive = archive_state_for(
                object.progress.progress_state,
                &object.retention,
                object.age_days,
            );
            if object.archive_state != expected_archive {
                return fail(format!(
                    "object {} archive state is not reproducible from its retention policy",
                    object.activity_job_id
                ));
            }
            if object.affordances
                != affordances_for(object.job_family, object.progress.progress_state)
            {
                return fail(format!(
                    "object {} affordances are not reproducible from its family and state",
                    object.activity_job_id
                ));
            }
        }

        // Every object has exactly one row, reproducible from the object.
        for object in &self.objects {
            let Some(row) = self.row(&object.activity_job_id) else {
                return fail(format!(
                    "object {} has no rendered row",
                    object.activity_job_id
                ));
            };
            if &render_row(object) != row {
                return fail(format!(
                    "row for object {} is not reproducible",
                    object.activity_job_id
                ));
            }
        }
        if self.rows.len() != self.objects.len() {
            return fail("row count does not match object count".to_owned());
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
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

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical activity-objects bundle.
///
/// Deterministic: the same bytes every call. The family registry and object corpus
/// are fixed, every row is rendered by [`render_row`], and each invariant's `holds`
/// flag is computed from the built data, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn activity_objects_bundle() -> ActivityObjectsBundle {
    let objects = build_objects();
    let families = build_families(&objects);
    let rows = objects.iter().map(render_row).collect::<Vec<_>>();
    let invariants = compute_invariants(&families, &objects, &rows);

    ActivityObjectsBundle {
        record_kind: M5_ACTIVITY_OBJECTS_RECORD_KIND.to_owned(),
        m5_activity_objects_schema_version: M5_ACTIVITY_OBJECTS_SCHEMA_VERSION,
        schema_ref: M5_ACTIVITY_OBJECTS_SCHEMA_REF.to_owned(),
        bundle_id: M5_ACTIVITY_OBJECTS_BUNDLE_ID.to_owned(),
        as_of: M5_ACTIVITY_OBJECTS_AS_OF.to_owned(),
        matrix_ref: M5_ACTIVITY_OBJECTS_MATRIX_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ACTIVITY_OBJECTS_FREEZE_GATE_REF.to_owned(),
        summary: "One durable activity object for every claimed M5 job family — notebook runs, \
                  task / CI jobs, AI runs, preview routes, pipeline actions, continuity sync, \
                  offboarding, operator handoffs, and managed alerts — each carrying a stable job \
                  id, actor subsystem, phase / progress, cancel / retry / open-details affordances, \
                  evidence links, cost / trust / policy flags, and an archive / expiry state. Every \
                  object is the durable authoritative record that survives focus change and reopens \
                  its authoritative object; completion and failure history is retained until \
                  archived or expired by policy; the archive state is one shared truth across the \
                  activity center, support export, companion, and operator dashboard; privacy never \
                  widens on a surface; and badges derive only from durable, pending rows."
            .to_owned(),
        families,
        objects,
        rows,
        invariants,
        raw_payload_excluded: true,
    }
}

/// A frozen last-activity stamp `age_days` days before [`M5_ACTIVITY_OBJECTS_AS_OF`].
/// Approximate calendar labels for the specific ages the corpus uses; `age_days`
/// remains the authoritative retention clock.
fn updated_at_label(age_days: u32) -> &'static str {
    match age_days {
        0 => "2026-06-23T00:00:00Z",
        1 => "2026-06-22T00:00:00Z",
        2 => "2026-06-21T00:00:00Z",
        3 => "2026-06-20T00:00:00Z",
        5 => "2026-06-18T00:00:00Z",
        10 => "2026-06-13T00:00:00Z",
        45 => "2026-05-09T00:00:00Z",
        200 => "2025-12-05T00:00:00Z",
        _ => M5_ACTIVITY_OBJECTS_AS_OF,
    }
}

#[allow(clippy::too_many_arguments)]
fn object(
    family: JobFamilyClass,
    slug: &str,
    progress_state: AttentionStateClass,
    progress_permille: u16,
    determinate: bool,
    scope: AttentionScopeClass,
    age_days: u32,
    cost_flag: bool,
    trust_flag: bool,
    policy_impact_flag: bool,
) -> ActivityObject {
    let retention = family.retention();
    let archive_state = archive_state_for(progress_state, &retention, age_days);
    let affordances = affordances_for(family, progress_state);
    let can_cancel = affordances.contains(&ActivityAffordanceClass::Cancel);
    let can_retry = affordances.contains(&ActivityAffordanceClass::Retry);
    let stamp = updated_at_label(age_days).to_owned();
    let key = slug.replace('.', "_");

    ActivityObject {
        activity_job_id: format!("activity_job:{slug}:0001"),
        job_family: family,
        actor_subsystem: family.actor_subsystem(),
        title_key: format!("activity.{key}.title"),
        scope,
        scope_ref: format!("aureline://scope/{slug}/0001"),
        privacy_class: family.default_privacy(),
        default_redaction: family.default_redaction(),
        progress: ActivityProgress {
            phase: phase_for(progress_state),
            progress_state,
            progress_permille,
            determinate,
        },
        can_cancel,
        can_retry,
        can_open_details: true,
        affordances,
        evidence_refs: vec![format!("aureline://evidence/{slug}/0001")],
        reopen_anchor: ReopenAnchor {
            reopen_target: family.default_reopen_target(),
            object_ref: format!("aureline://object/{slug}/0001"),
            label_key: format!("activity.{key}.open"),
        },
        flags: ActivityFlags {
            cost_flag,
            trust_flag,
            policy_impact_flag,
        },
        created_at: stamp.clone(),
        updated_at: stamp,
        age_days,
        retention,
        archive_state,
        durable: true,
        survives_focus_change: true,
        toast_only: false,
    }
}

fn build_objects() -> Vec<ActivityObject> {
    use AttentionScopeClass::*;
    use AttentionStateClass::*;
    use JobFamilyClass as F;

    vec![
        // A notebook run in progress — cancelable, no retry yet, active.
        object(
            F::Notebook,
            "notebook.run",
            Running,
            450,
            true,
            Session,
            0,
            false,
            false,
            false,
        ),
        // A queued notebook run — covers the queued/waiting state.
        object(
            F::Notebook,
            "notebook.queued",
            QueuedWaiting,
            0,
            true,
            Session,
            0,
            false,
            false,
            false,
        ),
        // A failed task — retryable, failure history retained (age below archive).
        object(
            F::Task,
            "task.failed",
            Failed,
            720,
            true,
            Session,
            3,
            false,
            false,
            false,
        ),
        // An AI run in progress — metered cost, untrusted output, indeterminate.
        object(
            F::AiRun,
            "ai.run",
            Running,
            0,
            false,
            Session,
            0,
            true,
            true,
            false,
        ),
        // A completed preview route — completion history retained.
        object(
            F::PreviewRoute,
            "preview.route",
            Completed,
            1000,
            true,
            Window,
            2,
            false,
            false,
            false,
        ),
        // A partially completed pipeline action — cancelable and retryable.
        object(
            F::PipelineAction,
            "pipeline.deploy",
            PartiallyCompleted,
            700,
            true,
            Workspace,
            0,
            true,
            false,
            false,
        ),
        // A completed sync now archived by policy (age past the archive horizon).
        object(
            F::Sync,
            "sync.backup",
            Completed,
            1000,
            true,
            Workspace,
            45,
            false,
            false,
            false,
        ),
        // A resolved sync restore — terminal but recent, active.
        object(
            F::Sync,
            "sync.restore",
            Resolved,
            1000,
            true,
            Workspace,
            1,
            false,
            false,
            false,
        ),
        // A completed offboarding now expired by policy (age past the expiry horizon).
        object(
            F::Offboarding,
            "offboarding.export",
            Completed,
            1000,
            true,
            Workspace,
            200,
            false,
            false,
            false,
        ),
        // An operator handoff awaiting review — managed, policy-impacting.
        object(
            F::OperatorHandoff,
            "operator.handoff",
            UnknownRequiresReview,
            0,
            false,
            TenantOrg,
            5,
            false,
            false,
            true,
        ),
        // A failed managed alert — managed, retryable, policy-impacting.
        object(
            F::ManagedAlert,
            "managed.alert",
            Failed,
            500,
            true,
            TenantOrg,
            10,
            true,
            false,
            true,
        ),
    ]
}

fn build_families(objects: &[ActivityObject]) -> Vec<JobFamilyEntry> {
    JobFamilyClass::ALL
        .iter()
        .map(|family| {
            let example_object_ids = objects
                .iter()
                .filter(|o| o.job_family == *family)
                .map(|o| o.activity_job_id.clone())
                .collect();
            JobFamilyEntry {
                job_family: *family,
                label: family.label().to_owned(),
                summary: family_summary(*family).to_owned(),
                actor_subsystem: family.actor_subsystem(),
                long_running: family.long_running(),
                retryable: family.retryable(),
                evidence_bearing: family.evidence_bearing(),
                default_reopen_target: family.default_reopen_target(),
                default_privacy: family.default_privacy(),
                default_redaction: family.default_redaction(),
                retention: family.retention(),
                produced_by_refs: family_produced_by(*family),
                example_object_ids,
                spinner_or_toast_only: false,
            }
        })
        .collect()
}

fn family_summary(family: JobFamilyClass) -> &'static str {
    match family {
        JobFamilyClass::Notebook => {
            "Notebook cell and notebook runs become durable rows with phase, progress, and a reopen \
             anchor instead of an inline spinner."
        }
        JobFamilyClass::Task => {
            "Long-running task-runner and CI jobs become durable rows with cancel / retry \
             affordances and an evidence link instead of a completion toast."
        }
        JobFamilyClass::AiRun => {
            "AI / agent runs become durable rows carrying cost and trust flags and a review reopen \
             anchor, never a transient status line."
        }
        JobFamilyClass::PreviewRoute => {
            "Preview route renders become durable rows that reopen the route object so a reviewable \
             preview survives focus loss."
        }
        JobFamilyClass::PipelineAction => {
            "Pipeline actions become durable rows with cancel / retry affordances and an evidence \
             link to the run."
        }
        JobFamilyClass::Sync => {
            "Continuity backup, restore, and failover become durable rows so a long sync stays \
             reviewable after it completes."
        }
        JobFamilyClass::Offboarding => {
            "Offboarding jobs become durable rows that reopen the evidence packet and are retained \
             until archived or expired by policy."
        }
        JobFamilyClass::OperatorHandoff => {
            "Operator handoffs become durable managed rows that reopen the review request and are \
             retained for compliance."
        }
        JobFamilyClass::ManagedAlert => {
            "Managed-policy alerts become durable managed rows that reopen the evidence packet, \
             retained for compliance."
        }
    }
}

fn family_produced_by(family: JobFamilyClass) -> Vec<String> {
    let refs: &[&str] = match family {
        JobFamilyClass::Notebook => &[
            "crates/aureline-shell/src/m5_activity_objects/mod.rs",
            "crates/aureline-shell/src/activity_center/mod.rs",
        ],
        JobFamilyClass::Task | JobFamilyClass::PipelineAction => {
            &["crates/aureline-shell/src/m5_activity_objects/mod.rs"]
        }
        JobFamilyClass::AiRun => &["crates/aureline-ai/src/lib.rs"],
        JobFamilyClass::PreviewRoute => &["crates/aureline-shell/src/activity_center/mod.rs"],
        JobFamilyClass::Sync | JobFamilyClass::Offboarding => {
            &["crates/aureline-support/src/lib.rs"]
        }
        JobFamilyClass::OperatorHandoff | JobFamilyClass::ManagedAlert => {
            &["crates/aureline-incident/src/lib.rs"]
        }
    };
    refs.iter().map(|r| (*r).to_owned()).collect()
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> ActivityInvariant {
    ActivityInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    families: &[JobFamilyEntry],
    objects: &[ActivityObject],
    rows: &[ActivityRow],
) -> Vec<ActivityInvariant> {
    let matrix = attention_routing_matrix();
    let activity_entry = matrix.object(AttentionObjectClass::ActivityObject);
    let mut out = Vec::new();

    // Every claimed M5 job family has a registry entry and a durable object.
    out.push(invariant(
        "activity.every_family_has_durable_object",
        "Every claimed M5 job family has a registry entry and at least one durable activity \
         object, and none is marked spinner-or-toast-only.",
        JobFamilyClass::ALL.iter().all(|family| {
            families
                .iter()
                .any(|f| f.job_family == *family && !f.spinner_or_toast_only)
                && objects.iter().any(|o| o.job_family == *family)
        }),
    ));

    // Every object carries the required contract fields.
    out.push(invariant(
        "activity.required_fields_present",
        "Every activity object carries a job id, actor subsystem, phase, progress state, \
         cancel / retry / open-details affordances, an evidence link, a reopen anchor, created / \
         updated stamps, a retention policy, and an archive state.",
        objects.iter().all(|o| {
            !o.activity_job_id.is_empty()
                && !o.evidence_refs.is_empty()
                && !o.reopen_anchor.object_ref.is_empty()
                && o.can_open_details
                && !o.created_at.is_empty()
                && !o.updated_at.is_empty()
                && !o.retention.retention_class.is_empty()
        }),
    ));

    // No spinner- or toast-only truth: every object is a durable reopen-safe record.
    out.push(invariant(
        "activity.durable_never_toast_only",
        "Every activity object is a durable authoritative record that survives focus change and is \
         never reduced to a spinner or toast.",
        objects
            .iter()
            .all(|o| o.durable && o.survives_focus_change && !o.toast_only),
    ));

    // Every object reopens an authoritative object the matrix admits.
    out.push(invariant(
        "activity.reopen_target_authoritative",
        "Every activity object names a reopen anchor whose target is one the matrix's activity \
         object admits, so a surface reopens the authoritative object rather than reissuing a blind \
         side effect.",
        match activity_entry {
            Some(entry) => objects
                .iter()
                .all(|o| entry.can_reopen(o.reopen_anchor.reopen_target)),
            None => false,
        },
    ));

    // Phase and progress are consistent and the progress state is in the matrix set.
    out.push(invariant(
        "activity.progress_phase_consistent",
        "Every object's phase is derived from its progress state, its progress state is one the \
         matrix's activity object admits, and a clean terminal state shows full progress.",
        match activity_entry {
            Some(entry) => objects.iter().all(|o| {
                o.progress.phase == phase_for(o.progress.progress_state)
                    && entry.can_show(o.progress.progress_state)
                    && (!matches!(
                        o.progress.progress_state,
                        AttentionStateClass::Completed | AttentionStateClass::Resolved
                    ) || o.progress.progress_permille == 1000)
            }),
            None => false,
        },
    ));

    // Archive / expiry is deterministic from the retention policy.
    out.push(invariant(
        "activity.archive_expiry_deterministic",
        "Every object's archive state recomputes identically from its progress state, retention \
         policy, and age, so archive and expiry behavior is testable.",
        objects.iter().all(|o| {
            o.archive_state
                == archive_state_for(o.progress.progress_state, &o.retention, o.age_days)
        }),
    ));

    // Completion and failure history is retained until archived or expired.
    out.push(invariant(
        "activity.failure_completion_history_retained",
        "Failed and completed objects whose age is below the archive horizon stay active, so \
         completion and failure history is never dropped into transient chrome, and the corpus \
         exercises active, archived, and expired retention.",
        {
            let recent_terminal_active = objects.iter().all(|o| {
                if is_terminal_for_retention(o.progress.progress_state)
                    && o.age_days < o.retention.archive_after_days
                {
                    o.archive_state == ArchiveStateClass::Active
                } else {
                    true
                }
            });
            let has_archived = objects
                .iter()
                .any(|o| o.archive_state == ArchiveStateClass::Archived);
            let has_expired = objects
                .iter()
                .any(|o| o.archive_state == ArchiveStateClass::Expired);
            let has_active_failure = objects.iter().any(|o| {
                o.progress.progress_state == AttentionStateClass::Failed
                    && o.archive_state == ArchiveStateClass::Active
            });
            recent_terminal_active && has_archived && has_expired && has_active_failure
        },
    ));

    // Affordances match the family and state.
    out.push(invariant(
        "activity.affordances_match_state",
        "Cancel is offered only while work is in flight, retry only for failed or partial work of a \
         retryable family, and open-details is always available so every row can reopen its object.",
        objects.iter().all(|o| {
            o.affordances == affordances_for(o.job_family, o.progress.progress_state)
                && o.can_open_details
                && o.affordances.contains(&ActivityAffordanceClass::OpenDetails)
                && o.can_cancel == o.affordances.contains(&ActivityAffordanceClass::Cancel)
                && o.can_retry == o.affordances.contains(&ActivityAffordanceClass::Retry)
        }),
    ));

    // Exactly one reproducible row per object.
    out.push(invariant(
        "activity.row_per_object",
        "Every object has exactly one rendered row that reproduces from the object and shares its \
         job id and reopen target.",
        rows.len() == objects.len()
            && objects.iter().all(|o| {
                rows.iter()
                    .filter(|r| r.activity_job_id == o.activity_job_id)
                    .count()
                    == 1
                    && rows
                        .iter()
                        .find(|r| r.activity_job_id == o.activity_job_id)
                        .is_some_and(|r| &render_row(o) == r)
            }),
    ));

    // Archive state is one shared truth across every surface of a row.
    out.push(invariant(
        "activity.archive_state_shared_across_surfaces",
        "Every row reports the same archive state on every surface projection — desktop, CLI, \
         support export, companion, and operator — so archive / expiry is one shared truth across \
         clients.",
        rows.iter().all(|r| {
            !r.surface_projections.is_empty()
                && r.surface_projections
                    .iter()
                    .all(|p| p.archive_state == r.archive_state)
        }),
    ));

    // Privacy never widens on a surface, and managed-sensitive rows never reach the
    // companion.
    out.push(invariant(
        "activity.privacy_never_widens_on_surface",
        "Each included surface projection applies a redaction at least as strong as the object \
         default, managed-sensitive rows are never included on the companion, and operator rows are \
         limited to org-scoped or managed work.",
        rows.iter().all(|r| {
            let object = objects
                .iter()
                .find(|o| o.activity_job_id == r.activity_job_id);
            let Some(object) = object else {
                return false;
            };
            r.surface_projections.iter().all(|p| {
                let redaction_ok = redaction_rank(p.applied_redaction)
                    >= redaction_rank(object.default_redaction);
                let companion_ok = !(p.consumer == AttentionConsumerClass::CompanionCrossClient
                    && p.included
                    && object.privacy_class == NotificationPrivacyClass::ManagedSensitive);
                let operator_ok = !(p.consumer == AttentionConsumerClass::OperatorDashboard
                    && p.included
                    && object.scope != AttentionScopeClass::TenantOrg
                    && !matches!(
                        object.job_family,
                        JobFamilyClass::OperatorHandoff | JobFamilyClass::ManagedAlert
                    ));
                redaction_ok && companion_ok && operator_ok
            })
        }),
    ));

    // Badges derive only from durable, pending rows.
    out.push(invariant(
        "activity.badge_from_durable_items",
        "A row counts toward the badge only when its object is a durable, active, attention-pending \
         item; nothing toast-only or archived contributes.",
        rows.iter().all(|r| {
            let object = objects
                .iter()
                .find(|o| o.activity_job_id == r.activity_job_id);
            match object {
                Some(o) => r.badge_counts_toward == o.badge_bearing(),
                None => false,
            }
        }) && objects
            .iter()
            .filter(|o| o.badge_bearing())
            .all(|o| o.durable && !o.toast_only && o.archive_state == ArchiveStateClass::Active),
    ));

    // Every family flagged long-running / retryable / evidence-bearing is proven by
    // a durable object.
    out.push(invariant(
        "activity.long_running_retryable_evidence_covered",
        "Every family declared long-running, retryable, or evidence-bearing is represented by a \
         durable activity object, so no launch-bearing family is left as a spinner or toast.",
        families.iter().all(|f| {
            let has_object = objects.iter().any(|o| o.job_family == f.job_family);
            let evidence_ok = !f.evidence_bearing
                || objects
                    .iter()
                    .filter(|o| o.job_family == f.job_family)
                    .all(|o| !o.evidence_refs.is_empty());
            has_object && evidence_ok
        }),
    ));

    // Matrix-bound: retention class, progress states, reopen targets, and required
    // fields all bind back to the matrix's activity object.
    out.push(invariant(
        "activity.matrix_bound",
        "Every retention class, progress state, and reopen target the bundle uses is one the \
         attention-routing matrix's activity object defines, and every required matrix field is \
         carried.",
        matrix_bound_holds(objects, activity_entry),
    ));

    // Support-export safe.
    out.push(invariant(
        "activity.support_export_safe",
        "Every object scope ref, reopen object ref, evidence ref, and family source ref is a \
         repo-relative object ref or opaque aureline:// handle, never a URL, host, credential, or \
         absolute path, so the rows are safe to embed in a support export.",
        objects.iter().all(|o| {
            is_export_safe_ref(&o.scope_ref)
                && is_export_safe_ref(&o.reopen_anchor.object_ref)
                && o.evidence_refs.iter().all(|r| is_export_safe_ref(r))
        }) && families
            .iter()
            .all(|f| f.produced_by_refs.iter().all(|r| is_export_safe_ref(r))),
    ));

    out
}

fn matrix_bound_holds(
    objects: &[ActivityObject],
    activity_entry: Option<&crate::m5_attention_routing::AttentionObjectEntry>,
) -> bool {
    let Some(entry) = activity_entry else {
        return false;
    };
    // The retention class matches the matrix's activity object.
    let retention_class_ok = entry.retention_rule.retention_class == RETENTION_CLASS
        && objects
            .iter()
            .all(|o| o.retention.retention_class == entry.retention_rule.retention_class);

    // Every progress state and reopen target is one the matrix admits.
    let states_ok = objects.iter().all(|o| {
        entry.can_show(o.progress.progress_state) && entry.can_reopen(o.reopen_anchor.reopen_target)
    });

    // Every required matrix field (required == true) is carried by the working
    // object. The struct provides each field; this proves the names line up.
    let required_field_ids = [
        "activity_job_id",
        "job_family",
        "actor_subsystem",
        "phase",
        "progress_state",
        "cancel_affordances",
        "retry_affordances",
        "evidence_link",
        "reopen_anchor_ref",
    ];
    let fields_ok = required_field_ids.iter().all(|id| {
        entry
            .required_fields
            .iter()
            .any(|f| f.field_id == *id && f.required)
    });

    retention_class_ok && states_ok && fields_ok
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn activity_objects_lines(bundle: &ActivityObjectsBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Activity-objects bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Families: {}  Objects: {}  Rows: {}  Invariants: {}",
        bundle.families.len(),
        bundle.objects.len(),
        bundle.rows.len(),
        bundle.invariants.len(),
    ));

    lines.push("Families:".to_owned());
    for f in &bundle.families {
        lines.push(format!(
            "  - {} [{}] actor={} retry={} reopen={} retention={}d/{}d",
            f.job_family.as_str(),
            f.label,
            f.actor_subsystem.as_str(),
            f.retryable,
            f.default_reopen_target.as_str(),
            f.retention.archive_after_days,
            f.retention.expire_after_days,
        ));
    }

    lines.push("Objects:".to_owned());
    for o in &bundle.objects {
        let affs: Vec<&str> = o.affordances.iter().map(|a| a.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] phase={} state={} archive={} badge={}",
            o.job_family.as_str(),
            o.activity_job_id,
            o.progress.phase.as_str(),
            o.progress.progress_state.as_str(),
            o.archive_state.as_str(),
            o.badge_bearing(),
        ));
        lines.push(format!(
            "      affordances={} reopen={} redaction={} flags=cost:{}/trust:{}/policy:{}",
            affs.join(","),
            o.reopen_anchor.reopen_target.as_str(),
            redaction_token(o.default_redaction),
            o.flags.cost_flag,
            o.flags.trust_flag,
            o.flags.policy_impact_flag,
        ));
    }

    lines.push("Rows:".to_owned());
    for r in &bundle.rows {
        let parts: Vec<String> = r
            .surface_projections
            .iter()
            .map(|p| {
                format!(
                    "{}={}",
                    p.consumer_token,
                    if p.included { "shown" } else { "hidden" }
                )
            })
            .collect();
        lines.push(format!(
            "  - {} archive={} badge={} :: {}",
            r.activity_job_id,
            r.archive_state.as_str(),
            r.badge_counts_toward,
            parts.join(" "),
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
