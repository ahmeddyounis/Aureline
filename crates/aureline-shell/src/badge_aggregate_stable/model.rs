//! Canonical stable truth model for the **badge aggregate**: count-class
//! semantics, cross-client / cross-window dedupe, admin / quiet-hours
//! suppression lineage, and the persistent attention summary on a
//! claimed-stable desktop shell.
//!
//! ## Why one governed aggregate record
//!
//! The per-class durable-attention work
//! ([`crate::notification_attention_stable`]) locks one record per durable
//! attention class. This module sits one level up: it reconciles the **whole
//! shell's badge state** from the same durable object set the activity center
//! reads, so the dock/taskbar badge, the title-bar badge, the in-shell badge,
//! and the companion badge can never drift from the durable truth.
//!
//! The risk this closes: each badge surface counts on its own. A dock badge
//! outpaces the durable model; the same underlying object delivered to desktop,
//! companion, and a native notification multiplies a badge threefold; an admin
//! policy or quiet-hours window silences an item and the badge silently drops to
//! zero with no way for support to explain *why* no alert fired. The result is a
//! badge whose `0` might mean "nothing is wrong" or might mean "a fanout path
//! failed" — and nobody can tell which.
//!
//! A [`BadgeAggregateRecord`] mints, for one shell snapshot:
//!
//! - **Typed count classes** — every count is keyed by an
//!   [`AggregateCountClass`] (pending review/approval, failed runs, queued
//!   publish-later work, provider-auth attention, managed advisories, muted
//!   informational backlog, …), never by an arbitrary surface.
//! - **One durable object set** — the dock/taskbar, title-bar, in-shell, and
//!   companion projections are computed from the same deduped durable objects
//!   the activity center reads; a surface may never inflate a class above the
//!   authoritative active count.
//! - **Cross-client / cross-window dedupe** — repeated copies of the same
//!   underlying object (desktop + companion + native notification, or two
//!   windows) collapse to one durable object and count **once** per class.
//! - **Export-safe suppression lineage** — every admin-suppressed,
//!   quiet-hours-muted, or per-class-disabled badge difference carries a
//!   lineage entry that names the reason, the affected surfaces, and proves the
//!   durable object and its reopen target were preserved, so support and
//!   shiproom can explain a missing alert.
//! - **`0` means none** — an active badge count of zero for a class means there
//!   are no current durable objects of that class, not that a toast was hidden
//!   or a fanout path failed; counts are *derived* from durable object state.
//! - **A persistent, inspectable summary** — a durable attention summary that
//!   survives restart and is inspectable in-product, not only in release notes.
//! - **A public claim ceiling** and **automatic narrowing** — a snapshot that
//!   cannot prove a pillar, or whose lowest badge surface marker is below
//!   Stable, narrows below Stable with a named reason instead of inheriting an
//!   adjacent green row.
//!
//! The dedupe core, the per-item badge reconciliation
//! ([`crate::notifications::actions`]), and the count classes are **not**
//! reinvented here: every record is a genuine projection of the live attention
//! stack routed through [`crate::notification_envelope_corpus`].

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::notification_attention_stable::model::{
    is_canonical_object_ref, AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord,
    LayoutMode, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord, StableClaimClass,
};
use crate::notifications::actions::BadgeClass;
use crate::notifications::envelope::{
    ClientScope, DedupeKeyScheme, QuietHoursMode, SourceSubsystem,
};

/// Stable record-kind tag carried in serialized aggregate records.
pub const BADGE_AGGREGATE_RECORD_KIND: &str = "badge_aggregate_record";

/// Schema version for the [`BadgeAggregateRecord`] payload shape.
pub const BADGE_AGGREGATE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const BADGE_AGGREGATE_SHARED_CONTRACT_REF: &str = "shell:badge_aggregate_stable:v1";

/// Reviewer-facing notice rendered on every aggregate surface.
pub const BADGE_AGGREGATE_NOTICE: &str =
    "Badge-aggregate truth: every badge count is typed by count class, not by surface; the \
     dock/taskbar, title-bar, in-shell, and companion badges are computed from the same durable \
     object set the activity center reads, so no surface inflates a class; the same underlying \
     object delivered across desktop, companion, and native-notification surfaces dedupes to one \
     durable object and counts once per class; every admin-suppressed, quiet-hours-muted, or \
     per-class-disabled badge difference carries an export-safe lineage entry that preserves the \
     durable object and its reopen target so support can explain a missing alert; a badge count of \
     zero means there are no current durable objects of that class, not that a toast was hidden or \
     a fanout path failed; the persistent attention summary is durable and inspectable in-product; \
     and a snapshot that cannot prove a pillar, or whose lowest badge surface marker is below \
     Stable, narrows below Stable with a named reason rather than inheriting an adjacent green row.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present ref.
const MAX_REF_CHARS: usize = 200;

// ---------------------------------------------------------------------------
// Count-class taxonomy
// ---------------------------------------------------------------------------

/// Canonical, typed count class a badge aggregate is keyed by.
///
/// The first six variants are the count classes this lane is required to keep
/// explicit where they are exposed; the remaining variants cover the rest of the
/// durable attention space so every durable object lands in exactly one class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregateCountClass {
    /// Items awaiting human review or approval.
    PendingReviewApproval,
    /// Failed runs or retries that remain inspectable.
    FailedRuns,
    /// Offline / publish-later work queued for drain.
    QueuedPublishLater,
    /// Provider sign-in / re-authentication attention.
    ProviderAuthAttention,
    /// Managed (admin/policy) advisories.
    ManagedAdvisories,
    /// Muted informational backlog held out of the active badge.
    MutedInformationalBacklog,
    /// Session join / control / handoff requests.
    SessionRequests,
    /// Durable jobs that are queued or running.
    DurableRunning,
    /// Completed work that is unread.
    CompletionUnread,
}

impl AggregateCountClass {
    /// Stable token recorded in fixtures, exports, and badge projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReviewApproval => "pending_review_approval",
            Self::FailedRuns => "failed_runs",
            Self::QueuedPublishLater => "queued_publish_later",
            Self::ProviderAuthAttention => "provider_auth_attention",
            Self::ManagedAdvisories => "managed_advisories",
            Self::MutedInformationalBacklog => "muted_informational_backlog",
            Self::SessionRequests => "session_requests",
            Self::DurableRunning => "durable_running",
            Self::CompletionUnread => "completion_unread",
        }
    }

    /// The count classes this lane must keep explicit where they are exposed.
    pub const REQUIRED: [Self; 6] = [
        Self::PendingReviewApproval,
        Self::FailedRuns,
        Self::QueuedPublishLater,
        Self::ProviderAuthAttention,
        Self::ManagedAdvisories,
        Self::MutedInformationalBacklog,
    ];

    /// Every count class, in canonical order.
    pub const ALL: [Self; 9] = [
        Self::PendingReviewApproval,
        Self::FailedRuns,
        Self::QueuedPublishLater,
        Self::ProviderAuthAttention,
        Self::ManagedAdvisories,
        Self::MutedInformationalBacklog,
        Self::SessionRequests,
        Self::DurableRunning,
        Self::CompletionUnread,
    ];

    /// Maps an upstream per-item [`BadgeClass`] to its aggregate count class.
    ///
    /// `SecurityNotices` defaults to [`Self::ManagedAdvisories`]; provider-borne
    /// security attention is reclassified to [`Self::ProviderAuthAttention`] by
    /// the projector that knows the owning subsystem.
    pub const fn from_badge_class(badge_class: BadgeClass) -> Self {
        match badge_class {
            BadgeClass::NeedsReview => Self::PendingReviewApproval,
            BadgeClass::FailedRuns => Self::FailedRuns,
            BadgeClass::OfflinePublishPending => Self::QueuedPublishLater,
            BadgeClass::SecurityNotices => Self::ManagedAdvisories,
            BadgeClass::HeldOrSuppressedCount => Self::MutedInformationalBacklog,
            BadgeClass::SessionRequests | BadgeClass::Mentions => Self::SessionRequests,
            BadgeClass::DurableRunningCount => Self::DurableRunning,
            BadgeClass::CompletionUnread => Self::CompletionUnread,
        }
    }

    fn singular_label(self) -> &'static str {
        match self {
            Self::PendingReviewApproval => "review item",
            Self::FailedRuns => "failed run",
            Self::QueuedPublishLater => "queued publish item",
            Self::ProviderAuthAttention => "provider sign-in",
            Self::ManagedAdvisories => "managed advisory",
            Self::MutedInformationalBacklog => "muted item",
            Self::SessionRequests => "session request",
            Self::DurableRunning => "running item",
            Self::CompletionUnread => "unread completion",
        }
    }

    fn plural_label(self) -> &'static str {
        match self {
            Self::PendingReviewApproval => "review items",
            Self::FailedRuns => "failed runs",
            Self::QueuedPublishLater => "queued publish items",
            Self::ProviderAuthAttention => "provider sign-ins",
            Self::ManagedAdvisories => "managed advisories",
            Self::MutedInformationalBacklog => "muted items",
            Self::SessionRequests => "session requests",
            Self::DurableRunning => "running items",
            Self::CompletionUnread => "unread completions",
        }
    }
}

/// Privacy-safe compact label for one count class' active / held totals. Never
/// exposes secrets, identities, or content — only typed counts.
pub fn class_summary_label(
    count_class: AggregateCountClass,
    active_count: u32,
    held_or_suppressed_count: u32,
) -> String {
    let singular = count_class.singular_label();
    let plural = count_class.plural_label();
    match (active_count, held_or_suppressed_count) {
        (0, 0) => format!("No {plural}"),
        (1, 0) => format!("1 {singular}"),
        (n, 0) => format!("{n} {plural}"),
        (0, h) => format!("No active {plural}; {h} held"),
        (1, h) => format!("1 {singular}; {h} held"),
        (n, h) => format!("{n} {plural}; {h} held"),
    }
}

// ---------------------------------------------------------------------------
// Badge surfaces
// ---------------------------------------------------------------------------

/// A surface that renders a badge derived from the durable object set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeSurface {
    /// The activity center — the authoritative durable surface.
    ActivityCenter,
    /// The OS dock / taskbar app-icon badge.
    DockTaskbar,
    /// The in-window title-bar badge.
    TitleBar,
    /// The in-shell status / activity badge.
    InShell,
    /// The companion (mobile / web) push badge.
    Companion,
}

impl BadgeSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::DockTaskbar => "dock_taskbar",
            Self::TitleBar => "title_bar",
            Self::InShell => "in_shell",
            Self::Companion => "companion",
        }
    }

    /// Every badge surface a snapshot must project, in canonical order. The
    /// activity center is first because it is the authoritative durable surface.
    pub const REQUIRED: [Self; 5] = [
        Self::ActivityCenter,
        Self::DockTaskbar,
        Self::TitleBar,
        Self::InShell,
        Self::Companion,
    ];
}

// ---------------------------------------------------------------------------
// Durable object set
// ---------------------------------------------------------------------------

/// Lifecycle disposition of a durable object for badge-counting purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableItemDisposition {
    /// Contributes to the active badge for its class.
    Active,
    /// Held by quiet hours, snooze, mute, or admin policy. The durable object is
    /// preserved and contributes to the held/suppressed count, not the active
    /// badge.
    HeldOrSuppressed,
    /// Cleared or resolved through its owning model. No longer a current durable
    /// object; contributes to no badge.
    ClearedResolved,
}

impl DurableItemDisposition {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::HeldOrSuppressed => "held_or_suppressed",
            Self::ClearedResolved => "cleared_resolved",
        }
    }
}

/// One raw appearance of a durable object reported by a client surface. Several
/// appearances of the same `object_ref` (e.g. desktop + companion + a native
/// notification) collapse, by canonical object identity, to a single durable
/// object so a badge counts the object once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawObjectAppearance {
    /// Canonical durable-object ref — the dedupe key.
    pub object_ref: String,
    /// Client scope this appearance was reported from.
    pub client_scope: ClientScope,
    /// Count class the object contributes to.
    pub count_class: AggregateCountClass,
    /// Lifecycle disposition.
    pub disposition: DurableItemDisposition,
    /// Reviewable label.
    pub label: String,
    /// Owning subsystem.
    pub owner_subsystem: SourceSubsystem,
    /// Whether the durable object is preserved (must be true to count honestly).
    pub durable_object_preserved: bool,
    /// Reopen target ref this object resolves to.
    pub reopen_target_ref: String,
    /// Canonical event id from the projected route outcome.
    pub canonical_event_id: String,
    /// Dedupe key scheme from the projected route outcome.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Dedupe key ref from the projected route outcome.
    pub dedupe_key_ref: String,
    /// Route-outcome id this appearance projects from.
    pub route_outcome_id_ref: String,
    /// Notification-envelope id this appearance projects from.
    pub envelope_id_ref: String,
}

/// One durable object after cross-client / cross-window dedupe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DedupedDurableObject {
    /// Canonical durable-object ref.
    pub object_ref: String,
    /// Count class.
    pub count_class: AggregateCountClass,
    /// Lifecycle disposition.
    pub disposition: DurableItemDisposition,
    /// Reviewable label.
    pub label: String,
    /// Owning subsystem.
    pub owner_subsystem: SourceSubsystem,
    /// Distinct client scopes this object was reported from, sorted.
    pub appearances: Vec<ClientScope>,
    /// Number of raw appearances that collapsed into this object.
    pub raw_appearance_count: u32,
    /// Whether the durable object is preserved across every appearance.
    pub durable_object_preserved: bool,
    /// Reopen target ref this object resolves to.
    pub reopen_target_ref: String,
    /// Canonical event id from the projected route outcome.
    pub canonical_event_id: String,
    /// Dedupe key scheme from the projected route outcome.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Dedupe key ref from the projected route outcome.
    pub dedupe_key_ref: String,
    /// Route-outcome id this object projects from.
    pub route_outcome_id_ref: String,
    /// Notification-envelope id this object projects from.
    pub envelope_id_ref: String,
}

/// Reconciled active / held totals for one count class, derived from the durable
/// object set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassAggregate {
    /// Count class.
    pub count_class: AggregateCountClass,
    /// Active durable objects of this class.
    pub active_count: u32,
    /// Held / suppressed durable objects of this class.
    pub held_or_suppressed_count: u32,
    /// Cleared / resolved objects of this class (badged nowhere; kept for audit).
    pub cleared_count: u32,
    /// Privacy-safe compact label for badge / OS summary surfaces.
    pub privacy_safe_summary_label: String,
}

// ---------------------------------------------------------------------------
// Surface projections
// ---------------------------------------------------------------------------

/// One class' reported count on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceClassCount {
    /// Count class.
    pub count_class: AggregateCountClass,
    /// Count this surface actually reports for the class.
    pub reported_count: u32,
}

/// What one surface actually shows (its reported per-class counts and any
/// per-class badge disablement), before reconciliation against the durable set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceProjectionInput {
    /// Surface this projection describes.
    pub surface: BadgeSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Reported per-class counts.
    pub class_counts: Vec<SurfaceClassCount>,
    /// Classes whose badge is disabled on this surface (by user or admin).
    pub disabled_classes: Vec<AggregateCountClass>,
}

/// One surface's reconciled badge projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceProjection {
    /// Surface this projection describes.
    pub surface: BadgeSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Reconciled per-class counts (echoes what the surface reports), sorted.
    pub class_counts: Vec<SurfaceClassCount>,
    /// Classes whose badge is disabled on this surface, sorted.
    pub disabled_classes: Vec<AggregateCountClass>,
    /// Total reported across classes.
    pub total_reported: u32,
    /// Whether every reported count equals the authoritative durable count
    /// (accounting for disablement).
    pub matches_durable_set: bool,
    /// Whether the surface inflates any class above the authoritative count.
    pub inflates_any_class: bool,
}

// ---------------------------------------------------------------------------
// Suppression lineage
// ---------------------------------------------------------------------------

/// Why a badge produced no (or a reduced) user-facing alert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeSuppressionReason {
    /// Held by an administrator policy.
    AdminPolicySuppression,
    /// Held by a quiet-hours / do-not-disturb / focus window.
    QuietHoursMuting,
    /// The class' badge is disabled on a surface by user or admin setting.
    PerClassBadgeDisabled,
    /// Muted by the user for this class or source.
    UserMuted,
}

impl BadgeSuppressionReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminPolicySuppression => "admin_policy_suppression",
            Self::QuietHoursMuting => "quiet_hours_muting",
            Self::PerClassBadgeDisabled => "per_class_badge_disabled",
            Self::UserMuted => "user_muted",
        }
    }
}

/// What a lineage entry is scoped to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionScope {
    /// One durable object held out of the active badge.
    Object,
    /// One count class held out of the active badge across surfaces.
    ClassWide,
    /// One count class' badge disabled on one surface.
    SurfaceClass,
}

impl SuppressionScope {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Object => "object",
            Self::ClassWide => "class_wide",
            Self::SurfaceClass => "surface_class",
        }
    }
}

/// One export-safe explanation for why a badge alert was suppressed, muted, or
/// disabled while the durable object stayed reachable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionLineageEntry {
    /// Why the alert was suppressed.
    pub reason: BadgeSuppressionReason,
    /// What this entry is scoped to.
    pub scope: SuppressionScope,
    /// Object this entry covers, when object-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_ref: Option<String>,
    /// Count class this entry covers, when class- or surface-class-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_class: Option<AggregateCountClass>,
    /// Surface this entry covers, when surface-class-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<BadgeSurface>,
    /// Surfaces whose alert was changed by this suppression.
    pub affected_surfaces: Vec<BadgeSurface>,
    /// Whether the durable object stayed reachable (must be true).
    pub durable_object_preserved: bool,
    /// Whether the exact reopen target stayed reachable (must be true).
    pub reopen_target_preserved: bool,
    /// Privacy-safe, support-readable explanation.
    pub export_safe_summary: String,
}

// ---------------------------------------------------------------------------
// Cross-client dedupe + summary
// ---------------------------------------------------------------------------

/// Cross-client / cross-window dedupe disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossClientDedupeDisclosure {
    /// Total raw appearances reported across surfaces and windows.
    pub raw_appearance_count: u32,
    /// Distinct durable objects after dedupe.
    pub deduped_object_count: u32,
    /// Raw appearances that collapsed (raw − deduped).
    pub cross_client_collapsed: u32,
    /// Dedupe key scheme used for cross-client collapse.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Whether every appearance of an object agreed on its count class.
    pub class_integrity_preserved: bool,
    /// Distinct client scopes the snapshot covers, sorted.
    pub scopes_covered: Vec<ClientScope>,
}

/// The persistent, in-product attention summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentAttentionSummary {
    /// Stable summary id.
    pub summary_id: String,
    /// Total active durable objects across classes.
    pub total_active: u32,
    /// Total held / suppressed durable objects across classes.
    pub total_held_or_suppressed: u32,
    /// Per-class active / held totals, in canonical class order.
    pub per_class: Vec<ClassAggregate>,
    /// Privacy-safe one-line summary label.
    pub privacy_safe_summary_label: String,
    /// Whether the summary is durable and survives restart.
    pub durable_and_persistent: bool,
    /// Whether the summary is inspectable in-product.
    pub inspectable_in_product: bool,
}

// ---------------------------------------------------------------------------
// Claim ceiling + qualification
// ---------------------------------------------------------------------------

/// The public claim ceiling: what a snapshot is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BadgeAggregateClaimCeiling {
    /// May claim every surface derives from one durable object set with no
    /// inflation.
    pub asserts_one_durable_set: bool,
    /// May claim cross-client dedupe preserves count-class integrity.
    pub asserts_cross_client_dedupe: bool,
    /// May claim every suppressed / disabled badge has export-safe lineage.
    pub asserts_suppression_lineage_export_safe: bool,
    /// May claim a zero badge means no current durable objects of that class.
    pub asserts_zero_means_no_durable_items: bool,
    /// May claim the attention summary is persistent and inspectable.
    pub asserts_summary_persistent_inspectable: bool,
}

/// Reason a snapshot is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeAggregateNarrowingReason {
    /// A surface does not derive from the one durable object set (it inflates a
    /// class or shows an unexplained shortfall).
    OneDurableSetNotProven,
    /// Cross-client dedupe did not preserve count-class integrity.
    CrossClientDedupeNotClassTruthful,
    /// A suppressed / muted / disabled badge lacks export-safe lineage.
    SuppressionLineageNotExportSafe,
    /// A zero badge could mean a hidden toast or a failed fanout, not "no
    /// durable objects of that class".
    ZeroDoesNotMeanNoDurableItems,
    /// The attention summary is not persistent or not inspectable in-product.
    SummaryNotPersistentOrInspectable,
    /// The lowest badge surface marker is below Stable, so the snapshot must not
    /// inherit Stable by adjacency.
    SurfaceNotYetStable,
}

impl BadgeAggregateNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneDurableSetNotProven => "one_durable_set_not_proven",
            Self::CrossClientDedupeNotClassTruthful => "cross_client_dedupe_not_class_truthful",
            Self::SuppressionLineageNotExportSafe => "suppression_lineage_not_export_safe",
            Self::ZeroDoesNotMeanNoDurableItems => "zero_does_not_mean_no_durable_items",
            Self::SummaryNotPersistentOrInspectable => "summary_not_persistent_or_inspectable",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

/// The derived stable-claim verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregateQualification {
    /// The derived claim class.
    pub claim_class: StableClaimClass,
    /// Whether the snapshot qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// Reasons the snapshot is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<BadgeAggregateNarrowingReason>,
}

/// The derived pillar verdicts (what the snapshot can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregatePillars {
    /// Every surface derives from one durable set with no inflation.
    pub one_durable_set_holds: bool,
    /// Cross-client dedupe preserved count-class integrity.
    pub cross_client_dedupe_holds: bool,
    /// Every suppressed / disabled badge carries export-safe lineage.
    pub suppression_lineage_export_safe: bool,
    /// A zero badge means no current durable objects of that class.
    pub zero_means_no_durable_items: bool,
    /// The attention summary is persistent and inspectable.
    pub summary_persistent_inspectable: bool,
}

/// Upstream ids the record is a genuine projection of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregateUpstream {
    /// Notification-envelope corpus packet id this record projects from.
    pub corpus_packet_ref: String,
    /// Corpus case ids the appearances came from, sorted and deduped.
    pub contributing_case_refs: Vec<String>,
    /// Route-outcome ids the appearances came from, sorted and deduped.
    pub contributing_route_outcome_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`BadgeAggregateRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeAggregateInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token (the snapshot scenario).
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Stable summary id for the persistent attention summary.
    pub summary_id: String,
    /// Raw object appearances (deduped by the builder).
    pub raw_appearances: Vec<RawObjectAppearance>,
    /// Per-surface reported projections.
    pub surface_projections: Vec<SurfaceProjectionInput>,
    /// Export-safe suppression lineage.
    pub suppression_lineage: Vec<SuppressionLineageEntry>,
    /// Quiet-hours / admin modes active for this snapshot.
    pub active_quiet_hours_modes: Vec<QuietHoursMode>,
    /// Public claim ceiling.
    pub claim_ceiling: BadgeAggregateClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the aggregate.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the snapshot stays available without an account.
    pub available_without_account: bool,
    /// Whether the snapshot stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: BadgeAggregateUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed badge-aggregate record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeAggregateRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The lowest badge surface marker — the record's overall surface marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// The deduped durable object set, sorted by object ref.
    pub deduped_objects: Vec<DedupedDurableObject>,
    /// Per-class aggregates, in canonical class order.
    pub class_aggregates: Vec<ClassAggregate>,
    /// Per-surface reconciled projections, in canonical surface order.
    pub surface_projections: Vec<SurfaceProjection>,
    /// Cross-client dedupe disclosure.
    pub cross_client_dedupe: CrossClientDedupeDisclosure,
    /// Export-safe suppression lineage.
    pub suppression_lineage: Vec<SuppressionLineageEntry>,
    /// Quiet-hours / admin modes active for this snapshot.
    pub active_quiet_hours_modes: Vec<QuietHoursMode>,
    /// The persistent, inspectable attention summary.
    pub summary_digest: PersistentAttentionSummary,
    /// The derived pillar verdicts.
    pub pillars: BadgeAggregatePillars,
    /// The public claim ceiling.
    pub claim_ceiling: BadgeAggregateClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: BadgeAggregateQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the aggregate.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the snapshot stays available without an account.
    pub available_without_account: bool,
    /// Whether the snapshot stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: BadgeAggregateUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`BadgeAggregateRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// Two appearances of the same object disagreed on count class or
    /// disposition — cross-client dedupe could not preserve class integrity.
    DedupeClassConflict { object_ref: String },
    /// A required badge surface projection was missing.
    SurfaceProjectionMissing { surface: BadgeSurface },
    /// A badge surface projection was duplicated.
    DuplicateSurfaceProjection { surface: BadgeSurface },
    /// A held / suppressed object had no object-scoped lineage entry.
    MissingObjectSuppressionLineage { object_ref: String },
    /// A per-class badge disablement on a surface had no lineage entry.
    MissingDisablementLineage {
        surface: BadgeSurface,
        count_class: AggregateCountClass,
    },
    /// The claim ceiling asserted one durable set it cannot prove.
    OverclaimsOneDurableSet,
    /// The claim ceiling asserted cross-client dedupe it cannot prove.
    OverclaimsCrossClientDedupe,
    /// The claim ceiling asserted export-safe suppression lineage it cannot
    /// prove.
    OverclaimsSuppressionLineage,
    /// The claim ceiling asserted zero-means-none it cannot prove.
    OverclaimsZeroMeansNone,
    /// The claim ceiling asserted a persistent summary it cannot prove.
    OverclaimsSummaryPersistent,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: BadgeRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: AttentionRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: AttentionRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: AttentionRouteSurface },
    /// An entry route did not activate the same aggregate.
    RouteTargetsDifferentItem { surface: AttentionRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// A snapshot was hidden when no account was present.
    HiddenWithoutAccount,
    /// A snapshot was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::MissingRef { field } => write!(f, "ref `{field}` must be present"),
            Self::DedupeClassConflict { object_ref } => write!(
                f,
                "object {object_ref:?} appeared with conflicting count class or disposition; \
                 cross-client dedupe cannot preserve class integrity"
            ),
            Self::SurfaceProjectionMissing { surface } => {
                write!(
                    f,
                    "badge surface projection `{}` is missing",
                    surface.as_str()
                )
            }
            Self::DuplicateSurfaceProjection { surface } => {
                write!(
                    f,
                    "badge surface projection `{}` is duplicated",
                    surface.as_str()
                )
            }
            Self::MissingObjectSuppressionLineage { object_ref } => write!(
                f,
                "held object {object_ref:?} produced no active alert but has no export-safe \
                 lineage entry"
            ),
            Self::MissingDisablementLineage {
                surface,
                count_class,
            } => write!(
                f,
                "class `{}` is disabled on surface `{}` but has no export-safe lineage entry",
                count_class.as_str(),
                surface.as_str()
            ),
            Self::OverclaimsOneDurableSet => write!(
                f,
                "claim ceiling may not assert one durable set when a surface inflates or diverges"
            ),
            Self::OverclaimsCrossClientDedupe => write!(
                f,
                "claim ceiling may not assert cross-client dedupe that loses class integrity"
            ),
            Self::OverclaimsSuppressionLineage => write!(
                f,
                "claim ceiling may not assert export-safe lineage that erases the durable object"
            ),
            Self::OverclaimsZeroMeansNone => write!(
                f,
                "claim ceiling may not assert zero-means-none when counts are not derived from \
                 durable objects"
            ),
            Self::OverclaimsSummaryPersistent => write!(
                f,
                "claim ceiling may not assert a persistent summary it cannot prove"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(
                    f,
                    "snapshot must expose recovery route `{}`",
                    action.as_str()
                )
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route `{action_id}` must be keyboard reachable")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(
                    f,
                    "entry route surface `{}` is duplicated",
                    surface.as_str()
                )
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentItem { surface } => write!(
                f,
                "entry route surface `{}` must activate the same aggregate",
                surface.as_str()
            ),
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(
                    f,
                    "accessibility layout mode `{}` is missing",
                    mode.as_str()
                )
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::HiddenWithoutAccount => {
                write!(
                    f,
                    "a badge aggregate must stay available without an account"
                )
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a badge aggregate must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingRef { field })
    }
}

impl BadgeAggregateRecord {
    /// Builds a governed badge-aggregate record from validated input.
    ///
    /// The dedupe, the per-class aggregates, the surface reconciliation, and the
    /// pillar verdicts are all *derived* from the durable object set, so a record
    /// can never publish a claim wider than its proof. Structural lies (a dedupe
    /// class conflict, a held object with no lineage, a missing route surface)
    /// are rejected outright; provable-but-imperfect snapshots (a surface that
    /// inflates, a below-Stable companion marker) are minted but narrowed below
    /// Stable with a named reason.
    pub fn build(input: BadgeAggregateInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        for (field, value) in [
            ("title", &input.title),
            ("summary", &input.summary),
            ("posture_label", &input.posture_label),
        ] {
            if !is_reviewable_sentence(value) {
                return Err(BuildError::InvalidSentence { field });
            }
        }
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_present_ref(
            "upstream.corpus_packet_ref",
            &input.upstream.corpus_packet_ref,
        )?;

        // --- cross-client / cross-window dedupe ------------------------------
        let mut by_object: BTreeMap<String, DedupedDurableObject> = BTreeMap::new();
        let mut scopes_covered: BTreeSet<ClientScope> = BTreeSet::new();
        let raw_appearance_count = input.raw_appearances.len() as u32;
        for appearance in &input.raw_appearances {
            require_canonical_ref("raw_appearances.object_ref", &appearance.object_ref)?;
            require_present_ref(
                "raw_appearances.reopen_target_ref",
                &appearance.reopen_target_ref,
            )?;
            require_present_ref(
                "raw_appearances.canonical_event_id",
                &appearance.canonical_event_id,
            )?;
            require_present_ref("raw_appearances.dedupe_key_ref", &appearance.dedupe_key_ref)?;
            require_present_ref(
                "raw_appearances.route_outcome_id_ref",
                &appearance.route_outcome_id_ref,
            )?;
            require_present_ref(
                "raw_appearances.envelope_id_ref",
                &appearance.envelope_id_ref,
            )?;
            if !is_reviewable_sentence(&appearance.label) {
                return Err(BuildError::InvalidSentence {
                    field: "raw_appearances.label",
                });
            }
            scopes_covered.insert(appearance.client_scope);
            match by_object.get_mut(&appearance.object_ref) {
                Some(existing) => {
                    if existing.count_class != appearance.count_class
                        || existing.disposition != appearance.disposition
                    {
                        return Err(BuildError::DedupeClassConflict {
                            object_ref: appearance.object_ref.clone(),
                        });
                    }
                    if !existing.appearances.contains(&appearance.client_scope) {
                        existing.appearances.push(appearance.client_scope);
                    }
                    existing.raw_appearance_count += 1;
                    existing.durable_object_preserved &= appearance.durable_object_preserved;
                }
                None => {
                    by_object.insert(
                        appearance.object_ref.clone(),
                        DedupedDurableObject {
                            object_ref: appearance.object_ref.clone(),
                            count_class: appearance.count_class,
                            disposition: appearance.disposition,
                            label: appearance.label.clone(),
                            owner_subsystem: appearance.owner_subsystem,
                            appearances: vec![appearance.client_scope],
                            raw_appearance_count: 1,
                            durable_object_preserved: appearance.durable_object_preserved,
                            reopen_target_ref: appearance.reopen_target_ref.clone(),
                            canonical_event_id: appearance.canonical_event_id.clone(),
                            dedupe_key_scheme: appearance.dedupe_key_scheme,
                            dedupe_key_ref: appearance.dedupe_key_ref.clone(),
                            route_outcome_id_ref: appearance.route_outcome_id_ref.clone(),
                            envelope_id_ref: appearance.envelope_id_ref.clone(),
                        },
                    );
                }
            }
        }
        let mut deduped_objects: Vec<DedupedDurableObject> = by_object.into_values().collect();
        for object in &mut deduped_objects {
            object.appearances.sort();
        }
        let deduped_object_count = deduped_objects.len() as u32;
        let cross_client_collapsed = raw_appearance_count - deduped_object_count;

        // --- per-class aggregates --------------------------------------------
        let mut active_by_class: BTreeMap<AggregateCountClass, u32> = BTreeMap::new();
        let mut held_by_class: BTreeMap<AggregateCountClass, u32> = BTreeMap::new();
        let mut cleared_by_class: BTreeMap<AggregateCountClass, u32> = BTreeMap::new();
        for object in &deduped_objects {
            let bucket = match object.disposition {
                DurableItemDisposition::Active => &mut active_by_class,
                DurableItemDisposition::HeldOrSuppressed => &mut held_by_class,
                DurableItemDisposition::ClearedResolved => &mut cleared_by_class,
            };
            *bucket.entry(object.count_class).or_insert(0) += 1;
        }
        let present_classes: BTreeSet<AggregateCountClass> = deduped_objects
            .iter()
            .map(|object| object.count_class)
            .collect();
        let class_aggregates: Vec<ClassAggregate> = AggregateCountClass::ALL
            .into_iter()
            .filter(|class| present_classes.contains(class))
            .map(|class| {
                let active_count = *active_by_class.get(&class).unwrap_or(&0);
                let held_or_suppressed_count = *held_by_class.get(&class).unwrap_or(&0);
                ClassAggregate {
                    count_class: class,
                    active_count,
                    held_or_suppressed_count,
                    cleared_count: *cleared_by_class.get(&class).unwrap_or(&0),
                    privacy_safe_summary_label: class_summary_label(
                        class,
                        active_count,
                        held_or_suppressed_count,
                    ),
                }
            })
            .collect();

        // The authoritative active count per class — the number every surface
        // must agree with (or explain a per-class disablement against).
        let authoritative_active =
            |class: AggregateCountClass| -> u32 { *active_by_class.get(&class).unwrap_or(&0) };

        // zero-means-none holds by construction: active_count is derived from
        // Active objects only; held / cleared objects never leak into it.
        let zero_means_no_durable_items = AggregateCountClass::ALL.into_iter().all(|class| {
            let active_objects = deduped_objects
                .iter()
                .filter(|object| {
                    object.count_class == class
                        && object.disposition == DurableItemDisposition::Active
                })
                .count() as u32;
            (authoritative_active(class) == 0) == (active_objects == 0)
                && authoritative_active(class) == active_objects
        });

        // --- surface projections ---------------------------------------------
        let mut seen_surfaces: BTreeSet<BadgeSurface> = BTreeSet::new();
        for projection in &input.surface_projections {
            if !seen_surfaces.insert(projection.surface) {
                return Err(BuildError::DuplicateSurfaceProjection {
                    surface: projection.surface,
                });
            }
        }
        for required in BadgeSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceProjectionMissing { surface: required });
            }
        }

        let mut surface_projections: Vec<SurfaceProjection> = Vec::new();
        let mut surfaces_share_durable_set = true;
        let mut no_surface_inflation = true;
        let mut activity_center_authoritative = true;
        // (surface, class) disablements that need a lineage entry.
        let mut disablements_needing_lineage: Vec<(BadgeSurface, AggregateCountClass)> = Vec::new();
        for required in BadgeSurface::REQUIRED {
            let projection = input
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .expect("surface presence checked above");
            let disabled: BTreeSet<AggregateCountClass> =
                projection.disabled_classes.iter().copied().collect();
            let reported: BTreeMap<AggregateCountClass, u32> = projection
                .class_counts
                .iter()
                .map(|entry| (entry.count_class, entry.reported_count))
                .collect();

            let mut matches = true;
            let mut inflates = false;
            // Check every class that is either present in the durable set or
            // reported by the surface.
            let mut classes: BTreeSet<AggregateCountClass> = present_classes.clone();
            classes.extend(reported.keys().copied());
            for class in classes {
                let expected = if disabled.contains(&class) {
                    0
                } else {
                    authoritative_active(class)
                };
                let reported_count = *reported.get(&class).unwrap_or(&0);
                if reported_count > expected {
                    inflates = true;
                    matches = false;
                } else if reported_count < expected {
                    matches = false;
                }
                if disabled.contains(&class) && authoritative_active(class) > 0 {
                    disablements_needing_lineage.push((required, class));
                }
            }
            if inflates {
                no_surface_inflation = false;
            }
            if !matches {
                surfaces_share_durable_set = false;
            }
            // The activity center is the authoritative durable surface: it may
            // not disable a class or diverge from the durable set.
            if required == BadgeSurface::ActivityCenter && (!matches || !disabled.is_empty()) {
                activity_center_authoritative = false;
            }

            let mut class_counts: Vec<SurfaceClassCount> = projection.class_counts.clone();
            class_counts.sort_by_key(|entry| entry.count_class);
            let mut disabled_classes: Vec<AggregateCountClass> =
                projection.disabled_classes.clone();
            disabled_classes.sort();
            let total_reported = class_counts.iter().map(|entry| entry.reported_count).sum();
            surface_projections.push(SurfaceProjection {
                surface: required,
                surface_marker: projection.surface_marker,
                class_counts,
                disabled_classes,
                total_reported,
                matches_durable_set: matches,
                inflates_any_class: inflates,
            });
        }

        // --- suppression lineage ---------------------------------------------
        for entry in &input.suppression_lineage {
            if !is_reviewable_sentence(&entry.export_safe_summary) {
                return Err(BuildError::InvalidSentence {
                    field: "suppression_lineage.export_safe_summary",
                });
            }
            if let Some(object_ref) = &entry.object_ref {
                require_canonical_ref("suppression_lineage.object_ref", object_ref)?;
            }
        }
        // Every held / suppressed object needs an object-scoped lineage entry.
        for object in &deduped_objects {
            if object.disposition != DurableItemDisposition::HeldOrSuppressed {
                continue;
            }
            let covered = input.suppression_lineage.iter().any(|entry| {
                entry.scope == SuppressionScope::Object
                    && entry.object_ref.as_deref() == Some(object.object_ref.as_str())
            });
            if !covered {
                return Err(BuildError::MissingObjectSuppressionLineage {
                    object_ref: object.object_ref.clone(),
                });
            }
        }
        // Every per-class disablement on a surface needs a lineage entry.
        for (surface, class) in &disablements_needing_lineage {
            let covered = input.suppression_lineage.iter().any(|entry| {
                entry.scope == SuppressionScope::SurfaceClass
                    && entry.surface == Some(*surface)
                    && entry.count_class == Some(*class)
            });
            if !covered {
                return Err(BuildError::MissingDisablementLineage {
                    surface: *surface,
                    count_class: *class,
                });
            }
        }
        let lineage_preserves_truth = input
            .suppression_lineage
            .iter()
            .all(|entry| entry.durable_object_preserved && entry.reopen_target_preserved);

        // --- derive pillars --------------------------------------------------
        let one_durable_set_holds =
            surfaces_share_durable_set && no_surface_inflation && activity_center_authoritative;
        let class_integrity_preserved = true; // conflicts are hard-errored above.
        let cross_client_dedupe_holds = class_integrity_preserved;
        let suppression_lineage_export_safe = lineage_preserves_truth;
        let all_objects_preserved = deduped_objects
            .iter()
            .all(|object| object.durable_object_preserved);
        let summary_persistent_inspectable = all_objects_preserved;
        let pillars = BadgeAggregatePillars {
            one_durable_set_holds,
            cross_client_dedupe_holds,
            suppression_lineage_export_safe,
            zero_means_no_durable_items,
            summary_persistent_inspectable,
        };

        // --- claim ceiling: never claim what cannot be proven ----------------
        if input.claim_ceiling.asserts_one_durable_set && !one_durable_set_holds {
            return Err(BuildError::OverclaimsOneDurableSet);
        }
        if input.claim_ceiling.asserts_cross_client_dedupe && !cross_client_dedupe_holds {
            return Err(BuildError::OverclaimsCrossClientDedupe);
        }
        if input.claim_ceiling.asserts_suppression_lineage_export_safe
            && !suppression_lineage_export_safe
        {
            return Err(BuildError::OverclaimsSuppressionLineage);
        }
        if input.claim_ceiling.asserts_zero_means_no_durable_items && !zero_means_no_durable_items {
            return Err(BuildError::OverclaimsZeroMeansNone);
        }
        if input.claim_ceiling.asserts_summary_persistent_inspectable
            && !summary_persistent_inspectable
        {
            return Err(BuildError::OverclaimsSummaryPersistent);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in BadgeRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_item {
                return Err(BuildError::RouteTargetsDifferentItem {
                    surface: route.surface,
                });
            }
        }
        for required in AttentionRouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- cross-client dedupe disclosure ----------------------------------
        let dedupe_key_scheme = deduped_objects
            .first()
            .map(|object| object.dedupe_key_scheme)
            .unwrap_or(DedupeKeyScheme::CrossClientCanonicalEventId);
        let cross_client_dedupe = CrossClientDedupeDisclosure {
            raw_appearance_count,
            deduped_object_count,
            cross_client_collapsed,
            dedupe_key_scheme,
            class_integrity_preserved,
            scopes_covered: scopes_covered.into_iter().collect(),
        };

        // --- persistent attention summary ------------------------------------
        let total_active: u32 = class_aggregates.iter().map(|c| c.active_count).sum();
        let total_held_or_suppressed: u32 = class_aggregates
            .iter()
            .map(|c| c.held_or_suppressed_count)
            .sum();
        let summary_digest = PersistentAttentionSummary {
            summary_id: input.summary_id,
            total_active,
            total_held_or_suppressed,
            per_class: class_aggregates.clone(),
            privacy_safe_summary_label: aggregate_summary_label(
                total_active,
                total_held_or_suppressed,
            ),
            durable_and_persistent: all_objects_preserved,
            inspectable_in_product: true,
        };

        // --- surface marker = lowest among the badge surfaces ----------------
        let surface_lifecycle_marker = surface_projections
            .iter()
            .map(|projection| projection.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !one_durable_set_holds {
            narrowing_reasons.push(BadgeAggregateNarrowingReason::OneDurableSetNotProven);
        }
        if !cross_client_dedupe_holds {
            narrowing_reasons
                .push(BadgeAggregateNarrowingReason::CrossClientDedupeNotClassTruthful);
        }
        if !suppression_lineage_export_safe {
            narrowing_reasons.push(BadgeAggregateNarrowingReason::SuppressionLineageNotExportSafe);
        }
        if !zero_means_no_durable_items {
            narrowing_reasons.push(BadgeAggregateNarrowingReason::ZeroDoesNotMeanNoDurableItems);
        }
        if !summary_persistent_inspectable {
            narrowing_reasons
                .push(BadgeAggregateNarrowingReason::SummaryNotPersistentOrInspectable);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(BadgeAggregateNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == BadgeAggregateNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = BadgeAggregateQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present =
            !qualifies_stable || surface_lifecycle_marker.is_below_stable();

        // --- normalise upstream refs -----------------------------------------
        let mut contributing_case_refs = input.upstream.contributing_case_refs.clone();
        contributing_case_refs.sort();
        contributing_case_refs.dedup();
        let mut contributing_route_outcome_refs =
            input.upstream.contributing_route_outcome_refs.clone();
        contributing_route_outcome_refs.sort();
        contributing_route_outcome_refs.dedup();

        Ok(Self {
            record_kind: BADGE_AGGREGATE_RECORD_KIND.to_string(),
            schema_version: BADGE_AGGREGATE_SCHEMA_VERSION,
            notice: BADGE_AGGREGATE_NOTICE.to_string(),
            shared_contract_ref: BADGE_AGGREGATE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            surface_lifecycle_marker,
            deduped_objects,
            class_aggregates,
            surface_projections,
            cross_client_dedupe,
            suppression_lineage: input.suppression_lineage,
            active_quiet_hours_modes: input.active_quiet_hours_modes,
            summary_digest,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: BadgeAggregateUpstream {
                corpus_packet_ref: input.upstream.corpus_packet_ref,
                contributing_case_refs,
                contributing_route_outcome_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("badge_aggregate: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!("surface_lifecycle_marker: {}", self.surface_lifecycle_marker.as_str()),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: one_durable_set={} cross_client_dedupe={} suppression_lineage={} zero_means_none={} summary_persistent={}",
                self.pillars.one_durable_set_holds,
                self.pillars.cross_client_dedupe_holds,
                self.pillars.suppression_lineage_export_safe,
                self.pillars.zero_means_no_durable_items,
                self.pillars.summary_persistent_inspectable
            ),
            format!(
                "cross_client_dedupe: raw={} deduped={} collapsed={} scheme={} class_integrity={} scopes=[{}]",
                self.cross_client_dedupe.raw_appearance_count,
                self.cross_client_dedupe.deduped_object_count,
                self.cross_client_dedupe.cross_client_collapsed,
                snake_scheme(self.cross_client_dedupe.dedupe_key_scheme),
                self.cross_client_dedupe.class_integrity_preserved,
                self.cross_client_dedupe
                    .scopes_covered
                    .iter()
                    .map(|scope| snake_scope(*scope))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "summary: total_active={} total_held={} label={:?} durable={} inspectable={}",
                self.summary_digest.total_active,
                self.summary_digest.total_held_or_suppressed,
                self.summary_digest.privacy_safe_summary_label,
                self.summary_digest.durable_and_persistent,
                self.summary_digest.inspectable_in_product
            ),
        ];
        lines.push("class_aggregates:".to_string());
        for class in &self.class_aggregates {
            lines.push(format!(
                "  - {} active={} held={} cleared={} label={:?}",
                class.count_class.as_str(),
                class.active_count,
                class.held_or_suppressed_count,
                class.cleared_count,
                class.privacy_safe_summary_label
            ));
        }
        lines.push("surface_projections:".to_string());
        for projection in &self.surface_projections {
            lines.push(format!(
                "  - {} marker={} total={} matches={} inflates={} disabled=[{}]",
                projection.surface.as_str(),
                projection.surface_marker.as_str(),
                projection.total_reported,
                projection.matches_durable_set,
                projection.inflates_any_class,
                projection
                    .disabled_classes
                    .iter()
                    .map(|class| class.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        lines.push("suppression_lineage:".to_string());
        for entry in &self.suppression_lineage {
            lines.push(format!(
                "  - {} scope={} object={:?} class={:?} surface={:?} preserves_object={} preserves_reopen={} :: {}",
                entry.reason.as_str(),
                entry.scope.as_str(),
                entry.object_ref,
                entry.count_class.map(|c| c.as_str()),
                entry.surface.map(|s| s.as_str()),
                entry.durable_object_preserved,
                entry.reopen_target_preserved,
                entry.export_safe_summary
            ));
        }
        lines.push("deduped_objects:".to_string());
        for object in &self.deduped_objects {
            lines.push(format!(
                "  - {} class={} disposition={} raw_appearances={} scopes=[{}] preserved={}",
                object.object_ref,
                object.count_class.as_str(),
                object.disposition.as_str(),
                object.raw_appearance_count,
                object
                    .appearances
                    .iter()
                    .map(|scope| snake_scope(*scope))
                    .collect::<Vec<_>>()
                    .join(", "),
                object.durable_object_preserved
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

/// Privacy-safe one-line summary for the whole aggregate.
fn aggregate_summary_label(total_active: u32, total_held_or_suppressed: u32) -> String {
    match (total_active, total_held_or_suppressed) {
        (0, 0) => "No attention items".to_string(),
        (0, h) => format!("No active items; {h} held"),
        (1, 0) => "1 attention item".to_string(),
        (n, 0) => format!("{n} attention items"),
        (1, h) => format!("1 attention item; {h} held"),
        (n, h) => format!("{n} attention items; {h} held"),
    }
}

fn snake_scheme(scheme: DedupeKeyScheme) -> String {
    serde_json::to_value(scheme)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default()
}

fn snake_scope(scope: ClientScope) -> String {
    serde_json::to_value(scope)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Recovery vocabulary
// ---------------------------------------------------------------------------

/// Closed recovery-action vocabulary exposed on a badge-aggregate snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeRecoveryAction {
    /// Open the activity center, the authoritative durable surface.
    OpenActivityCenter,
    /// Open per-class badge settings to inspect or change count classes.
    OpenBadgeSettings,
    /// Inspect the suppression lineage for muted / suppressed / disabled badges.
    InspectSuppressionLineage,
    /// Export a redacted badge-aggregate support packet.
    ExportBadgeSupport,
}

impl BadgeRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenActivityCenter => "open_activity_center",
            Self::OpenBadgeSettings => "open_badge_settings",
            Self::InspectSuppressionLineage => "inspect_suppression_lineage",
            Self::ExportBadgeSupport => "export_badge_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenActivityCenter => "Open activity center",
            Self::OpenBadgeSettings => "Badge settings",
            Self::InspectSuppressionLineage => "Inspect suppression lineage",
            Self::ExportBadgeSupport => "Export badge support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenActivityCenter => RecoveryActionRole::Primary,
            Self::OpenBadgeSettings | Self::InspectSuppressionLineage => {
                RecoveryActionRole::Recovery
            }
            Self::ExportBadgeSupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every snapshot must expose, in rendered order.
    pub const REQUIRED: [Self; 4] = [
        Self::OpenActivityCenter,
        Self::OpenBadgeSettings,
        Self::InspectSuppressionLineage,
        Self::ExportBadgeSupport,
    ];

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery routes every snapshot must expose, in rendered order.
pub fn required_recovery_routes() -> Vec<RecoveryRouteRecord> {
    BadgeRecoveryAction::REQUIRED
        .into_iter()
        .map(BadgeRecoveryAction::route)
        .collect()
}
