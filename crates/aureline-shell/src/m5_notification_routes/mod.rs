//! Notification privacy, quiet-hours, badge, dedupe, and admin-suppression
//! qualification audit for the M5 depth notification sources.
//!
//! The M5 depth lanes mint new notification sources — notebook-run outcomes,
//! data/API request results, pipeline rerun/cancel outcomes, profiler-capture
//! completions, preview-route state changes, companion summaries, incident
//! packets, sync state changes, and offboarding jobs. Each is easy to ship as
//! a raw toast that bypasses the one governed router, leaks workspace or
//! secret detail onto a lock screen, floods the user with semantically
//! identical retries, or paints an opaque badge counter. This module carries
//! the stable v1 shell promise forward into those lanes: every marketed M5
//! notification source MUST flow through the one governed notification
//! envelope, MUST carry a declared privacy class, MUST keep its lock-screen
//! and companion copy summary-first, MUST honour quiet-hours and admin
//! suppression without erasing the durable object, MUST coalesce repeated
//! failures by root cause, MUST derive badge counts from durable item state,
//! MUST reopen the exact authoritative object, and MUST stay support-safe so
//! support bundles and companion fanout refer back to the same envelope
//! instead of leaking raw provider payloads.
//!
//! The audit projects, for each registered M5 notification source, the
//! canonical source descriptor against the qualification result the source
//! actually certifies for each of the nine notification guarantees the M5
//! lanes must pass:
//!
//! - `privacy_classification`
//! - `lock_screen_privacy`
//! - `payload_minimization`
//! - `quiet_hours_policy`
//! - `admin_suppression`
//! - `root_cause_dedupe`
//! - `badge_semantics`
//! - `exact_target_reopen`
//! - `companion_fanout_honesty`
//!
//! The resulting [`M5NotificationRouteReport`] is the canonical truth object
//! for the M5 notification-privacy lane. It is consumed by:
//!
//! - the live shell notification router / activity-center / support inspector
//!   (so the in-product audit quotes the same per-source findings the CLI
//!   prints);
//! - the headless inspector (`aureline_shell_m5_notification_routes`), which is
//!   the only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/ux/m5/notification-dedupe/`;
//! - the support-export wrapper that lets a reviewer pivot from a support case
//!   to the source that leaked, flooded, or bypassed suppression;
//! - the markdown audit under
//!   `artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md`
//!   (rendered from the same seed); and
//! - the cross-surface hardening matrix and release-center packets, which
//!   ingest the audit directly when qualifying or narrowing a marketed M5
//!   notification source whose privacy/quiet-hours/dedupe evidence is stale or
//!   red.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 notification source must declare a qualification
//!    binding for each of the nine notification guarantees.
//! 2. Every source must carry a canonical exact-target reopen anchor, a
//!    non-empty support note, a declared privacy class, at least one declared
//!    fanout channel, and a flag asserting it rides the one governed router; a
//!    missing anchor, missing note, missing channel, or a source that invents
//!    its own feature-local notification rule is a blocker.
//! 3. A qualified guarantee must carry the captured evidence the guarantee
//!    requires — a notification-envelope ref, a declared privacy class, a
//!    lock-screen disclosure, and an evidence-freshness stamp for every
//!    guarantee; a payload-minimization outcome for the payload guarantee; a
//!    quiet-hours outcome for the quiet-hours guarantee; an admin-suppression
//!    outcome for the admin guarantee; a dedupe outcome for the dedupe
//!    guarantee; a badge outcome for the badge guarantee; a reopen outcome for
//!    the reopen guarantee; and an honest fanout label for the companion
//!    guarantee. A red result (a lock-screen leak, a secret-bearing payload, a
//!    bypassed quiet-hours window, an overridden admin suppression, a duplicate
//!    flood, a raw-event badge counter, a lost reopen target, or a silent
//!    fanout failure) is a blocker.
//! 4. A source that emits notifications through an ad-hoc feature-local rule
//!    outside the governed router (`unqualified_local_rule`), and a marketed
//!    guarantee claimed with no captured evidence (`missing_evidence`), are
//!    blockers.
//! 5. Stale durable evidence on a marketed guarantee is a blocker, so release
//!    tooling can narrow a marketed M5 source instead of shipping it as
//!    implicitly stable.
//! 6. At least one source must qualify each of the nine guarantees so the
//!    audit cannot regress into a single happy-path source.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/notification-dedupe/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_m5_notification_routes_audit`].

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// Schema version exported with every M5 notification-route record.
pub const M5_NOTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, docs, and support export.
pub const M5_NOTIFICATION_SHARED_CONTRACT_REF: &str = "shell:m5_notification_routes:v1";

/// Stable record kind for the audit report payload.
pub const M5_NOTIFICATION_REPORT_RECORD_KIND: &str = "shell_m5_notification_route_report_record";

/// Stable record kind for one per-source qualification row.
pub const M5_NOTIFICATION_ROW_RECORD_KIND: &str = "shell_m5_notification_route_row_record";

/// Stable record kind for the support-export wrapper.
pub const M5_NOTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_notification_route_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_NOTIFICATION_REPORT_ID: &str = "shell:m5_notification_routes:audit:v1";

/// Stable support-export id.
pub const M5_NOTIFICATION_SUPPORT_EXPORT_ID: &str = "support-export:m5-notification-routes:001";

/// Source schema ref for the canonical contract.
pub const M5_NOTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-notification-envelope-diff.schema.json";

/// Markdown publication ref this audit is rendered to.
pub const M5_NOTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md";

/// Companion doc publication ref.
pub const M5_NOTIFICATION_PUBLISHED_DOC_REF: &str = "docs/m5/notification-privacy-and-badges.md";

/// One M5 depth notification source whose privacy and interruptibility
/// guarantees the audit qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationSource {
    /// Notebook execution outcome.
    NotebookRun,
    /// Request or query (data/API) run outcome.
    DataApiRun,
    /// Pipeline rerun/cancel outcome.
    PipelineAction,
    /// Profiler capture completion.
    ProfilerCapture,
    /// Live preview route state change.
    PreviewRoute,
    /// Companion summary fanout.
    CompanionSummary,
    /// Incident-packet generation outcome.
    IncidentPacket,
    /// Workspace sync state change.
    SyncStateChange,
    /// Offboarding / export-and-wipe job outcome.
    OffboardingJob,
}

impl M5NotificationSource {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRun => "notebook_run",
            Self::DataApiRun => "data_api_run",
            Self::PipelineAction => "pipeline_action",
            Self::ProfilerCapture => "profiler_capture",
            Self::PreviewRoute => "preview_route",
            Self::CompanionSummary => "companion_summary",
            Self::IncidentPacket => "incident_packet",
            Self::SyncStateChange => "sync_state_change",
            Self::OffboardingJob => "offboarding_job",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NotebookRun => "Notebook run",
            Self::DataApiRun => "Data/API run",
            Self::PipelineAction => "Pipeline action",
            Self::ProfilerCapture => "Profiler capture",
            Self::PreviewRoute => "Preview route",
            Self::CompanionSummary => "Companion summary",
            Self::IncidentPacket => "Incident packet",
            Self::SyncStateChange => "Sync state change",
            Self::OffboardingJob => "Offboarding job",
        }
    }
}

/// Notification privacy class assigned to a source.
///
/// `security_critical` and `managed_sensitive` are the high-stakes classes:
/// their notifications must always carry an exact-target reopen affordance and
/// a non-empty suppression-control set, so the audit requires a present reopen
/// outcome on every qualified guarantee and non-empty suppression controls on
/// the descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationPrivacyClass {
    /// Summary-safe; carries no workspace, code, or secret detail.
    SummarySafe,
    /// Workspace-sensitive; may reference workspace content by reference only.
    WorkspaceSensitive,
    /// Security-critical; concerns credentials, approvals, or high-risk action.
    SecurityCritical,
    /// Managed-sensitive; governed by admin policy and managed-depth rules.
    ManagedSensitive,
}

impl M5NotificationPrivacyClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummarySafe => "summary_safe",
            Self::WorkspaceSensitive => "workspace_sensitive",
            Self::SecurityCritical => "security_critical",
            Self::ManagedSensitive => "managed_sensitive",
        }
    }

    /// `true` for the classes whose source is high-stakes for the audit.
    pub const fn is_high_stakes(self) -> bool {
        matches!(self, Self::SecurityCritical | Self::ManagedSensitive)
    }
}

/// The notification aspect a guarantee belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationAspect {
    /// Privacy classification, lock-screen, and payload minimization.
    Privacy,
    /// Quiet-hours and admin suppression.
    Suppression,
    /// Root-cause dedupe and badge semantics.
    Dedupe,
    /// Exact-target reopen and companion fanout honesty.
    Routing,
}

impl M5NotificationAspect {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Privacy => "privacy",
            Self::Suppression => "suppression",
            Self::Dedupe => "dedupe",
            Self::Routing => "routing",
        }
    }
}

/// One notification guarantee a source certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationGuarantee {
    /// The source declares a privacy class instead of defaulting to clear.
    PrivacyClassification,
    /// Lock-screen / OS-notification copy is summary-first and leaks no detail.
    LockScreenPrivacy,
    /// Notification packets and support exports carry stable enums, not
    /// secret-bearing payloads.
    PayloadMinimization,
    /// Quiet-hours suppression changes fanout without erasing the durable
    /// object or reopen target.
    QuietHoursPolicy,
    /// Admin suppression is honoured without erasing the durable object or
    /// audit trail.
    AdminSuppression,
    /// Repeated failures or retries from one root cause coalesce instead of
    /// flooding.
    RootCauseDedupe,
    /// Badge counts and companion summaries are derived from durable item
    /// state, not raw event fanout.
    BadgeSemantics,
    /// Notifications and badges reopen the exact authoritative object.
    ExactTargetReopen,
    /// Companion fanout refers to the same object and labels stale/failed
    /// fanout honestly.
    CompanionFanoutHonesty,
}

impl M5NotificationGuarantee {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrivacyClassification => "privacy_classification",
            Self::LockScreenPrivacy => "lock_screen_privacy",
            Self::PayloadMinimization => "payload_minimization",
            Self::QuietHoursPolicy => "quiet_hours_policy",
            Self::AdminSuppression => "admin_suppression",
            Self::RootCauseDedupe => "root_cause_dedupe",
            Self::BadgeSemantics => "badge_semantics",
            Self::ExactTargetReopen => "exact_target_reopen",
            Self::CompanionFanoutHonesty => "companion_fanout_honesty",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::PrivacyClassification => "Privacy classification",
            Self::LockScreenPrivacy => "Lock-screen privacy",
            Self::PayloadMinimization => "Payload minimization",
            Self::QuietHoursPolicy => "Quiet-hours policy",
            Self::AdminSuppression => "Admin suppression",
            Self::RootCauseDedupe => "Root-cause dedupe",
            Self::BadgeSemantics => "Badge semantics",
            Self::ExactTargetReopen => "Exact-target reopen",
            Self::CompanionFanoutHonesty => "Companion fanout honesty",
        }
    }

    /// The nine notification guarantees, in canonical order.
    pub const fn required_guarantees() -> [Self; 9] {
        [
            Self::PrivacyClassification,
            Self::LockScreenPrivacy,
            Self::PayloadMinimization,
            Self::QuietHoursPolicy,
            Self::AdminSuppression,
            Self::RootCauseDedupe,
            Self::BadgeSemantics,
            Self::ExactTargetReopen,
            Self::CompanionFanoutHonesty,
        ]
    }

    /// The aspect this guarantee belongs to.
    pub const fn canonical_aspect(self) -> M5NotificationAspect {
        match self {
            Self::PrivacyClassification | Self::LockScreenPrivacy | Self::PayloadMinimization => {
                M5NotificationAspect::Privacy
            }
            Self::QuietHoursPolicy | Self::AdminSuppression => M5NotificationAspect::Suppression,
            Self::RootCauseDedupe | Self::BadgeSemantics => M5NotificationAspect::Dedupe,
            Self::ExactTargetReopen | Self::CompanionFanoutHonesty => M5NotificationAspect::Routing,
        }
    }

    /// `true` when a qualified binding must carry a payload-minimization
    /// outcome.
    pub const fn requires_payload_disclosure(self) -> bool {
        matches!(self, Self::PayloadMinimization)
    }

    /// `true` when a qualified binding must carry a quiet-hours outcome.
    pub const fn requires_quiet_hours(self) -> bool {
        matches!(self, Self::QuietHoursPolicy)
    }

    /// `true` when a qualified binding must carry an admin-suppression outcome.
    pub const fn requires_admin_suppression(self) -> bool {
        matches!(self, Self::AdminSuppression)
    }

    /// `true` when a qualified binding must carry a dedupe outcome.
    pub const fn requires_dedupe(self) -> bool {
        matches!(self, Self::RootCauseDedupe)
    }

    /// `true` when a qualified binding must carry a badge outcome.
    pub const fn requires_badge(self) -> bool {
        matches!(self, Self::BadgeSemantics)
    }

    /// `true` when a qualified binding must carry a reopen outcome.
    pub const fn requires_reopen_outcome(self) -> bool {
        matches!(self, Self::ExactTargetReopen)
    }

    /// `true` when a qualified binding must carry an honest fanout label.
    pub const fn requires_fanout_honesty(self) -> bool {
        matches!(self, Self::CompanionFanoutHonesty)
    }
}

/// Qualification status a source reports for one notification guarantee.
///
/// Only `Qualified` rows project captured evidence and are drift/red checked.
/// `ExplicitlyNarrowed`, `NotApplicable`, `PlatformOmitted`, and
/// `DeclaredCaptureGap` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnqualifiedLocalRule` (a feature-local notification
/// rule that bypasses the governed router) and `MissingEvidence` are blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationStatus {
    /// The guarantee is qualified with captured evidence.
    Qualified,
    /// The source narrows this guarantee; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The guarantee does not apply to this source; a reason MUST be set.
    NotApplicable,
    /// The guarantee is not surfaced on this client/platform; a reason MUST be
    /// set.
    PlatformOmitted,
    /// A provider-backed source declares a known capture gap honestly; a reason
    /// MUST be set.
    DeclaredCaptureGap,
    /// The source emits notifications through a feature-local rule that
    /// bypasses the governed router. Always a blocker.
    UnqualifiedLocalRule,
    /// A marketed guarantee is claimed with no captured evidence. Always a
    /// blocker.
    MissingEvidence,
}

impl M5NotificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::PlatformOmitted => "platform_omitted",
            Self::DeclaredCaptureGap => "declared_capture_gap",
            Self::UnqualifiedLocalRule => "unqualified_local_rule",
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

/// Whether the lock-screen / OS-notification copy stays summary-first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LockScreenDisclosure {
    /// The lock-screen copy is a bounded summary or open-app affordance only.
    SummaryOnly,
    /// The lock-screen copy leaks workspace, code, or secret detail. Always a
    /// blocker.
    LeaksDetail,
}

impl M5LockScreenDisclosure {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummaryOnly => "summary_only",
            Self::LeaksDetail => "leaks_detail",
        }
    }
}

/// Whether the notification packet keeps payloads minimized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PayloadDisclosure {
    /// The packet carries stable class and outcome enums only.
    EnumsOnly,
    /// The packet carries a secret-bearing or raw provider payload by default.
    /// Always a blocker.
    CarriesSecretBody,
}

impl M5PayloadDisclosure {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnumsOnly => "enums_only",
            Self::CarriesSecretBody => "carries_secret_body",
        }
    }
}

/// Whether quiet-hours suppression preserves the durable object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QuietHoursOutcome {
    /// Quiet-hours suppression changes fanout but preserves the durable object
    /// and reopen target.
    Respected,
    /// Quiet hours are bypassed by a feature-local exception. Always a blocker.
    Bypassed,
}

impl M5QuietHoursOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Respected => "respected",
            Self::Bypassed => "bypassed",
        }
    }
}

/// Whether admin suppression is honoured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdminSuppressionOutcome {
    /// Admin suppression changes fanout but preserves the durable object and
    /// audit trail.
    Honored,
    /// Admin suppression is overridden by a feature-local exception. Always a
    /// blocker.
    Overridden,
}

impl M5AdminSuppressionOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Honored => "honored",
            Self::Overridden => "overridden",
        }
    }
}

/// Whether repeated failures coalesce by root cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DedupeOutcome {
    /// Repeated failures or retries from one root cause coalesce into one
    /// durable item.
    CoalescedByRootCause,
    /// Semantically identical alerts flood the user. Always a blocker.
    FloodsDuplicates,
}

impl M5DedupeOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoalescedByRootCause => "coalesced_by_root_cause",
            Self::FloodsDuplicates => "floods_duplicates",
        }
    }
}

/// Whether badge counts are derived from durable item state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeOutcome {
    /// Badge counts derive from a durable count class and stay correct after
    /// retries and partial delivery.
    DurableCountClass,
    /// Badge counts reflect raw event fanout and drift after retries. Always a
    /// blocker.
    RawEventFanout,
}

impl M5BadgeOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableCountClass => "durable_count_class",
            Self::RawEventFanout => "raw_event_fanout",
        }
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
    /// The source has no reopen target on this guarantee.
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

/// Freshness of the captured evidence.
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

/// A suppression / interruptibility control a source exposes on its envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuppressionControl {
    /// Honour the user's quiet-hours window.
    QuietHours,
    /// Honour admin suppression policy.
    AdminSuppress,
    /// Mute future notifications for the object.
    Mute,
    /// Snooze the notification until later.
    Snooze,
    /// Show only a bounded lock-screen summary.
    LockScreenSummary,
    /// Degrade to a bounded summary / open-app affordance when detail cannot be
    /// shown safely.
    BoundedSummaryFallback,
}

impl M5SuppressionControl {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuietHours => "quiet_hours",
            Self::AdminSuppress => "admin_suppress",
            Self::Mute => "mute",
            Self::Snooze => "snooze",
            Self::LockScreenSummary => "lock_screen_summary",
            Self::BoundedSummaryFallback => "bounded_summary_fallback",
        }
    }
}

/// A notification channel a source fans out to.
///
/// The channel set is fixed: the M5 sources harden the channels Aureline
/// already claims and never expand the channel set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotificationChannel {
    /// In-app desktop toast.
    DesktopToast,
    /// Native OS notification / notification center.
    NativeOsNotification,
    /// Durable activity-center row.
    ActivityCenterRow,
    /// Companion summary surface.
    CompanionSummary,
}

impl M5NotificationChannel {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopToast => "desktop_toast",
            Self::NativeOsNotification => "native_os_notification",
            Self::ActivityCenterRow => "activity_center_row",
            Self::CompanionSummary => "companion_summary",
        }
    }
}

/// Lifecycle label retained on the canonical source descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; sources must point at the replacement.
    Deprecated,
}

impl M5SourceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Canonical descriptor for one M5 notification source's privacy and
/// interruptibility contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationSourceDescriptor {
    /// Stable source id (e.g. `notify:notebook_run`).
    pub source_id: String,
    /// Notification source the descriptor belongs to.
    pub notification_source: M5NotificationSource,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical exact-target reopen anchor ref the notification reopens the
    /// authoritative object from.
    pub reopen_anchor_ref: String,
    /// Support note retained on the descriptor. MUST be non-empty.
    pub support_note: String,
    /// Declared privacy class.
    pub privacy_class: M5NotificationPrivacyClass,
    /// Pinned source lifecycle label.
    pub lifecycle_label: M5SourceLifecycle,
    /// Suppression / interruptibility controls the source exposes, in canonical
    /// order.
    pub suppression_controls: Vec<M5SuppressionControl>,
    /// Channels the source fans out to. MUST be non-empty for a marketed
    /// source.
    pub fanout_channels: Vec<M5NotificationChannel>,
    /// `true` when the source is marketed on desktop and therefore must pass
    /// the claimed matrix or narrow accordingly.
    pub marketed_on_desktop: bool,
    /// `true` once the source rides the one governed notification router and
    /// does not invent a feature-local rule. MUST be `true`.
    pub routed_through_governed_router: bool,
}

impl M5NotificationSourceDescriptor {
    /// `true` when this source's privacy class makes it high-stakes for the
    /// audit.
    pub const fn is_high_stakes(&self) -> bool {
        self.privacy_class.is_high_stakes()
    }
}

/// Per-guarantee binding a source reports for one notification guarantee.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationBinding {
    /// Guarantee this binding covers.
    pub guarantee: M5NotificationGuarantee,
    /// Aspect projected for the guarantee. MUST equal the guarantee's canonical
    /// aspect.
    pub aspect: M5NotificationAspect,
    /// Qualification status the source reports.
    pub qualification_status: M5NotificationStatus,
    /// `true` when the source is marketed on this guarantee.
    pub marketed_on_guarantee: bool,
    /// Captured notification-envelope ref (`None` for non-qualified rows).
    pub projected_envelope_ref: Option<String>,
    /// Captured privacy class (`None` for non-qualified rows).
    pub projected_privacy_class: Option<M5NotificationPrivacyClass>,
    /// Captured lock-screen disclosure (`None` for non-qualified rows).
    pub projected_lock_screen: Option<M5LockScreenDisclosure>,
    /// Captured payload-minimization outcome (`None` unless the guarantee
    /// requires it).
    pub projected_payload_disclosure: Option<M5PayloadDisclosure>,
    /// Captured quiet-hours outcome (`None` unless the guarantee requires it).
    pub projected_quiet_hours: Option<M5QuietHoursOutcome>,
    /// Captured admin-suppression outcome (`None` unless the guarantee requires
    /// it).
    pub projected_admin_suppression: Option<M5AdminSuppressionOutcome>,
    /// Captured dedupe outcome (`None` unless the guarantee requires it).
    pub projected_dedupe: Option<M5DedupeOutcome>,
    /// Captured badge outcome (`None` unless the guarantee requires it).
    pub projected_badge: Option<M5BadgeOutcome>,
    /// Captured reopen outcome (`None` unless the guarantee requires it or the
    /// source is high-stakes).
    pub projected_reopen_outcome: Option<M5ReopenOutcome>,
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
pub enum M5NotificationBlockingFinding {
    /// A source emits notifications through a feature-local rule outside the
    /// governed router.
    UnqualifiedLocalRule {
        /// Source that exposes the gap.
        source_id: String,
        /// Guarantee that exposes the gap.
        guarantee: M5NotificationGuarantee,
    },
    /// A marketed guarantee is claimed with no captured evidence.
    MissingEvidence {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A qualified guarantee is missing its captured notification envelope.
    MissingEnvelopeRef {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee leaks workspace, code, or secret detail onto a lock screen.
    LockScreenLeak {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee carries a secret-bearing or raw provider payload by default.
    SecretBearingPayload {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee bypasses the quiet-hours window.
    QuietHoursBypassed {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee overrides admin suppression.
    AdminSuppressionOverridden {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee floods the user with semantically identical alerts.
    DuplicateFlood {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee paints a badge from raw event fanout instead of durable
    /// item state.
    BadgeRawEventFanout {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A guarantee loses the exact-target reopen affordance.
    ReopenTargetLost {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A failed companion fanout is hidden behind a silent success.
    FanoutFailureSilent {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A marketed guarantee carries stale evidence.
    StaleEvidenceOnMarketedRow {
        source_id: String,
        guarantee: M5NotificationGuarantee,
    },
    /// A binding projects an aspect that disagrees with the guarantee's
    /// canonical aspect.
    AspectDrift {
        source_id: String,
        guarantee: M5NotificationGuarantee,
        /// Projected aspect.
        projected_aspect: M5NotificationAspect,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        source_id: String,
        guarantee: M5NotificationGuarantee,
        qualification_status: M5NotificationStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        source_id: String,
        guarantee: M5NotificationGuarantee,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical exact-target reopen anchor.
    DescriptorMissingReopenAnchor { source_id: String },
    /// The descriptor carries no support note.
    MissingSupportNote { source_id: String },
    /// The source emits through a feature-local rule outside the governed
    /// router.
    SourceNotOnGovernedRouter { source_id: String },
    /// A high-stakes source exposes no suppression controls.
    MissingSuppressionControls { source_id: String },
    /// A marketed source declares no fanout channel.
    NoDeclaredChannel { source_id: String },
}

impl M5NotificationBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedLocalRule { .. } => "unqualified_local_rule",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingEnvelopeRef { .. } => "missing_envelope_ref",
            Self::LockScreenLeak { .. } => "lock_screen_leak",
            Self::SecretBearingPayload { .. } => "secret_bearing_payload",
            Self::QuietHoursBypassed { .. } => "quiet_hours_bypassed",
            Self::AdminSuppressionOverridden { .. } => "admin_suppression_overridden",
            Self::DuplicateFlood { .. } => "duplicate_flood",
            Self::BadgeRawEventFanout { .. } => "badge_raw_event_fanout",
            Self::ReopenTargetLost { .. } => "reopen_target_lost",
            Self::FanoutFailureSilent { .. } => "fanout_failure_silent",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::AspectDrift { .. } => "aspect_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingReopenAnchor { .. } => "descriptor_missing_reopen_anchor",
            Self::MissingSupportNote { .. } => "missing_support_note",
            Self::SourceNotOnGovernedRouter { .. } => "source_not_on_governed_router",
            Self::MissingSuppressionControls { .. } => "missing_suppression_controls",
            Self::NoDeclaredChannel { .. } => "no_declared_channel",
        }
    }

    /// Returns the source id this finding is attached to.
    pub fn source_id(&self) -> &str {
        match self {
            Self::UnqualifiedLocalRule { source_id, .. }
            | Self::MissingEvidence { source_id, .. }
            | Self::MissingEnvelopeRef { source_id, .. }
            | Self::LockScreenLeak { source_id, .. }
            | Self::SecretBearingPayload { source_id, .. }
            | Self::QuietHoursBypassed { source_id, .. }
            | Self::AdminSuppressionOverridden { source_id, .. }
            | Self::DuplicateFlood { source_id, .. }
            | Self::BadgeRawEventFanout { source_id, .. }
            | Self::ReopenTargetLost { source_id, .. }
            | Self::FanoutFailureSilent { source_id, .. }
            | Self::StaleEvidenceOnMarketedRow { source_id, .. }
            | Self::AspectDrift { source_id, .. }
            | Self::MissingNarrowingReason { source_id, .. }
            | Self::MissingProjection { source_id, .. }
            | Self::DescriptorMissingReopenAnchor { source_id }
            | Self::MissingSupportNote { source_id }
            | Self::SourceNotOnGovernedRouter { source_id }
            | Self::MissingSuppressionControls { source_id }
            | Self::NoDeclaredChannel { source_id } => source_id,
        }
    }

    /// Returns the guarantee this finding is attached to, when guarantee-scoped.
    pub fn guarantee(&self) -> Option<M5NotificationGuarantee> {
        match self {
            Self::UnqualifiedLocalRule { guarantee, .. }
            | Self::MissingEvidence { guarantee, .. }
            | Self::MissingEnvelopeRef { guarantee, .. }
            | Self::LockScreenLeak { guarantee, .. }
            | Self::SecretBearingPayload { guarantee, .. }
            | Self::QuietHoursBypassed { guarantee, .. }
            | Self::AdminSuppressionOverridden { guarantee, .. }
            | Self::DuplicateFlood { guarantee, .. }
            | Self::BadgeRawEventFanout { guarantee, .. }
            | Self::ReopenTargetLost { guarantee, .. }
            | Self::FanoutFailureSilent { guarantee, .. }
            | Self::StaleEvidenceOnMarketedRow { guarantee, .. }
            | Self::AspectDrift { guarantee, .. }
            | Self::MissingNarrowingReason { guarantee, .. }
            | Self::MissingProjection { guarantee, .. } => Some(*guarantee),
            Self::DescriptorMissingReopenAnchor { .. }
            | Self::MissingSupportNote { .. }
            | Self::SourceNotOnGovernedRouter { .. }
            | Self::MissingSuppressionControls { .. }
            | Self::NoDeclaredChannel { .. } => None,
        }
    }
}

/// One per-source notification qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationRouteRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the source.
    pub descriptor: M5NotificationSourceDescriptor,
    /// Guarantee-by-guarantee qualification bindings, in canonical order.
    pub bindings: Vec<M5NotificationBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5NotificationBlockingFinding>,
    /// `true` when the source's privacy class classifies it as high-stakes.
    pub high_stakes: bool,
    /// `true` when the source is marketed on desktop.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_local_rule` findings.
    pub unqualified_local_rule: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_envelope_ref` findings.
    pub missing_envelope_ref: usize,
    /// Number of `lock_screen_leak` findings.
    pub lock_screen_leak: usize,
    /// Number of `secret_bearing_payload` findings.
    pub secret_bearing_payload: usize,
    /// Number of `quiet_hours_bypassed` findings.
    pub quiet_hours_bypassed: usize,
    /// Number of `admin_suppression_overridden` findings.
    pub admin_suppression_overridden: usize,
    /// Number of `duplicate_flood` findings.
    pub duplicate_flood: usize,
    /// Number of `badge_raw_event_fanout` findings.
    pub badge_raw_event_fanout: usize,
    /// Number of `reopen_target_lost` findings.
    pub reopen_target_lost: usize,
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
    /// Number of `source_not_on_governed_router` findings.
    pub source_not_on_governed_router: usize,
    /// Number of `missing_suppression_controls` findings.
    pub missing_suppression_controls: usize,
    /// Number of `no_declared_channel` findings.
    pub no_declared_channel: usize,
}

impl M5NotificationFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_local_rule: 0,
            missing_evidence: 0,
            missing_envelope_ref: 0,
            lock_screen_leak: 0,
            secret_bearing_payload: 0,
            quiet_hours_bypassed: 0,
            admin_suppression_overridden: 0,
            duplicate_flood: 0,
            badge_raw_event_fanout: 0,
            reopen_target_lost: 0,
            fanout_failure_silent: 0,
            stale_evidence_on_marketed_row: 0,
            aspect_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_reopen_anchor: 0,
            missing_support_note: 0,
            source_not_on_governed_router: 0,
            missing_suppression_controls: 0,
            no_declared_channel: 0,
        }
    }

    fn record(&mut self, finding: &M5NotificationBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5NotificationBlockingFinding::UnqualifiedLocalRule { .. } => {
                self.unqualified_local_rule += 1
            }
            M5NotificationBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5NotificationBlockingFinding::MissingEnvelopeRef { .. } => {
                self.missing_envelope_ref += 1
            }
            M5NotificationBlockingFinding::LockScreenLeak { .. } => self.lock_screen_leak += 1,
            M5NotificationBlockingFinding::SecretBearingPayload { .. } => {
                self.secret_bearing_payload += 1
            }
            M5NotificationBlockingFinding::QuietHoursBypassed { .. } => {
                self.quiet_hours_bypassed += 1
            }
            M5NotificationBlockingFinding::AdminSuppressionOverridden { .. } => {
                self.admin_suppression_overridden += 1
            }
            M5NotificationBlockingFinding::DuplicateFlood { .. } => self.duplicate_flood += 1,
            M5NotificationBlockingFinding::BadgeRawEventFanout { .. } => {
                self.badge_raw_event_fanout += 1
            }
            M5NotificationBlockingFinding::ReopenTargetLost { .. } => self.reopen_target_lost += 1,
            M5NotificationBlockingFinding::FanoutFailureSilent { .. } => {
                self.fanout_failure_silent += 1
            }
            M5NotificationBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5NotificationBlockingFinding::AspectDrift { .. } => self.aspect_drift += 1,
            M5NotificationBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5NotificationBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5NotificationBlockingFinding::DescriptorMissingReopenAnchor { .. } => {
                self.descriptor_missing_reopen_anchor += 1
            }
            M5NotificationBlockingFinding::MissingSupportNote { .. } => {
                self.missing_support_note += 1
            }
            M5NotificationBlockingFinding::SourceNotOnGovernedRouter { .. } => {
                self.source_not_on_governed_router += 1
            }
            M5NotificationBlockingFinding::MissingSuppressionControls { .. } => {
                self.missing_suppression_controls += 1
            }
            M5NotificationBlockingFinding::NoDeclaredChannel { .. } => {
                self.no_declared_channel += 1
            }
        }
    }
}

/// Per-guarantee coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationCoverageSummary {
    /// Guarantee this summary covers.
    pub guarantee: M5NotificationGuarantee,
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
    /// Number of `unqualified_local_rule` rows on this guarantee.
    pub unqualified_local_rule_rows: usize,
    /// Number of `missing_evidence` rows on this guarantee.
    pub missing_evidence_rows: usize,
}

impl M5NotificationCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.platform_omitted_rows
            + self.declared_capture_gap_rows
    }
}

/// A single reopen-anchor index entry the audit publishes so the notification
/// router, docs, and release surfaces can reopen each source by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReopenAnchorEntry {
    /// Notification source the anchor belongs to.
    pub notification_source: M5NotificationSource,
    /// Source id the anchor reopens.
    pub source_id: String,
    /// Canonical exact-target reopen anchor ref.
    pub reopen_anchor_ref: String,
}

/// One marketed guarantee release tooling should narrow because its evidence is
/// stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NarrowableRow {
    /// Source id that must narrow.
    pub source_id: String,
    /// Guarantee that must narrow.
    pub guarantee: M5NotificationGuarantee,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 notification-route privacy and interruptibility qualification audit
/// report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationRouteReport {
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
    /// Required notification guarantees, in canonical order.
    pub required_guarantees: Vec<M5NotificationGuarantee>,
    /// Per-source qualification rows, sorted by `descriptor.source_id`.
    pub rows: Vec<M5NotificationRouteRow>,
    /// Per-guarantee coverage summary, in canonical order.
    pub guarantee_coverage: Vec<M5NotificationCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5NotificationFindingSummary,
    /// Canonical reopen-anchor index, sorted by source id.
    pub reopen_anchor_index: Vec<M5ReopenAnchorEntry>,
    /// Number of registered M5 sources present.
    pub registered_source_count: usize,
    /// Number of high-stakes sources present.
    pub high_stakes_source_count: usize,
    /// Number of sources marketed on desktop.
    pub marketed_source_count: usize,
    /// Total notification guarantees checked.
    pub notification_guarantees_checked: usize,
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

impl M5NotificationRouteReport {
    /// Returns `true` when every required guarantee is qualified by at least
    /// one source.
    pub fn every_required_guarantee_qualified(&self) -> bool {
        for guarantee in M5NotificationGuarantee::required_guarantees() {
            let any_qualified = self.rows.iter().any(|source| {
                source.bindings.iter().any(|binding| {
                    binding.guarantee == guarantee
                        && binding.qualification_status == M5NotificationStatus::Qualified
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
            "audit: sources={}, high_stakes={}, marketed={}, guarantees={}, blocking={}, clean={}",
            self.registered_source_count,
            self.high_stakes_source_count,
            self.marketed_source_count,
            self.notification_guarantees_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.guarantee_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, unqualified={}, missing_evidence={}",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_rule_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for source in &self.rows {
            for finding in &source.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.source_id(),
                    finding
                        .guarantee()
                        .map(M5NotificationGuarantee::as_str)
                        .unwrap_or("source"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_rows {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.source_id,
                narrowable.guarantee.as_str(),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 notification privacy, quiet-hours, and badge qualification audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_notification_routes`](../../../../crates/aureline-shell/src/m5_notification_routes/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- report-md > \\\n  artifacts/ux/m5/quiet-hours-and-privacy/m5_notification_routes_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Registered M5 sources: `{}`\n",
            self.registered_source_count
        ));
        out.push_str(&format!(
            "- High-stakes sources: `{}`\n",
            self.high_stakes_source_count
        ));
        out.push_str(&format!(
            "- Marketed sources: `{}`\n",
            self.marketed_source_count
        ));
        out.push_str(&format!(
            "- Notification guarantees checked: `{}`\n",
            self.notification_guarantees_checked
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
            "| Notification guarantee | Qualified | Narrowed | Unqualified | Missing evidence |\n\
             | ---------------------- | --------: | -------: | ----------: | ---------------: |\n",
        );
        for coverage in &self.guarantee_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.guarantee.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_rule_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_local_rule` | {} |\n",
            self.findings_summary.unqualified_local_rule
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_envelope_ref` | {} |\n",
            self.findings_summary.missing_envelope_ref
        ));
        out.push_str(&format!(
            "| `lock_screen_leak` | {} |\n",
            self.findings_summary.lock_screen_leak
        ));
        out.push_str(&format!(
            "| `secret_bearing_payload` | {} |\n",
            self.findings_summary.secret_bearing_payload
        ));
        out.push_str(&format!(
            "| `quiet_hours_bypassed` | {} |\n",
            self.findings_summary.quiet_hours_bypassed
        ));
        out.push_str(&format!(
            "| `admin_suppression_overridden` | {} |\n",
            self.findings_summary.admin_suppression_overridden
        ));
        out.push_str(&format!(
            "| `duplicate_flood` | {} |\n",
            self.findings_summary.duplicate_flood
        ));
        out.push_str(&format!(
            "| `badge_raw_event_fanout` | {} |\n",
            self.findings_summary.badge_raw_event_fanout
        ));
        out.push_str(&format!(
            "| `reopen_target_lost` | {} |\n",
            self.findings_summary.reopen_target_lost
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
            "| `source_not_on_governed_router` | {} |\n",
            self.findings_summary.source_not_on_governed_router
        ));
        out.push_str(&format!(
            "| `missing_suppression_controls` | {} |\n",
            self.findings_summary.missing_suppression_controls
        ));
        out.push_str(&format!(
            "| `no_declared_channel` | {} |\n\n",
            self.findings_summary.no_declared_channel
        ));

        out.push_str("## Reopen anchor index\n\n");
        out.push_str(
            "| Notification source | Source id | Reopen anchor |\n| ------------------- | --------- | ------------- |\n",
        );
        for entry in &self.reopen_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.notification_source.display_label(),
                entry.source_id,
                entry.reopen_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-source rows\n\n");
        for source in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {}, {})\n\n",
                source.descriptor.source_id,
                source.descriptor.notification_source.as_str(),
                source.descriptor.privacy_class.as_str(),
                source.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                source.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Privacy class: `{}`\n",
                source.descriptor.privacy_class.as_str()
            ));
            out.push_str(&format!(
                "- Reopen anchor: `{}`\n",
                source.descriptor.reopen_anchor_ref
            ));
            out.push_str(&format!(
                "- Suppression controls: {}\n",
                if source.descriptor.suppression_controls.is_empty() {
                    "none".to_owned()
                } else {
                    source
                        .descriptor
                        .suppression_controls
                        .iter()
                        .map(|control| format!("`{}`", control.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            out.push_str(&format!(
                "- Fanout channels: {}\n",
                if source.descriptor.fanout_channels.is_empty() {
                    "none".to_owned()
                } else {
                    source
                        .descriptor
                        .fanout_channels
                        .iter()
                        .map(|channel| format!("`{}`", channel.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            out.push_str(&format!(
                "- Marketed on desktop: `{}`\n",
                if source.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- High-stakes: `{}`\n\n",
                if source.high_stakes { "yes" } else { "no" }
            ));

            out.push_str(
                "| Notification guarantee | Status | Lock screen | Payload | Quiet hours | Admin | Dedupe | Badge | Reopen | Fanout | Freshness | Narrowing reason |\n\
                 | ---------------------- | ------ | ----------- | ------- | ----------- | ----- | ------ | ----- | ------ | ------ | --------- | ---------------- |\n",
            );
            for binding in &source.bindings {
                let lock_screen = binding
                    .projected_lock_screen
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let payload = binding
                    .projected_payload_disclosure
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let quiet_hours = binding
                    .projected_quiet_hours
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let admin = binding
                    .projected_admin_suppression
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let dedupe = binding
                    .projected_dedupe
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let badge = binding
                    .projected_badge
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let reopen = binding
                    .projected_reopen_outcome
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
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.guarantee.display_label(),
                    binding.qualification_status.as_str(),
                    lock_screen,
                    payload,
                    quiet_hours,
                    admin,
                    dedupe,
                    badge,
                    reopen,
                    fanout,
                    freshness,
                    narrowing,
                ));
            }
            out.push('\n');

            if source.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &source.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .guarantee()
                            .map(M5NotificationGuarantee::as_str)
                            .unwrap_or("source"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_notification_routes -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_notification_routes_fixtures\n");
        out.push_str("python3 tools/ci/m5/notification_routes_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 notification-route audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NotificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5NotificationRouteReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5NotificationSupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5NotificationRouteReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for source in &report.rows {
            case_ids.push(source.descriptor.source_id.clone());
            case_ids.push(source.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_NOTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_NOTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_NOTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-source blocking findings from a descriptor and its
/// guarantee bindings.
fn compute_source_findings(
    descriptor: &M5NotificationSourceDescriptor,
    bindings: &[M5NotificationBinding],
    high_stakes: bool,
) -> Vec<M5NotificationBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (source-scoped) findings.
    if descriptor.reopen_anchor_ref.trim().is_empty() {
        findings.push(
            M5NotificationBlockingFinding::DescriptorMissingReopenAnchor {
                source_id: descriptor.source_id.clone(),
            },
        );
    }
    if descriptor.support_note.trim().is_empty() {
        findings.push(M5NotificationBlockingFinding::MissingSupportNote {
            source_id: descriptor.source_id.clone(),
        });
    }
    if !descriptor.routed_through_governed_router {
        findings.push(M5NotificationBlockingFinding::SourceNotOnGovernedRouter {
            source_id: descriptor.source_id.clone(),
        });
    }
    if high_stakes && descriptor.suppression_controls.is_empty() {
        findings.push(M5NotificationBlockingFinding::MissingSuppressionControls {
            source_id: descriptor.source_id.clone(),
        });
    }
    if descriptor.marketed_on_desktop && descriptor.fanout_channels.is_empty() {
        findings.push(M5NotificationBlockingFinding::NoDeclaredChannel {
            source_id: descriptor.source_id.clone(),
        });
    }

    for binding in bindings {
        let guarantee = binding.guarantee;
        let source_id = descriptor.source_id.clone();

        // A binding's aspect must match its guarantee's canonical aspect.
        if binding.aspect != guarantee.canonical_aspect() {
            findings.push(M5NotificationBlockingFinding::AspectDrift {
                source_id: source_id.clone(),
                guarantee,
                projected_aspect: binding.aspect,
            });
        }

        match binding.qualification_status {
            M5NotificationStatus::UnqualifiedLocalRule => {
                findings.push(M5NotificationBlockingFinding::UnqualifiedLocalRule {
                    source_id: source_id.clone(),
                    guarantee,
                });
            }
            M5NotificationStatus::MissingEvidence => {
                findings.push(M5NotificationBlockingFinding::MissingEvidence {
                    source_id: source_id.clone(),
                    guarantee,
                });
            }
            M5NotificationStatus::Qualified => {
                compute_qualified_findings(binding, high_stakes, &source_id, &mut findings);
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5NotificationBlockingFinding::MissingNarrowingReason {
                        source_id: source_id.clone(),
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

/// Computes the blocking findings for one qualified notification binding.
fn compute_qualified_findings(
    binding: &M5NotificationBinding,
    high_stakes: bool,
    source_id: &str,
    findings: &mut Vec<M5NotificationBlockingFinding>,
) {
    let guarantee = binding.guarantee;

    // Required captured-evidence projections (universal for qualified rows).
    if binding.projected_envelope_ref.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_envelope_ref".to_owned(),
        });
    }
    if binding.projected_privacy_class.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_privacy_class".to_owned(),
        });
    }
    if binding.projected_lock_screen.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_lock_screen".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "evidence_freshness".to_owned(),
        });
    }

    // Guarantee-specific required projections.
    if guarantee.requires_payload_disclosure() && binding.projected_payload_disclosure.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_payload_disclosure".to_owned(),
        });
    }
    if guarantee.requires_quiet_hours() && binding.projected_quiet_hours.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_quiet_hours".to_owned(),
        });
    }
    if guarantee.requires_admin_suppression() && binding.projected_admin_suppression.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_admin_suppression".to_owned(),
        });
    }
    if guarantee.requires_dedupe() && binding.projected_dedupe.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_dedupe".to_owned(),
        });
    }
    if guarantee.requires_badge() && binding.projected_badge.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_badge".to_owned(),
        });
    }
    if guarantee.requires_reopen_outcome() && binding.projected_reopen_outcome.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_reopen_outcome".to_owned(),
        });
    }
    if guarantee.requires_fanout_honesty() && binding.projected_fanout_honesty.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_fanout_honesty".to_owned(),
        });
    }
    if high_stakes && binding.projected_reopen_outcome.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingProjection {
            source_id: source_id.to_owned(),
            guarantee,
            field: "projected_reopen_outcome".to_owned(),
        });
    }

    // Red captured results.
    if binding.projected_envelope_ref.is_none() {
        findings.push(M5NotificationBlockingFinding::MissingEnvelopeRef {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_lock_screen == Some(M5LockScreenDisclosure::LeaksDetail) {
        findings.push(M5NotificationBlockingFinding::LockScreenLeak {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_payload_disclosure == Some(M5PayloadDisclosure::CarriesSecretBody) {
        findings.push(M5NotificationBlockingFinding::SecretBearingPayload {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_quiet_hours == Some(M5QuietHoursOutcome::Bypassed) {
        findings.push(M5NotificationBlockingFinding::QuietHoursBypassed {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_admin_suppression == Some(M5AdminSuppressionOutcome::Overridden) {
        findings.push(M5NotificationBlockingFinding::AdminSuppressionOverridden {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_dedupe == Some(M5DedupeOutcome::FloodsDuplicates) {
        findings.push(M5NotificationBlockingFinding::DuplicateFlood {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_badge == Some(M5BadgeOutcome::RawEventFanout) {
        findings.push(M5NotificationBlockingFinding::BadgeRawEventFanout {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_reopen_outcome == Some(M5ReopenOutcome::TargetLost) {
        findings.push(M5NotificationBlockingFinding::ReopenTargetLost {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.projected_fanout_honesty == Some(M5FanoutHonesty::SilentFailure) {
        findings.push(M5NotificationBlockingFinding::FanoutFailureSilent {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
    if binding.marketed_on_guarantee
        && binding.evidence_freshness == Some(M5EvidenceFreshness::Stale)
    {
        findings.push(M5NotificationBlockingFinding::StaleEvidenceOnMarketedRow {
            source_id: source_id.to_owned(),
            guarantee,
        });
    }
}

/// Computes the per-guarantee coverage and per-class finding summary.
fn summarize_report(
    sources: &[M5NotificationRouteRow],
) -> (
    Vec<M5NotificationCoverageSummary>,
    M5NotificationFindingSummary,
) {
    let mut coverage: Vec<M5NotificationCoverageSummary> =
        M5NotificationGuarantee::required_guarantees()
            .iter()
            .map(|guarantee| M5NotificationCoverageSummary {
                guarantee: *guarantee,
                qualified_rows: 0,
                explicitly_narrowed_rows: 0,
                not_applicable_rows: 0,
                platform_omitted_rows: 0,
                declared_capture_gap_rows: 0,
                unqualified_local_rule_rows: 0,
                missing_evidence_rows: 0,
            })
            .collect();
    let mut summary = M5NotificationFindingSummary::empty();

    for source in sources {
        for binding in &source.bindings {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|row| row.guarantee == binding.guarantee)
            {
                match binding.qualification_status {
                    M5NotificationStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5NotificationStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5NotificationStatus::NotApplicable => coverage_row.not_applicable_rows += 1,
                    M5NotificationStatus::PlatformOmitted => {
                        coverage_row.platform_omitted_rows += 1
                    }
                    M5NotificationStatus::DeclaredCaptureGap => {
                        coverage_row.declared_capture_gap_rows += 1
                    }
                    M5NotificationStatus::UnqualifiedLocalRule => {
                        coverage_row.unqualified_local_rule_rows += 1
                    }
                    M5NotificationStatus::MissingEvidence => {
                        coverage_row.missing_evidence_rows += 1
                    }
                }
            }
        }
        for finding in &source.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Computes the marketed rows release tooling should narrow because their
/// evidence is stale or red.
fn compute_narrowable_rows(sources: &[M5NotificationRouteRow]) -> Vec<M5NarrowableRow> {
    let mut narrowable = Vec::new();
    for source in sources {
        if !source.marketed {
            continue;
        }
        for finding in &source.blocking_findings {
            if let Some(guarantee) = finding.guarantee() {
                narrowable.push(M5NarrowableRow {
                    source_id: source.descriptor.source_id.clone(),
                    guarantee,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5NotificationRouteRow`] from a descriptor and its guarantee
/// bindings, computing the per-source blocking findings.
pub fn build_m5_notification_row(
    descriptor: M5NotificationSourceDescriptor,
    bindings: Vec<M5NotificationBinding>,
) -> M5NotificationRouteRow {
    let high_stakes = descriptor.is_high_stakes();
    let marketed = descriptor.marketed_on_desktop;
    let blocking_findings = compute_source_findings(&descriptor, &bindings, high_stakes);

    M5NotificationRouteRow {
        record_kind: M5_NOTIFICATION_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_NOTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_NOTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_stakes,
        marketed,
    }
}

/// Builds a full [`M5NotificationRouteReport`] from per-source rows.
pub fn build_m5_notification_routes_audit(
    sources: Vec<M5NotificationRouteRow>,
) -> M5NotificationRouteReport {
    let mut sources = sources;
    sources.sort_by(|left, right| left.descriptor.source_id.cmp(&right.descriptor.source_id));

    let registered_source_count = sources.len();
    let high_stakes_source_count = sources.iter().filter(|row| row.high_stakes).count();
    let marketed_source_count = sources.iter().filter(|row| row.marketed).count();
    let notification_guarantees_checked =
        sources.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (guarantee_coverage, findings_summary) = summarize_report(&sources);
    let narrowable_marketed_rows = compute_narrowable_rows(&sources);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut reopen_anchor_index: Vec<M5ReopenAnchorEntry> = sources
        .iter()
        .map(|source| M5ReopenAnchorEntry {
            notification_source: source.descriptor.notification_source,
            source_id: source.descriptor.source_id.clone(),
            reopen_anchor_ref: source.descriptor.reopen_anchor_ref.clone(),
        })
        .collect();
    reopen_anchor_index.sort_by(|left, right| left.source_id.cmp(&right.source_id));

    M5NotificationRouteReport {
        record_kind: M5_NOTIFICATION_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_NOTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_NOTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_NOTIFICATION_REPORT_ID.to_owned(),
        source_schema_ref: M5_NOTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        required_guarantees: M5NotificationGuarantee::required_guarantees().to_vec(),
        rows: sources,
        guarantee_coverage,
        findings_summary,
        reopen_anchor_index,
        registered_source_count,
        high_stakes_source_count,
        marketed_source_count,
        notification_guarantees_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_NOTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_NOTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_NOTIFICATION_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/durable-progress-and-reopen.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-notification-routes".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_notification_routes`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5NotificationValidationError {
    /// The audit has no registered sources.
    NoRegisteredSources,
    /// A required notification guarantee has no qualified source.
    RequiredGuaranteeNotQualified { guarantee: String },
    /// A source is missing a required guarantee from its binding set.
    MissingRequiredGuarantee {
        source_id: String,
        guarantee: String,
    },
    /// A blocking finding remains on the source.
    BlockingFindingPresent {
        source_id: String,
        guarantee: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A source's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { source_id: String },
}

/// Validates an audit report against the M5 notification-privacy acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_notification_routes(
    report: &M5NotificationRouteReport,
) -> Result<(), Vec<M5NotificationValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5NotificationValidationError::NoRegisteredSources);
    }

    for guarantee in M5NotificationGuarantee::required_guarantees() {
        let any_qualified = report.rows.iter().any(|source| {
            source.bindings.iter().any(|binding| {
                binding.guarantee == guarantee
                    && binding.qualification_status == M5NotificationStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(
                M5NotificationValidationError::RequiredGuaranteeNotQualified {
                    guarantee: guarantee.as_str().to_owned(),
                },
            );
        }
    }

    for source in &report.rows {
        for guarantee in M5NotificationGuarantee::required_guarantees() {
            if !source
                .bindings
                .iter()
                .any(|binding| binding.guarantee == guarantee)
            {
                errors.push(M5NotificationValidationError::MissingRequiredGuarantee {
                    source_id: source.descriptor.source_id.clone(),
                    guarantee: guarantee.as_str().to_owned(),
                });
            }
        }
        if source.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                M5NotificationValidationError::MissingDescriptorRevisionRef {
                    source_id: source.descriptor.source_id.clone(),
                },
            );
        }
        for finding in &source.blocking_findings {
            errors.push(M5NotificationValidationError::BlockingFindingPresent {
                source_id: finding.source_id().to_owned(),
                guarantee: finding
                    .guarantee()
                    .map(|guarantee| guarantee.as_str().to_owned())
                    .unwrap_or_else(|| "source".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5NotificationValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5NotificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_notification_routes_audit`].
struct SourceSeed {
    source_id: &'static str,
    notification_source: M5NotificationSource,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    reopen_anchor_ref: &'static str,
    support_note: &'static str,
    privacy_class: M5NotificationPrivacyClass,
    lifecycle_label: M5SourceLifecycle,
    suppression_controls: &'static [M5SuppressionControl],
    fanout_channels: &'static [M5NotificationChannel],
    reopen_outcome: M5ReopenOutcome,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    guarantee: M5NotificationGuarantee,
    qualification_status: M5NotificationStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified guarantee with captured evidence.
const fn qualified(guarantee: M5NotificationGuarantee) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5NotificationStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: an honestly-declared capture gap with a documented reason.
const fn declared_capture_gap(
    guarantee: M5NotificationGuarantee,
    reason: &'static str,
) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5NotificationStatus::DeclaredCaptureGap,
        narrowing_reason: Some(reason),
        note: None,
    }
}

/// Helper: a not-applicable guarantee with a documented reason.
const fn not_applicable(guarantee: M5NotificationGuarantee, reason: &'static str) -> BindingSeed {
    BindingSeed {
        guarantee,
        qualification_status: M5NotificationStatus::NotApplicable,
        narrowing_reason: Some(reason),
        note: None,
    }
}

use M5NotificationChannel::{
    ActivityCenterRow, CompanionSummary as CompanionChannel, DesktopToast, NativeOsNotification,
};
use M5NotificationGuarantee::{
    AdminSuppression, BadgeSemantics, CompanionFanoutHonesty, ExactTargetReopen, LockScreenPrivacy,
    PayloadMinimization, PrivacyClassification, QuietHoursPolicy, RootCauseDedupe,
};
use M5SuppressionControl::{
    AdminSuppress, BoundedSummaryFallback, LockScreenSummary, Mute, QuietHours, Snooze,
};

const FULL_SUPPRESSION: &[M5SuppressionControl] = &[
    QuietHours,
    AdminSuppress,
    Mute,
    Snooze,
    LockScreenSummary,
    BoundedSummaryFallback,
];

const BASIC_SUPPRESSION: &[M5SuppressionControl] =
    &[QuietHours, Mute, LockScreenSummary, BoundedSummaryFallback];

const ALL_CHANNELS: &[M5NotificationChannel] = &[
    DesktopToast,
    NativeOsNotification,
    ActivityCenterRow,
    CompanionChannel,
];

const LOCAL_CHANNELS: &[M5NotificationChannel] = &[DesktopToast, ActivityCenterRow];

const OFFBOARDING_CHANNELS: &[M5NotificationChannel] =
    &[DesktopToast, NativeOsNotification, ActivityCenterRow];

const COMPANION_CHANNELS: &[M5NotificationChannel] = &[ActivityCenterRow, CompanionChannel];

const FULL_BINDINGS: &[BindingSeed] = &[
    qualified(PrivacyClassification),
    qualified(LockScreenPrivacy),
    qualified(PayloadMinimization),
    qualified(QuietHoursPolicy),
    qualified(AdminSuppression),
    qualified(RootCauseDedupe),
    qualified(BadgeSemantics),
    qualified(ExactTargetReopen),
    qualified(CompanionFanoutHonesty),
];

const PROFILER_BINDINGS: &[BindingSeed] = &[
    qualified(PrivacyClassification),
    qualified(LockScreenPrivacy),
    qualified(PayloadMinimization),
    qualified(QuietHoursPolicy),
    qualified(AdminSuppression),
    qualified(RootCauseDedupe),
    qualified(BadgeSemantics),
    qualified(ExactTargetReopen),
    not_applicable(
        CompanionFanoutHonesty,
        "profiler_captures_are_local_only_so_there_is_no_companion_fanout_to_label",
    ),
];

const OFFBOARDING_BINDINGS: &[BindingSeed] = &[
    qualified(PrivacyClassification),
    qualified(LockScreenPrivacy),
    qualified(PayloadMinimization),
    qualified(QuietHoursPolicy),
    qualified(AdminSuppression),
    qualified(RootCauseDedupe),
    qualified(BadgeSemantics),
    qualified(ExactTargetReopen),
    declared_capture_gap(
        CompanionFanoutHonesty,
        "offboarding_runs_local_only_so_companion_fanout_is_declared_not_emitted",
    ),
];

const SOURCE_SEEDS: &[SourceSeed] = &[
    // Notebook run outcome. Workspace-sensitive; long-running and failable.
    SourceSeed {
        source_id: "notify:notebook_run",
        notification_source: M5NotificationSource::NotebookRun,
        descriptor_revision_ref: "notify-rev:notebook_run:2026.06.01-01",
        primary_label_ref: "label:notify.notebook_run:primary",
        reopen_anchor_ref: "notify:reopen:notebook_run",
        support_note: "Notebook-run notifications flow through the one governed envelope, keep lock-screen copy summary-first, honour quiet-hours and admin suppression, coalesce repeated failures, and reopen the exact cell and output.",
        privacy_class: M5NotificationPrivacyClass::WorkspaceSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: BASIC_SUPPRESSION,
        fanout_channels: ALL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Data/API run outcome. Security-critical; remote provider work.
    SourceSeed {
        source_id: "notify:data_api_run",
        notification_source: M5NotificationSource::DataApiRun,
        descriptor_revision_ref: "notify-rev:data_api_run:2026.06.01-01",
        primary_label_ref: "label:notify.data_api_run:primary",
        reopen_anchor_ref: "notify:reopen:data_api_run",
        support_note: "Data/API-run notifications are security-critical: the lock-screen copy never exposes credentials or response bodies, retries coalesce by root cause, and the notification reopens the exact request and result.",
        privacy_class: M5NotificationPrivacyClass::SecurityCritical,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: FULL_SUPPRESSION,
        fanout_channels: ALL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Pipeline rerun/cancel outcome. Workspace-sensitive; status object.
    SourceSeed {
        source_id: "notify:pipeline_action",
        notification_source: M5NotificationSource::PipelineAction,
        descriptor_revision_ref: "notify-rev:pipeline_action:2026.06.01-01",
        primary_label_ref: "label:notify.pipeline_action:primary",
        reopen_anchor_ref: "notify:reopen:pipeline_action",
        support_note: "Pipeline rerun/cancel notifications coalesce repeated stage failures by root cause and reopen the exact pipeline run they acted on.",
        privacy_class: M5NotificationPrivacyClass::WorkspaceSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: BASIC_SUPPRESSION,
        fanout_channels: ALL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Profiler capture completion. Workspace-sensitive; local-only.
    SourceSeed {
        source_id: "notify:profiler_capture",
        notification_source: M5NotificationSource::ProfilerCapture,
        descriptor_revision_ref: "notify-rev:profiler_capture:2026.06.01-01",
        primary_label_ref: "label:notify.profiler_capture:primary",
        reopen_anchor_ref: "notify:reopen:profiler_capture",
        support_note: "Profiler-capture notifications reopen the exact capture; the capture is local-only and declares no companion fanout to label.",
        privacy_class: M5NotificationPrivacyClass::WorkspaceSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: BASIC_SUPPRESSION,
        fanout_channels: LOCAL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: PROFILER_BINDINGS,
    },
    // Preview route state change. Summary-safe; live preview object.
    SourceSeed {
        source_id: "notify:preview_route",
        notification_source: M5NotificationSource::PreviewRoute,
        descriptor_revision_ref: "notify-rev:preview_route:2026.06.01-01",
        primary_label_ref: "label:notify.preview_route:primary",
        reopen_anchor_ref: "notify:reopen:preview_route",
        support_note: "Preview-route notifications are summary-safe and reopen the exact route and scope they were serving.",
        privacy_class: M5NotificationPrivacyClass::SummarySafe,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: BASIC_SUPPRESSION,
        fanout_channels: ALL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Companion summary fanout. Summary-safe; companion-bound.
    SourceSeed {
        source_id: "notify:companion_summary",
        notification_source: M5NotificationSource::CompanionSummary,
        descriptor_revision_ref: "notify-rev:companion_summary:2026.06.01-01",
        primary_label_ref: "label:notify.companion_summary:primary",
        reopen_anchor_ref: "notify:reopen:companion_summary",
        support_note: "Companion-summary fanout is summary-safe, labels stale or failed delivery honestly, and reopens the same authoritative object on the desktop.",
        privacy_class: M5NotificationPrivacyClass::SummarySafe,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: BASIC_SUPPRESSION,
        fanout_channels: COMPANION_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Incident-packet generation outcome. Security-critical; reviewable packet.
    SourceSeed {
        source_id: "notify:incident_packet",
        notification_source: M5NotificationSource::IncidentPacket,
        descriptor_revision_ref: "notify-rev:incident_packet:2026.06.01-01",
        primary_label_ref: "label:notify.incident_packet:primary",
        reopen_anchor_ref: "notify:reopen:incident_packet",
        support_note: "Incident-packet notifications are security-critical: the lock-screen copy is a bounded summary, the packet carries stable enums only, and the notification reopens the exact packet and incident.",
        privacy_class: M5NotificationPrivacyClass::SecurityCritical,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: FULL_SUPPRESSION,
        fanout_channels: ALL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Sync state change. Managed-sensitive; conflict-capable.
    SourceSeed {
        source_id: "notify:sync_state_change",
        notification_source: M5NotificationSource::SyncStateChange,
        descriptor_revision_ref: "notify-rev:sync_state_change:2026.06.01-01",
        primary_label_ref: "label:notify.sync_state_change:primary",
        reopen_anchor_ref: "notify:reopen:sync_state_change",
        support_note: "Sync-state notifications are managed-sensitive: admin suppression is honoured, repeated conflict alerts coalesce by root cause, and the notification reopens the exact workspace and conflict.",
        privacy_class: M5NotificationPrivacyClass::ManagedSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: FULL_SUPPRESSION,
        fanout_channels: ALL_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: FULL_BINDINGS,
    },
    // Offboarding job outcome. Managed-sensitive; destructive, local-only.
    SourceSeed {
        source_id: "notify:offboarding_job",
        notification_source: M5NotificationSource::OffboardingJob,
        descriptor_revision_ref: "notify-rev:offboarding_job:2026.06.01-01",
        primary_label_ref: "label:notify.offboarding_job:primary",
        reopen_anchor_ref: "notify:reopen:offboarding_job",
        support_note: "Offboarding-job notifications are managed-sensitive and run local-only: they reopen the exact export-and-wipe job and declare their companion fanout gap honestly.",
        privacy_class: M5NotificationPrivacyClass::ManagedSensitive,
        lifecycle_label: M5SourceLifecycle::Beta,
        suppression_controls: FULL_SUPPRESSION,
        fanout_channels: OFFBOARDING_CHANNELS,
        reopen_outcome: M5ReopenOutcome::ExactTargetResolved,
        bindings: OFFBOARDING_BINDINGS,
    },
];

fn build_binding_from_seed(seed: &SourceSeed, binding_seed: &BindingSeed) -> M5NotificationBinding {
    let guarantee = binding_seed.guarantee;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_stakes = seed.privacy_class.is_high_stakes();
    let marketed_on_guarantee = !matches!(
        binding_seed.qualification_status,
        M5NotificationStatus::NotApplicable | M5NotificationStatus::PlatformOmitted
    );

    M5NotificationBinding {
        guarantee,
        aspect: guarantee.canonical_aspect(),
        qualification_status: binding_seed.qualification_status,
        marketed_on_guarantee,
        projected_envelope_ref: qualified
            .then(|| format!("notify-envelope:{}:{}", seed.source_id, guarantee.as_str())),
        projected_privacy_class: qualified.then_some(seed.privacy_class),
        projected_lock_screen: qualified.then_some(M5LockScreenDisclosure::SummaryOnly),
        projected_payload_disclosure: (qualified && guarantee.requires_payload_disclosure())
            .then_some(M5PayloadDisclosure::EnumsOnly),
        projected_quiet_hours: (qualified && guarantee.requires_quiet_hours())
            .then_some(M5QuietHoursOutcome::Respected),
        projected_admin_suppression: (qualified && guarantee.requires_admin_suppression())
            .then_some(M5AdminSuppressionOutcome::Honored),
        projected_dedupe: (qualified && guarantee.requires_dedupe())
            .then_some(M5DedupeOutcome::CoalescedByRootCause),
        projected_badge: (qualified && guarantee.requires_badge())
            .then_some(M5BadgeOutcome::DurableCountClass),
        projected_reopen_outcome: (qualified
            && (guarantee.requires_reopen_outcome() || high_stakes))
            .then_some(seed.reopen_outcome),
        projected_fanout_honesty: (qualified && guarantee.requires_fanout_honesty())
            .then_some(M5FanoutHonesty::HonestlyLabeled),
        evidence_freshness: qualified.then_some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_source_from_seed(seed: &SourceSeed) -> M5NotificationRouteRow {
    let descriptor = M5NotificationSourceDescriptor {
        source_id: seed.source_id.to_owned(),
        notification_source: seed.notification_source,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        reopen_anchor_ref: seed.reopen_anchor_ref.to_owned(),
        support_note: seed.support_note.to_owned(),
        privacy_class: seed.privacy_class,
        lifecycle_label: seed.lifecycle_label,
        suppression_controls: seed.suppression_controls.to_vec(),
        fanout_channels: seed.fanout_channels.to_vec(),
        marketed_on_desktop: true,
        routed_through_governed_router: true,
    };
    let bindings: Vec<M5NotificationBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_notification_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5/notification-dedupe/`.
pub fn seeded_m5_notification_routes_audit() -> M5NotificationRouteReport {
    let sources = SOURCE_SEEDS.iter().map(build_source_from_seed).collect();
    build_m5_notification_routes_audit(sources)
}
