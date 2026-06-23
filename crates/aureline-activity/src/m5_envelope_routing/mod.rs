//! M5 *typed notification-envelope routing*: the working envelope path every M5
//! producer emits and the deterministic engine that routes it.
//!
//! Where [`m5_attention_routing`](crate::m5_attention_routing) *names and freezes
//! the contract* — the attention object families, the shared state vocabulary, the
//! controlled vocabulary, and the fanout channels — this lane *implements the
//! producer side of that contract*. Every M5 subsystem that can request attention
//! (shell command results, notebook runs, long-running tasks, AI/agent handoffs,
//! collaboration reviews, incidents, operator alerts, managed-policy changes,
//! cross-client companion status, security revocations, restore/continuity, and
//! support exports) emits the *same* typed [`NotificationEnvelope`] instead of
//! surface-local toast, banner, or badge logic, and a single pure
//! [`route_envelope`] decides which surfaces it reaches.
//!
//! Each envelope carries exactly what the spec requires as the contract: a source
//! subsystem, a scope, a privacy class, a severity, a stable dedupe key, a
//! recommended surface set, and a stable [`ActionTarget`]. Message copy is carried
//! as localizable keys ([`NotificationEnvelope::title_key`] /
//! [`NotificationEnvelope::body_key`]), never as raw bodies, so copy stays
//! revisable while the stable enums, ids, and action target are the actual
//! contract.
//!
//! Routing is a deterministic function of the envelope and a
//! [`RoutingContext`] — active window, focus mode, do-not-disturb,
//! presentation/follow mode, screen-reader posture, collaboration role, and the
//! user and admin notification policy. [`route_envelope`] returns a
//! [`RoutingDecision`] with one [`SurfaceRouteOutcome`] per handled surface, so a
//! routing decision is reproducible byte-for-byte in support export and CLI/headless
//! diagnostics. The honesty rules the track invariant requires are enforced, not
//! just described:
//!
//! - **No toast-only truth.** Every decision delivers the in-app activity center as
//!   a durable record; deferring or suppressing an out-of-window surface never
//!   drops it (`envelope.durable_record_always`,
//!   `envelope.suppression_separate_from_durable`).
//! - **One source envelope and action target on every surface.** The in-app
//!   activity center, OS notification, companion, and badge outcomes in a decision
//!   all reference the same stable action target
//!   (`envelope.stable_action_target_shared`, `envelope.consumer_parity`).
//! - **Fanout cannot bypass preview/approval.** When an envelope's action routes
//!   through the in-product preview/approval flow, no out-of-window surface
//!   executes it inline; they all hand off to the in-product surface
//!   (`envelope.fanout_cannot_bypass_preview_approval`).
//! - **Privacy never widens on fanout.** Every out-of-window outcome applies a
//!   redaction at least as strong as the envelope default and the channel's privacy
//!   ceiling (`envelope.privacy_never_widens_on_fanout`).
//!
//! The canonical [`envelope_routing_bundle`] freezes the producer registry, the
//! envelope corpus, the representative routing contexts, and every routing decision
//! so the freeze gate and checked-in fixture pin the contract byte-for-byte. Each
//! channel profile and every severity, scope, privacy class, dedupe rule, channel,
//! and reopen target the bundle uses is one the attention-routing matrix defines,
//! so the producer path can never drift from the frozen object model
//! (`envelope.matrix_bound`).
//!
//! The record carries no message bodies, credentials, raw provider payloads,
//! hostnames, or absolute paths — only opaque object refs, localizable copy keys,
//! stable tokens, and short reviewable sentences — so it is safe to embed in a
//! support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_attention_routing::{
    all_unique, attention_routing_matrix, is_export_safe_ref, AttentionRedactionClass,
    AttentionScopeClass, FanoutChannelClass, NotificationPrivacyClass, ReopenTargetClass,
    M5_ATTENTION_ROUTING_MATRIX_ID,
};

#[cfg(test)]
mod tests;

/// Schema version for the envelope-routing bundle.
pub const M5_ENVELOPE_ROUTING_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the envelope-routing bundle.
pub const M5_ENVELOPE_ROUTING_SCHEMA_REF: &str = "schemas/activity/m5-envelope-routing.schema.json";

/// Stable record-kind tag for the envelope-routing bundle.
pub const M5_ENVELOPE_ROUTING_RECORD_KIND: &str = "m5_envelope_routing_bundle";

/// Stable id for the canonical envelope-routing bundle.
pub const M5_ENVELOPE_ROUTING_BUNDLE_ID: &str = "m5-envelope-routing:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ENVELOPE_ROUTING_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The attention-routing matrix fixture this lane binds its vocabulary back to.
pub const M5_ENVELOPE_ROUTING_MATRIX_REF: &str =
    "fixtures/activity/m5-attention-routing/canonical_matrix.json";

/// The freeze gate that keeps the bundle current. Stable promotion runs this gate;
/// it fails when the in-code bundle drifts from the checked-in fixture or any
/// invariant flips.
pub const M5_ENVELOPE_ROUTING_FREEZE_GATE_REF: &str =
    "crates/aureline-activity/tests/m5_envelope_routing.rs";

// ---------------------------------------------------------------------------
// Producer subsystems.
// ---------------------------------------------------------------------------

/// The closed set of M5 source subsystems that emit notification envelopes.
///
/// Every claimed M5 producer maps to exactly one of these. Adding one is a
/// breaking change to the producer registry; the tokens are frozen here so a
/// consumer can resolve a producer's subsystem by a stable token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceSubsystemClass {
    /// The shell: command results, undo, and surface-level confirmations.
    Shell,
    /// Notebooks: cell and notebook execution outcomes.
    Notebook,
    /// The task runner: long-running jobs, CI, and batch work.
    TaskRunner,
    /// AI / agent / composer: handoffs awaiting review or approval.
    Ai,
    /// Collaboration: shared review requests and session handoffs.
    Collaboration,
    /// Incidents: new and updated incident threads.
    Incident,
    /// The operator / admin plane: fleet alerts and managed routing.
    Operator,
    /// The managed-policy plane: policy, bundle, and entitlement changes.
    ManagedPolicy,
    /// The cross-client companion: fanout and continuation status.
    Companion,
    /// Security: credential revocation and trust advisories.
    Security,
    /// Continuity: backup, restore, and failover outcomes.
    Sync,
    /// Support: export-ready and diagnostic-bundle notices.
    Support,
}

impl SourceSubsystemClass {
    /// All subsystems, in registry order.
    pub const ALL: [Self; 12] = [
        Self::Shell,
        Self::Notebook,
        Self::TaskRunner,
        Self::Ai,
        Self::Collaboration,
        Self::Incident,
        Self::Operator,
        Self::ManagedPolicy,
        Self::Companion,
        Self::Security,
        Self::Sync,
        Self::Support,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Notebook => "notebook",
            Self::TaskRunner => "task_runner",
            Self::Ai => "ai",
            Self::Collaboration => "collaboration",
            Self::Incident => "incident",
            Self::Operator => "operator",
            Self::ManagedPolicy => "managed_policy",
            Self::Companion => "companion",
            Self::Security => "security",
            Self::Sync => "sync",
            Self::Support => "support",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Shell => "Shell",
            Self::Notebook => "Notebook",
            Self::TaskRunner => "Task runner",
            Self::Ai => "AI / agent",
            Self::Collaboration => "Collaboration",
            Self::Incident => "Incident",
            Self::Operator => "Operator plane",
            Self::ManagedPolicy => "Managed policy",
            Self::Companion => "Companion",
            Self::Security => "Security",
            Self::Sync => "Continuity",
            Self::Support => "Support",
        }
    }
}

// ---------------------------------------------------------------------------
// Severity.
// ---------------------------------------------------------------------------

/// The severity class of an attention event.
///
/// The tokens are exactly the `severity` controlled vocabulary the
/// attention-routing matrix freezes; `envelope.matrix_bound` proves the binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationSeverityClass {
    /// A minor success or an undo affordance.
    MinorSuccess,
    /// An informational notice.
    Informational,
    /// A long-running progress update.
    Progress,
    /// Workspace degradation that needs eventual attention.
    Degraded,
    /// A review, incident, or collaboration handoff that needs a person.
    HandoffActionable,
    /// A security advisory or revocation.
    SecurityAdvisory,
}

impl NotificationSeverityClass {
    /// All severities, in ascending interruptiveness order.
    pub const ALL: [Self; 6] = [
        Self::MinorSuccess,
        Self::Informational,
        Self::Progress,
        Self::Degraded,
        Self::HandoffActionable,
        Self::SecurityAdvisory,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinorSuccess => "minor_success",
            Self::Informational => "informational",
            Self::Progress => "progress",
            Self::Degraded => "degraded",
            Self::HandoffActionable => "handoff_actionable",
            Self::SecurityAdvisory => "security_advisory",
        }
    }

    /// Interruptiveness rank; higher is more urgent.
    const fn rank(self) -> u8 {
        match self {
            Self::MinorSuccess => 1,
            Self::Informational => 2,
            Self::Progress => 3,
            Self::Degraded => 4,
            Self::HandoffActionable => 5,
            Self::SecurityAdvisory => 6,
        }
    }

    /// Whether this severity is important enough to break an `important_only` user
    /// filter (a review/incident/collaboration handoff or a security advisory).
    pub const fn is_important(self) -> bool {
        self.rank() >= Self::HandoffActionable.rank()
    }

    /// Whether this severity is a security advisory that may bypass quiet-hours,
    /// focus, and mute with a redacted summary.
    pub const fn is_security(self) -> bool {
        matches!(self, Self::SecurityAdvisory)
    }
}

// ---------------------------------------------------------------------------
// Dedupe strategy.
// ---------------------------------------------------------------------------

/// How repeated events coalesce into one canonical envelope.
///
/// The tokens are exactly the `dedupe_rule` controlled vocabulary the
/// attention-routing matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DedupeStrategyClass {
    /// Coalesce repeats by a canonical key.
    CanonicalKeyCoalesce,
    /// Collapse repeats that share one root cause.
    RootCauseCollapse,
    /// The latest event supersedes prior ones.
    LatestSupersedes,
    /// Roll repeats up into a single count.
    CountRollup,
    /// No dedupe; each event is distinct.
    NoDedupe,
}

impl DedupeStrategyClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalKeyCoalesce => "canonical_key_coalesce",
            Self::RootCauseCollapse => "root_cause_collapse",
            Self::LatestSupersedes => "latest_supersedes",
            Self::CountRollup => "count_rollup",
            Self::NoDedupe => "no_dedupe",
        }
    }
}

// ---------------------------------------------------------------------------
// Action target.
// ---------------------------------------------------------------------------

/// The primary action a notification offers, as a stable verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionVerbClass {
    /// Open / reopen the authoritative object.
    Open,
    /// Review and approve a change through the preview/approval flow.
    ReviewApprove,
    /// Retry a failed durable job.
    Retry,
    /// Acknowledge — mark read, keep the durable record.
    Acknowledge,
    /// Resolve — close on the underlying change.
    Resolve,
}

impl ActionVerbClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::ReviewApprove => "review_approve",
            Self::Retry => "retry",
            Self::Acknowledge => "acknowledge",
            Self::Resolve => "resolve",
        }
    }
}

/// The stable action target every surface uses to act on an envelope.
///
/// The target is a stable contract: a stable id, a verb, the authoritative object
/// it reopens, an opaque object ref, and a localizable label key. It never encodes
/// a blind side effect — it always names a [`reopen_target`](Self::reopen_target).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionTarget {
    /// Stable, namespaced action-target id, identical on every surface.
    pub action_target_id: String,
    /// The primary action verb.
    pub primary_action: ActionVerbClass,
    /// The authoritative object this action reopens.
    pub reopen_target: ReopenTargetClass,
    /// The opaque object ref the action resolves to (never a URL, host, or path).
    pub object_ref: String,
    /// The localizable label key for the action (never raw copy).
    pub label_key: String,
    /// Whether completing this action must route through the in-product
    /// preview/approval flow. When true, no out-of-window surface may execute it
    /// inline; they hand off to the in-product surface.
    pub routes_through_preview_approval: bool,
}

// ---------------------------------------------------------------------------
// Notification envelope.
// ---------------------------------------------------------------------------

/// The typed, privacy-aware unit of attention every M5 producer emits.
///
/// This is the working record behind the
/// [`NotificationEnvelope`](crate::m5_attention_routing::AttentionObjectClass::NotificationEnvelope)
/// object family the matrix names. It carries the contract fields the spec
/// requires — source subsystem, scope, privacy class, severity, dedupe key,
/// recommended surface set, and a stable action target — plus localizable copy
/// keys instead of raw bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationEnvelope {
    /// Stable, namespaced envelope id.
    pub notification_envelope_id: String,
    /// The canonical event id this envelope was created from.
    pub canonical_event_id: String,
    /// The source subsystem that emitted the envelope.
    pub source_subsystem: SourceSubsystemClass,
    /// The stable producer id that emitted the envelope.
    pub producer_id: String,
    /// The severity class.
    pub severity: NotificationSeverityClass,
    /// The scope namespace the attention applies to.
    pub scope: AttentionScopeClass,
    /// The opaque scope object ref.
    pub scope_ref: String,
    /// The privacy class governing what may be shown, mirrored, or exported.
    pub privacy_class: NotificationPrivacyClass,
    /// How repeated events coalesce.
    pub dedupe_strategy: DedupeStrategyClass,
    /// The stable, metadata-safe dedupe key.
    pub dedupe_key: String,
    /// The recommended surface set, in canonical channel order; always includes the
    /// in-app activity center.
    pub recommended_surfaces: Vec<FanoutChannelClass>,
    /// The stable action target every surface uses.
    pub action_target: ActionTarget,
    /// The authoritative objects a surface may reopen.
    pub reopen_targets: Vec<ReopenTargetClass>,
    /// The localizable title key (never raw copy).
    pub title_key: String,
    /// The localizable body key (never raw copy).
    pub body_key: String,
    /// The default redaction posture on out-of-window surfaces and export.
    pub default_redaction: AttentionRedactionClass,
    /// Whether the envelope is backed by a durable authoritative record (always
    /// true — never toast-only).
    pub carries_durable_record: bool,
    /// Whether user-facing copy is carried as localizable keys rather than raw
    /// bodies (always true — copy stays revisable, the contract is the enums/ids).
    pub carries_localizable_copy: bool,
    /// Evaluation stamp.
    pub created_at: String,
}

impl NotificationEnvelope {
    /// Whether this envelope recommends a surface.
    pub fn recommends(&self, surface: FanoutChannelClass) -> bool {
        self.recommended_surfaces.contains(&surface)
    }
}

// ---------------------------------------------------------------------------
// Routing context.
// ---------------------------------------------------------------------------

/// Where the active app window stands relative to the user's focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveWindowClass {
    /// The app is foreground and focused.
    AppForegroundFocused,
    /// The app is foreground but not the focused window.
    AppForegroundUnfocused,
    /// The app is in the background.
    AppBackground,
    /// The app is minimized or hidden.
    AppMinimizedHidden,
    /// The screen is locked.
    ScreenLocked,
}

impl ActiveWindowClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppForegroundFocused => "app_foreground_focused",
            Self::AppForegroundUnfocused => "app_foreground_unfocused",
            Self::AppBackground => "app_background",
            Self::AppMinimizedHidden => "app_minimized_hidden",
            Self::ScreenLocked => "screen_locked",
        }
    }
}

/// The OS / app focus-mode posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusModeClass {
    /// No focus mode active.
    Off,
    /// Focus assist / focus mode is on.
    FocusAssistOn,
}

impl FocusModeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::FocusAssistOn => "focus_assist_on",
        }
    }
}

/// The do-not-disturb posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoNotDisturbClass {
    /// Do-not-disturb is off.
    Off,
    /// Do-not-disturb is on.
    On,
}

impl DoNotDisturbClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
        }
    }
}

/// The presentation / follow-mode posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationModeClass {
    /// Not presenting.
    Off,
    /// Actively presenting / screen-sharing.
    Presenting,
    /// Following another participant's view.
    FollowMode,
}

impl PresentationModeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Presenting => "presenting",
            Self::FollowMode => "follow_mode",
        }
    }
}

/// The screen-reader posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenReaderPostureClass {
    /// No screen reader active.
    Off,
    /// A screen reader is active.
    On,
}

impl ScreenReaderPostureClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::On => "on",
        }
    }
}

/// The collaboration role of the current user in the active scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationRoleClass {
    /// Working solo.
    Solo,
    /// The owner / admin of the scope.
    Owner,
    /// A reviewer in a shared review.
    Reviewer,
    /// A read-only viewer.
    Viewer,
    /// A guest with limited access.
    Guest,
}

impl CollaborationRoleClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Solo => "solo",
            Self::Owner => "owner",
            Self::Reviewer => "reviewer",
            Self::Viewer => "viewer",
            Self::Guest => "guest",
        }
    }

    /// Whether this role only sees collaboration handoffs in-product (viewers and
    /// guests do not receive cross-client fanout for collaboration-scoped events).
    const fn is_limited(self) -> bool {
        matches!(self, Self::Viewer | Self::Guest)
    }
}

/// The user's notification policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserNotificationPolicyClass {
    /// All notifications allowed.
    AllAllowed,
    /// Only important notifications break through.
    ImportantOnly,
    /// All out-of-window notifications muted.
    Muted,
}

impl UserNotificationPolicyClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllAllowed => "all_allowed",
            Self::ImportantOnly => "important_only",
            Self::Muted => "muted",
        }
    }
}

/// The admin / managed notification policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminNotificationPolicyClass {
    /// Unmanaged install.
    Unmanaged,
    /// Managed with default notification policy.
    ManagedDefault,
    /// Managed with restricted (raised-redaction) fanout.
    ManagedRestricted,
    /// Managed with locked cross-client fanout.
    ManagedLocked,
}

impl AdminNotificationPolicyClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unmanaged => "unmanaged",
            Self::ManagedDefault => "managed_default",
            Self::ManagedRestricted => "managed_restricted",
            Self::ManagedLocked => "managed_locked",
        }
    }
}

/// The routing context an envelope is routed against.
///
/// Carries every routing input the spec requires: active window, focus mode,
/// do-not-disturb, presentation/follow mode, screen-reader posture, collaboration
/// role, and the user and admin notification policy, plus whether quiet-hours is
/// active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContext {
    /// Stable, namespaced context id.
    pub context_id: String,
    /// One reviewable sentence describing the context.
    pub summary: String,
    /// The active-window / focus state.
    pub active_window: ActiveWindowClass,
    /// The focus-mode posture.
    pub focus_mode: FocusModeClass,
    /// The do-not-disturb posture.
    pub do_not_disturb: DoNotDisturbClass,
    /// The presentation / follow-mode posture.
    pub presentation_mode: PresentationModeClass,
    /// The screen-reader posture.
    pub screen_reader: ScreenReaderPostureClass,
    /// The collaboration role.
    pub collaboration_role: CollaborationRoleClass,
    /// The user notification policy.
    pub user_policy: UserNotificationPolicyClass,
    /// The admin / managed notification policy.
    pub admin_policy: AdminNotificationPolicyClass,
    /// Whether quiet-hours is currently active.
    pub quiet_hours_active: bool,
}

impl RoutingContext {
    /// Whether any focus posture (focus mode, do-not-disturb, or presentation /
    /// follow) is suppressing out-of-window interruptions.
    fn focus_suppresses(&self) -> bool {
        self.focus_mode == FocusModeClass::FocusAssistOn
            || self.do_not_disturb == DoNotDisturbClass::On
            || matches!(
                self.presentation_mode,
                PresentationModeClass::Presenting | PresentationModeClass::FollowMode
            )
    }
}

// ---------------------------------------------------------------------------
// Routing decision.
// ---------------------------------------------------------------------------

/// The disposition of one envelope on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteDispositionClass {
    /// Delivered to this surface now.
    Deliver,
    /// Delivered with a raised redaction (privacy ceiling or policy).
    DeliverRedacted,
    /// Deferred by quiet-hours.
    DeferQuietHours,
    /// Deferred by focus mode, do-not-disturb, or presentation / follow.
    DeferFocus,
    /// Routed to the in-product surface instead of this out-of-window surface.
    RouteToInProduct,
    /// Suppressed by the user's notification policy.
    SuppressedByUserPolicy,
    /// Suppressed by the admin / managed notification policy.
    SuppressedByAdminPolicy,
}

impl RouteDispositionClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deliver => "deliver",
            Self::DeliverRedacted => "deliver_redacted",
            Self::DeferQuietHours => "defer_quiet_hours",
            Self::DeferFocus => "defer_focus",
            Self::RouteToInProduct => "route_to_in_product",
            Self::SuppressedByUserPolicy => "suppressed_by_user_policy",
            Self::SuppressedByAdminPolicy => "suppressed_by_admin_policy",
        }
    }

    /// Whether the surface was delivered (possibly redacted).
    pub const fn is_delivered(self) -> bool {
        matches!(self, Self::Deliver | Self::DeliverRedacted)
    }

    /// Whether the surface was deferred or routed back to the in-product surface.
    pub const fn is_deferred(self) -> bool {
        matches!(
            self,
            Self::DeferQuietHours | Self::DeferFocus | Self::RouteToInProduct
        )
    }

    /// Whether the surface was suppressed by user or admin policy.
    pub const fn is_suppressed(self) -> bool {
        matches!(
            self,
            Self::SuppressedByUserPolicy | Self::SuppressedByAdminPolicy
        )
    }
}

/// The outcome of routing one envelope to one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRouteOutcome {
    /// The fanout surface.
    pub surface: FanoutChannelClass,
    /// Stable, namespaced surface id.
    pub surface_id: String,
    /// The disposition on this surface.
    pub disposition: RouteDispositionClass,
    /// Stable reason token.
    pub reason_token: String,
    /// One reviewable sentence explaining the disposition.
    pub reason: String,
    /// The redaction actually applied on this surface.
    pub applied_redaction: AttentionRedactionClass,
    /// The stable action target id (identical to the envelope's action target).
    pub action_target_id: String,
    /// Whether this surface holds the durable authoritative record (the in-app
    /// activity center only).
    pub delivers_durable_record: bool,
    /// Whether this surface mirrors the authoritative object rather than being it.
    pub mirrors_authoritative: bool,
    /// Whether a preview/approval-gated action hands off to the in-product surface
    /// here instead of executing inline (always true for out-of-window surfaces when
    /// the action routes through preview/approval).
    pub dangerous_action_handoff_to_in_product: bool,
    /// Whether this surface must render an accessible (non-visual) affordance for
    /// the active screen-reader posture.
    pub requires_accessible_affordance: bool,
}

/// The full routing decision for one envelope against one context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingDecision {
    /// Stable, namespaced decision id.
    pub decision_id: String,
    /// The envelope id routed.
    pub envelope_id: String,
    /// The context id routed against.
    pub context_id: String,
    /// The source subsystem of the routed envelope.
    pub source_subsystem: SourceSubsystemClass,
    /// The stable action target id shared by every surface in this decision.
    pub action_target_id: String,
    /// Whether a durable in-product record is present regardless of fanout (always
    /// true — never toast-only).
    pub durable_record_present: bool,
    /// The per-surface outcomes, one per handled surface, in canonical order.
    pub outcomes: Vec<SurfaceRouteOutcome>,
}

impl RoutingDecision {
    /// The outcome for a surface, if handled.
    pub fn outcome(&self, surface: FanoutChannelClass) -> Option<&SurfaceRouteOutcome> {
        self.outcomes.iter().find(|o| o.surface == surface)
    }

    /// The surfaces this decision delivered to (possibly redacted).
    pub fn delivered_surfaces(&self) -> Vec<FanoutChannelClass> {
        self.outcomes
            .iter()
            .filter(|o| o.disposition.is_delivered())
            .map(|o| o.surface)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Channel routing profiles (mirror the matrix; bound by an invariant).
// ---------------------------------------------------------------------------

/// The routing-relevant facts about a fanout channel, mirrored from the matrix.
struct ChannelRoutingProfile {
    privacy_ceiling: NotificationPrivacyClass,
    default_redaction: AttentionRedactionClass,
    mirrors_authoritative: bool,
}

fn channel_profile(channel: FanoutChannelClass) -> ChannelRoutingProfile {
    use AttentionRedactionClass::*;
    use FanoutChannelClass::*;
    use NotificationPrivacyClass::*;
    match channel {
        InAppActivityCenter => ChannelRoutingProfile {
            privacy_ceiling: ManagedSensitive,
            default_redaction: MetadataSafeDefault,
            mirrors_authoritative: false,
        },
        OsNativeNotification => ChannelRoutingProfile {
            privacy_ceiling: SummarySafe,
            default_redaction: SummaryOnly,
            mirrors_authoritative: true,
        },
        DockTaskbarBadge => ChannelRoutingProfile {
            privacy_ceiling: SummarySafe,
            default_redaction: CountOnly,
            mirrors_authoritative: true,
        },
        BrowserCompanion => ChannelRoutingProfile {
            privacy_ceiling: WorkspaceSensitive,
            default_redaction: RedactedPayload,
            mirrors_authoritative: true,
        },
        MobileCompanion => ChannelRoutingProfile {
            privacy_ceiling: SummarySafe,
            default_redaction: SummaryOnly,
            mirrors_authoritative: true,
        },
        OperatorDashboard => ChannelRoutingProfile {
            privacy_ceiling: ManagedSensitive,
            default_redaction: InternalSupportRestricted,
            mirrors_authoritative: true,
        },
    }
}

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

/// The token for a redaction class (the matrix enum carries no `as_str`).
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

fn privacy_rank(p: NotificationPrivacyClass) -> u8 {
    use NotificationPrivacyClass::*;
    match p {
        SummarySafe => 1,
        WorkspaceSensitive => 2,
        SecurityCritical => 3,
        ManagedSensitive => 4,
    }
}

// ---------------------------------------------------------------------------
// The routing engine.
// ---------------------------------------------------------------------------

/// Routes one envelope against one context, deterministically.
///
/// Pure: the same `(envelope, context)` yields the same [`RoutingDecision`] every
/// call, so routing decisions are reproducible in support export and CLI/headless
/// diagnostics. The in-app activity center is always delivered as a durable record;
/// out-of-window surfaces are gated by admin policy, user policy, collaboration
/// role, quiet-hours, and focus, with security advisories breaking through with a
/// redacted summary. Privacy never widens on fanout, and a preview/approval-gated
/// action never executes inline on an out-of-window surface.
pub fn route_envelope(
    envelope: &NotificationEnvelope,
    context: &RoutingContext,
) -> RoutingDecision {
    let mut outcomes = Vec::new();
    let accessible = context.screen_reader == ScreenReaderPostureClass::On;

    for channel in FanoutChannelClass::ALL {
        let recommended =
            channel == FanoutChannelClass::InAppActivityCenter || envelope.recommends(channel);
        if !recommended {
            continue;
        }
        let outcome = if channel == FanoutChannelClass::InAppActivityCenter {
            in_app_outcome(envelope, accessible)
        } else {
            out_of_window_outcome(envelope, context, channel, accessible)
        };
        outcomes.push(outcome);
    }

    let durable_record_present = outcomes.iter().any(|o| {
        o.surface == FanoutChannelClass::InAppActivityCenter
            && o.disposition == RouteDispositionClass::Deliver
            && o.delivers_durable_record
    });

    RoutingDecision {
        decision_id: format!(
            "m5-envelope-routing:decision:{}:{}",
            envelope.notification_envelope_id, context.context_id
        ),
        envelope_id: envelope.notification_envelope_id.clone(),
        context_id: context.context_id.clone(),
        source_subsystem: envelope.source_subsystem,
        action_target_id: envelope.action_target.action_target_id.clone(),
        durable_record_present,
        outcomes,
    }
}

fn in_app_outcome(envelope: &NotificationEnvelope, accessible: bool) -> SurfaceRouteOutcome {
    SurfaceRouteOutcome {
        surface: FanoutChannelClass::InAppActivityCenter,
        surface_id: FanoutChannelClass::InAppActivityCenter.channel_id(),
        disposition: RouteDispositionClass::Deliver,
        reason_token: "durable_authoritative_record".to_owned(),
        reason: "The in-app activity center holds the durable authoritative record and is always \
                 delivered in-product, independent of fanout, focus, or quiet-hours."
            .to_owned(),
        applied_redaction: envelope.default_redaction,
        action_target_id: envelope.action_target.action_target_id.clone(),
        delivers_durable_record: true,
        mirrors_authoritative: false,
        // The preview/approval flow lives in-product, so it is never a bypass.
        dangerous_action_handoff_to_in_product: false,
        requires_accessible_affordance: accessible,
    }
}

fn out_of_window_outcome(
    envelope: &NotificationEnvelope,
    context: &RoutingContext,
    channel: FanoutChannelClass,
    accessible: bool,
) -> SurfaceRouteOutcome {
    let profile = channel_profile(channel);

    // Privacy never widens on fanout: apply the stronger of the envelope default
    // and the channel default, and raise to a redacted payload when the envelope's
    // privacy class exceeds the channel's ceiling.
    let ceiling_exceeded =
        privacy_rank(envelope.privacy_class) > privacy_rank(profile.privacy_ceiling);
    let mut applied = stronger_redaction(envelope.default_redaction, profile.default_redaction);
    if ceiling_exceeded {
        applied = stronger_redaction(applied, AttentionRedactionClass::RedactedPayload);
    }

    let (disposition, reason_token, reason) =
        out_of_window_disposition(envelope, context, channel, applied, ceiling_exceeded);

    SurfaceRouteOutcome {
        surface: channel,
        surface_id: channel.channel_id(),
        disposition,
        reason_token: reason_token.to_owned(),
        reason,
        applied_redaction: applied,
        action_target_id: envelope.action_target.action_target_id.clone(),
        delivers_durable_record: false,
        mirrors_authoritative: profile.mirrors_authoritative,
        dangerous_action_handoff_to_in_product: envelope
            .action_target
            .routes_through_preview_approval,
        requires_accessible_affordance: accessible,
    }
}

fn deliver_disposition(
    envelope: &NotificationEnvelope,
    channel: FanoutChannelClass,
    applied: AttentionRedactionClass,
    ceiling_exceeded: bool,
) -> (RouteDispositionClass, &'static str, String) {
    let raised = redaction_rank(applied) > redaction_rank(envelope.default_redaction);
    if ceiling_exceeded || raised {
        (
            RouteDispositionClass::DeliverRedacted,
            "redacted_to_channel_ceiling",
            format!(
                "Delivered to the {} with a raised redaction ({}) so the envelope's privacy class \
                 is never widened on fanout.",
                channel.label(),
                redaction_token(applied)
            ),
        )
    } else {
        (
            RouteDispositionClass::Deliver,
            "delivered",
            format!("Delivered to the {}.", channel.label()),
        )
    }
}

fn out_of_window_disposition(
    envelope: &NotificationEnvelope,
    context: &RoutingContext,
    channel: FanoutChannelClass,
    applied: AttentionRedactionClass,
    ceiling_exceeded: bool,
) -> (RouteDispositionClass, &'static str, String) {
    // The operator dashboard is a separate read-only admin surface: it renders
    // managed attention truth independent of one user's quiet-hours, focus, or
    // mute, and is governed only by admin policy and recommended-surface membership.
    if channel == FanoutChannelClass::OperatorDashboard {
        return deliver_disposition(envelope, channel, applied, ceiling_exceeded);
    }

    // Admin policy can lock cross-client companion fanout entirely.
    if context.admin_policy == AdminNotificationPolicyClass::ManagedLocked
        && matches!(
            channel,
            FanoutChannelClass::BrowserCompanion | FanoutChannelClass::MobileCompanion
        )
    {
        return (
            RouteDispositionClass::SuppressedByAdminPolicy,
            "admin_locked_cross_client",
            "Admin policy locks cross-client companion fanout; the attention stays in-product."
                .to_owned(),
        );
    }

    // Security advisories break through quiet-hours, focus, and mute with a
    // redacted summary.
    if envelope.severity.is_security() {
        return (
            RouteDispositionClass::DeliverRedacted,
            "security_advisory_override",
            format!(
                "A security advisory breaks through to the {} with a redacted summary ({}); the \
                 full payload stays in-product.",
                channel.label(),
                redaction_token(applied)
            ),
        );
    }

    // A limited collaboration role keeps collaboration-scoped handoffs in-product.
    if context.collaboration_role.is_limited()
        && envelope.scope == AttentionScopeClass::Collaboration
    {
        return (
            RouteDispositionClass::RouteToInProduct,
            "limited_collaboration_role",
            "A limited collaboration role receives collaboration handoffs in-product rather than as \
             cross-client fanout."
                .to_owned(),
        );
    }

    // User mute suppresses out-of-window fanout.
    if context.user_policy == UserNotificationPolicyClass::Muted {
        return (
            RouteDispositionClass::SuppressedByUserPolicy,
            "user_muted",
            "The user muted out-of-window notifications; the durable in-product record still holds \
             the attention."
                .to_owned(),
        );
    }

    // Important-only keeps non-important fanout in-product.
    if context.user_policy == UserNotificationPolicyClass::ImportantOnly
        && !envelope.severity.is_important()
    {
        return (
            RouteDispositionClass::RouteToInProduct,
            "important_only_filter",
            "The user's important-only policy keeps this non-important attention in-product."
                .to_owned(),
        );
    }

    // A focused app makes a redundant OS notification noise; show it in-product.
    if context.active_window == ActiveWindowClass::AppForegroundFocused
        && channel == FanoutChannelClass::OsNativeNotification
        && !envelope.severity.is_important()
    {
        return (
            RouteDispositionClass::RouteToInProduct,
            "app_focused",
            "The app is focused, so the attention is shown in-product instead of a redundant OS \
             notification."
                .to_owned(),
        );
    }

    // Quiet-hours defers out-of-window fanout.
    if context.quiet_hours_active {
        return (
            RouteDispositionClass::DeferQuietHours,
            "quiet_hours_deferred",
            "Quiet-hours defers this out-of-window fanout; it returns when quiet-hours ends and the \
             durable record is unchanged."
                .to_owned(),
        );
    }

    // Focus mode, do-not-disturb, or presentation / follow defers fanout.
    if context.focus_suppresses() {
        return (
            RouteDispositionClass::DeferFocus,
            "focus_deferred",
            "Focus mode, do-not-disturb, or presentation / follow defers this out-of-window fanout; \
             the durable record is unchanged."
                .to_owned(),
        );
    }

    deliver_disposition(envelope, channel, applied, ceiling_exceeded)
}

// ---------------------------------------------------------------------------
// Producer registry and bundle record.
// ---------------------------------------------------------------------------

/// One M5 producer entry: a subsystem that emits the typed envelope path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerEntry {
    /// Stable, namespaced producer id.
    pub producer_id: String,
    /// The source subsystem.
    pub source_subsystem: SourceSubsystemClass,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing what the producer emits.
    pub summary: String,
    /// The crate module(s) that emit this producer's envelopes.
    pub produced_by_refs: Vec<String>,
    /// The envelope id this producer emits in the canonical corpus.
    pub emits_envelope_id: String,
    /// Whether the producer routes through the typed envelope path (always true).
    pub routes_through_typed_envelope: bool,
    /// Whether the producer retains surface-local toast/banner/badge logic instead
    /// of the typed path (always false — the spec forbids it).
    pub retains_surface_local_logic: bool,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeRoutingInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen envelope-routing bundle: the producer registry, the envelope corpus,
/// the representative routing contexts, and every routing decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeRoutingBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_envelope_routing_schema_version: u32,
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
    /// The producer registry.
    pub producers: Vec<ProducerEntry>,
    /// The canonical envelope corpus.
    pub envelopes: Vec<NotificationEnvelope>,
    /// The representative routing contexts.
    pub contexts: Vec<RoutingContext>,
    /// Every routing decision (each envelope routed against each context).
    pub decisions: Vec<RoutingDecision>,
    /// The computed invariants.
    pub invariants: Vec<EnvelopeRoutingInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvelopeRoutingValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for EnvelopeRoutingValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "envelope-routing bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for EnvelopeRoutingValidationError {}

impl EnvelopeRoutingBundle {
    /// The envelope with a given id, if present.
    pub fn envelope(&self, envelope_id: &str) -> Option<&NotificationEnvelope> {
        self.envelopes
            .iter()
            .find(|e| e.notification_envelope_id == envelope_id)
    }

    /// The producer with a given id, if present.
    pub fn producer(&self, producer_id: &str) -> Option<&ProducerEntry> {
        self.producers.iter().find(|p| p.producer_id == producer_id)
    }

    /// The routing context with a given id, if present.
    pub fn context(&self, context_id: &str) -> Option<&RoutingContext> {
        self.contexts.iter().find(|c| c.context_id == context_id)
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
        let from_producers = self
            .producers
            .iter()
            .flat_map(|p| p.produced_by_refs.iter().map(String::as_str));
        let from_envelopes = self.envelopes.iter().flat_map(|e| {
            std::iter::once(e.scope_ref.as_str())
                .chain(std::iter::once(e.action_target.object_ref.as_str()))
        });
        fixed.chain(from_producers).chain(from_envelopes)
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), EnvelopeRoutingValidationError> {
        let fail = |reason: String| Err(EnvelopeRoutingValidationError { reason });

        if self.record_kind != M5_ENVELOPE_ROUTING_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ENVELOPE_ROUTING_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.producers.is_empty() || self.envelopes.is_empty() || self.contexts.is_empty() {
            return fail("producers, envelopes, and contexts must be non-empty".to_owned());
        }

        // Stable ids are unique.
        if !all_unique(self.producers.iter().map(|p| p.producer_id.as_str())) {
            return fail("producer ids are not unique".to_owned());
        }
        if !all_unique(
            self.envelopes
                .iter()
                .map(|e| e.notification_envelope_id.as_str()),
        ) {
            return fail("envelope ids are not unique".to_owned());
        }
        if !all_unique(self.contexts.iter().map(|c| c.context_id.as_str())) {
            return fail("context ids are not unique".to_owned());
        }
        if !all_unique(self.decisions.iter().map(|d| d.decision_id.as_str())) {
            return fail("decision ids are not unique".to_owned());
        }

        // Every subsystem has at least one producer.
        for subsystem in SourceSubsystemClass::ALL {
            if !self
                .producers
                .iter()
                .any(|p| p.source_subsystem == subsystem)
            {
                return fail(format!("subsystem {} has no producer", subsystem.as_str()));
            }
        }

        // Every producer resolves to a corpus envelope and routes the typed path.
        for producer in &self.producers {
            if self.envelope(&producer.emits_envelope_id).is_none() {
                return fail(format!(
                    "producer {} emits unknown envelope {}",
                    producer.producer_id, producer.emits_envelope_id
                ));
            }
            if !producer.routes_through_typed_envelope || producer.retains_surface_local_logic {
                return fail(format!(
                    "producer {} does not route the typed envelope path",
                    producer.producer_id
                ));
            }
        }

        // Every envelope carries the required contract fields.
        for envelope in &self.envelopes {
            if envelope.recommended_surfaces.is_empty() {
                return fail(format!(
                    "envelope {} recommends no surface",
                    envelope.notification_envelope_id
                ));
            }
            if !envelope.recommends(FanoutChannelClass::InAppActivityCenter) {
                return fail(format!(
                    "envelope {} omits the in-app activity center",
                    envelope.notification_envelope_id
                ));
            }
            if envelope.reopen_targets.is_empty() {
                return fail(format!(
                    "envelope {} names no reopen target",
                    envelope.notification_envelope_id
                ));
            }
            if envelope.dedupe_key.is_empty() || envelope.action_target.action_target_id.is_empty()
            {
                return fail(format!(
                    "envelope {} is missing its dedupe key or action target",
                    envelope.notification_envelope_id
                ));
            }
        }

        // Every decision references a known envelope and context and recomputes
        // identically (reproducible routing).
        for decision in &self.decisions {
            let Some(envelope) = self.envelope(&decision.envelope_id) else {
                return fail(format!(
                    "decision {} references unknown envelope {}",
                    decision.decision_id, decision.envelope_id
                ));
            };
            let Some(context) = self.context(&decision.context_id) else {
                return fail(format!(
                    "decision {} references unknown context {}",
                    decision.decision_id, decision.context_id
                ));
            };
            if &route_envelope(envelope, context) != decision {
                return fail(format!(
                    "decision {} is not reproducible from its envelope and context",
                    decision.decision_id
                ));
            }
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

/// Builds the canonical envelope-routing bundle.
///
/// Deterministic: the same bytes every call. The producer registry, envelope
/// corpus, and routing contexts are fixed, every decision is computed by
/// [`route_envelope`], and each invariant's `holds` flag is computed from the built
/// data, so an inconsistent edit flips an invariant rather than silently passing.
pub fn envelope_routing_bundle() -> EnvelopeRoutingBundle {
    let producers = build_producers();
    let envelopes = build_envelopes();
    let contexts = build_contexts();
    let decisions = build_decisions(&envelopes, &contexts);
    let invariants = compute_invariants(&producers, &envelopes, &contexts, &decisions);

    EnvelopeRoutingBundle {
        record_kind: M5_ENVELOPE_ROUTING_RECORD_KIND.to_owned(),
        m5_envelope_routing_schema_version: M5_ENVELOPE_ROUTING_SCHEMA_VERSION,
        schema_ref: M5_ENVELOPE_ROUTING_SCHEMA_REF.to_owned(),
        bundle_id: M5_ENVELOPE_ROUTING_BUNDLE_ID.to_owned(),
        as_of: M5_ENVELOPE_ROUTING_AS_OF.to_owned(),
        matrix_ref: M5_ENVELOPE_ROUTING_MATRIX_REF.to_owned(),
        matrix_id: M5_ATTENTION_ROUTING_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ENVELOPE_ROUTING_FREEZE_GATE_REF.to_owned(),
        summary: "One typed notification-envelope path for every M5 producer — shell, notebooks, \
                  tasks, AI, collaboration, incidents, operator, managed policy, companion, \
                  security, continuity, and support — routed deterministically against active \
                  window, focus, do-not-disturb, presentation/follow, screen-reader, collaboration \
                  role, and user/admin policy. Every decision keeps a durable in-product record, \
                  shares one stable action target across the activity center, OS notification, \
                  companion, and badge, never widens privacy on fanout, and never lets an \
                  out-of-window surface bypass the in-product preview/approval flow."
            .to_owned(),
        producers,
        envelopes,
        contexts,
        decisions,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_decisions(
    envelopes: &[NotificationEnvelope],
    contexts: &[RoutingContext],
) -> Vec<RoutingDecision> {
    let mut decisions = Vec::with_capacity(contexts.len() * envelopes.len());
    for context in contexts {
        for envelope in envelopes {
            decisions.push(route_envelope(envelope, context));
        }
    }
    decisions
}

fn surfaces(items: &[FanoutChannelClass]) -> Vec<FanoutChannelClass> {
    let mut out = vec![FanoutChannelClass::InAppActivityCenter];
    for item in items {
        if !out.contains(item) {
            out.push(*item);
        }
    }
    out
}

fn reopen(items: &[ReopenTargetClass]) -> Vec<ReopenTargetClass> {
    items.to_vec()
}

#[allow(clippy::too_many_arguments)]
fn envelope(
    subsystem: SourceSubsystemClass,
    slug: &str,
    severity: NotificationSeverityClass,
    scope: AttentionScopeClass,
    privacy_class: NotificationPrivacyClass,
    dedupe_strategy: DedupeStrategyClass,
    recommended: Vec<FanoutChannelClass>,
    reopen_targets: Vec<ReopenTargetClass>,
    action: ActionTarget,
    default_redaction: AttentionRedactionClass,
) -> NotificationEnvelope {
    NotificationEnvelope {
        notification_envelope_id: format!("notification_envelope:{slug}:0001"),
        canonical_event_id: format!("event:{slug}:0001"),
        source_subsystem: subsystem,
        producer_id: format!("producer:{slug}"),
        severity,
        scope,
        scope_ref: format!("aureline://scope/{slug}/0001"),
        privacy_class,
        dedupe_strategy,
        dedupe_key: format!("dedupe:{slug}"),
        recommended_surfaces: recommended,
        action_target: action,
        reopen_targets,
        title_key: format!("notify.{}.title", slug.replace('.', "_")),
        body_key: format!("notify.{}.body", slug.replace('.', "_")),
        default_redaction,
        carries_durable_record: true,
        carries_localizable_copy: true,
        created_at: M5_ENVELOPE_ROUTING_AS_OF.to_owned(),
    }
}

fn action(
    slug: &str,
    primary_action: ActionVerbClass,
    reopen_target: ReopenTargetClass,
    routes_through_preview_approval: bool,
) -> ActionTarget {
    ActionTarget {
        action_target_id: format!("action_target:{slug}"),
        primary_action,
        reopen_target,
        object_ref: format!("aureline://object/{slug}/0001"),
        label_key: format!("notify.{}.action", slug.replace('.', "_")),
        routes_through_preview_approval,
    }
}

fn build_producers() -> Vec<ProducerEntry> {
    let p = |subsystem: SourceSubsystemClass,
             slug: &str,
             label: &str,
             summary: &str,
             produced_by: &[&str]| ProducerEntry {
        producer_id: format!("producer:{slug}"),
        source_subsystem: subsystem,
        label: label.to_owned(),
        summary: summary.to_owned(),
        produced_by_refs: produced_by.iter().map(|s| (*s).to_owned()).collect(),
        emits_envelope_id: format!("notification_envelope:{slug}:0001"),
        routes_through_typed_envelope: true,
        retains_surface_local_logic: false,
    };

    use SourceSubsystemClass::*;
    vec![
        p(
            Shell,
            "shell.command_result",
            "Shell command result",
            "Command results and undo affordances route through the typed envelope instead of a \
             surface-local toast.",
            &["crates/aureline-shell/src/activity_center/mod.rs"],
        ),
        p(
            Notebook,
            "notebook.cell_run",
            "Notebook cell run",
            "Notebook cell and run outcomes emit the typed envelope rather than an inline banner.",
            &["crates/aureline-shell/src/m5_activity_objects/mod.rs"],
        ),
        p(
            TaskRunner,
            "task.run_failed",
            "Task run failed",
            "Long-running task failures emit the typed envelope with a retry action instead of a \
             toast-only error.",
            &["crates/aureline-shell/src/m5_activity_objects/mod.rs"],
        ),
        p(
            Ai,
            "ai.awaiting_approval",
            "AI awaiting approval",
            "AI / agent handoffs awaiting review route through the typed envelope and the \
             in-product preview/approval flow.",
            &["crates/aureline-ai/src/lib.rs"],
        ),
        p(
            Collaboration,
            "collab.review_requested",
            "Collaboration review requested",
            "Shared review requests emit the typed envelope so every device opens the same review.",
            &["crates/aureline-companion/src/lib.rs"],
        ),
        p(
            Incident,
            "incident.thread_opened",
            "Incident thread opened",
            "New incident threads emit the typed envelope rather than a surface-local alert.",
            &["crates/aureline-incident/src/lib.rs"],
        ),
        p(
            Operator,
            "operator.fleet_alert",
            "Operator fleet alert",
            "Managed fleet alerts emit the typed envelope consumed by the read-only operator \
             dashboard.",
            &["crates/aureline-incident/src/lib.rs"],
        ),
        p(
            ManagedPolicy,
            "managed.policy_changed",
            "Managed policy changed",
            "Managed policy and bundle changes emit the typed envelope that reopens the policy \
             diff.",
            &["crates/aureline-policy/src/m5_admin_render/mod.rs"],
        ),
        p(
            Companion,
            "companion.fanout_status",
            "Companion fanout status",
            "Cross-client continuation and fanout status emit the typed envelope rather than an \
             ad hoc cross-client banner.",
            &["crates/aureline-companion/src/lib.rs"],
        ),
        p(
            Security,
            "security.credential_revoked",
            "Security credential revoked",
            "Credential revocation and trust advisories emit the typed envelope that breaks \
             through with a redacted summary.",
            &["crates/aureline-support/src/lib.rs"],
        ),
        p(
            Sync,
            "sync.restore_complete",
            "Continuity restore complete",
            "Backup, restore, and failover outcomes emit the typed envelope rather than a \
             surface-local success toast.",
            &["crates/aureline-support/src/lib.rs"],
        ),
        p(
            Support,
            "support.export_ready",
            "Support export ready",
            "Support-export and diagnostic-bundle notices emit the typed envelope that reopens the \
             evidence packet.",
            &["crates/aureline-support/src/lib.rs"],
        ),
    ]
}

fn build_envelopes() -> Vec<NotificationEnvelope> {
    use ActionVerbClass::*;
    use AttentionRedactionClass::*;
    use AttentionScopeClass::*;
    use DedupeStrategyClass::*;
    use FanoutChannelClass::*;
    use NotificationPrivacyClass::*;
    use NotificationSeverityClass::*;
    use ReopenTargetClass::*;
    use SourceSubsystemClass as S;

    vec![
        envelope(
            S::Shell,
            "shell.command_result",
            MinorSuccess,
            Window,
            SummarySafe,
            LatestSupersedes,
            surfaces(&[DockTaskbarBadge]),
            reopen(&[RouteObject, ActivityJobRow]),
            action("shell.command_result", Open, RouteObject, false),
            SummaryOnly,
        ),
        envelope(
            S::Notebook,
            "notebook.cell_run",
            Progress,
            Session,
            WorkspaceSensitive,
            CanonicalKeyCoalesce,
            surfaces(&[DockTaskbarBadge]),
            reopen(&[ActivityJobRow, EvidencePacket]),
            action("notebook.cell_run", Open, ActivityJobRow, false),
            MetadataSafeDefault,
        ),
        envelope(
            S::TaskRunner,
            "task.run_failed",
            Degraded,
            Session,
            WorkspaceSensitive,
            RootCauseCollapse,
            surfaces(&[OsNativeNotification, DockTaskbarBadge, MobileCompanion]),
            reopen(&[ActivityJobRow, EvidencePacket]),
            action("task.run_failed", Retry, ActivityJobRow, false),
            SummaryOnly,
        ),
        envelope(
            S::Ai,
            "ai.awaiting_approval",
            HandoffActionable,
            Session,
            WorkspaceSensitive,
            LatestSupersedes,
            surfaces(&[OsNativeNotification, BrowserCompanion, MobileCompanion]),
            reopen(&[ReviewRequest, ActivityJobRow]),
            // Approving an AI change must route through the in-product flow.
            action("ai.awaiting_approval", ReviewApprove, ReviewRequest, true),
            SummaryOnly,
        ),
        envelope(
            S::Collaboration,
            "collab.review_requested",
            HandoffActionable,
            Collaboration,
            WorkspaceSensitive,
            CanonicalKeyCoalesce,
            surfaces(&[OsNativeNotification, BrowserCompanion, MobileCompanion]),
            reopen(&[ReviewRequest, RouteObject]),
            action(
                "collab.review_requested",
                ReviewApprove,
                ReviewRequest,
                true,
            ),
            RedactedPayload,
        ),
        envelope(
            S::Incident,
            "incident.thread_opened",
            HandoffActionable,
            Workspace,
            SecurityCritical,
            RootCauseCollapse,
            surfaces(&[OsNativeNotification, MobileCompanion, OperatorDashboard]),
            reopen(&[IncidentThread, AuditEvent]),
            action("incident.thread_opened", Open, IncidentThread, false),
            RedactedPayload,
        ),
        envelope(
            S::Operator,
            "operator.fleet_alert",
            SecurityAdvisory,
            TenantOrg,
            ManagedSensitive,
            CountRollup,
            surfaces(&[OsNativeNotification, OperatorDashboard]),
            reopen(&[AuditEvent, RouteObject]),
            action("operator.fleet_alert", Open, AuditEvent, false),
            InternalSupportRestricted,
        ),
        envelope(
            S::ManagedPolicy,
            "managed.policy_changed",
            Informational,
            TenantOrg,
            ManagedSensitive,
            LatestSupersedes,
            surfaces(&[OperatorDashboard]),
            reopen(&[PolicyDiff, AuditEvent]),
            // Promoting a managed policy change is gated by preview/approval.
            action("managed.policy_changed", ReviewApprove, PolicyDiff, true),
            RedactedPayload,
        ),
        envelope(
            S::Companion,
            "companion.fanout_status",
            Informational,
            AppGlobal,
            SummarySafe,
            LatestSupersedes,
            surfaces(&[BrowserCompanion, MobileCompanion]),
            reopen(&[RouteObject, ActivityJobRow]),
            action("companion.fanout_status", Open, RouteObject, false),
            SummaryOnly,
        ),
        envelope(
            S::Security,
            "security.credential_revoked",
            SecurityAdvisory,
            AppGlobal,
            SecurityCritical,
            NoDedupe,
            surfaces(&[
                OsNativeNotification,
                DockTaskbarBadge,
                MobileCompanion,
                OperatorDashboard,
            ]),
            reopen(&[AuditEvent, EvidencePacket]),
            action("security.credential_revoked", Open, AuditEvent, false),
            RedactedPayload,
        ),
        envelope(
            S::Sync,
            "sync.restore_complete",
            MinorSuccess,
            Workspace,
            WorkspaceSensitive,
            LatestSupersedes,
            surfaces(&[DockTaskbarBadge]),
            reopen(&[ActivityJobRow, EvidencePacket]),
            action("sync.restore_complete", Open, ActivityJobRow, false),
            MetadataSafeDefault,
        ),
        envelope(
            S::Support,
            "support.export_ready",
            MinorSuccess,
            AppGlobal,
            SummarySafe,
            LatestSupersedes,
            surfaces(&[OsNativeNotification, DockTaskbarBadge]),
            reopen(&[EvidencePacket, ActivityJobRow]),
            action("support.export_ready", Open, EvidencePacket, false),
            SummaryOnly,
        ),
    ]
}

fn build_contexts() -> Vec<RoutingContext> {
    vec![
        RoutingContext {
            context_id: "context:default_focused".to_owned(),
            summary: "Working solo with the app focused, no quiet-hours or focus restrictions, \
                      unmanaged."
                .to_owned(),
            active_window: ActiveWindowClass::AppForegroundFocused,
            focus_mode: FocusModeClass::Off,
            do_not_disturb: DoNotDisturbClass::Off,
            presentation_mode: PresentationModeClass::Off,
            screen_reader: ScreenReaderPostureClass::Off,
            collaboration_role: CollaborationRoleClass::Solo,
            user_policy: UserNotificationPolicyClass::AllAllowed,
            admin_policy: AdminNotificationPolicyClass::Unmanaged,
            quiet_hours_active: false,
        },
        RoutingContext {
            context_id: "context:background_quiet_hours".to_owned(),
            summary: "App in the background during quiet-hours, working solo, unmanaged."
                .to_owned(),
            active_window: ActiveWindowClass::AppBackground,
            focus_mode: FocusModeClass::Off,
            do_not_disturb: DoNotDisturbClass::Off,
            presentation_mode: PresentationModeClass::Off,
            screen_reader: ScreenReaderPostureClass::Off,
            collaboration_role: CollaborationRoleClass::Solo,
            user_policy: UserNotificationPolicyClass::AllAllowed,
            admin_policy: AdminNotificationPolicyClass::Unmanaged,
            quiet_hours_active: true,
        },
        RoutingContext {
            context_id: "context:presenting_dnd".to_owned(),
            summary: "Presenting with do-not-disturb on; the owner is sharing their screen."
                .to_owned(),
            active_window: ActiveWindowClass::AppForegroundUnfocused,
            focus_mode: FocusModeClass::FocusAssistOn,
            do_not_disturb: DoNotDisturbClass::On,
            presentation_mode: PresentationModeClass::Presenting,
            screen_reader: ScreenReaderPostureClass::Off,
            collaboration_role: CollaborationRoleClass::Owner,
            user_policy: UserNotificationPolicyClass::AllAllowed,
            admin_policy: AdminNotificationPolicyClass::ManagedDefault,
            quiet_hours_active: false,
        },
        RoutingContext {
            context_id: "context:managed_locked_owner".to_owned(),
            summary: "A managed install that locks cross-client fanout, viewed by the org owner."
                .to_owned(),
            active_window: ActiveWindowClass::AppForegroundFocused,
            focus_mode: FocusModeClass::Off,
            do_not_disturb: DoNotDisturbClass::Off,
            presentation_mode: PresentationModeClass::Off,
            screen_reader: ScreenReaderPostureClass::Off,
            collaboration_role: CollaborationRoleClass::Owner,
            user_policy: UserNotificationPolicyClass::AllAllowed,
            admin_policy: AdminNotificationPolicyClass::ManagedLocked,
            quiet_hours_active: false,
        },
        RoutingContext {
            context_id: "context:screen_reader_reviewer".to_owned(),
            summary: "A reviewer with a screen reader active, important-only notifications."
                .to_owned(),
            active_window: ActiveWindowClass::AppForegroundUnfocused,
            focus_mode: FocusModeClass::Off,
            do_not_disturb: DoNotDisturbClass::Off,
            presentation_mode: PresentationModeClass::Off,
            screen_reader: ScreenReaderPostureClass::On,
            collaboration_role: CollaborationRoleClass::Reviewer,
            user_policy: UserNotificationPolicyClass::ImportantOnly,
            admin_policy: AdminNotificationPolicyClass::ManagedDefault,
            quiet_hours_active: false,
        },
        RoutingContext {
            context_id: "context:guest_muted".to_owned(),
            summary: "A guest with out-of-window notifications muted, app in the background."
                .to_owned(),
            active_window: ActiveWindowClass::AppBackground,
            focus_mode: FocusModeClass::Off,
            do_not_disturb: DoNotDisturbClass::Off,
            presentation_mode: PresentationModeClass::Off,
            screen_reader: ScreenReaderPostureClass::Off,
            collaboration_role: CollaborationRoleClass::Guest,
            user_policy: UserNotificationPolicyClass::Muted,
            admin_policy: AdminNotificationPolicyClass::ManagedRestricted,
            quiet_hours_active: false,
        },
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> EnvelopeRoutingInvariant {
    EnvelopeRoutingInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(
    producers: &[ProducerEntry],
    envelopes: &[NotificationEnvelope],
    contexts: &[RoutingContext],
    decisions: &[RoutingDecision],
) -> Vec<EnvelopeRoutingInvariant> {
    let matrix = attention_routing_matrix();
    let mut out = Vec::new();

    // Every claimed M5 producer routes through the typed envelope path.
    out.push(invariant(
        "envelope.every_producer_routes_typed",
        "Every M5 producer routes through the typed notification envelope and retains no \
         surface-local toast/banner/badge logic, and every subsystem has a producer.",
        producers
            .iter()
            .all(|p| p.routes_through_typed_envelope && !p.retains_surface_local_logic)
            && SourceSubsystemClass::ALL
                .iter()
                .all(|s| producers.iter().any(|p| p.source_subsystem == *s)),
    ));

    // Every envelope carries the required contract fields.
    out.push(invariant(
        "envelope.required_fields_present",
        "Every envelope carries a source subsystem, scope, privacy class, severity, non-empty \
         dedupe key, non-empty recommended surface set, and a stable action target.",
        envelopes.iter().all(|e| {
            !e.dedupe_key.is_empty()
                && !e.recommended_surfaces.is_empty()
                && !e.action_target.action_target_id.is_empty()
                && e.recommends(FanoutChannelClass::InAppActivityCenter)
        }),
    ));

    // One stable action target on every surface of a decision.
    out.push(invariant(
        "envelope.stable_action_target_shared",
        "In every decision, every surface outcome references the one stable action target the \
         source envelope declares.",
        decisions.iter().all(|d| {
            d.outcomes
                .iter()
                .all(|o| o.action_target_id == d.action_target_id)
                && self_envelope_action(envelopes, d).is_some_and(|t| t == d.action_target_id)
        }),
    ));

    // Every decision delivers a durable in-product record (no toast-only).
    out.push(invariant(
        "envelope.durable_record_always",
        "Every decision delivers the in-app activity center as a durable record, so no attention \
         lives only in an ephemeral toast.",
        decisions.iter().all(|d| {
            d.durable_record_present
                && d.outcome(FanoutChannelClass::InAppActivityCenter)
                    .is_some_and(|o| {
                        o.disposition == RouteDispositionClass::Deliver && o.delivers_durable_record
                    })
        }),
    ));

    // Deferral/suppression of a fanout surface never drops the durable record.
    out.push(invariant(
        "envelope.suppression_separate_from_durable",
        "When an out-of-window surface is deferred or suppressed, the in-app durable record is \
         still delivered, so suppression and quiet-hours state stay separate from the durable \
         object.",
        decisions.iter().all(|d| {
            let any_held = d
                .outcomes
                .iter()
                .any(|o| o.disposition.is_deferred() || o.disposition.is_suppressed());
            !any_held || d.durable_record_present
        }),
    ));

    // Privacy never widens on fanout.
    out.push(invariant(
        "envelope.privacy_never_widens_on_fanout",
        "Every out-of-window surface outcome applies a redaction at least as strong as the \
         envelope default and the channel's privacy ceiling, so privacy never widens on fanout.",
        decisions.iter().all(|d| {
            let Some(env) = self_envelope(envelopes, d) else {
                return false;
            };
            d.outcomes.iter().all(|o| {
                if o.surface == FanoutChannelClass::InAppActivityCenter {
                    return true;
                }
                let profile = channel_profile(o.surface);
                redaction_rank(o.applied_redaction) >= redaction_rank(env.default_redaction)
                    && redaction_rank(o.applied_redaction)
                        >= redaction_rank(profile.default_redaction)
            })
        }),
    ));

    // Fanout cannot bypass preview/approval.
    out.push(invariant(
        "envelope.fanout_cannot_bypass_preview_approval",
        "For every envelope whose action target routes through preview/approval, no out-of-window \
         surface executes it inline; each hands off to the in-product surface.",
        decisions.iter().all(|d| {
            let Some(env) = self_envelope(envelopes, d) else {
                return false;
            };
            if !env.action_target.routes_through_preview_approval {
                return true;
            }
            d.outcomes
                .iter()
                .filter(|o| o.surface != FanoutChannelClass::InAppActivityCenter)
                .all(|o| o.dangerous_action_handoff_to_in_product)
        }),
    ));

    // Routing is reproducible.
    out.push(invariant(
        "envelope.routing_reproducible",
        "Re-running the routing engine on every decision's envelope and context yields an \
         identical decision, so routing is reproducible in support export and diagnostics.",
        decisions.iter().all(
            |d| match (self_envelope(envelopes, d), self_context(contexts, d)) {
                (Some(env), Some(ctx)) => &route_envelope(env, ctx) == d,
                _ => false,
            },
        ),
    ));

    // Every routing context carries every required routing input. (Enum-typed
    // fields are always present; the invariant proves the contexts span the key
    // routing axes the spec lists rather than a single trivial context.)
    out.push(invariant(
        "envelope.context_inputs_complete",
        "The routing contexts exercise the active-window, focus, do-not-disturb, presentation/\
         follow, screen-reader, collaboration-role, and user/admin-policy routing inputs the \
         contract requires.",
        contexts.iter().any(|c| c.quiet_hours_active)
            && contexts
                .iter()
                .any(|c| c.do_not_disturb == DoNotDisturbClass::On)
            && contexts
                .iter()
                .any(|c| c.presentation_mode != PresentationModeClass::Off)
            && contexts
                .iter()
                .any(|c| c.screen_reader == ScreenReaderPostureClass::On)
            && contexts
                .iter()
                .any(|c| c.admin_policy == AdminNotificationPolicyClass::ManagedLocked)
            && contexts
                .iter()
                .any(|c| c.user_policy == UserNotificationPolicyClass::Muted)
            && contexts.iter().any(|c| c.collaboration_role.is_limited()),
    ));

    // Copy is localizable; the contract is the enums/ids/action targets.
    out.push(invariant(
        "envelope.copy_localizable_not_contract",
        "Every envelope carries localizable title/body/action keys rather than raw copy, so \
         message text stays revisable while the stable enums, ids, and action targets are the \
         contract.",
        envelopes.iter().all(|e| {
            e.carries_localizable_copy
                && e.title_key.starts_with("notify.")
                && e.body_key.starts_with("notify.")
                && e.action_target.label_key.starts_with("notify.")
        }),
    ));

    // Support-export safe.
    out.push(invariant(
        "envelope.support_export_safe",
        "Every envelope scope ref, action-target object ref, and producer source ref is a \
         repo-relative object ref or opaque aureline:// handle, never a URL, host, credential, or \
         absolute path, so the routing decisions are safe to embed in a support export.",
        envelopes.iter().all(|e| {
            is_export_safe_ref(&e.scope_ref) && is_export_safe_ref(&e.action_target.object_ref)
        }) && producers
            .iter()
            .all(|p| p.produced_by_refs.iter().all(|r| is_export_safe_ref(r)))
            && decisions
                .iter()
                .all(|d| d.outcomes.iter().all(|o| !o.reason.is_empty())),
    ));

    // The activity center, OS notification, companion, and badge consume the same
    // envelope and action target.
    out.push(invariant(
        "envelope.consumer_parity",
        "At least one decision routes the in-app activity center, OS notification, dock/taskbar \
         badge, and a companion surface from one envelope, each carrying the same action target, \
         and each of those four surface families is delivered somewhere.",
        consumer_parity_holds(decisions),
    ));

    // Every vocabulary token is one the attention-routing matrix defines.
    out.push(invariant(
        "envelope.matrix_bound",
        "Every severity, scope, privacy class, dedupe rule, channel, and reopen target the bundle \
         uses is one the attention-routing matrix defines, and the channel routing profiles match \
         the matrix.",
        matrix_bound_holds(envelopes, &matrix),
    ));

    // Every action target reopens an authoritative object.
    out.push(invariant(
        "envelope.action_target_reopens_authoritative",
        "Every envelope's action target names an authoritative reopen target, so a surface reopens \
         the object rather than issuing a blind side effect.",
        envelopes
            .iter()
            .all(|e| e.reopen_targets.contains(&e.action_target.reopen_target)),
    ));

    // Every recommended surface is handled exactly once, with the activity center
    // always present.
    out.push(invariant(
        "envelope.recommended_surfaces_handled",
        "Every recommended surface produces exactly one outcome in each of the envelope's \
         decisions, and the in-app activity center is always among them.",
        decisions.iter().all(|d| {
            let Some(env) = self_envelope(envelopes, d) else {
                return false;
            };
            let handled: Vec<FanoutChannelClass> = d.outcomes.iter().map(|o| o.surface).collect();
            handled.contains(&FanoutChannelClass::InAppActivityCenter)
                && env
                    .recommended_surfaces
                    .iter()
                    .all(|s| d.outcomes.iter().filter(|o| o.surface == *s).count() == 1)
                && all_unique(d.outcomes.iter().map(|o| o.surface.as_str()))
        }),
    ));

    out
}

fn self_envelope<'a>(
    envelopes: &'a [NotificationEnvelope],
    decision: &RoutingDecision,
) -> Option<&'a NotificationEnvelope> {
    envelopes
        .iter()
        .find(|e| e.notification_envelope_id == decision.envelope_id)
}

fn self_context<'a>(
    contexts: &'a [RoutingContext],
    decision: &RoutingDecision,
) -> Option<&'a RoutingContext> {
    contexts
        .iter()
        .find(|c| c.context_id == decision.context_id)
}

fn self_envelope_action(
    envelopes: &[NotificationEnvelope],
    decision: &RoutingDecision,
) -> Option<String> {
    self_envelope(envelopes, decision).map(|e| e.action_target.action_target_id.clone())
}

fn consumer_parity_holds(decisions: &[RoutingDecision]) -> bool {
    let families = [
        FanoutChannelClass::InAppActivityCenter,
        FanoutChannelClass::OsNativeNotification,
        FanoutChannelClass::DockTaskbarBadge,
    ];
    // Some decision routes the activity center, OS, badge, and a companion from one
    // envelope with one shared action target.
    let parity = decisions.iter().any(|d| {
        let has_companion = d.outcome(FanoutChannelClass::BrowserCompanion).is_some()
            || d.outcome(FanoutChannelClass::MobileCompanion).is_some();
        families.iter().all(|f| d.outcome(*f).is_some())
            && has_companion
            && d.outcomes
                .iter()
                .all(|o| o.action_target_id == d.action_target_id)
    });
    // Each of the four required surface families is delivered somewhere.
    let delivered = |surface: FanoutChannelClass| {
        decisions.iter().any(|d| {
            d.outcome(surface)
                .is_some_and(|o| o.disposition.is_delivered())
        })
    };
    let companion_delivered = delivered(FanoutChannelClass::BrowserCompanion)
        || delivered(FanoutChannelClass::MobileCompanion);
    parity
        && delivered(FanoutChannelClass::InAppActivityCenter)
        && delivered(FanoutChannelClass::OsNativeNotification)
        && delivered(FanoutChannelClass::DockTaskbarBadge)
        && companion_delivered
}

fn matrix_bound_holds(
    envelopes: &[NotificationEnvelope],
    matrix: &crate::m5_attention_routing::AttentionRoutingMatrix,
) -> bool {
    let severities: Vec<&str> = matrix
        .shared_vocabulary
        .severities
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let dedupes: Vec<&str> = matrix
        .shared_vocabulary
        .dedupe_rules
        .iter()
        .map(|t| t.token.as_str())
        .collect();
    let reopens: Vec<&str> = matrix
        .shared_vocabulary
        .reopen_targets
        .iter()
        .map(|t| t.token.as_str())
        .collect();

    let tokens_bound = envelopes.iter().all(|e| {
        severities.contains(&e.severity.as_str())
            && dedupes.contains(&e.dedupe_strategy.as_str())
            && reopens.contains(&e.action_target.reopen_target.as_str())
            && e.reopen_targets
                .iter()
                .all(|t| reopens.contains(&t.as_str()))
    });

    // The local channel routing profiles match the matrix channel entries.
    let profiles_match = FanoutChannelClass::ALL.iter().all(|channel| {
        let local = channel_profile(*channel);
        matrix.channel(*channel).is_some_and(|entry| {
            entry.privacy_ceiling == local.privacy_ceiling
                && entry.default_redaction == local.default_redaction
                && entry.mirrors_authoritative == local.mirrors_authoritative
        })
    });

    tokens_bound && profiles_match
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn envelope_routing_lines(bundle: &EnvelopeRoutingBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Envelope-routing bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Producers: {}  Envelopes: {}  Contexts: {}  Decisions: {}  Invariants: {}",
        bundle.producers.len(),
        bundle.envelopes.len(),
        bundle.contexts.len(),
        bundle.decisions.len(),
        bundle.invariants.len(),
    ));

    lines.push("Producers:".to_owned());
    for p in &bundle.producers {
        lines.push(format!(
            "  - {} [{}] -> {}",
            p.source_subsystem.as_str(),
            p.producer_id,
            p.emits_envelope_id,
        ));
        lines.push(format!("      {}", p.summary));
    }

    lines.push("Envelopes:".to_owned());
    for e in &bundle.envelopes {
        let surfaces: Vec<&str> = e.recommended_surfaces.iter().map(|s| s.as_str()).collect();
        lines.push(format!(
            "  - {} [{}] severity={} scope={} privacy={} dedupe={}",
            e.source_subsystem.as_str(),
            e.notification_envelope_id,
            e.severity.as_str(),
            e.scope.as_str(),
            e.privacy_class.as_str(),
            e.dedupe_strategy.as_str(),
        ));
        lines.push(format!(
            "      action={} verb={} preview_approval={} surfaces={}",
            e.action_target.action_target_id,
            e.action_target.primary_action.as_str(),
            e.action_target.routes_through_preview_approval,
            surfaces.join(", "),
        ));
    }

    lines.push("Decisions:".to_owned());
    for d in &bundle.decisions {
        let parts: Vec<String> = d
            .outcomes
            .iter()
            .map(|o| format!("{}={}", o.surface.as_str(), o.disposition.as_str()))
            .collect();
        lines.push(format!(
            "  - {} x {} durable={} :: {}",
            d.envelope_id,
            d.context_id,
            d.durable_record_present,
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
