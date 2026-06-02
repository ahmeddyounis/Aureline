//! Canonical stable truth model for notification routing, durable
//! activity-center / job-row truth, quiet-hours policy, privacy-safe OS alerts,
//! interruptibility, and exact-target reopen on a claimed-stable attention
//! surface.
//!
//! ## Why one lock record per durable attention class
//!
//! A long-running or reviewable flow — indexing, restore, install/update,
//! download, attach/reconnect, a task/test run, a provider sync, a policy
//! notice, a managed alert — must survive look-away, quiet-hours policy,
//! sleep/resume, and cross-client delivery without losing its identity or
//! quietly bypassing approval. When the toast, the banner, the status item, the
//! activity-center row, the dock badge, the native OS notification, and the
//! companion push each reason about the alert with their own bespoke behaviour
//! they drift: a toast becomes the *only* record of a failure, a badge outpaces
//! the durable job model, a notification shortcut runs a mutating action off the
//! lock screen, or a reopen lands on a generic home pane instead of the
//! authoritative object.
//!
//! This module mints one governed [`AttentionLockRecord`] per claimed-stable
//! durable attention class. The record binds, for a single canonical attention
//! identity:
//!
//! - **The routing** — the alert flows from one typed notification envelope
//!   through the one governed router into a single
//!   [`crate::attention_router::NotificationRouteOutcome`]; every resolved
//!   surface keeps the same reopen target.
//! - **The durable job row** — a job id, actor/subsystem, current phase, label,
//!   and cancel/retry/open-details affordances that survive look-away,
//!   sleep/resume, and restart/restore where continuity is claimed.
//! - **The quiet-hours / admin-suppression policy** — applied coherently across
//!   in-app, OS, and companion surfaces; suppression may change fanout but never
//!   erases the durable object, the reopen target, or the audit trail.
//! - **The privacy-safe OS alert** — lock-screen / notification-center copy is
//!   summary-first and never exposes secrets, raw code, AI prompt content, or
//!   high-risk action detail by default.
//! - **Interruptibility** — durable jobs and repeated failures never degrade to
//!   toast-only truth; repeats coalesce by root cause instead of badge churn.
//! - **The exact-target reopen** — notifications, badges, and job rows reopen
//!   the authoritative object, or a truthful placeholder that names what is now
//!   unavailable, and never re-issue a side effect from the notification
//!   surface.
//! - **The lifecycle semantics** — acknowledge, resolve, dismiss, snooze, and
//!   mute are distinct state transitions derived from one durable activity
//!   object, not surface-local counters.
//! - **The badge truth** — counts derive from durable item state, not raw event
//!   fanout.
//! - **A public claim ceiling** — no row asserts a pillar the product cannot
//!   prove.
//! - **Automatic narrowing** — a row missing a pillar, or sitting on a surface
//!   whose own lifecycle marker is below Stable, is narrowed below Stable with a
//!   named reason instead of inheriting an adjacent green row.
//! - **Recovery, route, and accessibility parity** — the same item reachable
//!   from the activity center, command palette, status bar, and a menu command,
//!   keyboard-first, in normal / high-contrast / zoomed layouts.
//! - **No-account / no-managed-services availability**.
//!
//! The envelope, router, lifecycle grammar, quiet-hours posture, and badge
//! reconciliation are **not** reinvented here. The record is a genuine
//! projection of the live attention stack in [`crate::notifications`] and
//! [`crate::attention_router`]; the corpus routes real envelopes through the one
//! governed router and reads the resulting outcome, so a lock record can never
//! drift from what ships.

use serde::{Deserialize, Serialize};

use crate::attention_router::AvailableLifecycleAction;
use crate::notifications::actions::{BadgeClass, NotificationLifecycleActionKind};
use crate::notifications::envelope::{
    DedupeKeyScheme, PrivacyClass, PrivacyPayloadClass, QuietHoursMode, RedactionClass,
    ReopenTargetKind, SeverityClass, SourceSubsystem,
};

/// Stable record-kind tag carried in serialized lock records.
pub const ATTENTION_LOCK_RECORD_KIND: &str = "notification_attention_lock_record";

/// Schema version for the [`AttentionLockRecord`] payload shape.
pub const ATTENTION_LOCK_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const ATTENTION_LOCK_SHARED_CONTRACT_REF: &str = "shell:notification_attention_lock_stable:v1";

/// Reviewer-facing notice rendered on every lock surface.
pub const ATTENTION_LOCK_NOTICE: &str =
    "Durable attention truth: the toast, banner, status item, activity-center row, dock/taskbar \
     badge, native OS notification, and companion push all route from one typed notification \
     envelope into one governed route outcome; every launch-critical long-running or reviewable \
     flow keeps a durable job row that survives look-away, sleep/resume, and restart, so no \
     meaningful policy, network, cost, trust, or repair issue is represented by toast-only truth; \
     quiet-hours and admin suppression may change fanout surfaces but never erase the durable \
     object, the reopen target, or the audit trail; lock-screen and notification-center copy is \
     summary-first and never exposes secrets, raw code, AI prompt content, or high-risk action \
     detail by default; acknowledge, resolve, dismiss, snooze, and mute are distinct transitions \
     on the durable object and badge counts derive from durable item state, not raw fanout; exact-\
     target reopen returns to the authoritative object or a truthful placeholder and never \
     re-issues a side effect from a notification surface; a row missing a pillar, or on a surface \
     whose own marker is below Stable, is narrowed below Stable with a named reason rather than \
     inheriting an adjacent green row; the same item opens from the activity center, command \
     palette, status bar, and a menu command, keyboard-first; and every row stays available \
     without an account or managed services.";

/// Canonical durable-object URI scheme. Every minted ref must be one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one is rejected so chrome cannot
/// wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home",
    "dashboard",
    "landing",
    "index",
    "overview",
    "start",
    "root",
];

/// The lifecycle verbs every claimed-stable durable attention row must keep
/// distinct: acknowledge, resolve, dismiss, snooze, and mute. `Clear` and
/// `Suppress` exist on the upstream grammar but are not part of the required
/// distinct-verb bar this record asserts.
pub const REQUIRED_LIFECYCLE_VERBS: [NotificationLifecycleActionKind; 5] = [
    NotificationLifecycleActionKind::Acknowledge,
    NotificationLifecycleActionKind::Resolve,
    NotificationLifecycleActionKind::Dismiss,
    NotificationLifecycleActionKind::Snooze,
    NotificationLifecycleActionKind::Mute,
];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

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
        Err(BuildError::MissingUpstreamRef { field })
    }
}

/// Compact snake_case token for any of the upstream stable enums, derived
/// through serde so this record never maintains a parallel vocabulary.
pub fn snake_token<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default()
}

/// Public claim class for the lane, reusing the stable lifecycle cutline.
///
/// `Stable` sits at or above the launch cutline; everything else is narrowed
/// below it. The builder *derives* this from the evidence, so a row can never
/// publish a claim wider than its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The durable attention pillars are replacement-grade.
    Stable,
    /// Narrowed to the beta promise.
    Beta,
    /// Narrowed to the preview / limited-availability promise.
    Preview,
    /// No public promise yet.
    NotClaimed,
}

impl StableClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }

    /// Returns `true` when the claim sits at or above the launch cutline.
    pub const fn at_or_above_cutline(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Closed reason a row is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableNarrowingReason {
    /// The flow does not keep a durable surface; truth would be toast-only.
    DurableAttentionNotProven,
    /// Quiet-hours / admin suppression does not preserve the durable object,
    /// the reopen target, or the audit trail coherently across surfaces.
    QuietHoursNotCoherent,
    /// The OS / lock-screen alert is not summary-first and privacy-safe.
    OsAlertNotPrivacySafe,
    /// Badge counts do not derive from durable item state.
    BadgeCountNotClassTruthful,
    /// Exact-target reopen is not deterministic (or could land on a generic
    /// home / re-issue a side effect from a notification surface).
    ExactTargetReopenNotDeterministic,
    /// The attention surface's own lifecycle marker is below Stable, so it must
    /// not inherit Stable by adjacency.
    SurfaceNotYetStable,
}

impl StableNarrowingReason {
    /// Returns the stable string vocabulary for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableAttentionNotProven => "durable_attention_not_proven",
            Self::QuietHoursNotCoherent => "quiet_hours_not_coherent",
            Self::OsAlertNotPrivacySafe => "os_alert_not_privacy_safe",
            Self::BadgeCountNotClassTruthful => "badge_count_not_class_truthful",
            Self::ExactTargetReopenNotDeterministic => "exact_target_reopen_not_deterministic",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

/// Lifecycle marker carried by the attention surface itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMarker {
    /// Preview / limited-availability.
    Preview,
    /// Beta promise.
    Beta,
    /// Replacement-grade stable.
    Stable,
}

impl LifecycleMarker {
    /// Returns the stable string vocabulary for this marker.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Returns `true` when the marker sits below the stable cutline.
    pub const fn is_below_stable(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Surface a durable attention item can be reached from. The same item must be
/// reachable from all four so the activity center and the in-product attention
/// surfaces stay consistent for keyboard-only and assistive-technology users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionRouteSurface {
    /// The activity center.
    ActivityCenter,
    /// The command palette.
    CommandPalette,
    /// The status bar / status overflow.
    StatusBar,
    /// An application menu command.
    MenuCommand,
}

impl AttentionRouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::CommandPalette => "command_palette",
            Self::StatusBar => "status_bar",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach an item.
    pub const REQUIRED: [Self; 4] = [
        Self::ActivityCenter,
        Self::CommandPalette,
        Self::StatusBar,
        Self::MenuCommand,
    ];
}

/// Layout mode an accessibility disclosure is checked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    /// Default desktop layout.
    Normal,
    /// High-contrast theme.
    HighContrast,
    /// Zoomed / enlarged layout.
    Zoomed,
}

impl LayoutMode {
    /// Returns the stable string vocabulary for this layout mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::HighContrast => "high_contrast",
            Self::Zoomed => "zoomed",
        }
    }

    /// The three layout modes every disclosure must hold in.
    pub const REQUIRED: [Self; 3] = [Self::Normal, Self::HighContrast, Self::Zoomed];
}

/// Role a recovery action plays, used for placement and confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionRole {
    /// Opens or reopens the canonical attention object.
    Primary,
    /// Cancels, retries, or otherwise recovers the durable job.
    Recovery,
    /// Non-mutating acknowledge / export.
    Secondary,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
        }
    }
}

/// Closed recovery-action vocabulary exposed on a durable attention row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionRecoveryAction {
    /// Open the activity center scrolled to this item.
    OpenActivityCenter,
    /// Open the durable job's details.
    OpenJobDetails,
    /// Reopen the authoritative object or its truthful placeholder.
    ReopenTarget,
    /// Cancel the running job (only when the job is safely cancelable).
    CancelJob,
    /// Retry the job (only when the job is safely retriable).
    RetryJob,
    /// Acknowledge the item, clearing active attention without mutating source.
    AcknowledgeItem,
    /// Resolve the item through its owning model (only when resolvable).
    ResolveItem,
    /// Export a redacted attention-support packet.
    ExportAttentionSupport,
}

impl AttentionRecoveryAction {
    /// Returns the stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenActivityCenter => "open_activity_center",
            Self::OpenJobDetails => "open_job_details",
            Self::ReopenTarget => "reopen_target",
            Self::CancelJob => "cancel_job",
            Self::RetryJob => "retry_job",
            Self::AcknowledgeItem => "acknowledge_item",
            Self::ResolveItem => "resolve_item",
            Self::ExportAttentionSupport => "export_attention_support",
        }
    }

    /// Returns the reviewer-facing action label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenActivityCenter => "Open activity center",
            Self::OpenJobDetails => "Open job details",
            Self::ReopenTarget => "Reopen target",
            Self::CancelJob => "Cancel job",
            Self::RetryJob => "Retry job",
            Self::AcknowledgeItem => "Acknowledge",
            Self::ResolveItem => "Resolve",
            Self::ExportAttentionSupport => "Export attention support",
        }
    }

    /// Returns the placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenActivityCenter | Self::OpenJobDetails | Self::ReopenTarget => {
                RecoveryActionRole::Primary
            }
            Self::CancelJob | Self::RetryJob | Self::ResolveItem => RecoveryActionRole::Recovery,
            Self::AcknowledgeItem | Self::ExportAttentionSupport => RecoveryActionRole::Secondary,
        }
    }

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

/// Returns the recovery actions a row must expose, in rendered order, given the
/// durable job's safe affordances.
pub fn required_recovery_actions(
    cancelable: bool,
    retriable: bool,
    resolvable: bool,
) -> Vec<AttentionRecoveryAction> {
    let mut actions = vec![
        AttentionRecoveryAction::OpenActivityCenter,
        AttentionRecoveryAction::OpenJobDetails,
        AttentionRecoveryAction::ReopenTarget,
    ];
    if cancelable {
        actions.push(AttentionRecoveryAction::CancelJob);
    }
    if retriable {
        actions.push(AttentionRecoveryAction::RetryJob);
    }
    actions.push(AttentionRecoveryAction::AcknowledgeItem);
    if resolvable {
        actions.push(AttentionRecoveryAction::ResolveItem);
    }
    actions.push(AttentionRecoveryAction::ExportAttentionSupport);
    actions
}

/// The one-envelope routing disclosure: the alert flows through the single
/// governed router and every resolved surface keeps the same reopen target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingDisclosure {
    /// Upstream notification-envelope id this record projects from.
    pub envelope_id_ref: String,
    /// Upstream route-outcome id this record projects from.
    pub route_outcome_id_ref: String,
    /// Canonical event id, stable across surfaces and dedupe repeats.
    pub canonical_event_id: String,
    /// Source subsystem the alert originated from.
    pub source_subsystem: SourceSubsystem,
    /// Severity class used for routing and escalation.
    pub severity_class: SeverityClass,
    /// Dedupe key scheme the router collapses repeats with.
    pub dedupe_key_scheme: DedupeKeyScheme,
    /// Dedupe key the router collapses repeats with.
    pub dedupe_key_ref: String,
    /// Count of resolved surface routes.
    pub resolved_surface_count: u32,
    /// Count of resolved routes that actually render.
    pub visible_surface_count: u32,
    /// Whether every resolved surface points at the same reopen target.
    pub routes_from_one_envelope: bool,
}

/// The durable activity-center / job row bound to this attention identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableJobRow {
    /// Stable job id.
    pub job_id: String,
    /// Canonical durable-object ref the activity center, badges, and reopen all
    /// resolve from.
    pub durable_object_ref: String,
    /// Actor / owning subsystem.
    pub actor_subsystem: SourceSubsystem,
    /// Reviewable label.
    pub label: String,
    /// Reviewable current-phase sentence.
    pub current_phase: String,
    /// Whether a Cancel affordance is safely available.
    pub cancelable: bool,
    /// Whether a Retry affordance is safely available.
    pub retriable: bool,
    /// Whether a Resolve affordance is safely available.
    pub resolvable: bool,
    /// Whether an Open-details affordance is available.
    pub open_details_available: bool,
    /// Whether a durable surface (durable row / status / activity-center card)
    /// is present so the truth is never toast-only.
    pub durable_surface_present: bool,
    /// Whether the row survives look-away / window switch.
    pub survives_lookaway: bool,
    /// Whether the row survives sleep / resume.
    pub survives_sleep_resume: bool,
    /// Whether the row survives restart / restore where continuity is claimed.
    pub survives_restart_restore: bool,
}

impl DurableJobRow {
    /// Returns `true` when the job keeps durable truth across look-away and
    /// sleep/resume on a present durable surface.
    pub fn is_durable(&self) -> bool {
        self.durable_surface_present && self.survives_lookaway && self.survives_sleep_resume
    }
}

/// Quiet-hours / admin-suppression policy applied coherently across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuietHoursPolicy {
    /// Effective quiet-hours / focus / admin modes active for this outcome.
    pub active_modes: Vec<QuietHoursMode>,
    /// Whether suppression preserves the durable object.
    pub suppression_preserves_durable_object: bool,
    /// Whether suppression preserves the exact reopen target.
    pub suppression_preserves_reopen_target: bool,
    /// Whether a suppression audit trail (a receipt per held/suppressed route)
    /// is present.
    pub suppression_audit_trail_present: bool,
    /// Whether the policy is applied coherently across in-app, OS, and
    /// companion surfaces (suppression on external surfaces does not erase the
    /// durable in-app object or its reopen target).
    pub coherent_across_in_app_os_companion: bool,
}

impl QuietHoursPolicy {
    /// Returns `true` when suppression never erases the underlying activity
    /// object, reopen target, or audit trail.
    pub fn is_coherent(&self) -> bool {
        self.suppression_preserves_durable_object
            && self.suppression_preserves_reopen_target
            && self.suppression_audit_trail_present
            && self.coherent_across_in_app_os_companion
    }
}

/// Privacy-safe OS / lock-screen / companion alert posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacySafeAlert {
    /// Coarse privacy gate.
    pub privacy_class: PrivacyClass,
    /// Concrete OS / lock-screen payload redaction posture.
    pub privacy_payload_class: PrivacyPayloadClass,
    /// Export / retention redaction posture.
    pub redaction_class: RedactionClass,
    /// Whether lock-screen / notification-center copy is safe by default.
    pub lock_screen_safe_by_default: bool,
    /// Whether the external payload is summary-first.
    pub summary_first: bool,
    /// Whether the external payload would expose secrets, raw code, AI prompt
    /// content, or high-risk action detail (must be false).
    pub exposes_restricted_detail: bool,
    /// Whether companion / OS surfaces are summary-only.
    pub companion_summary_only: bool,
}

impl PrivacySafeAlert {
    /// Returns `true` when the OS / lock-screen / companion alert is privacy
    /// safe by default.
    pub fn is_privacy_safe(&self) -> bool {
        self.lock_screen_safe_by_default
            && self.summary_first
            && !self.exposes_restricted_detail
            && self.companion_summary_only
    }
}

/// Interruptibility posture: durable work and repeated failures never degrade
/// into toast-only truth or badge / toast spam.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interruptibility {
    /// Whether the meaningful record is never toast-only.
    pub no_toast_only_truth: bool,
    /// Whether a durable surface is present.
    pub durable_surface_present: bool,
    /// Whether repeated failures coalesce by root cause / owning object.
    pub repeated_failures_coalesced_by_root_cause: bool,
    /// Whether the routing avoids badge / toast spam.
    pub no_badge_or_toast_spam: bool,
}

impl Interruptibility {
    /// Returns `true` when interruptibility holds.
    pub fn holds(&self) -> bool {
        self.no_toast_only_truth
            && self.durable_surface_present
            && self.repeated_failures_coalesced_by_root_cause
            && self.no_badge_or_toast_spam
    }
}

/// Exact-target reopen posture for notifications, badges, and job rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactTargetReopen {
    /// Reopen target kind.
    pub reopen_target_kind: ReopenTargetKind,
    /// Upstream reopen target ref this record projects from.
    pub reopen_target_ref: String,
    /// Whether the reopen resolves to a concrete authoritative object.
    pub resolves_to_exact_target: bool,
    /// Whether a stale / unavailable target degrades to a truthful placeholder.
    pub degrades_to_truthful_placeholder: bool,
    /// Whether the reopen avoids a generic home / wrong pane.
    pub no_generic_home_reopen: bool,
    /// Whether every resolved route preserves the reopen target.
    pub all_routes_preserve_reopen_target: bool,
    /// Whether the notification surface cannot execute a destructive or
    /// authority-widening action on its own.
    pub no_side_effects_from_notification_surface: bool,
}

impl ExactTargetReopen {
    /// Returns `true` when reopen is deterministic and side-effect free.
    pub fn is_deterministic(&self) -> bool {
        (self.resolves_to_exact_target || self.degrades_to_truthful_placeholder)
            && self.no_generic_home_reopen
            && self.all_routes_preserve_reopen_target
            && self.no_side_effects_from_notification_surface
    }
}

/// Lifecycle semantics for the durable attention item: acknowledge, resolve,
/// dismiss, snooze, and mute are distinct transitions on one durable object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleSemantics {
    /// The governed lifecycle actions the router publishes for this outcome.
    pub available_actions: Vec<AvailableLifecycleAction>,
    /// Whether all required verbs are present.
    pub required_verbs_present: bool,
    /// Whether the required verbs are distinguishable by their export effect.
    pub verbs_distinct: bool,
}

/// Badge disclosure: counts derive from durable item state, not raw fanout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeDisclosure {
    /// Badge class this item contributes to.
    pub badge_class: BadgeClass,
    /// Active count reconciled from durable item state.
    pub active_count: u32,
    /// Held / suppressed count reconciled from durable item state.
    pub held_or_suppressed_count: u32,
    /// Whether durable history is preserved by the reconciliation.
    pub durable_history_preserved: bool,
    /// Whether the count derives from durable item state rather than raw
    /// fanout.
    pub derived_from_durable_item_state: bool,
    /// Privacy-safe compact label for badge / OS summary surfaces.
    pub privacy_safe_summary_label: String,
}

impl BadgeDisclosure {
    /// Returns `true` when the badge count matches the durable item class.
    pub fn count_class_truthful(&self) -> bool {
        self.durable_history_preserved && self.derived_from_durable_item_state
    }
}

/// The public claim ceiling: what a row is allowed to assert. Each field must be
/// provable from the row's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AttentionClaimCeiling {
    /// Whether the row may claim durable attention is proven.
    pub asserts_durable_attention: bool,
    /// Whether the row may claim quiet-hours / admin suppression is coherent.
    pub asserts_quiet_hours_coherent: bool,
    /// Whether the row may claim the OS / lock-screen alert is privacy-safe.
    pub asserts_privacy_safe: bool,
    /// Whether the row may claim badge counts are class-truthful.
    pub asserts_badge_count_class_truthful: bool,
    /// Whether the row may claim exact-target reopen is deterministic.
    pub asserts_exact_target_reopen: bool,
}

/// The derived stable-claim verdict for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the row qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the row is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<StableNarrowingReason>,
}

/// One recovery route exposed on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
}

/// One route to the same item from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: AttentionRouteSurface,
    /// Canonical route ref pointing at the item on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same canonical attention item.
    pub activates_same_item: bool,
}

/// Accessibility disclosure for one layout mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutModeDisclosure {
    /// Layout mode this disclosure was checked under.
    pub mode: LayoutMode,
    /// Whether the row narration is available in this mode.
    pub row_narration_available: bool,
    /// Whether the recovery affordances stay reachable in this mode.
    pub recovery_affordances_reachable: bool,
}

/// Accessibility disclosure for one row across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the row in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the row and its actions expose.
    pub tab_stop_count: u32,
    /// Row narration read by assistive tech; discloses the owning subsystem.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

/// Cross-surface parity between the activity center and the other entry
/// projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParity {
    /// Activity-center row id for this item.
    pub activity_center_row_id: String,
    /// Status-bar item id for this item.
    pub status_bar_item_id: String,
    /// Command-palette command id that opens this item.
    pub command_palette_command_id: String,
    /// Recovery action ids shared by every surface.
    pub recovery_action_ids: Vec<String>,
    /// Reopen surfaces (os_notification / companion_push / support_export)
    /// retained.
    pub reopen_surfaces: Vec<String>,
    /// Whether the projections agree on identity and recovery behaviour.
    pub parity_holds: bool,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRefs {
    /// Notification-envelope corpus packet id this record projects from.
    pub corpus_packet_ref: String,
    /// Corpus case id the representative envelope came from.
    pub case_id_ref: String,
    /// Route-outcome id the routing facts came from.
    pub route_outcome_id_ref: String,
    /// Notification-envelope id the routing facts came from.
    pub envelope_id_ref: String,
}

/// Validated input used to mint an [`AttentionLockRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttentionLockInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable attention-class token (the canonical attention identity).
    pub attention_class: String,
    /// Compact attention-class label.
    pub attention_class_label: String,
    /// The attention surface's own lifecycle marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// Conformance-lane violation tokens for the projected route outcome (must
    /// be empty for a shippable row).
    pub route_conformance_violations: Vec<String>,
    /// The one-envelope routing disclosure.
    pub routing: RoutingDisclosure,
    /// The durable activity-center / job row.
    pub durable_job: DurableJobRow,
    /// The quiet-hours / admin-suppression policy.
    pub quiet_hours: QuietHoursPolicy,
    /// The privacy-safe OS / lock-screen alert posture.
    pub privacy: PrivacySafeAlert,
    /// The interruptibility posture.
    pub interruptibility: Interruptibility,
    /// The exact-target reopen posture.
    pub reopen: ExactTargetReopen,
    /// The lifecycle semantics.
    pub lifecycle: LifecycleSemantics,
    /// The badge disclosure.
    pub badge: BadgeDisclosure,
    /// Public claim ceiling for this row.
    pub claim_ceiling: AttentionClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same item.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the row stays available without an account.
    pub available_without_account: bool,
    /// Whether the row stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed durable-attention lock record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionLockRecord {
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
    /// Stable attention-class token.
    pub attention_class: String,
    /// Compact attention-class label.
    pub attention_class_label: String,
    /// The attention surface's own lifecycle marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The one-envelope routing disclosure.
    pub routing: RoutingDisclosure,
    /// The durable activity-center / job row.
    pub durable_job: DurableJobRow,
    /// The quiet-hours / admin-suppression policy.
    pub quiet_hours: QuietHoursPolicy,
    /// The privacy-safe OS / lock-screen alert posture.
    pub privacy: PrivacySafeAlert,
    /// The interruptibility posture.
    pub interruptibility: Interruptibility,
    /// The exact-target reopen posture.
    pub reopen: ExactTargetReopen,
    /// The lifecycle semantics.
    pub lifecycle: LifecycleSemantics,
    /// The badge disclosure.
    pub badge: BadgeDisclosure,
    /// Public claim ceiling.
    pub claim_ceiling: AttentionClaimCeiling,
    /// The derived stable-claim verdict (Stable, or narrowed with reasons).
    pub stable_qualification: StableQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same item.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the row stays available without an account.
    pub available_without_account: bool,
    /// Whether the row stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons an [`AttentionLockRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// An upstream projection ref was missing.
    MissingUpstreamRef { field: &'static str },
    /// The projected route outcome failed the conformance lane; such a row can
    /// never ship as Stable.
    RouteOutcomeNonConformant { tokens: Vec<String> },
    /// The claim ceiling asserted durable attention it cannot prove.
    OverclaimsDurableAttention,
    /// The claim ceiling asserted coherent quiet hours it cannot prove.
    OverclaimsQuietHoursCoherent,
    /// The claim ceiling asserted a privacy-safe alert it cannot prove.
    OverclaimsPrivacySafe,
    /// The claim ceiling asserted class-truthful badge counts it cannot prove.
    OverclaimsBadgeCountClass,
    /// The claim ceiling asserted deterministic exact-target reopen it cannot
    /// prove.
    OverclaimsExactTargetReopen,
    /// A required lifecycle verb was missing.
    MissingLifecycleVerb {
        verb: NotificationLifecycleActionKind,
    },
    /// The required lifecycle verbs were not distinguishable.
    LifecycleVerbsNotDistinct,
    /// The badge count would outpace the durable job model.
    BadgeOutpacesDurableModel,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: AttentionRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// The two surface projections disagreed on identity or recovery behaviour.
    SurfaceParityBroken,
    /// A required reopen surface was missing.
    ReopenSurfaceMissing { surface: &'static str },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: AttentionRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: AttentionRouteSurface },
    /// An entry route did not activate the same canonical item.
    RouteTargetsDifferentItem { surface: AttentionRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: AttentionRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The row narration did not disclose the owning subsystem.
    NarrationOmitsSubsystem,
    /// A row was hidden when no account was present.
    HiddenWithoutAccount,
    /// A row was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical object ref, got {value:?}")
            }
            Self::MissingUpstreamRef { field } => {
                write!(f, "upstream projection ref `{field}` must be present")
            }
            Self::RouteOutcomeNonConformant { tokens } => write!(
                f,
                "projected route outcome failed the conformance lane: {}",
                tokens.join(", ")
            ),
            Self::OverclaimsDurableAttention => write!(
                f,
                "claim ceiling may not assert durable attention without a present durable surface"
            ),
            Self::OverclaimsQuietHoursCoherent => write!(
                f,
                "claim ceiling may not assert coherent quiet hours when suppression erases truth"
            ),
            Self::OverclaimsPrivacySafe => write!(
                f,
                "claim ceiling may not assert a privacy-safe alert that leaks restricted detail"
            ),
            Self::OverclaimsBadgeCountClass => write!(
                f,
                "claim ceiling may not assert class-truthful badge counts not derived from durable state"
            ),
            Self::OverclaimsExactTargetReopen => write!(
                f,
                "claim ceiling may not assert deterministic reopen that could land on a generic home"
            ),
            Self::MissingLifecycleVerb { verb } => {
                write!(f, "row must expose the `{}` lifecycle verb", verb.as_str())
            }
            Self::LifecycleVerbsNotDistinct => write!(
                f,
                "acknowledge, resolve, dismiss, snooze, and mute must be distinct transitions"
            ),
            Self::BadgeOutpacesDurableModel => write!(
                f,
                "badge count must not outpace the durable job model"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(f, "row must expose recovery route `{}`", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route `{action_id}` must be keyboard reachable")
            }
            Self::SurfaceParityBroken => write!(
                f,
                "activity center and other surface projections must share identity and recovery behaviour"
            ),
            Self::ReopenSurfaceMissing { surface } => {
                write!(f, "reopen surface `{surface}` is missing")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentItem { surface } => write!(
                f,
                "entry route surface `{}` must activate the same item",
                surface.as_str()
            ),
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "entry route surface `{}` is duplicated", surface.as_str())
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "accessibility layout mode `{}` is missing", mode.as_str())
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
            Self::NarrationOmitsSubsystem => {
                write!(f, "row narration must disclose the owning subsystem")
            }
            Self::HiddenWithoutAccount => {
                write!(f, "a durable attention row must stay available without an account")
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a durable attention row must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl AttentionLockRecord {
    /// Builds a governed lock record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about routing, durable attention, quiet-hours coherence, OS-alert
    /// privacy, interruptibility, exact-target reopen, lifecycle semantics, the
    /// badge model, recovery, cross-surface parity, route reachability, or
    /// accessibility. The stable claim class is *derived* from the evidence, so
    /// a row can never publish a claim wider than its proof.
    pub fn build(input: AttentionLockInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_reviewable_sentence(&input.durable_job.label) {
            return Err(BuildError::InvalidSentence {
                field: "durable_job.label",
            });
        }
        if !is_reviewable_sentence(&input.durable_job.current_phase) {
            return Err(BuildError::InvalidSentence {
                field: "durable_job.current_phase",
            });
        }
        require_canonical_ref(
            "durable_job.durable_object_ref",
            &input.durable_job.durable_object_ref,
        )?;
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }

        // Upstream projection refs keep their source scheme; only presence is
        // required so support can trace the record back to the live outcome.
        require_present_ref("routing.envelope_id_ref", &input.routing.envelope_id_ref)?;
        require_present_ref(
            "routing.route_outcome_id_ref",
            &input.routing.route_outcome_id_ref,
        )?;
        require_present_ref(
            "routing.canonical_event_id",
            &input.routing.canonical_event_id,
        )?;
        require_present_ref("reopen.reopen_target_ref", &input.reopen.reopen_target_ref)?;
        require_present_ref(
            "upstream.corpus_packet_ref",
            &input.upstream.corpus_packet_ref,
        )?;
        require_present_ref("upstream.case_id_ref", &input.upstream.case_id_ref)?;

        // --- conformance: an unshippable route outcome can never be Stable ----
        if !input.route_conformance_violations.is_empty() {
            return Err(BuildError::RouteOutcomeNonConformant {
                tokens: input.route_conformance_violations.clone(),
            });
        }

        // --- lifecycle: required verbs present and distinct -------------------
        for verb in REQUIRED_LIFECYCLE_VERBS {
            if !input
                .lifecycle
                .available_actions
                .iter()
                .any(|action| action.action_kind == verb)
            {
                return Err(BuildError::MissingLifecycleVerb { verb });
            }
        }
        if !input.lifecycle.verbs_distinct {
            return Err(BuildError::LifecycleVerbsNotDistinct);
        }

        // --- badge must never outpace the durable job model ------------------
        if input.badge.active_count > 1 && !input.durable_job.durable_surface_present {
            return Err(BuildError::BadgeOutpacesDurableModel);
        }

        // --- derive the pillars from the evidence -----------------------------
        let durable_attention = input.durable_job.is_durable() && input.interruptibility.holds();
        let quiet_hours_coherent = input.quiet_hours.is_coherent();
        let privacy_safe = input.privacy.is_privacy_safe();
        let badge_truthful = input.badge.count_class_truthful();
        let reopen_deterministic = input.reopen.is_deterministic();

        // --- claim ceiling: never claim what the product cannot prove ---------
        if input.claim_ceiling.asserts_durable_attention && !durable_attention {
            return Err(BuildError::OverclaimsDurableAttention);
        }
        if input.claim_ceiling.asserts_quiet_hours_coherent && !quiet_hours_coherent {
            return Err(BuildError::OverclaimsQuietHoursCoherent);
        }
        if input.claim_ceiling.asserts_privacy_safe && !privacy_safe {
            return Err(BuildError::OverclaimsPrivacySafe);
        }
        if input.claim_ceiling.asserts_badge_count_class_truthful && !badge_truthful {
            return Err(BuildError::OverclaimsBadgeCountClass);
        }
        if input.claim_ceiling.asserts_exact_target_reopen && !reopen_deterministic {
            return Err(BuildError::OverclaimsExactTargetReopen);
        }

        // --- recovery routes -------------------------------------------------
        let required_actions = required_recovery_actions(
            input.durable_job.cancelable,
            input.durable_job.retriable,
            input.durable_job.resolvable,
        );
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_actions {
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

        // --- cross-surface parity --------------------------------------------
        if !input.surfaces.parity_holds {
            return Err(BuildError::SurfaceParityBroken);
        }
        let parity_ids: Vec<&str> = input
            .surfaces
            .recovery_action_ids
            .iter()
            .map(String::as_str)
            .collect();
        if parity_ids != route_ids {
            return Err(BuildError::SurfaceParityBroken);
        }
        for required in ["os_notification", "companion_push", "support_export"] {
            if !input
                .surfaces
                .reopen_surfaces
                .iter()
                .any(|surface| surface == required)
            {
                return Err(BuildError::ReopenSurfaceMissing { surface: required });
            }
        }

        // --- route parity across surfaces ------------------------------------
        let mut seen_surfaces = Vec::new();
        for route in &input.routes {
            if seen_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_surfaces.push(route.surface);
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
            if !seen_surfaces.contains(&required) {
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
        let subsystem_token = snake_token(&input.durable_job.actor_subsystem);
        if !subsystem_token.is_empty()
            && !input.accessibility.row_narration.contains(&subsystem_token)
        {
            return Err(BuildError::NarrationOmitsSubsystem);
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

        // --- availability: never bury a row behind account or services -------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- derive the stable-claim verdict from the evidence ---------------
        let mut narrowing_reasons = Vec::new();
        if !durable_attention {
            narrowing_reasons.push(StableNarrowingReason::DurableAttentionNotProven);
        }
        if !quiet_hours_coherent {
            narrowing_reasons.push(StableNarrowingReason::QuietHoursNotCoherent);
        }
        if !privacy_safe {
            narrowing_reasons.push(StableNarrowingReason::OsAlertNotPrivacySafe);
        }
        if !badge_truthful {
            narrowing_reasons.push(StableNarrowingReason::BadgeCountNotClassTruthful);
        }
        if !reopen_deterministic {
            narrowing_reasons.push(StableNarrowingReason::ExactTargetReopenNotDeterministic);
        }
        if input.surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(StableNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else {
            // The claim narrows to the surface's own marker when that is the
            // only gap; a missing pillar caps it at Beta.
            if narrowing_reasons.len() == 1
                && narrowing_reasons[0] == StableNarrowingReason::SurfaceNotYetStable
            {
                match input.surface_lifecycle_marker {
                    LifecycleMarker::Preview => StableClaimClass::Preview,
                    _ => StableClaimClass::Beta,
                }
            } else {
                StableClaimClass::Beta
            }
        };
        let stable_qualification = StableQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };

        let honesty_marker_present =
            !qualifies_stable || input.surface_lifecycle_marker.is_below_stable();

        Ok(Self {
            record_kind: ATTENTION_LOCK_RECORD_KIND.to_string(),
            schema_version: ATTENTION_LOCK_SCHEMA_VERSION,
            notice: ATTENTION_LOCK_NOTICE.to_string(),
            shared_contract_ref: ATTENTION_LOCK_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            attention_class: input.attention_class,
            attention_class_label: input.attention_class_label,
            surface_lifecycle_marker: input.surface_lifecycle_marker,
            title: input.title,
            summary: input.summary,
            routing: input.routing,
            durable_job: input.durable_job,
            quiet_hours: input.quiet_hours,
            privacy: input.privacy,
            interruptibility: input.interruptibility,
            reopen: input.reopen,
            lifecycle: input.lifecycle,
            badge: input.badge,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            surfaces: input.surfaces,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: input.upstream,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("attention_lock: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!(
                "attention_class: {} ({})",
                self.attention_class, self.attention_class_label
            ),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "routing: subsystem={} severity={} dedupe={} resolved_surfaces={} visible={} one_envelope={}",
                snake_token(&self.routing.source_subsystem),
                snake_token(&self.routing.severity_class),
                snake_token(&self.routing.dedupe_key_scheme),
                self.routing.resolved_surface_count,
                self.routing.visible_surface_count,
                self.routing.routes_from_one_envelope
            ),
            format!(
                "durable_job: {} actor={} cancel={} retry={} resolve={} durable_surface={} survives_lookaway={} sleep_resume={} restart={} is_durable={}",
                self.durable_job.job_id,
                snake_token(&self.durable_job.actor_subsystem),
                self.durable_job.cancelable,
                self.durable_job.retriable,
                self.durable_job.resolvable,
                self.durable_job.durable_surface_present,
                self.durable_job.survives_lookaway,
                self.durable_job.survives_sleep_resume,
                self.durable_job.survives_restart_restore,
                self.durable_job.is_durable()
            ),
            format!("durable_job.phase: {}", self.durable_job.current_phase),
            format!(
                "quiet_hours: modes=[{}] preserves_object={} preserves_reopen={} audit_trail={} coherent={} is_coherent={}",
                self.quiet_hours
                    .active_modes
                    .iter()
                    .map(snake_token)
                    .collect::<Vec<_>>()
                    .join(", "),
                self.quiet_hours.suppression_preserves_durable_object,
                self.quiet_hours.suppression_preserves_reopen_target,
                self.quiet_hours.suppression_audit_trail_present,
                self.quiet_hours.coherent_across_in_app_os_companion,
                self.quiet_hours.is_coherent()
            ),
            format!(
                "privacy: class={} payload={} redaction={} lock_screen_safe={} summary_first={} exposes_restricted={} companion_summary_only={} is_privacy_safe={}",
                snake_token(&self.privacy.privacy_class),
                snake_token(&self.privacy.privacy_payload_class),
                snake_token(&self.privacy.redaction_class),
                self.privacy.lock_screen_safe_by_default,
                self.privacy.summary_first,
                self.privacy.exposes_restricted_detail,
                self.privacy.companion_summary_only,
                self.privacy.is_privacy_safe()
            ),
            format!(
                "interruptibility: no_toast_only={} durable_surface={} coalesced={} no_spam={} holds={}",
                self.interruptibility.no_toast_only_truth,
                self.interruptibility.durable_surface_present,
                self.interruptibility.repeated_failures_coalesced_by_root_cause,
                self.interruptibility.no_badge_or_toast_spam,
                self.interruptibility.holds()
            ),
            format!(
                "reopen: kind={} exact={} placeholder={} no_generic_home={} preserves_target={} no_side_effects={} deterministic={}",
                snake_token(&self.reopen.reopen_target_kind),
                self.reopen.resolves_to_exact_target,
                self.reopen.degrades_to_truthful_placeholder,
                self.reopen.no_generic_home_reopen,
                self.reopen.all_routes_preserve_reopen_target,
                self.reopen.no_side_effects_from_notification_surface,
                self.reopen.is_deterministic()
            ),
            format!(
                "badge: class={} active={} held={} durable_history={} from_durable_state={} truthful={} label={:?}",
                self.badge.badge_class.as_str(),
                self.badge.active_count,
                self.badge.held_or_suppressed_count,
                self.badge.durable_history_preserved,
                self.badge.derived_from_durable_item_state,
                self.badge.count_class_truthful(),
                self.badge.privacy_safe_summary_label
            ),
            format!(
                "claim_ceiling: durable_attention={} quiet_hours={} privacy={} badge={} reopen={}",
                self.claim_ceiling.asserts_durable_attention,
                self.claim_ceiling.asserts_quiet_hours_coherent,
                self.claim_ceiling.asserts_privacy_safe,
                self.claim_ceiling.asserts_badge_count_class_truthful,
                self.claim_ceiling.asserts_exact_target_reopen
            ),
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
        ];
        lines.push("lifecycle_actions:".to_string());
        for action in &self.lifecycle.available_actions {
            lines.push(format!(
                "  - {} badge={} retention={} export={} mutates_source={}",
                action.action_kind.as_str(),
                action.badge_effect.as_str(),
                action.retention_effect.as_str(),
                action.export_effect.as_str(),
                action.mutates_source_object
            ));
        }
        lines.push("recovery_routes:".to_string());
        for route in &self.recovery_routes {
            lines.push(format!(
                "  - {} ({}) role={} keyboard={}",
                route.action_id,
                route.action_label,
                route.action_role.as_str(),
                route.keyboard_reachable
            ));
        }
        lines.push(format!(
            "surfaces: activity_center={} status_bar={} command={} parity_holds={} reopen=[{}]",
            self.surfaces.activity_center_row_id,
            self.surfaces.status_bar_item_id,
            self.surfaces.command_palette_command_id,
            self.surfaces.parity_holds,
            self.surfaces.reopen_surfaces.join(", ")
        ));
        lines.push("routes:".to_string());
        for route in &self.routes {
            lines.push(format!(
                "  - {} -> {} keyboard={} same_item={}",
                route.surface.as_str(),
                route.route_ref,
                route.keyboard_reachable,
                route.activates_same_item
            ));
        }
        lines.push(format!(
            "accessibility: tab_order={} tab_stops={} narration={:?}",
            self.accessibility.focus_order_index,
            self.accessibility.tab_stop_count,
            self.accessibility.row_narration
        ));
        for mode in &self.accessibility.layout_modes {
            lines.push(format!(
                "  layout {} narration={} affordances_reachable={}",
                mode.mode.as_str(),
                mode.row_narration_available,
                mode.recovery_affordances_reachable
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
            "upstream: packet={} case={} route_outcome={} envelope={}",
            self.upstream.corpus_packet_ref,
            self.upstream.case_id_ref,
            self.upstream.route_outcome_id_ref,
            self.upstream.envelope_id_ref
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}
