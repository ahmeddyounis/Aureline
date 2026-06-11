//! Durable activity-object qualification audit for the M5 depth job families.
//!
//! The M5 depth lanes mint new long-running or reviewable work objects —
//! notebook execution sessions, request/query runs, result-grid exports,
//! profiler and trace captures, replay sessions, pipeline rerun/cancel
//! actions, preview routes, sync state changes, offboarding jobs, and
//! incident-packet generation. Each is easy to ship as a toast that
//! disappears on focus loss, leaving no authoritative object to reopen. This
//! module carries the stable v1 shell promise forward into those lanes: every
//! marketed M5 job family MUST land its work in the durable activity center,
//! MUST reopen the exact authoritative object it acted on (after focus loss,
//! app restart, or degraded provider state), MUST differentiate its lifecycle
//! actions where the product requires them, and MUST stay exportable and
//! support-safe so CLI/headless output, support bundles, and companion fanout
//! refer back to the same row instead of reconstructing activity from logs.
//!
//! The audit projects, for each registered M5 job family, the canonical
//! activity-object descriptor against the qualification result the family
//! actually certifies for each of the eight durable-attention guarantees the
//! M5 lanes must pass:
//!
//! - `activity_center_landing`
//! - `exact_target_reopen`
//! - `reopen_after_focus_loss`
//! - `reopen_after_restart`
//! - `reopen_after_degraded_provider`
//! - `lifecycle_action_semantics`
//! - `support_export_identity`
//! - `companion_fanout_honesty`
//!
//! The resulting [`M5ActivityObjectReport`] is the canonical truth object for
//! the M5 durable-attention lane. It is consumed by:
//!
//! - the live shell activity-center / support inspector (so the in-product
//!   audit quotes the same per-family findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_activity_objects`), which is
//!   the only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/ux/m5/activity-center/`;
//! - the support-export wrapper that lets a reviewer pivot from a support
//!   case to the family that flagged a stale or red durable result;
//! - the markdown audit under
//!   `artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md`
//!   (rendered from the same seed); and
//! - the cross-surface hardening matrix and release-center packets, which
//!   ingest the audit directly when qualifying or narrowing a marketed M5 job
//!   family whose durable-attention evidence is stale or red.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 job family must declare a qualification binding for
//!    each of the eight durable-attention guarantees.
//! 2. Every family must carry a canonical exact-target reopen anchor, a
//!    non-empty support note, and a flag asserting it rides the shared
//!    activity-center model; a missing anchor, missing note, or a family that
//!    invents its own parallel history model is a blocker.
//! 3. A qualified guarantee must carry the captured evidence the guarantee
//!    requires — a durable attention packet, durable (not toast-only) truth,
//!    and an evidence-freshness stamp for every guarantee; a reopen outcome
//!    for the reopen guarantees; a survival outcome for the focus-loss,
//!    restart, and degraded-provider guarantees; differentiated action
//!    semantics for the lifecycle guarantee; a stable export identity for the
//!    support-export guarantee; and an honest fanout label for the companion
//!    guarantee. A red result (a lost reopen target, toast-only truth, a lost
//!    reopen after focus loss / restart / degraded provider, collapsed
//!    lifecycle actions, a reconstructed export identity, or a silent fanout
//!    failure) is a blocker.
//! 4. A family that tracks work in an ad-hoc parallel history model outside
//!    the shared activity center (`unqualified_local_history`), and a marketed
//!    guarantee claimed with no captured evidence (`missing_evidence`), are
//!    blockers.
//! 5. Stale durable evidence on a marketed guarantee is a blocker, so release
//!    tooling can narrow a marketed M5 family instead of shipping it as
//!    implicitly stable.
//! 6. At least one family must qualify each of the eight guarantees so the
//!    audit cannot regress into a single happy-path object.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/activity-center/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_m5_activity_objects_audit`].

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// Schema version exported with every M5 activity-object record.
pub const M5_ACTIVITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, docs, and support export.
pub const M5_ACTIVITY_SHARED_CONTRACT_REF: &str = "shell:m5_activity_objects:v1";

/// Stable record kind for the audit report payload.
pub const M5_ACTIVITY_REPORT_RECORD_KIND: &str = "shell_m5_activity_object_report_record";

/// Stable record kind for one per-family qualification row.
pub const M5_ACTIVITY_ROW_RECORD_KIND: &str = "shell_m5_activity_object_row_record";

/// Stable record kind for the support-export wrapper.
pub const M5_ACTIVITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_activity_object_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_ACTIVITY_REPORT_ID: &str = "shell:m5_activity_objects:audit:v1";

/// Stable support-export id.
pub const M5_ACTIVITY_SUPPORT_EXPORT_ID: &str = "support-export:m5-activity-objects:001";

/// Source schema ref for the canonical contract.
pub const M5_ACTIVITY_SOURCE_SCHEMA_REF: &str = "schemas/ux/m5-activity-object.schema.json";

/// Markdown publication ref this audit is rendered to.
pub const M5_ACTIVITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md";

/// Companion doc publication ref.
pub const M5_ACTIVITY_PUBLISHED_DOC_REF: &str = "docs/m5/durable-progress-and-reopen.md";

/// One M5 depth job family whose durable-attention guarantees the audit
/// qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivityJobFamily {
    /// Notebook execution session.
    NotebookRun,
    /// Request or query run.
    QueryRun,
    /// Result-grid export.
    ResultGridExport,
    /// Profiler capture.
    ProfilerCapture,
    /// Trace replay session.
    ReplaySession,
    /// Pipeline rerun/cancel action object.
    PipelineAction,
    /// Live preview route.
    PreviewRoute,
    /// Workspace sync state change.
    SyncStateChange,
    /// Offboarding / export-and-wipe job.
    OffboardingJob,
    /// Incident-packet generation.
    IncidentPacket,
}

impl M5ActivityJobFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRun => "notebook_run",
            Self::QueryRun => "query_run",
            Self::ResultGridExport => "result_grid_export",
            Self::ProfilerCapture => "profiler_capture",
            Self::ReplaySession => "replay_session",
            Self::PipelineAction => "pipeline_action",
            Self::PreviewRoute => "preview_route",
            Self::SyncStateChange => "sync_state_change",
            Self::OffboardingJob => "offboarding_job",
            Self::IncidentPacket => "incident_packet",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NotebookRun => "Notebook run",
            Self::QueryRun => "Query run",
            Self::ResultGridExport => "Result-grid export",
            Self::ProfilerCapture => "Profiler capture",
            Self::ReplaySession => "Replay session",
            Self::PipelineAction => "Pipeline action",
            Self::PreviewRoute => "Preview route",
            Self::SyncStateChange => "Sync state change",
            Self::OffboardingJob => "Offboarding job",
            Self::IncidentPacket => "Incident packet",
        }
    }
}

/// The durable-attention aspect a guarantee belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableAspect {
    /// The work lands on a durable surface instead of a toast.
    Landing,
    /// The work reopens its exact authoritative target.
    Reopen,
    /// The work carries differentiated lifecycle action semantics.
    Lifecycle,
    /// The work stays exportable and support/companion-safe.
    Export,
}

impl M5DurableAspect {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Landing => "landing",
            Self::Reopen => "reopen",
            Self::Lifecycle => "lifecycle",
            Self::Export => "export",
        }
    }
}

/// One durable-attention guarantee a family certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableGuarantee {
    /// The work object lands in the activity center, not a toast.
    ActivityCenterLanding,
    /// The object reopens its exact authoritative target.
    ExactTargetReopen,
    /// The exact-target reopen survives focus loss.
    ReopenAfterFocusLoss,
    /// The exact-target reopen survives an app restart.
    ReopenAfterRestart,
    /// The exact-target reopen survives a degraded provider state.
    ReopenAfterDegradedProvider,
    /// The object differentiates its lifecycle actions where required.
    LifecycleActionSemantics,
    /// The object identity stays stable across CLI/headless and support
    /// bundles.
    SupportExportIdentity,
    /// Companion fanout refers to the same object and labels stale/failed
    /// fanout honestly.
    CompanionFanoutHonesty,
}

impl M5DurableGuarantee {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenterLanding => "activity_center_landing",
            Self::ExactTargetReopen => "exact_target_reopen",
            Self::ReopenAfterFocusLoss => "reopen_after_focus_loss",
            Self::ReopenAfterRestart => "reopen_after_restart",
            Self::ReopenAfterDegradedProvider => "reopen_after_degraded_provider",
            Self::LifecycleActionSemantics => "lifecycle_action_semantics",
            Self::SupportExportIdentity => "support_export_identity",
            Self::CompanionFanoutHonesty => "companion_fanout_honesty",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ActivityCenterLanding => "Activity-center landing",
            Self::ExactTargetReopen => "Exact-target reopen",
            Self::ReopenAfterFocusLoss => "Reopen after focus loss",
            Self::ReopenAfterRestart => "Reopen after restart",
            Self::ReopenAfterDegradedProvider => "Reopen after degraded provider",
            Self::LifecycleActionSemantics => "Lifecycle action semantics",
            Self::SupportExportIdentity => "Support-export identity",
            Self::CompanionFanoutHonesty => "Companion fanout honesty",
        }
    }

    /// The eight durable-attention guarantees, in canonical order.
    pub const fn required_guarantees() -> [Self; 8] {
        [
            Self::ActivityCenterLanding,
            Self::ExactTargetReopen,
            Self::ReopenAfterFocusLoss,
            Self::ReopenAfterRestart,
            Self::ReopenAfterDegradedProvider,
            Self::LifecycleActionSemantics,
            Self::SupportExportIdentity,
            Self::CompanionFanoutHonesty,
        ]
    }

    /// The aspect this guarantee belongs to.
    pub const fn canonical_aspect(self) -> M5DurableAspect {
        match self {
            Self::ActivityCenterLanding => M5DurableAspect::Landing,
            Self::ExactTargetReopen
            | Self::ReopenAfterFocusLoss
            | Self::ReopenAfterRestart
            | Self::ReopenAfterDegradedProvider => M5DurableAspect::Reopen,
            Self::LifecycleActionSemantics => M5DurableAspect::Lifecycle,
            Self::SupportExportIdentity | Self::CompanionFanoutHonesty => M5DurableAspect::Export,
        }
    }

    /// `true` when a qualified binding must carry a reopen outcome.
    pub const fn requires_reopen_outcome(self) -> bool {
        matches!(
            self,
            Self::ExactTargetReopen
                | Self::ReopenAfterFocusLoss
                | Self::ReopenAfterRestart
                | Self::ReopenAfterDegradedProvider
        )
    }

    /// `true` when a qualified binding must carry a survival outcome.
    pub const fn requires_survival(self) -> bool {
        matches!(
            self,
            Self::ReopenAfterFocusLoss
                | Self::ReopenAfterRestart
                | Self::ReopenAfterDegradedProvider
        )
    }

    /// `true` when a qualified binding must carry differentiated action
    /// semantics.
    pub const fn requires_action_semantics(self) -> bool {
        matches!(self, Self::LifecycleActionSemantics)
    }

    /// `true` when a qualified binding must carry a stable export identity.
    pub const fn requires_export_identity(self) -> bool {
        matches!(self, Self::SupportExportIdentity)
    }

    /// `true` when a qualified binding must carry an honest fanout label.
    pub const fn requires_fanout_honesty(self) -> bool {
        matches!(self, Self::CompanionFanoutHonesty)
    }
}

/// Qualification status a family reports for one durable-attention guarantee.
///
/// Only `Qualified` rows project captured evidence and are drift/red checked.
/// `ExplicitlyNarrowed`, `NotApplicable`, `PlatformOmitted`, and
/// `DeclaredCaptureGap` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnqualifiedLocalHistory` (an ad-hoc parallel history
/// model outside the shared activity center) and `MissingEvidence` are
/// blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DurableStatus {
    /// The guarantee is qualified with captured durable evidence.
    Qualified,
    /// The family narrows this guarantee; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The guarantee does not apply to this family; a reason MUST be set.
    NotApplicable,
    /// The guarantee is not surfaced on this client/platform; a reason MUST
    /// be set.
    PlatformOmitted,
    /// A provider-backed family declares a known capture gap honestly; a
    /// reason MUST be set.
    DeclaredCaptureGap,
    /// The family tracks this work in an ad-hoc parallel history model
    /// outside the shared activity center. Always a blocker.
    UnqualifiedLocalHistory,
    /// A marketed guarantee is claimed with no captured evidence. Always a
    /// blocker.
    MissingEvidence,
}

impl M5DurableStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::PlatformOmitted => "platform_omitted",
            Self::DeclaredCaptureGap => "declared_capture_gap",
            Self::UnqualifiedLocalHistory => "unqualified_local_history",
            Self::MissingEvidence => "missing_evidence",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::NotApplicable
                | Self::PlatformOmitted
                | Self::DeclaredCaptureGap
        )
    }

    /// `true` for the status that projects captured evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

/// Whether the exact-target reopen resolves the authoritative object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReopenOutcome {
    /// Reopen resolves the exact authoritative target.
    ExactTargetResolved,
    /// Reopen fails to resolve its target. Always a blocker.
    TargetLost,
    /// The family has no reopen target on this guarantee.
    NotApplicable,
}

impl M5ReopenOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTargetResolved => "exact_target_resolved",
            Self::TargetLost => "target_lost",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the work object survives beyond an ephemeral toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastIndependence {
    /// The object is durable: it survives the toast or status item.
    Durable,
    /// The only truth is a toast. Always a blocker.
    ToastOnly,
}

impl M5ToastIndependence {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Durable => "durable",
            Self::ToastOnly => "toast_only",
        }
    }
}

/// Whether the reopen target survives a focus, restart, or provider event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurvivalOutcome {
    /// The reopen target survives the event.
    Survives,
    /// The reopen target is lost after the event. Always a blocker.
    Lost,
}

impl M5SurvivalOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Survives => "survives",
            Self::Lost => "lost",
        }
    }
}

/// Whether lifecycle actions stay differentiated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActionSemantics {
    /// Dismiss, mute, snooze, acknowledge, resolve, and reopen stay distinct
    /// where the product requires them.
    Differentiated,
    /// Reviewable work is collapsed into one generic close action. Always a
    /// blocker.
    Collapsed,
}

impl M5ActionSemantics {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Differentiated => "differentiated",
            Self::Collapsed => "collapsed",
        }
    }
}

/// Whether the object identity is a stable reference or reconstructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportIdentity {
    /// CLI/headless output and support bundles refer to the same row id.
    StableReference,
    /// Activity is reconstructed from logs instead of a stable id. Always a
    /// blocker.
    Reconstructed,
}

impl M5ExportIdentity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableReference => "stable_reference",
            Self::Reconstructed => "reconstructed",
        }
    }
}

/// Whether companion fanout labels stale or failed delivery honestly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FanoutHonesty {
    /// Stale or failed fanout is labelled honestly and points at the same
    /// object.
    HonestlyLabeled,
    /// A failed fanout is hidden behind a silent success. Always a blocker.
    SilentFailure,
}

impl M5FanoutHonesty {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HonestlyLabeled => "honestly_labeled",
            Self::SilentFailure => "silent_failure",
        }
    }
}

/// Freshness of the captured durable evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed guarantee.
    Stale,
}

impl M5EvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// A differentiated lifecycle action a family exposes on its activity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivityAction {
    /// Dismiss the row from the active list (keeps it as history).
    Dismiss,
    /// Mute future notifications for the object.
    Mute,
    /// Snooze the row until later.
    Snooze,
    /// Acknowledge the object without resolving it.
    Acknowledge,
    /// Resolve a reviewable object.
    Resolve,
    /// Reopen the exact authoritative target.
    Reopen,
}

impl M5ActivityAction {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dismiss => "dismiss",
            Self::Mute => "mute",
            Self::Snooze => "snooze",
            Self::Acknowledge => "acknowledge",
            Self::Resolve => "resolve",
            Self::Reopen => "reopen",
        }
    }
}

/// How much review, lifecycle, or risk meaning the work object conveys.
///
/// A family that conveys lifecycle, review, or risk meaning is
/// "high-salience": its durable object must always carry an exact-target
/// reopen affordance and a differentiated lifecycle action set, so the audit
/// requires a present reopen outcome on every qualified guarantee and a
/// non-empty action set on the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActivitySalience {
    /// Informational only; no lifecycle, review, or risk meaning.
    Informational,
    /// Conveys lifecycle state (preview, stale, pending).
    LifecycleBearing,
    /// Conveys a reviewable result the user is expected to return to.
    ReviewBearing,
    /// Conveys risk (destructive, blocked, failed).
    RiskBearing,
}

impl M5ActivitySalience {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::LifecycleBearing => "lifecycle_bearing",
            Self::ReviewBearing => "review_bearing",
            Self::RiskBearing => "risk_bearing",
        }
    }

    /// `true` for salience classes whose object must stay reopen-able and
    /// differentiated.
    pub const fn is_high_salience(self) -> bool {
        matches!(
            self,
            Self::LifecycleBearing | Self::ReviewBearing | Self::RiskBearing
        )
    }
}

/// Lifecycle label retained on the canonical family descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FamilyLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; families must point at the replacement.
    Deprecated,
}

impl M5FamilyLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Canonical descriptor for one M5 job family's durable-attention contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivityObjectDescriptor {
    /// Stable family id (e.g. `activity:notebook_run`).
    pub family_id: String,
    /// Job family the descriptor belongs to.
    pub job_family: M5ActivityJobFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical exact-target reopen anchor ref the activity center reopens
    /// the authoritative object from.
    pub reopen_anchor_ref: String,
    /// Support note retained on the descriptor. MUST be non-empty.
    pub support_note: String,
    /// Pinned semantic salience.
    pub semantic_salience: M5ActivitySalience,
    /// Pinned family lifecycle label.
    pub lifecycle_label: M5FamilyLifecycle,
    /// Differentiated lifecycle actions the family exposes, in canonical
    /// order.
    pub supported_actions: Vec<M5ActivityAction>,
    /// `true` when the family is marketed on desktop and therefore must pass
    /// the claimed matrix or narrow accordingly.
    pub marketed_on_desktop: bool,
    /// `true` once the family rides the shared activity-center model and does
    /// not invent a parallel history model. MUST be `true`.
    pub registered_on_activity_center: bool,
}

impl M5ActivityObjectDescriptor {
    /// `true` when this family's salience makes it high-salience for the
    /// audit.
    pub const fn is_high_salience(&self) -> bool {
        self.semantic_salience.is_high_salience()
    }
}

/// Per-guarantee binding a family reports for one durable-attention guarantee.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DurableBinding {
    /// Guarantee this binding covers.
    pub guarantee: M5DurableGuarantee,
    /// Aspect projected for the guarantee. MUST equal the guarantee's
    /// canonical aspect.
    pub aspect: M5DurableAspect,
    /// Qualification status the family reports.
    pub qualification_status: M5DurableStatus,
    /// `true` when the family is marketed on this guarantee.
    pub marketed_on_guarantee: bool,
    /// Captured durable attention-packet ref (`None` for non-qualified rows).
    pub projected_durable_packet_ref: Option<String>,
    /// Captured reopen outcome (`None` unless the guarantee requires it or
    /// the family is high-salience).
    pub projected_reopen_outcome: Option<M5ReopenOutcome>,
    /// Captured toast-independence result (`None` for non-qualified rows).
    pub projected_toast_independence: Option<M5ToastIndependence>,
    /// Captured survival outcome (`None` unless the guarantee requires it).
    pub projected_survival: Option<M5SurvivalOutcome>,
    /// Captured action-semantics result (`None` unless the guarantee requires
    /// it).
    pub projected_action_semantics: Option<M5ActionSemantics>,
    /// Captured export-identity result (`None` unless the guarantee requires
    /// it).
    pub projected_export_identity: Option<M5ExportIdentity>,
    /// Captured fanout-honesty result (`None` unless the guarantee requires
    /// it).
    pub projected_fanout_honesty: Option<M5FanoutHonesty>,
    /// Freshness of the captured evidence (`None` for non-qualified rows).
    pub evidence_freshness: Option<M5EvidenceFreshness>,
    /// Timestamp the evidence was captured (`None` for non-qualified rows).
    pub evidence_captured_at: Option<String>,
    /// Narrowing reason set when `qualification_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5ActivityBlockingFinding {
    /// A family tracks this work in an ad-hoc parallel history model outside
    /// the shared activity center.
    UnqualifiedLocalHistory {
        /// Family that exposes the gap.
        family_id: String,
        /// Guarantee that exposes the gap.
        guarantee: M5DurableGuarantee,
    },
    /// A marketed guarantee is claimed with no captured evidence.
    MissingEvidence {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// A qualified guarantee is missing its captured durable attention packet.
    MissingDurablePacket {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// A guarantee relies on toast-only truth.
    ToastOnlyTruth {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// A guarantee loses the exact-target reopen affordance.
    ReopenTargetLost {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// The reopen target is lost after focus loss.
    ReopenLostAfterFocusLoss {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// The reopen target is lost after an app restart.
    ReopenLostAfterRestart {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// The reopen target is lost under a degraded provider state.
    ReopenLostUnderDegradedProvider {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// Reviewable work is collapsed into one generic close action.
    LifecycleActionsCollapsed {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// The object identity is reconstructed from logs instead of a stable id.
    ExportIdentityReconstructed {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// A failed companion fanout is hidden behind a silent success.
    FanoutFailureSilent {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// A marketed guarantee carries stale durable evidence.
    StaleEvidenceOnMarketedRow {
        family_id: String,
        guarantee: M5DurableGuarantee,
    },
    /// A binding projects an aspect that disagrees with the guarantee's
    /// canonical aspect.
    AspectDrift {
        family_id: String,
        guarantee: M5DurableGuarantee,
        /// Projected aspect.
        projected_aspect: M5DurableAspect,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        family_id: String,
        guarantee: M5DurableGuarantee,
        qualification_status: M5DurableStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        family_id: String,
        guarantee: M5DurableGuarantee,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical exact-target reopen anchor.
    DescriptorMissingReopenAnchor { family_id: String },
    /// The descriptor carries no support note.
    MissingSupportNote { family_id: String },
    /// The family invents a parallel history model outside the activity
    /// center.
    FamilyNotOnActivityCenter { family_id: String },
    /// A high-salience family exposes no differentiated lifecycle actions.
    MissingLifecycleActions { family_id: String },
}

impl M5ActivityBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedLocalHistory { .. } => "unqualified_local_history",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingDurablePacket { .. } => "missing_durable_packet",
            Self::ToastOnlyTruth { .. } => "toast_only_truth",
            Self::ReopenTargetLost { .. } => "reopen_target_lost",
            Self::ReopenLostAfterFocusLoss { .. } => "reopen_lost_after_focus_loss",
            Self::ReopenLostAfterRestart { .. } => "reopen_lost_after_restart",
            Self::ReopenLostUnderDegradedProvider { .. } => "reopen_lost_under_degraded_provider",
            Self::LifecycleActionsCollapsed { .. } => "lifecycle_actions_collapsed",
            Self::ExportIdentityReconstructed { .. } => "export_identity_reconstructed",
            Self::FanoutFailureSilent { .. } => "fanout_failure_silent",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::AspectDrift { .. } => "aspect_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingReopenAnchor { .. } => "descriptor_missing_reopen_anchor",
            Self::MissingSupportNote { .. } => "missing_support_note",
            Self::FamilyNotOnActivityCenter { .. } => "family_not_on_activity_center",
            Self::MissingLifecycleActions { .. } => "missing_lifecycle_actions",
        }
    }

    /// Returns the family id this finding is attached to.
    pub fn family_id(&self) -> &str {
        match self {
            Self::UnqualifiedLocalHistory { family_id, .. }
            | Self::MissingEvidence { family_id, .. }
            | Self::MissingDurablePacket { family_id, .. }
            | Self::ToastOnlyTruth { family_id, .. }
            | Self::ReopenTargetLost { family_id, .. }
            | Self::ReopenLostAfterFocusLoss { family_id, .. }
            | Self::ReopenLostAfterRestart { family_id, .. }
            | Self::ReopenLostUnderDegradedProvider { family_id, .. }
            | Self::LifecycleActionsCollapsed { family_id, .. }
            | Self::ExportIdentityReconstructed { family_id, .. }
            | Self::FanoutFailureSilent { family_id, .. }
            | Self::StaleEvidenceOnMarketedRow { family_id, .. }
            | Self::AspectDrift { family_id, .. }
            | Self::MissingNarrowingReason { family_id, .. }
            | Self::MissingProjection { family_id, .. }
            | Self::DescriptorMissingReopenAnchor { family_id }
            | Self::MissingSupportNote { family_id }
            | Self::FamilyNotOnActivityCenter { family_id }
            | Self::MissingLifecycleActions { family_id } => family_id,
        }
    }

    /// Returns the guarantee this finding is attached to, when guarantee-scoped.
    pub fn guarantee(&self) -> Option<M5DurableGuarantee> {
        match self {
            Self::UnqualifiedLocalHistory { guarantee, .. }
            | Self::MissingEvidence { guarantee, .. }
            | Self::MissingDurablePacket { guarantee, .. }
            | Self::ToastOnlyTruth { guarantee, .. }
            | Self::ReopenTargetLost { guarantee, .. }
            | Self::ReopenLostAfterFocusLoss { guarantee, .. }
            | Self::ReopenLostAfterRestart { guarantee, .. }
            | Self::ReopenLostUnderDegradedProvider { guarantee, .. }
            | Self::LifecycleActionsCollapsed { guarantee, .. }
            | Self::ExportIdentityReconstructed { guarantee, .. }
            | Self::FanoutFailureSilent { guarantee, .. }
            | Self::StaleEvidenceOnMarketedRow { guarantee, .. }
            | Self::AspectDrift { guarantee, .. }
            | Self::MissingNarrowingReason { guarantee, .. }
            | Self::MissingProjection { guarantee, .. } => Some(*guarantee),
            Self::DescriptorMissingReopenAnchor { .. }
            | Self::MissingSupportNote { .. }
            | Self::FamilyNotOnActivityCenter { .. }
            | Self::MissingLifecycleActions { .. } => None,
        }
    }
}

/// One per-family durable-attention qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivityObjectRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the family.
    pub descriptor: M5ActivityObjectDescriptor,
    /// Guarantee-by-guarantee qualification bindings, in canonical order.
    pub bindings: Vec<M5DurableBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5ActivityBlockingFinding>,
    /// `true` when the family's descriptor classifies it as high-salience.
    pub high_salience: bool,
    /// `true` when the family is marketed on desktop.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivityFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_local_history` findings.
    pub unqualified_local_history: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_durable_packet` findings.
    pub missing_durable_packet: usize,
    /// Number of `toast_only_truth` findings.
    pub toast_only_truth: usize,
    /// Number of `reopen_target_lost` findings.
    pub reopen_target_lost: usize,
    /// Number of `reopen_lost_after_focus_loss` findings.
    pub reopen_lost_after_focus_loss: usize,
    /// Number of `reopen_lost_after_restart` findings.
    pub reopen_lost_after_restart: usize,
    /// Number of `reopen_lost_under_degraded_provider` findings.
    pub reopen_lost_under_degraded_provider: usize,
    /// Number of `lifecycle_actions_collapsed` findings.
    pub lifecycle_actions_collapsed: usize,
    /// Number of `export_identity_reconstructed` findings.
    pub export_identity_reconstructed: usize,
    /// Number of `fanout_failure_silent` findings.
    pub fanout_failure_silent: usize,
    /// Number of `stale_evidence_on_marketed_row` findings.
    pub stale_evidence_on_marketed_row: usize,
    /// Number of `aspect_drift` findings.
    pub aspect_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_reopen_anchor` findings.
    pub descriptor_missing_reopen_anchor: usize,
    /// Number of `missing_support_note` findings.
    pub missing_support_note: usize,
    /// Number of `family_not_on_activity_center` findings.
    pub family_not_on_activity_center: usize,
    /// Number of `missing_lifecycle_actions` findings.
    pub missing_lifecycle_actions: usize,
}

impl M5ActivityFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_local_history: 0,
            missing_evidence: 0,
            missing_durable_packet: 0,
            toast_only_truth: 0,
            reopen_target_lost: 0,
            reopen_lost_after_focus_loss: 0,
            reopen_lost_after_restart: 0,
            reopen_lost_under_degraded_provider: 0,
            lifecycle_actions_collapsed: 0,
            export_identity_reconstructed: 0,
            fanout_failure_silent: 0,
            stale_evidence_on_marketed_row: 0,
            aspect_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_reopen_anchor: 0,
            missing_support_note: 0,
            family_not_on_activity_center: 0,
            missing_lifecycle_actions: 0,
        }
    }

    fn record(&mut self, finding: &M5ActivityBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5ActivityBlockingFinding::UnqualifiedLocalHistory { .. } => {
                self.unqualified_local_history += 1
            }
            M5ActivityBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5ActivityBlockingFinding::MissingDurablePacket { .. } => {
                self.missing_durable_packet += 1
            }
            M5ActivityBlockingFinding::ToastOnlyTruth { .. } => self.toast_only_truth += 1,
            M5ActivityBlockingFinding::ReopenTargetLost { .. } => self.reopen_target_lost += 1,
            M5ActivityBlockingFinding::ReopenLostAfterFocusLoss { .. } => {
                self.reopen_lost_after_focus_loss += 1
            }
            M5ActivityBlockingFinding::ReopenLostAfterRestart { .. } => {
                self.reopen_lost_after_restart += 1
            }
            M5ActivityBlockingFinding::ReopenLostUnderDegradedProvider { .. } => {
                self.reopen_lost_under_degraded_provider += 1
            }
            M5ActivityBlockingFinding::LifecycleActionsCollapsed { .. } => {
                self.lifecycle_actions_collapsed += 1
            }
            M5ActivityBlockingFinding::ExportIdentityReconstructed { .. } => {
                self.export_identity_reconstructed += 1
            }
            M5ActivityBlockingFinding::FanoutFailureSilent { .. } => {
                self.fanout_failure_silent += 1
            }
            M5ActivityBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5ActivityBlockingFinding::AspectDrift { .. } => self.aspect_drift += 1,
            M5ActivityBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5ActivityBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5ActivityBlockingFinding::DescriptorMissingReopenAnchor { .. } => {
                self.descriptor_missing_reopen_anchor += 1
            }
            M5ActivityBlockingFinding::MissingSupportNote { .. } => self.missing_support_note += 1,
            M5ActivityBlockingFinding::FamilyNotOnActivityCenter { .. } => {
                self.family_not_on_activity_center += 1
            }
            M5ActivityBlockingFinding::MissingLifecycleActions { .. } => {
                self.missing_lifecycle_actions += 1
            }
        }
    }
}

/// Per-guarantee coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivityCoverageSummary {
    /// Guarantee this summary covers.
    pub guarantee: M5DurableGuarantee,
    /// Number of `qualified` rows on this guarantee.
    pub qualified_rows: usize,
    /// Number of `explicitly_narrowed` rows on this guarantee.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this guarantee.
    pub not_applicable_rows: usize,
    /// Number of `platform_omitted` rows on this guarantee.
    pub platform_omitted_rows: usize,
    /// Number of `declared_capture_gap` rows on this guarantee.
    pub declared_capture_gap_rows: usize,
    /// Number of `unqualified_local_history` rows on this guarantee.
    pub unqualified_local_history_rows: usize,
    /// Number of `missing_evidence` rows on this guarantee.
    pub missing_evidence_rows: usize,
}

impl M5ActivityCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.platform_omitted_rows
            + self.declared_capture_gap_rows
    }
}

/// A single reopen-anchor index entry the audit publishes so the activity
/// center, docs, and release surfaces can reopen each family by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReopenAnchorEntry {
    /// Job family the anchor belongs to.
    pub job_family: M5ActivityJobFamily,
    /// Family id the anchor reopens.
    pub family_id: String,
    /// Canonical exact-target reopen anchor ref.
    pub reopen_anchor_ref: String,
}

/// One marketed guarantee release tooling should narrow because its durable
/// evidence is stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NarrowableRow {
    /// Family id that must narrow.
    pub family_id: String,
    /// Guarantee that must narrow.
    pub guarantee: M5DurableGuarantee,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 durable activity-object qualification audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivityObjectReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required durable-attention guarantees, in canonical order.
    pub required_guarantees: Vec<M5DurableGuarantee>,
    /// Per-family qualification rows, sorted by `descriptor.family_id`.
    pub rows: Vec<M5ActivityObjectRow>,
    /// Per-guarantee coverage summary, in canonical order.
    pub guarantee_coverage: Vec<M5ActivityCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5ActivityFindingSummary,
    /// Canonical reopen-anchor index, sorted by family id.
    pub reopen_anchor_index: Vec<M5ReopenAnchorEntry>,
    /// Number of registered M5 families present.
    pub registered_family_count: usize,
    /// Number of high-salience families present.
    pub high_salience_family_count: usize,
    /// Number of families marketed on desktop.
    pub marketed_family_count: usize,
    /// Total durable-attention guarantees checked.
    pub durable_guarantees_checked: usize,
    /// Marketed rows release tooling should narrow because their evidence is
    /// stale or red.
    pub narrowable_marketed_rows: Vec<M5NarrowableRow>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5ActivityObjectReport {
    /// Returns `true` when every required guarantee is qualified by at least
    /// one family.
    pub fn every_required_guarantee_qualified(&self) -> bool {
        for guarantee in M5DurableGuarantee::required_guarantees() {
            let any_qualified = self.rows.iter().any(|family| {
                family.bindings.iter().any(|binding| {
                    binding.guarantee == guarantee
                        && binding.qualification_status == M5DurableStatus::Qualified
                })
            });
            if !any_qualified {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: families={}, high_salience={}, marketed={}, guarantees={}, blocking={}, clean={}",
            self.registered_family_count,
            self.high_salience_family_count,
            self.marketed_family_count,
            self.durable_guarantees_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.guarantee_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, unqualified={}, missing_evidence={}",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_history_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for family in &self.rows {
            for finding in &family.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.family_id(),
                    finding
                        .guarantee()
                        .map(M5DurableGuarantee::as_str)
                        .unwrap_or("family"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_rows {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.family_id,
                narrowable.guarantee.as_str(),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 durable activity-object qualification audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_activity_objects`](../../../../crates/aureline-shell/src/m5_activity_objects/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- report-md > \\\n  artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Registered M5 families: `{}`\n",
            self.registered_family_count
        ));
        out.push_str(&format!(
            "- High-salience families: `{}`\n",
            self.high_salience_family_count
        ));
        out.push_str(&format!(
            "- Marketed families: `{}`\n",
            self.marketed_family_count
        ));
        out.push_str(&format!(
            "- Durable guarantees checked: `{}`\n",
            self.durable_guarantees_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed rows: `{}`\n",
            self.narrowable_marketed_rows.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Per-guarantee coverage\n\n");
        out.push_str(
            "| Durable guarantee | Qualified | Narrowed | Unqualified | Missing evidence |\n\
             | ----------------- | --------: | -------: | ----------: | ---------------: |\n",
        );
        for coverage in &self.guarantee_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_history_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_local_history` | {} |\n",
            self.findings_summary.unqualified_local_history
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_durable_packet` | {} |\n",
            self.findings_summary.missing_durable_packet
        ));
        out.push_str(&format!(
            "| `toast_only_truth` | {} |\n",
            self.findings_summary.toast_only_truth
        ));
        out.push_str(&format!(
            "| `reopen_target_lost` | {} |\n",
            self.findings_summary.reopen_target_lost
        ));
        out.push_str(&format!(
            "| `reopen_lost_after_focus_loss` | {} |\n",
            self.findings_summary.reopen_lost_after_focus_loss
        ));
        out.push_str(&format!(
            "| `reopen_lost_after_restart` | {} |\n",
            self.findings_summary.reopen_lost_after_restart
        ));
        out.push_str(&format!(
            "| `reopen_lost_under_degraded_provider` | {} |\n",
            self.findings_summary.reopen_lost_under_degraded_provider
        ));
        out.push_str(&format!(
            "| `lifecycle_actions_collapsed` | {} |\n",
            self.findings_summary.lifecycle_actions_collapsed
        ));
        out.push_str(&format!(
            "| `export_identity_reconstructed` | {} |\n",
            self.findings_summary.export_identity_reconstructed
        ));
        out.push_str(&format!(
            "| `fanout_failure_silent` | {} |\n",
            self.findings_summary.fanout_failure_silent
        ));
        out.push_str(&format!(
            "| `stale_evidence_on_marketed_row` | {} |\n",
            self.findings_summary.stale_evidence_on_marketed_row
        ));
        out.push_str(&format!(
            "| `aspect_drift` | {} |\n",
            self.findings_summary.aspect_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_reopen_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_reopen_anchor
        ));
        out.push_str(&format!(
            "| `missing_support_note` | {} |\n",
            self.findings_summary.missing_support_note
        ));
        out.push_str(&format!(
            "| `family_not_on_activity_center` | {} |\n",
            self.findings_summary.family_not_on_activity_center
        ));
        out.push_str(&format!(
            "| `missing_lifecycle_actions` | {} |\n\n",
            self.findings_summary.missing_lifecycle_actions
        ));

        out.push_str("## Reopen anchor index\n\n");
        out.push_str(
            "| Job family | Family id | Reopen anchor |\n| ---------- | --------- | ------------- |\n",
        );
        for entry in &self.reopen_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.job_family.display_label(),
                entry.family_id,
                entry.reopen_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-family rows\n\n");
        for family in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {})\n\n",
                family.descriptor.family_id,
                family.descriptor.job_family.as_str(),
                family.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                family.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Semantic salience: `{}`\n",
                family.descriptor.semantic_salience.as_str()
            ));
            out.push_str(&format!(
                "- Reopen anchor: `{}`\n",
                family.descriptor.reopen_anchor_ref
            ));
            out.push_str(&format!(
                "- Lifecycle actions: {}\n",
                if family.descriptor.supported_actions.is_empty() {
                    "none".to_owned()
                } else {
                    family
                        .descriptor
                        .supported_actions
                        .iter()
                        .map(|action| format!("`{}`", action.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            out.push_str(&format!(
                "- Marketed on desktop: `{}`\n",
                if family.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- High-salience: `{}`\n\n",
                if family.high_salience { "yes" } else { "no" }
            ));

            out.push_str(
                "| Durable guarantee | Status | Reopen | Toast | Survival | Action | Export | Fanout | Freshness | Narrowing reason |\n\
                 | ----------------- | ------ | ------ | ----- | -------- | ------ | ------ | ------ | --------- | ---------------- |\n",
            );
            for binding in &family.bindings {
                let reopen = binding
                    .projected_reopen_outcome
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let toast = binding
                    .projected_toast_independence
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let survival = binding
                    .projected_survival
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let action = binding
                    .projected_action_semantics
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let export = binding
                    .projected_export_identity
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let fanout = binding
                    .projected_fanout_honesty
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let freshness = binding
                    .evidence_freshness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.guarantee.display_label(),
                    binding.qualification_status.as_str(),
                    reopen,
                    toast,
                    survival,
                    action,
                    export,
                    fanout,
                    freshness,
                    narrowing,
                ));
            }
            out.push('\n');

            if family.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &family.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .guarantee()
                            .map(M5DurableGuarantee::as_str)
                            .unwrap_or("family"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_activity_objects -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_activity_objects_fixtures\n");
        out.push_str("python3 tools/ci/m5/activity_objects_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 durable activity-object audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ActivitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5ActivityObjectReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5ActivitySupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5ActivityObjectReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for family in &report.rows {
            case_ids.push(family.descriptor.family_id.clone());
            case_ids.push(family.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_ACTIVITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ACTIVITY_SCHEMA_VERSION,
            shared_contract_ref: M5_ACTIVITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-family blocking findings from a descriptor and its
/// guarantee bindings.
fn compute_family_findings(
    descriptor: &M5ActivityObjectDescriptor,
    bindings: &[M5DurableBinding],
    high_salience: bool,
) -> Vec<M5ActivityBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (family-scoped) findings.
    if descriptor.reopen_anchor_ref.trim().is_empty() {
        findings.push(M5ActivityBlockingFinding::DescriptorMissingReopenAnchor {
            family_id: descriptor.family_id.clone(),
        });
    }
    if descriptor.support_note.trim().is_empty() {
        findings.push(M5ActivityBlockingFinding::MissingSupportNote {
            family_id: descriptor.family_id.clone(),
        });
    }
    if !descriptor.registered_on_activity_center {
        findings.push(M5ActivityBlockingFinding::FamilyNotOnActivityCenter {
            family_id: descriptor.family_id.clone(),
        });
    }
    if high_salience && descriptor.supported_actions.is_empty() {
        findings.push(M5ActivityBlockingFinding::MissingLifecycleActions {
            family_id: descriptor.family_id.clone(),
        });
    }

    for binding in bindings {
        let guarantee = binding.guarantee;
        let family_id = descriptor.family_id.clone();

        // A binding's aspect must match its guarantee's canonical aspect.
        if binding.aspect != guarantee.canonical_aspect() {
            findings.push(M5ActivityBlockingFinding::AspectDrift {
                family_id: family_id.clone(),
                guarantee,
                projected_aspect: binding.aspect,
            });
        }

        match binding.qualification_status {
            M5DurableStatus::UnqualifiedLocalHistory => {
                findings.push(M5ActivityBlockingFinding::UnqualifiedLocalHistory {
                    family_id: family_id.clone(),
                    guarantee,
                });
            }
            M5DurableStatus::MissingEvidence => {
                findings.push(M5ActivityBlockingFinding::MissingEvidence {
                    family_id: family_id.clone(),
                    guarantee,
                });
            }
            M5DurableStatus::Qualified => {
                compute_qualified_findings(binding, high_salience, &family_id, &mut findings);
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5ActivityBlockingFinding::MissingNarrowingReason {
                        family_id: family_id.clone(),
                        guarantee,
                        qualification_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the blocking findings for one qualified durable binding.
fn compute_qualified_findings(
    binding: &M5DurableBinding,
    high_salience: bool,
    family_id: &str,
    findings: &mut Vec<M5ActivityBlockingFinding>,
) {
    let guarantee = binding.guarantee;

    // Required captured-evidence projections.
    if binding.projected_durable_packet_ref.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_durable_packet_ref".to_owned(),
        });
    }
    if binding.projected_toast_independence.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_toast_independence".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "evidence_freshness".to_owned(),
        });
    }
    if guarantee.requires_reopen_outcome() && binding.projected_reopen_outcome.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_reopen_outcome".to_owned(),
        });
    }
    if guarantee.requires_survival() && binding.projected_survival.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_survival".to_owned(),
        });
    }
    if guarantee.requires_action_semantics() && binding.projected_action_semantics.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_action_semantics".to_owned(),
        });
    }
    if guarantee.requires_export_identity() && binding.projected_export_identity.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_export_identity".to_owned(),
        });
    }
    if guarantee.requires_fanout_honesty() && binding.projected_fanout_honesty.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_fanout_honesty".to_owned(),
        });
    }
    if high_salience && binding.projected_reopen_outcome.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingProjection {
            family_id: family_id.to_owned(),
            guarantee,
            field: "projected_reopen_outcome".to_owned(),
        });
    }

    // Red captured results.
    if binding.projected_durable_packet_ref.is_none() {
        findings.push(M5ActivityBlockingFinding::MissingDurablePacket {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_toast_independence == Some(M5ToastIndependence::ToastOnly) {
        findings.push(M5ActivityBlockingFinding::ToastOnlyTruth {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_reopen_outcome == Some(M5ReopenOutcome::TargetLost) {
        findings.push(M5ActivityBlockingFinding::ReopenTargetLost {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_survival == Some(M5SurvivalOutcome::Lost) {
        let finding = match guarantee {
            M5DurableGuarantee::ReopenAfterFocusLoss => {
                M5ActivityBlockingFinding::ReopenLostAfterFocusLoss {
                    family_id: family_id.to_owned(),
                    guarantee,
                }
            }
            M5DurableGuarantee::ReopenAfterRestart => {
                M5ActivityBlockingFinding::ReopenLostAfterRestart {
                    family_id: family_id.to_owned(),
                    guarantee,
                }
            }
            _ => M5ActivityBlockingFinding::ReopenLostUnderDegradedProvider {
                family_id: family_id.to_owned(),
                guarantee,
            },
        };
        findings.push(finding);
    }
    if binding.projected_action_semantics == Some(M5ActionSemantics::Collapsed) {
        findings.push(M5ActivityBlockingFinding::LifecycleActionsCollapsed {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_export_identity == Some(M5ExportIdentity::Reconstructed) {
        findings.push(M5ActivityBlockingFinding::ExportIdentityReconstructed {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_fanout_honesty == Some(M5FanoutHonesty::SilentFailure) {
        findings.push(M5ActivityBlockingFinding::FanoutFailureSilent {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
    if binding.marketed_on_guarantee
        && binding.evidence_freshness == Some(M5EvidenceFreshness::Stale)
    {
        findings.push(M5ActivityBlockingFinding::StaleEvidenceOnMarketedRow {
            family_id: family_id.to_owned(),
            guarantee,
        });
    }
}

/// Computes the per-guarantee coverage and per-class finding summary.
fn summarize_report(
    families: &[M5ActivityObjectRow],
) -> (Vec<M5ActivityCoverageSummary>, M5ActivityFindingSummary) {
    let mut coverage: Vec<M5ActivityCoverageSummary> = M5DurableGuarantee::required_guarantees()
        .iter()
        .map(|guarantee| M5ActivityCoverageSummary {
            guarantee: *guarantee,
            qualified_rows: 0,
            explicitly_narrowed_rows: 0,
            not_applicable_rows: 0,
            platform_omitted_rows: 0,
            declared_capture_gap_rows: 0,
            unqualified_local_history_rows: 0,
            missing_evidence_rows: 0,
        })
        .collect();
    let mut summary = M5ActivityFindingSummary::empty();

    for family in families {
        for binding in &family.bindings {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|row| row.guarantee == binding.guarantee)
            {
                match binding.qualification_status {
                    M5DurableStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5DurableStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5DurableStatus::NotApplicable => coverage_row.not_applicable_rows += 1,
                    M5DurableStatus::PlatformOmitted => coverage_row.platform_omitted_rows += 1,
                    M5DurableStatus::DeclaredCaptureGap => {
                        coverage_row.declared_capture_gap_rows += 1
                    }
                    M5DurableStatus::UnqualifiedLocalHistory => {
                        coverage_row.unqualified_local_history_rows += 1
                    }
                    M5DurableStatus::MissingEvidence => coverage_row.missing_evidence_rows += 1,
                }
            }
        }
        for finding in &family.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Computes the marketed rows release tooling should narrow because their
/// durable evidence is stale or red.
fn compute_narrowable_rows(families: &[M5ActivityObjectRow]) -> Vec<M5NarrowableRow> {
    let mut narrowable = Vec::new();
    for family in families {
        if !family.marketed {
            continue;
        }
        for finding in &family.blocking_findings {
            if let Some(guarantee) = finding.guarantee() {
                narrowable.push(M5NarrowableRow {
                    family_id: family.descriptor.family_id.clone(),
                    guarantee,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5ActivityObjectRow`] from a descriptor and its guarantee
/// bindings, computing the per-family blocking findings.
pub fn build_m5_activity_row(
    descriptor: M5ActivityObjectDescriptor,
    bindings: Vec<M5DurableBinding>,
) -> M5ActivityObjectRow {
    let high_salience = descriptor.is_high_salience();
    let marketed = descriptor.marketed_on_desktop;
    let blocking_findings = compute_family_findings(&descriptor, &bindings, high_salience);

    M5ActivityObjectRow {
        record_kind: M5_ACTIVITY_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_ACTIVITY_SCHEMA_VERSION,
        shared_contract_ref: M5_ACTIVITY_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_salience,
        marketed,
    }
}

/// Builds a full [`M5ActivityObjectReport`] from per-family rows.
pub fn build_m5_activity_object_audit(
    families: Vec<M5ActivityObjectRow>,
) -> M5ActivityObjectReport {
    let mut families = families;
    families.sort_by(|left, right| left.descriptor.family_id.cmp(&right.descriptor.family_id));

    let registered_family_count = families.len();
    let high_salience_family_count = families.iter().filter(|row| row.high_salience).count();
    let marketed_family_count = families.iter().filter(|row| row.marketed).count();
    let durable_guarantees_checked = families.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (guarantee_coverage, findings_summary) = summarize_report(&families);
    let narrowable_marketed_rows = compute_narrowable_rows(&families);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut reopen_anchor_index: Vec<M5ReopenAnchorEntry> = families
        .iter()
        .map(|family| M5ReopenAnchorEntry {
            job_family: family.descriptor.job_family,
            family_id: family.descriptor.family_id.clone(),
            reopen_anchor_ref: family.descriptor.reopen_anchor_ref.clone(),
        })
        .collect();
    reopen_anchor_index.sort_by(|left, right| left.family_id.cmp(&right.family_id));

    M5ActivityObjectReport {
        record_kind: M5_ACTIVITY_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_ACTIVITY_SCHEMA_VERSION,
        shared_contract_ref: M5_ACTIVITY_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_ACTIVITY_REPORT_ID.to_owned(),
        source_schema_ref: M5_ACTIVITY_SOURCE_SCHEMA_REF.to_owned(),
        required_guarantees: M5DurableGuarantee::required_guarantees().to_vec(),
        rows: families,
        guarantee_coverage,
        findings_summary,
        reopen_anchor_index,
        registered_family_count,
        high_salience_family_count,
        marketed_family_count,
        durable_guarantees_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_ACTIVITY_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_ACTIVITY_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_ACTIVITY_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/component-state-parity.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-activity-objects".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_activity_objects`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5ActivityValidationError {
    /// The audit has no registered families.
    NoRegisteredFamilies,
    /// A required durable guarantee has no qualified family.
    RequiredGuaranteeNotQualified { guarantee: String },
    /// A family is missing a required guarantee from its binding set.
    MissingRequiredGuarantee {
        family_id: String,
        guarantee: String,
    },
    /// A blocking finding remains on the family.
    BlockingFindingPresent {
        family_id: String,
        guarantee: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A family's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { family_id: String },
}

/// Validates an audit report against the M5 durable-attention acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_activity_objects(
    report: &M5ActivityObjectReport,
) -> Result<(), Vec<M5ActivityValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5ActivityValidationError::NoRegisteredFamilies);
    }

    for guarantee in M5DurableGuarantee::required_guarantees() {
        let any_qualified = report.rows.iter().any(|family| {
            family.bindings.iter().any(|binding| {
                binding.guarantee == guarantee
                    && binding.qualification_status == M5DurableStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(M5ActivityValidationError::RequiredGuaranteeNotQualified {
                guarantee: guarantee.as_str().to_owned(),
            });
        }
    }

    for family in &report.rows {
        for guarantee in M5DurableGuarantee::required_guarantees() {
            if !family
                .bindings
                .iter()
                .any(|binding| binding.guarantee == guarantee)
            {
                errors.push(M5ActivityValidationError::MissingRequiredGuarantee {
                    family_id: family.descriptor.family_id.clone(),
                    guarantee: guarantee.as_str().to_owned(),
                });
            }
        }
        if family.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(M5ActivityValidationError::MissingDescriptorRevisionRef {
                family_id: family.descriptor.family_id.clone(),
            });
        }
        for finding in &family.blocking_findings {
            errors.push(M5ActivityValidationError::BlockingFindingPresent {
                family_id: finding.family_id().to_owned(),
                guarantee: finding
                    .guarantee()
                    .map(|guarantee| guarantee.as_str().to_owned())
                    .unwrap_or_else(|| "family".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5ActivityValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5ActivityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_activity_objects_audit`].
struct FamilySeed {
    family_id: &'static str,
    job_family: M5ActivityJobFamily,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    reopen_anchor_ref: &'static str,
    support_note: &'static str,
    semantic_salience: M5ActivitySalience,
    lifecycle_label: M5FamilyLifecycle,
    supported_actions: &'static [M5ActivityAction],
    reopen_outcome: M5ReopenOutcome,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    guarantee: M5DurableGuarantee,
    qualification_status: M5DurableStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified guarantee with captured durable evidence.
const fn qualified(guarantee: M5DurableGuarantee) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5DurableStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: an honestly-declared capture gap with a documented reason.
const fn declared_capture_gap(guarantee: M5DurableGuarantee, reason: &'static str) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5DurableStatus::DeclaredCaptureGap,
        narrowing_reason: Some(reason),
        note: None,
    }
}

/// Helper: a not-applicable guarantee with a documented reason.
const fn not_applicable(guarantee: M5DurableGuarantee, reason: &'static str) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5DurableStatus::NotApplicable,
        narrowing_reason: Some(reason),
        note: None,
    }
}

use M5ActivityAction::{Acknowledge, Dismiss, Mute, Reopen, Resolve, Snooze};
use M5DurableGuarantee::{
    ActivityCenterLanding, CompanionFanoutHonesty, ExactTargetReopen, LifecycleActionSemantics,
    ReopenAfterDegradedProvider, ReopenAfterFocusLoss, ReopenAfterRestart, SupportExportIdentity,
};

const FAMILY_SEEDS: &[FamilySeed] = &[
    // Notebook execution session. Risk-bearing; long-running and failable.
    FamilySeed {
        family_id: "activity:notebook_run",
        job_family: M5ActivityJobFamily::NotebookRun,
        descriptor_revision_ref: "activity-rev:notebook_run:2026.06.01-01",
        primary_label_ref: "label:activity.notebook_run:primary",
        reopen_anchor_ref: "activity:reopen:notebook_run",
        support_note: "Each notebook run lands a durable row that reopens the exact cell and output, and survives focus loss, restart, and degraded providers.",
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Snooze, Acknowledge, Resolve, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Request / query run. Risk-bearing; remote provider work.
    FamilySeed {
        family_id: "activity:query_run",
        job_family: M5ActivityJobFamily::QueryRun,
        descriptor_revision_ref: "activity-rev:query_run:2026.06.01-01",
        primary_label_ref: "label:activity.query_run:primary",
        reopen_anchor_ref: "activity:reopen:query_run",
        support_note: "Each request or query run lands a durable row that reopens the exact request and result, with provider degradation labelled honestly.",
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Resolve, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Result-grid export. Lifecycle-bearing; reviewable artifact.
    FamilySeed {
        family_id: "activity:result_grid_export",
        job_family: M5ActivityJobFamily::ResultGridExport,
        descriptor_revision_ref: "activity-rev:result_grid_export:2026.06.01-01",
        primary_label_ref: "label:activity.result_grid_export:primary",
        reopen_anchor_ref: "activity:reopen:result_grid_export",
        support_note: "Each result-grid export lands a durable row that reopens the exact export artifact and its source grid.",
        semantic_salience: M5ActivitySalience::LifecycleBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Profiler capture. Review-bearing; local-only capture, no companion fanout.
    FamilySeed {
        family_id: "activity:profiler_capture",
        job_family: M5ActivityJobFamily::ProfilerCapture,
        descriptor_revision_ref: "activity-rev:profiler_capture:2026.06.01-01",
        primary_label_ref: "label:activity.profiler_capture:primary",
        reopen_anchor_ref: "activity:reopen:profiler_capture",
        support_note: "Each profiler capture lands a durable row that reopens the exact capture; the capture is local-only and declares no companion fanout.",
        semantic_salience: M5ActivitySalience::ReviewBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            not_applicable(
                CompanionFanoutHonesty,
                "profiler_captures_are_local_only_so_there_is_no_companion_fanout_to_label",
            ),
        ],
    },
    // Trace replay session. Review-bearing; reviewable timeline.
    FamilySeed {
        family_id: "activity:replay_session",
        job_family: M5ActivityJobFamily::ReplaySession,
        descriptor_revision_ref: "activity-rev:replay_session:2026.06.01-01",
        primary_label_ref: "label:activity.replay_session:primary",
        reopen_anchor_ref: "activity:reopen:replay_session",
        support_note: "Each replay session lands a durable row that reopens the exact trace and playhead position.",
        semantic_salience: M5ActivitySalience::ReviewBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Pipeline rerun / cancel action. Risk-bearing; status object.
    FamilySeed {
        family_id: "activity:pipeline_action",
        job_family: M5ActivityJobFamily::PipelineAction,
        descriptor_revision_ref: "activity-rev:pipeline_action:2026.06.01-01",
        primary_label_ref: "label:activity.pipeline_action:primary",
        reopen_anchor_ref: "activity:reopen:pipeline_action",
        support_note: "Each pipeline rerun or cancel lands a durable row that reopens the exact pipeline run it acted on.",
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Mute, Acknowledge, Resolve, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Preview route. Lifecycle-bearing; live preview object.
    FamilySeed {
        family_id: "activity:preview_route",
        job_family: M5ActivityJobFamily::PreviewRoute,
        descriptor_revision_ref: "activity-rev:preview_route:2026.06.01-01",
        primary_label_ref: "label:activity.preview_route:primary",
        reopen_anchor_ref: "activity:reopen:preview_route",
        support_note: "Each preview route lands a durable row that reopens the exact route and scope it was serving.",
        semantic_salience: M5ActivitySalience::LifecycleBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Sync state change. Risk-bearing; conflict-capable.
    FamilySeed {
        family_id: "activity:sync_state_change",
        job_family: M5ActivityJobFamily::SyncStateChange,
        descriptor_revision_ref: "activity-rev:sync_state_change:2026.06.01-01",
        primary_label_ref: "label:activity.sync_state_change:primary",
        reopen_anchor_ref: "activity:reopen:sync_state_change",
        support_note: "Each sync state change lands a durable row that reopens the exact workspace and conflict it concerns, with provider degradation labelled honestly.",
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Mute, Snooze, Acknowledge, Resolve, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
    // Offboarding job. Risk-bearing; destructive, local-only.
    FamilySeed {
        family_id: "activity:offboarding_job",
        job_family: M5ActivityJobFamily::OffboardingJob,
        descriptor_revision_ref: "activity-rev:offboarding_job:2026.06.01-01",
        primary_label_ref: "label:activity.offboarding_job:primary",
        reopen_anchor_ref: "activity:reopen:offboarding_job",
        support_note: "Each offboarding job lands a durable row that reopens the exact export-and-wipe job; it runs local-only and declares its companion fanout gap honestly.",
        semantic_salience: M5ActivitySalience::RiskBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Resolve, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            declared_capture_gap(
                CompanionFanoutHonesty,
                "offboarding_runs_local_only_so_companion_fanout_is_declared_not_emitted",
            ),
        ],
    },
    // Incident-packet generation. Review-bearing; reviewable evidence packet.
    FamilySeed {
        family_id: "activity:incident_packet",
        job_family: M5ActivityJobFamily::IncidentPacket,
        descriptor_revision_ref: "activity-rev:incident_packet:2026.06.01-01",
        primary_label_ref: "label:activity.incident_packet:primary",
        reopen_anchor_ref: "activity:reopen:incident_packet",
        support_note: "Each incident-packet generation lands a durable row that reopens the exact packet and the incident it documents.",
        semantic_salience: M5ActivitySalience::ReviewBearing,
        lifecycle_label: M5FamilyLifecycle::Beta,
        supported_actions: &[Dismiss, Acknowledge, Resolve, Reopen],
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: &[
            qualified(ActivityCenterLanding),
            qualified(ExactTargetReopen),
            qualified(ReopenAfterFocusLoss),
            qualified(ReopenAfterRestart),
            qualified(ReopenAfterDegradedProvider),
            qualified(LifecycleActionSemantics),
            qualified(SupportExportIdentity),
            qualified(CompanionFanoutHonesty),
        ],
    },
];

fn build_binding_from_seed(seed: &FamilySeed, binding_seed: &BindingSeed) -> M5DurableBinding {
    let guarantee = binding_seed.guarantee;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_salience = seed.semantic_salience.is_high_salience();
    let marketed_on_guarantee = !matches!(
        binding_seed.qualification_status,
        M5DurableStatus::NotApplicable | M5DurableStatus::PlatformOmitted
    );

    M5DurableBinding {
        guarantee,
        aspect: guarantee.canonical_aspect(),
        qualification_status: binding_seed.qualification_status,
        marketed_on_guarantee,
        projected_durable_packet_ref: qualified
            .then(|| format!("durable-packet:{}:{}", seed.family_id, guarantee.as_str())),
        projected_reopen_outcome: (qualified
            && (guarantee.requires_reopen_outcome() || high_salience))
            .then_some(seed.reopen_outcome),
        projected_toast_independence: qualified.then_some(M5ToastIndependence::Durable),
        projected_survival: (qualified && guarantee.requires_survival())
            .then_some(M5SurvivalOutcome::Survives),
        projected_action_semantics: (qualified && guarantee.requires_action_semantics())
            .then_some(M5ActionSemantics::Differentiated),
        projected_export_identity: (qualified && guarantee.requires_export_identity())
            .then_some(M5ExportIdentity::StableReference),
        projected_fanout_honesty: (qualified && guarantee.requires_fanout_honesty())
            .then_some(M5FanoutHonesty::HonestlyLabeled),
        evidence_freshness: qualified.then_some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_family_from_seed(seed: &FamilySeed) -> M5ActivityObjectRow {
    let descriptor = M5ActivityObjectDescriptor {
        family_id: seed.family_id.to_owned(),
        job_family: seed.job_family,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        reopen_anchor_ref: seed.reopen_anchor_ref.to_owned(),
        support_note: seed.support_note.to_owned(),
        semantic_salience: seed.semantic_salience,
        lifecycle_label: seed.lifecycle_label,
        supported_actions: seed.supported_actions.to_vec(),
        marketed_on_desktop: true,
        registered_on_activity_center: true,
    };
    let bindings: Vec<M5DurableBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_activity_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5/activity-center/`.
pub fn seeded_m5_activity_objects_audit() -> M5ActivityObjectReport {
    let families = FAMILY_SEEDS.iter().map(build_family_from_seed).collect();
    build_m5_activity_object_audit(families)
}
